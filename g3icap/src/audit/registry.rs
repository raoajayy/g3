/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Audit registry for ICAP server

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use super::ops::IcapAuditOps;

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

/// Get global audit registry
pub fn get_global_audit_registry() -> &'static IcapAuditRegistry {
    &AUDIT_REGISTRY
}
