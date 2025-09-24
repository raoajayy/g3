//! Nom-based ICAP Protocol Parser (RFC 3507 compliant)

use crate::error::IcapError;
use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse, EncapsulatedData};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode, Uri, Version};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{space1, digit1, multispace0},
    combinator::{map_res, value},
    sequence::tuple,
    branch::alt,
    multi::{many0, separated_list1},
    IResult,
};

/// Parse ICAP method
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
    let (input, (version, _, status_code, _, _reason, _)) = tuple((
        take_until(" "),
        space1,
        map_res(digit1, |s: &str| s.parse::<u16>()),
        space1,
        take_until("\r\n"),
        tag("\r\n"),
    ))(input)?;
    Ok((input, (version.to_string(), status_code, _reason.to_string())))
}

/// Parse header line
fn parse_header_line(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, val) = take_until("\r\n")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, (key.trim().to_ascii_lowercase(), val.trim().to_string())))
}

/// Parse headers section
fn parse_headers(input: &str) -> IResult<&str, Vec<(String, String)>> {
    many0(parse_header_line)(input)
}

/// Parse encapsulated section entry
fn parse_encapsulated_section(input: &str) -> IResult<&str, (String, usize)> {
    let (input, section) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, offset) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    Ok((input, (section.trim().to_ascii_lowercase(), offset)))
}

/// Parse encapsulated header value
fn parse_encapsulated_header(input: &str) -> IResult<&str, Vec<(String, usize)>> {
    separated_list1(tag(", "), parse_encapsulated_section)(input)
}

/// Find next section offset or body end
fn find_next_section_offset(sections: &[(String, usize)], current: usize, body_len: usize) -> usize {
    sections.iter()
        .filter(|(_, off)| *off > current)
        .map(|(_, off)| *off)
        .min()
        .unwrap_or(body_len)
}

/// Parse ICAP request
pub fn parse_icap_request(input: &str) -> Result<IcapRequest, IcapError> {
    let (rem, (method, uri_s, version_s)) = parse_icap_request_line(input)
        .map_err(|e| IcapError::protocol_error(&format!("Bad request line: {:?}", e), "PARSER"))?;
    let uri = uri_s.parse::<Uri>()
        .map_err(|e| IcapError::protocol_error(&format!("Invalid URI: {}", e), "PARSER"))?;
    let version = match version_s.as_str() {
        "ICAP/1.0" => Version::HTTP_11, // ICAP/1.0 maps to HTTP/1.1 for compatibility
        _ => return Err(IcapError::protocol_error(&format!("Unsupported version: {}", version_s), "PARSER")),
    };
    
    let idx = rem.find("\r\n\r\n")
        .ok_or_else(|| IcapError::protocol_error("Missing header terminator", "PARSER"))?;
    let (hdrs_str, body_str) = rem.split_at(idx + 4);
    let (_, kvs) = parse_headers(hdrs_str)
        .map_err(|e| IcapError::protocol_error(&format!("Header parse failure: {:?}", e), "PARSER"))?;
    
    let mut headers = HeaderMap::new();
    for (k, v) in kvs {
        let name = HeaderName::from_bytes(k.as_bytes()).map_err(|_| IcapError::protocol_error("Bad header name", "PARSER"))?;
        let val = HeaderValue::from_str(&v).map_err(|_| IcapError::protocol_error("Bad header value", "PARSER"))?;
            headers.insert(name, val);
    }

    // Required header checks
    if !headers.contains_key("host") {
        return Err(IcapError::protocol_error("Host header required", "PARSER"));
    }
    let enc_hdr = headers.get("encapsulated")
        .ok_or_else(|| IcapError::protocol_error("Encapsulated header required", "PARSER"))?;
    let enc_str = enc_hdr.to_str()
        .map_err(|_| IcapError::protocol_error("Invalid encapsulated value", "PARSER"))?;
    let (_, sections) = parse_encapsulated_header(enc_str)
        .map_err(|e| IcapError::protocol_error(&format!("Encap parse error: {:?}", e), "PARSER"))?;

    // Offsets must increase
    for w in sections.windows(2) {
        if w[1].1 <= w[0].1 {
            return Err(IcapError::protocol_error("Encap offsets not increasing", "PARSER"));
        }
    }

    // Body must be chunked
    let body_bytes = body_str.as_bytes();
    if !is_chunked_data(body_bytes) && !sections.iter().any(|(t, _)| t == "null-body") {
        return Err(IcapError::protocol_error("Chunked encoding required", "PARSER"));
    }

    // Parse encapsulated data
    let encapsulated = Some(parse_encapsulated_data(enc_hdr, body_bytes)?);
    
    Ok(IcapRequest {
        method,
        uri,
        version,
        headers,
        body: Bytes::from(body_bytes.to_vec()),
        encapsulated,
    })
}

