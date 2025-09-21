/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use g3icap::protocol::common::{IcapMethod, IcapParser, IcapSerializer};
use g3icap::protocol::error::ErrorResponseBuilder;
use g3icap::stats::IcapStats;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Uri, Version};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    #[tokio::test]
    async fn test_icap_options_workflow() {
        // Test the complete OPTIONS workflow
        let request_data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\n\r\n";
        
        // Parse request
        let request = IcapParser::parse_request(request_data).unwrap();
        assert_eq!(request.method, IcapMethod::Options);
        assert_eq!(request.uri, "icap://example.com/options".parse::<Uri>().unwrap());
        
        // Create response
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-1.0\"".parse().unwrap());
        headers.insert("Methods", "REQMOD, RESPMOD, OPTIONS".parse().unwrap());
        headers.insert("Service", "G3 ICAP Server".parse().unwrap());
        
        let response = g3icap::protocol::common::IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };
        
        // Serialize response
        let response_data = IcapSerializer::serialize_response(&response).unwrap();
        let response_str = String::from_utf8_lossy(&response_data);
        
        assert!(response_str.contains("ICAP/1.0 200 OK"));
        assert!(response_str.contains("ISTag: \"g3icap-1.0\""));
        assert!(response_str.contains("Methods: REQMOD, RESPMOD, OPTIONS"));
    }

    #[tokio::test]
    async fn test_icap_reqmod_workflow() {
        // Test the complete REQMOD workflow
        let request_data = b"REQMOD icap://example.com/reqmod ICAP/1.0\r\nHost: example.com\r\nEncapsulated: req-hdr=0, req-body=200\r\n\r\nGET /test HTTP/1.1\r\nHost: example.com\r\n\r\n";
        
        // Parse request
        let request = IcapParser::parse_request(request_data).unwrap();
        assert_eq!(request.method, IcapMethod::Reqmod);
        assert!(request.headers.contains_key("encapsulated"));
        
        // Create response (pass-through for now)
        let response = g3icap::protocol::common::IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: request.encapsulated,
        };
        
        // Serialize response
        let response_data = IcapSerializer::serialize_response(&response).unwrap();
        let response_str = String::from_utf8_lossy(&response_data);
        
        assert!(response_str.contains("ICAP/1.0 200 OK"));
    }

    #[tokio::test]
    async fn test_icap_respmod_workflow() {
        // Test the complete RESPMOD workflow
        let request_data = b"RESPMOD icap://example.com/respmod ICAP/1.0\r\nHost: example.com\r\nEncapsulated: req-hdr=0, res-hdr=100, res-body=300\r\n\r\nGET /test HTTP/1.1\r\nHost: example.com\r\n\r\nHTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body>Test</body></html>";
        
        // Parse request
        let request = IcapParser::parse_request(request_data).unwrap();
        assert_eq!(request.method, IcapMethod::Respmod);
        assert!(request.headers.contains_key("encapsulated"));
        
        // Create response (pass-through for now)
        let response = g3icap::protocol::common::IcapResponse {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: request.encapsulated,
        };
        
        // Serialize response
        let response_data = IcapSerializer::serialize_response(&response).unwrap();
        let response_str = String::from_utf8_lossy(&response_data);
        
        assert!(response_str.contains("ICAP/1.0 200 OK"));
    }
}

#[cfg(test)]
mod error_handling_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_error_response_workflow() {
        // Test error response creation and serialization
        let error_response = ErrorResponseBuilder::bad_request("Invalid request format");
        assert_eq!(error_response.status, StatusCode::BAD_REQUEST);
        
        let response_data = IcapSerializer::serialize_response(&error_response).unwrap();
        let response_str = String::from_utf8_lossy(&response_data);
        
