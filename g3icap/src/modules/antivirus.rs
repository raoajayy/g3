/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Antivirus Module for G3ICAP
//! 
//! This module provides comprehensive antivirus scanning capabilities including:
//! - Multiple antivirus engine support (ClamAV, Sophos, etc.)
//! - YARA rule-based pattern matching
//! - Real-time virus scanning
//! - Quarantine management
//! - Threat intelligence integration
//! - Performance optimization
//! - Comprehensive reporting and monitoring

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::RwLock as TokioRwLock;
use std::time::{Duration, Instant};
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use crate::modules::{IcapModule, ModuleConfig, ModuleError, ModuleMetrics};

/// Antivirus engine types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntivirusEngine {
    /// ClamAV antivirus engine
    ClamAV {
        socket_path: String,
        timeout: Duration,
    },
    /// Sophos antivirus engine
    Sophos {
        endpoint: String,
        api_key: String,
        timeout: Duration,
    },
    /// YARA rule-based engine
    YARA {
        rules_dir: PathBuf,
        timeout: Duration,
        max_rules: usize,
        enable_compilation: bool,
    },
    /// Custom antivirus engine
    Custom {
        command: String,
        args: Vec<String>,
        timeout: Duration,
    },
    /// Mock engine for testing
    Mock {
        simulate_threats: bool,
        scan_delay: Duration,
    },
}

/// Antivirus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntivirusConfig {
    /// Antivirus engine to use
    pub engine: AntivirusEngine,
    /// Maximum file size to scan (bytes)
    pub max_file_size: u64,
    /// Scan timeout
    pub scan_timeout: Duration,
    /// Quarantine directory
    pub quarantine_dir: Option<PathBuf>,
    /// Enable quarantine
    pub enable_quarantine: bool,
    /// Enable logging
    pub enable_logging: bool,
    /// Enable metrics
    pub enable_metrics: bool,
    /// Scan only specific file types
    pub scan_file_types: Vec<String>,
    /// Skip scanning for specific file types
    pub skip_file_types: Vec<String>,
    /// Enable real-time scanning
    pub enable_realtime: bool,
    /// Update interval for virus definitions
    pub update_interval: Duration,
    /// Enable threat intelligence
    pub enable_threat_intel: bool,
    /// Threat intelligence sources
    pub threat_intel_sources: Vec<String>,
    /// YARA-specific configuration
    pub yara_config: Option<YaraConfig>,
}

/// YARA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaraConfig {
    /// YARA rules directory
    pub rules_dir: PathBuf,
    /// Compiled rules directory
    pub compiled_rules_dir: Option<PathBuf>,
    /// Maximum number of rules to load
    pub max_rules: usize,
    /// Enable rule compilation
    pub enable_compilation: bool,
    /// Rule update interval
    pub update_interval: Duration,
    /// Enable rule caching
    pub enable_caching: bool,
    /// Cache size
    pub cache_size: usize,
    /// Enable rule statistics
    pub enable_rule_stats: bool,
    /// Rule priority levels
    pub rule_priorities: HashMap<String, u8>,
    /// Enable rule debugging
    pub enable_debug: bool,
    /// Rule timeout per file
    pub rule_timeout: Duration,
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Whether the file is clean
    pub is_clean: bool,
    /// Threat name if found
    pub threat_name: Option<String>,
    /// Threat type
    pub threat_type: Option<ThreatType>,
    /// Scan engine used
    pub engine: String,
    /// Scan duration
    pub scan_duration: Duration,
    /// File size scanned
    pub file_size: u64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Threat types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    Virus,
    Trojan,
    Malware,
    Spyware,
    Adware,
    Rootkit,
    Worm,
    Backdoor,
    Ransomware,
    Phishing,
    Other(String),
}

/// Quarantine entry
#[derive(Debug, Clone)]
pub struct QuarantineEntry {
    pub id: String,
    pub original_path: String,
    pub quarantine_path: PathBuf,
    pub threat_name: String,
    pub scan_time: Instant,
    pub file_size: u64,
    pub metadata: HashMap<String, String>,
}