/// Parse ICAP response
pub fn parse_icap_response(input: &str) -> Result<IcapResponse, IcapError> {
    let (rem, (vers, code, _reason)) = parse_icap_status_line(input)
        .map_err(|e| IcapError::protocol_error(&format!("Bad status line: {:?}", e), "PARSER"))?;
    let version = match vers.as_str() {
        "ICAP/1.0" => Version::HTTP_11, // ICAP/1.0 maps to HTTP/1.1 for compatibility
        _ => return Err(IcapError::protocol_error(&format!("Unsupported version: {}", vers), "PARSER")),
    };
    let status = StatusCode::from_u16(code)
        .map_err(|_| IcapError::protocol_error(&format!("Invalid status code: {}", code), "PARSER"))?;

    let idx = rem.find("\r\n\r\n")
        .ok_or_else(|| IcapError::protocol_error("Missing header terminator", "PARSER"))?;
    let (hdrs_str, body_str) = rem.split_at(idx + 4);
    let (_, kvs) = parse_headers(hdrs_str)
        .map_err(|e| IcapError::protocol_error(&format!("Header parse failure: {:?}", e), "PARSER"))?;
    
    let mut headers = HeaderMap::new();
    for (k, v) in kvs {
        let name = HeaderName::from_bytes(k.as_bytes()).map_err(|_| IcapError::protocol_error("Bad header name", "PARSER"))?;
        let val = HeaderValue::from_str(&v).map_err(|_| IcapError::protocol_error("Bad header value", "PARSER"))?;
            headers.insert(name, val);
    }

    // Required response headers
    if !headers.contains_key("istag") {
        return Err(IcapError::protocol_error("ISTag required", "PARSER"));
    }
    let enc_hdr = headers.get("encapsulated")
        .ok_or_else(|| IcapError::protocol_error("Encapsulated header required", "PARSER"))?;
    let enc_str = enc_hdr.to_str()
        .map_err(|_| IcapError::protocol_error("Invalid encapsulated value", "PARSER"))?;
    let (_, sections) = parse_encapsulated_header(enc_str)
        .map_err(|e| IcapError::protocol_error(&format!("Encap parse error: {:?}", e), "PARSER"))?;
    for w in sections.windows(2) {
        if w[1].1 <= w[0].1 {
            return Err(IcapError::protocol_error("Encap offsets not increasing", "PARSER"));
        }
    }
    let body_bytes = body_str.as_bytes();
    if !is_chunked_data(body_bytes) && !sections.iter().any(|(t, _)| t == "null-body") {
        return Err(IcapError::protocol_error("Chunked encoding required", "PARSER"));
    }

    let encapsulated = Some(parse_encapsulated_data(enc_hdr, body_bytes)?);
    
    Ok(IcapResponse {
        status,
        version,
        headers,
        body: Bytes::from(body_bytes.to_vec()),
        encapsulated,
    })
}

/// Check chunked transfer-coding
fn is_chunked_data(data: &[u8]) -> bool {
    if let Some(pos) = data.windows(2).position(|w| w == b"\r\n") {
        if pos > 0 && pos < 20 {
            if let Ok(line) = std::str::from_utf8(&data[..pos]) {
                return line.chars().all(|c| c.is_ascii_hexdigit());
            }
        }
    }
    false
}

