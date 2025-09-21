# G3ICAP Implementation Guide

## Quick Start

### 1. Choose Your Configuration Level

**For Development/Testing**:
```bash
# Copy basic configuration
cp examples/simple_config.yaml config.yaml

# Run with basic config
cargo run -- --config config.yaml
```

**For Production**:
```bash
# Copy standard configuration
cp examples/modular_config.yaml config.yaml

# Run with standard config
cargo run -- --config config.yaml
```

**For Complex Deployments**:
```bash
# Copy advanced configuration
cp examples/advanced_config.yaml config.yaml

# Run with advanced config
cargo run -- --config config.yaml
```

### 2. Run Examples

```bash
# Run modular server example
cargo run --example modular_server

# Run simple server
cargo run --example simple_server

# Run test client
cargo run --example test_client
```

### 3. Test the Modular Architecture

```bash
# Run modular architecture tests
cargo test modular_tests

# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### 4. Build and Deploy

```bash
# Build the project
cargo build --release

# Run with your configuration
cargo run --release -- --config config.yaml
```

## Implementation Steps

### Step 1: Module Development

#### Create a Custom Module

```rust
use g3icap::modules::{IcapModule, ModuleConfig, ModuleError, ModuleMetrics};
use g3icap::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use async_trait::async_trait;

pub struct CustomModule {
    name: String,
    version: String,
    metrics: ModuleMetrics,
}

impl CustomModule {
    pub fn new() -> Self {
        Self {
            name: "custom".to_string(),
            version: "1.0.0".to_string(),
            metrics: ModuleMetrics::default(),
        }
    }
}

#[async_trait]
impl IcapModule for CustomModule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_methods(&self) -> Vec<IcapMethod> {
        vec![IcapMethod::Reqmod, IcapMethod::Respmod]
    }
    
    async fn init(&mut self, _config: &ModuleConfig) -> Result<(), ModuleError> {
        // Initialize your module
        Ok(())
    }
    
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        // Process REQMOD request
        Ok(IcapResponse {
            status: http::StatusCode::OK,
            version: request.version,
            headers: request.headers.clone(),
            body: request.body.clone(),
            encapsulated: request.encapsulated.clone(),
        })
    }
    
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        // Process RESPMOD request
        Ok(IcapResponse {
            status: http::StatusCode::OK,
            version: request.version,
            headers: request.headers.clone(),
            body: request.body.clone(),
            encapsulated: request.encapsulated.clone(),
        })
    }
    
    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError> {
        // Process OPTIONS request
        let mut headers = http::HeaderMap::new();
        headers.insert("ISTag", "\"custom-1.0\"".parse().unwrap());
        headers.insert("Methods", "REQMOD, RESPMOD".parse().unwrap());
        headers.insert("Service", "Custom Service".parse().unwrap());
        
        Ok(IcapResponse {
            status: http::StatusCode::OK,
            version: request.version,
            headers,
            body: bytes::Bytes::new(),
            encapsulated: None,
        })
    }
    
    fn is_healthy(&self) -> bool {
        true
    }
    
    fn get_metrics(&self) -> ModuleMetrics {
        self.metrics.clone()
    }
    
    async fn cleanup(&mut self) {
        // Cleanup resources
    }
}
```

### Step 2: Service Registration

#### Register Your Module as a Service

```rust
use g3icap::services::{ServiceManager, ServiceConfig, LoadBalancingStrategy};

async fn register_custom_service() -> Result<(), Box<dyn std::error::Error>> {
    let service_manager = ServiceManager::new();
    
    let config = ServiceConfig {
        name: "custom".to_string(),
        path: "/custom".to_string(),
        methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod],
        preview_size: 1024,
        timeout: Duration::from_secs(30),
        max_connections: 100,
        health_check_enabled: true,
        health_check_interval: Duration::from_secs(10),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    
    let module = Box::new(CustomModule::new());
    service_manager.register_service(config, module).await?;
    
    Ok(())
}
```

### Step 3: Pipeline Development

#### Create a Custom Pipeline Stage

```rust
use g3icap::pipeline::{PipelineStage, PipelineContext, StageType, PipelineError};
use async_trait::async_trait;

