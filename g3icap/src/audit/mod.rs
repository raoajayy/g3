/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Audit module for ICAP server
//! 
//! This module provides audit logging and compliance functionality
//! following G3Proxy patterns.

pub mod ops;
pub mod registry;

use g3_types::metrics::NodeName;

/// Audit handle for ICAP operations
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

/// Load all audit handlers
pub async fn load_all() -> anyhow::Result<()> {
    // For now, just return Ok as we don't have complex audit loading
    Ok(())
}
