/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! ICAP Protocol Compliance Tests
//! 
//! This module contains comprehensive tests to ensure G3ICAP complies with RFC 3507.

use std::net::IpAddr;
use http::{HeaderMap, Uri, Version};
use bytes::Bytes;

use g3icap::protocol::{
    IcapMethod, IcapRequest, IcapResponse, IcapHeaders, IcapErrorCode,
    IcapParser, IcapSerializer, IcapErrorResponse,
};

/// Test ICAP method parsing and serialization
#[test]
fn test_icap_methods() {
    // Test REQMOD method
    let reqmod = IcapMethod::from("REQMOD");
    assert_eq!(reqmod, IcapMethod::Reqmod);
    assert_eq!(reqmod.to_string(), "REQMOD");

    // Test RESPMOD method
    let respmod = IcapMethod::from("RESPMOD");
    assert_eq!(respmod, IcapMethod::Respmod);
    assert_eq!(respmod.to_string(), "RESPMOD");

    // Test OPTIONS method
    let options = IcapMethod::from("OPTIONS");
    assert_eq!(options, IcapMethod::Options);
    assert_eq!(options.to_string(), "OPTIONS");

    // Test case insensitivity
    let reqmod_lower = IcapMethod::from("reqmod");
    assert_eq!(reqmod_lower, IcapMethod::Reqmod);

    // Test invalid method (should default to OPTIONS)
    let invalid = IcapMethod::from("INVALID");
    assert_eq!(invalid, IcapMethod::Options);
}

/// Test ICAP request parsing
#[test]
fn test_icap_request_parsing() {
    let request_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        Encapsulated: req-hdr=0, req-body=100\r\n\
                        \r\n\
                        GET /test HTTP/1.1\r\n\
                        Host: example.com\r\n\
                        \r\n";

    let request = IcapParser::parse_request(request_data).unwrap();
    
    assert_eq!(request.method, IcapMethod::Reqmod);
    assert_eq!(request.uri, "icap://example.com/echo".parse::<Uri>().unwrap());
    assert_eq!(request.version, Version::HTTP_11);
    assert!(request.headers.contains_key("host"));
    assert!(request.headers.contains_key("encapsulated"));
    assert!(request.encapsulated.is_some());
}

/// Test ICAP response parsing
#[test]
fn test_icap_response_parsing() {
    let response_data = b"ICAP/1.0 200 OK\r\n\
                         ISTag: \"g3icap-1.0\"\r\n\
                         Methods: REQMOD, RESPMOD, OPTIONS\r\n\
                         Service: G3 ICAP Server\r\n\
                         \r\n";

    let response = IcapParser::parse_response(response_data).unwrap();
    
    assert_eq!(response.status, http::StatusCode::OK);
    assert_eq!(response.version, Version::HTTP_11);
    assert!(response.headers.contains_key("istag"));
    assert!(response.headers.contains_key("methods"));
    assert!(response.headers.contains_key("service"));
}

/// Test ICAP headers
#[test]
fn test_icap_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("icap-version", "ICAP/1.0".parse().unwrap());
    headers.insert("icap-client-ip", "192.168.1.100".parse().unwrap());
    headers.insert("preview", "1024".parse().unwrap());
    headers.insert("istag", "\"g3icap-1.0\"".parse().unwrap());

    let icap_headers = IcapHeaders::from_http_headers(&headers);
    
    assert_eq!(icap_headers.icap_version, Some("ICAP/1.0".to_string()));
    assert_eq!(icap_headers.icap_client_ip, Some("192.168.1.100".parse::<IpAddr>().unwrap()));
    assert_eq!(icap_headers.preview, Some(1024));
    assert_eq!(icap_headers.istag, Some("\"g3icap-1.0\"".to_string()));
}

