/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Upgrade actor for ICAP server

/// Upgrade actor following G3Proxy pattern
pub struct UpgradeActor;

impl UpgradeActor {
    /// Connect to old daemon during upgrade
    pub fn connect_to_old_daemon() {
        // Implementation for connecting to old daemon during upgrade
        // This handles graceful shutdown of the old daemon process
        // and ensures smooth transition during service updates
        
        // Check for existing daemon PID file
        let pid_file = std::path::Path::new("/var/run/g3icap.pid");
        if pid_file.exists() {
            // Read PID from file
            if let Ok(pid_str) = std::fs::read_to_string(pid_file) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    // Check if process is still running
                    if is_process_running(pid) {
                        // Send SIGTERM to old daemon
                        if let Err(e) = std::process::Command::new("kill")
                            .arg("-TERM")
                            .arg(pid.to_string())
                            .output()
                        {
                            eprintln!("Failed to send SIGTERM to old daemon: {}", e);
                        } else {
                            // Wait for graceful shutdown
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            
                            // Check if process is still running
                            if is_process_running(pid) {
                                // Force kill if still running
                                let _ = std::process::Command::new("kill")
                                    .arg("-KILL")
                                    .arg(pid.to_string())
                                    .output();
                            }
                        }
                    }
                }
            }
            
            // Clean up PID file
            let _ = std::fs::remove_file(pid_file);
        }
    }
    

    /// Handle upgrade action
    pub async fn handle_upgrade(&self) -> anyhow::Result<()> {
        // Handle upgrade actions
        Ok(())
    }
}

/// Check if process is running
fn is_process_running(pid: u32) -> bool {
    // Check if process exists by sending signal 0
    std::process::Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
