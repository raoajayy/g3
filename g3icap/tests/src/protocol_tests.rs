/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use g3icap::protocol::common::{IcapMethod, IcapParser, IcapRequest, IcapResponse, IcapSerializer};
use g3icap::protocol::error::ErrorResponseBuilder;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Uri, Version};

#[cfg(test)]
mod icap_method_tests {
    use super::*;

    #[test]
    fn test_icap_method_from_string() {
        assert_eq!(IcapMethod::from("REQMOD"), IcapMethod::Reqmod);
        assert_eq!(IcapMethod::from("RESPMOD"), IcapMethod::Respmod);
        assert_eq!(IcapMethod::from("OPTIONS"), IcapMethod::Options);
        assert_eq!(IcapMethod::from("UNKNOWN"), IcapMethod::Options); // Default fallback
    }

    #[test]
    fn test_icap_method_to_string() {
        assert_eq!(IcapMethod::Reqmod.to_string(), "REQMOD");
        assert_eq!(IcapMethod::Respmod.to_string(), "RESPMOD");
        assert_eq!(IcapMethod::Options.to_string(), "OPTIONS");
    }
}

#[cfg(test)]
mod icap_parser_tests {
    use super::*;

    #[test]
    fn test_parse_simple_options_request() {
        let data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\n\r\n";
        let request = IcapParser::parse_request(data).unwrap();
        
        assert_eq!(request.method, IcapMethod::Options);
        assert_eq!(request.uri, "icap://example.com/options".parse::<Uri>().unwrap());
        assert_eq!(request.version, Version::HTTP_11);
        assert!(request.headers.contains_key("host"));
        assert!(request.headers.contains_key("user-agent"));
        assert!(request.body.is_empty());
        assert!(request.encapsulated.is_none());
    }

    #[test]
    fn test_parse_reqmod_request_with_encapsulated() {
        let data = b"REQMOD icap://example.com/reqmod ICAP/1.0\r\nHost: example.com\r\nEncapsulated: req-hdr=0, req-body=200\r\n\r\nGET /test HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = IcapParser::parse_request(data).unwrap();
        
        assert_eq!(request.method, IcapMethod::Reqmod);
        assert_eq!(request.uri, "icap://example.com/reqmod".parse::<Uri>().unwrap());
        assert!(request.headers.contains_key("encapsulated"));
        assert!(request.encapsulated.is_some());
    }

    #[test]
    fn test_parse_respmod_request() {
        let data = b"RESPMOD icap://example.com/respmod ICAP/1.0\r\nHost: example.com\r\n\r\n";
        let request = IcapParser::parse_request(data).unwrap();
        
        assert_eq!(request.method, IcapMethod::Respmod);
        assert_eq!(request.uri, "icap://example.com/respmod".parse::<Uri>().unwrap());
    }

    #[test]
    fn test_parse_icap_response() {
        let data = b"ICAP/1.0 200 OK\r\nISTag: \"test-1.0\"\r\nMethods: REQMOD, RESPMOD\r\n\r\n";
        let response = IcapParser::parse_response(data).unwrap();
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.version, Version::HTTP_11);
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("methods"));
    }

    #[test]
    fn test_parse_invalid_request_line() {
        let data = b"INVALID REQUEST\r\n\r\n";
        let result = IcapParser::parse_request(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_malformed_headers() {
        let data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nInvalid-Header\r\n\r\n";
        let request = IcapParser::parse_request(data).unwrap();
        // Should still parse successfully, just ignore invalid headers
        assert_eq!(request.method, IcapMethod::Options);
    }
}

#[cfg(test)]
mod icap_serializer_tests {
    use super::*;

    #[test]
    fn test_serialize_options_request() {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("user-agent", "test-client".parse().unwrap());

        let request = IcapRequest {
            method: IcapMethod::Options,
            uri: "icap://example.com/options".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };

        let serialized = IcapSerializer::serialize_request(&request).unwrap();
        let serialized_str = String::from_utf8_lossy(&serialized);
        
        assert!(serialized_str.contains("OPTIONS icap://example.com/options ICAP/1.0"));
        assert!(serialized_str.contains("Host: example.com"));
        assert!(serialized_str.contains("User-Agent: test-client"));
    }

    #[test]
    fn test_serialize_icap_response() {
        let mut headers = HeaderMap::new();
        headers.insert("istag", "\"test-1.0\"".parse().unwrap());
        headers.insert("methods", "REQMOD, RESPMOD".parse().unwrap());

        let response = IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };

        let serialized = IcapSerializer::serialize_response(&response).unwrap();
        let serialized_str = String::from_utf8_lossy(&serialized);
        
        assert!(serialized_str.contains("ICAP/1.0 200 OK"));
        assert!(serialized_str.contains("ISTag: \"test-1.0\""));
        assert!(serialized_str.contains("Methods: REQMOD, RESPMOD"));
    }

    #[test]
    fn test_serialize_request_with_body() {
        let request = IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::from("test body content"),
            encapsulated: None,
        };

        let serialized = IcapSerializer::serialize_request(&request).unwrap();
        let serialized_str = String::from_utf8_lossy(&serialized);
        
        assert!(serialized_str.contains("REQMOD icap://example.com/reqmod ICAP/1.0"));
        assert!(serialized_str.contains("test body content"));
    }
}

