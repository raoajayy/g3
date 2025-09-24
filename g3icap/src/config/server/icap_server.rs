/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! ICAP Server Configuration following G3Proxy patterns
//!
//! This module provides comprehensive configuration for ICAP servers
//! with support for audit integration, client communication, and
//! advanced server management features.

use std::time::Duration;
use std::str::FromStr;

use anyhow::Result;
use g3_types::metrics::NodeName;

use crate::opts::ProcArgs;
use crate::error::IcapError;

/// ICAP Server Configuration following G3Proxy patterns
#[derive(Debug, Clone)]
pub struct IcapServerConfig {
    /// Server name
    pub name: NodeName,
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// TLS configuration
    pub tls: bool,
    /// TLS certificate path
    pub tls_cert: Option<String>,
    /// TLS key path
    pub tls_key: Option<String>,
    /// Statistics enabled
    pub stats_enabled: bool,
    /// Statistics port
    pub stats_port: u16,
    /// Metrics enabled
    pub metrics_enabled: bool,
    /// Metrics port
    pub metrics_port: u16,
    /// Audit configuration
    pub audit_config: Option<AuditConfig>,
    /// Client configuration for server-to-server communication
    pub client_config: Option<ClientConfig>,
}

/// Audit configuration for ICAP server
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit log level
    pub log_level: String,
    /// Audit log file path
    pub log_file: Option<String>,
    /// Content filtering configuration
    pub content_filter: Option<ContentFilterConfig>,
    /// Antivirus configuration
    pub antivirus: Option<AntivirusConfig>,
}

/// Content filtering configuration
#[derive(Debug, Clone)]
pub struct ContentFilterConfig {
    /// Enable content filtering
    pub enabled: bool,
    /// Blocked keywords
    pub blocked_keywords: Vec<String>,
    /// Blocked URLs
    pub blocked_urls: Vec<String>,
    /// Blocked MIME types
    pub blocked_mime_types: Vec<String>,
    /// Maximum file size
    pub max_file_size: u64,
    /// Blocking action
    pub blocking_action: BlockingAction,
}

/// Antivirus configuration
#[derive(Debug, Clone)]
pub struct AntivirusConfig {
    /// Enable antivirus scanning
    pub enabled: bool,
    /// Antivirus engine
    pub engine: String,
    /// Maximum file size for scanning
    pub max_file_size: u64,
    /// Scan timeout
    pub scan_timeout: Duration,
    /// Quarantine enabled
    pub quarantine_enabled: bool,
    /// Quarantine directory
    pub quarantine_dir: Option<String>,
}

/// Blocking action types
#[derive(Debug, Clone)]
pub enum BlockingAction {
    /// Block the request/response
    Block,
    /// Log and allow
    Log,
    /// Redirect to a different URL
    Redirect(String),
}

/// Client configuration for server-to-server communication
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Enable client mode
    pub enabled: bool,
    /// Target ICAP server URL
    pub target_url: String,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Retry configuration
    pub retry_config: RetryConfig,
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Retry delay
    pub delay: Duration,
    /// Exponential backoff
    pub exponential_backoff: bool,
}

impl IcapServerConfig {
    /// Create a new ICAP server configuration
    pub fn new(name: NodeName) -> Self {
        Self {
            name,
            host: "0.0.0.0".to_string(),
            port: 1344,
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            tls: false,
            tls_cert: None,
            tls_key: None,
            stats_enabled: true,
            stats_port: 8080,
            metrics_enabled: true,
            metrics_port: 9090,
            audit_config: None,
            client_config: None,
        }
    }

    /// Create configuration from ProcArgs (backward compatibility)
    pub fn from_proc_args(args: ProcArgs) -> Result<Self> {
        let name = NodeName::from_str("g3icap")
            .map_err(|e| IcapError::config_simple(format!("Invalid server name: {}", e)))?;
        
        let mut config = Self::new(name);
        
        config.host = args.host;
        config.port = args.port;
        config.max_connections = args.max_connections as usize;
        config.connection_timeout = Duration::from_secs(args.connection_timeout);
        config.request_timeout = Duration::from_secs(args.request_timeout);
        config.tls = args.tls;
        config.tls_cert = args.tls_cert.map(|p| p.to_string_lossy().to_string());
        config.tls_key = args.tls_key.map(|p| p.to_string_lossy().to_string());
        config.stats_enabled = args.stats;
        config.stats_port = args.stats_port;
        config.metrics_enabled = args.metrics;
        config.metrics_port = args.metrics_port;
        
        Ok(config)
    }

