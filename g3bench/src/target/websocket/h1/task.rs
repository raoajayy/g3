/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

use std::sync::Arc;

use anyhow::{Context, anyhow};
use futures_util::FutureExt;
use http::Method;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, BufReader};
use tokio::time::Instant;

use g3_http::client::HttpTransparentResponse;
use g3_io_ext::{LimitedReader, LimitedWriteExt, LimitedWriter};
use g3_types::net::HttpUpgradeToken;

use super::H1WebsocketArgs;
use crate::ProcArgs;
use crate::module::http::{HttpHistogramRecorder, HttpRuntimeStats, SavedHttpForwardConnection};
use crate::target::websocket::{FrameType, ServerFrameHeader};
use crate::target::{BenchError, BenchTaskContext};

pub(super) struct H1WebsocketTaskContext {
    args: Arc<H1WebsocketArgs>,
    proc_args: Arc<ProcArgs>,
    saved_connection: Option<SavedHttpForwardConnection>,
    reuse_conn_count: u64,

    runtime_stats: Arc<HttpRuntimeStats>,
    histogram_recorder: HttpHistogramRecorder,

    request_buf: Vec<u8>,
    response_buf: Vec<u8>,
}

impl H1WebsocketTaskContext {
    pub(super) fn new(
        args: Arc<H1WebsocketArgs>,
        proc_args: Arc<ProcArgs>,
        runtime_stats: Arc<HttpRuntimeStats>,
        histogram_recorder: HttpHistogramRecorder,
    ) -> Self {
        H1WebsocketTaskContext {
            args,
            proc_args,
            saved_connection: None,
            reuse_conn_count: 0,
            runtime_stats,
            histogram_recorder,
            request_buf: Vec::new(),
            response_buf: Vec::new(),
        }
    }

    async fn upgrade<R, W>(&self, writer: &mut W, reader: &mut R) -> anyhow::Result<()>
    where
        R: AsyncBufRead + Send + Sync + Unpin,
        W: AsyncWrite + Send + Sync + Unpin,
    {
        let mut buf = Vec::with_capacity(512);
        let key = self
            .args
            .build_upgrade_request(&mut buf)
            .context("failed to build upgrade request")?;

        writer
            .write_all_flush(&buf)
            .await
            .map_err(|e| anyhow!("failed to write upgrade request: {e}"))?;

        let (rsp, _) = HttpTransparentResponse::parse(reader, &Method::GET, true, 1024).await?;
        if rsp.code != 101 {
            return Err(anyhow!(
                "upgrade failed, code: {}, reason: {}",
                rsp.code,
                rsp.reason
            ));
        }
        if !matches!(rsp.upgrade, Some(HttpUpgradeToken::Websocket)) {
            return Err(anyhow!(
                "no valid 'Upgrade' header found or 'Connection' contains no 'Upgrade'"
            ));
        }

        self.args
            .common
            .verify_upgrade_response_headers(key, rsp.end_to_end_headers.into())?;
        Ok(())
    }

    async fn fetch_connection(&mut self) -> anyhow::Result<SavedHttpForwardConnection> {
        if let Some(mut c) = self.saved_connection.take() {
            let mut buf = [0u8; 4];
            if c.reader.read(&mut buf).now_or_never().is_none() {
                // no eof, reuse the old connection
                self.reuse_conn_count += 1;
                return Ok(c);
            }
        }

        self.histogram_recorder
            .record_conn_reuse_count(self.reuse_conn_count);
        self.reuse_conn_count = 0;

        self.runtime_stats.add_conn_attempt();
        let (r, w) = match tokio::time::timeout(
            self.args.common.connect_timeout,
            self.args.connect.new_http_connection(
                &self.args.common.target,
                &self.runtime_stats,
                &self.proc_args,
            ),
        )
        .await
        {
            Ok(Ok(c)) => c,
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(anyhow!("timeout to get new connection")),
        };

        let r = LimitedReader::local_limited(
            r,
            self.proc_args.tcp_sock_speed_limit.shift_millis,
            self.proc_args.tcp_sock_speed_limit.max_south,
            self.runtime_stats.clone(),
        );
        let mut w = LimitedWriter::local_limited(
            w,
            self.proc_args.tcp_sock_speed_limit.shift_millis,
            self.proc_args.tcp_sock_speed_limit.max_north,
            self.runtime_stats.clone(),
        );

        let mut r = BufReader::new(r);
        tokio::time::timeout(
            self.args.common.upgrade_timeout,
            self.upgrade(&mut w, &mut r),
        )
        .await
        .map_err(|_| anyhow!("websocket upgrade timed out"))??;

        self.runtime_stats.add_conn_success();
        Ok(SavedHttpForwardConnection::new(r, w))
    }

