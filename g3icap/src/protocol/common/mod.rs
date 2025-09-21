//! Common ICAP protocol types and utilities

use crate::error::IcapError;
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
    /// Parse ICAP request from bytes
    pub fn parse_request(data: &[u8]) -> Result<IcapRequest, IcapError> {
        println!("DEBUG: Parsing ICAP request, {} bytes", data.len());
        println!("DEBUG: Request data: {}", String::from_utf8_lossy(data));
        
        let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
        
        if lines.is_empty() {
            println!("DEBUG: Empty request");
            return Err(IcapError::Protocol("Empty request".to_string()));
        }
        
        // Parse request line
        let request_line = lines[0];
        
        let request_parts: Vec<&[u8]> = request_line.split(|&b| b == b' ').collect();
        if request_parts.len() < 3 {
            return Err(IcapError::Protocol("Invalid request line format".to_string()));
        }
        
        let method = String::from_utf8_lossy(request_parts[0]).to_string();
        let uri_str = String::from_utf8_lossy(request_parts[1]).to_string();
        let version_str = String::from_utf8_lossy(request_parts[2]).trim().to_string();
        
        let method = IcapMethod::from(method.as_str());
        let uri = uri_str.parse::<Uri>()
            .map_err(|e| IcapError::Protocol(format!("Invalid URI: {}", e)))?;
        let version = parse_http_version(&version_str)?;
        
        // Parse headers
        let mut headers = HeaderMap::new();
        let mut _body_start = 0;
        
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() || (line.len() == 1 && line[0] == b'\r') {
                _body_start = i + 1;
                break;
            }
            
            if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
                let header_name = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
                let header_value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
                
                if let (Ok(name), Ok(value)) = (header_name.parse::<http::HeaderName>(), header_value.parse::<http::HeaderValue>()) {
                    headers.insert(name, value);
                }
            }
        }
        
        // Parse body
        let body = if _body_start < lines.len() {
            let body_lines = &lines[_body_start..];
            if body_lines.is_empty() {
                Bytes::new()
            } else {
                Bytes::from(body_lines.join(&b'\n'))
            }
        } else {
            Bytes::new()
        };
        
        // Parse encapsulated data if present
        let encapsulated = if let Some(encapsulated_header) = headers.get("encapsulated") {
            Some(parse_encapsulated_data(encapsulated_header, &body)?)
        } else {
            None
        };
        
        Ok(IcapRequest {
            method,
            uri,
            version,
            headers,
            body,
            encapsulated,
        })
    }

    /// Parse ICAP response from bytes
    pub fn parse_response(data: &[u8]) -> Result<IcapResponse, IcapError> {
        let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
        
        if lines.is_empty() {
            return Err(IcapError::Protocol("Empty response".to_string()));
        }
        
        // Parse status line
        let status_line = lines[0];
        
        let status_parts: Vec<&[u8]> = status_line.split(|&b| b == b' ').collect();
        if status_parts.len() < 3 {
            return Err(IcapError::Protocol("Invalid status line format".to_string()));
        }
        
        let version_str = String::from_utf8_lossy(status_parts[0]).trim().to_string();
        let status_code = status_parts[1].iter()
            .filter(|&&b| b.is_ascii_digit())
            .fold(0u16, |acc, &b| acc * 10 + (b - b'0') as u16);
        
        let version = parse_http_version(&version_str)?;
        let status = StatusCode::from_u16(status_code)
            .map_err(|e| IcapError::Protocol(format!("Invalid status code: {}", e)))?;
        
        // Parse headers
        let mut headers = HeaderMap::new();
        let mut _body_start = 0;
        
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() || (line.len() == 1 && line[0] == b'\r') {
                _body_start = i + 1;
                break;
            }
            
            if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
                let header_name = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
                let header_value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
                
                if let (Ok(name), Ok(value)) = (header_name.parse::<http::HeaderName>(), header_value.parse::<http::HeaderValue>()) {
                    headers.insert(name, value);
                }
            }
        }
        
        // Parse body
        let body = if _body_start < lines.len() {
            let body_lines = &lines[_body_start..];
            if body_lines.is_empty() {
                Bytes::new()
            } else {
                Bytes::from(body_lines.join(&b'\n'))
            }
        } else {
            Bytes::new()
        };
        
        // Parse encapsulated data if present
        let encapsulated = if let Some(encapsulated_header) = headers.get("encapsulated") {
            Some(parse_encapsulated_data(encapsulated_header, &body)?)
        } else {
            None
        };
        
        Ok(IcapResponse {
            status,
            version,
            headers,
            body,
            encapsulated,
        })
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
        _ => Err(IcapError::Protocol(format!("Unsupported version: {}", version_str))),
    }
}

