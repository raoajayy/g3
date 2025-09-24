/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Audit operations for ICAP server following g3proxy patterns
//!
//! This module provides comprehensive audit operations with detailed
//! event logging, statistics, and performance monitoring.

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use g3_types::metrics::NodeName;
use serde::{Serialize, Deserialize};

use super::{IcapAuditHandle, AuditHandle};
use super::registry;

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Request received
    RequestReceived,
    /// Request processed
    RequestProcessed,
    /// Request blocked
    RequestBlocked,
    /// Response scanned
    ResponseScanned,
    /// Response blocked
    ResponseBlocked,
    /// Configuration changed
    ConfigChanged,
    /// Service started
    ServiceStarted,
    /// Service stopped
    ServiceStopped,
    /// Error occurred
    ErrorOccurred,
    /// Security event
    SecurityEvent,
    /// Compliance event
    ComplianceEvent,
}

/// Audit event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event timestamp (Unix epoch)
    pub timestamp: u64,
    /// Event type
    pub event_type: AuditEventType,
    /// Event message
    pub message: String,
    /// Event details
    pub details: String,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Request URI
    pub request_uri: Option<String>,
    /// Response status
    pub response_status: Option<u16>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Severity level
    pub severity: AuditSeverity,
}

/// Audit severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditSeverity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Audit operations trait
pub trait IcapAuditOps: Send + Sync {
    /// Get audit handle
    fn get_audit_handle(&self) -> &IcapAuditHandle;
    
    /// Log audit event
    fn log_audit_event(&self, event: &str, details: &str) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::RequestProcessed,
            message: event.to_string(),
            details: details.to_string(),
            client_ip: None,
            user_agent: None,
            request_uri: None,
            response_status: None,
            metadata: HashMap::new(),
            severity: AuditSeverity::Info,
        });
    }
    
    /// Log structured audit event
    fn log_structured_event(&self, event: AuditEvent) {
        if self.get_audit_handle().is_enabled() {
            // Log to console (in production, this would go to a proper audit log)
            println!("AUDIT[{}]: {:?} - {} | {}", 
                event.timestamp, 
                event.event_type, 
                event.message, 
                event.details
            );
            
            // Log additional metadata if present
            if !event.metadata.is_empty() {
                println!("AUDIT_METADATA: {:?}", event.metadata);
            }
            
            // Log client information if available
            if let Some(client_ip) = &event.client_ip {
                println!("AUDIT_CLIENT: IP={}", client_ip);
            }
            if let Some(user_agent) = &event.user_agent {
                println!("AUDIT_USER_AGENT: {}", user_agent);
            }
            if let Some(uri) = &event.request_uri {
                println!("AUDIT_URI: {}", uri);
            }
            if let Some(status) = event.response_status {
                println!("AUDIT_STATUS: {}", status);
            }
        }
    }
    
    /// Log request received event
    fn log_request_received(&self, client_ip: &str, user_agent: &str, uri: &str) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::RequestReceived,
            message: "ICAP request received".to_string(),
            details: format!("Client: {}, URI: {}", client_ip, uri),
            client_ip: Some(client_ip.to_string()),
            user_agent: Some(user_agent.to_string()),
            request_uri: Some(uri.to_string()),
            response_status: None,
            metadata: HashMap::new(),
            severity: AuditSeverity::Info,
        });
    }
    
    /// Log request blocked event
    fn log_request_blocked(&self, client_ip: &str, uri: &str, reason: &str) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::RequestBlocked,
            message: "ICAP request blocked".to_string(),
            details: format!("Reason: {}", reason),
            client_ip: Some(client_ip.to_string()),
            user_agent: None,
            request_uri: Some(uri.to_string()),
            response_status: Some(403),
            metadata: HashMap::new(),
            severity: AuditSeverity::Warning,
        });
    }
    
    /// Log response scanned event
    fn log_response_scanned(&self, client_ip: &str, uri: &str, scan_result: &str) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::ResponseScanned,
            message: "ICAP response scanned".to_string(),
            details: format!("Scan result: {}", scan_result),
            client_ip: Some(client_ip.to_string()),
            user_agent: None,
            request_uri: Some(uri.to_string()),
            response_status: Some(200),
            metadata: HashMap::new(),
            severity: AuditSeverity::Info,
        });
    }
    
    /// Log response blocked event
    fn log_response_blocked(&self, client_ip: &str, uri: &str, threat_name: &str) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::ResponseBlocked,
            message: "ICAP response blocked".to_string(),
            details: format!("Threat detected: {}", threat_name),
            client_ip: Some(client_ip.to_string()),
            user_agent: None,
            request_uri: Some(uri.to_string()),
            response_status: Some(403),
            metadata: HashMap::new(),
            severity: AuditSeverity::Critical,
        });
    }
    
    /// Log security event
    fn log_security_event(&self, event: &str, details: &str, severity: AuditSeverity) {
        self.log_structured_event(AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: AuditEventType::SecurityEvent,
            message: event.to_string(),
            details: details.to_string(),
            client_ip: None,
            user_agent: None,
            request_uri: None,
            response_status: None,
            metadata: HashMap::new(),
            severity,
        });
    }
}

/// Default audit operations implementation
pub struct DefaultIcapAuditOps {
    handle: IcapAuditHandle,
}

impl DefaultIcapAuditOps {
    pub fn new(name: NodeName, enabled: bool) -> Self {
        Self {
            handle: IcapAuditHandle::new(name, enabled),
        }
    }
}

impl IcapAuditOps for DefaultIcapAuditOps {
    fn get_audit_handle(&self) -> &IcapAuditHandle {
        &self.handle
    }
}

/// Initialize audit handles following g3proxy patterns
pub async fn initialize_audit_handles() -> Result<()> {
    // Load audit configurations from registry
    let configs = registry::get_all_audit_configs().await?;
    
    // Create audit handles for each configuration
    for (name, _config) in configs {
        let handle = AuditHandle::new(name.clone(), true); // Default to enabled
        registry::register_audit_handle(name, Arc::new(handle))?;
    }
    
    Ok(())
}

/// Load all audit handlers
pub async fn load_all() -> Result<()> {
    // Initialize audit handles
    initialize_audit_handles().await?;
    
    // Load audit configurations
    registry::load_all().await?;
    
    Ok(())
}

/// Get audit handle by name
pub fn get_audit_handle(name: &NodeName) -> Option<Arc<AuditHandle>> {
    registry::get_audit_handle(name)
}

/// Get all audit handles
pub fn get_all_audit_handles() -> HashMap<NodeName, Arc<AuditHandle>, foldhash::fast::FixedState> {
    registry::get_all_audit_handles()
}

/// Create a new audit handle
pub fn create_audit_handle(name: NodeName, enabled: bool) -> Arc<AuditHandle> {
    Arc::new(AuditHandle::new(name, enabled))
}

/// Reload audit configurations
pub async fn reload() -> Result<()> {
    // Clear existing handles
    registry::clear_all_handles();
    
    // Reload configurations
    registry::load_all().await?;
    
    // Reinitialize handles
    initialize_audit_handles().await?;
    
    Ok(())
}
