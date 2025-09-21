/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Tests for modular architecture components
//! 
//! This module contains comprehensive tests for the modular architecture,
//! including module system, service management, and content pipeline.

use std::time::Duration;
use tokio::time::sleep;

use g3icap::modules::{ModuleConfig, ModuleRegistry, builtin::{EchoModule, LoggingModule}};
use g3icap::services::{ServiceConfig, ServiceManager, LoadBalancingStrategy};
use g3icap::pipeline::{ContentPipeline, PipelineConfig, stages::{LoggingStage, ContentFilterStage, AntivirusStage}};
use g3icap::protocol::common::{IcapMethod, IcapRequest, IcapResponse};

/// Test module registry functionality
#[tokio::test]
async fn test_module_registry() {
    let config = ModuleConfig {
        name: "test".to_string(),
        path: std::path::PathBuf::from("/tmp"),
        version: "1.0.0".to_string(),
        config: serde_json::Value::Object(serde_json::Map::new()),
        dependencies: Vec::new(),
        load_timeout: Duration::from_secs(5),
        max_memory: 1024 * 1024,
        sandbox: true,
    };
    
    let registry = ModuleRegistry::new(config);
    
    // Test listing modules (should be empty initially)
    let modules = registry.list_modules();
    assert!(modules.is_empty());
    
    // Test getting non-existent module
    let module = registry.get_module("nonexistent");
    assert!(module.is_none());
}