pub struct CustomStage {
    name: String,
    config: serde_json::Value,
}

impl CustomStage {
    pub fn new(name: String, config: serde_json::Value) -> Self {
        Self { name, config }
    }
}

#[async_trait]
impl PipelineStage for CustomStage {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn stage_type(&self) -> StageType {
        StageType::Custom("custom".to_string())
    }
    
    fn can_handle(&self, content_type: &str) -> bool {
        // Define which content types this stage can handle
        content_type.starts_with("text/") || content_type.starts_with("application/")
    }
    
    async fn process(&self, context: &mut PipelineContext) -> Result<(), PipelineError> {
        // Process the content
        log::info!("Processing request in custom stage");
        
        // Add custom metadata
        context.metadata.insert(
            "processed_by".to_string(),
            "custom_stage".to_string(),
        );
        
        Ok(())
    }
    
    async fn init(&mut self, _config: &StageConfig) -> Result<(), PipelineError> {
        // Initialize the stage
        Ok(())
    }
    
    async fn cleanup(&mut self) {
        // Cleanup resources
    }
}
```

### Step 4: Configuration

#### Choose Your Configuration Level

**Basic Configuration** (Recommended for most users):
```yaml
# simple_config.yaml
server:
  host: "127.0.0.1"
  port: 1344

services:
  - name: "echo"
    path: "/echo"
    module: "echo"

  - name: "log"
    path: "/log"
    module: "logging"

pipeline:
  stages: ["logging", "echo"]

stats:
  enabled: true
```

**Standard Configuration** (Recommended for production):
```yaml
# modular_config.yaml
server:
  host: "127.0.0.1"
  port: 1344
  max_connections: 1000

logging:
  level: "info"
  file: "/var/log/g3icap/g3icap.log"

services:
  - name: "echo"
    path: "/echo"
    module: "echo"
    methods: ["REQMOD", "RESPMOD", "OPTIONS"]

  - name: "filter"
    path: "/filter"
    module: "content_filter"
    methods: ["REQMOD", "RESPMOD"]
    config:
      blocked_patterns: ["malware", "virus"]

pipeline:
  name: "default"
  stages: ["logging", "filter"]
  timeout: 60

stats:
  enabled: true
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
```

**Advanced Configuration** (For complex deployments):
```yaml
# advanced_config.yaml
# See examples/advanced_config.yaml for full configuration
```

### Step 5: Testing

#### Create Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_custom_module() {
        let module = CustomModule::new();
        assert_eq!(module.name(), "custom");
        assert_eq!(module.version(), "1.0.0");
        assert!(module.is_healthy());
    }

    #[test]
    async fn test_custom_stage() {
        let stage = CustomStage::new(
            "test".to_string(),
            serde_json::Value::Object(serde_json::Map::new()),
        );
        assert_eq!(stage.name(), "test");
        assert!(stage.can_handle("text/html"));
    }
}
```

#### Run Tests

```bash
# Run specific test
cargo test test_custom_module

# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## Advanced Usage

### 1. Dynamic Module Loading

```rust
use g3icap::modules::ModuleRegistry;

async fn load_dynamic_module() -> Result<(), Box<dyn std::error::Error>> {
    let config = ModuleConfig {
        name: "dynamic".to_string(),
        path: PathBuf::from("/usr/lib/g3icap/modules/libdynamic.so"),
        version: "1.0.0".to_string(),
        config: serde_json::Value::Object(serde_json::Map::new()),
        dependencies: Vec::new(),
        load_timeout: Duration::from_secs(30),
        max_memory: 104857600,
        sandbox: true,
    };
    
    let registry = ModuleRegistry::new(config);
    
    // Load module dynamically
    registry.load_module("dynamic", PathBuf::from("/usr/lib/g3icap/modules/libdynamic.so")).await?;
    
    Ok(())
}
```

### 2. Health Monitoring

```rust
use g3icap::services::ServiceManager;