#[cfg(test)]
mod error_response_tests {
    use super::*;

    #[test]
    fn test_bad_request_response() {
        let response = ErrorResponseBuilder::bad_request("Invalid request format");
        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        assert!(response.body.to_vec().contains(b"Bad Request"));
        assert!(response.body.to_vec().contains(b"Invalid request format"));
    }

    #[test]
    fn test_not_found_response() {
        let response = ErrorResponseBuilder::not_found("Service not found");
        assert_eq!(response.status, StatusCode::NOT_FOUND);
        assert!(response.body.to_vec().contains(b"Not Found"));
        assert!(response.body.to_vec().contains(b"Service not found"));
    }

    #[test]
    fn test_internal_server_error_response() {
        let response = ErrorResponseBuilder::internal_server_error("Database connection failed");
        assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(response.body.to_vec().contains(b"Internal Server Error"));
        assert!(response.body.to_vec().contains(b"Database connection failed"));
    }

    #[test]
    fn test_continue_response() {
        let response = ErrorResponseBuilder::continue_response();
        assert_eq!(response.status, StatusCode::CONTINUE);
        assert!(response.headers.contains_key("istag"));
    }

    #[test]
    fn test_no_content_response() {
        let response = ErrorResponseBuilder::no_content();
        assert_eq!(response.status, StatusCode::NO_CONTENT);
        assert!(response.headers.contains_key("istag"));
    }
}

#[cfg(test)]
mod roundtrip_tests {
    use super::*;

    #[test]
    fn test_request_serialization_roundtrip() {
        let original_data = b"REQMOD icap://example.com/reqmod ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\n\r\n";
        let request = IcapParser::parse_request(original_data).unwrap();
        let serialized = IcapSerializer::serialize_request(&request).unwrap();
        
        // The serialized data should contain the same information
        assert!(serialized.contains(b"REQMOD"));
        assert!(serialized.contains(b"icap://example.com/reqmod"));
        assert!(serialized.contains(b"ICAP/1.0"));
        assert!(serialized.contains(b"Host: example.com"));
    }

    #[test]
    fn test_response_serialization_roundtrip() {
        let original_data = b"ICAP/1.0 200 OK\r\nISTag: \"test-1.0\"\r\nMethods: REQMOD, RESPMOD\r\n\r\n";
        let response = IcapParser::parse_response(original_data).unwrap();
        let serialized = IcapSerializer::serialize_response(&response).unwrap();
        
        // The serialized data should contain the same information
        assert!(serialized.contains(b"ICAP/1.0"));
        assert!(serialized.contains(b"200"));
        assert!(serialized.contains(b"OK"));
        assert!(serialized.contains(b"ISTag: \"test-1.0\""));
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_parsing_performance() {
        let data = b"REQMOD icap://example.com/reqmod ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\nContent-Length: 1000\r\n\r\n";
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _request = IcapParser::parse_request(data).unwrap();
        }
        let duration = start.elapsed();
        
        // Should parse 1000 requests in less than 100ms
        assert!(duration.as_millis() < 100, "Parsing took {}ms, expected < 100ms", duration.as_millis());
    }

    #[test]
    fn test_serialization_performance() {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("user-agent", "test-client".parse().unwrap());

        let request = IcapRequest {
            method: IcapMethod::Options,
            uri: "icap://example.com/options".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _serialized = IcapSerializer::serialize_request(&request).unwrap();
        }
        let duration = start.elapsed();
        
        // Should serialize 1000 requests in less than 100ms
        assert!(duration.as_millis() < 100, "Serialization took {}ms, expected < 100ms", duration.as_millis());
    }
}
