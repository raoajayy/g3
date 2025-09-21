/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! ICAP-specific headers implementation
//! 
//! This module implements ICAP-specific headers as defined in RFC 3507.

use std::net::IpAddr;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

/// ICAP-specific headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcapHeaders {
    /// ICAP protocol version
    pub icap_version: Option<String>,
    /// Client IP address
    pub icap_client_ip: Option<IpAddr>,
    /// Server IP address
    pub icap_server_ip: Option<IpAddr>,
    /// Request identification
    pub icap_request_id: Option<String>,
    /// Encapsulated data specification
    pub encapsulated: Option<String>,
    /// Preview size
    pub preview: Option<usize>,
    /// Allowed response codes
    pub allow: Option<String>,
    /// Service identification tag
    pub istag: Option<String>,
    /// Service description
    pub service: Option<String>,
    /// Maximum connections
    pub max_connections: Option<usize>,
    /// Options TTL
    pub options_ttl: Option<usize>,
    /// Service ID
    pub service_id: Option<String>,
    /// Transfer preview
    pub transfer_preview: Option<String>,
    /// Update
    pub update: Option<String>,
    /// Transfer complete
    pub transfer_complete: Option<String>,
    /// Transfer ignore
    pub transfer_ignore: Option<String>,
}

impl Default for IcapHeaders {
    fn default() -> Self {
        Self {
            icap_version: Some("ICAP/1.0".to_string()),
            icap_client_ip: None,
            icap_server_ip: None,
            icap_request_id: None,
            encapsulated: None,
            preview: None,
            allow: None,
            istag: None,
            service: None,
            max_connections: None,
            options_ttl: None,
            service_id: None,
            transfer_preview: None,
            update: None,
            transfer_complete: None,
            transfer_ignore: None,
        }
    }
}

impl IcapHeaders {
    /// Create new ICAP headers
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse ICAP headers from HTTP headers
    pub fn from_http_headers(headers: &HeaderMap) -> Self {
        let mut icap_headers = Self::new();

        // Parse ICAP-specific headers
        if let Some(version) = headers.get("icap-version") {
            icap_headers.icap_version = version.to_str().ok().map(|s| s.to_string());
        }

        if let Some(client_ip) = headers.get("icap-client-ip") {
            if let Ok(ip_str) = client_ip.to_str() {
                icap_headers.icap_client_ip = ip_str.parse().ok();
            }
        }

        if let Some(server_ip) = headers.get("icap-server-ip") {
            if let Ok(ip_str) = server_ip.to_str() {
                icap_headers.icap_server_ip = ip_str.parse().ok();
            }
        }

        if let Some(request_id) = headers.get("icap-request-id") {
            icap_headers.icap_request_id = request_id.to_str().ok().map(|s| s.to_string());
        }

        if let Some(encapsulated) = headers.get("encapsulated") {
            icap_headers.encapsulated = encapsulated.to_str().ok().map(|s| s.to_string());
        }

        if let Some(preview) = headers.get("preview") {
            if let Ok(preview_str) = preview.to_str() {
                icap_headers.preview = preview_str.parse().ok();
            }
        }

        if let Some(allow) = headers.get("allow") {
            icap_headers.allow = allow.to_str().ok().map(|s| s.to_string());
        }

        if let Some(istag) = headers.get("istag") {
            icap_headers.istag = istag.to_str().ok().map(|s| s.to_string());
        }

        if let Some(service) = headers.get("service") {
            icap_headers.service = service.to_str().ok().map(|s| s.to_string());
        }

        if let Some(max_conn) = headers.get("max-connections") {
            if let Ok(max_conn_str) = max_conn.to_str() {
                icap_headers.max_connections = max_conn_str.parse().ok();
            }
        }

        if let Some(options_ttl) = headers.get("options-ttl") {
            if let Ok(ttl_str) = options_ttl.to_str() {
                icap_headers.options_ttl = ttl_str.parse().ok();
            }
        }

        if let Some(service_id) = headers.get("service-id") {
            icap_headers.service_id = service_id.to_str().ok().map(|s| s.to_string());
        }

        if let Some(transfer_preview) = headers.get("transfer-preview") {
            icap_headers.transfer_preview = transfer_preview.to_str().ok().map(|s| s.to_string());
        }

        if let Some(update) = headers.get("update") {
            icap_headers.update = update.to_str().ok().map(|s| s.to_string());
        }

        if let Some(transfer_complete) = headers.get("transfer-complete") {
            icap_headers.transfer_complete = transfer_complete.to_str().ok().map(|s| s.to_string());
        }

        if let Some(transfer_ignore) = headers.get("transfer-ignore") {
            icap_headers.transfer_ignore = transfer_ignore.to_str().ok().map(|s| s.to_string());
        }

        icap_headers
    }

    /// Convert to HTTP headers
    pub fn to_http_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(version) = &self.icap_version {
            if let Ok(header_value) = version.parse() {
                headers.insert("icap-version", header_value);
            }
        }

        if let Some(client_ip) = &self.icap_client_ip {
            if let Ok(header_value) = client_ip.to_string().parse() {
                headers.insert("icap-client-ip", header_value);
            }
        }

