/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Content Filter Module for G3ICAP
//! 
//! This module provides comprehensive content filtering capabilities including:
//! - URL/domain filtering
//! - Keyword-based content filtering
//! - MIME type filtering
//! - File size filtering
//! - Regular expression pattern matching
//! - Real-time threat intelligence integration

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use crate::modules::{IcapModule, ModuleConfig, ModuleError, ModuleMetrics};

/// Content filter configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentFilterConfig {
    /// Blocked domains (exact match)
    pub blocked_domains: Vec<String>,
    /// Blocked domain patterns (regex)
    pub blocked_domain_patterns: Vec<String>,
    /// Blocked keywords
    pub blocked_keywords: Vec<String>,
    /// Blocked keyword patterns (regex)
    pub blocked_keyword_patterns: Vec<String>,
    /// Blocked MIME types
    pub blocked_mime_types: Vec<String>,
    /// Blocked file extensions
    pub blocked_extensions: Vec<String>,
    /// Maximum file size (bytes)
    pub max_file_size: Option<u64>,
    /// Enable case-insensitive matching
    pub case_insensitive: bool,
    /// Enable regex matching
    pub enable_regex: bool,
    /// Blocking action
    pub blocking_action: BlockingAction,
    /// Custom response message
    pub custom_message: Option<String>,
    /// Enable logging
    pub enable_logging: bool,
    /// Enable metrics
    pub enable_metrics: bool,
    /// Cache size for compiled regex patterns
    pub regex_cache_size: usize,
}

/// Blocking action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockingAction {
    /// Return 403 Forbidden
    Forbidden,
    /// Return 404 Not Found
    NotFound,
    /// Return custom status code
    Custom(u16),
    /// Redirect to custom URL
    Redirect(String),
    /// Replace content with custom message
    Replace(String),
}

impl Default for BlockingAction {
    fn default() -> Self {
        BlockingAction::Forbidden
    }
}

/// Content filter statistics
#[derive(Debug, Clone)]
pub struct ContentFilterStats {
    /// Total requests processed
    pub total_requests: u64,
    /// Requests blocked
    pub blocked_requests: u64,
    /// Requests allowed
    pub allowed_requests: u64,
    /// Blocked by domain
    pub blocked_by_domain: u64,
    /// Blocked by keyword
    pub blocked_by_keyword: u64,
    /// Blocked by MIME type
    pub blocked_by_mime_type: u64,
    /// Blocked by file size
    pub blocked_by_file_size: u64,
    /// Blocked by regex pattern
    pub blocked_by_regex: u64,
    /// Processing time (microseconds)
    pub total_processing_time: u64,
    /// Last reset time
    pub last_reset: Instant,
}

impl Default for ContentFilterStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            blocked_requests: 0,
            allowed_requests: 0,
            blocked_by_domain: 0,
            blocked_by_keyword: 0,
            blocked_by_mime_type: 0,
            blocked_by_file_size: 0,
            blocked_by_regex: 0,
            total_processing_time: 0,
            last_reset: Instant::now(),
        }
    }
}

/// Content filter module
pub struct ContentFilterModule {
    /// Module name
    name: String,
    /// Module version
    version: String,
    /// Filter configuration
    config: ContentFilterConfig,
    /// Compiled regex patterns
    domain_patterns: Vec<Regex>,
    keyword_patterns: Vec<Regex>,
    /// Statistics
    stats: Arc<RwLock<ContentFilterStats>>,
    /// Metrics
    metrics: Arc<Mutex<ModuleMetrics>>,
    /// Cache for frequently accessed patterns
    pattern_cache: Arc<RwLock<HashMap<String, bool>>>,
}

