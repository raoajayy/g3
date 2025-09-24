//! REQMOD (Request Modification) implementation
//!
//! This module handles the REQMOD ICAP method for modifying HTTP requests.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse, IcapMethod};
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};
use async_trait::async_trait;

/// REQMOD handler trait
#[async_trait]
pub trait ReqmodHandler: Send + Sync {
    /// Handle REQMOD request
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError>;
}

/// Default REQMOD handler implementation
pub struct DefaultReqmodHandler;

#[async_trait]
impl ReqmodHandler for DefaultReqmodHandler {
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Default implementation - just pass through the request
        // RFC 3507: Every ICAP response must include an Encapsulated header
        let mut headers = HeaderMap::new();
        headers.insert("Encapsulated", "null-body=0".parse().unwrap());
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: request.encapsulated.clone(),
        })
    }
}

/// REQMOD service
pub struct ReqmodService {
    handler: Box<dyn ReqmodHandler>,
}

impl ReqmodService {
    /// Create a new REQMOD service
    pub fn new(handler: Box<dyn ReqmodHandler>) -> Self {
        Self { handler }
    }

    /// Process REQMOD request
    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Validate that this is a REQMOD request
        if request.method != IcapMethod::Reqmod {
            return Err(IcapError::protocol_error("Expected REQMOD method", "ICAP"));
        }

        // Call the handler
        self.handler.handle_reqmod(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::common::EncapsulatedData;
    use http::Uri;

    #[tokio::test]
    async fn test_default_reqmod_handler_has_encapsulated_header() {
        let handler = DefaultReqmodHandler;
        
        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: Some(EncapsulatedData {
                req_hdr: None,
                res_hdr: None,
                req_body: None,
                res_body: None,
                null_body: true,
            }),
        };

        // Call the handler
        let response = handler.handle_reqmod(&request).await.unwrap();

        // Verify the response has the required Encapsulated header
        assert!(response.headers.contains_key("Encapsulated"));
        assert_eq!(response.headers.get("Encapsulated").unwrap(), "null-body=0");
        assert_eq!(response.status, StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_reqmod_service_validates_method() {
        let handler = DefaultReqmodHandler;
        let service = ReqmodService::new(Box::new(handler));

        // Create a test request with wrong method
        let request = IcapRequest {
            method: IcapMethod::Respmod, // Wrong method
            uri: "icap://example.com/reqmod".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the service - should return an error
        let result = service.process_request(&request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected REQMOD method"));
    }
}
