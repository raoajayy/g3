/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use tokio::sync::Mutex;
use std::future::Future;

mod quit;
pub use quit::QuitActor;

mod upgrade;
pub use upgrade::UpgradeActor;

mod local;
pub use local::{DaemonController, UniqueController};

#[allow(dead_code)]
static IO_MUTEX: Mutex<Option<Mutex<()>>> = Mutex::const_new(Some(Mutex::const_new(())));

#[allow(dead_code)]
pub(crate) async fn run_protected_io<F: Future>(future: F) -> Option<F::Output> {
    let outer = IO_MUTEX.lock().await;
    if let Some(inner) = &*outer {
        // io tasks that should avoid corrupt at exit should hold this lock
        let _guard = inner.lock();
        Some(future.await)
    } else {
        None
    }
}

#[allow(dead_code)]
pub(crate) async fn disable_protected_io() {
    let mut outer = IO_MUTEX.lock().await;
    if let Some(inner) = outer.take() {
        // wait all inner lock finish
        let _ = inner.lock().await;
    }
}

/// Spawn working thread for control operations
pub async fn spawn_working_thread() -> anyhow::Result<tokio::task::JoinHandle<()>> {
    let handle = tokio::spawn(async {
        // Control thread implementation
        // This handles background control operations
        
        // Start daemon controller
        let daemon_controller = match DaemonController::start().await {
            Ok(controller) => controller,
            Err(e) => {
                eprintln!("Failed to start daemon controller: {}", e);
                return;
            }
        };
        
        // Start unique controller
        let unique_controller = match UniqueController::start().await {
            Ok(controller) => controller,
            Err(e) => {
                eprintln!("Failed to start unique controller: {}", e);
                return;
            }
        };
        
        // Run both controllers concurrently
        tokio::select! {
            _ = daemon_controller.run() => {
                println!("Daemon controller stopped");
            }
            _ = unique_controller.run() => {
                println!("Unique controller stopped");
            }
        }
    });
    Ok(handle)
}
