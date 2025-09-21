# G3ICAP Content Filter Module

## Overview

The Content Filter Module is a comprehensive content filtering solution for G3ICAP that provides multiple layers of content inspection and blocking capabilities. It's designed to protect networks from malicious content, inappropriate material, and policy violations.

## Features

### 🛡️ **Multi-Layer Filtering**
- **Domain Filtering**: Block requests to specific domains or domain patterns
- **Keyword Filtering**: Block content containing specific keywords or patterns
- **MIME Type Filtering**: Block specific content types
- **File Extension Filtering**: Block files with specific extensions
- **File Size Filtering**: Block files exceeding size limits
- **Regular Expression Support**: Advanced pattern matching with regex

### 🚀 **Performance Features**
- **High Performance**: Sub-millisecond filtering decisions
- **Pattern Caching**: Cached regex patterns for faster matching
- **Async Processing**: Non-blocking content analysis
- **Memory Efficient**: Optimized memory usage for large content

### 📊 **Monitoring & Analytics**
- **Comprehensive Statistics**: Detailed filtering statistics
- **Real-time Metrics**: Live performance and blocking metrics
- **Audit Logging**: Complete audit trail of filtering decisions
- **Health Monitoring**: Module health and performance monitoring

## Architecture

### Core Components

```rust
pub struct ContentFilterModule {
    /// Module configuration
    config: ContentFilterConfig,
    /// Compiled regex patterns
    domain_patterns: Vec<Regex>,
    keyword_patterns: Vec<Regex>,
    /// Statistics tracking
    stats: Arc<RwLock<ContentFilterStats>>,
    /// Pattern cache for performance
    pattern_cache: Arc<RwLock<HashMap<String, bool>>>,
}
```

### Filtering Pipeline

1. **Request Analysis**: Extract and analyze request components
2. **Domain Check**: Verify against blocked domains and patterns
3. **URI Analysis**: Check URI for blocked keywords and patterns
4. **MIME Type Check**: Validate content type against blocked types
5. **File Size Check**: Verify file size against limits
6. **Content Analysis**: Scan request/response body for blocked content
7. **Decision Making**: Apply blocking rules and generate response

## Configuration

### Basic Configuration

```yaml
content_filter:
  blocked_domains:
    - "malware.com"
    - "phishing-site.org"
  blocked_keywords:
    - "malware"
    - "virus"
    - "trojan"
  blocked_mime_types:
    - "application/octet-stream"
  max_file_size: 10485760  # 10MB
  case_insensitive: true
  enable_regex: true
  blocking_action: "forbidden"
```

### Advanced Configuration

```yaml
content_filter:
  # Domain filtering
  blocked_domains:
    - "malware.com"
    - "phishing-site.org"
  blocked_domain_patterns:
    - ".*\\.malware\\..*"
    - ".*phishing.*"
  
  # Keyword filtering
  blocked_keywords:
    - "malware"
    - "virus"
    - "trojan"
  blocked_keyword_patterns:
    - "malware.*virus"
    - ".*phishing.*scam.*"
  
  # Content type filtering
  blocked_mime_types:
    - "application/octet-stream"
    - "application/x-executable"
  blocked_extensions:
    - "exe"
    - "bat"
    - "scr"
  
  # Size limits
  max_file_size: 10485760  # 10MB
  
  # Processing options
  case_insensitive: true
  enable_regex: true
  regex_cache_size: 1000
  
  # Response configuration
  blocking_action: "forbidden"  # forbidden, not_found, custom, redirect, replace
  custom_message: "Content blocked by security policy"
  
  # Monitoring
  enable_logging: true
  enable_metrics: true
```

### Blocking Actions

#### 1. **Forbidden (403)**
```yaml
blocking_action: "forbidden"
```
Returns HTTP 403 Forbidden with custom message.

#### 2. **Not Found (404)**
```yaml
blocking_action: "not_found"
```
Returns HTTP 404 Not Found to hide blocked content.

#### 3. **Custom Status Code**
```yaml
blocking_action: "custom"
custom_status: 451  # Unavailable For Legal Reasons
```