    /// Get server address
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Check if TLS is enabled
    pub fn is_tls_enabled(&self) -> bool {
        self.tls
    }

    /// Get audit configuration
    pub fn audit_config(&self) -> Option<&AuditConfig> {
        self.audit_config.as_ref()
    }

    /// Get client configuration
    pub fn client_config(&self) -> Option<&ClientConfig> {
        self.client_config.as_ref()
    }

    /// Set audit configuration
    pub fn set_audit_config(&mut self, audit_config: AuditConfig) {
        self.audit_config = Some(audit_config);
    }

    /// Set client configuration
    pub fn set_client_config(&mut self, client_config: ClientConfig) {
        self.client_config = Some(client_config);
    }
}

impl Default for IcapServerConfig {
    fn default() -> Self {
        Self::new(NodeName::new_static("g3icap"))
    }
}

impl AuditConfig {
    /// Create a new audit configuration
    pub fn new() -> Self {
        Self {
            enabled: false,
            log_level: "info".to_string(),
            log_file: None,
            content_filter: None,
            antivirus: None,
        }
    }

    /// Enable audit logging
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable audit logging
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set log level
    pub fn set_log_level(&mut self, level: String) {
        self.log_level = level;
    }

    /// Set log file
    pub fn set_log_file(&mut self, file: String) {
        self.log_file = Some(file);
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentFilterConfig {
    /// Create a new content filter configuration
    pub fn new() -> Self {
        Self {
            enabled: false,
            blocked_keywords: Vec::new(),
            blocked_urls: Vec::new(),
            blocked_mime_types: Vec::new(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            blocking_action: BlockingAction::Block,
        }
    }

    /// Enable content filtering
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Add blocked keyword
    pub fn add_blocked_keyword(&mut self, keyword: String) {
        self.blocked_keywords.push(keyword);
    }

    /// Add blocked URL
    pub fn add_blocked_url(&mut self, url: String) {
        self.blocked_urls.push(url);
    }

    /// Add blocked MIME type
    pub fn add_blocked_mime_type(&mut self, mime_type: String) {
        self.blocked_mime_types.push(mime_type);
    }
}

impl Default for ContentFilterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AntivirusConfig {
    /// Create a new antivirus configuration
    pub fn new() -> Self {
        Self {
            enabled: false,
            engine: "mock".to_string(),
            max_file_size: 50 * 1024 * 1024, // 50MB
            scan_timeout: Duration::from_secs(30),
            quarantine_enabled: false,
            quarantine_dir: None,
        }
    }

    /// Enable antivirus scanning
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Set antivirus engine
    pub fn set_engine(&mut self, engine: String) {
        self.engine = engine;
    }

    /// Enable quarantine
    pub fn enable_quarantine(&mut self, dir: String) {
        self.quarantine_enabled = true;
        self.quarantine_dir = Some(dir);
    }
}

impl Default for AntivirusConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientConfig {
    /// Create a new client configuration
    pub fn new(target_url: String) -> Self {
        Self {
            enabled: false,
            target_url,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            retry_config: RetryConfig::new(),
        }
    }

    /// Enable client mode
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Set connection timeout
    pub fn set_connection_timeout(&mut self, timeout: Duration) {
        self.connection_timeout = timeout;
    }

    /// Set request timeout
    pub fn set_request_timeout(&mut self, timeout: Duration) {
        self.request_timeout = timeout;
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::new("http://localhost:1344".to_string())
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new() -> Self {
        Self {
            max_attempts: 3,
            delay: Duration::from_secs(1),
            exponential_backoff: true,
        }
    }

    /// Set maximum retry attempts
    pub fn set_max_attempts(&mut self, attempts: u32) {
        self.max_attempts = attempts;
    }

    /// Set retry delay
    pub fn set_delay(&mut self, delay: Duration) {
        self.delay = delay;
    }

    /// Enable exponential backoff
    pub fn enable_exponential_backoff(&mut self) {
        self.exponential_backoff = true;
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self::new()
    }
}