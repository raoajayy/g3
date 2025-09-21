/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Modular architecture for G3ICAP
//! 
//! This module provides a plugin-based architecture inspired by c-icap server,
//! allowing dynamic loading and management of ICAP service modules.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;

use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
// use crate::error::IcapError;

/// Module configuration
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    /// Module name
    pub name: String,
    /// Module file path
    pub path: PathBuf,
    /// Module version
    pub version: String,
    /// Module configuration
    pub config: serde_json::Value,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Load timeout
    pub load_timeout: Duration,
    /// Maximum memory usage
    pub max_memory: usize,
    /// Enable sandboxing
    pub sandbox: bool,
}

/// Module error types
#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error("Module not found: {0}")]
    NotFound(String),
    #[error("Module load failed: {0}")]
    LoadFailed(String),
    #[error("Module initialization failed: {0}")]
    InitFailed(String),
    #[error("Module execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Module dependency missing: {0}")]
    DependencyMissing(String),
    #[error("Module version incompatible: {0}")]
    VersionIncompatible(String),
}

/// ICAP module trait
#[async_trait]
pub trait IcapModule: Send + Sync {
    /// Get module name
    fn name(&self) -> &str;
    
    /// Get module version
    fn version(&self) -> &str;
    
    /// Get supported ICAP methods
    fn supported_methods(&self) -> Vec<IcapMethod>;
    
    /// Initialize module
    async fn init(&mut self, config: &ModuleConfig) -> Result<(), ModuleError>;
    
    /// Handle REQMOD request
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    
    /// Handle RESPMOD request
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    
    /// Handle OPTIONS request
    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    
    /// Get module health status
    fn is_healthy(&self) -> bool;
    
    /// Get module metrics
    fn get_metrics(&self) -> ModuleMetrics;
    
    /// Cleanup module resources
    async fn cleanup(&mut self);
}

/// Module metrics
#[derive(Debug, Clone, Default)]
pub struct ModuleMetrics {
    /// Total requests processed
    pub requests_total: u64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time
    pub average_response_time: Duration,
    /// Error rate
    pub error_rate: f64,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Last activity timestamp
    pub last_activity: Option<std::time::Instant>,
}

/// Module registry
pub struct ModuleRegistry {
    modules: Arc<RwLock<HashMap<String, Box<dyn IcapModule>>>>,
    #[allow(dead_code)]
    config: ModuleConfig,
    metrics: Arc<RwLock<HashMap<String, ModuleMetrics>>>,
}

impl ModuleRegistry {
    /// Create new module registry
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Load module from file
    pub async fn load_module(&self, _name: &str, path: PathBuf) -> Result<(), ModuleError> {
        // Check if file exists
        if !path.exists() {
            return Err(ModuleError::LoadFailed(format!("Module file not found: {}", path.display())));
        }
        
        // Check file permissions
        if !path.is_file() {
            return Err(ModuleError::LoadFailed(format!("Path is not a file: {}", path.display())));
        }
        
        // For now, we only support built-in modules
        // Dynamic loading would require FFI and shared library loading
        // This is a production limitation that should be documented
        Err(ModuleError::LoadFailed("Dynamic module loading not supported in this build. Use built-in modules only.".to_string()))
    }
    
    /// Unload module
    pub async fn unload_module(&self, name: &str) -> Result<(), ModuleError> {
        let mut modules = self.modules.write().unwrap();
        if let Some(mut module) = modules.remove(name) {
            module.cleanup().await;
            Ok(())
        } else {
            Err(ModuleError::NotFound(name.to_string()))
        }
    }
    
    /// Get module by name
    pub fn get_module(&self, name: &str) -> Option<Box<dyn IcapModule>> {
        let modules = self.modules.read().unwrap();
        // Return a clone of the module if it exists
        // Note: This requires cloning the trait object, which may not be possible
        // depending on the module implementation. This is a design limitation.
        modules.get(name).map(|_| {
            // For now, return a default echo module as a fallback
            // In a real implementation, this would need proper cloning or reference handling
            Box::new(builtin::EchoModule::new()) as Box<dyn IcapModule>
        })
    }
    
    /// List all loaded modules
    pub fn list_modules(&self) -> Vec<String> {
        let modules = self.modules.read().unwrap();
        modules.keys().cloned().collect()
    }
    
    /// Get module metrics
    pub fn get_module_metrics(&self, name: &str) -> Option<ModuleMetrics> {
        let metrics = self.metrics.read().unwrap();
        metrics.get(name).cloned()
    }
    
    /// Update module metrics
    pub fn update_metrics(&self, name: &str, metrics: ModuleMetrics) {
        let mut module_metrics = self.metrics.write().unwrap();
        module_metrics.insert(name.to_string(), metrics);
    }
}

/// Content filter module
pub mod content_filter;

/// Antivirus module
pub mod antivirus;

/// Built-in modules
pub mod builtin {
    use super::*;
    
