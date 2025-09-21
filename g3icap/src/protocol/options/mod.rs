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
        headers.insert("Max-Connections", "1000".parse().unwrap());
        headers.insert("Options-TTL", "3600".parse().unwrap());
        headers.insert("Allow", "204".parse().unwrap());
        headers.insert("Preview", "1024".parse().unwrap());
        
        // Add service-specific headers (if needed in the future)
        // Currently using default headers only
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
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
            return Err(IcapError::Protocol("Expected OPTIONS method".to_string()));
        }

        // Call the handler
        self.handler.handle_options(request).await
    }
}
