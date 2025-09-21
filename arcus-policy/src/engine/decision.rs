/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy decision structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::policy::PolicyAction;

/// Policy decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub action: PolicyAction,
    pub reason: String,
    pub policy_name: Option<String>,
    pub message: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl PolicyDecision {
    pub fn allow() -> Self {
        Self {
            action: PolicyAction::Allow,
            reason: "No blocking policies matched".to_string(),
            policy_name: None,
            message: None,
            metadata: HashMap::new(),
        }
    }

    pub fn block(reason: String, policy_name: String) -> Self {
        Self {
            action: PolicyAction::Block,
            reason,
            policy_name: Some(policy_name),
            message: None,
            metadata: HashMap::new(),
        }
    }

    pub fn warn(reason: String, policy_name: String) -> Self {
        Self {
            action: PolicyAction::Warn,
            reason,
            policy_name: Some(policy_name),
            message: None,
            metadata: HashMap::new(),
        }
    }

    pub fn inspect(reason: String, policy_name: String) -> Self {
        Self {
            action: PolicyAction::Inspect,
            reason,
            policy_name: Some(policy_name),
            message: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
