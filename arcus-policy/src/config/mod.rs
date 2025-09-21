/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Configuration generation for G3proxy

use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod generator;
pub mod g3proxy;
pub mod escaper;
pub mod user_group;

pub use generator::ConfigGenerator;
pub use g3proxy::G3proxyConfig;
pub use escaper::EscaperConfig;
pub use user_group::UserGroupConfig;

/// Configuration generation context
#[derive(Debug, Clone)]
pub struct ConfigContext {
    pub organization: String,
    pub proxy_instances: u32,
    pub load_balancer: Option<String>,
    pub database: String,
    pub cache: Option<String>,
    pub tls_version: String,
    pub certificate_authority: String,
    pub key_rotation: String,
}

impl Default for ConfigContext {
    fn default() -> Self {
        Self {
            organization: "company".to_string(),
            proxy_instances: 3,
            load_balancer: Some("nginx".to_string()),
            database: "sqlite".to_string(),
            cache: Some("redis".to_string()),
            tls_version: "1.2+".to_string(),
            certificate_authority: "internal".to_string(),
            key_rotation: "90d".to_string(),
        }
    }
}

/// Generated configuration components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedConfig {
    pub runtime: RuntimeConfig,
    pub log: LogConfig,
    pub stat: StatConfig,
    pub resolver: Vec<ResolverConfig>,
    pub escaper: Vec<EscaperConfig>,
    pub user_group: Vec<UserGroupConfig>,
    pub auditor: Vec<AuditorConfig>,
    pub server: Vec<ServerConfig>,
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub thread_number: u32,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: Option<String>,
    pub output: Vec<LogOutput>,
}

/// Log output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOutput {
    pub output_type: String,
    pub path: Option<String>,
    pub level: Option<String>,
    pub max_size: Option<String>,
    pub max_files: Option<u32>,
    pub facility: Option<String>,
    pub tag: Option<String>,
}

/// Statistics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatConfig {
    pub target: StatTarget,
}

/// Statistics target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatTarget {
    pub udp: String,
}

/// Resolver configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverConfig {
    pub name: String,
    pub resolver_type: String,
    pub server: Vec<String>,
}

/// Auditor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditorConfig {
    pub name: String,
    pub protocol_inspection: HashMap<String, serde_yaml::Value>,
    pub tls_cert_generator: HashMap<String, serde_yaml::Value>,
    pub tls_ticketer: HashMap<String, serde_yaml::Value>,
    pub tls_stream_dump: HashMap<String, serde_yaml::Value>,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub server_type: String,
    pub escaper: String,
    pub auditor: Option<String>,
    pub user_group: Option<String>,
    pub listen: ServerListen,
    pub tls_client: Option<HashMap<String, serde_yaml::Value>>,
    pub tls_server: Option<TlsServerConfig>,
}

/// Server listen configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerListen {
    pub address: String,
}

/// TLS server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsServerConfig {
    pub cert_pairs: CertPair,
}

/// Certificate pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertPair {
    pub certificate: String,
    pub private_key: String,
}