/// Test service manager functionality
#[tokio::test]
async fn test_service_manager() {
    let service_manager = ServiceManager::new();
    
    // Test listing services (should be empty initially)
    let services = service_manager.list_services();
    assert!(services.is_empty());
    
    // Test registering a service
    let config = ServiceConfig {
        name: "test_echo".to_string(),
        path: "/test_echo".to_string(),
        methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 10,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Test listing services after registration
    let services = service_manager.list_services();
    assert_eq!(services.len(), 1);
    assert!(services.contains(&"test_echo".to_string()));
    
    // Test getting service metrics
    let metrics = service_manager.get_service_metrics("test_echo");
    assert!(metrics.is_some());
    
    // Test unregistering service
    service_manager.unregister_service("test_echo").await.unwrap();
    
    // Test listing services after unregistration
    let services = service_manager.list_services();
    assert!(services.is_empty());
}

/// Test service request handling
#[tokio::test]
async fn test_service_request_handling() {
    let service_manager = ServiceManager::new();
    
    // Register echo service
    let config = ServiceConfig {
        name: "echo".to_string(),
        path: "/echo".to_string(),
        methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod, IcapMethod::Options],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 10,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Create test request
    let request = create_test_request();
    
    // Test REQMOD request
    let response = service_manager.handle_request(&request).await.unwrap();
    assert_eq!(response.status, http::StatusCode::OK);
    assert_eq!(response.body, request.body);
    
    // Test OPTIONS request
    let mut options_request = request.clone();
    options_request.method = IcapMethod::Options;
    let response = service_manager.handle_request(&options_request).await.unwrap();
    assert_eq!(response.status, http::StatusCode::OK);
    assert!(response.headers.contains_key("ISTag"));
    assert!(response.headers.contains_key("Methods"));
}

/// Test content pipeline functionality
#[tokio::test]
async fn test_content_pipeline() {
    let config = PipelineConfig {
        name: "test".to_string(),
        stages: Vec::new(),
        timeout: Duration::from_secs(60),
        parallel: false,
        max_concurrent: 5,
    };
    
    let mut pipeline = ContentPipeline::new(config);
    
    // Add logging stage
    let logging_stage = Box::new(LoggingStage::new(
        "logging".to_string(),
        "info".to_string(),
    ));
    pipeline.add_stage(logging_stage);
    
    // Add content filter stage
    let filter_stage = Box::new(ContentFilterStage::new(
        "content_filter".to_string(),
        vec!["malware".to_string(), "virus".to_string()],
    ));
    pipeline.add_stage(filter_stage);
    
    // Test processing clean content
    let clean_request = create_test_request();
    let response = pipeline.process_request(clean_request).await.unwrap();
    assert_eq!(response.status, http::StatusCode::OK);
    
    // Test processing blocked content
    let mut blocked_request = create_test_request();
    blocked_request.body = bytes::Bytes::from("This content contains malware");
    let response = pipeline.process_request(blocked_request).await;
    assert!(response.is_err());
    
    // Test pipeline metrics
    let metrics = pipeline.get_metrics();
    assert!(metrics.requests_total > 0);
}

/// Test load balancing strategies
#[tokio::test]
async fn test_load_balancing() {
    let service_manager = ServiceManager::new();
    
    // Register multiple services for load balancing
    for i in 0..3 {
        let config = ServiceConfig {
            name: format!("service_{}", i),
            path: format!("/service_{}", i),
            methods: vec![IcapMethod::Reqmod],
            preview_size: 1024,
            timeout: Duration::from_secs(30),
            max_connections: 10,
            health_check_enabled: true,
            health_check_interval: Duration::from_secs(10),
            load_balancing: LoadBalancingStrategy::RoundRobin,
        };
        
        let module = Box::new(EchoModule::new());
        service_manager.register_service(config, module).await.unwrap();
    }
    
    // Test round-robin load balancing
    let services = service_manager.list_services();
    assert_eq!(services.len(), 3);
    
    // Test load balancer selection
    let load_balancer = service_manager.load_balancer;
    let selected = load_balancer.select_service(&services, &LoadBalancingStrategy::RoundRobin);
    assert!(selected.is_some());
    assert!(services.contains(&selected.unwrap()));
}

/// Test health monitoring
#[tokio::test]
async fn test_health_monitoring() {
    let service_manager = ServiceManager::new();
    
    // Register service with health checking
    let config = ServiceConfig {
        name: "health_test".to_string(),
        path: "/health_test".to_string(),
        methods: vec![IcapMethod::Reqmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 10,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(1),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Test health checker
    let health_checker = &service_manager.health_checker;
    
    // Start health checking
    health_checker.start_health_check("health_test", Duration::from_millis(100)).await.unwrap();
    
    // Wait a bit for health check to run
    sleep(Duration::from_millis(200)).await;
    
    // Check health status
    let is_healthy = health_checker.is_healthy("health_test");
    assert!(is_healthy);
    
    // Stop health checking
    health_checker.stop_health_check("health_test").await;
}

/// Test module metrics
#[tokio::test]
async fn test_module_metrics() {
    let echo_module = EchoModule::new();
    
    // Test basic metrics
    let metrics = echo_module.get_metrics();
    assert_eq!(metrics.requests_total, 0);
    assert_eq!(metrics.requests_per_second, 0.0);
    assert_eq!(metrics.error_rate, 0.0);
    assert!(echo_module.is_healthy());
    
    // Test module info
    assert_eq!(echo_module.name(), "echo");
    assert_eq!(echo_module.version(), "1.0.0");
    
    let methods = echo_module.supported_methods();
    assert!(methods.contains(&IcapMethod::Reqmod));
    assert!(methods.contains(&IcapMethod::Respmod));
    assert!(methods.contains(&IcapMethod::Options));
}

/// Test service metrics
#[tokio::test]
async fn test_service_metrics() {
    let service_manager = ServiceManager::new();
    
    // Register service
    let config = ServiceConfig {
        name: "metrics_test".to_string(),
        path: "/metrics_test".to_string(),
        methods: vec![IcapMethod::Reqmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 10,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(EchoModule::new());
    service_manager.register_service(config, module).await.unwrap();
    
    // Process some requests
    let request = create_test_request();
    for _ in 0..5 {
        let _ = service_manager.handle_request(&request).await;
    }
    
    // Check metrics
    let metrics = service_manager.get_service_metrics("metrics_test").unwrap();
    assert!(metrics.requests_total > 0);
    assert!(metrics.last_activity.is_some());
}

/// Test pipeline stage processing
#[tokio::test]
async fn test_pipeline_stages() {
    // Test logging stage
    let logging_stage = LoggingStage::new("test_logging".to_string(), "info".to_string());
    assert_eq!(logging_stage.name(), "test_logging");
    assert!(logging_stage.can_handle("text/html"));
    assert!(logging_stage.can_handle("application/json"));
    
    // Test content filter stage
    let filter_stage = ContentFilterStage::new(
        "test_filter".to_string(),
        vec!["malware".to_string(), "virus".to_string()],
    );
    assert_eq!(filter_stage.name(), "test_filter");
    assert!(filter_stage.can_handle("text/html"));
    assert!(!filter_stage.can_handle("audio/mp3"));
    
    // Test antivirus stage
    let antivirus_stage = AntivirusStage::new(
        "test_antivirus".to_string(),
        Duration::from_secs(30),
    );
    assert_eq!(antivirus_stage.name(), "test_antivirus");
    assert!(antivirus_stage.can_handle("text/html"));
    assert!(!antivirus_stage.can_handle("audio/mp3"));
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() {
    let service_manager = ServiceManager::new();
    
    // Test handling request for non-existent service
    let request = create_test_request();
    let result = service_manager.handle_request(&request).await;
    assert!(result.is_err());
    
    // Test unregistering non-existent service
    let result = service_manager.unregister_service("nonexistent").await;
    assert!(result.is_err());
}

/// Test concurrent processing
#[tokio::test]
async fn test_concurrent_processing() {
    let service_manager = ServiceManager::new();
    
    // Register service
    let config = ServiceConfig {
        name: "concurrent_test".to_string(),
        path: "/concurrent_test".to_string(),
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
    
    // Process multiple requests concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let service_manager = service_manager.clone();
        let request = create_test_request();
        let handle = tokio::spawn(async move {
            service_manager.handle_request(&request).await
        });
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
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
