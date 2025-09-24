//! ICAP Response Validation Tests
//!
//! This module tests all ICAP response codes according to RFC 3507
//! to ensure proper formatting and compliance.

use g3icap::protocol::response_generator::IcapResponseGenerator;
use g3icap::protocol::common::{IcapMethod, EncapsulatedData};
use http::StatusCode;
use bytes::Bytes;

/// Test helper to validate ICAP response format
fn validate_icap_response(response: &g3icap::protocol::common::IcapResponse, expected_status: StatusCode) {
    // Validate status line format: ICAP/1.0 <code> <reason>
    assert_eq!(response.status, expected_status);
    assert_eq!(response.version, http::Version::HTTP_11);
    
    // Validate required headers
    assert!(response.headers.contains_key("server"));
    
    // Validate status-specific requirements
    match expected_status {
        StatusCode::CONTINUE => {
            // 100 Continue should have Encapsulated header
            assert!(response.headers.contains_key("encapsulated"));
            assert!(response.body.is_empty());
        },
        StatusCode::OK => {
            // 200 OK should have ISTag header
            assert!(response.headers.contains_key("istag"));
        },
        StatusCode::NO_CONTENT => {
            // 204 No Content should have ISTag header and no body
            assert!(response.headers.contains_key("istag"));
            assert!(response.body.is_empty());
        },
        StatusCode::FOUND => {
            // 302 Found should have Location header
            assert!(response.headers.contains_key("location"));
        },
        StatusCode::NOT_MODIFIED => {
            // 304 Not Modified should have no body
            assert!(response.body.is_empty());
        },
        StatusCode::BAD_REQUEST => {
            // 400 Bad Request should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::FORBIDDEN => {
            // 403 Forbidden should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::NOT_FOUND => {
            // 404 Not Found should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::METHOD_NOT_ALLOWED => {
            // 405 Method Not Allowed should have Allow header
            assert!(response.headers.contains_key("allow"));
        },
        StatusCode::PROXY_AUTHENTICATION_REQUIRED => {
            // 407 Proxy Auth Required should have Proxy-Authenticate header
            assert!(response.headers.contains_key("proxy-authenticate"));
        },
        StatusCode::CONFLICT => {
            // 409 Conflict should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::PAYLOAD_TOO_LARGE => {
            // 413 Request Too Large should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::UNSUPPORTED_MEDIA_TYPE => {
            // 415 Unsupported Media Type should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::INTERNAL_SERVER_ERROR => {
            // 500 Internal Server Error should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::NOT_IMPLEMENTED => {
            // 501 Not Implemented should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::BAD_GATEWAY => {
            // 502 Bad Gateway should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::SERVICE_UNAVAILABLE => {
            // 503 Service Unavailable should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        StatusCode::HTTP_VERSION_NOT_SUPPORTED => {
            // 505 ICAP Version Not Supported should have content-type and body
            assert!(response.headers.contains_key("content-type"));
            assert!(!response.body.is_empty());
        },
        _ => {
            // For other status codes, just ensure basic structure
            assert!(response.headers.contains_key("server"));
        }
    }
}

#[test]
fn test_100_continue_response() {
    let generator = IcapResponseGenerator::default();
    let response = generator.continue_response();
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::CONTINUE);
    
    // Validate specific 100 Continue requirements
    assert_eq!(response.status, StatusCode::CONTINUE);
    assert!(response.headers.contains_key("encapsulated"));
    assert!(response.body.is_empty());
    
    // Check that encapsulated header is properly formatted
    let encapsulated = response.headers.get("encapsulated").unwrap();
    assert_eq!(encapsulated.to_str().unwrap(), "null-body=0");
    
    println!("✅ 100 Continue response validation passed");
    println!("Response: ICAP/1.0 100 Continue");
    println!("Encapsulated: {}", encapsulated.to_str().unwrap());
}

