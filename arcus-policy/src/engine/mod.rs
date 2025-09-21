/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy engine for evaluating and enforcing policies

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use tracing::{debug, info, warn};

pub mod evaluator;
pub mod context;
pub mod decision;

pub use evaluator::PolicyEvaluator;
pub use context::PolicyContext;
pub use decision::PolicyDecision;

/// Policy engine for evaluating requests against policies
pub struct PolicyEngine {
    evaluator: PolicyEvaluator,
    context: PolicyContext,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            evaluator: PolicyEvaluator::new(),
            context: PolicyContext::new(),
        }
    }

    /// Evaluate a request against all applicable policies
    pub async fn evaluate_request(&self, request: &PolicyRequest) -> Result<PolicyDecision> {
        debug!("Evaluating request: {} {}", request.method, request.url);
        
        // Get applicable policies for this request
        let applicable_policies = self.get_applicable_policies(request).await?;
        
        if applicable_policies.is_empty() {
            return Ok(PolicyDecision::Allow);
        }

        // Evaluate each policy in priority order
        for policy in applicable_policies {
            let decision = self.evaluator.evaluate_policy(policy, request).await?;
            
            // If policy explicitly blocks or allows, return that decision
            match decision.action {
                PolicyAction::Block => {
                    info!("Request blocked by policy: {}", policy.metadata.name);
                    return Ok(decision);
                }
                PolicyAction::Allow => {
                    info!("Request allowed by policy: {}", policy.metadata.name);
                    return Ok(decision);
                }
                _ => {
                    // Continue evaluating other policies
                    debug!("Policy {} returned action: {:?}, continuing evaluation", 
                           policy.metadata.name, decision.action);
                }
            }
        }

        // Default decision if no policy explicitly allows or blocks
        Ok(PolicyDecision::Allow)
    }

    /// Get policies applicable to this request
    async fn get_applicable_policies(&self, request: &PolicyRequest) -> Result<Vec<&Arc<SecurityPolicy>>> {
        let mut applicable = Vec::new();
        
        // This is a simplified implementation
        // In a real system, you'd query the policy manager for applicable policies
        // based on user, group, source network, etc.
        
        debug!("Found {} applicable policies for request", applicable.len());
        Ok(applicable)
    }
}

/// Policy request structure
#[derive(Debug, Clone)]
pub struct PolicyRequest {
    pub method: String,
    pub url: String,
    pub user: Option<String>,
    pub user_groups: Vec<String>,
    pub source_ip: String,
    pub user_agent: Option<String>,
    pub headers: HashMap<String, String>,
}

impl PolicyRequest {
    pub fn new(method: String, url: String, source_ip: String) -> Self {
        Self {
            method,
            url,
            user: None,
            user_groups: Vec::new(),
            source_ip,
            user_agent: None,
            headers: HashMap::new(),
        }
    }
}

/// Policy action (re-exported from policy module)
use crate::policy::PolicyAction;
use crate::policy::SecurityPolicy;
