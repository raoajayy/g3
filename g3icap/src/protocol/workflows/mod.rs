//! Enhanced REQMOD/RESPMOD Processing Workflows
//! 
//! This module provides comprehensive processing workflows for ICAP REQMOD and RESPMOD
//! operations, including content filtering, request/response modification, and audit logging.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse, IcapMethod, EncapsulatedData};
use crate::protocol::streaming::ContentFilter;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};
use std::time::Instant;

/// REQMOD processing workflow
pub struct ReqmodWorkflow {
    content_filters: Vec<Box<dyn ContentFilter + Send + Sync>>,
    audit_logger: Option<Box<dyn AuditLogger + Send + Sync>>,
    max_request_size: usize,
    allow_modifications: bool,
}

impl ReqmodWorkflow {
    /// Create a new REQMOD workflow
    pub fn new(max_request_size: usize) -> Self {
        Self {
            content_filters: Vec::new(),
            audit_logger: None,
            max_request_size,
            allow_modifications: true,
        }
    }
    
    /// Add a content filter to the workflow
    pub fn add_content_filter<F>(&mut self, filter: F)
    where
        F: ContentFilter + Send + Sync + 'static,
    {
        self.content_filters.push(Box::new(filter));
    }
    
    /// Set audit logger
    pub fn set_audit_logger<L>(&mut self, logger: L)
    where
        L: AuditLogger + Send + Sync + 'static,
    {
        self.audit_logger = Some(Box::new(logger));
    }
    
    /// Process a REQMOD request
    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let start_time = Instant::now();
        
        // Log request start
        if let Some(ref logger) = self.audit_logger {
            logger.log_request_start(request).await?;
        }
        
        // Validate request
        self.validate_request(request)?;
        
        // Extract HTTP request from encapsulated data
        let http_request = self.extract_http_request(request)?;
        
        // Apply content filters
        let modified_request = self.apply_content_filters(&http_request).await?;
        
        // Check if request should be blocked
        if let Some(block_reason) = self.should_block_request(&modified_request).await? {
            return self.create_blocked_response(request, &block_reason).await;
        }
        
        // Create response based on modifications
        let response = if self.has_modifications(&http_request, &modified_request) {
            self.create_modified_response(request, &modified_request).await?
        } else {
            self.create_no_content_response(request).await?
        };
        
        // Log request completion
        if let Some(ref logger) = self.audit_logger {
            logger.log_request_completion(request, &response, start_time.elapsed()).await?;
        }
        