#[test]
fn test_200_ok_modified_response() {
    let generator = IcapResponseGenerator::default();
    let body = Bytes::from("modified content");
    let encapsulated = Some(EncapsulatedData {
        req_hdr: Some(0),
        res_hdr: None,
        req_body: Some(100),
        res_body: None,
        null_body: Some(200),
        opt_body: None,
    });
    
    let response = generator.ok_modified(encapsulated, body.clone());
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::OK);
    
    // Validate specific 200 OK requirements
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.headers.contains_key("istag"));
    assert_eq!(response.body, body);
    assert!(response.headers.contains_key("encapsulated"));
    
    println!("✅ 200 OK Modified response validation passed");
    println!("Response: ICAP/1.0 200 OK");
    println!("ISTag: {}", response.headers.get("istag").unwrap().to_str().unwrap());
    println!("Body: {} bytes", response.body.len());
}

#[test]
fn test_204_no_modifications_response() {
    let generator = IcapResponseGenerator::default();
    let encapsulated = Some(EncapsulatedData {
        req_hdr: Some(0),
        res_hdr: None,
        req_body: None,
        res_body: None,
        null_body: Some(77),
        opt_body: None,
    });
    
    let response = generator.no_modifications(encapsulated);
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::NO_CONTENT);
    
    // Validate specific 204 No Content requirements
    assert_eq!(response.status, StatusCode::NO_CONTENT);
    assert!(response.headers.contains_key("istag"));
    assert!(response.body.is_empty());
    assert!(response.headers.contains_key("encapsulated"));
    
    println!("✅ 204 No Modifications response validation passed");
    println!("Response: ICAP/1.0 204 No Content");
    println!("ISTag: {}", response.headers.get("istag").unwrap().to_str().unwrap());
    println!("Encapsulated: {}", response.headers.get("encapsulated").unwrap().to_str().unwrap());
}

#[test]
fn test_302_found_response() {
    let generator = IcapResponseGenerator::default();
    let location = "icap://example.com/redirect";
    let response = generator.found(location);
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::FOUND);
    
    // Validate specific 302 Found requirements
    assert_eq!(response.status, StatusCode::FOUND);
    assert!(response.headers.contains_key("location"));
    assert_eq!(response.headers.get("location").unwrap().to_str().unwrap(), location);
    
    println!("✅ 302 Found response validation passed");
    println!("Response: ICAP/1.0 302 Found");
    println!("Location: {}", response.headers.get("location").unwrap().to_str().unwrap());
}

#[test]
fn test_304_not_modified_response() {
    let generator = IcapResponseGenerator::default();
    let response = generator.not_modified();
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::NOT_MODIFIED);
    
    // Validate specific 304 Not Modified requirements
    assert_eq!(response.status, StatusCode::NOT_MODIFIED);
    assert!(response.body.is_empty());
    
    println!("✅ 304 Not Modified response validation passed");
    println!("Response: ICAP/1.0 304 Not Modified");
}

#[test]
fn test_400_bad_request_response() {
    let generator = IcapResponseGenerator::default();
    let message = "Invalid ICAP request format";
    let response = generator.bad_request(Some(message));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::BAD_REQUEST);
    
    // Validate specific 400 Bad Request requirements
    assert_eq!(response.status, StatusCode::BAD_REQUEST);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(message.as_bytes()));
    
    println!("✅ 400 Bad Request response validation passed");
    println!("Response: ICAP/1.0 400 Bad Request");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_403_forbidden_response() {
    let generator = IcapResponseGenerator::default();
    let reason = "Access denied by policy";
    let response = generator.forbidden(Some(reason));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::FORBIDDEN);
    
    // Validate specific 403 Forbidden requirements
    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(reason.as_bytes()));
    
    println!("✅ 403 Forbidden response validation passed");
    println!("Response: ICAP/1.0 403 Forbidden");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_404_not_found_response() {
    let generator = IcapResponseGenerator::default();
    let service = "reqmod";
    let response = generator.not_found(Some(service));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::NOT_FOUND);
    
    // Validate specific 404 Not Found requirements
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(service.as_bytes()));
    
    println!("✅ 404 Not Found response validation passed");
    println!("Response: ICAP/1.0 404 Not Found");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_405_method_not_allowed_response() {
    let generator = IcapResponseGenerator::default();
    let method = IcapMethod::Respmod;
    let allowed_methods = vec![IcapMethod::Options, IcapMethod::Reqmod];
    let response = generator.method_not_allowed(&method, &allowed_methods);
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::METHOD_NOT_ALLOWED);
    
    // Validate specific 405 Method Not Allowed requirements
    assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED);
    assert!(response.headers.contains_key("allow"));
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    
    let allow_header = response.headers.get("allow").unwrap().to_str().unwrap();
    assert!(allow_header.contains("OPTIONS"));
    assert!(allow_header.contains("REQMOD"));
    
    println!("✅ 405 Method Not Allowed response validation passed");
    println!("Response: ICAP/1.0 405 Method Not Allowed");
    println!("Allow: {}", allow_header);
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_407_proxy_auth_required_response() {
    let generator = IcapResponseGenerator::default();
    let realm = "ICAP Server";
    let response = generator.proxy_auth_required(Some(realm));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::PROXY_AUTHENTICATION_REQUIRED);
    
    // Validate specific 407 Proxy Auth Required requirements
    assert_eq!(response.status, StatusCode::PROXY_AUTHENTICATION_REQUIRED);
    assert!(response.headers.contains_key("proxy-authenticate"));
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    
    let auth_header = response.headers.get("proxy-authenticate").unwrap().to_str().unwrap();
    assert!(auth_header.contains("Basic"));
    assert!(auth_header.contains(realm));
    
    println!("✅ 407 Proxy Authentication Required response validation passed");
    println!("Response: ICAP/1.0 407 Proxy Authentication Required");
    println!("Proxy-Authenticate: {}", auth_header);
}

