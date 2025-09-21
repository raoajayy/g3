# G3ICAP Examples

This directory contains examples and configuration files for G3ICAP.

## Configuration Files

### 1. **simple_config.yaml** - Basic Configuration
Minimal configuration for quick start and development.

```yaml
server:
  host: "127.0.0.1"
  port: 1344

services:
  - name: "echo"
    path: "/echo"
    module: "echo"

pipeline:
  stages: ["logging", "echo"]

stats:
  enabled: true
```

**Use when**:
- Getting started with G3ICAP
- Development and testing
- Simple deployments

### 2. **modular_config.yaml** - Standard Configuration
Production-ready configuration with essential features.

```yaml
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
  stages: ["logging", "filter", "scan"]
  timeout: 60

stats:
  enabled: true
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
```

**Use when**:
- Production deployments
- Need custom service configurations
- Want pipeline processing
- Need statistics collection

### 3. **advanced_config.yaml** - Advanced Configuration
Full configuration with all options for complex deployments.

**Use when**:
- Complex production environments
- Need advanced features
- Want full control over all settings
- High-performance requirements

## Example Programs

### 1. **simple_server.rs** - Basic Server
Simple ICAP server example with basic functionality.

```bash
cargo run --example simple_server
```

### 2. **modular_server.rs** - Modular Server
Demonstrates the modular architecture with services and pipelines.

```bash
cargo run --example modular_server
```

### 3. **test_client.rs** - Test Client
ICAP client for testing the server.

```bash
cargo run --example test_client
```

## Quick Start

### 1. Choose Your Configuration

```bash
# For development
cp examples/simple_config.yaml config.yaml

# For production
cp examples/modular_config.yaml config.yaml

# For complex deployments
cp examples/advanced_config.yaml config.yaml
```

### 2. Run G3ICAP

```bash
# Run with your configuration
cargo run -- --config config.yaml

# Or run an example
cargo run --example modular_server
```

### 3. Test the Server

```bash
# Run test client
cargo run --example test_client

# Run tests
cargo test
```

## Configuration Levels

| Level | File | Use Case | Complexity |
|-------|------|----------|------------|
| Basic | `simple_config.yaml` | Development, Testing | Low |
| Standard | `modular_config.yaml` | Production | Medium |
| Advanced | `advanced_config.yaml` | Complex Deployments | High |

## Customization

### Adding Services

```yaml
services:
  - name: "my_service"
    path: "/my_service"
    module: "my_module"
    methods: ["REQMOD", "RESPMOD"]
    config:
      custom_setting: "value"
```

### Configuring Pipeline

```yaml
pipeline:
  name: "my_pipeline"
  stages: ["logging", "my_service"]
  timeout: 60
```

### Setting Up Statistics

```yaml
stats:
  enabled: true
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
```

## Troubleshooting

### Common Issues

1. **Configuration not found**
   ```
   Error: Configuration file not found: config.yaml
   ```
   **Solution**: Ensure the config file exists and path is correct.

2. **Invalid YAML**
   ```
   Error: Invalid YAML syntax at line X
   ```
   **Solution**: Check YAML syntax and indentation.

3. **Service not found**
   ```
   Error: Service not found: echo
   ```
   **Solution**: Ensure the service is defined in the services section.

### Debug Configuration

```bash
# Enable debug logging
RUST_LOG=debug cargo run -- --config config.yaml

# Validate configuration
cargo run -- --config config.yaml --validate

# Test configuration
cargo run -- --config config.yaml --test-config
```

## Next Steps

1. **Review Configuration**: Choose the appropriate configuration level
2. **Customize**: Modify the configuration for your needs
3. **Test**: Run the examples and test your configuration
4. **Deploy**: Deploy to your environment
5. **Monitor**: Set up monitoring and metrics collection

For more information, see:
- [Configuration Guide](../docs/CONFIGURATION_GUIDE.md)
- [Modular Architecture Guide](../docs/MODULAR_ARCHITECTURE.md)
- [Implementation Guide](../docs/IMPLEMENTATION_GUIDE.md)
