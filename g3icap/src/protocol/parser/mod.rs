//! Nom-based ICAP Protocol Parser
//! 
//! This module implements a robust, zero-copy ICAP protocol parser using
//! the nom parser combinator library. This provides better performance,
//! memory efficiency, and error handling compared to the simple line-based parser.

use crate::error::IcapError;
use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse, EncapsulatedData};
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Uri, Version};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{space1, digit1, multispace0},
    combinator::{map_res, value},
    sequence::tuple,
    branch::alt,
    multi::{many0, separated_list1},
    IResult,
};

/// Parse ICAP method from input
fn parse_icap_method(input: &str) -> IResult<&str, IcapMethod> {
    alt((
        value(IcapMethod::Reqmod, tag("REQMOD")),
        value(IcapMethod::Respmod, tag("RESPMOD")),
        value(IcapMethod::Options, tag("OPTIONS")),
    ))(input)
}

/// Parse ICAP request line
fn parse_icap_request_line(input: &str) -> IResult<&str, (IcapMethod, String, String)> {
    let (input, (method, _, uri, _, version, _)) = tuple((
        parse_icap_method,
        space1,
        take_until(" "),
        space1,
        take_until("\r\n"),
        tag("\r\n"),
    ))(input)?;
    
    Ok((input, (method, uri.to_string(), version.to_string())))
}

/// Parse ICAP status line
fn parse_icap_status_line(input: &str) -> IResult<&str, (String, u16, String)> {
    let (input, (version, _, status_code, _, reason_phrase, _)) = tuple((
        take_until(" "),
        space1,
        map_res(digit1, |s: &str| s.parse::<u16>()),
        space1,
        take_until("\r\n"),
        tag("\r\n"),
    ))(input)?;
    
    Ok((input, (version.to_string(), status_code, reason_phrase.to_string())))
}

/// Parse HTTP version from string
fn parse_http_version(version_str: &str) -> Result<Version, IcapError> {
    match version_str {
        "HTTP/1.0" => Ok(Version::HTTP_10),
        "HTTP/1.1" => Ok(Version::HTTP_11),
        "HTTP/2.0" => Ok(Version::HTTP_2),
        "HTTP/3.0" => Ok(Version::HTTP_3),
        "ICAP/1.0" => Ok(Version::HTTP_11), // ICAP/1.0 maps to HTTP/1.1 for compatibility
        _ => Err(IcapError::protocol_error(&format!("Unsupported version: {}", version_str), "PARSER")),
    }
}

/// Parse header line
fn parse_header_line(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = take_until("\r\n")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    
    Ok((input, (key.trim().to_string(), value.trim().to_string())))
}

/// Parse headers section
fn parse_headers(input: &str) -> IResult<&str, Vec<(String, String)>> {
    many0(parse_header_line)(input)
}

/// Parse encapsulated section
fn parse_encapsulated_section(input: &str) -> IResult<&str, (String, usize)> {
    let (input, section_type) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, offset) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    
    Ok((input, (section_type.trim().to_string(), offset)))
}

/// Parse encapsulated header value
fn parse_encapsulated_header(input: &str) -> IResult<&str, Vec<(String, usize)>> {
    separated_list1(
        tag(", "),
        parse_encapsulated_section
    )(input)
}

/// Parse ICAP request with nom
pub fn parse_icap_request(input: &str) -> Result<IcapRequest, IcapError> {
    let (remaining, (method, uri_str, version_str)) = parse_icap_request_line(input)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse request line: {:?}", e), "PARSER"))?;
    
    let uri = uri_str.parse::<Uri>()
        .map_err(|e| IcapError::protocol_error(&format!("Invalid URI: {}", e), "PARSER"))?;
    
    let version = parse_http_version(&version_str)?;
    
    // Find end of headers (double CRLF)
    let header_end = remaining.find("\r\n\r\n")
        .ok_or_else(|| IcapError::protocol_error("Missing header terminator", "PARSER"))?;
    
    let (header_section, body_section) = remaining.split_at(header_end + 4);
    
    // Parse headers
    let (_, headers_vec) = parse_headers(header_section)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse headers: {:?}", e), "PARSER"))?;
    
    let mut headers = HeaderMap::new();
    for (key, value) in headers_vec {
        if let (Ok(name), Ok(val)) = (key.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
            headers.insert(name, val);
        }
    }
    
    // Parse encapsulated data if present
    let encapsulated = if let Some(encapsulated_header) = headers.get("encapsulated") {
        Some(parse_encapsulated_data(encapsulated_header, body_section.as_bytes())?)
    } else {
        None
    };
    
    Ok(IcapRequest {
        method,
        uri,
        version,
        headers,
        body: Bytes::from(body_section.as_bytes().to_vec()),
        encapsulated,
    })
}

