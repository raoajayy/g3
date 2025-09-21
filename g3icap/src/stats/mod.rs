//! Statistics collection for G3 ICAP Server
//!
//! This module provides statistics collection and metrics for the ICAP server.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
// use std::time::Instant;
use std::thread::JoinHandle;

use anyhow::{Context, Result};
use g3_statsd_client::{StatsdClient, StatsdClientConfig, StatsdTagGroup};
use g3_daemon::metrics::TAG_KEY_DAEMON_GROUP;

use crate::opts::daemon_group;

pub mod thread;

/// Spawn working threads for statistics following G3Proxy pattern
pub fn spawn_working_threads(config: StatsdClientConfig) -> Result<Vec<JoinHandle<()>>> {
    let mut handlers = Vec::with_capacity(1);
    let stats = Arc::new(IcapStats::new());
    let main_handle = thread::spawn_stats_thread(&config, stats)
        .context("failed to spawn main stats thread")?;
    handlers.push(main_handle);
    Ok(handlers)
}

/// Stop working threads
pub fn stop_working_threads() {
    thread::quit_stats_thread();
}

// Metric name constants following G3Proxy pattern
const METRIC_NAME_ICAP_REQUESTS_TOTAL: &str = "icap.requests.total";
const METRIC_NAME_ICAP_REQUESTS_REQMOD: &str = "icap.requests.reqmod";
const METRIC_NAME_ICAP_REQUESTS_RESPMOD: &str = "icap.requests.respmod";
const METRIC_NAME_ICAP_REQUESTS_OPTIONS: &str = "icap.requests.options";
const METRIC_NAME_ICAP_RESPONSES_SUCCESSFUL: &str = "icap.responses.successful";
const METRIC_NAME_ICAP_RESPONSES_ERROR: &str = "icap.responses.error";
const METRIC_NAME_ICAP_REQUESTS_BLOCKED: &str = "icap.requests.blocked";
const METRIC_NAME_ICAP_BYTES_TOTAL: &str = "icap.bytes.total";
const METRIC_NAME_ICAP_CONNECTIONS_TOTAL: &str = "icap.connections.total";
const METRIC_NAME_ICAP_CONNECTIONS_ACTIVE: &str = "icap.connections.active";
const METRIC_NAME_ICAP_CONNECTIONS_ERROR: &str = "icap.connections.error";
const METRIC_NAME_ICAP_PROCESSING_TIME_TOTAL: &str = "icap.processing_time.total";
const METRIC_NAME_ICAP_PROCESSING_TIME_AVG: &str = "icap.processing_time.avg";

/// ICAP Server Statistics
pub struct IcapStats {
    /// Total number of requests processed
    total_requests: AtomicU64,
    /// Total number of REQMOD requests
    reqmod_requests: AtomicU64,
    /// Total number of RESPMOD requests
    respmod_requests: AtomicU64,
    /// Total number of OPTIONS requests
    options_requests: AtomicU64,
    /// Total number of successful responses
    successful_responses: AtomicU64,
    /// Total number of error responses
    error_responses: AtomicU64,
    /// Total number of blocked requests
    blocked_requests: AtomicU64,
    /// Total bytes processed
    total_bytes: AtomicU64,
    /// Current number of active connections
    active_connections: AtomicU64,
    /// Total number of connections accepted
    total_connections: AtomicU64,
    /// Connection errors
    connection_errors: AtomicU64,
    /// Request processing time (microseconds)
    total_processing_time: AtomicU64,
    /// StatsD client for metrics emission
    #[allow(dead_code)]
    statsd_client: Option<Arc<Mutex<StatsdClient>>>,
}

