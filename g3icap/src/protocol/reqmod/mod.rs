//! REQMOD (Request Modification) implementation
//!
//! This module handles the REQMOD ICAP method for modifying HTTP requests
//! in compliance with RFC 3507.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse, IcapMethod};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, StatusCode, Version};
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
        // Build response headers
        let mut headers = HeaderMap::new();

        // ISTag header for cache validation (RFC 3507 §4.3)
        headers.insert(
            "ISTag",
            HeaderValue::from_static("\"DefaultReqmodHandler-1\""),
        );

        // Encapsulated header is mandatory (RFC 3507 §4.2).
        // No HTTP message or service-info body is returned => null-body=0
        headers.insert(
            "Encapsulated",
            HeaderValue::from_static("null-body=0"),
        );

        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            // encapsulated field may track request parts, but actual offsets are declared via Encapsulated header
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
            // Build error response headers
            let mut headers = HeaderMap::new();
            headers.insert("ISTag", HeaderValue::from_static("\"ErrorHandler-1\""));
            headers.insert("Encapsulated", HeaderValue::from_static("null-body=0"));

            return Err(IcapError::protocol_error("Expected REQMOD method", "ICAP"));
        }

        // Delegate to handler
        let mut resp = self.handler.handle_reqmod(request).await?;

        // Ensure ISTag and Encapsulated are present even if handler omitted them
        if !resp.headers.contains_key("ISTag") {
            resp.headers.insert(
                "ISTag",
                HeaderValue::from_static("\"DefaultReqmodHandler-1\""),
            );
        }
        if !resp.headers.contains_key("Encapsulated") {
            resp.headers.insert(
                "Encapsulated",
                HeaderValue::from_static("null-body=0"),
            );
        }

        Ok(resp)
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

        // Verify the response has the required headers
        assert!(response.headers.contains_key("Encapsulated"));
        assert_eq!(response.headers.get("Encapsulated").unwrap(), "null-body=0");
        assert!(response.headers.contains_key("ISTag"));
        assert_eq!(response.headers.get("ISTag").unwrap(), "\"DefaultReqmodHandler-1\"");
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

    #[tokio::test]
    async fn test_reqmod_service_ensures_required_headers() {
        // Create a custom handler that doesn't include required headers
        struct CustomHandler;
        #[async_trait]
        impl ReqmodHandler for CustomHandler {
            async fn handle_reqmod(&self, _request: &IcapRequest) -> Result<IcapResponse, IcapError> {
                Ok(IcapResponse {
                    status: StatusCode::NO_CONTENT,
                    version: Version::HTTP_11,
                    headers: HeaderMap::new(), // Missing required headers
                    body: Bytes::new(),
                    encapsulated: None,
                })
            }
        }

        let service = ReqmodService::new(Box::new(CustomHandler));

        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the service
        let response = service.process_request(&request).await.unwrap();

        // Verify the service added the required headers
        assert!(response.headers.contains_key("ISTag"));
        assert!(response.headers.contains_key("Encapsulated"));
        assert_eq!(response.headers.get("Encapsulated").unwrap(), "null-body=0");
    }
}