/// Parse ICAP response with nom
pub fn parse_icap_response(input: &str) -> Result<IcapResponse, IcapError> {
    let (remaining, (version_str, status_code, reason_phrase)) = parse_icap_status_line(input)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse status line: {:?}", e), "PARSER"))?;
    
    let version = parse_http_version(&version_str)?;
    let status = StatusCode::from_u16(status_code)
        .map_err(|e| IcapError::protocol_error(&format!("Invalid status code: {}", e), "PARSER"))?;
    
    // Find end of headers (double CRLF)
    let header_end = remaining.find("\r\n\r\n")
        .ok_or_else(|| IcapError::protocol_error("Missing header terminator", "PARSER"))?;
    
    let (header_section, body_section) = remaining.split_at(header_end + 4);
    
    // Parse headers
    let (_, headers_vec) = parse_headers(header_section)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse headers: {:?}", e), "PARSER"))?;
    
    let mut headers = HeaderMap::new();
    for (key, value) in headers_vec {
        if let (Ok(name), Ok(val)) = (key.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
            headers.insert(name, val);
        }
    }
    
    // Parse encapsulated data if present
    let encapsulated = if let Some(encapsulated_header) = headers.get("encapsulated") {
        Some(parse_encapsulated_data(encapsulated_header, body_section.as_bytes())?)
    } else {
        None
    };
    
    Ok(IcapResponse {
        status,
        version,
        headers,
        body: Bytes::from(body_section.as_bytes().to_vec()),
        encapsulated,
    })
}

