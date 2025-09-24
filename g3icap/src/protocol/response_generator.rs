/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::collections::HashMap;
use std::fmt;

use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};

use crate::protocol::common::{EncapsulatedData, IcapMethod, IcapResponse};

/// ICAP Response Generator - Handles all ICAP response codes per RFC 3507
/// 
/// This implementation follows g3proxy standards for ICAP response handling,
/// providing consistent error handling, header management, and response formatting.
pub struct IcapResponseGenerator {
    server_name: String,
    server_version: String,
    service_id: Option<String>,
}

impl IcapResponseGenerator {
    /// Create a new response generator
    pub fn new(server_name: String, server_version: String) -> Self {
        Self {
            server_name,
            server_version,
            service_id: None,
        }
    }

    /// Create a new response generator with service ID
    pub fn with_service_id(server_name: String, server_version: String, service_id: Option<String>) -> Self {
        Self {
            server_name,
            server_version,
            service_id,
        }
    }

    /// Set the service ID for this generator
    pub fn set_service_id(&mut self, service_id: Option<String>) {
        self.service_id = service_id;
    }

    /// Generate a 100 Continue response
    pub fn continue_response(&self) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        self.add_null_body_header(&mut headers);
        
        IcapResponse {
            status: StatusCode::CONTINUE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Generate a 200 OK response with modified content
    pub fn ok_modified(&self, encapsulated: Option<EncapsulatedData>, body: Bytes) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        if let Some(enc) = &encapsulated {
            let encapsulated_header = self.serialize_encapsulated_header(enc);
            headers.insert("encapsulated", encapsulated_header.parse().unwrap());
        }

        IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers,
            body,
            encapsulated,
        }
    }

    /// Generate a 200 OK response with chunked transfer encoding
    pub fn ok_modified_chunked(&self, encapsulated: Option<EncapsulatedData>, body: Bytes) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        if let Some(enc) = &encapsulated {
            let encapsulated_header = self.serialize_encapsulated_header(enc);
            headers.insert("encapsulated", encapsulated_header.parse().unwrap());
        }

        // Add chunked transfer encoding for the HTTP response body
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers,
            body,
            encapsulated,
        }
    }

    /// Generate a 204 No Modifications response (RFC 3507 compliant)
    pub fn no_modifications(&self, encapsulated: Option<EncapsulatedData>) -> IcapResponse {
        let mut headers = HeaderMap::new();
        
        // RFC 3507: ISTag is MANDATORY for 204 responses
        headers.insert("istag", format!("\"{}\"", self.server_version).parse().unwrap());
        
        // RFC 3507: Encapsulated header is MANDATORY for 204 responses
        if let Some(enc) = &encapsulated {
            let encapsulated_header = self.serialize_encapsulated_header(enc);
            headers.insert("encapsulated", encapsulated_header.parse().unwrap());
        } else {
            headers.insert("encapsulated", "null-body=0".parse().unwrap());
        }

        IcapResponse {
            status: StatusCode::NO_CONTENT, // This maps to ICAP 204 No Modifications
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated,
        }
    }

    /// Generate a 302 Found response
    pub fn found(&self, location: &str) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        headers.insert("location", location.parse().unwrap());
        self.add_null_body_header(&mut headers);

        IcapResponse {
            status: StatusCode::FOUND,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Generate a 304 Not Modified response
    pub fn not_modified(&self) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        self.add_null_body_header(&mut headers);

        IcapResponse {
            status: StatusCode::NOT_MODIFIED,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Generate a 400 Bad Request response
    pub fn bad_request(&self, message: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(msg) = message {
            self.format_error_message(StatusCode::BAD_REQUEST, &format!("Bad Request: {}", msg))
        } else {
            self.format_error_message(StatusCode::BAD_REQUEST, "Malformed ICAP request")
        };

        IcapResponse {
            status: StatusCode::BAD_REQUEST,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 400 Bad Request response with chunked transfer encoding
    pub fn bad_request_chunked(&self, message: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(msg) = message {
            self.format_error_message(StatusCode::BAD_REQUEST, &format!("Bad Request: {}", msg))
        } else {
            self.format_error_message(StatusCode::BAD_REQUEST, "Malformed ICAP request")
        };

        IcapResponse {
            status: StatusCode::BAD_REQUEST,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 403 Forbidden response (RFC 3507 compliant)
    pub fn forbidden(&self, reason: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(reason) = reason {
            self.format_error_message(StatusCode::FORBIDDEN, &format!("Forbidden: {}", reason))
        } else {
            self.format_error_message(StatusCode::FORBIDDEN, "Access denied")
        };

        IcapResponse {
            status: StatusCode::FORBIDDEN,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 403 Forbidden response with chunked transfer encoding (RFC 3507 compliant)
    pub fn forbidden_chunked(&self, reason: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(reason) = reason {
            format!("<html><body><h1>403 Forbidden</h1><p>Access Denied - {}</p></body></html>", reason)
        } else {
            "<html><body><h1>403 Forbidden</h1><p>Access Denied - Blocked</p></body></html>".to_string()
        };

        IcapResponse {
            status: StatusCode::FORBIDDEN,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 404 Not Found response
    pub fn not_found(&self, service: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(svc) = service {
            self.format_error_message(StatusCode::NOT_FOUND, &format!("ICAP service '{}' is not available", svc))
        } else {
            self.format_error_message(StatusCode::NOT_FOUND, "ICAP service not available")
        };

        IcapResponse {
            status: StatusCode::NOT_FOUND,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 405 Method Not Allowed response
    pub fn method_not_allowed(&self, method: &IcapMethod, allowed_methods: &[IcapMethod]) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        
        let allowed_str = allowed_methods.iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        headers.insert("allow", allowed_str.parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = self.format_error_message(
            StatusCode::METHOD_NOT_ALLOWED,
            &format!("{:?} method is not allowed for this service. Allowed methods: {}", method, allowed_str)
        );

        IcapResponse {
            status: StatusCode::METHOD_NOT_ALLOWED,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 407 Proxy Authentication Required response
    pub fn proxy_auth_required(&self, realm: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        headers.insert("proxy-authenticate", 
            format!("Basic realm=\"{}\"", realm.unwrap_or("ICAP Server")).parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        IcapResponse {
            status: StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(self.format_error_message(StatusCode::PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required")),
            encapsulated: None,
        }
    }

    /// Generate a 409 Conflict response
    pub fn conflict(&self, reason: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(reason) = reason {
            self.format_error_message(StatusCode::CONFLICT, &format!("Conflict: {}", reason))
        } else {
            self.format_error_message(StatusCode::CONFLICT, "Request could not be completed due to a conflict")
        };

        IcapResponse {
            status: StatusCode::CONFLICT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 413 Request Too Large response
    pub fn request_too_large(&self, max_size: Option<usize>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        
        if let Some(size) = max_size {
            headers.insert("content-length", size.to_string().parse().unwrap());
        }

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(size) = max_size {
            self.format_error_message(StatusCode::PAYLOAD_TOO_LARGE, &format!("Request exceeds maximum size of {} bytes", size))
        } else {
            self.format_error_message(StatusCode::PAYLOAD_TOO_LARGE, "Request exceeds maximum allowed size")
        };

        IcapResponse {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 415 Unsupported Media Type response
    pub fn unsupported_media_type(&self, content_type: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(ct) = content_type {
            self.format_error_message(StatusCode::UNSUPPORTED_MEDIA_TYPE, &format!("Content type '{}' is not supported", ct))
        } else {
            self.format_error_message(StatusCode::UNSUPPORTED_MEDIA_TYPE, "Content type is not supported")
        };

        IcapResponse {
            status: StatusCode::UNSUPPORTED_MEDIA_TYPE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 500 Internal Server Error response
    pub fn internal_server_error(&self, error: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(err) = error {
            self.format_error_message(StatusCode::INTERNAL_SERVER_ERROR, &format!("Internal Server Error: {}", err))
        } else {
            self.format_error_message(StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred")
        };

        IcapResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 500 Internal Server Error response with chunked transfer encoding
    pub fn internal_server_error_chunked(&self, error: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(err) = error {
            format!("Internal scan failure: {}", err)
        } else {
            "Internal scan failure".to_string()
        };

        IcapResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 501 Not Implemented response
    pub fn not_implemented(&self, method: Option<&IcapMethod>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(m) = method {
            self.format_error_message(StatusCode::NOT_IMPLEMENTED, &format!("{:?} method is not supported", m))
        } else {
            self.format_error_message(StatusCode::NOT_IMPLEMENTED, "Request method is not supported")
        };

        IcapResponse {
            status: StatusCode::NOT_IMPLEMENTED,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 502 Bad Gateway response
    pub fn bad_gateway(&self, reason: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(reason) = reason {
            self.format_error_message(StatusCode::BAD_GATEWAY, &format!("Bad Gateway: {}", reason))
        } else {
            self.format_error_message(StatusCode::BAD_GATEWAY, "Invalid response from downstream server")
        };

        IcapResponse {
            status: StatusCode::BAD_GATEWAY,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a 503 Service Unavailable response
    pub fn service_unavailable(&self, retry_after: Option<u64>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());
        
        if let Some(seconds) = retry_after {
            headers.insert("retry-after", seconds.to_string().parse().unwrap());
        }

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        IcapResponse {
            status: StatusCode::SERVICE_UNAVAILABLE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(self.format_error_message(StatusCode::SERVICE_UNAVAILABLE, "ICAP service is temporarily unavailable")),
            encapsulated: None,
        }
    }

    /// Generate a 505 ICAP Version Not Supported response
    pub fn version_not_supported(&self, version: Option<&str>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = if let Some(v) = version {
            self.format_error_message(StatusCode::HTTP_VERSION_NOT_SUPPORTED, &format!("Version '{}' is not supported", v))
        } else {
            self.format_error_message(StatusCode::HTTP_VERSION_NOT_SUPPORTED, "ICAP version is not supported")
        };

        IcapResponse {
            status: StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Generate a custom response with specific status code
    pub fn custom_response(&self, status: StatusCode, headers: HeaderMap, body: Bytes, encapsulated: Option<EncapsulatedData>) -> IcapResponse {
        IcapResponse {
            status,
            version: Version::HTTP_11,
            headers,
            body,
            encapsulated,
        }
    }

    /// Generate an OPTIONS response with service capabilities
    pub fn options_response(&self, methods: &[IcapMethod], capabilities: HashMap<String, String>) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // Add methods
        let methods_str = methods.iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        headers.insert("methods", methods_str.parse().unwrap());
        
        // Add service description
        headers.insert("service", "G3 ICAP Server - Content Filtering & Antivirus".parse().unwrap());
        
        // Add capabilities
        for (key, value) in capabilities.into_iter() {
            let key_header: http::HeaderName = key.parse().unwrap();
            let value_header: http::HeaderValue = value.parse().unwrap();
            headers.insert(key_header, value_header);
        }
        
        // Add null-body header for OPTIONS response
        self.add_null_body_header(&mut headers);

        IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Build standard ICAP response headers
    fn build_standard_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        // Server header
        headers.insert("server", self.server_name.as_str().parse().unwrap());
        
        // ISTag header for cache validation
        headers.insert("istag", format!("\"{}\"", self.server_version).parse().unwrap());
        
        // Service ID if available
        if let Some(service_id) = &self.service_id {
            headers.insert("service-id", service_id.as_str().parse().unwrap());
        }
        
        headers
    }

    /// Add null-body encapsulated header for responses without bodies
    fn add_null_body_header(&self, headers: &mut HeaderMap) {
        headers.insert("encapsulated", "null-body=0".parse().unwrap());
    }

    /// Serialize encapsulated header for response
    fn serialize_encapsulated_header(&self, encapsulated: &EncapsulatedData) -> String {
        let mut parts = Vec::new();
        
        if let Some(req_hdr) = &encapsulated.req_hdr {
            parts.push(format!("req-hdr={}", req_hdr.len()));
        }
        if let Some(res_hdr) = &encapsulated.res_hdr {
            parts.push(format!("res-hdr={}", res_hdr.len()));
        }
        if let Some(req_body) = &encapsulated.req_body {
            parts.push(format!("req-body={}", req_body.len()));
        }
        if let Some(res_body) = &encapsulated.res_body {
            parts.push(format!("res-body={}", res_body.len()));
        }
        if encapsulated.null_body {
            parts.push("null-body=0".to_string());
        }

        parts.join(", ")
    }


    /// Format error message with proper ICAP error formatting
    /// Following g3proxy's error formatting patterns
    fn format_error_message(&self, status: StatusCode, message: &str) -> String {
        // For ICAP, we use a simpler format than HTTP
        // g3proxy uses: "blocked by icap server: {} - {}"
        format!("{} {}", status.as_u16(), message)
    }

    /// Format HTML error message following g3proxy's HTML error response pattern
    fn format_html_error_message(&self, status: StatusCode, message: &str) -> String {
        let code = status.as_str();
        let reason = Self::get_reason_phrase(status);
        
        format!(
            "<html>\n\
             <head><title>{code} {reason}</title></head>\n\
             <body>\n\
             <div style=\"text-align: center;\"><h1>{code} {reason}</h1>\n\
             <p>{message}</p></div>\n\
             </body>\n\
             </html>\n"
        )
    }

    /// Generate a response from a status code with optional message
    pub fn from_status_code(&self, status: StatusCode, message: Option<&str>) -> IcapResponse {
        match status {
            StatusCode::CONTINUE => self.continue_response(),
            StatusCode::OK => {
                // For 200 OK, we need encapsulated data and body
                self.ok_modified(None, Bytes::new())
            }
            StatusCode::NO_CONTENT => self.no_modifications(None),
            StatusCode::NOT_MODIFIED => self.not_modified(),
            StatusCode::BAD_REQUEST => self.bad_request(message),
            StatusCode::FORBIDDEN => self.forbidden(message),
            StatusCode::NOT_FOUND => self.not_found(message),
            StatusCode::METHOD_NOT_ALLOWED => {
                // For 405, we need to specify allowed methods
                let allowed = vec![IcapMethod::Options, IcapMethod::Reqmod, IcapMethod::Respmod];
                self.method_not_allowed(&IcapMethod::Options, &allowed)
            }
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => self.proxy_auth_required(message),
            StatusCode::CONFLICT => self.conflict(message),
            StatusCode::PAYLOAD_TOO_LARGE => self.request_too_large(None),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => self.unsupported_media_type(message),
            StatusCode::INTERNAL_SERVER_ERROR => self.internal_server_error(message),
            StatusCode::NOT_IMPLEMENTED => self.not_implemented(None),
            StatusCode::BAD_GATEWAY => self.bad_gateway(message),
            StatusCode::SERVICE_UNAVAILABLE => self.service_unavailable(None),
            StatusCode::HTTP_VERSION_NOT_SUPPORTED => self.version_not_supported(message),
            _ => {
                // For unknown status codes, create a custom response
                let mut headers = self.build_standard_headers();
                
                // RFC 3507: Add required Encapsulated header for error responses
                self.add_null_body_header(&mut headers);
                
                // Add connection close for error responses
                headers.insert("connection", "close".parse().unwrap());
                
                // For ICAP error responses, we don't include content-type at ICAP level
                // The error message goes in the body without HTTP encapsulation
                let body = self.format_error_message(status, message.unwrap_or("Unknown error"));
                
                IcapResponse {
                    status,
                    version: Version::HTTP_11,
                    headers,
                    body: Bytes::from(body),
                    encapsulated: None,
                }
            }
        }
    }

    /// Generate a chunked response from a status code with optional message
    pub fn from_status_code_chunked(&self, status: StatusCode, message: Option<&str>) -> IcapResponse {
        match status {
            StatusCode::CONTINUE => self.continue_response(),
            StatusCode::OK => {
                // For 200 OK with chunked encoding
                self.ok_modified_chunked(None, Bytes::new())
            }
            StatusCode::NO_CONTENT => self.no_modifications(None),
            StatusCode::NOT_MODIFIED => self.not_modified(),
            StatusCode::BAD_REQUEST => self.bad_request_chunked(message),
            StatusCode::FORBIDDEN => self.forbidden_chunked(message),
            StatusCode::NOT_FOUND => self.not_found(message),
            StatusCode::METHOD_NOT_ALLOWED => {
                // For 405, we need to specify allowed methods
                let allowed = vec![IcapMethod::Options, IcapMethod::Reqmod, IcapMethod::Respmod];
                self.method_not_allowed(&IcapMethod::Options, &allowed)
            }
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => self.proxy_auth_required(message),
            StatusCode::CONFLICT => self.conflict(message),
            StatusCode::PAYLOAD_TOO_LARGE => self.request_too_large(None),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => self.unsupported_media_type(message),
            StatusCode::INTERNAL_SERVER_ERROR => self.internal_server_error_chunked(message),
            StatusCode::NOT_IMPLEMENTED => self.not_implemented(None),
            StatusCode::BAD_GATEWAY => self.bad_gateway(message),
            StatusCode::SERVICE_UNAVAILABLE => self.service_unavailable(None),
            StatusCode::HTTP_VERSION_NOT_SUPPORTED => self.version_not_supported(message),
            _ => {
                // For unknown status codes, create a custom chunked response
                let mut headers = self.build_standard_headers();
                
                // RFC 3507: Add required Encapsulated header for error responses
                self.add_null_body_header(&mut headers);
                
                // Add connection close for error responses
                headers.insert("connection", "close".parse().unwrap());
                headers.insert("transfer-encoding", "chunked".parse().unwrap());
                
                // For ICAP error responses, we don't include content-type at ICAP level
                // The error message goes in the body without HTTP encapsulation
                let body = self.format_error_message(status, message.unwrap_or("Unknown error"));
                
                IcapResponse {
                    status,
                    version: Version::HTTP_11,
                    headers,
                    body: Bytes::from(body),
                    encapsulated: None,
                }
            }
        }
    }

    /// Check if a status code represents an error
    pub fn is_error_status(status: StatusCode) -> bool {
        status.as_u16() >= 400
    }

    /// Get the reason phrase for a status code
    pub fn get_reason_phrase(status: StatusCode) -> &'static str {
        match status {
            StatusCode::CONTINUE => "Continue",
            StatusCode::OK => "OK",
            StatusCode::NO_CONTENT => "No Content",
            StatusCode::NOT_MODIFIED => "Not Modified",
            StatusCode::BAD_REQUEST => "Bad Request",
            StatusCode::FORBIDDEN => "Forbidden",
            StatusCode::NOT_FOUND => "Not Found",
            StatusCode::METHOD_NOT_ALLOWED => "Method Not Allowed",
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => "Proxy Authentication Required",
            StatusCode::CONFLICT => "Conflict",
            StatusCode::PAYLOAD_TOO_LARGE => "Request Entity Too Large",
            StatusCode::UNSUPPORTED_MEDIA_TYPE => "Unsupported Media Type",
            StatusCode::INTERNAL_SERVER_ERROR => "Internal Server Error",
            StatusCode::NOT_IMPLEMENTED => "Not Implemented",
            StatusCode::BAD_GATEWAY => "Bad Gateway",
            StatusCode::SERVICE_UNAVAILABLE => "Service Unavailable",
            StatusCode::HTTP_VERSION_NOT_SUPPORTED => "HTTP Version Not Supported",
            _ => "Unknown",
        }
    }

    /// Check if a response should use chunked transfer encoding
    /// This is useful for antivirus/malware ICAP servers that don't know the full response size upfront
    pub fn should_use_chunked_encoding(&self, body_size: Option<usize>) -> bool {
        // Use chunked encoding if:
        // 1. Body size is unknown (None)
        // 2. Body size is large (> 1MB)
        // 3. For streaming responses
        match body_size {
            None => true,
            Some(size) => size > 1024 * 1024, // > 1MB threshold
        }
    }

    /// Create a chunked response with proper encapsulated headers
    pub fn create_chunked_response(
        &self,
        status: StatusCode,
        encapsulated: Option<EncapsulatedData>,
        body: Bytes,
        content_type: &str,
    ) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        if let Some(enc) = &encapsulated {
            let encapsulated_header = self.serialize_encapsulated_header(enc);
            headers.insert("encapsulated", encapsulated_header.parse().unwrap());
        }

        // Add content type and chunked transfer encoding
        headers.insert("content-type", content_type.parse().unwrap());
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        IcapResponse {
            status,
            version: Version::HTTP_11,
            headers,
            body,
            encapsulated,
        }
    }

    /// Create a response from an error type, following g3proxy's error mapping pattern
    pub fn from_error_type(&self, error_type: &str, message: Option<&str>) -> IcapResponse {
        match error_type {
            "InternalServerError" | "InternalAdapterError" | "InternalResolverError" | "UnclassifiedError" => {
                self.internal_server_error(message)
            }
            "InternalTlsClientError" => {
                self.internal_server_error(Some("TLS client error"))
            }
            "EscaperNotUsable" => {
                self.service_unavailable(None)
            }
            "ForbiddenByRule" => {
                self.forbidden(message)
            }
            "InvalidClientProtocol" | "ClientAppError" => {
                self.bad_request(message)
            }
            "UnimplementedProtocol" => {
                self.not_implemented(None)
            }
            "UpstreamNotResolved" => {
                self.bad_gateway(Some("Upstream not resolved"))
            }
            "UpstreamConnectFailed" => {
                self.bad_gateway(Some("Upstream connection failed"))
            }
            "UpstreamReadFailed" => {
                self.bad_gateway(Some("Upstream read failed"))
            }
            "UpstreamWriteFailed" => {
                self.bad_gateway(Some("Upstream write failed"))
            }
            "UpstreamTimeout" => {
                self.service_unavailable(Some(30)) // 30 second retry
            }
            "ClientTcpWriteFailed" => {
                self.internal_server_error(Some("Client write failed"))
            }
            "ClientTcpReadFailed" => {
                self.bad_request(Some("Client read failed"))
            }
            _ => {
                self.internal_server_error(message)
            }
        }
    }

    /// Create an HTML error response following g3proxy's HTML error pattern
    pub fn html_error_response(&self, status: StatusCode, message: &str) -> IcapResponse {
        let mut headers = self.build_standard_headers();
        
        // RFC 3507: Add required Encapsulated header for error responses
        self.add_null_body_header(&mut headers);
        
        // Add connection close for error responses
        headers.insert("connection", "close".parse().unwrap());

        // For ICAP error responses, we don't include content-type at ICAP level
        // The error message goes in the body without HTTP encapsulation
        let body = self.format_html_error_message(status, message);

        IcapResponse {
            status,
            version: Version::HTTP_11,
            headers,
            body: Bytes::from(body),
            encapsulated: None,
        }
    }

    /// Serialize response to bytes following g3proxy's serialization pattern
    pub fn serialize_response(&self, response: &IcapResponse) -> Vec<u8> {
        const RESPONSE_BUFFER_SIZE: usize = 1024;
        let mut buf = Vec::<u8>::with_capacity(RESPONSE_BUFFER_SIZE);
        
        // Write status line
        let reason = Self::get_reason_phrase(response.status);
        buf.extend_from_slice(format!("ICAP/1.0 {} {}\r\n", response.status.as_str(), reason).as_bytes());
        
        // Write headers
        for (name, value) in &response.headers {
            buf.extend_from_slice(name.as_str().as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(value.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }
        
        // End headers
        buf.extend_from_slice(b"\r\n");
        
        // Write body if present
        if !response.body.is_empty() {
            buf.extend_from_slice(&response.body);
        }
        
        buf
    }

    /// Check if response should close connection (following g3proxy pattern)
    pub fn should_close_connection(&self, response: &IcapResponse) -> bool {
        // Check for connection: close header
        if let Some(connection) = response.headers.get("connection") {
            if connection.to_str().unwrap_or("").to_lowercase() == "close" {
                return true;
            }
        }
        
        // Check for error status codes that typically close connections
        matches!(response.status, 
            StatusCode::BAD_REQUEST | 
            StatusCode::UNAUTHORIZED | 
            StatusCode::FORBIDDEN | 
            StatusCode::NOT_FOUND | 
            StatusCode::METHOD_NOT_ALLOWED | 
            StatusCode::REQUEST_TIMEOUT | 
            StatusCode::CONFLICT | 
            StatusCode::PAYLOAD_TOO_LARGE | 
            StatusCode::UNSUPPORTED_MEDIA_TYPE | 
            StatusCode::INTERNAL_SERVER_ERROR | 
            StatusCode::NOT_IMPLEMENTED | 
            StatusCode::BAD_GATEWAY | 
            StatusCode::SERVICE_UNAVAILABLE | 
            StatusCode::HTTP_VERSION_NOT_SUPPORTED
        )
    }

    /// Add custom header following g3proxy's header addition pattern
    pub fn add_custom_header(&self, headers: &mut HeaderMap, name: &str, value: &str) {
        if let (Ok(header_name), Ok(header_value)) = (name.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
            headers.insert(header_name, header_value);
        }
    }
}

impl Default for IcapResponseGenerator {
    fn default() -> Self {
        Self::new("G3ICAP/1.0.0".to_string(), "g3icap-1.0.0".to_string())
    }
}

impl fmt::Display for IcapResponseGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IcapResponseGenerator({}, {})", self.server_name, self.server_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::common::IcapMethod;

    #[test]
    fn test_continue_response() {
        let generator = IcapResponseGenerator::default();
        let response = generator.continue_response();
        
        assert_eq!(response.status, StatusCode::CONTINUE);
        assert!(response.headers.contains_key("server"));
        assert!(response.headers.contains_key("istag"));
        assert!(response.body.is_empty());
    }

    #[test]
    fn test_ok_modified_response() {
        let generator = IcapResponseGenerator::default();
        let body = Bytes::from("modified content");
        let response = generator.ok_modified(None, body.clone());
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body, body);
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("server"));
    }

    #[test]
    fn test_no_modifications_response() {
        let generator = IcapResponseGenerator::default();
        let response = generator.no_modifications(None);
        
        assert_eq!(response.status, StatusCode::NO_CONTENT);
        assert!(response.body.is_empty());
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("server"));
    }

    #[test]
    fn test_bad_request_response() {
        let generator = IcapResponseGenerator::default();
        let response = generator.bad_request(Some("Invalid header"));
        
        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("Invalid header"));
        assert!(response.headers.contains_key("server"));
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("content-type"));
        assert!(response.headers.contains_key("connection"));
    }

    #[test]
    fn test_method_not_allowed_response() {
        let generator = IcapResponseGenerator::default();
        let allowed = vec![IcapMethod::Options, IcapMethod::Reqmod];
        let response = generator.method_not_allowed(&IcapMethod::Respmod, &allowed);
        
        assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED);
        assert!(response.headers.contains_key("allow"));
        assert!(response.headers.contains_key("server"));
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("content-type"));
        assert!(response.headers.contains_key("connection"));
    }

    #[test]
    fn test_options_response() {
        let generator = IcapResponseGenerator::default();
        let methods = vec![IcapMethod::Options, IcapMethod::Reqmod, IcapMethod::Respmod];
        let mut capabilities = HashMap::new();
        capabilities.insert("preview".to_string(), "1024".to_string());
        capabilities.insert("max-connections".to_string(), "1000".to_string());
        
        let response = generator.options_response(&methods, capabilities);
        
        assert_eq!(response.status, StatusCode::NO_CONTENT);
        assert!(response.headers.contains_key("methods"));
        assert!(response.headers.contains_key("preview"));
        assert!(response.headers.contains_key("max-connections"));
        assert!(response.headers.contains_key("server"));
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("service"));
    }

    #[test]
    fn test_from_status_code() {
        let generator = IcapResponseGenerator::default();
        let response = generator.from_status_code(StatusCode::BAD_REQUEST, Some("Test error"));
        
        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("Test error"));
    }

    #[test]
    fn test_is_error_status() {
        assert!(IcapResponseGenerator::is_error_status(StatusCode::BAD_REQUEST));
        assert!(IcapResponseGenerator::is_error_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(!IcapResponseGenerator::is_error_status(StatusCode::OK));
        assert!(!IcapResponseGenerator::is_error_status(StatusCode::NO_CONTENT));
    }

    #[test]
    fn test_get_reason_phrase() {
        assert_eq!(IcapResponseGenerator::get_reason_phrase(StatusCode::OK), "OK");
        assert_eq!(IcapResponseGenerator::get_reason_phrase(StatusCode::BAD_REQUEST), "Bad Request");
        assert_eq!(IcapResponseGenerator::get_reason_phrase(StatusCode::NOT_FOUND), "Not Found");
    }

    #[test]
    fn test_service_id() {
        let generator = IcapResponseGenerator::with_service_id(
            "TestServer".to_string(),
            "1.0.0".to_string(),
            Some("test-service".to_string())
        );
        let response = generator.continue_response();
        
        assert!(response.headers.contains_key("service-id"));
        let service_id = response.headers.get("service-id").unwrap();
        assert_eq!(service_id, "test-service");
    }

    #[test]
    fn test_ok_modified_chunked() {
        let generator = IcapResponseGenerator::default();
        let body = Bytes::from("chunked content");
        let response = generator.ok_modified_chunked(None, body.clone());
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body, body);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("server"));
    }

    #[test]
    fn test_bad_request_chunked() {
        let generator = IcapResponseGenerator::default();
        let response = generator.bad_request_chunked(Some("Invalid header"));
        
        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        assert!(response.headers.contains_key("content-type"));
        assert_eq!(response.headers.get("content-type").unwrap(), "text/plain");
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("Invalid header"));
    }

    #[test]
    fn test_forbidden_chunked() {
        let generator = IcapResponseGenerator::default();
        let response = generator.forbidden_chunked(Some("Malware detected"));
        
        assert_eq!(response.status, StatusCode::FORBIDDEN);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        assert!(response.headers.contains_key("content-type"));
        assert_eq!(response.headers.get("content-type").unwrap(), "text/html");
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("Malware detected"));
        assert!(body_str.contains("<html>"));
    }

    #[test]
    fn test_internal_server_error_chunked() {
        let generator = IcapResponseGenerator::default();
        let response = generator.internal_server_error_chunked(Some("Scan engine failure"));
        
        assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("Scan engine failure"));
    }

    #[test]
    fn test_from_status_code_chunked() {
        let generator = IcapResponseGenerator::default();
        let response = generator.from_status_code_chunked(StatusCode::OK, None);
        
        assert_eq!(response.status, StatusCode::OK);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
    }

    #[test]
    fn test_should_use_chunked_encoding() {
        let generator = IcapResponseGenerator::default();
        
        // Unknown size should use chunked
        assert!(generator.should_use_chunked_encoding(None));
        
        // Large size should use chunked
        assert!(generator.should_use_chunked_encoding(Some(2 * 1024 * 1024))); // 2MB
        
        // Small size should not use chunked
        assert!(!generator.should_use_chunked_encoding(Some(1024))); // 1KB
        
        // Exactly at threshold should not use chunked
        assert!(!generator.should_use_chunked_encoding(Some(1024 * 1024))); // 1MB
    }

    #[test]
    fn test_create_chunked_response() {
        let generator = IcapResponseGenerator::default();
        let body = Bytes::from("chunked response body");
        let response = generator.create_chunked_response(
            StatusCode::OK,
            None,
            body.clone(),
            "text/html"
        );
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body, body);
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        assert!(response.headers.contains_key("content-type"));
        assert_eq!(response.headers.get("content-type").unwrap(), "text/html");
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("server"));
    }

    #[test]
    fn test_chunked_response_with_encapsulated() {
        let generator = IcapResponseGenerator::default();
        let body = Bytes::from("chunked content");
        
        // Create a proper HeaderMap for the response headers
        let mut res_headers = HeaderMap::new();
        res_headers.insert("content-type", "text/html".parse().unwrap());
        
        let encapsulated = EncapsulatedData {
            req_hdr: None,
            res_hdr: Some(res_headers),
            req_body: None,
            res_body: Some(body.clone()),
            null_body: false,
        };
        
        let response = generator.create_chunked_response(
            StatusCode::OK,
            Some(encapsulated),
            body.clone(),
            "text/html"
        );
        
        assert_eq!(response.status, StatusCode::OK);
        assert!(response.headers.contains_key("encapsulated"));
        assert!(response.headers.contains_key("transfer-encoding"));
        assert_eq!(response.headers.get("transfer-encoding").unwrap(), "chunked");
        
        let encapsulated_header = response.headers.get("encapsulated").unwrap();
        let encapsulated_str = encapsulated_header.to_str().unwrap();
        assert!(encapsulated_str.contains("res-hdr="));
        assert!(encapsulated_str.contains("res-body="));
    }

    #[test]
    fn test_from_error_type() {
        let generator = IcapResponseGenerator::default();
        
        // Test internal server error
        let response = generator.from_error_type("InternalServerError", Some("Test error"));
        assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
        
        // Test forbidden error
        let response = generator.from_error_type("ForbiddenByRule", Some("Access denied"));
        assert_eq!(response.status, StatusCode::FORBIDDEN);
        
        // Test bad request error
        let response = generator.from_error_type("ClientAppError", Some("Invalid request"));
        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        
        // Test service unavailable
        let response = generator.from_error_type("EscaperNotUsable", None);
        assert_eq!(response.status, StatusCode::SERVICE_UNAVAILABLE);
        
        // Test unknown error type
        let response = generator.from_error_type("UnknownError", Some("Unknown"));
        assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_html_error_response() {
        let generator = IcapResponseGenerator::default();
        let response = generator.html_error_response(StatusCode::FORBIDDEN, "Access denied");
        
        assert_eq!(response.status, StatusCode::FORBIDDEN);
        assert!(response.headers.contains_key("content-type"));
        assert_eq!(response.headers.get("content-type").unwrap(), "text/html");
        
        let body_str = String::from_utf8_lossy(&response.body);
        assert!(body_str.contains("<html>"));
        assert!(body_str.contains("403 Forbidden"));
        assert!(body_str.contains("Access denied"));
    }

    #[test]
    fn test_serialize_response() {
        let generator = IcapResponseGenerator::default();
        let response = generator.bad_request(Some("Test error"));
        let serialized = generator.serialize_response(&response);
        
        let serialized_str = String::from_utf8_lossy(&serialized);
        assert!(serialized_str.starts_with("ICAP/1.0 400 Bad Request\r\n"));
        assert!(serialized_str.contains("server: G3ICAP/1.0.0\r\n"));
        assert!(serialized_str.contains("istag: \"g3icap-1.0.0\"\r\n"));
        assert!(serialized_str.contains("content-type: text/plain\r\n"));
        assert!(serialized_str.contains("connection: close\r\n"));
        assert!(serialized_str.ends_with("\r\n\r\n400 Bad Request: Test error"));
    }

    #[test]
    fn test_should_close_connection() {
        let generator = IcapResponseGenerator::default();
        
        // Test error responses that should close
        let response = generator.bad_request(None);
        assert!(generator.should_close_connection(&response));
        
        let response = generator.internal_server_error(None);
        assert!(generator.should_close_connection(&response));
        
        // Test success responses that should not close
        let response = generator.continue_response();
        assert!(!generator.should_close_connection(&response));
        
        let response = generator.no_modifications(None);
        assert!(!generator.should_close_connection(&response));
    }

    #[test]
    fn test_add_custom_header() {
        let generator = IcapResponseGenerator::default();
        let mut headers = HeaderMap::new();
        
        generator.add_custom_header(&mut headers, "x-custom-header", "custom-value");
        assert!(headers.contains_key("x-custom-header"));
        assert_eq!(headers.get("x-custom-header").unwrap(), "custom-value");
        
        // Test invalid header (should not panic)
        generator.add_custom_header(&mut headers, "invalid\nheader", "value");
        assert!(!headers.contains_key("invalid\nheader"));
    }

    #[test]
    fn test_format_html_error_message() {
        let generator = IcapResponseGenerator::default();
        let html = generator.format_html_error_message(StatusCode::FORBIDDEN, "Access denied");
        
        assert!(html.contains("<html>"));
        assert!(html.contains("<head><title>403 Forbidden</title></head>"));
        assert!(html.contains("<h1>403 Forbidden</h1>"));
        assert!(html.contains("<p>Access denied</p>"));
        assert!(html.contains("</html>"));
    }
}