        Ok(response)
    }
    
    /// Validate the REQMOD request
    fn validate_request(&self, request: &IcapRequest) -> Result<(), IcapError> {
        if request.method != IcapMethod::Reqmod {
            return Err(IcapError::protocol_error("Expected REQMOD method", "WORKFLOW"));
        }
        
        if request.encapsulated.is_none() {
            return Err(IcapError::protocol_error("REQMOD request must have encapsulated data", "WORKFLOW"));
        }
        
        // Check request size
        if let Some(ref encapsulated) = request.encapsulated {
            if let Some(ref req_body) = encapsulated.req_body {
                if req_body.len() > self.max_request_size {
                    return Err(IcapError::resource_exhausted_simple(
                        &format!("Request body too large: {} bytes (max: {})", req_body.len(), self.max_request_size)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract HTTP request from encapsulated data
    fn extract_http_request(&self, request: &IcapRequest) -> Result<HttpRequest, IcapError> {
        let encapsulated = request.encapsulated.as_ref()
            .ok_or_else(|| IcapError::protocol_error("No encapsulated data", "WORKFLOW"))?;
        
        let headers = encapsulated.req_hdr.as_ref()
            .ok_or_else(|| IcapError::protocol_error("No request headers in encapsulated data", "WORKFLOW"))?;
        
        let body = encapsulated.req_body.as_ref()
            .map(|b| b.to_vec())
            .unwrap_or_default();
        
        // Parse request line from headers
        let request_line = self.parse_request_line(headers)?;
        
        Ok(HttpRequest {
            method: request_line.method,
            uri: request_line.uri,
            version: request_line.version,
            headers: headers.clone(),
            body: Bytes::from(body),
        })
    }
    
    /// Parse request line from headers
    fn parse_request_line(&self, headers: &HeaderMap) -> Result<RequestLine, IcapError> {
        // This is a simplified parser - in production, you'd want more robust parsing
        // For now, we'll extract from the first header or use defaults
        let method = "GET".to_string(); // Default method
        let uri = "/".to_string(); // Default URI
        let version = Version::HTTP_11; // Default version
        
        Ok(RequestLine { method, uri, version })
    }
    
    /// Apply content filters to the request
    async fn apply_content_filters(&self, request: &HttpRequest) -> Result<HttpRequest, IcapError> {
        let mut modified_request = request.clone();
        
        for filter in &self.content_filters {
            // Apply filter to request body
            if !modified_request.body.is_empty() {
                let filtered_body = filter.filter_request_data(&modified_request.body).await
                    .map_err(|e| IcapError::content_filter_error(&e.to_string()))?;
                modified_request.body = filtered_body;
            }
            
            // Apply filter to headers
            modified_request = self.apply_header_filters(modified_request, filter.as_ref()).await?;
        }
        
        Ok(modified_request)
    }
    
    /// Apply header-specific filters
    async fn apply_header_filters(&self, mut request: HttpRequest, filter: &dyn ContentFilter) -> Result<HttpRequest, IcapError> {
        // Convert headers to bytes for filtering
        let mut header_bytes = Vec::new();
        for (name, value) in &request.headers {
            header_bytes.extend_from_slice(name.as_str().as_bytes());
            header_bytes.extend_from_slice(b": ");
            header_bytes.extend_from_slice(value.as_bytes());
            header_bytes.extend_from_slice(b"\r\n");
        }
        header_bytes.extend_from_slice(b"\r\n");
        
        // Apply filter
        let filtered_headers = filter.filter_request_data(&header_bytes).await
            .map_err(|e| IcapError::content_filter_error(&e.to_string()))?;
        
        // Parse filtered headers back
        request.headers = self.parse_headers_from_bytes(&filtered_headers)?;
        
        Ok(request)
    }
    
    /// Check if request should be blocked
    async fn should_block_request(&self, request: &HttpRequest) -> Result<Option<String>, IcapError> {
        // Check URL patterns
        if self.is_blocked_url(&request.uri) {
            return Ok(Some("Blocked URL pattern".to_string()));
        }
        
        // Check content patterns
        if !request.body.is_empty() {
            let content_str = String::from_utf8_lossy(&request.body);
            if self.contains_blocked_content(&content_str) {
                return Ok(Some("Blocked content pattern".to_string()));
            }
        }
        
        // Check headers
        for (name, value) in &request.headers {
            if self.is_blocked_header(name.as_str(), value.as_bytes()) {
                return Ok(Some("Blocked header pattern".to_string()));
            }
        }
        
        Ok(None)
    }
    
    /// Check if URL should be blocked
    fn is_blocked_url(&self, uri: &str) -> bool {
        // Simple URL blocking logic - in production, use regex or more sophisticated matching
        let blocked_patterns = [
            "malware.com",
            "phishing.net",
            "spam.org",
        ];
        
        blocked_patterns.iter().any(|pattern| uri.contains(pattern))
    }
    
    /// Check if content contains blocked patterns
    fn contains_blocked_content(&self, content: &str) -> bool {
        let blocked_keywords = [
            "malware",
            "virus",
            "phishing",
            "spam",
        ];
        
        blocked_keywords.iter().any(|keyword| content.to_lowercase().contains(keyword))
    }
    
    /// Check if header should be blocked
    fn is_blocked_header(&self, name: &str, value: &[u8]) -> bool {
        // Block suspicious headers
        if name.to_lowercase() == "x-suspicious" {
            return true;
        }
        
        // Check for suspicious values
        let value_str = String::from_utf8_lossy(value);
        value_str.contains("malicious") || value_str.contains("suspicious")
    }
    
    /// Check if request has modifications
    fn has_modifications(&self, original: &HttpRequest, modified: &HttpRequest) -> bool {
        original.body != modified.body || original.headers != modified.headers
    }
    
    /// Create a blocked response
    async fn create_blocked_response(&self, request: &IcapRequest, reason: &str) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-blocked\"".parse().unwrap());
        headers.insert("X-Block-Reason", reason.parse().unwrap());
        
        Ok(IcapResponse {
            status: StatusCode::FORBIDDEN,
            version: request.version,
            headers,
            body: Bytes::from(format!("Request blocked: {}", reason)),
            encapsulated: None,
        })
    }
    
    /// Create a modified response
    async fn create_modified_response(&self, request: &IcapRequest, modified_request: &HttpRequest) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-modified\"".parse().unwrap());
        
        // Create encapsulated data with modified request
        let encapsulated = EncapsulatedData {
            req_hdr: Some(self.create_request_headers(modified_request)?),
            req_body: Some(modified_request.body.clone()),
            res_hdr: None,
            res_body: None,
            null_body: false,
        };
        
        Ok(IcapResponse {
            status: StatusCode::OK,
            version: request.version,
            headers,
            body: Bytes::new(),
            encapsulated: Some(encapsulated),
        })
    }
    
    /// Create a no-content response
    async fn create_no_content_response(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-unchanged\"".parse().unwrap());
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: request.version,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        })
    }
    
    /// Create request headers from HTTP request
    fn create_request_headers(&self, request: &HttpRequest) -> Result<HeaderMap, IcapError> {
        let mut headers = HeaderMap::new();
        
        // Add request line as first header
        let request_line = format!("{} {} {:?}", request.method, request.uri, request.version);
        headers.insert("X-Request-Line", request_line.parse().unwrap());
        
        // Add other headers
        for (name, value) in &request.headers {
            headers.insert(name, value.clone());
        }
        
        Ok(headers)
    }
    
    /// Parse headers from bytes
    fn parse_headers_from_bytes(&self, data: &[u8]) -> Result<HeaderMap, IcapError> {
        let mut headers = HeaderMap::new();
        
        let data_str = std::str::from_utf8(data)
            .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8 in headers: {}", e), "WORKFLOW"))?;
        
        for line in data_str.lines() {
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                
                if let (Ok(header_name), Ok(header_value)) = (name.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
                    headers.insert(header_name, header_value);
                }
            }
        }
        
        Ok(headers)
    }
}

