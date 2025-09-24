//! Common ICAP protocol types and utilities

use crate::error::IcapError;
use crate::protocol::chunked::ChunkedParser;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Uri, Version};
use std::collections::HashMap;

/// ICAP method types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IcapMethod {
    /// REQMOD method - request modification
    Reqmod,
    /// RESPMOD method - response modification
    Respmod,
    /// OPTIONS method - service discovery
    Options,
}

impl From<&str> for IcapMethod {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "REQMOD" => IcapMethod::Reqmod,
            "RESPMOD" => IcapMethod::Respmod,
            "OPTIONS" => IcapMethod::Options,
            _ => IcapMethod::Options, // Default fallback
        }
    }
}

impl ToString for IcapMethod {
    fn to_string(&self) -> String {
        match self {
            IcapMethod::Reqmod => "REQMOD".to_string(),
            IcapMethod::Respmod => "RESPMOD".to_string(),
            IcapMethod::Options => "OPTIONS".to_string(),
        }
    }
}

/// ICAP request structure
#[derive(Debug, Clone)]
pub struct IcapRequest {
    /// ICAP method
    pub method: IcapMethod,
    /// Request URI
    pub uri: Uri,
    /// ICAP version
    pub version: Version,
    /// Request headers
    pub headers: HeaderMap,
    /// Request body
    pub body: Bytes,
    /// Encapsulated headers (for REQMOD/RESPMOD)
    pub encapsulated: Option<EncapsulatedData>,
}

/// ICAP response structure
#[derive(Debug, Clone)]
pub struct IcapResponse {
    /// Response status code
    pub status: StatusCode,
    /// ICAP version
    pub version: Version,
    /// Response headers
    pub headers: HeaderMap,
    /// Response body
    pub body: Bytes,
    /// Encapsulated headers (for REQMOD/RESPMOD)
    pub encapsulated: Option<EncapsulatedData>,
}

/// Encapsulated data for REQMOD/RESPMOD
#[derive(Debug, Clone)]
pub struct EncapsulatedData {
    /// HTTP request headers
    pub req_hdr: Option<HeaderMap>,
    /// HTTP request body
    pub req_body: Option<Bytes>,
    /// HTTP response headers
    pub res_hdr: Option<HeaderMap>,
    /// HTTP response body
    pub res_body: Option<Bytes>,
    /// Null body indicator
    pub null_body: bool,
}

/// ICAP service information
#[derive(Debug, Clone)]
pub struct IcapService {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Supported methods
    pub methods: Vec<IcapMethod>,
    /// Service options
    pub options: HashMap<String, String>,
}

/// ICAP message parser
pub struct IcapParser;

impl IcapParser {
    /// Parse ICAP request from bytes using nom parser
    pub fn parse_request(data: &[u8]) -> Result<IcapRequest, IcapError> {
        let data_str = std::str::from_utf8(data)
            .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8: {}", e), "PARSER"))?;
        
        crate::protocol::parser::parse_icap_request(data_str)
    }

    /// Parse ICAP response from bytes using nom parser
    pub fn parse_response(data: &[u8]) -> Result<IcapResponse, IcapError> {
        let data_str = std::str::from_utf8(data)
            .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8: {}", e), "PARSER"))?;
        
        crate::protocol::parser::parse_icap_response(data_str)
    }
}

/// Parse HTTP/ICAP version from string
fn parse_http_version(version_str: &str) -> Result<Version, IcapError> {
    match version_str {
        "HTTP/1.0" => Ok(Version::HTTP_10),
        "HTTP/1.1" => Ok(Version::HTTP_11),
        "HTTP/2.0" => Ok(Version::HTTP_2),
        "HTTP/3.0" => Ok(Version::HTTP_3),
        "ICAP/1.0" => Ok(Version::HTTP_11), // ICAP/1.0 maps to HTTP/1.1 for compatibility
        _ => Err(IcapError::protocol_simple(format!("Unsupported version: {}", version_str))),
    }
}

