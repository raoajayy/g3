//! RESPMOD (Response Modification) implementation

use crate::error::IcapError;
use crate::protocol::common::{EncapsulatedData, IcapRequest, IcapResponse, IcapMethod};
use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, StatusCode, Version};

/// RESPMOD handler trait
#[async_trait]
pub trait RespmodHandler: Send + Sync {
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError>;
}

/// Default RESPMOD handler - pass through.
pub struct DefaultRespmodHandler;

#[async_trait]
impl RespmodHandler for DefaultRespmodHandler {
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", HeaderValue::from_static("\"DefaultRespmod-1\""));
        headers.insert("Encapsulated", HeaderValue::from_static("null-body=0"));

        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: Some(EncapsulatedData {
                req_hdr: None,
                req_body: None,
                res_hdr: None,
                res_body: None,
                null_body: true,
            }),
        })
    }
}

/// RESPMOD service
pub struct RespmodService {
    handler: Box<dyn RespmodHandler>,
}

impl RespmodService {
    pub fn new(handler: Box<dyn RespmodHandler>) -> Self {
        Self { handler }
    }

    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        if request.method != IcapMethod::Respmod {
            let mut headers = HeaderMap::new();
            headers.insert("ISTag", HeaderValue::from_static("\"RespmodService-Err\""));
            headers.insert("Encapsulated", HeaderValue::from_static("null-body=0"));
            return Err(IcapError::protocol_error("Expected RESPMOD", "ICAP"));
        }

        let mut resp = self.handler.handle_respmod(request).await?;
        if !resp.headers.contains_key("ISTag") {
            resp.headers.insert("ISTag", HeaderValue::from_static("\"DefaultRespmod-1\""));
        }
        if !resp.headers.contains_key("Encapsulated") {
            resp.headers.insert("Encapsulated", HeaderValue::from_static("null-body=0"));
        }
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Uri;

    #[tokio::test]
    async fn test_default_respmod_handler_has_required_headers() {
        let handler = DefaultRespmodHandler;
        
        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Respmod,
            uri: "icap://example.com/respmod".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the handler
        let response = handler.handle_respmod(&request).await.unwrap();

        // Verify the response has the required headers
        assert!(response.headers.contains_key("ISTag"));
        assert_eq!(response.headers.get("ISTag").unwrap(), "\"DefaultRespmod-1\"");
        assert!(response.headers.contains_key("Encapsulated"));
        assert_eq!(response.headers.get("Encapsulated").unwrap(), "null-body=0");
        assert_eq!(response.status, StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_respmod_service_validates_method() {
        let handler = DefaultRespmodHandler;
        let service = RespmodService::new(Box::new(handler));

        // Create a test request with wrong method
        let request = IcapRequest {
            method: IcapMethod::Reqmod, // Wrong method
            uri: "icap://example.com/respmod".parse::<Uri>().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        };

        // Call the service - should return an error
        let result = service.process_request(&request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected RESPMOD"));
    }

    #[tokio::test]
    async fn test_respmod_service_ensures_required_headers() {
        // Create a custom handler that doesn't include required headers
        struct CustomHandler;
        #[async_trait]
        impl RespmodHandler for CustomHandler {
            async fn handle_respmod(&self, _request: &IcapRequest) -> Result<IcapResponse, IcapError> {
                Ok(IcapResponse {
                    status: StatusCode::NO_CONTENT,
                    version: Version::HTTP_11,
                    headers: HeaderMap::new(), // Missing required headers
                    body: Bytes::new(),
                    encapsulated: None,
                })
            }
        }

        let service = RespmodService::new(Box::new(CustomHandler));

        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Respmod,
            uri: "icap://example.com/respmod".parse::<Uri>().unwrap(),
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