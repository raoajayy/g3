//! Signal handling for G3 ICAP Server
//!
//! This module handles system signals for graceful shutdown and reload.

use crate::error::IcapResult;
use slog::{info, warn, Logger, o};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;

/// Signal handler for graceful shutdown
pub struct SignalHandler {
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Logger
    logger: Logger,
}

impl SignalHandler {
    /// Create a new signal handler
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(AtomicBool::new(false)),
            logger: slog::Logger::root(slog::Discard, o!()),
        }
    }

    /// Setup signal handling
    pub fn setup(&self) -> IcapResult<()> {
        let shutdown = self.shutdown.clone();
        let logger = self.logger.clone();

        // Handle SIGTERM and SIGINT
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    info!(logger, "Received SIGINT, initiating graceful shutdown");
                    shutdown.store(true, Ordering::Relaxed);
                }
                Err(err) => {
                    warn!(logger, "Failed to listen for SIGINT"; "error" => %err);
                }
            }
        });

        // Handle SIGTERM on Unix systems
        #[cfg(unix)]
        {
            let shutdown = self.shutdown.clone();
            let logger = self.logger.clone();
            
            tokio::spawn(async move {
                use tokio::signal::unix::{signal, SignalKind};
                
                let mut sigterm = match signal(SignalKind::terminate()) {
                    Ok(sig) => sig,
                    Err(err) => {
                        warn!(logger, "Failed to listen for SIGTERM"; "error" => %err);
                        return;
                    }
                };

                if sigterm.recv().await.is_some() {
                    info!(logger, "Received SIGTERM, initiating graceful shutdown");
                    shutdown.store(true, Ordering::Relaxed);
                }
            });
        }

        Ok(())
    }

    /// Check if shutdown is requested
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown.load(Ordering::Relaxed)
    }

    /// Wait for shutdown signal
    pub async fn wait_for_shutdown(&self) {
        while !self.is_shutdown_requested() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

/// Register signal handlers following G3Proxy pattern
pub fn register() -> anyhow::Result<()> {
    // Signal registration is handled by g3_daemon
    Ok(())
}