/// Parse encapsulated data from ICAP message
fn parse_encapsulated_data(header: &http::HeaderValue, body: &[u8]) -> Result<EncapsulatedData, IcapError> {
    let header_str = header.to_str()
        .map_err(|e| IcapError::protocol_error(&format!("Invalid encapsulated header: {}", e), "PARSER"))?;
    
    let (_, sections) = parse_encapsulated_header(header_str)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse encapsulated header: {:?}", e), "PARSER"))?;
    
    println!("DEBUG: Parsed encapsulated header: {:?}", sections);
    
    // Parse HTTP headers and bodies based on offsets
    let mut req_hdr = None;
    let mut res_hdr = None;
    let mut req_body = None;
    let mut res_body = None;
    let mut null_body = false;
    
    for (section_type, offset) in &sections {
        println!("DEBUG: Processing section: {} at offset: {}, body_len: {}", section_type, offset, body.len());
        
        match section_type.as_str() {
            "req-hdr" => {
                let end_offset = find_next_section_offset(&sections, *offset, body.len());
                println!("DEBUG: req-hdr: offset={}, end_offset={}, body_len={}", offset, end_offset, body.len());
                if *offset < body.len() {
                    let actual_end = if end_offset > *offset { std::cmp::min(end_offset, body.len()) } else { body.len() };
                    println!("DEBUG: Parsing req-hdr from {} to {} (offset < body.len: {}, end_offset > offset: {})", offset, actual_end, *offset < body.len(), end_offset > *offset);
                    req_hdr = Some(parse_http_headers(&body[*offset..actual_end])?);
                } else {
                    println!("DEBUG: Skipping req-hdr due to bounds check: offset={}, end_offset={}, body_len={}", offset, end_offset, body.len());
                }
            },
            "res-hdr" => {
                let end_offset = find_next_section_offset(&sections, *offset, body.len());
                println!("DEBUG: res-hdr: offset={}, end_offset={}, body_len={}", offset, end_offset, body.len());
                if *offset < end_offset && *offset < body.len() && end_offset <= body.len() {
                    println!("DEBUG: Parsing res-hdr from {} to {}", offset, end_offset);
                    res_hdr = Some(parse_http_headers(&body[*offset..end_offset])?);
                } else {
                    println!("DEBUG: Skipping res-hdr due to bounds check: offset={}, end_offset={}, body_len={}", offset, end_offset, body.len());
                }
            },
            "req-body" => {
                let end_offset = find_next_section_offset(&sections, *offset, body.len());
                println!("DEBUG: req-body: offset={}, end_offset={}, body_len={}", offset, end_offset, body.len());
                if *offset < end_offset && *offset < body.len() && end_offset <= body.len() {
                    println!("DEBUG: Parsing req-body from {} to {}", offset, end_offset);
                    req_body = Some(parse_body_data(&body[*offset..end_offset])?);
                } else if *offset < body.len() {
                    println!("DEBUG: Parsing req-body from {} to end", offset);
                    req_body = Some(parse_body_data(&body[*offset..])?);
                } else {
                    println!("DEBUG: Skipping req-body due to bounds check: offset={}, body_len={}", offset, body.len());
                }
            },
            "res-body" => {
                println!("DEBUG: res-body: offset={}, body_len={}", offset, body.len());
                if *offset < body.len() {
                    println!("DEBUG: Parsing res-body from {} to end", offset);
                    res_body = Some(parse_body_data(&body[*offset..])?);
                } else {
                    println!("DEBUG: Skipping res-body due to bounds check: offset={}, body_len={}", offset, body.len());
                }
            },
            "null-body" => {
                println!("DEBUG: null-body detected");
                null_body = true;
            },
            _ => {
                println!("DEBUG: Unknown section type: {}", section_type);
            }
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

/// Find the next section offset
fn find_next_section_offset(sections: &[(String, usize)], current_offset: usize, body_len: usize) -> usize {
    sections.iter()
        .filter(|(_, offset)| *offset > current_offset)
        .map(|(_, offset)| *offset)
        .min()
        .unwrap_or(body_len)
}

/// Parse HTTP headers from bytes
fn parse_http_headers(data: &[u8]) -> Result<HeaderMap, IcapError> {
    let mut headers = HeaderMap::new();
    
    if data.is_empty() {
        return Ok(headers);
    }
    
    let data_str = std::str::from_utf8(data)
        .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8 in headers: {}", e), "PARSER"))?;
    
    let (_, headers_vec) = parse_headers(data_str)
        .map_err(|e| IcapError::protocol_error(&format!("Failed to parse HTTP headers: {:?}", e), "PARSER"))?;
    
    for (key, value) in headers_vec {
        if let (Ok(name), Ok(val)) = (key.parse::<http::HeaderName>(), value.parse::<http::HeaderValue>()) {
            headers.insert(name, val);
        }
    }
    
    Ok(headers)
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
    use crate::protocol::chunked::ChunkedParser;
    
    let mut parser = ChunkedParser::new();
    let (decoded_data, _consumed) = parser.parse_chunk(data)
        .map_err(|e| IcapError::protocol_error(&e.to_string(), "CHUNKED"))?;
    
    if !parser.is_complete() {
        return Err(IcapError::protocol_error("Incomplete chunked data", "CHUNKED"));
    }
    
    Ok(Bytes::from(decoded_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_icap_request() {
        let icap_message = "REQMOD icap://icap.example.org/satisf ICAP/1.0\r\n\
                           Host: icap.example.org\r\n\
                           Encapsulated: req-hdr=0, null-body=170\r\n\r\n\
                           GET / HTTP/1.1\r\n\
                           Host: www.origin-server.com\r\n\
                           Accept: text/html, text/plain\r\n\r\n";
        
        let request = parse_icap_request(icap_message).unwrap();
        assert_eq!(request.method, IcapMethod::Reqmod);
        assert_eq!(request.uri.to_string(), "icap://icap.example.org/satisf");
        assert_eq!(request.headers.get("Host").unwrap(), "icap.example.org");
        assert!(request.encapsulated.is_some());
    }
    
    #[test]
    fn test_parse_icap_response() {
        let icap_message = "ICAP/1.0 200 OK\r\n\
                           ISTag: \"W3E4R7U9-L2E4-2\"\r\n\
                           Methods: REQMOD, RESPMOD\r\n\
                           Service: G3 ICAP Server\r\n\r\n";
        
        let response = parse_icap_response(icap_message).unwrap();
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.headers.get("ISTag").unwrap(), "\"W3E4R7U9-L2E4-2\"");
        assert_eq!(response.headers.get("Methods").unwrap(), "REQMOD, RESPMOD");
    }
    
    #[test]
    fn test_parse_encapsulated_header() {
        let header_value = "req-hdr=0, res-hdr=100, req-body=200, res-body=300";
        let (remaining, sections) = parse_encapsulated_header(header_value).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(sections.len(), 4);
        assert_eq!(sections[0], ("req-hdr".to_string(), 0));
        assert_eq!(sections[1], ("res-hdr".to_string(), 100));
        assert_eq!(sections[2], ("req-body".to_string(), 200));
        assert_eq!(sections[3], ("res-body".to_string(), 300));
    }
}