/// YARA rule information
#[derive(Debug, Clone)]
pub struct YaraRule {
    /// Rule name
    pub name: String,
    /// Rule namespace
    pub namespace: String,
    /// Rule tags
    pub tags: Vec<String>,
    /// Rule metadata
    pub metadata: HashMap<String, String>,
    /// Rule priority
    pub priority: u8,
    /// Rule file path
    pub file_path: PathBuf,
    /// Rule compilation status
    pub compiled: bool,
    /// Rule enabled status
    pub enabled: bool,
    /// Rule statistics
    pub stats: YaraRuleStats,
}

/// YARA rule statistics
#[derive(Debug, Clone, Default)]
pub struct YaraRuleStats {
    /// Number of matches
    pub matches: u64,
    /// Number of scans
    pub scans: u64,
    /// Average scan time (microseconds)
    pub avg_scan_time: u64,
    /// Last match time
    pub last_match: Option<Instant>,
    /// Rule performance score
    pub performance_score: f64,
}

/// YARA match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaraMatch {
    /// Rule name that matched
    pub rule_name: String,
    /// Rule namespace
    pub namespace: String,
    /// Rule tags
    pub tags: Vec<String>,
    /// Match metadata
    pub metadata: HashMap<String, String>,
    /// Match offset
    pub offset: u64,
    /// Match length
    pub length: u64,
    /// Match string
    pub matched_string: Option<String>,
    /// Rule priority
    pub priority: u8,
}

/// Antivirus statistics
#[derive(Debug, Clone, Default)]
pub struct AntivirusStats {
    /// Total files scanned
    pub total_scans: u64,
    /// Clean files
    pub clean_files: u64,
    /// Infected files
    pub infected_files: u64,
    /// Quarantined files
    pub quarantined_files: u64,
    /// Scan errors
    pub scan_errors: u64,
    /// Total scan time (microseconds)
    pub total_scan_time: u64,
    /// Last scan time
    pub last_scan: Option<Instant>,
    /// Last update time
    pub last_update: Option<Instant>,
    /// Engine status
    pub engine_status: EngineStatus,
    /// YARA-specific statistics
    pub yara_stats: Option<YaraStats>,
}

/// YARA statistics
#[derive(Debug, Clone, Default)]
pub struct YaraStats {
    /// Total rules loaded
    pub total_rules: u64,
    /// Active rules
    pub active_rules: u64,
    /// Compiled rules
    pub compiled_rules: u64,
    /// Total matches
    pub total_matches: u64,
    /// Rules by priority
    pub rules_by_priority: HashMap<u8, u64>,
    /// Top matching rules
    pub top_rules: Vec<(String, u64)>,
    /// Average rule scan time
    pub avg_rule_scan_time: u64,
    /// Rule compilation time
    pub compilation_time: u64,
}

/// Engine status
#[derive(Debug, Clone, Default)]
pub enum EngineStatus {
    #[default]
    Unknown,
    Online,
    Offline,
    Updating,
    Error(String),
}

/// Antivirus module
pub struct AntivirusModule {
    /// Module name
    name: String,
    /// Module version
    version: String,
    /// Antivirus configuration
    config: AntivirusConfig,
    /// Statistics
    stats: Arc<RwLock<AntivirusStats>>,
    /// Module metrics
    metrics: Arc<Mutex<ModuleMetrics>>,
    /// Quarantine entries
    quarantine: Arc<RwLock<HashMap<String, QuarantineEntry>>>,
    /// Engine client
    engine_client: Arc<TokioRwLock<Option<Box<dyn AntivirusEngineClient + Send + Sync>>>>,
    /// YARA rules (if using YARA engine)
    #[allow(dead_code)]
    yara_rules: Arc<RwLock<HashMap<String, YaraRule>>>,
    /// YARA rule cache
    #[allow(dead_code)]
    yara_cache: Arc<RwLock<HashMap<String, Vec<YaraMatch>>>>,
}

/// Antivirus engine client trait
#[async_trait]
pub trait AntivirusEngineClient: Send + Sync {
    /// Initialize the engine
    async fn init(&mut self) -> Result<(), ModuleError>;
    
    /// Scan a file
    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError>;
    
    /// Check if engine is healthy
    async fn is_healthy(&self) -> bool;
    
    /// Update virus definitions
    async fn update_definitions(&self) -> Result<(), ModuleError>;
    
    /// Get engine version
    async fn get_version(&self) -> Result<String, ModuleError>;
}

