/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Authentication registry for ICAP server

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use super::ops::IcapAuthOps;

/// Authentication registry following G3Proxy patterns
pub struct IcapAuthRegistry {
    inner: Mutex<HashMap<NodeName, Arc<dyn IcapAuthOps + Send + Sync>, FixedState>>,
}

impl IcapAuthRegistry {
    /// Create a new authentication registry
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::with_hasher(FixedState::with_seed(0))),
        }
    }

    /// Add authentication handler
    pub fn add(&self, name: NodeName, handler: Arc<dyn IcapAuthOps + Send + Sync>) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.insert(name, handler);
        }
    }

    /// Get authentication handler
    pub fn get(&self, name: &NodeName) -> Option<Arc<dyn IcapAuthOps + Send + Sync>> {
        self.inner
            .lock()
            .ok()
            .and_then(|inner| inner.get(name).cloned())
    }
}

/// Global authentication registry
static AUTH_REGISTRY: IcapAuthRegistry = IcapAuthRegistry::new();

/// Get global authentication registry
pub fn get_global_auth_registry() -> &'static IcapAuthRegistry {
    &AUTH_REGISTRY
}
