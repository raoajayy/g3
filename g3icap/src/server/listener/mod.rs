//! ICAP Server Listener
//!
//! This module handles the main server listener that accepts incoming connections.

use crate::error::{IcapError, IcapResult};
use crate::log::server::{get_logger, ServerEvent};
use crate::stats::IcapStats;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// ICAP Server Listener
#[derive(Clone)]
pub struct IcapListener {
    /// Bind address
    addr: SocketAddr,
    /// Statistics collector
    stats: Arc<IcapStats>,
}

impl IcapListener {
    /// Create a new ICAP listener
    pub fn new(
        host: String,
        port: u16,
        stats: Arc<IcapStats>,
    ) -> IcapResult<Self> {
        let addr = format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .map_err(|e| IcapError::network_simple(format!("Invalid address: {}", e)))?;

        Ok(Self {
            addr,
            stats,
        })
    }

    /// Start listening for connections
    pub async fn start(&self) -> IcapResult<()> {
        let listener = TcpListener::bind(self.addr)
            .await
            .map_err(|e| IcapError::network_simple(format!("Failed to bind to {}: {}", self.addr, e)))?;

        let logger = get_logger("main").unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });
        ServerEvent::Started.log(&logger, &format!("ICAP Server listening on {}", self.addr));

        loop {
            println!("DEBUG: Waiting for connections...");
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    println!("DEBUG: New connection from {}", peer_addr);
                    ServerEvent::ServiceRegistered.log(&logger, &format!("New connection from {}", peer_addr));
                    
                    // Handle connection in a separate task
                    let stats = self.stats.clone();
                    let listener = self.clone();
                    
                    println!("DEBUG: Spawning connection handler task");
                    tokio::spawn(async move {
                        println!("DEBUG: Connection handler task started");
                        if let Err(e) = listener.handle_connection(stream, peer_addr, stats).await {
                            println!("DEBUG: Connection error: {}", e);
                            let error_logger = get_logger("error").unwrap_or_else(|| {
                                slog::Logger::root(slog::Discard, slog::o!())
                            });
                            ServerEvent::Error.log(&error_logger, &format!("Connection error from {}: {}", peer_addr, e));
                        } else {
                            println!("DEBUG: Connection handled successfully");
                        }
                    });
                }
                Err(e) => {
                    let error_logger = get_logger("error").unwrap_or_else(|| {
                        slog::Logger::root(slog::Discard, slog::o!())
                    });
                    ServerEvent::Error.log(&error_logger, &format!("Failed to accept connection: {}", e));
                }
            }
        }
    }

    /// Handle a single connection
    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        peer_addr: SocketAddr,
        stats: Arc<IcapStats>,
    ) -> IcapResult<()> {
        let connection_id = format!("{}", peer_addr);
        let logger = get_logger(&connection_id).unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });

        // Create connection handler
        let mut connection = crate::server::connection::IcapConnection::new(stream, peer_addr, stats, logger);
        
        // Process the connection
        connection.process().await?;
        
        Ok(())
    }
}
