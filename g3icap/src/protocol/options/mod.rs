//! OPTIONS method implementation
//!
//! This module handles the OPTIONS ICAP method for service discovery.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse};
use crate::service::ServiceRegistry;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use chrono::Utc;

/// OPTIONS handler trait
#[async_trait]
pub trait OptionsHandler: Send + Sync {
    /// Handle OPTIONS request
    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError>;
}

/// Default OPTIONS handler implementation
pub struct DefaultOptionsHandler {
    /// Service registry
    registry: Arc<RwLock<ServiceRegistry>>,
}

impl DefaultOptionsHandler {
    /// Create a new default OPTIONS handler
    pub fn new(registry: Arc<RwLock<ServiceRegistry>>) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl OptionsHandler for DefaultOptionsHandler {
    async fn handle_options(&self, _request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let registry = self.registry.read().await;
        let services = registry.list_services().await;

        // Build service information
        let mut methods = Vec::new();
        let mut service_info = String::new();

        for service in services {
            methods.extend(service.methods.iter().map(|m| m.to_string()));
            service_info.push_str(&format!("{}: {}\n", service.name, service.description));
        }

        methods.sort();
        methods.dedup();

        // Create response headers
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-1.0\"".parse().unwrap());
        headers.insert("Methods", methods.join(", ").parse().unwrap());
        headers.insert("Service", "G3 ICAP Server".parse().unwrap());
        headers.insert("Service-ID", "g3icap".parse().unwrap());
        headers.insert("Max-Connections", "1000".parse().unwrap());
        headers.insert("Options-TTL", "3600".parse().unwrap());
        headers.insert("Allow", "204".parse().unwrap());
        headers.insert("Preview", "1024".parse().unwrap());
        headers.insert("Transfer-Preview", "*".parse().unwrap());
        headers.insert("Transfer-Complete", "exe,com,bat".parse().unwrap());
        headers.insert(
            "Date",
            Utc::now()
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string()
                .parse()
                .unwrap(),
        );
        // Required Encapsulated header for OPTIONS responses
        headers.insert("Encapsulated", "opt-body=0".parse().unwrap());

        Ok(IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(service_info),
            encapsulated: None,
        })
    }
}

/// OPTIONS service
pub struct OptionsService {
    handler: Box<dyn OptionsHandler>,
}

impl OptionsService {
    /// Create a new OPTIONS service
    pub fn new(handler: Box<dyn OptionsHandler>) -> Self {
        Self { handler }
    }

    /// Process OPTIONS request
    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Validate that this is an OPTIONS request
        if request.method != crate::protocol::common::IcapMethod::Options {
            return Err(IcapError::protocol_error("Expected OPTIONS method", "ICAP"));
        }

        // Call the handler
        self.handler.handle_options(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::ServiceRegistry;
    use http::Uri;

    #[tokio::test]
    async fn test_default_options_handler_has_required_headers() {
        let registry = Arc::new(RwLock::new(ServiceRegistry::new()));
        let handler = DefaultOptionsHandler::new(registry);
        
        // Create a test request
        let request = IcapRequest {
            method: crate::protocol::common::IcapMethod::Options,
            uri: "icap://example.com/options".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the handler
        let response = handler.handle_options(&request).await.unwrap();

        // Verify the response has the required headers
        assert!(response.headers.contains_key("ISTag"));
        assert!(response.headers.contains_key("Methods"));
        assert!(response.headers.contains_key("Service"));
        assert!(response.headers.contains_key("Service-ID"));
        assert!(response.headers.contains_key("Encapsulated"));
        assert_eq!(response.headers.get("Encapsulated").unwrap(), "opt-body=0");
        assert_eq!(response.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_options_service_validates_method() {
        let registry = Arc::new(RwLock::new(ServiceRegistry::new()));
        let handler = DefaultOptionsHandler::new(registry);
        let service = OptionsService::new(Box::new(handler));

        // Create a test request with wrong method
        let request = IcapRequest {
            method: crate::protocol::common::IcapMethod::Reqmod, // Wrong method
            uri: "icap://example.com/options".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the service - should return an error
        let result = service.process_request(&request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected OPTIONS method"));
    }
}