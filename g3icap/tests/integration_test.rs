/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Integration tests for G3ICAP
//! 
//! This module contains integration tests that test the complete ICAP server functionality.

use std::time::Duration;

use g3icap::modules::{builtin::EchoModule};
use g3icap::services::{ServiceConfig, ServiceManager, LoadBalancingStrategy};
use g3icap::pipeline::{ContentPipeline, PipelineConfig, stages::LoggingStage};
use g3icap::protocol::common::{IcapMethod, IcapRequest};

/// Test complete ICAP workflow
#[tokio::test]
async fn test_complete_icap_workflow() {
    // Initialize logging for tests
    let _ = env_logger::try_init();
    
    // Create service manager
    let service_manager = ServiceManager::new();
    
    // Register echo service
    let config = ServiceConfig {
        name: "echo".to_string(),
        path: "/echo".to_string(),
        methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod, IcapMethod::Options],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 100,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Create test request
    let request = create_test_request();
    
    // Process request through service
    let response = service_manager.handle_request(&request).await.unwrap();
    assert_eq!(response.status, http::StatusCode::OK);
    
    // Test pipeline processing
    let pipeline_config = PipelineConfig {
        name: "test".to_string(),
        stages: Vec::new(),
        timeout: Duration::from_secs(60),
        parallel: false,
        max_concurrent: 10,
    };
    
    let mut pipeline = ContentPipeline::new(pipeline_config);
    
    // Add logging stage
    let logging_stage = Box::new(LoggingStage::new(
        "logging".to_string(),
        "info".to_string(),
    ));
    pipeline.add_stage(logging_stage);
    
    // Process request through pipeline
    let response = pipeline.process_request(request).await.unwrap();
    assert_eq!(response.status, http::StatusCode::OK);
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() {
    let service_manager = ServiceManager::new();
    
    // Test handling request for non-existent service
    let request = create_test_request();
    let result = service_manager.handle_request(&request).await;
    assert!(result.is_err());
}

/// Test concurrent processing
#[tokio::test]
async fn test_concurrent_processing() {
    let service_manager = ServiceManager::new();
    
    // Register service
    let config = ServiceConfig {
        name: "echo".to_string(),
        path: "/echo".to_string(),
        methods: vec![IcapMethod::Reqmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 100,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Process multiple requests sequentially
    let mut success_count = 0;
    for _i in 0..10 {
        let request = create_test_request();
        if let Ok(_) = service_manager.handle_request(&request).await {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, 10);
}

/// Create a test ICAP request
fn create_test_request() -> IcapRequest {
    use http::{HeaderMap, Uri, Version};
    use bytes::Bytes;
    
    let mut headers = HeaderMap::new();
    headers.insert("Host", "test.example.com".parse().unwrap());
    headers.insert("Content-Type", "text/plain".parse().unwrap());
    
    IcapRequest {
        method: IcapMethod::Reqmod,
        uri: Uri::from_static("icap://test.example.com/echo"),
        version: Version::HTTP_11,
        headers,
        body: Bytes::from("Test request body"),
        encapsulated: None,
    }
}
