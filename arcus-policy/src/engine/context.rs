/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy evaluation context

use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Policy evaluation context
#[derive(Debug, Clone)]
pub struct PolicyContext {
    pub user_id: Option<String>,
    pub user_groups: Vec<String>,
    pub source_ip: String,
    pub source_network: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<String>,
    pub request_id: String,
    pub metadata: HashMap<String, String>,
}

impl PolicyContext {
    pub fn new() -> Self {
        Self {
            user_id: None,
            user_groups: Vec::new(),
            source_ip: "0.0.0.0".to_string(),
            source_network: None,
            timestamp: Utc::now(),
            session_id: None,
            request_id: uuid::Uuid::new_v4().to_string(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_groups(mut self, groups: Vec<String>) -> Self {
        self.user_groups = groups;
        self
    }

    pub fn with_source_ip(mut self, ip: String) -> Self {
        self.source_ip = ip;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
