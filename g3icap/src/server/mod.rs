/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::sync::Arc;

use async_trait::async_trait;
use slog::Logger;
use tokio::net::TcpStream;

use g3_daemon::listen::{AcceptTcpServer, ListenStats};
use g3_daemon::server::{BaseServer, ClientConnectionInfo, ReloadServer};
use g3_types::metrics::NodeName;
use std::str::FromStr;

use crate::error::IcapResult;
use crate::log::server::{get_logger, ServerEvent};
use crate::opts::ProcArgs;
use crate::stat::get_global_stats;
use crate::serve::ServerInternal;

pub mod connection;
pub mod handler;
pub mod listener;

/// ICAP Server following G3Proxy architecture
pub struct IcapServer {
    /// Server configuration
    config: ProcArgs,
    /// Server statistics
    server_stats: Arc<crate::stats::IcapStats>,
    /// Listen statistics
    listen_stats: Arc<ListenStats>,
    /// Task logger
    task_logger: Option<Logger>,
    /// Reload version
    reload_version: usize,
}

impl IcapServer {
    /// Create a new ICAP server
    pub fn new(config: ProcArgs) -> IcapResult<Self> {
        let server_stats = get_global_stats().unwrap_or_else(|| Arc::new(crate::stats::IcapStats::new()));
        let node_name = NodeName::from_str("g3icap").unwrap();
        let listen_stats = Arc::new(ListenStats::new(&node_name));
        
        Ok(Self {
            config,
            server_stats,
            listen_stats,
            task_logger: None,
            reload_version: 1,
        })
    }

    /// Start the ICAP server using simple tokio listener (following G3Proxy pattern)
    pub async fn start(&mut self) -> IcapResult<()> {
        let logger = get_logger("main").unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });
        
        ServerEvent::Started.log(&logger, "Starting G3 ICAP Server");

        // Create listen address
        let listen_addr = format!("{}:{}", self.config.host, self.config.port);

        // Start listening using tokio directly (simpler approach)
        let listener = tokio::net::TcpListener::bind(&listen_addr)
            .await
            .map_err(|e| crate::error::IcapError::Network(format!("Failed to bind to {}: {}", listen_addr, e)))?;

        println!("DEBUG: ICAP Server listening on {}", listen_addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    println!("DEBUG: New connection from {}", peer_addr);
                    self.server_stats.increment_connections();
                    
                    // Handle connection in a separate task
                    let stats = self.server_stats.clone();
                    let logger = self.task_logger.clone().unwrap_or_else(|| {
                        slog::Logger::root(slog::Discard, slog::o!())
                    });
                    
                    tokio::spawn(async move {
                        let mut connection = crate::server::connection::IcapConnection::new(
                            stream,
                            peer_addr,
                            stats,
                            logger,
                        );

                        if let Err(e) = connection.process().await {
                            println!("DEBUG: Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    println!("DEBUG: Failed to accept connection: {}", e);
                    self.server_stats.increment_errors();
                }
            }
        }
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
            reload_version: self.reload_version + 1,
        }
    }
}

impl ServerInternal for IcapServer {
    fn _clone_config(&self) -> crate::config::server::AnyServerConfig {
        // For now, return a dummy config
        crate::config::server::AnyServerConfig::Icap(self.config.clone())
    }

    fn _depend_on_server(&self, _name: &NodeName) -> bool {
        false
    }

    fn _reload_config_notify_runtime(&self) {
        // No-op for now
    }

    fn _update_next_servers_in_place(&self) {
        // No-op for now
    }

    fn _update_escaper_in_place(&self) {
        // No-op for now
    }

    fn _update_user_group_in_place(&self) {
        // No-op for now
    }

    fn _update_audit_handle_in_place(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn _reload_with_old_notifier(
        &self,
        _config: crate::config::server::AnyServerConfig,
        _registry: &mut crate::serve::ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>> {
        Ok(Arc::new(self.reload()))
    }

    fn _reload_with_new_notifier(
        &self,
        _config: crate::config::server::AnyServerConfig,
        _registry: &mut crate::serve::ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>> {
        Ok(Arc::new(self.reload()))
    }

    fn _start_runtime(&self, _server: Arc<dyn BaseServer>) -> anyhow::Result<()> {
        // This will be called by the server registry
        // For now, we handle the runtime in the start() method
        Ok(())
    }

    fn _abort_runtime(&self) {
        // No-op for now
    }
}

#[async_trait]
impl AcceptTcpServer for IcapServer {
    async fn run_tcp_task(&self, stream: TcpStream, cc_info: ClientConnectionInfo) {
        let client_addr = cc_info.client_addr();
        self.server_stats.increment_connections();
        
        // Create connection handler
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
            println!("DEBUG: Connection error: {}", e);
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
            reload_version: self.reload_version,
        }
    }
}