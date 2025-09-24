# G3ICAP Client Usage Guide

This guide provides comprehensive examples and documentation for using the G3ICAP client libraries in Python, Go, and JavaScript/Node.js.

## Table of Contents

1. [Python Client](#python-client)
2. [Go Client](#go-client)
3. [JavaScript/Node.js Client](#javascriptnodejs-client)
4. [Configuration](#configuration)
5. [Authentication](#authentication)
6. [Error Handling](#error-handling)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## Python Client

### Installation

```bash
cd examples/clients/python
pip install -r requirements.txt
```

### Basic Usage

```python
from icap_client import IcapClient, IcapConfig, HttpRequest, HttpResponse

# Create configuration
config = IcapConfig(
    host="127.0.0.1",
    port=1344,
    timeout=30,
    retries=3,
    logging_level="INFO"
)

# Create client
client = IcapClient(config)

# Send OPTIONS request
response = client.options()
print(f"Server: {response.headers.get('Server')}")
print(f"Methods: {response.headers.get('Methods')}")

# Send REQMOD request
http_request = HttpRequest(
    method="GET",
    uri="/",
    version="HTTP/1.1",
    headers={"Host": "example.com"},
    body=b""
)

response = client.reqmod(http_request)
print(f"REQMOD Response: {response.status_code} {response.reason}")

# Send RESPMOD request
http_response = HttpResponse(
    version="HTTP/1.1",
    status_code=200,
    reason="OK",
    headers={"Content-Type": "text/html"},
    body=b"<html><body>Hello World</body></html>"
)

response = client.respmod(http_response)
print(f"RESPMOD Response: {response.status_code} {response.reason}")

# Health check
health = client.health_check()
print(f"Health Status: {health['status']}")

# Close client
client.close()
```

### Advanced Usage with Authentication

```python
from icap_client import IcapClient, IcapConfig, AuthenticationMethod

# Create configuration with authentication
config = IcapConfig(
    host="127.0.0.1",
    port=1344,
    timeout=30,
    retries=3,
    authentication={
        "method": AuthenticationMethod.BASIC,
        "username": "testuser",
        "password": "testpass"
    }
)

client = IcapClient(config)

# Use client as before
response = client.options()
```

### Configuration from File

```python
from icap_client import IcapClient, IcapConfig

# Load configuration from YAML file
config = IcapConfig.from_file("config.yaml")
client = IcapClient(config)
```

### Error Handling

```python
from icap_client import IcapClient, IcapConfig, IcapError

try:
    client = IcapClient(config)
    response = client.options()
except IcapError as e:
    print(f"ICAP Error: {e.message} (Code: {e.code})")
except Exception as e:
    print(f"Unexpected error: {e}")
```

## Go Client

### Installation

```bash
cd examples/clients/go
go mod tidy
```

### Basic Usage

```go
package main

import (
    "context"
    "fmt"
    "log"
    "time"
)

func main() {
    // Create configuration
    config := &IcapConfig{
        Host:               "127.0.0.1",
        Port:               1344,
        Timeout:            30 * time.Second,
        Retries:            3,
        RetryDelay:         time.Second,
        MaxRetryDelay:      60 * time.Second,
        BackoffFactor:      2.0,
        ConnectionPoolSize: 10,
        KeepAlive:          true,
        VerifySSL:          true,
        LoggingLevel:       "INFO",
        MetricsEnabled:     true,
    }

    // Create client
    client := NewIcapClient(config)
    defer client.Close()

    ctx := context.Background()

    // Send OPTIONS request
    response, err := client.Options(ctx)
    if err != nil {
        log.Fatalf("OPTIONS request failed: %v", err)
    }
    fmt.Printf("Server: %s\n", response.Headers["Server"])
    fmt.Printf("Methods: %s\n", response.Headers["Methods"])

    // Send REQMOD request
    httpRequest := &HttpRequest{
        Method:  "GET",
        URI:     "/",
        Version: "HTTP/1.1",
        Headers: map[string]string{
            "Host": "example.com",
        },
    }

    response, err = client.Reqmod(ctx, httpRequest)
    if err != nil {
        log.Fatalf("REQMOD request failed: %v", err)
    }
    fmt.Printf("REQMOD Response: %d %s\n", response.StatusCode, response.Reason)

    // Send RESPMOD request
    httpResponse := &HttpResponse{
        Version:    "HTTP/1.1",
        StatusCode: 200,
        Reason:     "OK",
        Headers: map[string]string{
            "Content-Type": "text/html",
        },
        Body: []byte("<html><body>Hello World</body></html>"),
    }

    response, err = client.Respmod(ctx, httpResponse)
    if err != nil {
        log.Fatalf("RESPMOD request failed: %v", err)
    }
    fmt.Printf("RESPMOD Response: %d %s\n", response.StatusCode, response.Reason)

    // Health check
    health, err := client.HealthCheck(ctx)
    if err != nil {
        log.Fatalf("Health check failed: %v", err)
    }
    fmt.Printf("Health Status: %s\n", health["status"])
}
```

### Advanced Usage with Authentication

```go
// Create configuration with authentication
config := &IcapConfig{
    Host:               "127.0.0.1",
    Port:               1344,
    Timeout:            30 * time.Second,
    Retries:            3,
    ConnectionPoolSize: 10,
    KeepAlive:          true,
    VerifySSL:          true,
    LoggingLevel:       "INFO",
    MetricsEnabled:     true,
    Authentication: map[string]string{
        "method":   "basic",
        "username": "testuser",
        "password": "testpass",
    },
}

client := NewIcapClient(config)
defer client.Close()
```

### Configuration from File

```go
// Load configuration from YAML file
config, err := LoadConfig("config.yaml")
if err != nil {
    log.Fatalf("Failed to load config: %v", err)
}

client := NewIcapClient(config)
defer client.Close()
```

### Error Handling

```go
response, err := client.Options(ctx)
if err != nil {
    if icapErr, ok := err.(*IcapError); ok {
        log.Printf("ICAP Error: %s (Code: %d)", icapErr.Message, icapErr.Code)
    } else {
        log.Printf("Unexpected error: %v", err)
    }
    return
}
```

## JavaScript/Node.js Client

### Installation

```bash
cd examples/clients/javascript
npm install
```

### Basic Usage

```javascript
const { IcapClient, IcapConfig, HttpRequest, HttpResponse } = require('./icap_client');

// Create configuration
const config = new IcapConfig({
    host: '127.0.0.1',
    port: 1344,
    timeout: 30000,
    retries: 3,
    loggingLevel: 'info'
});

// Create client
const client = new IcapClient(config);

async function main() {
    try {
        // Send OPTIONS request
        const response = await client.options();
        console.log(`Server: ${response.headers.Server}`);
        console.log(`Methods: ${response.headers.Methods}`);

        // Send REQMOD request
        const httpRequest = new HttpRequest({
            method: 'GET',
            uri: '/',
            version: 'HTTP/1.1',
            headers: { 'Host': 'example.com' },
            body: Buffer.alloc(0)
        });

        const reqmodResponse = await client.reqmod(httpRequest);
        console.log(`REQMOD Response: ${reqmodResponse.statusCode} ${reqmodResponse.reason}`);

        // Send RESPMOD request
        const httpResponse = new HttpResponse({
            version: 'HTTP/1.1',
            statusCode: 200,
            reason: 'OK',
            headers: { 'Content-Type': 'text/html' },
            body: Buffer.from('<html><body>Hello World</body></html>')
        });

        const respmodResponse = await client.respmod(httpResponse);
        console.log(`RESPMOD Response: ${respmodResponse.statusCode} ${respmodResponse.reason}`);

        // Health check
        const health = await client.healthCheck();
        console.log(`Health Status: ${health.status}`);

    } catch (error) {
        console.error('Error:', error.message);
    } finally {
        client.close();
    }
}

main();
```

### Advanced Usage with Authentication

```javascript
// Create configuration with authentication
const config = new IcapConfig({
    host: '127.0.0.1',
    port: 1344,
    timeout: 30000,
    retries: 3,
    authentication: {
        method: 'basic',
        username: 'testuser',
        password: 'testpass'
    }
});

const client = new IcapClient(config);
```

### Configuration from File

```javascript
// Load configuration from YAML file
const config = IcapConfig.fromFile('config.yaml');
const client = new IcapClient(config);
```

### Error Handling

```javascript
try {
    const response = await client.options();
} catch (error) {
    if (error instanceof IcapError) {
        console.error(`ICAP Error: ${error.message} (Code: ${error.code})`);
    } else {
        console.error(`Unexpected error: ${error.message}`);
    }
}
```

## Configuration

### Configuration Options

All clients support the following configuration options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `host` | string | `127.0.0.1` | ICAP server hostname |
| `port` | integer | `1344` | ICAP server port |
| `timeout` | integer/duration | `30s` | Request timeout |
| `retries` | integer | `3` | Number of retry attempts |
| `retry_delay` | integer/duration | `1s` | Initial retry delay |
| `max_retry_delay` | integer/duration | `60s` | Maximum retry delay |
| `backoff_factor` | float | `2.0` | Exponential backoff factor |
| `connection_pool_size` | integer | `10` | Connection pool size |
| `keep_alive` | boolean | `true` | Enable keep-alive connections |
| `verify_ssl` | boolean | `true` | Verify SSL certificates |
| `logging_level` | string | `INFO` | Logging level |
| `metrics_enabled` | boolean | `true` | Enable metrics collection |

### YAML Configuration Example

```yaml
host: 127.0.0.1
port: 1344
timeout: 30s
retries: 3
retry_delay: 1s
max_retry_delay: 60s
backoff_factor: 2.0
connection_pool_size: 10
keep_alive: true
verify_ssl: true
logging_level: INFO
metrics_enabled: true
authentication:
  method: basic
  username: testuser
  password: testpass
```

## Authentication

### Supported Authentication Methods

1. **None** - No authentication
2. **Basic** - HTTP Basic Authentication
3. **Bearer** - Bearer token authentication
4. **JWT** - JSON Web Token authentication
5. **API Key** - Custom API key authentication

### Authentication Examples

#### Basic Authentication

```python
# Python
config = IcapConfig(
    authentication={
        "method": AuthenticationMethod.BASIC,
        "username": "testuser",
        "password": "testpass"
    }
)
```

```go
// Go
config := &IcapConfig{
    Authentication: map[string]string{
        "method":   "basic",
        "username": "testuser",
        "password": "testpass",
    },
}
```

```javascript
// JavaScript
const config = new IcapConfig({
    authentication: {
        method: 'basic',
        username: 'testuser',
        password: 'testpass'
    }
});
```

#### Bearer Token Authentication

```python
# Python
config = IcapConfig(
    authentication={
        "method": AuthenticationMethod.BEARER,
        "token": "your-bearer-token"
    }
)
```

```go
// Go
config := &IcapConfig{
    Authentication: map[string]string{
        "method": "bearer",
        "token":  "your-bearer-token",
    },
}
```

```javascript
// JavaScript
const config = new IcapConfig({
    authentication: {
        method: 'bearer',
        token: 'your-bearer-token'
    }
});
```

#### API Key Authentication

```python
# Python
config = IcapConfig(
    authentication={
        "method": AuthenticationMethod.API_KEY,
        "api_key": "your-api-key",
        "header_name": "X-API-Key"
    }
)
```

```go
// Go
config := &IcapConfig{
    Authentication: map[string]string{
        "method":    "api_key",
        "api_key":   "your-api-key",
        "header_name": "X-API-Key",
    },
}
```

```javascript
// JavaScript
const config = new IcapConfig({
    authentication: {
        method: 'api_key',
        api_key: 'your-api-key',
        header_name: 'X-API-Key'
    }
});
```

## Error Handling

### Error Types

All clients provide structured error handling:

- **IcapError** - ICAP-specific errors
- **NetworkError** - Network connectivity issues
- **TimeoutError** - Request timeout errors
- **AuthenticationError** - Authentication failures

### Error Handling Examples

#### Python

```python
from icap_client import IcapError, NetworkError, TimeoutError

try:
    response = client.options()
except IcapError as e:
    print(f"ICAP Error: {e.message} (Code: {e.code})")
except NetworkError as e:
    print(f"Network Error: {e.message}")
except TimeoutError as e:
    print(f"Timeout Error: {e.message}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

#### Go

```go
response, err := client.Options(ctx)
if err != nil {
    switch e := err.(type) {
    case *IcapError:
        log.Printf("ICAP Error: %s (Code: %d)", e.Message, e.Code)
    case *NetworkError:
        log.Printf("Network Error: %s", e.Message)
    case *TimeoutError:
        log.Printf("Timeout Error: %s", e.Message)
    default:
        log.Printf("Unexpected error: %v", err)
    }
}
```

#### JavaScript

```javascript
try {
    const response = await client.options();
} catch (error) {
    if (error instanceof IcapError) {
        console.error(`ICAP Error: ${error.message} (Code: ${error.code})`);
    } else if (error instanceof NetworkError) {
        console.error(`Network Error: ${error.message}`);
    } else if (error instanceof TimeoutError) {
        console.error(`Timeout Error: ${error.message}`);
    } else {
        console.error(`Unexpected error: ${error.message}`);
    }
}
```

## Best Practices

### 1. Connection Management

- Use connection pooling for better performance
- Implement proper connection cleanup
- Monitor connection pool metrics

### 2. Error Handling

- Always implement proper error handling
- Use structured error types
- Implement retry logic with exponential backoff
- Log errors with appropriate context

### 3. Performance

- Enable metrics collection for monitoring
- Use appropriate timeout values
- Implement request/response buffering
- Monitor memory usage

### 4. Security

- Use secure authentication methods
- Validate SSL certificates in production
- Implement proper credential management
- Use secure communication channels

### 5. Monitoring

- Enable health checks
- Monitor response times
- Track error rates
- Set up alerting

## Troubleshooting

### Common Issues

#### 1. Connection Refused

**Problem**: `Connection refused` error when connecting to ICAP server.

**Solutions**:
- Verify ICAP server is running
- Check host and port configuration
- Ensure firewall allows connections
- Verify network connectivity

#### 2. Authentication Failures

**Problem**: `401 Unauthorized` or `403 Forbidden` errors.

**Solutions**:
- Verify authentication credentials
- Check authentication method configuration
- Ensure user has proper permissions
- Verify authentication headers

#### 3. Timeout Errors

**Problem**: Request timeouts occur frequently.

**Solutions**:
- Increase timeout values
- Check network latency
- Verify server performance
- Implement retry logic

#### 4. SSL/TLS Issues

**Problem**: SSL certificate verification failures.

**Solutions**:
- Verify SSL certificate validity
- Check certificate chain
- Use proper CA certificates
- Consider disabling SSL verification for testing

### Debugging

#### Enable Debug Logging

```python
# Python
config = IcapConfig(logging_level="DEBUG")
```

```go
// Go
config := &IcapConfig{
    LoggingLevel: "DEBUG",
}
```

```javascript
// JavaScript
const config = new IcapConfig({
    loggingLevel: 'debug'
});
```

#### Monitor Metrics

```python
# Python
if config.metrics_enabled:
    metrics = client.get_metrics()
    print(f"Requests: {metrics['requests_total']}")
    print(f"Success Rate: {metrics['success_rate']}")
```

```go
// Go
if config.MetricsEnabled {
    metrics := client.GetMetrics()
    log.Printf("Requests: %d", metrics.RequestsTotal)
    log.Printf("Success Rate: %.2f", metrics.SuccessRate)
}
```

```javascript
// JavaScript
if (config.metricsEnabled) {
    const metrics = client.getMetrics();
    console.log(`Requests: ${metrics.requestsTotal}`);
    console.log(`Success Rate: ${metrics.successRate}`);
}
```

### Getting Help

1. Check the logs for detailed error messages
2. Verify configuration settings
3. Test with a simple OPTIONS request
4. Check ICAP server status
5. Review network connectivity
6. Consult the G3ICAP documentation

For additional support, please refer to the G3ICAP project documentation or create an issue in the project repository.