/// Parse encapsulated data from ICAP message
fn parse_encapsulated_data(header: &http::HeaderValue, body: &[u8]) -> Result<EncapsulatedData, IcapError> {
    let header_str = header.to_str()
        .map_err(|e| IcapError::protocol_simple(format!("Invalid encapsulated header: {}", e)))?;
    
    // Parse encapsulated header format: "req-hdr=0, res-hdr=100, req-body=200, res-body=300"
    let mut req_hdr_offset = None;
    let mut res_hdr_offset = None;
    let mut req_body_offset = None;
    let mut res_body_offset = None;
    let mut null_body = false;
    
    for part in header_str.split(',') {
        let part = part.trim();
        if let Some((key, value)) = part.split_once('=') {
            let offset = value.parse::<usize>()
                .map_err(|e| IcapError::protocol_simple(format!("Invalid offset in encapsulated header: {}", e)))?;
            
            match key.trim() {
                "req-hdr" => req_hdr_offset = Some(offset),
                "res-hdr" => res_hdr_offset = Some(offset),
                "req-body" => req_body_offset = Some(offset),
                "res-body" => res_body_offset = Some(offset),
                "null-body" => null_body = true,
                _ => {}
            }
        }
    }
    
    // Parse HTTP headers and bodies based on offsets
    let mut req_hdr = None;
    let mut res_hdr = None;
    let mut req_body = None;
    let mut res_body = None;
    
    if let Some(offset) = req_hdr_offset {
        // Find the end of request headers by looking for the next offset or end of data
        let end_offset = res_hdr_offset
            .or(req_body_offset)
            .or(res_body_offset)
            .unwrap_or(body.len());
        
        // Ensure we don't exceed the actual body length
        let safe_end_offset = std::cmp::min(end_offset, body.len());
        
        if offset < safe_end_offset && offset < body.len() {
            req_hdr = Some(parse_http_headers(&body[offset..safe_end_offset])?);
        }
    }
    
    if let Some(offset) = res_hdr_offset {
        let end_offset = req_body_offset
            .or(res_body_offset)
            .unwrap_or(body.len());
        
        // Ensure we don't exceed the actual body length
        let safe_end_offset = std::cmp::min(end_offset, body.len());
        
        if offset < safe_end_offset && offset < body.len() {
            res_hdr = Some(parse_http_headers(&body[offset..safe_end_offset])?);
        }
    }
    
    if let Some(offset) = req_body_offset {
        let end_offset = res_body_offset.unwrap_or(body.len());
        
        // Ensure we don't exceed the actual body length
        let safe_end_offset = std::cmp::min(end_offset, body.len());
        
        if offset < safe_end_offset && offset < body.len() {
            // For ICAP, bodies should be chunked, but handle non-chunked data gracefully
            let body_data = &body[offset..safe_end_offset];
            req_body = Some(parse_body_data(body_data)?);
        } else if offset < body.len() {
            // For ICAP, bodies should be chunked, but handle non-chunked data gracefully
            let body_data = &body[offset..];
            req_body = Some(parse_body_data(body_data)?);
        }
    }
    
    if let Some(offset) = res_body_offset {
        if offset < body.len() {
            // For ICAP, bodies should be chunked, but handle non-chunked data gracefully
            let body_data = &body[offset..];
            res_body = Some(parse_body_data(body_data)?);
        }
    }
    
    Ok(EncapsulatedData {
        req_hdr,
        req_body,
        res_hdr,
        res_body,
        null_body,
    })
}

