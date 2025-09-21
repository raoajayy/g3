/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Test validation script for G3ICAP
//! This script validates that all test modules can be compiled and basic functionality works

use std::time::Instant;

fn main() {
    println!("G3ICAP Test Validation");
    println!("=====================");
    
    let start_time = Instant::now();
    
    // Test 1: Basic ICAP method parsing
    println!("Test 1: ICAP Method Parsing");
    test_icap_methods();
    
    // Test 2: Basic request parsing
    println!("Test 2: ICAP Request Parsing");
    test_request_parsing();
    
    // Test 3: Basic response parsing
    println!("Test 3: ICAP Response Parsing");
    test_response_parsing();
    
    // Test 4: Statistics functionality
    println!("Test 4: Statistics Functionality");
    test_statistics();
    
    // Test 5: Error handling
    println!("Test 5: Error Handling");
    test_error_handling();
    
    let duration = start_time.elapsed();
    println!("\nAll validation tests completed in {:?}", duration);
    println!("✅ G3ICAP test validation successful!");
}

fn test_icap_methods() {
    use g3icap::protocol::common::IcapMethod;
    
    // Test method parsing
    assert_eq!(IcapMethod::from("REQMOD"), IcapMethod::Reqmod);
    assert_eq!(IcapMethod::from("RESPMOD"), IcapMethod::Respmod);
    assert_eq!(IcapMethod::from("OPTIONS"), IcapMethod::Options);
    
    // Test method to string
    assert_eq!(IcapMethod::Reqmod.to_string(), "REQMOD");
    assert_eq!(IcapMethod::Respmod.to_string(), "RESPMOD");
    assert_eq!(IcapMethod::Options.to_string(), "OPTIONS");
    
    println!("  ✓ Method parsing and conversion works");
}

fn test_request_parsing() {
    use g3icap::protocol::common::IcapParser;
    
    let data = b"OPTIONS icap://example.com/options ICAP/1.0\r\nHost: example.com\r\n\r\n";
    let request = IcapParser::parse_request(data).unwrap();
    
    assert_eq!(request.method.to_string(), "OPTIONS");
    assert_eq!(request.uri.to_string(), "icap://example.com/options");
    assert!(request.headers.contains_key("host"));
    
    println!("  ✓ Request parsing works");
}

fn test_response_parsing() {
    use g3icap::protocol::common::IcapParser;
    
    let data = b"ICAP/1.0 200 OK\r\nISTag: \"test-1.0\"\r\n\r\n";
    let response = IcapParser::parse_response(data).unwrap();
    
    assert_eq!(response.status.as_u16(), 200);
    assert!(response.headers.contains_key("istag"));
    
    println!("  ✓ Response parsing works");
}

fn test_statistics() {
    use g3icap::stats::IcapStats;
    
    let stats = IcapStats::new();
    
    // Test initial state
    assert_eq!(stats.get_total_connections(), 0);
    assert_eq!(stats.get_total_requests(), 0);
    
    // Test incrementing
    stats.add_connection();
    stats.increment_requests();
    stats.increment_successful_responses();
    
    assert_eq!(stats.get_total_connections(), 1);
    assert_eq!(stats.get_active_connections(), 1);
    assert_eq!(stats.get_total_requests(), 1);
    assert_eq!(stats.get_successful_responses(), 1);
    
    // Test decrementing
    stats.remove_connection();
    assert_eq!(stats.get_active_connections(), 0);
    
    println!("  ✓ Statistics functionality works");
}

fn test_error_handling() {
    use g3icap::protocol::error::ErrorResponseBuilder;
    use g3icap::protocol::common::IcapParser;
    
    // Test error response creation
    let error_response = ErrorResponseBuilder::bad_request("Test error");
    assert_eq!(error_response.status.as_u16(), 400);
    assert!(error_response.body.to_vec().contains(b"Test error"));
    
    // Test parsing error handling
    let invalid_data = b"INVALID REQUEST\r\n\r\n";
    let result = IcapParser::parse_request(invalid_data);
    assert!(result.is_err());
    
    println!("  ✓ Error handling works");
}