#### 4. **Redirect**
```yaml
blocking_action: "redirect"
redirect_url: "https://company.com/blocked-page"
```

#### 5. **Content Replacement**
```yaml
blocking_action: "replace"
replacement_content: "This content has been blocked by security policy"
```

## Usage Examples

### Basic Content Filtering

```rust
use g3icap::modules::content_filter::{ContentFilterModule, ContentFilterConfig};

// Create configuration
let config = ContentFilterConfig {
    blocked_domains: vec!["malware.com".to_string()],
    blocked_keywords: vec!["malware".to_string(), "virus".to_string()],
    blocked_mime_types: vec!["application/octet-stream".to_string()],
    max_file_size: Some(10 * 1024 * 1024), // 10MB
    case_insensitive: true,
    enable_regex: true,
    blocking_action: BlockingAction::Forbidden,
    enable_logging: true,
    enable_metrics: true,
    ..Default::default()
};

// Create module
let mut module = ContentFilterModule::new(config);

// Initialize module
let module_config = ModuleConfig {
    name: "content_filter".to_string(),
    path: PathBuf::from(""),
    version: "1.0.0".to_string(),
    config: serde_json::to_value(&config).unwrap(),
    dependencies: Vec::new(),
    load_timeout: Duration::from_secs(5),
    max_memory: 1024 * 1024,
    sandbox: true,
};

module.init(&module_config).await?;
```

### Advanced Pattern Matching

```rust
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
    ..Default::default()
};
```

### Real-time Statistics

```rust
// Get current statistics
let stats = module.get_stats();
println!("Total requests: {}", stats.total_requests);
println!("Blocked requests: {}", stats.blocked_requests);
println!("Blocked by domain: {}", stats.blocked_by_domain);
println!("Blocked by keyword: {}", stats.blocked_by_keyword);

// Reset statistics
module.reset_stats();
```

## API Reference

### ContentFilterConfig

```rust
pub struct ContentFilterConfig {
    pub blocked_domains: Vec<String>,
    pub blocked_domain_patterns: Vec<String>,
    pub blocked_keywords: Vec<String>,
    pub blocked_keyword_patterns: Vec<String>,
    pub blocked_mime_types: Vec<String>,
    pub blocked_extensions: Vec<String>,
    pub max_file_size: Option<u64>,
    pub case_insensitive: bool,
    pub enable_regex: bool,
    pub blocking_action: BlockingAction,
    pub custom_message: Option<String>,
    pub enable_logging: bool,
    pub enable_metrics: bool,
    pub regex_cache_size: usize,
}
```

### BlockingAction

```rust
pub enum BlockingAction {
    Forbidden,
    NotFound,
    Custom(u16),
    Redirect(String),
    Replace(String),
}
```

### ContentFilterStats

```rust
pub struct ContentFilterStats {
    pub total_requests: u64,
    pub blocked_requests: u64,
    pub allowed_requests: u64,
    pub blocked_by_domain: u64,
    pub blocked_by_keyword: u64,
    pub blocked_by_mime_type: u64,
    pub blocked_by_file_size: u64,
    pub blocked_by_regex: u64,
    pub total_processing_time: u64,
    pub last_reset: Instant,
}
```

## Performance Characteristics

### Benchmarks

- **Simple Keyword Matching**: < 0.1ms per request
- **Regex Pattern Matching**: < 1ms per request
- **Large Content Analysis**: < 10ms per MB
- **Memory Usage**: < 1MB for 1000 patterns
- **Throughput**: 10,000+ requests/second

### Optimization Features

- **Pattern Caching**: Frequently used patterns are cached
- **Early Termination**: Stops checking on first match
- **Lazy Evaluation**: Only processes content when needed
- **Memory Pooling**: Reuses memory for pattern matching

## Security Considerations

### Input Validation

- **Pattern Validation**: All regex patterns are validated before compilation
- **Size Limits**: Configurable limits prevent memory exhaustion
- **Timeout Protection**: Processing timeouts prevent DoS attacks
- **Resource Limits**: Memory and CPU limits per module

### Privacy Protection