/// Parse HTTP headers from bytes
fn parse_http_headers(data: &[u8]) -> Result<HeaderMap, IcapError> {
    let mut headers = HeaderMap::new();
    
    if data.is_empty() {
        return Ok(headers);
    }
    
    let data_str = std::str::from_utf8(data)
        .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8 in headers: {}", e), "PARSER"))?;
    
    let lines: Vec<&str> = data_str.split('\n').collect();
    let mut is_first_line = true;
    
    for line in lines {
        let line = line.trim();
        
        // Skip empty lines
        if line.is_empty() {
            break;
        }
        
        // Skip the HTTP request line (first line) - it starts with HTTP method
        if is_first_line {
            if line.starts_with("GET ") || line.starts_with("POST ") || line.starts_with("HEAD ") || 
               line.starts_with("PUT ") || line.starts_with("DELETE ") || line.starts_with("OPTIONS ") ||
               line.starts_with("PATCH ") || line.starts_with("CONNECT ") || line.starts_with("TRACE ") {
                is_first_line = false;
                continue;
            }
        }
        
        // Parse header line
        if let Some(colon_pos) = line.find(':') {
            let header_name = line[..colon_pos].trim();
            let header_value = line[colon_pos + 1..].trim();
            
            if !header_name.is_empty() && !header_value.is_empty() {
                if let (Ok(name), Ok(value)) = (header_name.parse::<http::HeaderName>(), header_value.parse::<http::HeaderValue>()) {
                    headers.insert(name, value);
                }
            }
        }
    }
    
    Ok(headers)
}

/// ICAP message serializer
pub struct IcapSerializer;

impl IcapSerializer {
    /// Serialize ICAP request to bytes
    pub fn serialize_request(request: &IcapRequest) -> Result<Bytes, IcapError> {
        let mut output = Vec::new();
        
        // Serialize request line
        let version_str = format_http_version(request.version);
        output.extend_from_slice(format!("{} {} {}\r\n", 
            request.method.to_string(), 
            request.uri, 
            version_str
        ).as_bytes());
        
        // Serialize headers
        for (name, value) in &request.headers {
            output.extend_from_slice(format!("{}: {}\r\n", name, value.to_str().unwrap_or("")).as_bytes());
        }
        
        // Serialize encapsulated header if present
        if let Some(encapsulated) = &request.encapsulated {
            let encapsulated_header = serialize_encapsulated_header(encapsulated)?;
            output.extend_from_slice(format!("Encapsulated: {}\r\n", encapsulated_header).as_bytes());
        }
        
        // Empty line to separate headers from body
        output.extend_from_slice(b"\r\n");
        
        // Serialize body
        if !request.body.is_empty() {
            output.extend_from_slice(&request.body);
        }
        
        Ok(Bytes::from(output))
    }

    /// Serialize ICAP response to bytes
    pub fn serialize_response(response: &IcapResponse) -> Result<Bytes, IcapError> {
        let mut output = Vec::new();
        
        // Serialize status line - ICAP responses must use ICAP/1.0 protocol version
        let reason = match response.status.as_u16() {
            204 => "No Modifications", // ICAP 204 is "No Modifications", not "No Content"
            _ => response.status.canonical_reason().unwrap_or("Unknown"),
        };
        let status_line = format!("ICAP/1.0 {} {}\r\n", 
            response.status.as_u16(), 
            reason
        );
        println!("DEBUG: Serializing ICAP response: {}", status_line.trim());
        output.extend_from_slice(status_line.as_bytes());
        
        // Serialize headers
        for (name, value) in &response.headers {
            let header_line = format!("{}: {}\r\n", name, value.to_str().unwrap_or(""));
            println!("DEBUG: Response header: {}", header_line.trim());
            output.extend_from_slice(header_line.as_bytes());
        }
        
        // Serialize encapsulated header if present and not already in headers
        if let Some(encapsulated) = &response.encapsulated {
            if !response.headers.contains_key("encapsulated") {
                let encapsulated_header = serialize_encapsulated_header(encapsulated)?;
                let encapsulated_line = format!("Encapsulated: {}\r\n", encapsulated_header);
                println!("DEBUG: Response encapsulated: {}", encapsulated_line.trim());
                output.extend_from_slice(encapsulated_line.as_bytes());
            }
        }
        
        // Empty line to separate headers from body
        output.extend_from_slice(b"\r\n");
        println!("DEBUG: Response headers complete, body length: {}", response.body.len());
        
        // Serialize body - RFC 3507: 204 No Modifications responses must not have a body
        if response.status.as_u16() == 204 {
            println!("DEBUG: 204 No Modifications response - skipping body as per RFC 3507");
        } else if !response.body.is_empty() {
            println!("DEBUG: Adding response body: {} bytes", response.body.len());
            output.extend_from_slice(&response.body);
        }
        
        let result = Bytes::from(output);
        println!("DEBUG: Complete ICAP response serialized: {} bytes", result.len());
        println!("DEBUG: Response content: {}", String::from_utf8_lossy(&result));
        
        Ok(result)
    }
}

