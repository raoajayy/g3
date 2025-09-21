//! ICAP Error handling and standard responses
//!
//! This module provides standard ICAP error responses and error handling utilities.

use crate::error::IcapError;
use crate::protocol::common::IcapResponse;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};

/// ICAP error response builder
pub struct ErrorResponseBuilder;

impl ErrorResponseBuilder {
    /// Create a 400 Bad Request response
    pub fn bad_request(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::BAD_REQUEST, "Bad Request", message)
    }

    /// Create a 404 Not Found response
    pub fn not_found(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::NOT_FOUND, "Not Found", message)
    }

    /// Create a 405 Method Not Allowed response
    pub fn method_not_allowed(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed", message)
    }

    /// Create a 408 Request Timeout response
    pub fn request_timeout(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::REQUEST_TIMEOUT, "Request Timeout", message)
    }

    /// Create a 413 Payload Too Large response
    pub fn payload_too_large(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::PAYLOAD_TOO_LARGE, "Payload Too Large", message)
    }

    /// Create a 500 Internal Server Error response
    pub fn internal_server_error(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", message)
    }

    /// Create a 501 Not Implemented response
    pub fn not_implemented(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::NOT_IMPLEMENTED, "Not Implemented", message)
    }

    /// Create a 502 Bad Gateway response
    pub fn bad_gateway(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::BAD_GATEWAY, "Bad Gateway", message)
    }

    /// Create a 503 Service Unavailable response
    pub fn service_unavailable(message: &str) -> IcapResponse {
        Self::create_error_response(StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable", message)
    }

    /// Create a 100 Continue response (for preview mode)
    pub fn continue_response() -> IcapResponse {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-continue\"".parse().unwrap());

        IcapResponse {
            status: StatusCode::CONTINUE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Create a 204 No Content response
    pub fn no_content() -> IcapResponse {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-no-content\"".parse().unwrap());

        IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Create a generic error response
    fn create_error_response(status: StatusCode, reason: &str, message: &str) -> IcapResponse {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-error\"".parse().unwrap());
        headers.insert("Content-Type", "text/plain".parse().unwrap());
        headers.insert("Content-Length", message.len().to_string().parse().unwrap());

        let body = format!("{}: {}", reason, message);

        IcapResponse {
            status,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }
}

/// Convert IcapError to appropriate ICAP response
pub fn error_to_response(error: &IcapError) -> IcapResponse {
    match error {
        IcapError::Config(msg) => ErrorResponseBuilder::internal_server_error(&format!("Configuration error: {}", msg)),
        IcapError::Protocol(msg) => ErrorResponseBuilder::bad_request(&format!("Protocol error: {}", msg)),
        IcapError::Network(msg) => ErrorResponseBuilder::bad_gateway(&format!("Network error: {}", msg)),
        IcapError::Service(msg) => ErrorResponseBuilder::service_unavailable(&format!("Service error: {}", msg)),
        IcapError::Io(_) => ErrorResponseBuilder::internal_server_error("I/O error occurred"),
        IcapError::Http(_) => ErrorResponseBuilder::bad_request("HTTP error occurred"),
        IcapError::Url(_) => ErrorResponseBuilder::bad_request("Invalid URL"),
        IcapError::Json(_) => ErrorResponseBuilder::bad_request("JSON parsing error"),
        IcapError::Yaml(_) => ErrorResponseBuilder::bad_request("YAML parsing error"),
        IcapError::Anyhow(_) => ErrorResponseBuilder::internal_server_error("Internal error occurred"),
    }
}

/// ICAP status code utilities
pub struct IcapStatusCodes;

impl IcapStatusCodes {
    /// Check if status code indicates success
    pub fn is_success(status: StatusCode) -> bool {
        status.is_success()
    }

    /// Check if status code indicates client error
    pub fn is_client_error(status: StatusCode) -> bool {
        status.is_client_error()
    }

    /// Check if status code indicates server error
    pub fn is_server_error(status: StatusCode) -> bool {
        status.is_server_error()
    }

    /// Get appropriate error message for status code
    pub fn get_error_message(status: StatusCode) -> &'static str {
        match status {
            StatusCode::BAD_REQUEST => "Bad Request - The request could not be understood by the server",
            StatusCode::UNAUTHORIZED => "Unauthorized - Authentication is required",
            StatusCode::FORBIDDEN => "Forbidden - The server understood the request but refuses to authorize it",
            StatusCode::NOT_FOUND => "Not Found - The requested resource was not found",
            StatusCode::METHOD_NOT_ALLOWED => "Method Not Allowed - The method is not allowed for this resource",
            StatusCode::REQUEST_TIMEOUT => "Request Timeout - The server timed out waiting for the request",
            StatusCode::PAYLOAD_TOO_LARGE => "Payload Too Large - The request entity is too large",
            StatusCode::INTERNAL_SERVER_ERROR => "Internal Server Error - The server encountered an unexpected condition",
            StatusCode::NOT_IMPLEMENTED => "Not Implemented - The server does not support the functionality required",
            StatusCode::BAD_GATEWAY => "Bad Gateway - The server received an invalid response from upstream",
            StatusCode::SERVICE_UNAVAILABLE => "Service Unavailable - The server is currently unable to handle the request",
            _ => "Unknown error occurred",
        }
    }
}