/// RESPMOD processing workflow
pub struct RespmodWorkflow {
    content_filters: Vec<Box<dyn ContentFilter + Send + Sync>>,
    audit_logger: Option<Box<dyn AuditLogger + Send + Sync>>,
    max_response_size: usize,
    allow_modifications: bool,
}

impl RespmodWorkflow {
    /// Create a new RESPMOD workflow
    pub fn new(max_response_size: usize) -> Self {
        Self {
            content_filters: Vec::new(),
            audit_logger: None,
            max_response_size,
            allow_modifications: true,
        }
    }
    
    /// Add a content filter to the workflow
    pub fn add_content_filter<F>(&mut self, filter: F)
    where
        F: ContentFilter + Send + Sync + 'static,
    {
        self.content_filters.push(Box::new(filter));
    }
    
    /// Set audit logger
    pub fn set_audit_logger<L>(&mut self, logger: L)
    where
        L: AuditLogger + Send + Sync + 'static,
    {
        self.audit_logger = Some(Box::new(logger));
    }
    
    /// Process a RESPMOD request
    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let start_time = Instant::now();
        
        // Log request start
        if let Some(ref logger) = self.audit_logger {
            logger.log_request_start(request).await?;
        }
        
        // Validate request
        self.validate_request(request)?;
        
        // Extract HTTP request and response from encapsulated data
        let (http_request, http_response) = self.extract_http_request_and_response(request)?;
        
        // Apply content filters
        let modified_response = self.apply_content_filters(&http_request, &http_response).await?;
        