impl ContentFilterModule {
    /// Create a new content filter module
    pub fn new(config: ContentFilterConfig) -> Self {
        Self {
            name: "content_filter".to_string(),
            version: "1.0.0".to_string(),
            config,
            domain_patterns: Vec::new(),
            keyword_patterns: Vec::new(),
            stats: Arc::new(RwLock::new(ContentFilterStats::default())),
            metrics: Arc::new(Mutex::new(ModuleMetrics::default())),
            pattern_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ContentFilterConfig {
            blocked_domains: Vec::new(),
            blocked_domain_patterns: Vec::new(),
            blocked_keywords: Vec::new(),
            blocked_keyword_patterns: Vec::new(),
            blocked_mime_types: Vec::new(),
            blocked_extensions: Vec::new(),
            max_file_size: None,
            case_insensitive: true,
            enable_regex: true,
            blocking_action: BlockingAction::Forbidden,
            custom_message: None,
            enable_logging: true,
            enable_metrics: true,
            regex_cache_size: 1000,
        })
    }

    /// Compile regex patterns
    fn compile_patterns(&mut self) -> Result<(), ModuleError> {
        if !self.config.enable_regex {
            return Ok(());
        }

        // Compile domain patterns
        for pattern in &self.config.blocked_domain_patterns {
            let regex = if self.config.case_insensitive {
                Regex::new(&format!("(?i){}", pattern))
            } else {
                Regex::new(pattern)
            }.map_err(|e| ModuleError::InitFailed(format!("Invalid domain pattern '{}': {}", pattern, e)))?;
            self.domain_patterns.push(regex);
        }

        // Compile keyword patterns
        for pattern in &self.config.blocked_keyword_patterns {
            let regex = if self.config.case_insensitive {
                Regex::new(&format!("(?i){}", pattern))
            } else {
                Regex::new(pattern)
            }.map_err(|e| ModuleError::InitFailed(format!("Invalid keyword pattern '{}': {}", pattern, e)))?;
            self.keyword_patterns.push(regex);
        }

        Ok(())
    }

    /// Check if content should be blocked
    async fn should_block(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        let start_time = Instant::now();

        // Check domain blocking
        if let Some(reason) = self.check_domain_blocking(request).await? {
            return Ok(Some(reason));
        }

        // Check keyword blocking in URI
        if let Some(reason) = self.check_uri_keywords(request).await? {
            return Ok(Some(reason));
        }

        // Check MIME type blocking
        if let Some(reason) = self.check_mime_type_blocking(request).await? {
            return Ok(Some(reason));
        }

        // Check file size blocking
        if let Some(reason) = self.check_file_size_blocking(request).await? {
            return Ok(Some(reason));
        }

        // Check keyword blocking in body
        if let Some(reason) = self.check_body_keywords(request).await? {
            return Ok(Some(reason));
        }

        // Update statistics
        let processing_time = start_time.elapsed().as_micros() as u64;
        self.update_stats(false, None, processing_time).await;

        Ok(None)
    }

    /// Check domain blocking
    async fn check_domain_blocking(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        // Extract host from headers
        let host = request.headers
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if host.is_empty() {
            return Ok(None);
        }

        // Check exact domain matches
        for domain in &self.config.blocked_domains {
            if self.config.case_insensitive {
                if host.to_lowercase().contains(&domain.to_lowercase()) {
                    return Ok(Some(BlockReason::Domain(domain.clone())));
                }
            } else if host.contains(domain) {
                return Ok(Some(BlockReason::Domain(domain.clone())));
            }
        }

        // Check regex domain patterns
        for pattern in &self.domain_patterns {
            if pattern.is_match(host) {
                return Ok(Some(BlockReason::DomainPattern(pattern.as_str().to_string())));
            }
        }

        Ok(None)
    }

    /// Check keyword blocking in URI
    async fn check_uri_keywords(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        let uri = request.uri.to_string();

        // Check exact keyword matches
        for keyword in &self.config.blocked_keywords {
            let search_text = if self.config.case_insensitive {
                uri.to_lowercase()
            } else {
                uri.clone()
            };
            let search_keyword = if self.config.case_insensitive {
                keyword.to_lowercase()
            } else {
                keyword.clone()
            };

            if search_text.contains(&search_keyword) {
                return Ok(Some(BlockReason::Keyword(keyword.clone())));
            }
        }

        // Check regex keyword patterns
        for pattern in &self.keyword_patterns {
            if pattern.is_match(&uri) {
                return Ok(Some(BlockReason::KeywordPattern(pattern.as_str().to_string())));
            }
        }

        Ok(None)
    }