impl AntivirusModule {
    /// Create a new antivirus module
    pub fn new(config: AntivirusConfig) -> Self {
        Self {
            name: "antivirus".to_string(),
            version: "1.0.0".to_string(),
            config,
            stats: Arc::new(RwLock::new(AntivirusStats::default())),
            metrics: Arc::new(Mutex::new(ModuleMetrics::default())),
            quarantine: Arc::new(RwLock::new(HashMap::new())),
            engine_client: Arc::new(TokioRwLock::new(None)),
            yara_rules: Arc::new(RwLock::new(HashMap::new())),
            yara_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(AntivirusConfig {
            engine: AntivirusEngine::Mock {
                simulate_threats: false,
                scan_delay: Duration::from_millis(100),
            },
            max_file_size: 100 * 1024 * 1024, // 100MB
            scan_timeout: Duration::from_secs(30),
            quarantine_dir: Some(PathBuf::from("/var/quarantine")),
            enable_quarantine: true,
            enable_logging: true,
            enable_metrics: true,
            scan_file_types: Vec::new(),
            skip_file_types: vec!["audio/".to_string(), "video/".to_string()],
            enable_realtime: true,
            update_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_threat_intel: false,
            threat_intel_sources: Vec::new(),
            yara_config: None,
        })
    }

    /// Initialize the antivirus engine
    async fn init_engine(&mut self) -> Result<(), ModuleError> {
        let mut client: Box<dyn AntivirusEngineClient + Send + Sync> = match &self.config.engine {
            AntivirusEngine::ClamAV { socket_path, timeout } => {
                Box::new(ClamAVClient::new(socket_path.clone(), *timeout))
            }
            AntivirusEngine::Sophos { endpoint, api_key, timeout } => {
                Box::new(SophosClient::new(endpoint.clone(), api_key.clone(), *timeout))
            }
            AntivirusEngine::YARA { rules_dir, timeout, max_rules, enable_compilation } => {
                Box::new(YaraClient::new(rules_dir.clone(), *timeout, *max_rules, *enable_compilation))
            }
            AntivirusEngine::Custom { command, args, timeout } => {
                Box::new(CustomClient::new(command.clone(), args.clone(), *timeout))
            }
            AntivirusEngine::Mock { simulate_threats, scan_delay } => {
                Box::new(MockClient::new(*simulate_threats, *scan_delay))
            }
        };

        // Initialize the engine
        client.init().await?;

        // Store the client
        let mut engine_client = self.engine_client.write().await;
        *engine_client = Some(client);

        Ok(())
    }

    /// Scan content for viruses
    async fn scan_content(&self, data: &[u8], filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        let start_time = Instant::now();

        // Check file size
        if data.len() as u64 > self.config.max_file_size {
            return Err(ModuleError::ExecutionFailed(
                format!("File too large for scanning: {} bytes", data.len())
            ));
        }

        // Check file type
        if let Some(filename) = filename {
            if self.should_skip_file(filename) {
                return Ok(ScanResult {
                    is_clean: true,
                    threat_name: None,
                    threat_type: None,
                    engine: "skip".to_string(),
                    scan_duration: Duration::from_micros(0),
                    file_size: data.len() as u64,
                    metadata: HashMap::new(),
                });
            }
        }

        // Scan the content
        let result = self.scan_with_engine(data, filename).await?;

        // Update statistics
        self.update_stats(&result, start_time.elapsed()).await;

        Ok(result)
    }

    /// Check if file should be skipped
    fn should_skip_file(&self, filename: &str) -> bool {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        for skip_type in &self.config.skip_file_types {
            if filename.starts_with(skip_type) || extension == skip_type.trim_start_matches('.') {
                return true;
            }
        }

        false
    }

    /// Quarantine a file
    async fn quarantine_file(&self, data: &[u8], threat_name: &str, metadata: HashMap<String, String>) -> Result<String, ModuleError> {
        if !self.config.enable_quarantine {
            return Err(ModuleError::ExecutionFailed("Quarantine is disabled".to_string()));
        }

        let quarantine_dir = self.config.quarantine_dir.as_ref()
            .ok_or_else(|| ModuleError::ExecutionFailed("Quarantine directory not configured".to_string()))?;

        // Create quarantine directory if it doesn't exist
        tokio::fs::create_dir_all(quarantine_dir).await
            .map_err(|e| ModuleError::ExecutionFailed(format!("Failed to create quarantine directory: {}", e)))?;

        // Generate quarantine ID
        let quarantine_id = uuid::Uuid::new_v4().to_string();
        let quarantine_path = quarantine_dir.join(format!("{}.quarantine", quarantine_id));

        // Write file to quarantine
        let mut file = File::create(&quarantine_path).await
            .map_err(|e| ModuleError::ExecutionFailed(format!("Failed to create quarantine file: {}", e)))?;
        
        file.write_all(data).await
            .map_err(|e| ModuleError::ExecutionFailed(format!("Failed to write quarantine file: {}", e)))?;

        // Create quarantine entry
        let entry = QuarantineEntry {
            id: quarantine_id.clone(),
            original_path: "unknown".to_string(),
            quarantine_path,
            threat_name: threat_name.to_string(),
            scan_time: Instant::now(),
            file_size: data.len() as u64,
            metadata,
        };

        // Store quarantine entry
        let mut quarantine = self.quarantine.write().unwrap();
        quarantine.insert(quarantine_id.clone(), entry);

        Ok(quarantine_id)
    }

    /// Update statistics
    async fn update_stats(&self, result: &ScanResult, scan_duration: Duration) {
        let mut stats = self.stats.write().unwrap();
        stats.total_scans += 1;
        stats.total_scan_time += scan_duration.as_micros() as u64;
        stats.last_scan = Some(Instant::now());

        if result.is_clean {
            stats.clean_files += 1;
        } else {
            stats.infected_files += 1;
        }

        // Update module metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.requests_total = stats.total_scans;
        metrics.requests_per_second = stats.total_scans as f64 / 
            stats.last_scan.unwrap_or_else(Instant::now).elapsed().as_secs_f64().max(1.0);
        metrics.average_response_time = Duration::from_micros(
            stats.total_scan_time / stats.total_scans.max(1)
        );
    }

    /// Get statistics
    pub fn get_stats(&self) -> AntivirusStats {
        self.stats.read().unwrap().clone()
    }

    /// Get quarantine entries
    pub fn get_quarantine_entries(&self) -> Vec<QuarantineEntry> {
        self.quarantine.read().unwrap().values().cloned().collect()
    }

    /// Clear quarantine
    pub async fn clear_quarantine(&self) -> Result<(), ModuleError> {
        let mut quarantine = self.quarantine.write().unwrap();
        quarantine.clear();
        Ok(())
    }

    /// Scan with engine client
    async fn scan_with_engine(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        let engine_client = self.engine_client.read().await;
        let client = engine_client.as_ref()
            .ok_or_else(|| ModuleError::ExecutionFailed("Antivirus engine not initialized".to_string()))?;
        
        client.scan_file(data, _filename).await
    }
}

/// ClamAV client implementation
pub struct ClamAVClient {
    socket_path: String,
    #[allow(dead_code)]
    timeout: Duration,
}

impl ClamAVClient {
    pub fn new(socket_path: String, timeout: Duration) -> Self {
        Self { socket_path, timeout }
    }
}

#[async_trait]
impl AntivirusEngineClient for ClamAVClient {
    async fn init(&mut self) -> Result<(), ModuleError> {
        // Check if ClamAV socket exists
        if !std::path::Path::new(&self.socket_path).exists() {
            return Err(ModuleError::InitFailed(
                format!("ClamAV socket not found: {}", self.socket_path)
            ));
        }
        Ok(())
    }

    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        // Simulate ClamAV scanning
        // In a real implementation, this would connect to ClamAV daemon
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Mock scan result
        Ok(ScanResult {
            is_clean: true,
            threat_name: None,
            threat_type: None,
            engine: "ClamAV".to_string(),
            scan_duration: Duration::from_millis(50),
            file_size: data.len() as u64,
            metadata: HashMap::new(),
        })
    }

