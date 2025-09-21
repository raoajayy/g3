# G3ICAP Configuration Guide

## Configuration Complexity Levels

G3ICAP supports three levels of configuration complexity to suit different use cases:

### 1. **Basic Configuration** (Recommended for most users)

**File**: `examples/simple_config.yaml`

```yaml
# G3ICAP Basic Configuration
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

**Features**:
- ✅ Minimal configuration
- ✅ Quick setup
- ✅ Essential services only
- ✅ Default settings for everything else

### 2. **Standard Configuration** (Recommended for production)

**File**: `examples/modular_config.yaml`

```yaml
# G3ICAP Standard Configuration
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

  - name: "scan"
    path: "/scan"
    module: "antivirus"
    methods: ["REQMOD", "RESPMOD"]
    config:
      engine: "clamav"
      timeout: 30

pipeline:
  name: "default"
  stages: ["logging", "filter", "scan"]
  timeout: 60

stats:
  enabled: true
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
```

**Features**:
- ✅ Production-ready settings
- ✅ Custom service configurations
- ✅ Pipeline processing
- ✅ Statistics collection
- ✅ Logging configuration

### 3. **Advanced Configuration** (For complex deployments)

**File**: `examples/advanced_config.yaml`

```yaml
# G3ICAP Advanced Configuration
# Full configuration with all options

server:
  host: "127.0.0.1"
  port: 1344
  metrics_port: 9090
  max_connections: 1000
  connection_timeout: 30
  read_timeout: 60
  write_timeout: 60

logging:
  level: "info"
  format: "json"
  output: "stdout"
  file: "/var/log/g3icap/g3icap.log"
  rotation:
    max_size: "100MB"
    max_files: 10
    compress: true

modules:
  - name: "echo"
    path: "/usr/lib/g3icap/modules/libecho.so"
    version: "1.0.0"
    config:
      enabled: true
      timeout: 30
    dependencies: []
    load_timeout: 30
    max_memory: 104857600
    sandbox: true

services:
  - name: "echo"
    path: "/echo"
    module: "echo"
    methods: ["REQMOD", "RESPMOD", "OPTIONS"]
    preview_size: 1024
    timeout: 30
    max_connections: 100
    health_check:
      enabled: true
      interval: 10
      timeout: 5
      retries: 3
    load_balancing: "round_robin"

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
    timeout: 120
    parallel: false
    max_concurrent: 10
    fail_fast: true

connection_pool:
  max_connections: 1000
  min_connections: 10
  idle_timeout: 300
  max_lifetime: 3600

memory:
  max_heap_size: 1073741824
  gc_interval: 60
  gc_threshold: 0.8

cache:
  enabled: true
  max_size: 104857600
  ttl: 3600
  eviction_policy: "lru"

security:
  enable_tls: true
  tls_cert: "/etc/g3icap/certs/server.crt"
  tls_key: "/etc/g3icap/certs/server.key"
  require_client_cert: false

rate_limiting:
  enabled: true
  requests_per_second: 100
  burst_size: 200
  per_client: true

monitoring:
  prometheus:
    enabled: true
    port: 9090
    path: "/metrics"
  health_check:
    enabled: true
    port: 8080
    path: "/health"

stats:
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
  emit_interval: 10
  buffer_size: 1024
  udp_enabled: true
  tcp_enabled: false
  tags:
    daemon_group: "g3icap"
    environment: "production"
```

**Features**:
- ✅ All configuration options
- ✅ Advanced module management
- ✅ Complex pipeline configurations
- ✅ Performance tuning
- ✅ Security settings
- ✅ Monitoring and metrics
- ✅ Rate limiting
- ✅ Caching and memory management

## Configuration Sections

### Server Configuration

#### Basic
```yaml
server:
  host: "127.0.0.1"
  port: 1344
```

#### Standard
```yaml
server:
  host: "127.0.0.1"
  port: 1344
  max_connections: 1000
```

#### Advanced
```yaml
server:
  host: "127.0.0.1"
  port: 1344
  metrics_port: 9090
  max_connections: 1000
  connection_timeout: 30
  read_timeout: 60
  write_timeout: 60