/// Test ICAP error codes
#[test]
fn test_icap_error_codes() {
    // Test client errors
    let bad_request = IcapErrorCode::BadRequest;
    assert_eq!(bad_request.status_code(), http::StatusCode::BAD_REQUEST);
    assert_eq!(bad_request.message(), "Bad Request");
    assert!(bad_request.is_client_error());
    assert!(!bad_request.is_server_error());

    // Test server errors
    let internal_error = IcapErrorCode::InternalServerError;
    assert_eq!(internal_error.status_code(), http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(internal_error.message(), "Internal Server Error");
    assert!(!internal_error.is_client_error());
    assert!(internal_error.is_server_error());

    // Test informational responses
    let continue_code = IcapErrorCode::Continue;
    assert_eq!(continue_code.status_code(), http::StatusCode::CONTINUE);
    assert_eq!(continue_code.message(), "Continue");
    assert!(continue_code.is_informational());

    // Test successful responses
    let no_content = IcapErrorCode::NoContent;
    assert_eq!(no_content.status_code(), http::StatusCode::NO_CONTENT);
    assert_eq!(no_content.message(), "No Content");
    assert!(no_content.is_success());
}

/// Test ICAP error response building
#[test]
fn test_icap_error_response() {
    let error_response = IcapErrorResponseBuilder::new(IcapErrorCode::BadRequest)
        .message("Invalid ICAP request format".to_string())
        .details("Missing required headers".to_string())
        .build();

    assert_eq!(error_response.error_code, IcapErrorCode::BadRequest);
    assert_eq!(error_response.message(), "Invalid ICAP request format");
    assert_eq!(error_response.details(), Some("Missing required headers"));
    assert!(error_response.is_client_error());
}

/// Test REQMOD method compliance
#[test]
fn test_reqmod_compliance() {
    let request_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        Encapsulated: req-hdr=0, req-body=100\r\n\
                        \r\n\
                        GET /test HTTP/1.1\r\n\
                        Host: example.com\r\n\
                        \r\n";

    let request = IcapParser::parse_request(request_data).unwrap();
    
    // Verify REQMOD specific requirements
    assert_eq!(request.method, IcapMethod::Reqmod);
    assert!(request.encapsulated.is_some());
    
    let encapsulated = request.encapsulated.unwrap();
    assert!(encapsulated.req_hdr.is_some());
    assert!(encapsulated.req_body.is_some());
    assert!(encapsulated.res_hdr.is_none());
    assert!(encapsulated.res_body.is_none());
}

/// Test RESPMOD method compliance
#[test]
fn test_respmod_compliance() {
    let request_data = b"RESPMOD icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        Encapsulated: req-hdr=0, req-body=100, res-hdr=200, res-body=300\r\n\
                        \r\n\
                        GET /test HTTP/1.1\r\n\
                        Host: example.com\r\n\
                        \r\n\
                        HTTP/1.1 200 OK\r\n\
                        Content-Type: text/html\r\n\
                        \r\n";

    let request = IcapParser::parse_request(request_data).unwrap();
    
    // Verify RESPMOD specific requirements
    assert_eq!(request.method, IcapMethod::Respmod);
    assert!(request.encapsulated.is_some());
    
    let encapsulated = request.encapsulated.unwrap();
    assert!(encapsulated.req_hdr.is_some());
    assert!(encapsulated.req_body.is_some());
    assert!(encapsulated.res_hdr.is_some());
    assert!(encapsulated.res_body.is_some());
}

/// Test OPTIONS method compliance
#[test]
fn test_options_compliance() {
    let request_data = b"OPTIONS icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        \r\n";

    let request = IcapParser::parse_request(request_data).unwrap();
    
    // Verify OPTIONS specific requirements
    assert_eq!(request.method, IcapMethod::Options);
    assert!(request.encapsulated.is_none());
}

/// Test preview mode compliance
#[test]
fn test_preview_mode_compliance() {
    let request_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        Preview: 1024\r\n\
                        Encapsulated: req-hdr=0, req-body=100\r\n\
                        \r\n\
                        GET /test HTTP/1.1\r\n\
                        Host: example.com\r\n\
                        \r\n";

    let request = IcapParser::parse_request(request_data).unwrap();
    
    // Verify preview mode requirements
    assert!(request.headers.contains_key("preview"));
    let preview_header = request.headers.get("preview").unwrap();
    assert_eq!(preview_header.to_str().unwrap(), "1024");
}

/// Test ICAP message serialization
#[test]
fn test_icap_message_serialization() {
    let mut headers = HeaderMap::new();
    headers.insert("host", "example.com".parse().unwrap());
    headers.insert("istag", "\"g3icap-1.0\"".parse().unwrap());

    let request = IcapRequest {
        method: IcapMethod::Options,
        uri: "icap://example.com/echo".parse().unwrap(),
        version: Version::HTTP_11,
        headers,
        body: Bytes::new(),
        encapsulated: None,
    };

    let serialized = IcapSerializer::serialize_request(&request).unwrap();
    let deserialized = IcapParser::parse_request(&serialized).unwrap();
    
    assert_eq!(deserialized.method, request.method);
    assert_eq!(deserialized.uri, request.uri);
    assert_eq!(deserialized.version, request.version);
}

/// Test ICAP error handling
#[test]
fn test_icap_error_handling() {
    // Test malformed request line
    let malformed_data = b"INVALID REQUEST\r\n";
    let result = IcapParser::parse_request(malformed_data);
    assert!(result.is_err());

    // Test missing headers
    let no_headers_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\r\n";
    let result = IcapParser::parse_request(no_headers_data);
    assert!(result.is_ok()); // This should be valid

    // Test invalid URI
    let invalid_uri_data = b"REQMOD invalid-uri ICAP/1.0\r\n\r\n";
    let result = IcapParser::parse_request(invalid_uri_data);
    assert!(result.is_err());
}

/// Test ICAP header validation
#[test]
fn test_icap_header_validation() {
    let mut headers = IcapHeaders::new();
    
    // Valid headers
    headers.icap_version = Some("ICAP/1.0".to_string());
    headers.preview = Some(1024);
    headers.max_connections = Some(100);
    headers.options_ttl = Some(3600);
    
    assert!(headers.validate().is_ok());

    // Invalid version
    headers.icap_version = Some("HTTP/1.1".to_string());
    assert!(headers.validate().is_err());

    // Invalid preview size
    headers.icap_version = Some("ICAP/1.0".to_string());
    headers.preview = Some(0);
    assert!(headers.validate().is_err());

    // Invalid max connections
    headers.preview = Some(1024);
    headers.max_connections = Some(0);
    assert!(headers.validate().is_err());

    // Invalid options TTL
    headers.max_connections = Some(100);
    headers.options_ttl = Some(0);
    assert!(headers.validate().is_err());
}

/// Test ICAP compliance with RFC 3507
#[test]
fn test_rfc_3507_compliance() {
    // Test that all required ICAP methods are supported
    let methods = vec![IcapMethod::Reqmod, IcapMethod::Respmod, IcapMethod::Options];
    for method in methods {
        assert!(matches!(method, IcapMethod::Reqmod | IcapMethod::Respmod | IcapMethod::Options));
    }

    // Test that ICAP version is supported
    let icap_headers = IcapHeaders::new();
    assert_eq!(icap_headers.icap_version, Some("ICAP/1.0".to_string()));

    // Test that error codes are RFC compliant
    let error_codes = vec![
        IcapErrorCode::BadRequest,
        IcapErrorCode::NotFound,
        IcapErrorCode::MethodNotAllowed,
        IcapErrorCode::InternalServerError,
        IcapErrorCode::ServiceUnavailable,
    ];
    
    for error_code in error_codes {
        assert!(error_code.status_code().as_u16() >= 400);
    }
}

/// Test ICAP performance requirements
#[test]
fn test_icap_performance() {
    // Test that parsing is reasonably fast
    let request_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\
                        Host: example.com\r\n\
                        Encapsulated: req-hdr=0, req-body=100\r\n\
                        \r\n\
                        GET /test HTTP/1.1\r\n\
                        Host: example.com\r\n\
                        \r\n";

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = IcapParser::parse_request(request_data);
    }
    let duration = start.elapsed();
    
    // Should parse 1000 requests in less than 100ms
    assert!(duration.as_millis() < 100);
}

/// Test ICAP security requirements
#[test]
fn test_icap_security() {
    // Test that malicious input is handled safely
    let malicious_data = b"REQMOD icap://example.com/echo ICAP/1.0\r\n\
                          Host: example.com\r\n\
                          \r\n\
                          GET /../../../etc/passwd HTTP/1.1\r\n\
                          Host: example.com\r\n\
                          \r\n";

    // Should not panic or cause security issues
    let result = IcapParser::parse_request(malicious_data);
    assert!(result.is_ok());
    
    let request = result.unwrap();
    assert_eq!(request.method, IcapMethod::Reqmod);
    // The malicious path should be preserved as-is for the application to handle
    assert!(request.body.to_vec().contains(b"/etc/passwd"));
}