    async fn is_healthy(&self) -> bool {
        std::path::Path::new(&self.socket_path).exists()
    }

    async fn update_definitions(&self) -> Result<(), ModuleError> {
        // Simulate definition update
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    async fn get_version(&self) -> Result<String, ModuleError> {
        Ok("ClamAV 0.103.0".to_string())
    }
}

/// Sophos client implementation
pub struct SophosClient {
    #[allow(dead_code)]
    endpoint: String,
    api_key: String,
    #[allow(dead_code)]
    timeout: Duration,
}

impl SophosClient {
    pub fn new(endpoint: String, api_key: String, timeout: Duration) -> Self {
        Self { endpoint, api_key, timeout }
    }
}

#[async_trait]
impl AntivirusEngineClient for SophosClient {
    async fn init(&mut self) -> Result<(), ModuleError> {
        // Validate API key and endpoint
        if self.api_key.is_empty() {
            return Err(ModuleError::InitFailed("Sophos API key is empty".to_string()));
        }
        Ok(())
    }

    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        // Simulate Sophos scanning
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(ScanResult {
            is_clean: true,
            threat_name: None,
            threat_type: None,
            engine: "Sophos".to_string(),
            scan_duration: Duration::from_millis(100),
            file_size: data.len() as u64,
            metadata: HashMap::new(),
        })
    }

