/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use g3icap::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use g3icap::stats::IcapStats;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Uri, Version};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;

#[cfg(test)]
mod connection_handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_creation() {
        let stats = Arc::new(IcapStats::new());
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        
        // Create a mock TCP stream (this would normally be from TcpListener::accept)
        let (client_stream, _server_stream) = tokio::io::duplex(1024);
        let peer_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        
        let connection = g3icap::server::connection::IcapConnection::new(
            client_stream,
            peer_addr,
            stats,
            logger,
        );
        
        assert_eq!(connection.peer_addr, peer_addr);
    }

    #[test]
    fn test_is_complete_request() {
        let stats = Arc::new(IcapStats::new());
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let peer_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        
        // Create a mock connection (we can't easily test the actual connection without a real stream)
        let connection = g3icap::server::connection::IcapConnection {
            stream: unsafe { std::mem::zeroed() }, // This is unsafe but needed for testing
            peer_addr,
            stats,
            logger,
        };
        
        // Test complete request detection
        let complete_request = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\n\r\n";
        assert!(connection.is_complete_request(complete_request));
        
        let incomplete_request = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\n";
        assert!(!connection.is_complete_request(incomplete_request));
        
        let empty_request = b"";
        assert!(!connection.is_complete_request(empty_request));
    }
}

#[cfg(test)]
mod request_processing_tests {
    use super::*;

    fn create_test_request() -> IcapRequest {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("user-agent", "test-client".parse().unwrap());

        IcapRequest {
            method: IcapMethod::Options,
            uri: "icap://example.com/options".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    fn create_test_reqmod_request() -> IcapRequest {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("encapsulated", "req-hdr=0, req-body=200".parse().unwrap());

        IcapRequest {
            method: IcapMethod::Reqmod,
            uri: "icap://example.com/reqmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::from("GET /test HTTP/1.1\r\nHost: example.com\r\n\r\n"),
            encapsulated: None,
        }
    }

    fn create_test_respmod_request() -> IcapRequest {
        let mut headers = HeaderMap::new();
        headers.insert("host", "example.com".parse().unwrap());
        headers.insert("encapsulated", "req-hdr=0, res-hdr=100, res-body=300".parse().unwrap());

        IcapRequest {
            method: IcapMethod::Respmod,
            uri: "icap://example.com/respmod".parse().unwrap(),
            version: Version::HTTP_11,
            headers,
            body: Bytes::from("GET /test HTTP/1.1\r\nHost: example.com\r\n\r\nHTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body>Test</body></html>"),
            encapsulated: None,
        }
    }

    #[tokio::test]
    async fn test_handle_options_request() {
        let stats = Arc::new(IcapStats::new());
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let peer_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        
        let connection = g3icap::server::connection::IcapConnection {
            stream: unsafe { std::mem::zeroed() },
            peer_addr,
            stats,
            logger,
        };
        
        let request = create_test_request();
        let response = connection.handle_options_request(request).await.unwrap();
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.version, Version::HTTP_11);
        assert!(response.headers.contains_key("istag"));
        assert!(response.headers.contains_key("methods"));
        assert!(response.headers.contains_key("service"));
    }

    #[tokio::test]
    async fn test_handle_reqmod_request() {
        let stats = Arc::new(IcapStats::new());
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let peer_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        
        let connection = g3icap::server::connection::IcapConnection {
            stream: unsafe { std::mem::zeroed() },
            peer_addr,
            stats,
            logger,
        };
        
        let request = create_test_reqmod_request();
        let response = connection.handle_reqmod_request(request).await.unwrap();
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.version, Version::HTTP_11);
        // REQMOD should pass through the encapsulated data
        assert!(response.encapsulated.is_some());
    }

    #[tokio::test]
    async fn test_handle_respmod_request() {
        let stats = Arc::new(IcapStats::new());
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let peer_addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        
        let connection = g3icap::server::connection::IcapConnection {
            stream: unsafe { std::mem::zeroed() },
            peer_addr,
            stats,
            logger,
        };
        
        let request = create_test_respmod_request();
        let response = connection.handle_respmod_request(request).await.unwrap();
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.version, Version::HTTP_11);
        // RESPMOD should pass through the encapsulated data
        assert!(response.encapsulated.is_some());
    }
}

#[cfg(test)]
mod statistics_tests {
    use super::*;

