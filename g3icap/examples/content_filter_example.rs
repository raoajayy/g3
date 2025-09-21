/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Content Filter Module Example
//! 
//! This example demonstrates how to use the G3ICAP Content Filter Module
//! for various content filtering scenarios.

use std::time::Duration;
use http::{HeaderMap, Uri, Version};
use bytes::Bytes;

use g3icap::modules::content_filter::{ContentFilterModule, ContentFilterConfig, BlockingAction};
use g3icap::modules::{IcapModule, ModuleConfig};
use g3icap::protocol::common::{IcapMethod, IcapRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("G3ICAP Content Filter Module Example");
    println!("=====================================");

    // Example 1: Basic Content Filtering
    example_basic_filtering().await?;
    
    // Example 2: Advanced Pattern Matching
    example_advanced_patterns().await?;
    
    // Example 3: Different Blocking Actions
    example_blocking_actions().await?;
    
    // Example 4: Performance Testing
    example_performance_testing().await?;
    
    // Example 5: Statistics and Monitoring
    example_statistics_monitoring().await?;

    Ok(())
}

/// Example 1: Basic Content Filtering
async fn example_basic_filtering() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Basic Content Filtering");
    println!("--------------------------");

    // Create basic configuration
    let config = ContentFilterConfig {
        blocked_domains: vec![
            "malware.com".to_string(),
            "phishing-site.org".to_string(),
        ],
        blocked_keywords: vec![
            "malware".to_string(),
            "virus".to_string(),
            "trojan".to_string(),
        ],
        blocked_mime_types: vec![
            "application/octet-stream".to_string(),
        ],
        max_file_size: Some(10 * 1024 * 1024), // 10MB
        case_insensitive: true,
        enable_regex: false,
        blocking_action: BlockingAction::Forbidden,
        enable_logging: true,
        enable_metrics: true,
        ..Default::default()
    };

    // Create and initialize module
    let mut module = ContentFilterModule::new(config);
    let module_config = create_module_config("basic_filter");
    module.init(&module_config).await?;

    // Test cases
    let test_cases = vec![
        ("http://example.com/clean", "clean content", false),
        ("http://malware.com/virus", "clean content", true), // Blocked by domain
        ("http://example.com/malware", "clean content", true), // Blocked by keyword
        ("http://example.com/clean", "malware content", true), // Blocked by body keyword
    ];

    for (url, body, should_block) in test_cases {
        let request = create_test_request(url, body);
        let response = module.handle_reqmod(&request).await?;
        
        let blocked = response.status == http::StatusCode::FORBIDDEN;
        let status = if blocked == should_block { "✓" } else { "✗" };
        
        println!("  {} URL: {} | Body: {} | Blocked: {} | Expected: {}", 
            status, url, body, blocked, should_block);
    }

    Ok(())
}

/// Example 2: Advanced Pattern Matching
async fn example_advanced_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Advanced Pattern Matching");
    println!("-----------------------------");

    // Create advanced configuration with regex patterns
    let config = ContentFilterConfig {
        blocked_domain_patterns: vec![
            r".*\.malware\..*".to_string(),
            r".*phishing.*".to_string(),
            r".*\.(tk|ml|ga|cf)$".to_string(), // Suspicious TLDs
        ],
        blocked_keyword_patterns: vec![
            r"malware.*virus".to_string(),
            r".*phishing.*scam.*".to_string(),
            r"bitcoin.*wallet".to_string(),
        ],
        enable_regex: true,
        case_insensitive: true,
        blocking_action: BlockingAction::Forbidden,
        enable_logging: true,
        enable_metrics: true,
        ..Default::default()
    };

    // Create and initialize module
    let mut module = ContentFilterModule::new(config);
    let module_config = create_module_config("advanced_filter");
    module.init(&module_config).await?;

    // Test cases
    let test_cases = vec![
        ("http://example.com/clean", "clean content", false),
        ("http://test.malware.com/path", "clean content", true), // Blocked by domain pattern
        ("http://phishing-site.org/path", "clean content", true), // Blocked by domain pattern
        ("http://suspicious.tk/path", "clean content", true), // Blocked by TLD pattern
        ("http://example.com/clean", "malware and virus content", true), // Blocked by keyword pattern
        ("http://example.com/clean", "phishing scam content", true), // Blocked by keyword pattern
    ];

    for (url, body, should_block) in test_cases {
        let request = create_test_request(url, body);
        let response = module.handle_reqmod(&request).await?;
        
        let blocked = response.status == http::StatusCode::FORBIDDEN;
        let status = if blocked == should_block { "✓" } else { "✗" };
        
        println!("  {} URL: {} | Body: {} | Blocked: {} | Expected: {}", 
            status, url, body, blocked, should_block);
    }

    Ok(())
}

