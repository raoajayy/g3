//! ICAP Request Handlers
//!
//! This module contains various ICAP request handlers for different use cases.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse};
use crate::protocol::reqmod::ReqmodHandler;
use crate::protocol::respmod::RespmodHandler;
use http::{HeaderMap, StatusCode, Version};
use bytes::Bytes;
use async_trait::async_trait;

/// Default REQMOD handler
pub struct DefaultReqmodHandler;

#[async_trait]
impl ReqmodHandler for DefaultReqmodHandler {
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Default implementation - just pass through the request
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        })
    }
}

/// Default RESPMOD handler
pub struct DefaultRespmodHandler;

#[async_trait]
impl RespmodHandler for DefaultRespmodHandler {
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Default implementation - just pass through the request
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: None,
        })
    }
}

/// Content filtering handler
pub struct ContentFilterHandler {
    /// Blocked domains
    blocked_domains: Vec<String>,
    /// Blocked keywords
    blocked_keywords: Vec<String>,
}

impl ContentFilterHandler {
    /// Create a new content filter handler
    pub fn new(blocked_domains: Vec<String>, blocked_keywords: Vec<String>) -> Self {
        Self {
            blocked_domains,
            blocked_keywords,
        }
    }
}

#[async_trait]
impl ReqmodHandler for ContentFilterHandler {
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Check if the request should be blocked
        if self.should_block_request(request) {
            return Ok(IcapResponse {
                status: StatusCode::FORBIDDEN,
                version: Version::HTTP_11,
                headers: HeaderMap::new(),
                body: Bytes::from("Request blocked by content filter"),
                encapsulated: None,
            });
        }

        // Allow the request - copy relevant headers from request
        let mut response_headers = HeaderMap::new();
        
        // Copy host header if present
        if let Some(host) = request.headers.get("host") {
            response_headers.insert("host", host.clone());
        }
        
        // Copy encapsulated header if present - preserve original values
        if let Some(encapsulated) = request.headers.get("encapsulated") {
            response_headers.insert("encapsulated", encapsulated.clone());
        }
        
        // Copy other relevant headers
        for (name, value) in request.headers.iter() {
            if name.as_str() == "x-client-ip" || name.as_str() == "x-client-port" {
                response_headers.insert(name, value.clone());
            }
        }
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: response_headers,
            body: Bytes::new(),
            encapsulated: None,
        })
    }
}

#[async_trait]
impl RespmodHandler for ContentFilterHandler {
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Check if the response should be blocked
        if self.should_block_response(request) {
            return Ok(IcapResponse {
                status: StatusCode::FORBIDDEN,
                version: Version::HTTP_11,
                headers: HeaderMap::new(),
                body: Bytes::from("Response blocked by content filter"),
                encapsulated: None,
            });
        }

        // Allow the response - copy relevant headers from request
        let mut response_headers = HeaderMap::new();
        
        // Copy host header if present
        if let Some(host) = request.headers.get("host") {
            response_headers.insert("host", host.clone());
        }
        
        // Copy encapsulated header if present - preserve original values
        if let Some(encapsulated) = request.headers.get("encapsulated") {
            response_headers.insert("encapsulated", encapsulated.clone());
        }
        
        // Copy other relevant headers
        for (name, value) in request.headers.iter() {
            if name.as_str() == "x-client-ip" || name.as_str() == "x-client-port" {
                response_headers.insert(name, value.clone());
            }
        }
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: response_headers,
            body: Bytes::new(),
            encapsulated: None,
        })
    }
}

impl ContentFilterHandler {
    /// Check if a request should be blocked
    fn should_block_request(&self, request: &IcapRequest) -> bool {
        // Check blocked domains
        if let Some(host) = request.headers.get("host") {
            if let Ok(host_str) = host.to_str() {
                for domain in &self.blocked_domains {
                    if host_str.contains(domain) {
                        return true;
                    }
                }
            }
        }

        // Check blocked keywords in URI
        let path = request.uri.path().to_lowercase();
        if !path.is_empty() {
            for keyword in &self.blocked_keywords {
                if path.contains(&keyword.to_lowercase()) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a response should be blocked
    fn should_block_response(&self, request: &IcapRequest) -> bool {
        // Check blocked keywords in response body
        if let Some(encapsulated) = &request.encapsulated {
            if let Some(body) = &encapsulated.res_body {
                let body_str = String::from_utf8_lossy(body);
                for keyword in &self.blocked_keywords {
                    if body_str.to_lowercase().contains(&keyword.to_lowercase()) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
