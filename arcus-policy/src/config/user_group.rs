/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! User group configuration for G3proxy

use serde::{Deserialize, Serialize};

/// User group configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupConfig {
    pub name: String,
    pub source: UserGroupSource,
    pub description: Option<String>,
}

/// User group source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupSource {
    pub source_type: String,
    pub path: String,
}

/// User definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub groups: Vec<String>,
    pub bandwidth_limit: Option<String>,
    pub daily_quota: Option<String>,
    pub bypass_filtering: Option<bool>,
}

/// User group definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
    pub description: String,
    pub default_policy: String,
}

/// User configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfigFile {
    pub users: Vec<User>,
    pub groups: std::collections::HashMap<String, UserGroup>,
}