/// Format HTTP version to string
fn format_http_version(version: Version) -> &'static str {
    match version {
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2.0",
        Version::HTTP_3 => "HTTP/3.0",
        _ => "HTTP/1.1",
    }
}

/// Serialize encapsulated header with proper byte offset calculation
fn serialize_encapsulated_header(encapsulated: &EncapsulatedData) -> Result<String, IcapError> {
    let mut parts = Vec::new();
    let mut offset = 0;
    
    // Calculate actual offsets for each section based on real data sizes
    if let Some(req_hdr) = &encapsulated.req_hdr {
        parts.push(format!("req-hdr={}", offset));
        // Calculate actual header size by serializing it
        let header_size = serialize_http_headers(req_hdr)?.len();
        offset += header_size;
    }
    
    if let Some(res_hdr) = &encapsulated.res_hdr {
        parts.push(format!("res-hdr={}", offset));
        // Calculate actual header size by serializing it
        let header_size = serialize_http_headers(res_hdr)?.len();
        offset += header_size;
    }
    
    if let Some(req_body) = &encapsulated.req_body {
        parts.push(format!("req-body={}", offset));
        offset += req_body.len();
    }
    
    if let Some(res_body) = &encapsulated.res_body {
        parts.push(format!("res-body={}", offset));
        offset += res_body.len();
    }
    
    if encapsulated.null_body {
        parts.push("null-body=0".to_string());
    }
    
    Ok(parts.join(", "))
}

/// Serialize HTTP headers to bytes for size calculation
fn serialize_http_headers(headers: &HeaderMap) -> Result<Vec<u8>, IcapError> {
    let mut output = Vec::new();
    
    for (name, value) in headers {
        output.extend_from_slice(name.as_str().as_bytes());
        output.extend_from_slice(b": ");
        output.extend_from_slice(value.as_bytes());
        output.extend_from_slice(b"\r\n");
    }
    
    // Add final CRLF
    output.extend_from_slice(b"\r\n");
    
    Ok(output)
}

/// Parse body data, handling both chunked and non-chunked data
fn parse_body_data(data: &[u8]) -> Result<Bytes, IcapError> {
    if data.is_empty() {
        return Ok(Bytes::new());
    }
    
    // Check if data looks like chunked encoding (starts with hex digits)
    if is_chunked_data(data) {
        parse_chunked_body(data)
    } else {
        // Treat as raw data
        Ok(Bytes::from(data.to_vec()))
    }
}

/// Check if data appears to be chunked transfer encoded
fn is_chunked_data(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }
    
    // Look for the first line to see if it's a hex number followed by CRLF
    if let Some(crlf_pos) = data.windows(2).position(|w| w == b"\r\n") {
        if crlf_pos > 0 && crlf_pos < 20 { // Reasonable chunk size line length
            let first_line = &data[..crlf_pos];
            if let Ok(line_str) = std::str::from_utf8(first_line) {
                // Check if it's a valid hex number
                return line_str.chars().all(|c| c.is_ascii_hexdigit());
            }
        }
    }
    
    false
}

/// Parse chunked transfer encoded body
fn parse_chunked_body(data: &[u8]) -> Result<Bytes, IcapError> {
    let mut parser = ChunkedParser::new();
    let (decoded_data, _consumed) = parser.parse_chunk(data)
        .map_err(|e| IcapError::protocol_error(&e.to_string(), "CHUNKED"))?;
    
    if !parser.is_complete() {
        return Err(IcapError::protocol_error("Incomplete chunked data", "CHUNKED"));
    }
    
    Ok(Bytes::from(decoded_data))
}