        // Check if response should be blocked
        if let Some(block_reason) = self.should_block_response(&http_request, &modified_response).await? {
            return self.create_blocked_response(request, &block_reason).await;
        }
        
        // Create response based on modifications
        let response = if self.has_modifications(&http_response, &modified_response) {
            self.create_modified_response(request, &http_request, &modified_response).await?
        } else {
            self.create_no_content_response(request).await?
        };
        
        // Log request completion
        if let Some(ref logger) = self.audit_logger {
            logger.log_request_completion(request, &response, start_time.elapsed()).await?;
        }
        
        Ok(response)
    }
    
    /// Validate the RESPMOD request
    fn validate_request(&self, request: &IcapRequest) -> Result<(), IcapError> {
        if request.method != IcapMethod::Respmod {
            return Err(IcapError::protocol_error("Expected RESPMOD method", "WORKFLOW"));
        }
        
        if request.encapsulated.is_none() {
            return Err(IcapError::protocol_error("RESPMOD request must have encapsulated data", "WORKFLOW"));
        }
        
        // Check response size
        if let Some(ref encapsulated) = request.encapsulated {
            if let Some(ref res_body) = encapsulated.res_body {
                if res_body.len() > self.max_response_size {
                    return Err(IcapError::resource_exhausted_simple(
                        &format!("Response body too large: {} bytes (max: {})", res_body.len(), self.max_response_size)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract HTTP request and response from encapsulated data
    fn extract_http_request_and_response(&self, request: &IcapRequest) -> Result<(HttpRequest, HttpResponse), IcapError> {
        let encapsulated = request.encapsulated.as_ref()
            .ok_or_else(|| IcapError::protocol_error("No encapsulated data", "WORKFLOW"))?;
        
        // Extract request
        let req_headers = encapsulated.req_hdr.as_ref()
            .ok_or_else(|| IcapError::protocol_error("No request headers in encapsulated data", "WORKFLOW"))?;
        
        let req_body = encapsulated.req_body.as_ref()
            .map(|b| b.to_vec())
            .unwrap_or_default();
        
        let http_request = HttpRequest {
            method: "GET".to_string(), // Simplified
            uri: "/".to_string(),
            version: Version::HTTP_11,
            headers: req_headers.clone(),
            body: Bytes::from(req_body),
        };
        
        // Extract response
        let res_headers = encapsulated.res_hdr.as_ref()
            .ok_or_else(|| IcapError::protocol_error("No response headers in encapsulated data", "WORKFLOW"))?;
        
        let res_body = encapsulated.res_body.as_ref()
            .map(|b| b.to_vec())
            .unwrap_or_default();
        
        let http_response = HttpResponse {
            status_code: 200, // Simplified
            headers: res_headers.clone(),
            body: Bytes::from(res_body),
        };
        
        Ok((http_request, http_response))
    }
    
    /// Apply content filters to the response
    async fn apply_content_filters(&self, request: &HttpRequest, response: &HttpResponse) -> Result<HttpResponse, IcapError> {
        let mut modified_response = response.clone();
        
        for filter in &self.content_filters {
            // Apply filter to response body
            if !modified_response.body.is_empty() {
                let filtered_body = filter.filter_response_data(&modified_response.body).await
                    .map_err(|e| IcapError::content_filter_error(&e.to_string()))?;
                modified_response.body = filtered_body;
            }
            
            // Apply filter to headers
            modified_response = self.apply_header_filters(modified_response, filter.as_ref()).await?;
        }
        
        Ok(modified_response)
    }
    
    /// Apply header-specific filters
    async fn apply_header_filters(&self, mut response: HttpResponse, filter: &dyn ContentFilter) -> Result<HttpResponse, IcapError> {
        // Convert headers to bytes for filtering
        let mut header_bytes = Vec::new();
        for (name, value) in &response.headers {
            header_bytes.extend_from_slice(name.as_str().as_bytes());
            header_bytes.extend_from_slice(b": ");
            header_bytes.extend_from_slice(value.as_bytes());
            header_bytes.extend_from_slice(b"\r\n");
        }
        header_bytes.extend_from_slice(b"\r\n");
        
        // Apply filter
        let filtered_headers = filter.filter_response_data(&header_bytes).await
            .map_err(|e| IcapError::content_filter_error(&e.to_string()))?;
        
        // Parse filtered headers back
        response.headers = self.parse_headers_from_bytes(&filtered_headers)?;
        
        Ok(response)
    }
    
    /// Check if response should be blocked
    async fn should_block_response(&self, request: &HttpRequest, response: &HttpResponse) -> Result<Option<String>, IcapError> {
        // Check content patterns
        if !response.body.is_empty() {
            let content_str = String::from_utf8_lossy(&response.body);
            if self.contains_blocked_content(&content_str) {
                return Ok(Some("Blocked content pattern in response".to_string()));
            }
        }
        
        // Check headers
        for (name, value) in &response.headers {
            if self.is_blocked_header(name.as_str(), value.as_bytes()) {
                return Ok(Some("Blocked header pattern in response".to_string()));
            }
        }
        
        Ok(None)
    }
    
    /// Check if content contains blocked patterns
    fn contains_blocked_content(&self, content: &str) -> bool {
        let blocked_keywords = [
            "malware",
            "virus",
            "phishing",
            "spam",
            "malicious",
        ];
        
        blocked_keywords.iter().any(|keyword| content.to_lowercase().contains(keyword))
    }
    
    /// Check if header should be blocked
    fn is_blocked_header(&self, name: &str, value: &[u8]) -> bool {
        // Block suspicious headers
        if name.to_lowercase() == "x-malicious" {
            return true;
        }
        
        // Check for suspicious values
        let value_str = String::from_utf8_lossy(value);
        value_str.contains("malicious") || value_str.contains("suspicious")
    }
    
    /// Check if response has modifications
    fn has_modifications(&self, original: &HttpResponse, modified: &HttpResponse) -> bool {
        original.body != modified.body || original.headers != modified.headers
    }
    
    /// Create a blocked response
    async fn create_blocked_response(&self, request: &IcapRequest, reason: &str) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-blocked\"".parse().unwrap());
        headers.insert("X-Block-Reason", reason.parse().unwrap());
        
        Ok(IcapResponse {
            status: StatusCode::FORBIDDEN,
            version: request.version,
            headers,
            body: Bytes::from(format!("Response blocked: {}", reason)),
            encapsulated: None,
        })
    }
    
    /// Create a modified response
    async fn create_modified_response(&self, request: &IcapRequest, http_request: &HttpRequest, modified_response: &HttpResponse) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-modified\"".parse().unwrap());
        
        // Create encapsulated data with modified response
        let encapsulated = EncapsulatedData {
            req_hdr: Some(self.create_request_headers(http_request)?),
            req_body: Some(http_request.body.clone()),
            res_hdr: Some(self.create_response_headers(modified_response)?),
            res_body: Some(modified_response.body.clone()),
            null_body: false,
        };
        
        Ok(IcapResponse {
            status: StatusCode::OK,
            version: request.version,
            headers,
            body: Bytes::new(),
            encapsulated: Some(encapsulated),
        })
    }
    
    /// Create a no-content response
    async fn create_no_content_response(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-unchanged\"".parse().unwrap());
        
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: request.version,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        })
    }
    
    /// Create request headers from HTTP request
    fn create_request_headers(&self, request: &HttpRequest) -> Result<HeaderMap, IcapError> {
        let mut headers = HeaderMap::new();
        
        // Add request line as first header
        let request_line = format!("{} {} {:?}", request.method, request.uri, request.version);
        headers.insert("X-Request-Line", request_line.parse().unwrap());
        
        // Add other headers
        for (name, value) in &request.headers {
            headers.insert(name, value.clone());
        }
        
        Ok(headers)
    }
    
    /// Create response headers from HTTP response
    fn create_response_headers(&self, response: &HttpResponse) -> Result<HeaderMap, IcapError> {
        let mut headers = HeaderMap::new();
        
        // Add status line as first header
        let status_line = format!("HTTP/1.1 {} OK", response.status_code);
        headers.insert("X-Status-Line", status_line.parse().unwrap());
        
        // Add other headers
        for (name, value) in &response.headers {
            headers.insert(name, value.clone());
        }
        
        Ok(headers)
    }
    
    /// Parse headers from bytes
    fn parse_headers_from_bytes(&self, data: &[u8]) -> Result<HeaderMap, IcapError> {
        let mut headers = HeaderMap::new();
        
        let data_str = std::str::from_utf8(data)
            .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8 in headers: {}", e), "WORKFLOW"))?;
        
        for line in data_str.lines() {
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                
                if let (Ok(header_name), Ok(header_value)) = (name.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
                    headers.insert(header_name, header_value);
                }
            }
        }
        
        Ok(headers)
    }
}