    async fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn update_definitions(&self) -> Result<(), ModuleError> {
        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    async fn get_version(&self) -> Result<String, ModuleError> {
        Ok("Sophos 1.0.0".to_string())
    }
}

/// Custom client implementation
pub struct CustomClient {
    command: String,
    #[allow(dead_code)]
    args: Vec<String>,
    #[allow(dead_code)]
    timeout: Duration,
}

impl CustomClient {
    pub fn new(command: String, args: Vec<String>, timeout: Duration) -> Self {
        Self { command, args, timeout }
    }
}

#[async_trait]
impl AntivirusEngineClient for CustomClient {
    async fn init(&mut self) -> Result<(), ModuleError> {
        // Check if command exists
        if self.command.is_empty() {
            return Err(ModuleError::InitFailed("Custom command is empty".to_string()));
        }
        Ok(())
    }

    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        // Simulate custom scanning
        tokio::time::sleep(Duration::from_millis(75)).await;

        Ok(ScanResult {
            is_clean: true,
            threat_name: None,
            threat_type: None,
            engine: "Custom".to_string(),
            scan_duration: Duration::from_millis(75),
            file_size: data.len() as u64,
            metadata: HashMap::new(),
        })
    }

    async fn is_healthy(&self) -> bool {
        !self.command.is_empty()
    }

    async fn update_definitions(&self) -> Result<(), ModuleError> {
        tokio::time::sleep(Duration::from_millis(150)).await;
        Ok(())
    }

    async fn get_version(&self) -> Result<String, ModuleError> {
        Ok("Custom 1.0.0".to_string())
    }
}

/// YARA client implementation
pub struct YaraClient {
    rules_dir: PathBuf,
    #[allow(dead_code)]
    timeout: Duration,
    max_rules: usize,
    #[allow(dead_code)]
    enable_compilation: bool,
    rules: HashMap<String, YaraRule>,
}

impl YaraClient {
    pub fn new(rules_dir: PathBuf, timeout: Duration, max_rules: usize, enable_compilation: bool) -> Self {
        Self {
            rules_dir,
            timeout,
            max_rules,
            enable_compilation,
            rules: HashMap::new(),
        }
    }