        if let Some(server_ip) = &self.icap_server_ip {
            if let Ok(header_value) = server_ip.to_string().parse() {
                headers.insert("icap-server-ip", header_value);
            }
        }

        if let Some(request_id) = &self.icap_request_id {
            if let Ok(header_value) = request_id.parse() {
                headers.insert("icap-request-id", header_value);
            }
        }

        if let Some(encapsulated) = &self.encapsulated {
            if let Ok(header_value) = encapsulated.parse() {
                headers.insert("encapsulated", header_value);
            }
        }

        if let Some(preview) = &self.preview {
            if let Ok(header_value) = preview.to_string().parse() {
                headers.insert("preview", header_value);
            }
        }

        if let Some(allow) = &self.allow {
            if let Ok(header_value) = allow.parse() {
                headers.insert("allow", header_value);
            }
        }

        if let Some(istag) = &self.istag {
            if let Ok(header_value) = istag.parse() {
                headers.insert("istag", header_value);
            }
        }

        if let Some(service) = &self.service {
            if let Ok(header_value) = service.parse() {
                headers.insert("service", header_value);
            }
        }

        if let Some(max_conn) = &self.max_connections {
            if let Ok(header_value) = max_conn.to_string().parse() {
                headers.insert("max-connections", header_value);
            }
        }

        if let Some(options_ttl) = &self.options_ttl {
            if let Ok(header_value) = options_ttl.to_string().parse() {
                headers.insert("options-ttl", header_value);
            }
        }

        if let Some(service_id) = &self.service_id {
            if let Ok(header_value) = service_id.parse() {
                headers.insert("service-id", header_value);
            }
        }

        if let Some(transfer_preview) = &self.transfer_preview {
            if let Ok(header_value) = transfer_preview.parse() {
                headers.insert("transfer-preview", header_value);
            }
        }

        if let Some(update) = &self.update {
            if let Ok(header_value) = update.parse() {
                headers.insert("update", header_value);
            }
        }

        if let Some(transfer_complete) = &self.transfer_complete {
            if let Ok(header_value) = transfer_complete.parse() {
                headers.insert("transfer-complete", header_value);
            }
        }

        if let Some(transfer_ignore) = &self.transfer_ignore {
            if let Ok(header_value) = transfer_ignore.parse() {
                headers.insert("transfer-ignore", header_value);
            }
        }

        headers
    }

    /// Validate ICAP headers
    pub fn validate(&self) -> Result<(), IcapHeaderError> {
        // Validate ICAP version
        if let Some(version) = &self.icap_version {
            if !version.starts_with("ICAP/") {
                return Err(IcapHeaderError::InvalidVersion(version.clone()));
            }
        }

        // Validate preview size
        if let Some(preview) = self.preview {
            if preview == 0 {
                return Err(IcapHeaderError::InvalidPreviewSize(preview));
            }
        }

        // Validate max connections
        if let Some(max_conn) = self.max_connections {
            if max_conn == 0 {
                return Err(IcapHeaderError::InvalidMaxConnections(max_conn));
            }
        }

        // Validate options TTL
        if let Some(ttl) = self.options_ttl {
            if ttl == 0 {
                return Err(IcapHeaderError::InvalidOptionsTtl(ttl));
            }
        }

        Ok(())
    }
}

/// ICAP header errors
#[derive(Debug, thiserror::Error)]
pub enum IcapHeaderError {
    #[error("Invalid ICAP version: {0}")]
    InvalidVersion(String),
    #[error("Invalid preview size: {0}")]
    InvalidPreviewSize(usize),
    #[error("Invalid max connections: {0}")]
    InvalidMaxConnections(usize),
    #[error("Invalid options TTL: {0}")]
    InvalidOptionsTtl(usize),
}

/// ICAP header constants
pub mod constants {
    /// ICAP version header
    pub const ICAP_VERSION: &str = "icap-version";
    /// Client IP header
    pub const ICAP_CLIENT_IP: &str = "icap-client-ip";
    /// Server IP header
    pub const ICAP_SERVER_IP: &str = "icap-server-ip";
    /// Request ID header
    pub const ICAP_REQUEST_ID: &str = "icap-request-id";
    /// Encapsulated header
    pub const ENCAPSULATED: &str = "encapsulated";
    /// Preview header
    pub const PREVIEW: &str = "preview";
    /// Allow header
    pub const ALLOW: &str = "allow";
    /// ISTag header
    pub const ISTAG: &str = "istag";
    /// Service header
    pub const SERVICE: &str = "service";
    /// Max connections header
    pub const MAX_CONNECTIONS: &str = "max-connections";
    /// Options TTL header
    pub const OPTIONS_TTL: &str = "options-ttl";
    /// Service ID header
    pub const SERVICE_ID: &str = "service-id";
    /// Transfer preview header
    pub const TRANSFER_PREVIEW: &str = "transfer-preview";
    /// Update header
    pub const UPDATE: &str = "update";
    /// Transfer complete header
    pub const TRANSFER_COMPLETE: &str = "transfer-complete";
    /// Transfer ignore header
    pub const TRANSFER_IGNORE: &str = "transfer-ignore";
}