/// HTTP request structure
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub version: Version,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// HTTP response structure
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// Request line structure
#[derive(Debug, Clone)]
struct RequestLine {
    method: String,
    uri: String,
    version: Version,
}

/// Audit logger trait
#[async_trait::async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log request start
    async fn log_request_start(&self, request: &IcapRequest) -> Result<(), IcapError>;
    
    /// Log request completion
    async fn log_request_completion(&self, request: &IcapRequest, response: &IcapResponse, duration: std::time::Duration) -> Result<(), IcapError>;
}

/// Simple audit logger implementation
pub struct SimpleAuditLogger;

#[async_trait::async_trait]
impl AuditLogger for SimpleAuditLogger {
    async fn log_request_start(&self, request: &IcapRequest) -> Result<(), IcapError> {
        println!("ICAP Request: {} {}", request.method.to_string(), request.uri);
        Ok(())
    }
    
    async fn log_request_completion(&self, request: &IcapRequest, response: &IcapResponse, duration: std::time::Duration) -> Result<(), IcapError> {
        println!("ICAP Response: {} {} ({}ms)", 
                 response.status, 
                 request.uri, 
                 duration.as_millis());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::streaming::PassThroughFilter;
    
    #[tokio::test]
    async fn test_reqmod_workflow() {
        let mut workflow = ReqmodWorkflow::new(1024 * 1024); // 1MB limit
        workflow.add_content_filter(PassThroughFilter);
        workflow.set_audit_logger(SimpleAuditLogger);
        
        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: Some(EncapsulatedData {
                req_hdr: Some(HeaderMap::new()),
                req_body: Some(Bytes::from("test content")),
                res_hdr: None,
                res_body: None,
                null_body: false,
            }),
        };
        
        let response = workflow.process_request(&request).await.unwrap();
        assert_eq!(response.status, StatusCode::NO_CONTENT);
    }
    
