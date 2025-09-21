/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Statistics thread for G3 ICAP Server
//!
//! This module handles periodic emission of statistics to StatsD following G3Proxy pattern.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;

use anyhow::anyhow;
use g3_statsd_client::{StatsdClient, StatsdClientConfig};

use super::IcapStats;

static QUIT_STAT_THREAD: AtomicBool = AtomicBool::new(false);

fn build_statsd_client(config: &StatsdClientConfig) -> anyhow::Result<StatsdClient> {
    let client = config
        .build()
        .map_err(|e| anyhow!("failed to build statsd client: {e}"))?;
    Ok(client.with_tag(
        g3_daemon::metrics::TAG_KEY_DAEMON_GROUP,
        crate::opts::daemon_group(),
    ))
}

/// Spawn statistics thread for periodic StatsD emission following G3Proxy pattern
pub fn spawn_stats_thread(config: &StatsdClientConfig, stats: Arc<IcapStats>) -> anyhow::Result<JoinHandle<()>> {
    let mut client = build_statsd_client(config)?;
    let emit_duration = config.emit_interval;
    
    let handle = std::thread::Builder::new()
        .name("g3icap-stat".to_string())
        .spawn(move || {
            loop {
                let instant_start = Instant::now();

                // Emit statistics to StatsD
                stats.emit_stats(&mut client);
                client.flush_sink();

                if QUIT_STAT_THREAD.load(Ordering::Relaxed) {
                    break;
                }

                g3_daemon::stat::emit::wait_duration(emit_duration, instant_start);
            }
        })
        .map_err(|e| anyhow!("failed to spawn thread: {e:?}"))?;
    
    Ok(handle)
}

/// Signal the statistics thread to quit
pub fn quit_stats_thread() {
    QUIT_STAT_THREAD.store(true, Ordering::Relaxed);
}