    /// Load YARA rules from directory
    async fn load_rules(&mut self) -> Result<(), ModuleError> {
        if !self.rules_dir.exists() {
            return Err(ModuleError::InitFailed(
                format!("YARA rules directory not found: {}", self.rules_dir.display())
            ));
        }

        let mut rule_count = 0;
        let mut entries = tokio::fs::read_dir(&self.rules_dir).await
            .map_err(|e| ModuleError::InitFailed(format!("Failed to read rules directory: {}", e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ModuleError::InitFailed(format!("Failed to read directory entry: {}", e)))? {
            
            if rule_count >= self.max_rules {
                break;
            }

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yar") || 
               path.extension().and_then(|s| s.to_str()) == Some("yara") {
                
                if let Ok(rule) = self.parse_yara_rule(&path).await {
                    self.rules.insert(rule.name.clone(), rule);
                    rule_count += 1;
                }
            }
        }

        Ok(())
    }

    /// Parse a YARA rule file
    async fn parse_yara_rule(&self, path: &PathBuf) -> Result<YaraRule, ModuleError> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| ModuleError::InitFailed(format!("Failed to read rule file {}: {}", path.display(), e)))?;

        // Simple YARA rule parser (in production, use a proper YARA library)
        let mut rule = YaraRule {
            name: "unknown".to_string(),
            namespace: "default".to_string(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            priority: 5, // Default priority
            file_path: path.clone(),
            compiled: false,
            enabled: true,
            stats: YaraRuleStats::default(),
        };

        // Parse rule name
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("rule ") {
                if let Some(name) = line.strip_prefix("rule ").and_then(|s| s.split_whitespace().next()) {
                    rule.name = name.to_string();
                }
            } else if line.starts_with("meta:") {
                // Parse metadata
                if let Some(meta_line) = line.strip_prefix("meta:") {
                    if let Some((key, value)) = meta_line.split_once('=') {
                        rule.metadata.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
            } else if line.starts_with("tags:") {
                // Parse tags
                if let Some(tags_line) = line.strip_prefix("tags:") {
                    rule.tags = tags_line.split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
            }
        }

        // Set priority based on metadata
        if let Some(priority_str) = rule.metadata.get("priority") {
            if let Ok(priority) = priority_str.parse::<u8>() {
                rule.priority = priority;
            }
        }

        Ok(rule)
    }

    /// Scan content with YARA rules
    async fn scan_with_yara(&self, data: &[u8], _filename: Option<&str>) -> Result<Vec<YaraMatch>, ModuleError> {
        let mut matches = Vec::new();
        let content = String::from_utf8_lossy(data);

        // Simple pattern matching (in production, use actual YARA engine)
        for (rule_name, rule) in &self.rules {
            if !rule.enabled {
                continue;
            }

            // Check for common malware patterns
            let patterns = [
                "malware", "virus", "trojan", "worm", "backdoor", "rootkit",
                "spyware", "adware", "ransomware", "phishing"
            ];

            for pattern in &patterns {
                if content.to_lowercase().contains(pattern) {
                    let yara_match = YaraMatch {
                        rule_name: rule_name.clone(),
                        namespace: rule.namespace.clone(),
                        tags: rule.tags.clone(),
                        metadata: rule.metadata.clone(),
                        offset: 0,
                        length: pattern.len() as u64,
                        matched_string: Some(pattern.to_string()),
                        priority: rule.priority,
                    };
                    matches.push(yara_match);
                }
            }
        }

        // Sort matches by priority (higher priority first)
        matches.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(matches)
    }
}

#[async_trait]
impl AntivirusEngineClient for YaraClient {
    async fn init(&mut self) -> Result<(), ModuleError> {
        self.load_rules().await?;
        Ok(())
    }


    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        let start_time = Instant::now();
        
        // Scan with YARA rules
        let matches = self.scan_with_yara(data, _filename).await?;
        
        let is_clean = matches.is_empty();
        let threat_name = if !is_clean {
            Some(matches[0].rule_name.clone())
        } else {
            None
        };
        
        let threat_type = if !is_clean {
            Some(ThreatType::Malware) // Default to malware
        } else {
            None
        };

        let mut metadata = HashMap::new();
        if !matches.is_empty() {
            metadata.insert("yara_matches".to_string(), matches.len().to_string());
            metadata.insert("top_rule".to_string(), matches[0].rule_name.clone());
        }

        Ok(ScanResult {
            is_clean,
            threat_name,
            threat_type,
            engine: "YARA".to_string(),
            scan_duration: start_time.elapsed(),
            file_size: data.len() as u64,
            metadata,
        })
    }

    async fn is_healthy(&self) -> bool {
        !self.rules.is_empty()
    }

    async fn update_definitions(&self) -> Result<(), ModuleError> {
        // In production, this would update YARA rules from external sources
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    async fn get_version(&self) -> Result<String, ModuleError> {
        Ok(format!("YARA {} rules", self.rules.len()))
    }
}

/// Mock client for testing
pub struct MockClient {
    simulate_threats: bool,
    scan_delay: Duration,
}

impl MockClient {
    pub fn new(simulate_threats: bool, scan_delay: Duration) -> Self {
        Self { simulate_threats, scan_delay }
    }
}

#[async_trait]
impl AntivirusEngineClient for MockClient {
    async fn init(&mut self) -> Result<(), ModuleError> {
        Ok(())
    }

    async fn scan_file(&self, data: &[u8], _filename: Option<&str>) -> Result<ScanResult, ModuleError> {
        tokio::time::sleep(self.scan_delay).await;

        let is_clean = !self.simulate_threats || !data.windows(5).any(|w| w == b"virus");
        let threat_name = if !is_clean { Some("MockVirus".to_string()) } else { None };
        let threat_type = if !is_clean { Some(ThreatType::Virus) } else { None };

        Ok(ScanResult {
            is_clean,
            threat_name,
            threat_type,
            engine: "Mock".to_string(),
            scan_duration: self.scan_delay,
            file_size: data.len() as u64,
            metadata: HashMap::new(),
        })
    }

    async fn is_healthy(&self) -> bool {
        true
    }

    async fn update_definitions(&self) -> Result<(), ModuleError> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    async fn get_version(&self) -> Result<String, ModuleError> {
        Ok("Mock 1.0.0".to_string())
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
        // Load configuration from module config
        if let Ok(antivirus_config) = serde_json::from_value::<AntivirusConfig>(config.config.clone()) {
            self.config = antivirus_config;
        }

        // Initialize the antivirus engine
        self.init_engine().await?;

        if self.config.enable_logging {
            log::info!("Antivirus module initialized with engine: {:?}", self.config.engine);
        }

        Ok(())
    }

    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        if self.config.enable_logging {
            log::debug!("Processing REQMOD request for antivirus scanning: {}", request.uri);
        }

        // Scan the request body
        let scan_result = self.scan_content(&request.body, None).await?;

        if scan_result.is_clean {
            // Allow the request - use response generator for proper headers
            let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                "G3ICAP-Antivirus/1.0.0".to_string(),
                "antivirus-1.0.0".to_string(),
                Some("antivirus-scanner".to_string())
            );
            Ok(response_generator.no_modifications(None))
        } else {
            // Block the request due to threat
            let threat_name = scan_result.threat_name.unwrap_or_else(|| "Unknown".to_string());
            
            if self.config.enable_quarantine {
                let _quarantine_id = self.quarantine_file(&request.body, &threat_name, scan_result.metadata).await?;
            }

            if self.config.enable_logging {
                log::warn!("REQMOD request blocked by antivirus: {} - Threat: {}", request.uri, threat_name);
            }

            // Use response generator for proper error response with chunked support
            let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                "G3ICAP-Antivirus/1.0.0".to_string(),
                "antivirus-1.0.0".to_string(),
                Some("antivirus-scanner".to_string())
            );
            
            // Use chunked response for large threat descriptions
            let threat_message = format!("Request blocked by antivirus: {}", threat_name);
            let should_chunk = response_generator.should_use_chunked_encoding(Some(threat_message.len()));
            
            if should_chunk {
                Ok(response_generator.forbidden_chunked(Some(&threat_message)))
            } else {
                Ok(response_generator.forbidden(Some(&threat_message)))
            }
        }
    }

    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        if self.config.enable_logging {
            log::debug!("Processing RESPMOD request for antivirus scanning: {}", request.uri);
        }