/// Example 3: Different Blocking Actions
async fn example_blocking_actions() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Different Blocking Actions");
    println!("-----------------------------");

    let blocking_actions = vec![
        ("Forbidden", BlockingAction::Forbidden),
        ("NotFound", BlockingAction::NotFound),
        ("Custom 451", BlockingAction::Custom(451)),
        ("Redirect", BlockingAction::Redirect("https://company.com/blocked".to_string())),
        ("Replace", BlockingAction::Replace("Content blocked by policy".to_string())),
    ];

    for (action_name, action) in blocking_actions {
        let config = ContentFilterConfig {
            blocked_keywords: vec!["malware".to_string()],
            blocking_action: action,
            enable_logging: false,
            enable_metrics: false,
            ..Default::default()
        };

        let mut module = ContentFilterModule::new(config);
        let module_config = create_module_config("blocking_action_test");
        module.init(&module_config).await?;

        let request = create_test_request("http://example.com/malware", "clean content");
        let response = module.handle_reqmod(&request).await?;

        println!("  {} Action: {} | Status: {} | Body: {}", 
            "✓", action_name, response.status, 
            String::from_utf8_lossy(&response.body));
    }

    Ok(())
}

/// Example 4: Performance Testing
async fn example_performance_testing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. Performance Testing");
    println!("----------------------");

    // Create configuration with many patterns
    let config = ContentFilterConfig {
        blocked_keywords: (1..=100).map(|i| format!("keyword{}", i)).collect(),
        blocked_domains: (1..=50).map(|i| format!("malware{}.com", i)).collect(),
        enable_regex: false,
        case_insensitive: true,
        blocking_action: BlockingAction::Forbidden,
        enable_logging: false,
        enable_metrics: true,
        ..Default::default()
    };

    let mut module = ContentFilterModule::new(config);
    let module_config = create_module_config("performance_test");
    module.init(&module_config).await?;

    // Performance test
    let num_requests = 1000;
    let start_time = std::time::Instant::now();

    for i in 0..num_requests {
        let url = if i % 10 == 0 {
            "http://malware5.com/path" // This will be blocked
        } else {
            "http://example.com/clean"
        };
        
        let body = if i % 20 == 0 {
            "keyword10 content" // This will be blocked
        } else {
            "clean content"
        };

        let request = create_test_request(url, body);
        let _response = module.handle_reqmod(&request).await?;
    }

    let duration = start_time.elapsed();
    let requests_per_second = num_requests as f64 / duration.as_secs_f64();

    println!("  Processed {} requests in {:?}", num_requests, duration);
    println!("  Performance: {:.2} requests/second", requests_per_second);

    // Show statistics
    let stats = module.get_stats();
    println!("  Total requests: {}", stats.total_requests);
    println!("  Blocked requests: {}", stats.blocked_requests);
    println!("  Allowed requests: {}", stats.allowed_requests);
    println!("  Average processing time: {:.2}μs", 
        stats.total_processing_time as f64 / stats.total_requests as f64);

    Ok(())
}