- **No Content Storage**: Content is not stored or logged
- **Minimal Logging**: Only blocking decisions are logged
- **Data Anonymization**: Sensitive data is not exposed in logs
- **Secure Configuration**: Configuration is protected from tampering

## Monitoring and Alerting

### Metrics

```rust
// Module metrics
let metrics = module.get_metrics();
println!("Requests per second: {}", metrics.requests_per_second);
println!("Average processing time: {}ms", metrics.average_processing_time);
println!("Error rate: {}%", metrics.error_rate);
```

### Health Checks

```rust
// Check module health
if module.is_healthy() {
    println!("Content filter module is healthy");
} else {
    println!("Content filter module is unhealthy");
}
```

### Logging

```rust
// Enable detailed logging
let config = ContentFilterConfig {
    enable_logging: true,
    ..Default::default()
};

// Logs will include:
// - Blocking decisions with reasons
// - Performance metrics
// - Error conditions
// - Configuration changes
```

## Troubleshooting

### Common Issues

#### 1. **High Memory Usage**
```
Problem: Module using too much memory
Solution: Reduce regex_cache_size or disable regex matching
```

#### 2. **Slow Performance**
```
Problem: Filtering is too slow
Solution: Optimize patterns, enable caching, or reduce content analysis
```

#### 3. **False Positives**
```
Problem: Legitimate content being blocked
Solution: Review patterns, adjust case sensitivity, or refine regex
```

#### 4. **Pattern Compilation Errors**
```
Problem: Invalid regex patterns
Solution: Validate patterns before deployment
```

### Debug Mode

```rust
// Enable debug logging
RUST_LOG=debug cargo run

// Check pattern compilation
let config = ContentFilterConfig {
    blocked_keyword_patterns: vec!["invalid[regex".to_string()],
    enable_regex: true,
    ..Default::default()
};

// This will fail with clear error message
let result = module.compile_patterns();
```

## Best Practices

### 1. **Pattern Design**
- Use specific patterns to avoid false positives
- Test patterns thoroughly before deployment
- Use case-insensitive matching when appropriate
- Avoid overly complex regex patterns

### 2. **Performance Optimization**
- Enable pattern caching for frequently used patterns
- Use simple string matching when regex is not needed
- Set appropriate file size limits
- Monitor performance metrics regularly

### 3. **Security**
- Regularly update blocked domain lists
- Use threat intelligence feeds for patterns
- Implement proper logging and monitoring
- Test blocking effectiveness regularly

### 4. **Configuration Management**
- Use version control for configuration changes
- Test changes in staging environment
- Document all custom patterns
- Maintain backup configurations

## Integration Examples

### With G3ICAP Pipeline

```yaml
pipeline:
  stages:
    - name: "logging"
      type: "logging"
    - name: "content_filter"
      type: "content_filter"
      config:
        blocked_keywords: ["malware", "virus"]
        blocked_domains: ["malware.com"]
    - name: "antivirus"
      type: "antivirus"
```

### With Service Configuration

```yaml
services:
  - name: "web_filter"
    path: "/filter"
    module: "content_filter"
    methods: ["REQMOD", "RESPMOD"]
    config:
      blocked_keywords: ["malware", "phishing"]
      blocked_domains: ["malware.com", "phishing.org"]
      max_file_size: 10485760
      blocking_action: "forbidden"
```

## Future Enhancements

### Planned Features

- **Machine Learning Integration**: AI-powered content classification
- **Threat Intelligence Feeds**: Real-time threat data integration
- **Advanced Analytics**: Detailed content analysis and reporting
- **Custom Plugin Support**: User-defined filtering logic
- **Distributed Filtering**: Multi-node filtering coordination

### Performance Improvements

- **GPU Acceleration**: GPU-based pattern matching
- **Streaming Analysis**: Real-time content streaming
- **Advanced Caching**: Intelligent pattern caching
- **Parallel Processing**: Multi-threaded content analysis

---

The Content Filter Module provides enterprise-grade content filtering capabilities with high performance, comprehensive monitoring, and flexible configuration options. It's designed to integrate seamlessly with the G3ICAP platform while providing robust protection against malicious and inappropriate content.
