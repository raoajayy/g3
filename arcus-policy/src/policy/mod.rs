/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy management and data structures

use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod schema;
pub mod manager;
pub mod validator;

pub use schema::*;
pub use manager::PolicyManager;
pub use validator::PolicyValidator;

/// Policy priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyPriority {
    Critical = 1000,
    High = 800,
    Medium = 500,
    Low = 200,
    Default = 100,
}

impl Default for PolicyPriority {
    fn default() -> Self {
        Self::Default
    }
}

/// Policy action types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow,
    Block,
    Warn,
    Inspect,
    Quarantine,
    Log,
}

/// Policy status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyStatus {
    Active,
    Inactive,
    Draft,
    Deprecated,
}

/// Core policy metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub tags: Vec<String>,
    pub status: PolicyStatus,
}

impl PolicyMetadata {
    pub fn new(name: String, created_by: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            version: "1.0".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
            created_by,
            tags: Vec::new(),
            status: PolicyStatus::Draft,
        }
    }
}

/// Policy ID type
pub type PolicyId = Uuid;

/// Policy reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRef {
    pub id: PolicyId,
    pub name: String,
    pub version: String,
    pub priority: PolicyPriority,
}

/// Policy collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCollection {
    pub policies: HashMap<PolicyId, Arc<SecurityPolicy>>,
    pub metadata: PolicyMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PolicyCollection {
    pub fn new(name: String, created_by: String) -> Self {
        let now = Utc::now();
        Self {
            policies: HashMap::new(),
            metadata: PolicyMetadata::new(name, created_by),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_policy(&mut self, policy: SecurityPolicy) -> PolicyId {
        let id = Uuid::new_v4();
        self.policies.insert(id, Arc::new(policy));
        self.updated_at = Utc::now();
        id
    }

    pub fn get_policy(&self, id: &PolicyId) -> Option<&Arc<SecurityPolicy>> {
        self.policies.get(id)
    }

    pub fn remove_policy(&mut self, id: &PolicyId) -> Option<Arc<SecurityPolicy>> {
        let result = self.policies.remove(id);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
}