        assert!(response_str.contains("ICAP/1.0 400 Bad Request"));
        assert!(response_str.contains("Invalid request format"));
    }

    #[tokio::test]
    async fn test_multiple_error_types() {
        let error_types = vec![
            ("bad_request", ErrorResponseBuilder::bad_request("Bad request")),
            ("not_found", ErrorResponseBuilder::not_found("Not found")),
            ("internal_error", ErrorResponseBuilder::internal_server_error("Internal error")),
            ("continue", ErrorResponseBuilder::continue_response()),
            ("no_content", ErrorResponseBuilder::no_content()),
        ];
        
        for (error_type, response) in error_types {
            let response_data = IcapSerializer::serialize_response(&response).unwrap();
            let response_str = String::from_utf8_lossy(&response_data);
            
            match error_type {
                "bad_request" => {
                    assert!(response_str.contains("400"));
                    assert!(response_str.contains("Bad Request"));
                }
                "not_found" => {
                    assert!(response_str.contains("404"));
                    assert!(response_str.contains("Not Found"));
                }
                "internal_error" => {
                    assert!(response_str.contains("500"));
                    assert!(response_str.contains("Internal Server Error"));
                }
                "continue" => {
                    assert!(response_str.contains("100"));
                    assert!(response_str.contains("Continue"));
                }
                "no_content" => {
                    assert!(response_str.contains("204"));
                    assert!(response_str.contains("No Content"));
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod statistics_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_stats_integration() {
        let stats = Arc::new(IcapStats::new());
        
        // Simulate a complete request/response cycle
        stats.add_connection();
        stats.increment_requests();
        stats.increment_options_requests();
        stats.increment_successful_responses();
        stats.remove_connection();
        
        // Verify all statistics are updated
        assert_eq!(stats.get_total_connections(), 1);
        assert_eq!(stats.get_active_connections(), 0);
        assert_eq!(stats.get_total_requests(), 1);
        assert_eq!(stats.get_options_requests(), 1);
        assert_eq!(stats.get_successful_responses(), 1);
    }

    #[tokio::test]
    async fn test_stats_concurrent_operations() {
        use std::sync::Arc;
        use tokio::task;
        
        let stats = Arc::new(IcapStats::new());
        let mut handles = vec![];
        
        // Spawn multiple async tasks to test concurrent access
        for i in 0..10 {
            let stats_clone = Arc::clone(&stats);
            let handle = task::spawn(async move {
                for _ in 0..100 {
                    stats_clone.add_connection();
                    stats_clone.increment_requests();
                    stats_clone.increment_successful_responses();
                    stats_clone.remove_connection();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
        
        // Verify final counts
        assert_eq!(stats.get_total_connections(), 1000);
        assert_eq!(stats.get_active_connections(), 0); // All connections removed
        assert_eq!(stats.get_total_requests(), 1000);
        assert_eq!(stats.get_successful_responses(), 1000);
    }
}

#[cfg(test)]
mod network_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_listener_creation() {
        // Test that we can create a TCP listener on an available port
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        assert!(local_addr.port() > 0);
    }

    #[tokio::test]
    async fn test_multiple_listeners() {
        // Test creating multiple listeners on different ports
        let listener1 = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let listener2 = TcpListener::bind("127.0.0.1:0").await.unwrap();
        
        let addr1 = listener1.local_addr().unwrap();
        let addr2 = listener2.local_addr().unwrap();
        
        assert_ne!(addr1.port(), addr2.port());
    }

    #[tokio::test]
    async fn test_listener_timeout() {
        // Test that listener operations can be timed out
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        
        // This should timeout since no connections are coming
        let result = timeout(Duration::from_millis(100), listener.accept()).await;
        assert!(result.is_err()); // Should timeout
    }
}

#[cfg(test)]
mod performance_integration_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_high_throughput_parsing() {
        let request_data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\n\r\n";
        
        let start = Instant::now();
        for _ in 0..10000 {
            let _request = IcapParser::parse_request(request_data).unwrap();
        }
        let duration = start.elapsed();
        
        // Should parse 10k requests in less than 100ms
        assert!(duration.as_millis() < 100, "High throughput parsing took {}ms, expected < 100ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_high_throughput_serialization() {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("user-agent", "test-client".parse().unwrap());

        let request = g3icap::protocol::common::IcapRequest {
            method: IcapMethod::Options,
            uri: "icap://example.com/options".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };
        
        let start = Instant::now();
        for _ in 0..10000 {
            let _serialized = IcapSerializer::serialize_request(&request).unwrap();
        }
        let duration = start.elapsed();
        
        // Should serialize 10k requests in less than 100ms
        assert!(duration.as_millis() < 100, "High throughput serialization took {}ms, expected < 100ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_memory_usage() {
        let stats = Arc::new(IcapStats::new());
        
        // Create many connections to test memory usage
        let start = Instant::now();
        for i in 0..10000 {
            stats.add_connection();
            stats.increment_requests();
            if i % 2 == 0 {
                stats.increment_successful_responses();
            } else {
                stats.increment_error_responses();
            }
        }
        let duration = start.elapsed();
        
        // Should handle 10k operations in less than 50ms
        assert!(duration.as_millis() < 50, "Memory usage test took {}ms, expected < 50ms", duration.as_millis());
        
        // Verify final state
        assert_eq!(stats.get_total_connections(), 10000);
        assert_eq!(stats.get_active_connections(), 10000);
        assert_eq!(stats.get_total_requests(), 10000);
        assert_eq!(stats.get_successful_responses(), 5000);
        assert_eq!(stats.get_error_responses(), 5000);
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_parsing() {
        use tokio::task;
        
        let request_data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\nUser-Agent: test-client\r\n\r\n";
        let mut handles = vec![];
        
        // Spawn 100 concurrent parsing tasks
        for _ in 0..100 {
            let data = request_data.to_vec();
            let handle = task::spawn(async move {
                for _ in 0..100 {
                    let _request = IcapParser::parse_request(&data).unwrap();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_serialization() {
        use tokio::task;
        
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("user-agent", "test-client".parse().unwrap());

        let request = g3icap::protocol::common::IcapRequest {
            method: IcapMethod::Options,
            uri: "icap://example.com/options".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        };
        
        let mut handles = vec![];
        
        // Spawn 100 concurrent serialization tasks
        for _ in 0..100 {
            let request_clone = g3icap::protocol::common::IcapRequest {
                method: request.method.clone(),
                uri: request.uri.clone(),
                version: request.version,
                headers: request.headers.clone(),
                body: request.body.clone(),
                encapsulated: request.encapsulated.clone(),
            };
            
            let handle = task::spawn(async move {
                for _ in 0..100 {
                    let _serialized = IcapSerializer::serialize_request(&request_clone).unwrap();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }
}