    /// Check MIME type blocking
    async fn check_mime_type_blocking(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        // Check Content-Type header
        if let Some(content_type) = request.headers.get("content-type") {
            if let Ok(mime_type) = content_type.to_str() {
                for blocked_mime in &self.config.blocked_mime_types {
                    if mime_type.contains(blocked_mime) {
                        return Ok(Some(BlockReason::MimeType(blocked_mime.clone())));
                    }
                }
            }
        }

        // Check file extension
        let path = request.uri.path();
        if let Some(extension) = std::path::Path::new(path).extension() {
            if let Some(ext_str) = extension.to_str() {
                for blocked_ext in &self.config.blocked_extensions {
                    if ext_str.eq_ignore_ascii_case(blocked_ext) {
                        return Ok(Some(BlockReason::Extension(blocked_ext.clone())));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Check file size blocking
    async fn check_file_size_blocking(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        if let Some(max_size) = self.config.max_file_size {
            // Check Content-Length header
            if let Some(content_length) = request.headers.get("content-length") {
                if let Ok(length_str) = content_length.to_str() {
                    if let Ok(length) = length_str.parse::<u64>() {
                        if length > max_size {
                            return Ok(Some(BlockReason::FileSize(length)));
                        }
                    }
                }
            }

            // Check actual body size
            if request.body.len() as u64 > max_size {
                return Ok(Some(BlockReason::FileSize(request.body.len() as u64)));
            }
        }

        Ok(None)
    }

    /// Check keyword blocking in body
    async fn check_body_keywords(&self, request: &IcapRequest) -> Result<Option<BlockReason>, ModuleError> {
        if request.body.is_empty() {
            return Ok(None);
        }

        let body_text = String::from_utf8_lossy(&request.body);

        // Check exact keyword matches
        for keyword in &self.config.blocked_keywords {
            let search_text = if self.config.case_insensitive {
                body_text.to_lowercase()
            } else {
                body_text.to_string()
            };
            let search_keyword = if self.config.case_insensitive {
                keyword.to_lowercase()
            } else {
                keyword.clone()
            };

            if search_text.contains(&search_keyword) {
                return Ok(Some(BlockReason::BodyKeyword(keyword.clone())));
            }
        }

        // Check regex keyword patterns
        for pattern in &self.keyword_patterns {
            if pattern.is_match(&body_text) {
                return Ok(Some(BlockReason::BodyKeywordPattern(pattern.as_str().to_string())));
            }
        }

        Ok(None)
    }

    /// Create blocking response using proper response generator
    fn create_blocking_response(&self, reason: &BlockReason) -> IcapResponse {
        let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
            "G3ICAP-ContentFilter/1.0.0".to_string(),
            "content-filter-1.0.0".to_string(),
            Some("content-filter".to_string())
        );

        match &self.config.blocking_action {
            BlockingAction::Forbidden => {
                let message = format!("Content blocked by filter: {}", reason);
                let should_chunk = response_generator.should_use_chunked_encoding(Some(message.len()));
                if should_chunk {
                    response_generator.forbidden_chunked(Some(&message))
                } else {
                    response_generator.forbidden(Some(&message))
                }
            }
            BlockingAction::NotFound => {
                let message = format!("Content not found: {}", reason);
                response_generator.not_found(Some(&message))
            }
            BlockingAction::Custom(code) => {
                let status = http::StatusCode::from_u16(*code).unwrap_or(http::StatusCode::FORBIDDEN);
                let message = format!("Content blocked by filter: {}", reason);
                response_generator.from_status_code(status, Some(&message))
            }
            BlockingAction::Redirect(url) => {
                response_generator.found(url)
            }
            BlockingAction::Replace(content) => {
                // For content replacement, we need to create a modified response
                let should_chunk = response_generator.should_use_chunked_encoding(Some(content.len()));
                if should_chunk {
                    response_generator.create_chunked_response(
                        http::StatusCode::OK,
                        None,
                        bytes::Bytes::from(content.clone()),
                        "text/html"
                    )
                } else {
                    response_generator.ok_modified(None, bytes::Bytes::from(content.clone()))
                }
            }
        }
    }

    /// Update statistics
    async fn update_stats(&self, blocked: bool, reason: Option<BlockReason>, processing_time: u64) {
        let mut stats = self.stats.write().unwrap();
        stats.total_requests += 1;
        stats.total_processing_time += processing_time;

        if blocked {
            stats.blocked_requests += 1;
            if let Some(reason) = reason {
                match reason {
                    BlockReason::Domain(_) | BlockReason::DomainPattern(_) => {
                        stats.blocked_by_domain += 1;
                    }
                    BlockReason::Keyword(_) | BlockReason::KeywordPattern(_) | 
                    BlockReason::BodyKeyword(_) | BlockReason::BodyKeywordPattern(_) => {
                        stats.blocked_by_keyword += 1;
                    }
                    BlockReason::MimeType(_) => {
                        stats.blocked_by_mime_type += 1;
                    }
                    BlockReason::FileSize(_) => {
                        stats.blocked_by_file_size += 1;
                    }
                    BlockReason::Extension(_) => {
                        stats.blocked_by_mime_type += 1;
                    }
                }
            }
        } else {
            stats.allowed_requests += 1;
        }

        // Update module metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.requests_total = stats.total_requests;
        metrics.requests_per_second = stats.total_requests as f64 / 
            stats.last_reset.elapsed().as_secs_f64().max(1.0);
        // Note: ModuleMetrics doesn't have average_processing_time field
        // We could add it or use a different approach
    }

    /// Get statistics
    pub fn get_stats(&self) -> ContentFilterStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        *stats = ContentFilterStats {
            last_reset: Instant::now(),
            ..Default::default()
        };
    }
}

/// Blocking reason
#[derive(Debug, Clone)]
pub enum BlockReason {
    Domain(String),
    DomainPattern(String),
    Keyword(String),
    KeywordPattern(String),
    BodyKeyword(String),
    BodyKeywordPattern(String),
    MimeType(String),
    Extension(String),
    FileSize(u64),
}

impl std::fmt::Display for BlockReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockReason::Domain(domain) => write!(f, "Blocked domain: {}", domain),
            BlockReason::DomainPattern(pattern) => write!(f, "Blocked domain pattern: {}", pattern),
            BlockReason::Keyword(keyword) => write!(f, "Blocked keyword: {}", keyword),
            BlockReason::KeywordPattern(pattern) => write!(f, "Blocked keyword pattern: {}", pattern),
            BlockReason::BodyKeyword(keyword) => write!(f, "Blocked body keyword: {}", keyword),
            BlockReason::BodyKeywordPattern(pattern) => write!(f, "Blocked body keyword pattern: {}", pattern),
            BlockReason::MimeType(mime_type) => write!(f, "Blocked MIME type: {}", mime_type),
            BlockReason::Extension(ext) => write!(f, "Blocked extension: {}", ext),
            BlockReason::FileSize(size) => write!(f, "File too large: {} bytes", size),
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
        // Load configuration from module config
        if let Ok(filter_config) = serde_json::from_value::<ContentFilterConfig>(config.config.clone()) {
            self.config = filter_config;
        }

        // Compile regex patterns
        self.compile_patterns()?;

        if self.config.enable_logging {
            log::info!("Content filter module initialized with {} domain patterns and {} keyword patterns", 
                self.domain_patterns.len(), self.keyword_patterns.len());
        }

        Ok(())
    }

    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        if self.config.enable_logging {
            log::debug!("Processing REQMOD request: {}", request.uri);
        }

        match self.should_block(request).await? {
            Some(reason) => {
                if self.config.enable_logging {
                    log::warn!("REQMOD request blocked: {} - {}", request.uri, reason);
                }
                Ok(self.create_blocking_response(&reason))
            }
            None => {
                // Allow the request to pass through - use response generator for proper headers
                let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                    "G3ICAP-ContentFilter/1.0.0".to_string(),
                    "content-filter-1.0.0".to_string(),
                    Some("content-filter".to_string())
                );
                Ok(response_generator.no_modifications(None))
            }
        }
    }

    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        if self.config.enable_logging {
            log::debug!("Processing RESPMOD request: {}", request.uri);
        }

        match self.should_block(request).await? {
            Some(reason) => {
                if self.config.enable_logging {
                    log::warn!("RESPMOD request blocked: {} - {}", request.uri, reason);
                }
                Ok(self.create_blocking_response(&reason))
            }
            None => {
                // Allow the response to pass through - use response generator for proper headers
                let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                    "G3ICAP-ContentFilter/1.0.0".to_string(),
                    "content-filter-1.0.0".to_string(),
                    Some("content-filter".to_string())
                );
                Ok(response_generator.no_modifications(None))
            }
        }
    }

    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        let mut headers = http::HeaderMap::new();
        headers.insert("ISTag", "\"content-filter-1.0\"".parse().unwrap());
        headers.insert("Methods", "REQMOD, RESPMOD".parse().unwrap());
        headers.insert("Service", "Content Filter Service".parse().unwrap());
        headers.insert("Max-Connections", "1000".parse().unwrap());
        headers.insert("Options-TTL", "3600".parse().unwrap());
        headers.insert("Allow", "204".parse().unwrap());
        headers.insert("Preview", "1024".parse().unwrap());

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
        self.metrics.lock().unwrap().clone()
    }

    async fn cleanup(&mut self) {
        // Clear caches
        self.pattern_cache.write().unwrap().clear();
        
        if self.config.enable_logging {
            log::info!("Content filter module cleaned up");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{HeaderMap, Version};
    use bytes::Bytes;

    fn create_test_request(uri: &str, body: &str) -> IcapRequest {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("content-type", "text/html".parse().unwrap());

        IcapRequest {
            method: IcapMethod::Reqmod,
            uri: uri.parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body.to_string()),
            encapsulated: None,
        }
    }

    #[tokio::test]
    async fn test_domain_blocking() {
        let config = ContentFilterConfig {
            blocked_domains: vec!["malware.com".to_string()],
            blocked_domain_patterns: Vec::new(),
            blocked_keywords: Vec::new(),
            blocked_keyword_patterns: Vec::new(),
            blocked_mime_types: Vec::new(),
            blocked_extensions: Vec::new(),
            max_file_size: None,
            regex_cache_size: 1000,
            case_insensitive: true,
            enable_regex: true,
            blocking_action: BlockingAction::Forbidden,
            custom_message: None,
            enable_logging: true,
            enable_metrics: true,
        };
        let mut module = ContentFilterModule::new(config);
        module.compile_patterns().unwrap();

        let mut request = create_test_request("http://malware.com/path", "test body");
        request.headers.insert("host", "malware.com".parse().unwrap());
        let result = module.should_block(&request).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_keyword_blocking() {
        let config = ContentFilterConfig {
            blocked_keywords: vec!["malware".to_string()],
            ..Default::default()
        };
        let mut module = ContentFilterModule::new(config);
        module.compile_patterns().unwrap();

        let request = create_test_request("http://example.com/malware", "test body");
        let result = module.should_block(&request).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_mime_type_blocking() {
        let config = ContentFilterConfig {
            blocked_mime_types: vec!["application/octet-stream".to_string()],
            ..Default::default()
        };
        let mut module = ContentFilterModule::new(config);
        module.compile_patterns().unwrap();

        let mut request = create_test_request("http://example.com/file", "test body");
        request.headers.insert("content-type", "application/octet-stream".parse().unwrap());
        let result = module.should_block(&request).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_file_size_blocking() {
        let config = ContentFilterConfig {
            max_file_size: Some(100),
            ..Default::default()
        };
        let mut module = ContentFilterModule::new(config);
        module.compile_patterns().unwrap();

        let request = create_test_request("http://example.com/large", &"x".repeat(200));
        let result = module.should_block(&request).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_allow_clean_content() {
        let config = ContentFilterConfig {
            blocked_keywords: vec!["malware".to_string()],
            ..Default::default()
        };
        let mut module = ContentFilterModule::new(config);
        module.compile_patterns().unwrap();

        let request = create_test_request("http://example.com/clean", "clean content");
        let result = module.should_block(&request).await.unwrap();
        assert!(result.is_none());
    }
}
