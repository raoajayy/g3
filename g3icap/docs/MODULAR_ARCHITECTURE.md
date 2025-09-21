# G3ICAP Modular Architecture

## Overview

G3ICAP implements a sophisticated modular architecture inspired by the [c-icap-server](https://github.com/c-icap/c-icap-server) project. This architecture enables dynamic loading of ICAP service modules, comprehensive service management, and flexible content processing pipelines.

## Architecture Components

### 1. Module System (`src/modules/`)

The module system provides a plugin-based architecture for ICAP services.

#### Core Components

- **`IcapModule` Trait**: Defines the interface for all ICAP modules
- **`ModuleRegistry`**: Manages module lifecycle and registration
- **`ModuleConfig`**: Configuration for module loading and execution
- **Built-in Modules**: Echo, Logging, Content Filter, Antivirus

#### Module Interface

```rust
#[async_trait]
pub trait IcapModule: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn supported_methods(&self) -> Vec<IcapMethod>;
    async fn init(&mut self, config: &ModuleConfig) -> Result<(), ModuleError>;
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    async fn handle_respmod(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    async fn handle_options(&self, request: &IcapRequest) -> Result<IcapResponse, ModuleError>;
    fn is_healthy(&self) -> bool;
    fn get_metrics(&self) -> ModuleMetrics;
    async fn cleanup(&mut self);
}
```

#### Built-in Modules

1. **Echo Module**: Basic request/response echoing for testing
2. **Logging Module**: Request/response logging and monitoring
3. **Content Filter Module**: Content filtering and blocking
4. **Antivirus Module**: Antivirus scanning integration

### 2. Service Management (`src/services/`)

The service management system handles service registration, health monitoring, and load balancing.

#### Core Components

- **`ServiceManager`**: Central service registry and management
- **`ServiceConfig`**: Service configuration and settings
- **`HealthChecker`**: Service health monitoring and recovery
- **`LoadBalancer`**: Load balancing strategies

#### Service Configuration

```yaml
services:
  - name: "content-filter"
    path: "/filter"
    methods: ["REQMOD", "RESPMOD"]
    preview_size: 1024
    timeout: 30
    max_connections: 100
    health_check:
      enabled: true
      interval: 10
    load_balancing: "round_robin"
    module: "content_filter"
```

#### Load Balancing Strategies

- **Round Robin**: Distribute requests evenly across services
- **Least Connections**: Route to service with fewest active connections
- **Weighted Round Robin**: Distribute based on service weights
- **Random**: Random selection for load distribution

### 3. Content Pipeline (`src/pipeline/`)

The content pipeline system enables complex content processing through multiple stages.

#### Core Components

- **`ContentPipeline`**: Main pipeline orchestrator
- **`PipelineStage` Trait**: Interface for pipeline stages
- **`PipelineContext`**: Context passed between stages
- **Built-in Stages**: Logging, Content Filter, Antivirus

#### Pipeline Configuration

```yaml
pipelines:
  - name: "default"
    stages:
      - name: "logging"
        type: "logging"
        enabled: true
      - name: "content_filter"
        type: "content_filter"
        config:
          blocked_patterns: ["malware", "virus"]
        enabled: true
      - name: "antivirus"
        type: "antivirus"
        config:
          engine: "clamav"
          timeout: 30
        enabled: true
    timeout: 60
    parallel: false
```

#### Pipeline Stages

1. **Logging Stage**: Request/response logging
2. **Content Filter Stage**: Content filtering and blocking
3. **Antivirus Stage**: Antivirus scanning
4. **Transform Stage**: Content transformation
5. **Custom Stages**: User-defined processing stages

## Usage Examples

### 1. Basic Module Usage

```rust
use g3icap::modules::{ModuleRegistry, builtin::EchoModule};

// Create module registry
let config = ModuleConfig {
    name: "example".to_string(),
    path: PathBuf::from("/tmp/modules"),
    version: "1.0.0".to_string(),
    config: serde_json::Value::Object(serde_json::Map::new()),
    dependencies: Vec::new(),
    load_timeout: Duration::from_secs(30),
    max_memory: 100 * 1024 * 1024,
    sandbox: true,
};

let registry = ModuleRegistry::new(config);

// Load echo module
let echo_module = EchoModule::new();
// Module loading would be implemented here
```

### 2. Service Management

```rust
use g3icap::services::{ServiceManager, ServiceConfig, LoadBalancingStrategy};

// Create service manager
let service_manager = ServiceManager::new();

// Register echo service
let config = ServiceConfig {
    name: "echo".to_string(),
    path: "/echo".to_string(),
    methods: vec![IcapMethod::Reqmod, IcapMethod::Respmod],
    preview_size: 1024,
    timeout: Duration::from_secs(30),
    max_connections: 100,
    health_check_enabled: true,
    health_check_interval: Duration::from_secs(10),
    load_balancing: LoadBalancingStrategy::RoundRobin,
};

let module = Box::new(EchoModule::new());
service_manager.register_service(config, module).await?;

// Handle request
let response = service_manager.handle_request(&request).await?;
```

### 3. Content Pipeline

```rust
use g3icap::pipeline::{ContentPipeline, PipelineConfig, stages::*};

// Create pipeline
let config = PipelineConfig {
    name: "default".to_string(),
    stages: Vec::new(),
    timeout: Duration::from_secs(60),
    parallel: false,
    max_concurrent: 10,
};

let mut pipeline = ContentPipeline::new(config);

// Add stages
let logging_stage = Box::new(LoggingStage::new("logging".to_string(), "info".to_string()));
pipeline.add_stage(logging_stage);

let filter_stage = Box::new(ContentFilterStage::new(
    "content_filter".to_string(),
    vec!["malware".to_string(), "virus".to_string()],
));
pipeline.add_stage(filter_stage);

// Process request
let response = pipeline.process_request(request).await?;
```

## Configuration

### Module Configuration

```yaml
modules:
  - name: "antivirus"
    path: "/usr/lib/g3icap/modules/libantivirus.so"
    version: "1.0.0"
    config:
      enabled: true
      engine: "clamav"
      scan_timeout: 30
      max_file_size: 10485760
    dependencies: ["logging"]
    load_timeout: 30
    max_memory: 104857600
    sandbox: false
```

### Service Configuration

```yaml
services:
  - name: "antivirus"
    path: "/scan"
    methods: ["REQMOD", "RESPMOD"]
    preview_size: 10240
    timeout: 120
    max_connections: 20
    health_check:
      enabled: true
      interval: 30
      timeout: 15
      retries: 2
    load_balancing: "round_robin"
    module: "antivirus"
```

### Pipeline Configuration

```yaml
pipelines:
  - name: "default"
    stages:
      - name: "logging"
        type: "logging"
        config:
          log_level: "info"
        enabled: true
      - name: "content_filter"
        type: "content_filter"
        config:
          blocked_patterns: ["malware", "virus"]
        enabled: true
      - name: "antivirus"
        type: "antivirus"
        config:
          engine: "clamav"
          timeout: 30
        enabled: true
    timeout: 120
    parallel: false
    max_concurrent: 10
    fail_fast: true
```

## Monitoring and Metrics

### Module Metrics

```rust
pub struct ModuleMetrics {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub error_rate: f64,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub last_activity: Option<Instant>,
}
```

### Service Metrics

```rust
pub struct ServiceMetrics {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub error_rate: f64,
    pub active_connections: usize,
    pub total_connections: u64,
    pub connection_errors: u64,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub last_activity: Option<Instant>,
    pub is_healthy: bool,
}
```

### Pipeline Metrics

```rust
pub struct PipelineMetrics {
    pub requests_total: u64,
    pub total_processing_time: Duration,
    pub average_processing_time: Duration,
    pub successful_stages: u64,
    pub failed_stages: u64,
    pub pipeline_errors: u64,
}
```

## Error Handling

### Module Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error("Module not found: {0}")]
    NotFound(String),
    #[error("Module load failed: {0}")]
    LoadFailed(String),
    #[error("Module initialization failed: {0}")]
    InitFailed(String),
    #[error("Module execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Module dependency missing: {0}")]
    DependencyMissing(String),
    #[error("Module version incompatible: {0}")]
    VersionIncompatible(String),
}
```

### Service Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Method not supported: {0}")]
    MethodNotSupported(String),
    #[error("Too many connections")]
    TooManyConnections,
    #[error("Service unhealthy")]
    ServiceUnhealthy,
    #[error("Module error: {0}")]
    ModuleError(ModuleError),
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("Load balancing error: {0}")]
    LoadBalancingError(String),
}
```

