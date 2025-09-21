/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::anyhow;
use foldhash::fast::FixedState;
use g3_types::metrics::NodeName;

use super::user_group::UserGroupConfig;

static REGISTRY: Mutex<HashMap<NodeName, UserGroupConfig, FixedState>> =
    Mutex::new(HashMap::with_hasher(FixedState::with_seed(0)));

pub(crate) fn add(config: UserGroupConfig, _replace: bool) -> anyhow::Result<()> {
    let mut registry = REGISTRY.lock().unwrap();
    let name = config.name().clone();
    if registry.contains_key(&name) {
        return Err(anyhow!("user group {name} already exists"));
    }
    registry.insert(name, config);
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn get(name: &NodeName) -> Option<UserGroupConfig> {
    let registry = REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}

#[allow(dead_code)]
pub(crate) fn get_all() -> Vec<(NodeName, UserGroupConfig)> {
    let registry = REGISTRY.lock().unwrap();
    registry.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[allow(dead_code)]
pub(crate) fn clear() {
    let mut registry = REGISTRY.lock().unwrap();
    registry.clear();
}