    #[tokio::test]
    async fn test_respmod_workflow() {
        let mut workflow = RespmodWorkflow::new(1024 * 1024); // 1MB limit
        workflow.add_content_filter(PassThroughFilter);
        workflow.set_audit_logger(SimpleAuditLogger);
        
        // Create a test request
        let request = IcapRequest {
            method: IcapMethod::Respmod,
            uri: "icap://example.com/respmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: Some(EncapsulatedData {
                req_hdr: Some(HeaderMap::new()),
                req_body: Some(Bytes::from("request content")),
                res_hdr: Some(HeaderMap::new()),
                res_body: Some(Bytes::from("response content")),
                null_body: false,
            }),
        };
        
        let response = workflow.process_request(&request).await.unwrap();
        assert_eq!(response.status, StatusCode::NO_CONTENT);
    }
    
    #[tokio::test]
    async fn test_blocked_request() {
        let mut workflow = ReqmodWorkflow::new(1024 * 1024);
        workflow.add_content_filter(PassThroughFilter);
        
        // Create a request with blocked content
        let request = IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: Some(EncapsulatedData {
                req_hdr: Some(HeaderMap::new()),
                req_body: Some(Bytes::from("This contains malware content")),
                res_hdr: None,
                res_body: None,
                null_body: false,
            }),
        };
        
        let response = workflow.process_request(&request).await.unwrap();
        assert_eq!(response.status, StatusCode::FORBIDDEN);
    }
}
