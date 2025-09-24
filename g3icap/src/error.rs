/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Error types for G3 ICAP Server
//!
//! This module provides comprehensive error handling following g3proxy patterns
//! with detailed error context and proper error propagation.

use std::fmt;
use thiserror::Error;

/// Result type for G3 ICAP operations
pub type IcapResult<T> = Result<T, IcapError>;

/// Error types for G3 ICAP Server following g3proxy patterns
#[derive(Error, Debug)]
pub enum IcapError {
    /// Configuration error with context
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Protocol error with detailed context
    #[error("Protocol error: {message}")]
    Protocol {
        message: String,
        protocol: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Network error with connection details
    #[error("Network error: {message}")]
    Network {
        message: String,
        address: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Service error with service details
    #[error("Service error: {message}")]
    Service {
        message: String,
        service: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Authentication error
    #[error("Authentication error: {message}")]
    Auth {
        message: String,
        user: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Authorization error
    #[error("Authorization error: {message}")]
    Authorization {
        message: String,
        user: Option<String>,
        resource: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Audit error
    #[error("Audit error: {message}")]
    Audit {
        message: String,
        operation: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Content filtering error
    #[error("Content filtering error: {message}")]
    ContentFilter {
        message: String,
        filter_type: Option<String>,
        content_type: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Antivirus scanning error
    #[error("Antivirus scanning error: {message}")]
    Antivirus {
        message: String,
        engine: Option<String>,
        file: Option<String>,
        context: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Timeout error
    #[error("Timeout error: {message}")]
    Timeout {
        message: String,
        operation: Option<String>,
        duration: Option<std::time::Duration>,
        context: Option<String>,
    },

    /// Resource exhaustion error
    #[error("Resource exhaustion: {message}")]
    ResourceExhausted {
        message: String,
        resource_type: Option<String>,
        limit: Option<usize>,
        current: Option<usize>,
        context: Option<String>,
    },

    /// IO error with context
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP error with context
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),

    /// URL error with context
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// JSON error with context
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// YAML error with context
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Anyhow error with context
    #[error("General error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl IcapError {
    /// Create a configuration error with context
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            context: None,
            source: None,
        }
    }

    /// Create a configuration error with context and source
    pub fn config_error_with_source(
        message: impl Into<String>,
        context: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Config {
            message: message.into(),
            context: Some(context.into()),
            source: Some(Box::new(source)),
        }
    }

    /// Create a protocol error with context
    pub fn protocol_error(message: impl Into<String>, protocol: impl Into<String>) -> Self {
        Self::Protocol {
            message: message.into(),
            protocol: Some(protocol.into()),
            context: None,
            source: None,
        }
    }

    /// Create a network error with context
    pub fn network_error(message: impl Into<String>, address: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            address: Some(address.into()),
            context: None,
            source: None,
        }
    }

    /// Create a service error with context
    pub fn service_error(message: impl Into<String>, service: impl Into<String>) -> Self {
        Self::Service {
            message: message.into(),
            service: Some(service.into()),
            context: None,
            source: None,
        }
    }

    /// Create an authentication error
    pub fn auth_error(message: impl Into<String>, user: impl Into<String>) -> Self {
        Self::Auth {
            message: message.into(),
            user: Some(user.into()),
            context: None,
            source: None,
        }
    }

    /// Create an authorization error
    pub fn authorization_error(
        message: impl Into<String>,
        user: impl Into<String>,
        resource: impl Into<String>,
    ) -> Self {
        Self::Authorization {
            message: message.into(),
            user: Some(user.into()),
            resource: Some(resource.into()),
            context: None,
            source: None,
        }
    }

    /// Create a timeout error
    pub fn timeout_error(
        message: impl Into<String>,
        operation: impl Into<String>,
        duration: std::time::Duration,
    ) -> Self {
        Self::Timeout {
            message: message.into(),
            operation: Some(operation.into()),
            duration: Some(duration),
            context: None,
        }
    }

    /// Create a resource exhaustion error
    pub fn resource_exhausted_error(
        message: impl Into<String>,
        resource_type: impl Into<String>,
        limit: usize,
        current: usize,
    ) -> Self {
        Self::ResourceExhausted {
            message: message.into(),
            resource_type: Some(resource_type.into()),
            limit: Some(limit),
            current: Some(current),
            context: None,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Config { .. } | Self::Protocol { .. } => ErrorSeverity::High,
            Self::Network { .. } | Self::Service { .. } => ErrorSeverity::Medium,
            Self::Auth { .. } | Self::Authorization { .. } => ErrorSeverity::High,
            Self::Audit { .. } => ErrorSeverity::Low,
            Self::ContentFilter { .. } | Self::Antivirus { .. } => ErrorSeverity::Medium,
            Self::Timeout { .. } => ErrorSeverity::Medium,
            Self::ResourceExhausted { .. } => ErrorSeverity::High,
            Self::Io(_) | Self::Http(_) | Self::Url(_) | Self::Json(_) | Self::Yaml(_) => {
                ErrorSeverity::Medium
            }
            Self::Anyhow(_) => ErrorSeverity::Low,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Network { .. } | Self::Service { .. } | Self::Timeout { .. } => true,
            Self::ResourceExhausted { .. } => false,
            _ => false,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - system failure
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "low"),
            ErrorSeverity::Medium => write!(f, "medium"),
            ErrorSeverity::High => write!(f, "high"),
            ErrorSeverity::Critical => write!(f, "critical"),
        }
    }
}

// Backward compatibility helpers
impl IcapError {
    /// Create a simple protocol error (backward compatibility)
    pub fn protocol_simple(message: impl Into<String>) -> Self {
        Self::Protocol {
            message: message.into(),
            protocol: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple network error (backward compatibility)
    pub fn network_simple(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            address: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple service error (backward compatibility)
    pub fn service_simple(message: impl Into<String>) -> Self {
        Self::Service {
            message: message.into(),
            service: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple config error (backward compatibility)
    pub fn config_simple(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            context: None,
            source: None,
        }
    }

    /// Create a content filter error
    pub fn content_filter_error(message: impl Into<String>) -> Self {
        Self::ContentFilter {
            message: message.into(),
            filter_type: None,
            content_type: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple resource exhausted error
    pub fn resource_exhausted_simple(message: impl Into<String>) -> Self {
        Self::ResourceExhausted {
            message: message.into(),
            resource_type: Some("unknown".to_string()),
            limit: Some(0),
            current: Some(0),
            context: None,
        }
    }
}