    #[test]
    fn test_stats_initialization() {
        let stats = IcapStats::new();
        
        assert_eq!(stats.get_total_connections(), 0);
        assert_eq!(stats.get_active_connections(), 0);
        assert_eq!(stats.get_connection_errors(), 0);
        assert_eq!(stats.get_total_requests(), 0);
        assert_eq!(stats.get_options_requests(), 0);
        assert_eq!(stats.get_reqmod_requests(), 0);
        assert_eq!(stats.get_respmod_requests(), 0);
        assert_eq!(stats.get_successful_responses(), 0);
        assert_eq!(stats.get_error_responses(), 0);
    }

    #[test]
    fn test_stats_increment_connections() {
        let stats = IcapStats::new();
        
        stats.add_connection();
        assert_eq!(stats.get_total_connections(), 1);
        assert_eq!(stats.get_active_connections(), 1);
        
        stats.add_connection();
        assert_eq!(stats.get_total_connections(), 2);
        assert_eq!(stats.get_active_connections(), 2);
        
        stats.remove_connection();
        assert_eq!(stats.get_total_connections(), 2);
        assert_eq!(stats.get_active_connections(), 1);
    }

    #[test]
    fn test_stats_increment_requests() {
        let stats = IcapStats::new();
        
        stats.increment_requests();
        assert_eq!(stats.get_total_requests(), 1);
        
        stats.increment_options_requests();
        assert_eq!(stats.get_options_requests(), 1);
        
        stats.increment_reqmod_requests();
        assert_eq!(stats.get_reqmod_requests(), 1);
        
        stats.increment_respmod_requests();
        assert_eq!(stats.get_respmod_requests(), 1);
    }

    #[test]
    fn test_stats_increment_responses() {
        let stats = IcapStats::new();
        
        stats.increment_successful_responses();
        assert_eq!(stats.get_successful_responses(), 1);
        
        stats.increment_error_responses();
        assert_eq!(stats.get_error_responses(), 1);
    }

    #[test]
    fn test_stats_connection_errors() {
        let stats = IcapStats::new();
        
        stats.add_connection_error();
        assert_eq!(stats.get_connection_errors(), 1);
        
        stats.add_connection_error();
        assert_eq!(stats.get_connection_errors(), 2);
    }

    #[test]
    fn test_stats_concurrent_access() {
        use std::sync::Arc;
        use std::thread;
        
        let stats = Arc::new(IcapStats::new());
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent access
        for i in 0..10 {
            let stats_clone = Arc::clone(&stats);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    stats_clone.add_connection();
                    stats_clone.increment_requests();
                    stats_clone.increment_successful_responses();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify final counts
        assert_eq!(stats.get_total_connections(), 1000);
        assert_eq!(stats.get_active_connections(), 1000);
        assert_eq!(stats.get_total_requests(), 1000);
        assert_eq!(stats.get_successful_responses(), 1000);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_uri_handling() {
        let data = b"OPTIONS invalid-uri ICAP/1.0\r\nHost: example.com\r\n\r\n";
        let result = g3icap::protocol::common::IcapParser::parse_request(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_request_handling() {
        let data = b"INVALID REQUEST LINE\r\n\r\n";
        let result = g3icap::protocol::common::IcapParser::parse_request(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_request_handling() {
        let data = b"";
        let result = g3icap::protocol::common::IcapParser::parse_request(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_request_handling() {
        let data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\n";
        let result = g3icap::protocol::common::IcapParser::parse_request(data);
        // This should still parse successfully as it has the required parts
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_stats_performance() {
        let stats = IcapStats::new();
        
        let start = Instant::now();
        for _ in 0..100000 {
            stats.increment_requests();
            stats.increment_successful_responses();
        }
        let duration = start.elapsed();
        
        // Should handle 100k operations in less than 10ms
        assert!(duration.as_millis() < 10, "Stats operations took {}ms, expected < 10ms", duration.as_millis());
    }

    #[test]
    fn test_concurrent_stats_performance() {
        use std::sync::Arc;
        use std::thread;
        
        let stats = Arc::new(IcapStats::new());
        let mut handles = vec![];
        
        let start = Instant::now();
        
        // Spawn 10 threads, each doing 10k operations
        for _ in 0..10 {
            let stats_clone = Arc::clone(&stats);
            let handle = thread::spawn(move || {
                for _ in 0..10000 {
                    stats_clone.increment_requests();
                    stats_clone.increment_successful_responses();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        
        // Should handle 100k concurrent operations in less than 50ms
        assert!(duration.as_millis() < 50, "Concurrent stats operations took {}ms, expected < 50ms", duration.as_millis());
        
        assert_eq!(stats.get_total_requests(), 100000);
        assert_eq!(stats.get_successful_responses(), 100000);
    }
}