    fn save_connection(&mut self, c: SavedHttpForwardConnection) {
        self.saved_connection = Some(c);
    }

    async fn read_frame_header<R>(&mut self, reader: &mut R) -> anyhow::Result<ServerFrameHeader>
    where
        R: AsyncBufRead + Unpin,
    {
        let buf = reader
            .fill_buf()
            .await
            .map_err(|e| anyhow!("failed to read frame from server: {e}"))?;

        let mut frame_header = if buf.len() < 2 {
            let mut buf = [0u8; 2];
            let nr = reader
                .read_exact(&mut buf)
                .await
                .map_err(|e| anyhow!("failed to read frame header: {e}"))?;
            if nr != buf.len() {
                return Err(anyhow!("not enough frame header data read"));
            }
            ServerFrameHeader::new(buf[0], buf[1]).context("invalid frame header received")?
        } else {
            let h =
                ServerFrameHeader::new(buf[0], buf[1]).context("invalid frame header received")?;
            reader.consume(2);
            h
        };

        if let Some(buf) = frame_header.payload_length_buf() {
            let nr = reader
                .read_exact(buf)
                .await
                .map_err(|e| anyhow!("failed to read payload length bytes: {e}"))?;
            if nr != buf.len() {
                return Err(anyhow!("not enough payload length bytes read"));
            }
            frame_header.parse_payload_length();
        }

        Ok(frame_header)
    }

    async fn recv_full_frame_data(
        &mut self,
        connection: &mut SavedHttpForwardConnection,
    ) -> anyhow::Result<FrameType> {
        self.response_buf.clear();
        let mut frame_type: Option<FrameType> = None;

        loop {
            let frame_header = self.read_frame_header(&mut connection.reader).await?;
            if frame_type.is_none() {
                if frame_header.frame_type() == FrameType::Continue {
                    return Err(anyhow!("the first frame type should not be Continue"));
                }
                frame_type = Some(frame_header.frame_type());
            } else if frame_header.frame_type() != FrameType::Continue {
                return Err(anyhow!(
                    "expected Continue frame type but we get {}",
                    frame_header.frame_type()
                ));
            }

            if frame_header.payload_length() > 0 {
                let Ok(to_read) = usize::try_from(frame_header.payload_length()) else {
                    return Err(anyhow!(
                        "too large frame payload length {}",
                        frame_header.payload_length()
                    ));
                };

                let nr = (&mut connection.reader)
                    .take(to_read as u64)
                    .read_to_end(&mut self.response_buf)
                    .await
                    .map_err(|e| anyhow!("failed to read payload: {e}"))?;
                if nr != to_read {
                    return Err(anyhow!(
                        "not enough payload data read: expected {to_read} but got {nr}"
                    ));
                }
            }

            if frame_header.is_last_frame() {
                break;
            }
        }

        frame_type.ok_or_else(|| anyhow!("no frame received"))
    }

    async fn run_with_connection(
        &mut self,
        connection: &mut SavedHttpForwardConnection,
    ) -> anyhow::Result<()> {
        self.request_buf.clear();
        self.args.common.build_request_frames(&mut self.request_buf);

        connection
            .writer
            .write_all_flush(&self.request_buf)
            .await
            .map_err(|e| anyhow!("failed to write request frames: {e}"))?;

        let frame_type = self.recv_full_frame_data(connection).await?;

        todo!()
    }
}

impl BenchTaskContext for H1WebsocketTaskContext {
    fn mark_task_start(&self) {
        self.runtime_stats.add_task_total();
        self.runtime_stats.inc_task_alive();
    }

    fn mark_task_passed(&self) {
        self.runtime_stats.add_task_passed();
        self.runtime_stats.dec_task_alive();
    }

    fn mark_task_failed(&self) {
        self.runtime_stats.add_task_failed();
        self.runtime_stats.dec_task_alive();
    }

    async fn run(&mut self, _task_id: usize, time_started: Instant) -> Result<(), BenchError> {
        let mut connection = self
            .fetch_connection()
            .await
            .context("connect to upstream failed")
            .map_err(BenchError::Fatal)?;

        match self.run_with_connection(&mut connection).await {
            Ok(_) => {
                let total_time = time_started.elapsed();
                self.histogram_recorder.record_total_time(total_time);
                self.save_connection(connection);
                Ok(())
            }
            Err(e) => Err(BenchError::Task(e)),
        }
    }
}