## Testing

The modular architecture includes comprehensive tests:

- **Module Tests**: Test module loading, initialization, and execution
- **Service Tests**: Test service registration, health monitoring, and load balancing
- **Pipeline Tests**: Test pipeline processing and stage execution
- **Integration Tests**: Test end-to-end functionality
- **Performance Tests**: Test concurrent processing and performance

Run tests with:

```bash
cargo test modular_tests
```

## Performance Considerations

### Memory Management

- Custom allocators for frequent allocations
- Memory pools for different allocation sizes
- Garbage collection optimization
- Memory usage monitoring per module

### Connection Pooling

- Efficient connection reuse
- Configurable pool sizes
- Connection lifecycle management
- Pool metrics and monitoring

### Caching

- Response caching for improved performance
- Content caching for frequently accessed data
- Cache invalidation strategies
- Cache metrics and monitoring

## Security

### Module Sandboxing

- Isolated execution environments
- Resource limits and quotas
- System call restrictions
- Memory access controls

### Authentication and Authorization

- Module authentication
- Service access controls
- Request validation
- Security headers

## Future Enhancements

### Planned Features

1. **Dynamic Module Loading**: Runtime loading of shared libraries
2. **Module Hot Reloading**: Update modules without server restart
3. **Advanced Load Balancing**: More sophisticated load balancing algorithms
4. **Pipeline Optimization**: Parallel stage processing
5. **Module Marketplace**: Community module distribution

### Extension Points

1. **Custom Module Types**: Support for different module architectures
2. **Plugin System**: Easy development and deployment of modules
3. **Configuration Management**: Advanced configuration management
4. **Monitoring Integration**: Integration with external monitoring systems

## Conclusion

The G3ICAP modular architecture provides a flexible, extensible, and high-performance foundation for ICAP service development. By following the established patterns and interfaces, developers can easily create custom modules and services that integrate seamlessly with the G3ICAP ecosystem.

The architecture is designed to be:
- **Extensible**: Easy to add new modules and services
- **Scalable**: Handle high loads with efficient resource management
- **Reliable**: Robust error handling and recovery mechanisms
- **Monitorable**: Comprehensive metrics and health monitoring
- **Maintainable**: Clean interfaces and well-documented APIs