/// Parse chunked body (delegates to chunked parser)
fn parse_chunked_body(data: &[u8]) -> Result<Bytes, IcapError> {
    use crate::protocol::chunked::ChunkedParser;
    let mut p = ChunkedParser::new();
    let (decoded, _consumed) =
        p.parse_chunk(data).map_err(|e| IcapError::protocol_error(&e.to_string(), "CHUNKED"))?;
    if !p.is_complete() {
        return Err(IcapError::protocol_error("Incomplete chunked data", "CHUNKED"));
    }
    Ok(Bytes::from(decoded))
}

/// Parse and split encapsulated data sections
fn parse_encapsulated_data(header: &HeaderValue, body: &[u8]) -> Result<EncapsulatedData, IcapError> {
    let s = header.to_str()
        .map_err(|_| IcapError::protocol_error("Bad encapsulated header", "PARSER"))?;
    let (_, sections) = parse_encapsulated_header(s)
        .map_err(|e| IcapError::protocol_error(&format!("Encap parse error: {:?}", e), "PARSER"))?;

    let mut req_hdr = None;
    let mut res_hdr = None;
    let mut req_body = None;
    let mut res_body = None;
    let mut null_body = false;
    
    for (typ, off) in &sections {
        let end = find_next_section_offset(&sections, *off, body.len());
        match typ.as_str() {
            "req-hdr" if *off < end => {
                req_hdr = Some(parse_http_headers(&body[*off..end])?);
            }
            "res-hdr" if *off < end => {
                res_hdr = Some(parse_http_headers(&body[*off..end])?);
            }
            "req-body" if *off < body.len() => {
                let slice = if end <= body.len() { &body[*off..end] } else { &body[*off..] };
                req_body = Some(if is_chunked_data(slice) { parse_chunked_body(slice)? } else { Bytes::from(slice.to_vec()) });
            }
            "res-body" if *off < body.len() => {
                let slice = &body[*off..];
                res_body = Some(if is_chunked_data(slice) { parse_chunked_body(slice)? } else { Bytes::from(slice.to_vec()) });
            }
            "null-body" => null_body = true,
            _ => {}
        }
    }
    
    Ok(EncapsulatedData {
        req_hdr,
        res_hdr,
        req_body,
        res_body,
        null_body,
    })
}

/// Parse HTTP headers from byte slice
fn parse_http_headers(data: &[u8]) -> Result<HeaderMap, IcapError> {
    let mut map = HeaderMap::new();
    if data.is_empty() {
        return Ok(map);
    }
    let s = std::str::from_utf8(data)
        .map_err(|e| IcapError::protocol_error(&format!("Invalid UTF-8: {}", e), "PARSER"))?;
    let (_, kvs) = parse_headers(s)
        .map_err(|e| IcapError::protocol_error(&format!("HTTP header parse failure: {:?}", e), "PARSER"))?;
    for (k, v) in kvs {
        if let Ok(name) = HeaderName::from_bytes(k.as_bytes()) {
            if let Ok(val) = HeaderValue::from_str(&v) {
                map.insert(name, val);
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_icap_request_minimal() {
        let msg = "REQMOD icap://ex/s ICAP/1.0\r\nHost: ex\r\nEncapsulated: null-body=0\r\n\r\n";
        let req = parse_icap_request(msg).unwrap();
        assert_eq!(req.method, IcapMethod::Reqmod);
        assert!(req.encapsulated.unwrap().null_body);
    }
    
    #[test]
    fn test_parse_icap_response_minimal() {
        let msg = "ICAP/1.0 204 No Content\r\nISTag: \"T\"\r\nEncapsulated: null-body=0\r\n\r\n";
        let res = parse_icap_response(msg).unwrap();
        assert_eq!(res.status, StatusCode::NO_CONTENT);
        assert!(res.encapsulated.unwrap().null_body);
    }
}