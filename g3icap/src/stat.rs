/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Statistics module for G3 ICAP Server
//! 
//! This module provides global statistics collection following G3Proxy pattern.

use std::sync::Arc;
use std::thread::JoinHandle;

use anyhow::{Context, Result};
use g3_statsd_client::StatsdClientConfig;

use crate::stats::{IcapStats, thread};

/// Global statistics instance
static GLOBAL_STATS: std::sync::OnceLock<Arc<IcapStats>> = std::sync::OnceLock::new();

/// Initialize global statistics
pub fn init_global_stats() {
    let stats = Arc::new(IcapStats::new());
    let _ = GLOBAL_STATS.set(stats);
}

/// Get global statistics instance
pub fn get_global_stats() -> Option<Arc<IcapStats>> {
    GLOBAL_STATS.get().cloned()
}

/// Spawn working threads for statistics following G3Proxy pattern
pub fn spawn_working_threads(config: StatsdClientConfig) -> Result<Vec<JoinHandle<()>>> {
    let mut handlers = Vec::with_capacity(1);
    let stats = get_global_stats().unwrap_or_else(|| Arc::new(IcapStats::new()));
    let main_handle = thread::spawn_stats_thread(&config, stats)
        .with_context(|| "failed to spawn main stats thread")?;
    handlers.push(main_handle);
    Ok(handlers)
}

/// Stop working threads
pub fn stop_working_threads() {
    thread::quit_stats_thread();
}
