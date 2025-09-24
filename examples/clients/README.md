# G3ICAP Client Examples

This directory contains comprehensive ICAP client examples in multiple programming languages, demonstrating how to interact with G3ICAP servers.

## Table of Contents

1. [Python Client](#python-client)
2. [Go Client](#go-client)
3. [JavaScript/Node.js Client](#javascriptnodejs-client)
4. [Rust Client](#rust-client)
5. [Java Client](#java-client)
6. [C++ Client](#c-client)
7. [Usage Examples](#usage-examples)
8. [Testing](#testing)

## Overview

These client examples demonstrate:

- **REQMOD Method** - Request modification
- **RESPMOD Method** - Response modification  
- **OPTIONS Method** - Service discovery
- **Authentication** - Basic, Bearer, JWT, API Key
- **Error Handling** - Comprehensive error management
- **Performance** - Connection pooling and optimization
- **Monitoring** - Health checks and metrics

## Prerequisites

### G3ICAP Server

Start the G3ICAP server:

```bash
# Start server with test configuration
cd /Users/ajaykumar/Documents/GitHub/Arcus/g3
RUST_LOG=debug ./target/debug/g3icap -c config/g3icap/g3icap.yaml
```

### Test Configuration

The examples use the following server configuration:

- **Host**: 127.0.0.1
- **Port**: 1344
- **Protocol**: ICAP
- **Authentication**: Optional (Basic, Bearer, JWT, API Key)
- **Auditors**: Content filter, Antivirus, Logging

## Quick Start

### 1. Python Client

```bash
cd python
pip install -r requirements.txt
python icap_client.py
```

### 2. Go Client

```bash
cd go
go mod tidy
go run icap_client.go
```

### 3. JavaScript Client

```bash
cd javascript
npm install
node icap_client.js
```

### 4. Rust Client

```bash
cd rust
cargo run
```

## Client Features

### Core Functionality

- ✅ **REQMOD Support** - Request modification
- ✅ **RESPMOD Support** - Response modification
- ✅ **OPTIONS Support** - Service discovery
- ✅ **Authentication** - Multiple auth methods
- ✅ **Error Handling** - Comprehensive error management
- ✅ **Connection Pooling** - Efficient connection reuse
- ✅ **Retry Logic** - Automatic retry with backoff
- ✅ **Logging** - Structured logging support
- ✅ **Metrics** - Performance monitoring
- ✅ **Health Checks** - Server health monitoring

### Advanced Features

- ✅ **Async Support** - Asynchronous operations
- ✅ **Streaming** - Large message handling
- ✅ **Compression** - Request/response compression
- ✅ **Caching** - Response caching
- ✅ **Load Balancing** - Multiple server support
- ✅ **Circuit Breaker** - Fault tolerance
- ✅ **Rate Limiting** - Client-side rate limiting
- ✅ **Tracing** - Distributed tracing support

## Testing

### Run All Tests

```bash
# Run all client tests
./test_all_clients.sh
```

### Individual Client Tests

```bash
# Python tests
cd python && python -m pytest

# Go tests
cd go && go test ./...

# JavaScript tests
cd javascript && npm test

# Rust tests
cd rust && cargo test
```

### Integration Tests

```bash
# Run integration tests against G3ICAP server
./integration_tests.sh
```

## Performance Benchmarks

### Throughput Tests

```bash
# Run throughput benchmarks
./benchmark_throughput.sh
```

### Latency Tests

```bash
# Run latency benchmarks
./benchmark_latency.sh
```

### Load Tests

```bash
# Run load tests
./benchmark_load.sh
```

## Configuration

### Client Configuration

Each client supports configuration via:

- **Environment Variables** - `ICAP_HOST`, `ICAP_PORT`, etc.
- **Configuration Files** - JSON, YAML, TOML
- **Command Line Arguments** - Runtime configuration
- **Code Configuration** - Programmatic configuration

### Example Configuration

```yaml
# config.yaml
server:
  host: "127.0.0.1"
  port: 1344
  timeout: 30
  retries: 3

authentication:
  method: "basic"
  username: "admin"
  password: "secret123"

connection:
  pool_size: 10
  keep_alive: true
  max_connections: 100

logging:
  level: "info"
  format: "json"
  file: "icap_client.log"

metrics:
  enabled: true
  interval: 60
  endpoint: "http://localhost:9090/metrics"
```

## Error Handling

### Common Error Scenarios

1. **Connection Errors** - Network connectivity issues
2. **Authentication Errors** - Invalid credentials
3. **Timeout Errors** - Request timeouts
4. **Protocol Errors** - Invalid ICAP messages
5. **Server Errors** - Internal server errors
6. **Rate Limit Errors** - Too many requests

### Error Recovery

- **Automatic Retry** - Exponential backoff
- **Circuit Breaker** - Prevent cascading failures
- **Fallback** - Alternative server selection
- **Graceful Degradation** - Continue with reduced functionality

## Monitoring and Observability

### Metrics

- **Request Count** - Total requests sent
- **Response Time** - Request/response latency
- **Error Rate** - Failed request percentage
- **Connection Pool** - Pool utilization
- **Throughput** - Requests per second

### Health Checks

- **Server Health** - ICAP server status
- **Client Health** - Client application status
- **Dependencies** - External service health

### Logging

- **Structured Logging** - JSON format
- **Log Levels** - DEBUG, INFO, WARN, ERROR
- **Correlation IDs** - Request tracing
- **Performance Logs** - Timing information

## Best Practices

### 1. Connection Management

- Use connection pooling
- Implement keep-alive
- Handle connection failures gracefully
- Monitor connection health

### 2. Error Handling

- Implement retry logic
- Use circuit breakers
- Log errors appropriately
- Provide meaningful error messages

### 3. Performance

- Use async operations
- Implement request batching
- Monitor performance metrics
- Optimize for your use case

### 4. Security

- Use secure authentication
- Validate server certificates
- Implement rate limiting
- Monitor for suspicious activity

### 5. Monitoring

- Implement comprehensive logging
- Use structured logging
- Monitor key metrics
- Set up alerting

## Troubleshooting

### Common Issues

1. **Connection Refused** - Server not running
2. **Authentication Failed** - Invalid credentials
3. **Timeout Errors** - Server overloaded
4. **Protocol Errors** - Invalid ICAP format
5. **Memory Issues** - Connection pool too large

### Debug Mode

Enable debug logging:

```bash
# Python
export ICAP_LOG_LEVEL=DEBUG
python icap_client.py

# Go
export ICAP_LOG_LEVEL=debug
go run icap_client.go

# JavaScript
export ICAP_LOG_LEVEL=debug
node icap_client.js
```

### Log Analysis

```bash
# Analyze client logs
grep "ERROR" icap_client.log | tail -20
grep "WARN" icap_client.log | tail -20
grep "performance" icap_client.log | tail -20
```

## Contributing

### Adding New Clients

1. Create a new directory for your language
2. Implement the core ICAP client interface
3. Add comprehensive tests
4. Update documentation
5. Add to CI/CD pipeline

### Client Interface

All clients should implement:

```rust
trait IcapClient {
    async fn reqmod(&self, request: HttpRequest) -> Result<IcapResponse, IcapError>;
    async fn respmod(&self, response: HttpResponse) -> Result<IcapResponse, IcapError>;
    async fn options(&self) -> Result<IcapResponse, IcapError>;
    async fn health_check(&self) -> Result<HealthStatus, IcapError>;
}
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](../../LICENSE) file for details.

## Support

For questions and support:

- **Documentation**: [G3ICAP Docs](../../docs/)
- **Issues**: [GitHub Issues](https://github.com/ByteDance/Arcus/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ByteDance/Arcus/discussions)
- **Email**: support@g3icap.com

## References

- [RFC 3507](https://tools.ietf.org/html/rfc3507) - Internet Content Adaptation Protocol
- [G3ICAP Server](../../g3icap/)
- [ICAP Protocol Guide](../../docs/icap-protocol.md)
- [API Documentation](../../docs/api/)