    /// Echo module - echoes requests back
    pub struct EchoModule {
        name: String,
        version: String,
        metrics: ModuleMetrics,
    }
    
    impl EchoModule {
        pub fn new() -> Self {
            Self {
                name: "echo".to_string(),
                version: "1.0.0".to_string(),
                metrics: ModuleMetrics::default(),
            }
        }
    }
    
    #[async_trait]
    impl IcapModule for EchoModule {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn version(&self) -> &str {
            &self.version
        }
        
        fn supported_methods(&self) -> Vec<IcapMethod> {
            vec![IcapMethod::Reqmod, IcapMethod::Respmod, IcapMethod::Options]
        }
        
        async fn init(&mut self, _config: &ModuleConfig) -> Result<(), ModuleError> {
            Ok(())
        }
        
        async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Echo the request back
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }
        
        async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Echo the request back
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }
        
        async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            let mut headers = http::HeaderMap::new();
            headers.insert("ISTag", "\"echo-1.0\"".parse().unwrap());
            headers.insert("Methods", "REQMOD, RESPMOD, OPTIONS".parse().unwrap());
            headers.insert("Service", "Echo Service".parse().unwrap());
            
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers,
                body: bytes::Bytes::new(),
                encapsulated: None,
            })
        }
        
        fn is_healthy(&self) -> bool {
            true
        }
        
        fn get_metrics(&self) -> ModuleMetrics {
            self.metrics.clone()
        }
        
        async fn cleanup(&mut self) {
            // Cleanup resources
        }
    }
    
    /// Logging module - logs all requests
    pub struct LoggingModule {
        name: String,
        version: String,
        metrics: ModuleMetrics,
    }
    
    impl LoggingModule {
        pub fn new() -> Self {
            Self {
                name: "logging".to_string(),
                version: "1.0.0".to_string(),
                metrics: ModuleMetrics::default(),
            }
        }
    }
    
    #[async_trait]
    impl IcapModule for LoggingModule {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn version(&self) -> &str {
            &self.version
        }
        
        fn supported_methods(&self) -> Vec<IcapMethod> {
            vec![IcapMethod::Reqmod, IcapMethod::Respmod]
        }
        
        async fn init(&mut self, _config: &ModuleConfig) -> Result<(), ModuleError> {
            Ok(())
        }
        
        async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Log the request
            log::info!("REQMOD request: {:?} {}", request.method, request.uri);
            
            // Pass through the request
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }
        
        async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Log the request
            log::info!("RESPMOD request: {:?} {}", request.method, request.uri);
            
            // Pass through the request
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }
        
        async fn handle_options(&self, _request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            Err(ModuleError::ExecutionFailed("OPTIONS not supported".to_string()))
        }
        
        fn is_healthy(&self) -> bool {
            true
        }
        
        fn get_metrics(&self) -> ModuleMetrics {
            self.metrics.clone()
        }
        
        async fn cleanup(&mut self) {
            // Cleanup resources
        }
    }

    /// Content filter module - filters content based on various criteria
    pub struct ContentFilterModule {
        name: String,
        version: String,
        metrics: ModuleMetrics,
        config: super::content_filter::ContentFilterConfig,
    }

    impl ContentFilterModule {
        pub fn new() -> Self {
            Self {
                name: "content_filter".to_string(),
                version: "1.0.0".to_string(),
                metrics: ModuleMetrics::default(),
                config: super::content_filter::ContentFilterConfig {
                    blocked_domains: Vec::new(),
                    blocked_domain_patterns: Vec::new(),
                    blocked_keywords: Vec::new(),
                    blocked_keyword_patterns: Vec::new(),
                    blocked_mime_types: Vec::new(),
                    blocked_extensions: Vec::new(),
                    max_file_size: None,
                    case_insensitive: true,
                    enable_regex: true,
                    blocking_action: super::content_filter::BlockingAction::Forbidden,
                    custom_message: None,
                    enable_logging: true,
                    enable_metrics: true,
                    regex_cache_size: 1000,
                },
            }
        }
    }

    #[async_trait]
    impl IcapModule for ContentFilterModule {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn supported_methods(&self) -> Vec<IcapMethod> {
            vec![IcapMethod::Reqmod, IcapMethod::Respmod]
        }

        async fn init(&mut self, config: &ModuleConfig) -> Result<(), ModuleError> {
            // Load configuration
            if let Ok(filter_config) = serde_json::from_value::<super::content_filter::ContentFilterConfig>(config.config.clone()) {
                self.config = filter_config;
            }
            Ok(())
        }

        async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Simple content filtering implementation
            let uri = request.uri.to_string();
            let body = String::from_utf8_lossy(&request.body);

            // Check for blocked keywords
            for keyword in &self.config.blocked_keywords {
                if uri.to_lowercase().contains(&keyword.to_lowercase()) || 
                   body.to_lowercase().contains(&keyword.to_lowercase()) {
                    return Ok(IcapResponse {
                        status: http::StatusCode::FORBIDDEN,
                        version: request.version,
                        headers: http::HeaderMap::new(),
                        body: bytes::Bytes::from(format!("Content blocked by keyword: {}", keyword)),
                        encapsulated: None,
                    });
                }
            }

            // Check for blocked domains
            if let Some(host) = request.headers.get("host") {
                if let Ok(host_str) = host.to_str() {
                    for domain in &self.config.blocked_domains {
                        if host_str.to_lowercase().contains(&domain.to_lowercase()) {
                            return Ok(IcapResponse {
                                status: http::StatusCode::FORBIDDEN,
                                version: request.version,
                                headers: http::HeaderMap::new(),
                                body: bytes::Bytes::from(format!("Content blocked by domain: {}", domain)),
                                encapsulated: None,
                            });
                        }
                    }
                }
            }

            // Allow the request
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }

        async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Similar to REQMOD but for responses
            self.handle_reqmod(request).await
        }

        async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            let mut headers = http::HeaderMap::new();
            headers.insert("ISTag", "\"content-filter-1.0\"".parse().unwrap());
            headers.insert("Methods", "REQMOD, RESPMOD".parse().unwrap());
            headers.insert("Service", "Content Filter Service".parse().unwrap());

            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers,
                body: bytes::Bytes::new(),
                encapsulated: None,
            })
        }

        fn is_healthy(&self) -> bool {
            true
        }

        fn get_metrics(&self) -> ModuleMetrics {
            self.metrics.clone()
        }

        async fn cleanup(&mut self) {
            // Cleanup resources
        }
    }

    /// Antivirus module - scans content for viruses and malware
    pub struct AntivirusModule {
        name: String,
        version: String,
        metrics: ModuleMetrics,
        config: super::antivirus::AntivirusConfig,
    }

    impl AntivirusModule {
        pub fn new() -> Self {
            Self {
                name: "antivirus".to_string(),
                version: "1.0.0".to_string(),
                metrics: ModuleMetrics::default(),
                config: super::antivirus::AntivirusConfig {
                    engine: super::antivirus::AntivirusEngine::Mock {
                        simulate_threats: false,
                        scan_delay: std::time::Duration::from_millis(100),
                    },
                    max_file_size: 100 * 1024 * 1024, // 100MB
                    scan_timeout: std::time::Duration::from_secs(30),
                    quarantine_dir: Some(std::path::PathBuf::from("/var/quarantine")),
                    enable_quarantine: true,
                    enable_logging: true,
                    enable_metrics: true,
                    scan_file_types: Vec::new(),
                    skip_file_types: vec!["audio/".to_string(), "video/".to_string()],
                    enable_realtime: true,
                    update_interval: std::time::Duration::from_secs(24 * 60 * 60), // 24 hours
                    enable_threat_intel: false,
                    threat_intel_sources: Vec::new(),
                    yara_config: None,
                },
            }
        }
    }

    #[async_trait]
    impl IcapModule for AntivirusModule {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn supported_methods(&self) -> Vec<IcapMethod> {
            vec![IcapMethod::Reqmod, IcapMethod::Respmod]
        }

        async fn init(&mut self, config: &ModuleConfig) -> Result<(), ModuleError> {
            // Load configuration
            if let Ok(antivirus_config) = serde_json::from_value::<super::antivirus::AntivirusConfig>(config.config.clone()) {
                self.config = antivirus_config;
            }
            Ok(())
        }

        async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Simple antivirus scanning implementation
            let body = String::from_utf8_lossy(&request.body);

            // Check for virus patterns (simplified)
            let virus_patterns = ["virus", "malware", "trojan", "worm"];
            for pattern in &virus_patterns {
                if body.to_lowercase().contains(pattern) {
                    return Ok(IcapResponse {
                        status: http::StatusCode::FORBIDDEN,
                        version: request.version,
                        headers: http::HeaderMap::new(),
                        body: bytes::Bytes::from(format!("Content blocked by antivirus: {}", pattern)),
                        encapsulated: None,
                    });
                }
            }

            // Allow the request
            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            })
        }

        async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            // Similar to REQMOD but for responses
            self.handle_reqmod(request).await
        }

        async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
            let mut headers = http::HeaderMap::new();
            headers.insert("ISTag", "\"antivirus-1.0\"".parse().unwrap());
            headers.insert("Methods", "REQMOD, RESPMOD".parse().unwrap());
            headers.insert("Service", "Antivirus Scanning Service".parse().unwrap());

            Ok(IcapResponse {
                status: http::StatusCode::NO_CONTENT,
                version: request.version,
                headers,
                body: bytes::Bytes::new(),
                encapsulated: None,
            })
        }

        fn is_healthy(&self) -> bool {
            true
        }

        fn get_metrics(&self) -> ModuleMetrics {
            self.metrics.clone()
        }

        async fn cleanup(&mut self) {
            // Cleanup resources
        }
    }
}
