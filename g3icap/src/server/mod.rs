/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use slog::Logger;
use tokio::net::TcpStream;
use tokio::time::Instant;

use g3_daemon::listen::{AcceptTcpServer, ListenStats};
use g3_daemon::server::{BaseServer, ClientConnectionInfo, ReloadServer, ServerQuitPolicy};
use g3_types::metrics::NodeName;
use std::str::FromStr;

use crate::error::IcapResult;
use crate::log::server::{get_logger, ServerEvent};
use crate::opts::ProcArgs;
use crate::stat::get_global_stats;
use crate::serve::ServerInternal;
use crate::audit::{AuditHandle, get_audit_handle};
use crate::config::server::icap_server::IcapServerConfig;

pub mod connection;
pub mod handler;
pub mod listener;

/// ICAP Server following G3Proxy architecture
pub struct IcapServer {
    /// Server configuration
    config: IcapServerConfig,
    /// Server statistics
    server_stats: Arc<crate::stats::IcapStats>,
    /// Listen statistics
    listen_stats: Arc<ListenStats>,
    /// Task logger
    task_logger: Option<Logger>,
    /// Audit handle for ICAP operations
    audit_handle: Option<Arc<AuditHandle>>,
    /// Reload version
    reload_version: usize,
    /// Server quit policy
    quit_policy: Arc<ServerQuitPolicy>,
    /// Server start time
    start_time: Instant,
}

impl IcapServer {
    /// Create a new ICAP server from ProcArgs (backward compatibility)
    pub fn new(config: ProcArgs) -> IcapResult<Self> {
        let icap_config = IcapServerConfig::from_proc_args(config)?;
        Self::new_with_config(icap_config)
    }

    /// Create a new ICAP server with configuration
    pub fn new_with_config(config: IcapServerConfig) -> IcapResult<Self> {
        let server_stats = get_global_stats().unwrap_or_else(|| Arc::new(crate::stats::IcapStats::new()));
        let node_name = NodeName::from_str("g3icap").unwrap();
        let listen_stats = Arc::new(ListenStats::new(&node_name));
        let quit_policy = Arc::new(ServerQuitPolicy::default());
        
        // Get audit handle if available
        let audit_handle = get_audit_handle(&node_name);
        
        Ok(Self {
            config,
            server_stats,
            listen_stats,
            task_logger: None,
            audit_handle,
            reload_version: 1,
            quit_policy,
            start_time: Instant::now(),
        })
    }

    /// Get server configuration
    pub fn config(&self) -> &IcapServerConfig {
        &self.config
    }

    /// Get server statistics
    pub fn server_stats(&self) -> &Arc<crate::stats::IcapStats> {
        &self.server_stats
    }

    /// Get listen statistics
    pub fn listen_stats(&self) -> &Arc<ListenStats> {
        &self.listen_stats
    }

    /// Get audit handle
    pub fn audit_handle(&self) -> Option<&Arc<AuditHandle>> {
        self.audit_handle.as_ref()
    }

    /// Get server uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if server should quit
    pub fn should_quit(&self) -> bool {
        // For now, always return false (server runs indefinitely)
        // In a real implementation, this would check the quit policy
        false
    }

    /// Get alive connection count
    pub fn alive_count(&self) -> i32 {
        // For now, return a placeholder
        // In a real implementation, this would track active connections
        0
    }

    /// Start the ICAP server using G3Proxy patterns
    pub async fn start(&mut self) -> IcapResult<()> {
        let logger = get_logger("main").unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });
        
        ServerEvent::Started.log(&logger, "Starting G3 ICAP Server");

        // Create listen address
        let listen_addr = format!("{}:{}", self.config.host, self.config.port);

        // Start listening using tokio directly
        let listener = tokio::net::TcpListener::bind(&listen_addr)
            .await
            .map_err(|e| crate::error::IcapError::network_simple(format!("Failed to bind to {}: {}", listen_addr, e)))?;

        slog::info!(logger, "ICAP Server listening on {}", listen_addr);

        // Main server loop following G3Proxy patterns
        loop {
            // Check if server should quit
            if self.should_quit() {
                slog::info!(logger, "Server quit requested, shutting down");
                break;
            }

            // Accept connections with timeout
            match tokio::time::timeout(Duration::from_secs(1), listener.accept()).await {
                Ok(Ok((stream, peer_addr))) => {
                    slog::debug!(logger, "New connection from {}", peer_addr);
                    self.server_stats.increment_connections();
                    
                    // Handle connection in a separate task
                    let stats = self.server_stats.clone();
                    let audit_handle = self.audit_handle.clone();
                    let config = self.config.clone();
                    let logger = self.task_logger.clone().unwrap_or_else(|| {
                        slog::Logger::root(slog::Discard, slog::o!())
                    });
                    
                    tokio::spawn(async move {
                        let mut connection = crate::server::connection::IcapConnection::new(
                            stream,
                            peer_addr,
                            stats,
                            logger.clone(),
                        );

                        if let Err(e) = connection.process().await {
                            slog::debug!(logger, "Connection error: {}", e);
                        }
                    });
                }
                Ok(Err(e)) => {
                    slog::error!(logger, "Failed to accept connection: {}", e);
                    self.server_stats.increment_errors();
                }
                Err(_) => {
                    // Timeout, continue loop
                    continue;
                }
            }
        }

        Ok(())
    }
}

