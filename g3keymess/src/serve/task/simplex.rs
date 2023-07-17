/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

use g3_io_ext::LimitedBufReadExt;

use super::KeylessTask;
use crate::log::request::RequestErrorLogContext;
use crate::protocol::{KeylessErrorResponse, KeylessResponse};
use crate::serve::{ServerReloadCommand, ServerTaskError};

impl KeylessTask {
    pub(crate) async fn into_simplex_running<R, W>(mut self, reader: R, mut writer: W)
    where
        R: AsyncRead + Send + Unpin + 'static,
        W: AsyncWrite + Send + Unpin + 'static,
    {
        let mut buf_reader = BufReader::new(reader);

        loop {
            tokio::select! {
                biased;

                r = buf_reader.fill_wait_data() => {
                    match r {
                        Ok(true) => {
                            if let Err(e) = self.read_and_handle(&mut buf_reader, &mut writer).await {
                                self.log_task_err(e);
                                break;
                            }
                        }
                        Ok(false) => break,
                        Err(e) => {
                            self.log_task_err(ServerTaskError::ReadFailed(e));
                            break;
                        }
                    }
                }
                r = self.ctx.reload_notifier.recv() => {
                    match r {
                        Ok(ServerReloadCommand::QuitRuntime) => {
                            // TODO close connection gracefully
                            self.log_task_err(ServerTaskError::ServerForceQuit);
                            break;
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            // force quit
                            self.log_task_err(ServerTaskError::ServerForceQuit);
                            break;
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {}
                    }
                }
            }
        }
    }

    async fn read_and_handle<R, W>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), ServerTaskError>
    where
        R: AsyncRead + Send + Unpin + 'static,
        W: AsyncWrite + Send + Unpin + 'static,
    {
        let req = self.timed_read_request(reader).await?;
        if let Some(pong) = req.inner.ping_pong() {
            req.stats.add_passed();
            return self
                .send_response(writer, KeylessResponse::Pong(pong))
                .await;
        }

        let rsp = KeylessErrorResponse::new(req.inner.id);

        let Some(key) = req.inner.find_key() else {
            req.stats.add_key_not_found();
            return self.send_response(writer, KeylessResponse::Error(rsp.key_not_found())).await;
        };

        let server_sem = if let Some(sem) = self.ctx.concurrency_limit.clone() {
            sem.acquire_owned().await.ok()
        } else {
            None
        };

        let rsp = match req.inner.process(&key) {
            Ok(d) => {
                req.stats.add_passed();
                KeylessResponse::Data(d)
            }
            Err(e) => {
                req.stats.add_by_error_code(e.error_code());
                KeylessResponse::Error(e)
            }
        };

        drop(server_sem);
        drop(req);

        self.send_response(writer, rsp).await
    }

    pub(super) async fn send_response<W>(
        &self,
        writer: &mut W,
        rsp: KeylessResponse,
    ) -> Result<(), ServerTaskError>
    where
        W: AsyncWrite + Send + Unpin + 'static,
    {
        RequestErrorLogContext { task_id: &self.id }.log(&self.ctx.request_logger, &rsp);

        writer
            .write_all(rsp.message())
            .await
            .map_err(ServerTaskError::WriteFailed)?;
        writer.flush().await.map_err(ServerTaskError::WriteFailed)?;
        Ok(())
    }
}