/// Example 5: Statistics and Monitoring
async fn example_statistics_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. Statistics and Monitoring");
    println!("----------------------------");

    let config = ContentFilterConfig {
        blocked_keywords: vec!["malware".to_string(), "virus".to_string()],
        blocked_domains: vec!["malware.com".to_string()],
        blocked_mime_types: vec!["application/octet-stream".to_string()],
        max_file_size: Some(1024), // 1KB
        enable_logging: true,
        enable_metrics: true,
        ..Default::default()
    };

    let mut module = ContentFilterModule::new(config);
    let module_config = create_module_config("monitoring_test");
    module.init(&module_config).await?;

    // Process various requests
    let large_content = "x".repeat(2000);
    let test_requests = vec![
        ("http://example.com/clean", "clean content", "text/html"),
        ("http://malware.com/path", "clean content", "text/html"),
        ("http://example.com/malware", "clean content", "text/html"),
        ("http://example.com/clean", "virus content", "text/html"),
        ("http://example.com/large", &large_content, "application/octet-stream"),
    ];

    for (url, body, content_type) in test_requests {
        let mut request = create_test_request(url, body);
        request.headers.insert("content-type", content_type.parse().unwrap());
        
        let response = module.handle_reqmod(&request).await?;
        let blocked = response.status == http::StatusCode::FORBIDDEN;
        
        println!("  URL: {} | Content-Type: {} | Blocked: {}", 
            url, content_type, blocked);
    }

    // Show detailed statistics
    let stats = module.get_stats();
    println!("\n  Detailed Statistics:");
    println!("    Total requests: {}", stats.total_requests);
    println!("    Blocked requests: {}", stats.blocked_requests);
    println!("    Allowed requests: {}", stats.allowed_requests);
    println!("    Blocked by domain: {}", stats.blocked_by_domain);
    println!("    Blocked by keyword: {}", stats.blocked_by_keyword);
    println!("    Blocked by MIME type: {}", stats.blocked_by_mime_type);
    println!("    Blocked by file size: {}", stats.blocked_by_file_size);
    println!("    Total processing time: {}μs", stats.total_processing_time);
    println!("    Average processing time: {:.2}μs", 
        stats.total_processing_time as f64 / stats.total_requests as f64);

    // Show module metrics
    let metrics = module.get_metrics();
    println!("\n  Module Metrics:");
    println!("    Requests per second: {:.2}", metrics.requests_per_second);
    println!("    Average processing time: {:.2}ms", metrics.average_response_time.as_millis());
    println!("    Error rate: {:.2}%", metrics.error_rate);

    // Test health check
    println!("\n  Health Check:");
    println!("    Module healthy: {}", module.is_healthy());

    Ok(())
}

/// Helper function to create test requests
fn create_test_request(url: &str, body: &str) -> IcapRequest {
    let mut headers = HeaderMap::new();
    headers.insert("host", "example.com".parse().unwrap());
    headers.insert("content-type", "text/html".parse().unwrap());

    IcapRequest {
        method: IcapMethod::Reqmod,
        uri: url.parse::<Uri>().unwrap(),
        version: Version::HTTP_11,
        headers,
        body: Bytes::from(body.to_string()),
        encapsulated: None,
    }
}

/// Helper function to create module configuration
fn create_module_config(name: &str) -> ModuleConfig {
    ModuleConfig {
        name: name.to_string(),
        path: std::path::PathBuf::from(""),
        version: "1.0.0".to_string(),
        config: serde_json::Value::Object(serde_json::Map::new()),
        dependencies: Vec::new(),
        load_timeout: Duration::from_secs(5),
        max_memory: 1024 * 1024,
        sandbox: true,
    }
}

/// Helper function to create default configuration
fn create_default_config() -> ContentFilterConfig {
    ContentFilterConfig {
        blocked_domains: Vec::new(),
        blocked_domain_patterns: Vec::new(),
        blocked_keywords: Vec::new(),
        blocked_keyword_patterns: Vec::new(),
        blocked_mime_types: Vec::new(),
        blocked_extensions: Vec::new(),
        max_file_size: Some(10 * 1024 * 1024), // 10MB
        regex_cache_size: 1000,
        case_insensitive: true,
        enable_regex: true,
        blocking_action: BlockingAction::Forbidden,
        custom_message: None,
        enable_logging: true,
        enable_metrics: true,
    }
}
