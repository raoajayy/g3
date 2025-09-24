# ISTag Management

This document describes the ISTag (Implementation-Specific Tag) management functionality in the G3ICAP server, which is implemented according to RFC 3507 specifications.

## Overview

ISTag management provides a comprehensive system for generating, validating, caching, and managing implementation-specific tags in ICAP servers. This functionality is essential for proper cache management, service identification, and protocol compliance.

## Features

### Core Functionality

- **ISTag Generation**: RFC 3507 compliant ISTag generation with multiple formats
- **ISTag Validation**: Comprehensive validation of ISTag format and content
- **Cache Management**: ISTag-based cache invalidation and management
- **Service Identification**: Unique service identification per ISTag
- **Version Management**: ISTag versioning and updates
- **Metrics Collection**: Detailed metrics for monitoring and optimization

### ISTag Types

The system supports multiple ISTag types:

- **Service ISTags**: Identify specific ICAP services
- **Version ISTags**: Identify service versions
- **Cache ISTags**: Manage cache invalidation
- **Session ISTags**: Track user sessions
- **Request ISTags**: Track individual requests

### ISTag Formats

- **Weak ISTags**: `W/"value"` format for cache validation
- **Strong ISTags**: `"value"` format for strict validation
- **Custom ISTags**: Configurable format for specific needs

## Configuration

### Basic Configuration

```yaml
# Enable ISTag functionality
enabled: true

# Service identification
service:
  id: "g3icap"
  name: "G3ICAP Service"
  version: "1.0.0"
  capabilities:
    - "REQMOD"
    - "RESPMOD"
    - "OPTIONS"
```

### Advanced Configuration

```yaml
# ISTag generation settings
generation:
  enabled: true
  default_format: "weak"
  include_timestamp: true
  include_service_info: true
  max_istag_length: 256

# ISTag validation settings
validation:
  enabled: true
  strict_mode: true
  validate_format: true
  validate_expiration: true

# ISTag caching settings
cache:
  enabled: true
  max_size: 10000
  default_ttl_seconds: 3600
  eviction_policy: "lru"
```

## Usage

### Basic Usage

```rust
use g3icap::istag::{IstagManager, IstagConfig, IstagType};

// Create configuration
let config = IstagConfig::default();

// Create manager
let manager = IstagManager::new(config)?;

// Generate service ISTag
let istag = manager.generate_service_istag(3600).await?;

// Validate ISTag
let validation = manager.validate_istag(&istag).await;
assert!(validation.is_valid);
```

### Advanced Usage

```rust
use g3icap::istag::{
    IstagManager, IstagConfig, IstagType, IstagFormat, 
    IstagGenerationOptions, ServiceStatus
};
use std::collections::HashMap;

// Create custom configuration
let mut config = IstagConfig::default();
config.service_id = "custom-service".to_string();
config.service_version = "2.0.0".to_string();

// Create manager
let manager = IstagManager::new(config)?;

// Generate custom ISTag
let mut metadata = HashMap::new();
metadata.insert("custom_field".to_string(), "custom_value".to_string());

let options = IstagGenerationOptions {
    istag_type: IstagType::Service,
    service_id: "custom-service".to_string(),
    version: "2.0.0".to_string(),
    expiration_seconds: 3600,
    metadata,
    format: IstagFormat::Weak,
    include_timestamp: true,
    include_service_info: true,
};

let istag = manager.generate_istag(options).await?;
```

### Cache Operations

```rust
// Store ISTag in cache
let service_info = ServiceInfo::new(
    "test-service".to_string(),
    "1.0.0".to_string(),
    "Test Service".to_string(),
    ServiceStatus::Active,
);

manager.store_istag(istag.clone(), service_info).await?;

// Retrieve ISTag from cache
if let Some(cache_entry) = manager.get_istag(&istag.value).await? {
    println!("Retrieved ISTag: {}", cache_entry.istag.value);
}

// Get ISTags by type
let service_istags = manager.get_istags_by_type(IstagType::Service).await?;
```

### Validation

```rust
// Validate ISTag
let validation = manager.validate_istag(&istag).await;
if validation.is_valid {
    println!("ISTag is valid");
} else {
    for error in &validation.errors {
        println!("Validation error: {}", error.message);
    }
}

// Validate ISTag from string
let validation = manager.validate_istag_string("W/\"test-istag\"", IstagType::Service).await;
```

## API Reference

### IstagManager

The main ISTag management class.

#### Methods

