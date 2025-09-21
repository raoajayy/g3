/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Escaper configuration for G3proxy

use serde::{Deserialize, Serialize};

/// Escaper configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EscaperConfig {
    pub name: String,
    pub escaper_type: String,
    pub rules: Vec<RoutingRule>,
    pub default_next: String,
    pub resolver: Option<String>,
    pub upstream: Option<String>,
    pub icap_service: Option<String>,
    pub next: Option<String>,
    pub message: Option<String>,
    pub exact_match: Option<Vec<ExactMatchRule>>,
    pub regex_match: Option<Vec<RegexMatchRule>>,
    pub child_match: Option<Vec<ChildMatchRule>>,
    pub subnet_match: Option<Vec<SubnetMatchRule>>,
    pub suffix_match: Option<Vec<SuffixMatchRule>>,
}

/// Routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    pub rule_type: String,
    pub pattern: Option<String>,
    pub next: String,
    pub priority: u32,
}

/// Exact match rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExactMatchRule {
    pub hosts: Vec<String>,
    pub next: String,
}

/// Regex match rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatchRule {
    pub pattern: String,
    pub next: String,
}

/// Child match rule (wildcard/domain matching)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildMatchRule {
    pub domains: Vec<String>,
    pub next: String,
}

/// Subnet match rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetMatchRule {
    pub subnets: Vec<String>,
    pub next: String,
}

/// Suffix match rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuffixMatchRule {
    pub suffixes: Vec<String>,
    pub next: String,
}