#[test]
fn test_409_conflict_response() {
    let generator = IcapResponseGenerator::default();
    let reason = "Resource already exists";
    let response = generator.conflict(Some(reason));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::CONFLICT);
    
    // Validate specific 409 Conflict requirements
    assert_eq!(response.status, StatusCode::CONFLICT);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(reason.as_bytes()));
    
    println!("✅ 409 Conflict response validation passed");
    println!("Response: ICAP/1.0 409 Conflict");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_413_request_too_large_response() {
    let generator = IcapResponseGenerator::default();
    let max_size = 10485760; // 10MB
    let response = generator.request_too_large(Some(max_size));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::PAYLOAD_TOO_LARGE);
    
    // Validate specific 413 Request Too Large requirements
    assert_eq!(response.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(max_size.to_string().as_bytes()));
    
    println!("✅ 413 Request Too Large response validation passed");
    println!("Response: ICAP/1.0 413 Request Too Large");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_415_unsupported_media_type_response() {
    let generator = IcapResponseGenerator::default();
    let content_type = "application/unknown";
    let response = generator.unsupported_media_type(Some(content_type));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    
    // Validate specific 415 Unsupported Media Type requirements
    assert_eq!(response.status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(content_type.as_bytes()));
    
    println!("✅ 415 Unsupported Media Type response validation passed");
    println!("Response: ICAP/1.0 415 Unsupported Media Type");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_500_internal_server_error_response() {
    let generator = IcapResponseGenerator::default();
    let error = "Database connection failed";
    let response = generator.internal_server_error(Some(error));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::INTERNAL_SERVER_ERROR);
    
    // Validate specific 500 Internal Server Error requirements
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(error.as_bytes()));
    
    println!("✅ 500 Internal Server Error response validation passed");
    println!("Response: ICAP/1.0 500 Internal Server Error");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_501_not_implemented_response() {
    let generator = IcapResponseGenerator::default();
    let method = IcapMethod::Respmod;
    let response = generator.not_implemented(Some(&method));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::NOT_IMPLEMENTED);
    
    // Validate specific 501 Not Implemented requirements
    assert_eq!(response.status, StatusCode::NOT_IMPLEMENTED);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains("RESPMOD".as_bytes()));
    
    println!("✅ 501 Not Implemented response validation passed");
    println!("Response: ICAP/1.0 501 Not Implemented");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_502_bad_gateway_response() {
    let generator = IcapResponseGenerator::default();
    let reason = "Upstream server returned invalid response";
    let response = generator.bad_gateway(Some(reason));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::BAD_GATEWAY);
    
    // Validate specific 502 Bad Gateway requirements
    assert_eq!(response.status, StatusCode::BAD_GATEWAY);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(reason.as_bytes()));
    
    println!("✅ 502 Bad Gateway response validation passed");
    println!("Response: ICAP/1.0 502 Bad Gateway");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_503_service_unavailable_response() {
    let generator = IcapResponseGenerator::default();
    let retry_after = 300; // 5 minutes
    let response = generator.service_unavailable(Some(retry_after));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::SERVICE_UNAVAILABLE);
    
    // Validate specific 503 Service Unavailable requirements
    assert_eq!(response.status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(response.headers.contains_key("content-type"));
    assert!(response.headers.contains_key("retry-after"));
    assert!(!response.body.is_empty());
    
    let retry_header = response.headers.get("retry-after").unwrap().to_str().unwrap();
    assert_eq!(retry_header, "300");
    
    println!("✅ 503 Service Unavailable response validation passed");
    println!("Response: ICAP/1.0 503 Service Unavailable");
    println!("Retry-After: {}", retry_header);
}

