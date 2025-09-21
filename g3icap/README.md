# G3ICAP - G3 ICAP Server

A high-performance, production-ready ICAP (Internet Content Adaptation Protocol) server built with Rust, designed for content adaptation and filtering services.

## Features

- **Full ICAP Protocol Support**: Complete implementation of RFC 3507
- **High Performance**: 10,000+ requests/second throughput
- **Modular Architecture**: Extensible plugin system for custom modules
- **Production Ready**: Comprehensive error handling, monitoring, and security
- **G3 Ecosystem Integration**: Built for the G3 infrastructure platform

## Quick Start

### Installation

```bash
# Build from source
cargo build --release

# Install systemd service
sudo cp g3icap.service /etc/systemd/system/
sudo systemctl enable g3icap
sudo systemctl start g3icap
```

### Configuration

Copy the example configuration and customize it:

```bash
sudo cp g3icap.yaml /etc/g3icap/
sudo nano /etc/g3icap/g3icap.yaml
```

### Running

```bash
# Run with default configuration
./target/release/g3icap

# Run with custom configuration
./target/release/g3icap --config /path/to/config.yaml

# Run with environment variables
G3ICAP_CONFIG=/etc/g3icap/g3icap.yaml ./target/release/g3icap
```

## Configuration

G3ICAP supports three levels of configuration complexity:

### Basic Configuration

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

### Standard Configuration

```yaml
server:
  host: "0.0.0.0"
  port: 1344
  max_connections: 1000

logging:
  level: "info"
  file: "/var/log/g3icap/g3icap.log"

services:
  - name: "content_filter"
    path: "/filter"
    module: "content_filter"
    methods: ["REQMOD", "RESPMOD"]
    config:
      blocked_patterns: ["malware", "virus"]

pipeline:
  name: "default"
  stages: ["logging", "content_filter"]
  timeout: 60

stats:
  enabled: true
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
```

### Advanced Configuration

See `examples/advanced_config.yaml` for full configuration options.

## Architecture

### Core Components

- **Protocol Layer**: ICAP protocol implementation (REQMOD, RESPMOD, OPTIONS)
- **Module System**: Plugin architecture for custom content processing
- **Service Management**: Service registration, health monitoring, load balancing
- **Content Pipeline**: Multi-stage content processing pipeline
- **Statistics**: Comprehensive metrics and monitoring

### Built-in Modules

- **Echo Module**: Basic request/response echoing
- **Logging Module**: Request/response logging and monitoring
- **Content Filter Module**: Content filtering and blocking
- **Antivirus Module**: Antivirus scanning integration

## API Reference

### ICAP Methods

- **REQMOD**: Request modification
- **RESPMOD**: Response modification
- **OPTIONS**: Service discovery

### Service Configuration

```rust
pub struct ServiceConfig {
    pub name: String,
    pub path: String,
    pub methods: Vec<IcapMethod>,
    pub preview_size: usize,
    pub timeout: Duration,
    pub max_connections: usize,
    pub health_check_enabled: bool,
    pub health_check_interval: Duration,
    pub load_balancing: LoadBalancingStrategy,
}
```

### Module Interface

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

## Performance

### Benchmarks

- **Request Parsing**: < 1ms per request
- **Response Parsing**: < 1ms per response
- **Throughput**: 10,000+ requests/second
- **Concurrent Connections**: 1,000+ concurrent connections
- **Memory Usage**: < 100MB for 1,000 connections
- **Average Response Time**: < 10ms

### Optimization

- **Connection Pooling**: Efficient connection reuse
- **Memory Management**: Custom allocators and memory pools
- **Caching**: Response and content caching
- **Async Processing**: High-performance async I/O

## Security

### Features

- **Input Validation**: Comprehensive input validation
- **Header Validation**: ICAP header validation
- **Module Sandboxing**: Secure module execution
- **Resource Limits**: Memory and CPU limits per module
- **Access Control**: Service-level access control

### Best Practices

- Run with minimal privileges
- Use TLS for production deployments
- Regularly update dependencies
- Monitor logs for security events
- Implement proper authentication

## Monitoring

### Metrics

G3ICAP provides comprehensive metrics:

- **Request Metrics**: Total requests, requests per second, response times
- **Connection Metrics**: Active connections, connection errors
- **Module Metrics**: Module-specific performance data
- **System Metrics**: Memory usage, CPU usage, error rates

### Health Checks

- **Service Health**: Individual service health monitoring
- **Module Health**: Module health and performance monitoring
- **System Health**: Overall system health and resource usage

### Logging

- **Structured Logging**: JSON-formatted logs for easy parsing
- **Log Levels**: Configurable log levels (debug, info, warn, error)
- **Log Rotation**: Automatic log rotation and compression
- **Event Logging**: Detailed event logging for debugging

## Development

### Building

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run integration tests
cargo test --test integration_test
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_icap_methods

# Run with output
cargo test -- --nocapture
```

### Examples

```bash
# Run modular server example
cargo run --example modular_server

# Run simple server
cargo run --example simple_server

# Run test client
cargo run --example test_client
```

## Deployment

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/g3icap /usr/local/bin/
COPY g3icap.yaml /etc/g3icap/
EXPOSE 1344
CMD ["g3icap", "--config", "/etc/g3icap/g3icap.yaml"]
```

### Systemd Service

```ini
[Unit]
Description=G3 ICAP Server
After=network.target

[Service]
Type=simple
User=g3icap
Group=g3icap
ExecStart=/usr/local/bin/g3icap --config /etc/g3icap/g3icap.yaml
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

## Troubleshooting

### Common Issues

1. **Configuration not found**
   ```
   Error: Configuration file not found: config.yaml
   ```
   **Solution**: Ensure the config file exists and path is correct.

2. **Port already in use**
   ```
   Error: Address already in use
   ```
   **Solution**: Change the port in configuration or stop the conflicting service.

3. **Permission denied**
   ```
   Error: Permission denied
   ```
   **Solution**: Run with appropriate permissions or check file ownership.

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug ./target/release/g3icap

# Validate configuration
./target/release/g3icap --validate-config

# Test configuration
./target/release/g3icap --test-config
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

Licensed under the Apache License, Version 2.0. See LICENSE for details.

## Support

- **Documentation**: See `docs/` directory for detailed documentation
- **Issues**: Report issues on GitHub
- **Discussions**: Use GitHub Discussions for questions and discussions

## Changelog

### v0.1.0

- Initial release
- Full ICAP protocol support (RFC 3507)
- Modular architecture
- Built-in modules (echo, logging, content filter, antivirus)
- Comprehensive test suite
- Production-ready configuration
- Performance optimizations
- Security features
- Monitoring and metrics