```

### Services Configuration

#### Basic
```yaml
services:
  - name: "echo"
    path: "/echo"
    module: "echo"
```

#### Standard
```yaml
services:
  - name: "echo"
    path: "/echo"
    module: "echo"
    methods: ["REQMOD", "RESPMOD", "OPTIONS"]
```

#### Advanced
```yaml
services:
  - name: "echo"
    path: "/echo"
    module: "echo"
    methods: ["REQMOD", "RESPMOD", "OPTIONS"]
    preview_size: 1024
    timeout: 30
    max_connections: 100
    health_check:
      enabled: true
      interval: 10
      timeout: 5
      retries: 3
    load_balancing: "round_robin"
```

### Pipeline Configuration

#### Basic
```yaml
pipeline:
  stages: ["logging", "echo"]
```

#### Standard
```yaml
pipeline:
  name: "default"
  stages: ["logging", "filter", "scan"]
  timeout: 60
```

#### Advanced
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
    timeout: 120
    parallel: false
    max_concurrent: 10
    fail_fast: true
```

## Quick Start Guide

### 1. Choose Your Configuration Level

**For Development/Testing**:
```bash
cp examples/simple_config.yaml config.yaml
```

**For Production**:
```bash
cp examples/modular_config.yaml config.yaml
```

**For Complex Deployments**:
```bash
cp examples/advanced_config.yaml config.yaml
```

### 2. Customize Your Configuration

Edit the configuration file to match your needs:

```yaml
# Change server settings
server:
  host: "0.0.0.0"  # Listen on all interfaces
  port: 1344

# Add your services
services:
  - name: "my_service"
    path: "/my_service"
    module: "my_module"
    methods: ["REQMOD", "RESPMOD"]
    config:
      custom_setting: "value"
```

### 3. Run G3ICAP

```bash
# Run with your configuration
cargo run -- --config config.yaml

# Or run the example
cargo run --example modular_server
```

## Configuration Validation

G3ICAP validates your configuration on startup and will report any errors:

```bash
# Validate configuration without starting server
cargo run -- --config config.yaml --validate

# Test configuration
cargo run -- --config config.yaml --test-config
```

## Best Practices

### 1. Start Simple
- Begin with the basic configuration
- Add complexity only when needed
- Test each change thoroughly

### 2. Use Environment Variables
```yaml
server:
  host: "${G3ICAP_HOST:127.0.0.1}"
  port: "${G3ICAP_PORT:1344}"
```

### 3. Separate Concerns
- Use different config files for different environments
- Keep sensitive data in environment variables
- Document your configuration choices

### 4. Monitor and Tune
- Start with default performance settings
- Monitor metrics and adjust as needed
- Use health checks to ensure reliability

## Troubleshooting

### Common Issues

#### 1. Configuration Not Found
```
Error: Configuration file not found: config.yaml
```
**Solution**: Ensure the config file exists and path is correct.

#### 2. Invalid YAML
```
Error: Invalid YAML syntax at line X
```
**Solution**: Check YAML syntax and indentation.

#### 3. Service Not Found
```
Error: Service not found: echo
```
**Solution**: Ensure the service is defined in the services section.

#### 4. Module Not Found
```
Error: Module not found: echo
```
**Solution**: Ensure the module is available and properly configured.

### Debug Configuration

```bash
# Enable debug logging
RUST_LOG=debug cargo run -- --config config.yaml

# Validate configuration
cargo run -- --config config.yaml --validate

# Test configuration
cargo run -- --config config.yaml --test-config
```

## Conclusion

G3ICAP's configuration system is designed to be flexible and easy to use. Start with the basic configuration and add complexity as needed. The three levels of configuration provide a clear path from simple setup to complex production deployments.

For more information, see:
- [Modular Architecture Guide](MODULAR_ARCHITECTURE.md)
- [Implementation Guide](IMPLEMENTATION_GUIDE.md)
- [API Reference](API_REFERENCE.md)