- `new(config: IstagConfig) -> IstagResult<Self>`: Create a new ISTag manager
- `generate_service_istag(expiration_seconds: u64) -> IstagResult<Istag>`: Generate service ISTag
- `generate_version_istag(version: String, expiration_seconds: u64) -> IstagResult<Istag>`: Generate version ISTag
- `generate_cache_istag(cache_key: String, expiration_seconds: u64) -> IstagResult<Istag>`: Generate cache ISTag
- `generate_session_istag(session_id: String, expiration_seconds: u64) -> IstagResult<Istag>`: Generate session ISTag
- `generate_request_istag(request_id: String, expiration_seconds: u64) -> IstagResult<Istag>`: Generate request ISTag
- `validate_istag(istag: &Istag) -> IstagValidationResult`: Validate ISTag
- `validate_istag_string(istag_string: &str, expected_type: IstagType) -> IstagValidationResult`: Validate ISTag from string
- `get_istag(istag_value: &str) -> IstagResult<Option<IstagCacheEntry>>`: Get ISTag from cache
- `store_istag(istag: Istag, service_info: ServiceInfo) -> IstagResult<()>`: Store ISTag in cache
- `remove_istag(istag_value: &str) -> IstagResult<bool>`: Remove ISTag from cache
- `get_istags_by_type(istag_type: IstagType) -> IstagResult<Vec<Istag>>`: Get ISTags by type
- `get_istags_by_service(service_id: &str) -> IstagResult<Vec<Istag>>`: Get ISTags by service
- `cleanup_expired() -> IstagResult<usize>`: Clean up expired ISTags
- `get_cache_stats() -> IstagStats`: Get cache statistics
- `get_metrics_summary() -> IstagMetricsSummary`: Get metrics summary
- `export_metrics_json() -> Result<String, serde_json::Error>`: Export metrics as JSON
- `export_metrics_csv() -> String`: Export metrics as CSV
- `export_metrics_prometheus() -> String`: Export metrics in Prometheus format

### Istag

Represents an ISTag.

#### Properties

- `value: String`: The ISTag value
- `istag_type: IstagType`: The ISTag type
- `service_id: String`: The service ID
- `version: String`: The service version
- `expires_at: Option<SystemTime>`: Expiration time
- `metadata: HashMap<String, String>`: Additional metadata

#### Methods

- `is_expired() -> bool`: Check if ISTag is expired
- `get_metadata(key: &str) -> Option<&String>`: Get metadata value
- `set_metadata(key: String, value: String)`: Set metadata value
- `to_string() -> String`: Convert to string representation

### IstagConfig

Configuration for ISTag management.

#### Properties

- `enabled: bool`: Enable ISTag functionality
- `service_id: String`: Service identifier
- `service_name: String`: Service name
- `service_version: String`: Service version
- `capabilities: Vec<String>`: Service capabilities
- `generation: IstagGenerationConfig`: Generation settings
- `validation: IstagValidationConfig`: Validation settings
- `cache: IstagCacheConfig`: Cache settings
- `metrics: IstagMetricsConfig`: Metrics settings

## Metrics

### Available Metrics

- **Generation Metrics**: ISTag generation counts and timing
- **Validation Metrics**: Validation success/failure rates
- **Cache Metrics**: Cache hit/miss ratios and performance
- **Performance Metrics**: Operation timing and throughput
- **Error Metrics**: Error rates and types

### Export Formats

- **JSON**: Structured data for programmatic access
- **CSV**: Tabular data for spreadsheet analysis
- **Prometheus**: Time-series data for monitoring systems

## Error Handling

### Error Types

- `IstagError::ConfigError`: Configuration-related errors
- `IstagError::GenerationError`: ISTag generation errors
- `IstagError::ValidationError`: ISTag validation errors
- `IstagError::CacheError`: Cache operation errors
- `IstagError::MetricsError`: Metrics collection errors

### Error Recovery

The system includes automatic error recovery mechanisms:

- **Retry Logic**: Automatic retry for transient failures
- **Fallback Mechanisms**: Graceful degradation when features fail
- **Error Logging**: Comprehensive error logging and monitoring
- **Health Checks**: Regular health checks and status reporting

## Performance Considerations

### Optimization Features

- **Connection Pooling**: Reuse connections for better performance
- **Caching**: Intelligent caching to reduce computation
- **Batch Operations**: Batch processing for multiple operations
- **Compression**: Data compression to reduce memory usage
- **Metrics**: Performance monitoring and optimization

### Best Practices

1. **Use appropriate ISTag types** for different use cases
2. **Set reasonable expiration times** to balance freshness and performance
3. **Monitor cache hit ratios** and adjust cache settings accordingly
4. **Use batch operations** when processing multiple ISTags
5. **Enable metrics collection** for performance monitoring
6. **Regular cleanup** of expired ISTags to maintain performance

## Troubleshooting

### Common Issues

1. **ISTag generation failures**: Check configuration and service settings
2. **Validation errors**: Verify ISTag format and content
3. **Cache performance issues**: Monitor cache hit ratios and adjust settings
4. **Memory usage**: Enable compression and regular cleanup
5. **Metrics collection**: Check metrics configuration and export settings

### Debug Mode

Enable debug mode for detailed logging:

```yaml
debug:
  enabled: true
  verbose_logging: true
  trace_operations: true
  profile_performance: true
```

## Compliance

### RFC 3507 Compliance

The ISTag management system is fully compliant with RFC 3507 specifications:

- **ISTag Format**: Proper weak and strong ISTag formats
- **Service Identification**: Unique service identification
- **Cache Management**: Proper cache invalidation mechanisms
- **Protocol Compliance**: Full ICAP protocol compliance

### Security Compliance

- **Authentication**: Optional authentication for ISTag operations
- **Authorization**: Service-based access control
- **Encryption**: Optional encryption for sensitive data
- **Rate Limiting**: Protection against abuse

## Examples

See the `examples/istag_example.rs` file for comprehensive usage examples.

## Support

For issues, questions, or contributions related to ISTag management, please refer to the main G3ICAP documentation or contact the development team.
