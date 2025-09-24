/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Audit module for ICAP server
//! 
//! This module provides comprehensive audit logging and compliance functionality
//! following G3Proxy patterns with detailed audit handling and statistics.

use std::sync::Arc;

use anyhow::Result;
use g3_types::metrics::NodeName;

pub mod ops;
pub mod registry;
pub mod handle;

// Re-export key types
pub use handle::{AuditHandle, AuditStats, AuditPerformanceMetrics};

/// Legacy audit handle for backward compatibility
#[derive(Debug, Clone)]
pub struct IcapAuditHandle {
    /// Audit name
    name: NodeName,
    /// Whether audit is enabled
    enabled: bool,
}

impl IcapAuditHandle {
    /// Create a new audit handle
    pub fn new(name: NodeName, enabled: bool) -> Self {
        Self { name, enabled }
    }

    /// Check if audit is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get audit name
    pub fn name(&self) -> &NodeName {
        &self.name
    }
}

/// Default audit handle (no-op)
pub static DEFAULT_AUDIT_HANDLE: IcapAuditHandle = IcapAuditHandle {
    name: g3_types::metrics::NodeName::new_static("default"),
    enabled: false,
};

/// Load all audit handlers following g3proxy patterns
pub async fn load_all() -> Result<()> {
    // Load audit configurations
    registry::load_all().await?;
    
    // Initialize audit handles
    ops::initialize_audit_handles().await?;
    
    Ok(())
}

/// Get audit handle by name
pub fn get_audit_handle(name: &NodeName) -> Option<Arc<AuditHandle>> {
    registry::get_audit_handle(name)
}

/// Get all audit handle names
pub fn get_audit_handle_names() -> Vec<NodeName> {
    registry::get_audit_handle_names()
}

/// Create a new audit handle
pub fn create_audit_handle(name: NodeName, enabled: bool) -> Arc<AuditHandle> {
    Arc::new(AuditHandle::new(name, enabled))
}