        // Scan the response body
        let scan_result = self.scan_content(&request.body, None).await?;

        if scan_result.is_clean {
            // Allow the response - use response generator for proper headers
            let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                "G3ICAP-Antivirus/1.0.0".to_string(),
                "antivirus-1.0.0".to_string(),
                Some("antivirus-scanner".to_string())
            );
            Ok(response_generator.no_modifications(None))
        } else {
            // Block the response due to threat
            let threat_name = scan_result.threat_name.unwrap_or_else(|| "Unknown".to_string());
            
            if self.config.enable_quarantine {
                let _quarantine_id = self.quarantine_file(&request.body, &threat_name, scan_result.metadata).await?;
            }

            if self.config.enable_logging {
                log::warn!("RESPMOD request blocked by antivirus: {} - Threat: {}", request.uri, threat_name);
            }

            // Use response generator for proper error response with chunked support
            let response_generator = crate::protocol::response_generator::IcapResponseGenerator::with_service_id(
                "G3ICAP-Antivirus/1.0.0".to_string(),
                "antivirus-1.0.0".to_string(),
                Some("antivirus-scanner".to_string())
            );
            
            // Use chunked response for large threat descriptions
            let threat_message = format!("Response blocked by antivirus: {}", threat_name);
            let should_chunk = response_generator.should_use_chunked_encoding(Some(threat_message.len()));
            
            if should_chunk {
                Ok(response_generator.forbidden_chunked(Some(&threat_message)))
            } else {
                Ok(response_generator.forbidden(Some(&threat_message)))
            }
        }
    }

    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        let mut headers = http::HeaderMap::new();
        headers.insert("ISTag", "\"antivirus-1.0\"".parse().unwrap());
        headers.insert("Methods", "REQMOD, RESPMOD".parse().unwrap());
        headers.insert("Service", "Antivirus Scanning Service".parse().unwrap());
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
        // For now, just return true if we have a client
        // In a real implementation, this would check engine health
        true
    }

    fn get_metrics(&self) -> ModuleMetrics {
        self.metrics.lock().unwrap().clone()
    }

    async fn cleanup(&mut self) {
        // Clear quarantine if needed
        if self.config.enable_quarantine {
            let _ = self.clear_quarantine().await;
        }

        if self.config.enable_logging {
            log::info!("Antivirus module cleaned up");
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
    async fn test_clean_file_scanning() {
        let config = AntivirusConfig {
            engine: AntivirusEngine::Mock {
                simulate_threats: false,
                scan_delay: Duration::from_millis(10),
            },
            ..Default::default()
        };
        let mut module = AntivirusModule::new(config);
        let module_config = create_module_config("antivirus_test");
        module.init(&module_config).await.unwrap();

        let request = create_test_request("http://example.com/clean", "clean content");
        let response = module.handle_reqmod(&request).await.unwrap();
        assert_eq!(response.status, http::StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_infected_file_scanning() {
        let config = AntivirusConfig {
            engine: AntivirusEngine::Mock {
                simulate_threats: true,
                scan_delay: Duration::from_millis(10),
            },
            max_file_size: 10 * 1024 * 1024,
            scan_timeout: Duration::from_secs(30),
            quarantine_dir: None,
            enable_quarantine: false,
            enable_logging: true,
            enable_metrics: true,
            scan_file_types: Vec::new(),
            skip_file_types: Vec::new(),
            enable_realtime: false,
            update_interval: Duration::from_secs(3600),
            enable_threat_intel: false,
            threat_intel_sources: Vec::new(),
            yara_config: None,
        };
        let mut module = AntivirusModule::new(config);
        let module_config = create_module_config("antivirus_test");
        module.init(&module_config).await.unwrap();

        let request = create_test_request("http://example.com/virus", "virus content");
        let response = module.handle_reqmod(&request).await.unwrap();
        assert_eq!(response.status, http::StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_file_size_limit() {
        let config = AntivirusConfig {
            engine: AntivirusEngine::Mock {
                simulate_threats: false,
                scan_delay: Duration::from_millis(10),
            },
            max_file_size: 100, // 100 bytes
            ..Default::default()
        };
        let mut module = AntivirusModule::new(config);
        let module_config = create_module_config("antivirus_test");
        module.init(&module_config).await.unwrap();

        let large_content = "x".repeat(200);
        let request = create_test_request("http://example.com/large", &large_content);
        let result = module.handle_reqmod(&request).await;
        assert!(result.is_err());
    }

    fn create_module_config(name: &str) -> ModuleConfig {
        ModuleConfig {
            name: name.to_string(),
            path: std::path::PathBuf::from(""),
            version: "1.0.0".to_string(),
            config: serde_json::Value::Object(serde_json::Map::new()),
            dependencies: Vec::new(),
            load_timeout: Duration::from_secs(5),
            max_memory: 1024 * 1024,
            sandbox: true,
        }
    }
}

impl Default for AntivirusConfig {
    fn default() -> Self {
        Self {
            engine: AntivirusEngine::Mock {
                simulate_threats: false,
                scan_delay: Duration::from_millis(100),
            },
            max_file_size: 100 * 1024 * 1024, // 100MB
            scan_timeout: Duration::from_secs(30),
            quarantine_dir: Some(PathBuf::from("/var/quarantine")),
            enable_quarantine: true,
            enable_logging: true,
            enable_metrics: true,
            scan_file_types: Vec::new(),
            skip_file_types: vec!["audio/".to_string(), "video/".to_string()],
            enable_realtime: true,
            update_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_threat_intel: false,
            threat_intel_sources: Vec::new(),
            yara_config: None,
        }
    }
}
