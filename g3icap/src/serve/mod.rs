/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Serve module implementation
use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use crate::config::server::AnyServerConfig;

/// Server registry following G3Proxy pattern
pub struct ServerRegistry {
    inner: HashMap<NodeName, Arc<dyn ServerInternal>, FixedState>,
}

impl ServerRegistry {
    pub const fn new() -> Self {
        ServerRegistry {
            inner: HashMap::with_hasher(FixedState::with_seed(0)),
        }
    }

    pub fn add(&mut self, name: NodeName, server: Arc<dyn ServerInternal>) -> anyhow::Result<()> {
        // For now, just add the server without starting runtime
        // The runtime will be started by the main server
        if let Some(old_server) = self.inner.insert(name, server) {
            old_server._abort_runtime();
        }
        Ok(())
    }

    pub fn del(&mut self, name: &NodeName) {
        if let Some(old_server) = self.inner.remove(name) {
            old_server._abort_runtime();
        }
    }

    pub fn get_names(&self) -> HashSet<NodeName> {
        self.inner.keys().cloned().collect()
    }

    pub fn get(&self, name: &NodeName) -> Option<Arc<dyn ServerInternal>> {
        self.inner.get(name).cloned()
    }
}

/// Server internal trait following G3Proxy pattern
pub trait ServerInternal: Send + Sync {
    fn _clone_config(&self) -> AnyServerConfig;
    fn _depend_on_server(&self, name: &NodeName) -> bool;
    fn _reload_config_notify_runtime(&self);
    fn _update_next_servers_in_place(&self);
    fn _update_escaper_in_place(&self);
    fn _update_user_group_in_place(&self);
    fn _update_audit_handle_in_place(&self) -> anyhow::Result<()>;
    fn _reload_with_old_notifier(
        &self,
        config: AnyServerConfig,
        registry: &mut ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>>;
    fn _reload_with_new_notifier(
        &self,
        config: AnyServerConfig,
        registry: &mut ServerRegistry,
    ) -> anyhow::Result<Arc<dyn ServerInternal>>;
    fn _start_runtime(&self, server: Arc<dyn g3_daemon::server::BaseServer>) -> anyhow::Result<()>;
    fn _abort_runtime(&self);
}

/// Global server registry
static RUNTIME_SERVER_REGISTRY: Mutex<ServerRegistry> = Mutex::new(ServerRegistry::new());

/// Get global server registry
pub fn get_global_server_registry() -> &'static Mutex<ServerRegistry> {
    &RUNTIME_SERVER_REGISTRY
}

/// Spawn offline cleanup task
pub fn spawn_offline_clean() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        interval.tick().await;
        loop {
            // Clean up offline servers
            interval.tick().await;
        }
    });
}

/// Spawn all servers
pub async fn spawn_all() -> anyhow::Result<()> {
    use crate::server::IcapServer;
    
    // Get the parsed command line arguments
    let proc_args = crate::opts::ProcArgs::parse().unwrap_or_else(|| {
        crate::opts::ProcArgs {
            daemon_config: g3_daemon::opts::DaemonArgs::new("g3icap"),
            config: None,
            port: 1344,
            host: "0.0.0.0".to_string(),
            max_connections: 1000,
            connection_timeout: 30,
            request_timeout: 60,
            tls: false,
            tls_cert: None,
            tls_key: None,
            stats: true,
            stats_port: 8080,
            metrics: true,
            metrics_port: 9090,
        }
    });
    
    // Create and start ICAP server
    let mut icap_server = IcapServer::new(proc_args)
        .map_err(|e| anyhow::anyhow!("Failed to create ICAP server: {}", e))?;
    
    // Spawn server in background task
    tokio::spawn(async move {
        if let Err(e) = icap_server.start().await {
            eprintln!("ICAP Server error: {}", e);
        }
    });
    
    println!("✅ G3ICAP Server spawned successfully");
    
    Ok(())
}
