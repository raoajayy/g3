/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Quit actor for ICAP server

use tokio::sync::broadcast;

/// Quit actor following G3Proxy pattern
pub struct QuitActor {
    sender: broadcast::Sender<()>,
}

impl QuitActor {
    /// Create a new quit actor
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1);
        Self { sender }
    }

    /// Get quit receiver
    pub fn get_receiver(&self) -> broadcast::Receiver<()> {
        self.sender.subscribe()
    }

    /// Send quit action
    pub fn send_quit(&self) {
        let _ = self.sender.send(());
    }

    /// Spawn quit actor task
    pub fn tokio_spawn_run() {
        tokio::spawn(async {
            // Quit actor implementation
            // This handles graceful shutdown signals
            
            // Listen for SIGTERM and SIGINT signals
            let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to create SIGTERM signal handler");
            let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
                .expect("Failed to create SIGINT signal handler");
            
            tokio::select! {
                _ = sigterm.recv() => {
                    println!("Received SIGTERM, initiating graceful shutdown...");
                    // Handle graceful shutdown
                    // 1. Stop accepting new connections
                    // 2. Wait for existing connections to finish
                    // 3. Clean up resources
                    // 4. Exit cleanly
                }
                _ = sigint.recv() => {
                    println!("Received SIGINT, initiating graceful shutdown...");
                    // Handle graceful shutdown
                    // Same as SIGTERM but with different logging
                }
            }
        });
    }
}

impl Default for QuitActor {
    fn default() -> Self {
        Self::new()
    }
}
