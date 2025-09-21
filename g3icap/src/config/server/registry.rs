/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::collections::HashMap;
use std::sync::Mutex;

use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use super::AnyServerConfig;

static REGISTRY: Mutex<HashMap<NodeName, AnyServerConfig, FixedState>> =
    Mutex::new(HashMap::with_hasher(FixedState::with_seed(0)));

pub(crate) fn add(config: AnyServerConfig) -> Option<AnyServerConfig> {
    let mut registry = REGISTRY.lock().unwrap();
    let name = match &config {
        AnyServerConfig::Icap(_proc_args) => NodeName::new_static("g3icap"),
    };
    registry.insert(name, config)
}

#[allow(dead_code)]
pub(crate) fn get(name: &NodeName) -> Option<AnyServerConfig> {
    let registry = REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}

#[allow(dead_code)]
pub(crate) fn get_all() -> Vec<(NodeName, AnyServerConfig)> {
    let registry = REGISTRY.lock().unwrap();
    registry.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[allow(dead_code)]
pub(crate) fn del(name: &NodeName) -> Option<AnyServerConfig> {
    let mut registry = REGISTRY.lock().unwrap();
    registry.remove(name)
}

#[allow(dead_code)]
pub(crate) fn clear() {
    let mut registry = REGISTRY.lock().unwrap();
    registry.clear();
}