/// Parse encapsulated data from ICAP message
fn parse_encapsulated_data(header: &http::HeaderValue, body: &[u8]) -> Result<EncapsulatedData, IcapError> {
    let header_str = header.to_str()
        .map_err(|e| IcapError::Protocol(format!("Invalid encapsulated header: {}", e)))?;
    
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
                .map_err(|e| IcapError::Protocol(format!("Invalid offset in encapsulated header: {}", e)))?;
            
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
            req_body = Some(Bytes::from(body[offset..safe_end_offset].to_vec()));
        } else if offset < body.len() {
            req_body = Some(Bytes::from(body[offset..].to_vec()));
        }
    }
    
    if let Some(offset) = res_body_offset {
        if offset < body.len() {
            res_body = Some(Bytes::from(body[offset..].to_vec()));
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
    
    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    
    for line in lines {
        if line.is_empty() || (line.len() == 1 && line[0] == b'\r') {
            break;
        }
        
        // Skip the HTTP request line (first line)
        if line.starts_with(b"GET ") || line.starts_with(b"POST ") || line.starts_with(b"HEAD ") || 
           line.starts_with(b"PUT ") || line.starts_with(b"DELETE ") || line.starts_with(b"OPTIONS ") {
            continue;
        }
        
        if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
            let header_name = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
            let header_value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
            
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
        let reason = response.status.canonical_reason().unwrap_or("Unknown");
        output.extend_from_slice(format!("ICAP/1.0 {} {}\r\n", 
            response.status.as_u16(), 
            reason
        ).as_bytes());
        
        // Serialize headers
        for (name, value) in &response.headers {
            output.extend_from_slice(format!("{}: {}\r\n", name, value.to_str().unwrap_or("")).as_bytes());
        }
        
        // Serialize encapsulated header if present and not already in headers
        if let Some(encapsulated) = &response.encapsulated {
            if !response.headers.contains_key("encapsulated") {
                let encapsulated_header = serialize_encapsulated_header(encapsulated)?;
                output.extend_from_slice(format!("Encapsulated: {}\r\n", encapsulated_header).as_bytes());
            }
        }
        
        // Empty line to separate headers from body
        output.extend_from_slice(b"\r\n");
        
        // Serialize body
        if !response.body.is_empty() {
            output.extend_from_slice(&response.body);
        }
        
        Ok(Bytes::from(output))
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

/// Serialize encapsulated header
fn serialize_encapsulated_header(encapsulated: &EncapsulatedData) -> Result<String, IcapError> {
    let mut parts = Vec::new();
    let mut offset = 0;
    
    // Calculate offsets for each section
    if encapsulated.req_hdr.is_some() {
        parts.push(format!("req-hdr={}", offset));
        // Estimate header size (this is simplified)
        offset += 200; // Rough estimate
    }
    
    if encapsulated.res_hdr.is_some() {
        parts.push(format!("res-hdr={}", offset));
        offset += 200; // Rough estimate
    }
    
    if encapsulated.req_body.is_some() {
        parts.push(format!("req-body={}", offset));
        if let Some(body) = &encapsulated.req_body {
            offset += body.len();
        }
    }
    
    if encapsulated.res_body.is_some() {
        parts.push(format!("res-body={}", offset));
    }
    
    if encapsulated.null_body {
        parts.push("null-body=0".to_string());
    }
    
    Ok(parts.join(", "))
}
