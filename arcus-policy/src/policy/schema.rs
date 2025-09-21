/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy schema definitions based on the Policy Creation Framework

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::{PolicyPriority, PolicyAction, PolicyMetadata};

/// Main security policy structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub api_version: String,
    pub kind: String,
    pub metadata: PolicyMetadata,
    pub spec: PolicySpec,
}

impl SecurityPolicy {
    pub fn new(name: String, created_by: String) -> Self {
        Self {
            api_version: "arcus.v1".to_string(),
            kind: "SecurityPolicy".to_string(),
            metadata: PolicyMetadata::new(name, created_by),
            spec: PolicySpec::default(),
        }
    }
}

/// Policy specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySpec {
    pub priority: PolicyPriority,
    pub enabled: bool,
    pub targets: PolicyTargets,
    pub url_filtering: Option<UrlFilteringPolicy>,
    pub content_security: Option<ContentSecurityPolicy>,
    pub traffic_control: Option<TrafficControlPolicy>,
    pub https_inspection: Option<HttpsInspectionPolicy>,
    pub audit: Option<AuditPolicy>,
}

impl Default for PolicySpec {
    fn default() -> Self {
        Self {
            priority: PolicyPriority::Default,
            enabled: true,
            targets: PolicyTargets::default(),
            url_filtering: None,
            content_security: None,
            traffic_control: None,
            https_inspection: None,
            audit: None,
        }
    }
}

/// Policy targeting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTargets {
    pub user_groups: Vec<String>,
    pub users: Vec<String>,
    pub source_networks: Vec<String>,
}

impl Default for PolicyTargets {
    fn default() -> Self {
        Self {
            user_groups: Vec::new(),
            users: Vec::new(),
            source_networks: Vec::new(),
        }
    }
}

/// URL filtering policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlFilteringPolicy {
    pub categories: CategoryFiltering,
    pub custom_rules: Vec<CustomRule>,
}

/// Category-based filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryFiltering {
    pub block: Vec<String>,
    pub warn: Vec<String>,
    pub allow: Vec<String>,
}

/// Custom filtering rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub name: String,
    pub action: PolicyAction,
    pub pattern: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub rule_type: RuleType,
    pub message: Option<String>,
    pub priority: Option<u32>,
}

/// Rule matching types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    Wildcard,
    Regex,
    Exact,
    Domain,
    Suffix,
}

/// Content security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSecurityPolicy {
    pub malware_scanning: Option<MalwareScanningConfig>,
    pub data_loss_prevention: Option<DataLossPreventionConfig>,
}

/// Malware scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareScanningConfig {
    pub enabled: bool,
    pub icap_server: Option<String>,
    pub action: PolicyAction,
    pub timeout: Option<String>,
}

/// Data loss prevention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLossPreventionConfig {
    pub enabled: bool,
    pub scan_uploads: bool,
    pub scan_downloads: bool,
    pub sensitive_data_patterns: Vec<SensitiveDataPattern>,
}

/// Sensitive data pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDataPattern {
    pub name: String,
    pub pattern: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub action: PolicyAction,
}

/// Traffic control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficControlPolicy {
    pub bandwidth_limits: Option<BandwidthLimits>,
    pub quotas: Option<QuotaLimits>,
    pub time_restrictions: Option<TimeRestrictions>,
}

/// Bandwidth limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimits {
    pub per_user: Option<String>,
    pub total: Option<String>,
}

/// Quota limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaLimits {
    pub daily_data_per_user: Option<String>,
    pub monthly_data_per_user: Option<String>,
}

/// Time-based restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub work_hours: Option<TimePolicy>,
    pub after_hours: Option<TimePolicy>,
}

/// Time policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePolicy {
    pub days: Vec<String>,
    pub time_range: String,
    pub timezone: String,
    pub policies: Vec<String>,
}

/// HTTPS inspection policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsInspectionPolicy {
    pub enabled: bool,
    pub mode: HttpsMode,
    pub certificate_generation: CertificateGeneration,
    pub ca_certificate: Option<String>,
    pub ca_private_key: Option<String>,
    pub bypass_domains: Vec<String>,
    pub inspect_domains: Vec<String>,
}

/// HTTPS inspection modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpsMode {
    Mitm,
    Passthrough,
    Selective,
}

/// Certificate generation modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateGeneration {
    Automatic,
    Manual,
    Hybrid,
}

/// Audit policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditPolicy {
    pub enabled: bool,
    pub log_level: LogLevel,
    pub retention: String,
    pub export_targets: Vec<ExportTarget>,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Minimal,
    Standard,
    Detailed,
    Verbose,
}

/// Export target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTarget {
    pub target_type: ExportType,
    pub endpoint: String,
    pub format: Option<String>,
    pub authentication: Option<ExportAuth>,
}

/// Export types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportType {
    Syslog,
    Json,
    Elasticsearch,
    Splunk,
}

/// Export authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAuth {
    pub auth_type: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}
