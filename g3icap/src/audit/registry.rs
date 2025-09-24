/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Audit registry for ICAP server following g3proxy patterns
//!
//! This module provides comprehensive audit registry functionality
//! with handle management and configuration tracking.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use super::ops::IcapAuditOps;
use super::handle::AuditHandle;
use crate::config::audit::AuditorConfig;

/// Audit registry following G3Proxy patterns
pub struct IcapAuditRegistry {
    inner: Mutex<HashMap<NodeName, Arc<dyn IcapAuditOps + Send + Sync>, FixedState>>,
}

impl IcapAuditRegistry {
    /// Create a new audit registry
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::with_hasher(FixedState::with_seed(0))),
        }
    }

    /// Add audit handler
    pub fn add(&self, name: NodeName, handler: Arc<dyn IcapAuditOps + Send + Sync>) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.insert(name, handler);
        }
    }

    /// Get audit handler
    pub fn get(&self, name: &NodeName) -> Option<Arc<dyn IcapAuditOps + Send + Sync>> {
        self.inner
            .lock()
            .ok()
            .and_then(|inner| inner.get(name).cloned())
    }
}

/// Global audit registry
static AUDIT_REGISTRY: IcapAuditRegistry = IcapAuditRegistry::new();

/// Global audit configuration registry
static AUDIT_CONFIG_REGISTRY: Mutex<HashMap<NodeName, Arc<AuditorConfig>, foldhash::fast::FixedState>> = Mutex::new(HashMap::with_hasher(foldhash::fast::FixedState::with_seed(0)));

/// Global audit handle registry
static AUDIT_HANDLE_REGISTRY: Mutex<HashMap<NodeName, Arc<AuditHandle>, foldhash::fast::FixedState>> = Mutex::new(HashMap::with_hasher(foldhash::fast::FixedState::with_seed(0)));

/// Get global audit registry
pub fn get_global_audit_registry() -> &'static IcapAuditRegistry {
    &AUDIT_REGISTRY
}

/// Load all audit configurations following g3proxy patterns
pub async fn load_all() -> Result<()> {
    // Load audit configurations from config system
    // In a real implementation, this would load from YAML files
    // For now, we'll create some default configurations
    
    let default_configs = vec![
        ("content_filter", AuditorConfig::new(None)),
        ("antivirus", AuditorConfig::new(None)),
    ];
    
    for (name_str, config) in default_configs {
        let name = NodeName::new_static(name_str);
        register_audit_config(name, config)?;
    }
    
    Ok(())
}

/// Get audit configuration by name
pub fn get_audit_config(name: &NodeName) -> Option<Arc<AuditorConfig>> {
    AUDIT_CONFIG_REGISTRY.lock().unwrap().get(name).cloned()
}

/// Get all audit configurations
pub async fn get_all_audit_configs() -> Result<HashMap<NodeName, Arc<AuditorConfig>>> {
    let registry = AUDIT_CONFIG_REGISTRY.lock().unwrap();
    let mut configs = HashMap::new();
    
    for (name, config) in registry.iter() {
        configs.insert(name.clone(), config.clone());
    }
    
    Ok(configs)
}

/// Get all audit configuration names
pub fn get_audit_config_names() -> Vec<NodeName> {
    AUDIT_CONFIG_REGISTRY.lock().unwrap().keys().cloned().collect()
}

/// Register audit configuration
pub fn register_audit_config(name: NodeName, config: AuditorConfig) -> Result<()> {
    let mut registry = AUDIT_CONFIG_REGISTRY.lock().unwrap();
    registry.insert(name, Arc::new(config));
    Ok(())
}

/// Get audit handle by name
pub fn get_audit_handle(name: &NodeName) -> Option<Arc<AuditHandle>> {
    AUDIT_HANDLE_REGISTRY.lock().unwrap().get(name).cloned()
}

/// Get all audit handles
pub fn get_all_audit_handles() -> HashMap<NodeName, Arc<AuditHandle>, foldhash::fast::FixedState> {
    AUDIT_HANDLE_REGISTRY.lock().unwrap().clone()
}

/// Get all audit handle names
pub fn get_audit_handle_names() -> Vec<NodeName> {
    AUDIT_HANDLE_REGISTRY.lock().unwrap().keys().cloned().collect()
}

/// Register audit handle
pub fn register_audit_handle(name: NodeName, handle: Arc<AuditHandle>) -> Result<()> {
    let mut registry = AUDIT_HANDLE_REGISTRY.lock().unwrap();
    registry.insert(name, handle);
    Ok(())
}

/// Clear all audit configurations
pub fn clear_all_configs() {
    let mut registry = AUDIT_CONFIG_REGISTRY.lock().unwrap();
    registry.clear();
}

/// Clear all audit handles
pub fn clear_all_handles() {
    let mut registry = AUDIT_HANDLE_REGISTRY.lock().unwrap();
    registry.clear();
}

/// Clear all audit data
pub fn clear_all() {
    clear_all_configs();
    clear_all_handles();
}
