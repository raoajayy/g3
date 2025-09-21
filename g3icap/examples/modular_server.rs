/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Modular ICAP Server Example
//! 
//! This example demonstrates the modular architecture of G3ICAP,
//! including module loading, service management, and content pipeline processing.

use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

use g3icap::modules::{ModuleConfig, ModuleRegistry, builtin::{EchoModule, LoggingModule}};
use g3icap::services::{ServiceConfig, ServiceManager, LoadBalancingStrategy};
use g3icap::pipeline::{ContentPipeline, PipelineConfig, stages::{LoggingStage, ContentFilterStage, AntivirusStage}};
use g3icap::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use g3icap::opts::ProcArgs;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 Starting G3ICAP Modular Server Example");
    
    // 1. Initialize Module Registry
    println!("\n📦 Initializing Module Registry...");
    let module_config = ModuleConfig {
        name: "example".to_string(),
        path: std::path::PathBuf::from("/tmp/modules"),
        version: "1.0.0".to_string(),
        config: serde_json::Value::Object(serde_json::Map::new()),
        dependencies: Vec::new(),
        load_timeout: Duration::from_secs(30),
        max_memory: 100 * 1024 * 1024, // 100MB
        sandbox: true,
    };
    
    let module_registry = ModuleRegistry::new(module_config);
    println!("✅ Module Registry initialized");
    
    // 2. Initialize Service Manager
    println!("\n🔧 Initializing Service Manager...");
    let service_manager = ServiceManager::new();
    
    // Register echo service
    let echo_config = ServiceConfig {
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
    
    let echo_module = Box::new(EchoModule::new());
    service_manager.register_service(echo_config, echo_module).await?;
    println!("✅ Echo service registered");
    
    // Register logging service
    let logging_config = ServiceConfig {
        name: "logging".to_string(),
        path: "/logging".to_string(),
        methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 50,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(15),
        load_balancing: LoadBalancingStrategy::LeastConnections,
    };
    
    let logging_module = Box::new(LoggingModule::new());
    service_manager.register_service(logging_config, logging_module).await?;
    println!("✅ Logging service registered");
    
    // 3. Initialize Content Pipeline
    println!("\n🔄 Initializing Content Pipeline...");
    let pipeline_config = PipelineConfig {
        name: "default".to_string(),
        stages: Vec::new(), // Will be populated with stage configs
        timeout: Duration::from_secs(60),
        parallel: false,
        max_concurrent: 10,
    };
    
    let mut pipeline = ContentPipeline::new(pipeline_config);
    
    // Add pipeline stages
    let logging_stage = Box::new(LoggingStage::new(
        "logging".to_string(),
        "info".to_string(),
    ));
    pipeline.add_stage(logging_stage);
    
    let content_filter_stage = Box::new(ContentFilterStage::new(
        "content_filter".to_string(),
        vec!["malware".to_string(), "virus".to_string()],
    ));
    pipeline.add_stage(content_filter_stage);
    
    let antivirus_stage = Box::new(AntivirusStage::new(
        "antivirus".to_string(),
        Duration::from_secs(30),
    ));
    pipeline.add_stage(antivirus_stage);
    
    println!("✅ Content Pipeline initialized with {} stages", 3);
    
    // 4. Demonstrate Service Processing
    println!("\n🧪 Demonstrating Service Processing...");
    
    // Create a sample ICAP request
    let sample_request = create_sample_request();
    
    // Process through echo service
    println!("📤 Processing request through echo service...");
    match service_manager.handle_request(&sample_request).await {
        Ok(response) => {
            println!("✅ Echo service response: {} {:?}", response.status, response.version);
            println!("   Headers: {} headers", response.headers.len());
            println!("   Body size: {} bytes", response.body.len());
        }
        Err(e) => {
            println!("❌ Echo service error: {}", e);
        }
    }
    
    // Process through logging service
    println!("\n📤 Processing request through logging service...");
    match service_manager.handle_request(&sample_request).await {
        Ok(response) => {
            println!("✅ Logging service response: {} {:?}", response.status, response.version);
        }
        Err(e) => {
            println!("❌ Logging service error: {}", e);
        }
    }
    
    // 5. Demonstrate Pipeline Processing
    println!("\n🔄 Demonstrating Pipeline Processing...");
    
    match pipeline.process_request(sample_request.clone()).await {
        Ok(response) => {
            println!("✅ Pipeline processing completed");
            println!("   Response: {} {:?}", response.status, response.version);
            println!("   Body size: {} bytes", response.body.len());
        }
        Err(e) => {
            println!("❌ Pipeline processing error: {}", e);
        }
    }
    
    // 6. Demonstrate Service Metrics
    println!("\n📊 Service Metrics:");
    let services = service_manager.list_services();
    for service_name in services {
        if let Some(metrics) = service_manager.get_service_metrics(&service_name) {
            println!("   {}: {} requests, {:.2} req/s, {:.2}% error rate",
                service_name,
                metrics.requests_total,
                metrics.requests_per_second,
                metrics.error_rate * 100.0
            );
        }
    }
    
    // 7. Demonstrate Pipeline Metrics
    println!("\n📊 Pipeline Metrics:");
    let pipeline_metrics = pipeline.get_metrics();
    println!("   Total requests: {}", pipeline_metrics.requests_total);
    println!("   Average processing time: {:?}", pipeline_metrics.average_processing_time);
    println!("   Successful stages: {}", pipeline_metrics.successful_stages);
    println!("   Failed stages: {}", pipeline_metrics.failed_stages);
    
    // 8. Demonstrate Health Monitoring
    println!("\n🏥 Health Monitoring:");
    for service_name in service_manager.list_services() {
        let is_healthy = service_manager.is_service_healthy(&service_name);
        println!("   {}: {}", service_name, if is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    }
    
    // 9. Simulate some load
    println!("\n⚡ Simulating load...");
    for i in 0..5 {
        let request = create_sample_request();
        match service_manager.handle_request(&request).await {
            Ok(_) => println!("   Request {}: ✅ Success", i + 1),
            Err(e) => println!("   Request {}: ❌ Error: {}", i + 1, e),
        }
        sleep(Duration::from_millis(100)).await;
    }
    
    // 10. Final metrics
    println!("\n📈 Final Metrics:");
    let all_metrics = service_manager.get_all_metrics();
    for (service_name, metrics) in all_metrics {
        println!("   {}: {} requests, {:.2} req/s, {:.2}% error rate",
            service_name,
            metrics.requests_total,
            metrics.requests_per_second,
            metrics.error_rate * 100.0
        );
    }
    
    println!("\n🎉 Modular Server Example completed successfully!");
    Ok(())
}

/// Create a sample ICAP request for testing
fn create_sample_request() -> IcapRequest {
    use http::{HeaderMap, Method, Uri, Version};
    use bytes::Bytes;
    
    let mut headers = HeaderMap::new();
    headers.insert("Host", "example.com".parse().unwrap());
    headers.insert("User-Agent", "G3ICAP-Test/1.0".parse().unwrap());
    headers.insert("Content-Type", "text/html".parse().unwrap());
    
    IcapRequest {
        method: IcapMethod::Reqmod,
        uri: Uri::from_static("icap://example.com/echo"),
        version: Version::HTTP_11,
        headers,
        body: Bytes::from("Hello, G3ICAP! This is a test request."),
        encapsulated: None,
    }
}