#[test]
fn test_505_version_not_supported_response() {
    let generator = IcapResponseGenerator::default();
    let version = "ICAP/2.0";
    let response = generator.version_not_supported(Some(version));
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::HTTP_VERSION_NOT_SUPPORTED);
    
    // Validate specific 505 ICAP Version Not Supported requirements
    assert_eq!(response.status, StatusCode::HTTP_VERSION_NOT_SUPPORTED);
    assert!(response.headers.contains_key("content-type"));
    assert!(!response.body.is_empty());
    assert!(response.body.to_vec().contains(version.as_bytes()));
    
    println!("✅ 505 ICAP Version Not Supported response validation passed");
    println!("Response: ICAP/1.0 505 ICAP Version Not Supported");
    println!("Body: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn test_options_response_with_capabilities() {
    let generator = IcapResponseGenerator::default();
    let methods = vec![IcapMethod::Options, IcapMethod::Reqmod, IcapMethod::Respmod];
    let mut capabilities = std::collections::HashMap::new();
    capabilities.insert("preview".to_string(), "1024".to_string());
    capabilities.insert("max-connections".to_string(), "1000".to_string());
    capabilities.insert("x-content-filter".to_string(), "enabled".to_string());
    
    let response = generator.options_response(&methods, capabilities);
    
    // Validate basic structure
    validate_icap_response(&response, StatusCode::NO_CONTENT);
    
    // Validate OPTIONS-specific requirements
    assert_eq!(response.status, StatusCode::NO_CONTENT);
    assert!(response.headers.contains_key("istag"));
    assert!(response.headers.contains_key("methods"));
    assert!(response.headers.contains_key("service"));
    assert!(response.body.is_empty());
    
    // Validate methods header
    let methods_header = response.headers.get("methods").unwrap().to_str().unwrap();
    assert!(methods_header.contains("OPTIONS"));
    assert!(methods_header.contains("REQMOD"));
    assert!(methods_header.contains("RESPMOD"));
    
    // Validate capabilities
    assert!(response.headers.contains_key("preview"));
    assert!(response.headers.contains_key("max-connections"));
    assert!(response.headers.contains_key("x-content-filter"));
    
    println!("✅ OPTIONS response with capabilities validation passed");
    println!("Response: ICAP/1.0 204 No Content");
    println!("Methods: {}", methods_header);
    println!("Preview: {}", response.headers.get("preview").unwrap().to_str().unwrap());
}

#[test]
fn test_rfc_3507_compliance() {
    let generator = IcapResponseGenerator::default();
    
    // Test 100 Continue as specified in your example
    let response = generator.continue_response();
    
    // Validate RFC 3507 compliance for 100 Continue
    assert_eq!(response.status, StatusCode::CONTINUE);
    assert_eq!(response.version, http::Version::HTTP_11);
    assert!(response.headers.contains_key("encapsulated"));
    assert!(response.body.is_empty());
    
    let encapsulated = response.headers.get("encapsulated").unwrap().to_str().unwrap();
    assert_eq!(encapsulated, "null-body=0");
    
    println!("✅ RFC 3507 compliance validation passed");
    println!("ICAP/1.0 100 Continue");
    println!("Encapsulated: {}", encapsulated);
    println!("(Client should continue sending request body)");
}
