//! Error types for G3 ICAP Server

use thiserror::Error;

/// Result type for G3 ICAP operations
pub type IcapResult<T> = Result<T, IcapError>;

/// Error types for G3 ICAP Server
#[derive(Error, Debug)]
pub enum IcapError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Service error
    #[error("Service error: {0}")]
    Service(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),

    /// URL error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// YAML error
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Anyhow error
    #[error("General error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