impl BaseServer for IcapServer {
    fn name(&self) -> &NodeName {
        static NAME: std::sync::OnceLock<NodeName> = std::sync::OnceLock::new();
        NAME.get_or_init(|| NodeName::from_str("g3icap").unwrap())
    }

    fn r#type(&self) -> &'static str {
        "icap"
    }

    fn version(&self) -> usize {
        self.reload_version
    }
}

impl ReloadServer for IcapServer {
    fn reload(&self) -> Self {
        Self {
            config: self.config.clone(),
            server_stats: self.server_stats.clone(),
            listen_stats: self.listen_stats.clone(),
            task_logger: self.task_logger.clone(),
            audit_handle: self.audit_handle.clone(),
            reload_version: self.reload_version + 1,
            quit_policy: self.quit_policy.clone(),
            start_time: self.start_time,
        }
    }
}

impl ServerInternal for IcapServer {
    fn _clone_config(&self) -> crate::config::server::AnyServerConfig {
        crate::config::server::AnyServerConfig::Icap(self.config.clone())
    }

    fn _depend_on_server(&self, _name: &NodeName) -> bool {
        false
    }

    fn _reload_config_notify_runtime(&self) {
        // Notify runtime of configuration changes
        // In a real implementation, this would update runtime state
    }

    fn _update_next_servers_in_place(&self) {
        // Update next servers in place
        // For ICAP server, this is typically not applicable
    }

    fn _update_escaper_in_place(&self) {
        // Update escaper in place
        // For ICAP server, this is typically not applicable
    }

    fn _update_user_group_in_place(&self) {
        // Update user group in place
        // For ICAP server, this is typically not applicable
    }

    fn _update_audit_handle_in_place(&self) -> anyhow::Result<()> {
        // Update audit handle in place
        // This would reload audit configuration if needed
        Ok(())
    }

    fn _reload_with_old_notifier(
        &self,
        config: crate::config::server::AnyServerConfig,
        _registry: &mut crate::serve::ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>> {
        match config {
            crate::config::server::AnyServerConfig::Icap(icap_config) => {
                let new_server = Self::new_with_config(icap_config)?;
                Ok(Arc::new(new_server))
            }
            _ => Err(anyhow::anyhow!("Invalid config type for ICAP server")),
        }
    }

    fn _reload_with_new_notifier(
        &self,
        config: crate::config::server::AnyServerConfig,
        _registry: &mut crate::serve::ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>> {
        match config {
            crate::config::server::AnyServerConfig::Icap(icap_config) => {
                let new_server = Self::new_with_config(icap_config)?;
                Ok(Arc::new(new_server))
            }
            _ => Err(anyhow::anyhow!("Invalid config type for ICAP server")),
        }
    }

    fn _start_runtime(&self, _server: Arc<dyn BaseServer>) -> anyhow::Result<()> {
        // Start runtime for the server
        // This is called by the server registry
        Ok(())
    }

    fn _abort_runtime(&self) {
        // Abort runtime for the server
        // This would clean up resources and stop the server
    }
}

#[async_trait]
impl AcceptTcpServer for IcapServer {
    async fn run_tcp_task(&self, stream: TcpStream, cc_info: ClientConnectionInfo) {
        let client_addr = cc_info.client_addr();
        self.server_stats.increment_connections();
        
        // Create connection handler following G3Proxy patterns
        let mut connection = crate::server::connection::IcapConnection::new(
            stream,
            client_addr,
            self.server_stats.clone(),
            self.task_logger.clone().unwrap_or_else(|| {
                slog::Logger::root(slog::Discard, slog::o!())
            }),
        );

        // Process the connection
        if let Err(e) = connection.process().await {
            slog::debug!(self.task_logger.as_ref().unwrap_or(&slog::Logger::root(slog::Discard, slog::o!())), 
                "Connection error: {}", e);
            self.server_stats.increment_errors();
        }
    }
}

impl Clone for IcapServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            server_stats: self.server_stats.clone(),
            listen_stats: self.listen_stats.clone(),
            task_logger: self.task_logger.clone(),
            audit_handle: self.audit_handle.clone(),
            reload_version: self.reload_version,
            quit_policy: self.quit_policy.clone(),
            start_time: self.start_time,
        }
    }
}