async fn monitor_health() -> Result<(), Box<dyn std::error::Error>> {
    let service_manager = ServiceManager::new();
    
    // Register services...
    
    // Check health status
    for service_name in service_manager.list_services() {
        let is_healthy = service_manager.health_checker.is_healthy(&service_name);
        println!("Service {}: {}", service_name, if is_healthy { "Healthy" } else { "Unhealthy" });
    }
    
    Ok(())
}
```

### 3. Metrics Collection

```rust
use g3icap::services::ServiceManager;

async fn collect_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let service_manager = ServiceManager::new();
    
    // Register services...
    
    // Get all metrics
    let all_metrics = service_manager.get_all_metrics();
    for (service_name, metrics) in all_metrics {
        println!("Service: {}", service_name);
        println!("  Requests: {}", metrics.requests_total);
        println!("  RPS: {:.2}", metrics.requests_per_second);
        println!("  Error Rate: {:.2}%", metrics.error_rate * 100.0);
        println!("  Active Connections: {}", metrics.active_connections);
    }
    
    Ok(())
}
```

### 4. Pipeline Processing

```rust
use g3icap::pipeline::{ContentPipeline, PipelineConfig};

async fn process_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let config = PipelineConfig {
        name: "test".to_string(),
        stages: Vec::new(),
        timeout: Duration::from_secs(60),
        parallel: false,
        max_concurrent: 10,
    };
    
    let mut pipeline = ContentPipeline::new(config);
    
    // Add stages
    let logging_stage = Box::new(LoggingStage::new("logging".to_string(), "info".to_string()));
    pipeline.add_stage(logging_stage);
    
    let custom_stage = Box::new(CustomStage::new(
        "custom".to_string(),
        serde_json::Value::Object(serde_json::Map::new()),
    ));
    pipeline.add_stage(custom_stage);
    
    // Process request
    let request = create_test_request();
    let response = pipeline.process_request(request).await?;
    
    println!("Pipeline response: {} {}", response.status, response.version);
    
    Ok(())
}
```

## Troubleshooting

### Common Issues

#### 1. Module Loading Errors

```
Error: Module load failed: Dynamic loading not implemented yet
```

**Solution**: Dynamic loading is not yet implemented. Use built-in modules or implement static module registration.

#### 2. Service Registration Errors

```
Error: Service not found: echo
```

**Solution**: Ensure the service is registered before trying to handle requests.

#### 3. Pipeline Processing Errors

```
Error: Stage error: Content blocked by pattern: malware
```

**Solution**: This is expected behavior for content filtering. Adjust the blocked patterns or handle the error appropriately.

### Debugging

#### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --example modular_server
```

#### Check Service Health

```rust
let is_healthy = service_manager.health_checker.is_healthy("service_name");
println!("Service health: {}", is_healthy);
```

#### Monitor Metrics

```rust
let metrics = service_manager.get_service_metrics("service_name");
if let Some(metrics) = metrics {
    println!("Metrics: {:?}", metrics);
}
```

## Performance Optimization

### 1. Connection Pooling

```yaml
connection_pool:
  max_connections: 1000
  min_connections: 10
  idle_timeout: 300
  max_lifetime: 3600
```

### 2. Memory Management

```yaml
memory:
  max_heap_size: 1073741824  # 1GB
  gc_interval: 60
  gc_threshold: 0.8
  custom_allocator: true
```

### 3. Caching

```yaml
cache:
  enabled: true
  max_size: 104857600  # 100MB
  ttl: 3600  # 1 hour
  eviction_policy: "lru"
```

## Security Considerations

### 1. Module Sandboxing

```yaml
modules:
  - name: "untrusted"
    sandbox: true
    max_memory: 52428800  # 50MB
```

### 2. Authentication

```yaml
security:
  enable_tls: true
  require_client_cert: true
  tls_cert: "/path/to/cert.pem"
  tls_key: "/path/to/key.pem"
```

### 3. Rate Limiting

```yaml
rate_limiting:
  enabled: true
  requests_per_second: 100
  burst_size: 200
  per_client: true
```

## Conclusion

This implementation guide provides a comprehensive overview of how to use and extend the G3ICAP modular architecture. By following these patterns and examples, you can create custom modules, services, and pipelines that integrate seamlessly with the G3ICAP ecosystem.

For more advanced usage and examples, refer to the source code and test files in the repository.