impl IcapStats {
    /// Create new statistics collector
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            reqmod_requests: AtomicU64::new(0),
            respmod_requests: AtomicU64::new(0),
            options_requests: AtomicU64::new(0),
            successful_responses: AtomicU64::new(0),
            error_responses: AtomicU64::new(0),
            blocked_requests: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            total_connections: AtomicU64::new(0),
            connection_errors: AtomicU64::new(0),
            total_processing_time: AtomicU64::new(0),
            statsd_client: None,
        }
    }

    /// Create new statistics collector with StatsD client
    pub fn new_with_statsd(config: &StatsdClientConfig) -> anyhow::Result<Self> {
        let client = config
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build statsd client: {e}"))?;
        
        let client_with_tag = client.with_tag(TAG_KEY_DAEMON_GROUP, daemon_group());
        
        Ok(Self {
            total_requests: AtomicU64::new(0),
            reqmod_requests: AtomicU64::new(0),
            respmod_requests: AtomicU64::new(0),
            options_requests: AtomicU64::new(0),
            successful_responses: AtomicU64::new(0),
            error_responses: AtomicU64::new(0),
            blocked_requests: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            total_connections: AtomicU64::new(0),
            connection_errors: AtomicU64::new(0),
            total_processing_time: AtomicU64::new(0),
            statsd_client: Some(Arc::new(Mutex::new(client_with_tag))),
        })
    }

    /// Increment total requests
    pub fn increment_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment REQMOD requests
    pub fn increment_reqmod_requests(&self) {
        self.reqmod_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment RESPMOD requests
    pub fn increment_respmod_requests(&self) {
        self.respmod_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment OPTIONS requests
    pub fn increment_options_requests(&self) {
        self.options_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment successful responses
    pub fn increment_successful_responses(&self) {
        self.successful_responses.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment error responses
    pub fn increment_error_responses(&self) {
        self.error_responses.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment connections
    pub fn increment_connections(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active connections
    pub fn decrement_active_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Increment errors
    pub fn increment_errors(&self) {
        self.connection_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment blocked requests
    pub fn increment_blocked_requests(&self) {
        self.blocked_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Add bytes processed
    pub fn add_bytes(&self, bytes: u64) {
        self.total_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Add processing time (microseconds)
    pub fn add_processing_time(&self, time_us: u64) {
        self.total_processing_time.fetch_add(time_us, Ordering::Relaxed);
    }

    /// Add connection
    pub fn add_connection(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Remove connection
    pub fn remove_connection(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Add connection error
    pub fn add_connection_error(&self) {
        self.connection_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Emit statistics to StatsD following G3Proxy pattern
    pub fn emit_stats(&self, client: &mut StatsdClient) {
        // Emit counter metrics with proper tagging
        let mut common_tags = StatsdTagGroup::default();
        common_tags.add_tag(TAG_KEY_DAEMON_GROUP, daemon_group());

        client
            .count_with_tags(METRIC_NAME_ICAP_REQUESTS_TOTAL, self.total_requests.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_REQUESTS_REQMOD, self.reqmod_requests.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_REQUESTS_RESPMOD, self.respmod_requests.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_REQUESTS_OPTIONS, self.options_requests.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_RESPONSES_SUCCESSFUL, self.successful_responses.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_RESPONSES_ERROR, self.error_responses.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_REQUESTS_BLOCKED, self.blocked_requests.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_BYTES_TOTAL, self.total_bytes.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_CONNECTIONS_TOTAL, self.total_connections.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_CONNECTIONS_ERROR, self.connection_errors.load(Ordering::Relaxed), &common_tags)
            .send();
        
        client
            .count_with_tags(METRIC_NAME_ICAP_PROCESSING_TIME_TOTAL, self.total_processing_time.load(Ordering::Relaxed), &common_tags)
            .send();

        // Emit gauge metrics
        client
            .gauge_with_tags(METRIC_NAME_ICAP_CONNECTIONS_ACTIVE, self.active_connections.load(Ordering::Relaxed), &common_tags)
            .send();

        // Emit timing metrics (average processing time)
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        if total_requests > 0 {
            let avg_processing_time = self.total_processing_time.load(Ordering::Relaxed) / total_requests;
            client
                .gauge(METRIC_NAME_ICAP_PROCESSING_TIME_AVG, avg_processing_time)
                .send();
        }
    }

    /// Get total requests
    pub fn total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    /// Get REQMOD requests
    pub fn reqmod_requests(&self) -> u64 {
        self.reqmod_requests.load(Ordering::Relaxed)
    }

    /// Get RESPMOD requests
    pub fn respmod_requests(&self) -> u64 {
        self.respmod_requests.load(Ordering::Relaxed)
    }

    /// Get OPTIONS requests
    pub fn options_requests(&self) -> u64 {
        self.options_requests.load(Ordering::Relaxed)
    }

    /// Get successful responses
    pub fn successful_responses(&self) -> u64 {
        self.successful_responses.load(Ordering::Relaxed)
    }

    /// Get error responses
    pub fn error_responses(&self) -> u64 {
        self.error_responses.load(Ordering::Relaxed)
    }

    /// Get blocked requests
    pub fn blocked_requests(&self) -> u64 {
        self.blocked_requests.load(Ordering::Relaxed)
    }

    /// Get total bytes
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes.load(Ordering::Relaxed)
    }

    /// Get active connections
    pub fn active_connections(&self) -> u64 {
        self.active_connections.load(Ordering::Relaxed)
    }

    /// Get total connections
    pub fn get_total_connections(&self) -> u64 {
        self.total_connections.load(Ordering::Relaxed)
    }

    /// Get connection errors
    pub fn get_connection_errors(&self) -> u64 {
        self.connection_errors.load(Ordering::Relaxed)
    }

    /// Get total processing time
    pub fn get_total_processing_time(&self) -> u64 {
        self.total_processing_time.load(Ordering::Relaxed)
    }

    /// Get average processing time (microseconds)
    pub fn get_avg_processing_time(&self) -> u64 {
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        if total_requests > 0 {
            self.total_processing_time.load(Ordering::Relaxed) / total_requests
        } else {
            0
        }
    }
}

impl Default for IcapStats {
    fn default() -> Self {
        Self::new()
    }
}
