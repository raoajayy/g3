# G3ICAP RFC 3507 Usage Examples

This document provides comprehensive usage examples for G3ICAP, demonstrating RFC 3507 compliance and practical implementation patterns.

## Table of Contents

1. [Basic ICAP Server Setup](#basic-icap-server-setup)
2. [REQMOD Method Examples](#reqmod-method-examples)
3. [RESPMOD Method Examples](#respmod-method-examples)
4. [OPTIONS Method Examples](#options-method-examples)
5. [Error Handling Examples](#error-handling-examples)
6. [Security Configuration Examples](#security-configuration-examples)
7. [Performance Tuning Examples](#performance-tuning-examples)
8. [Monitoring and Observability Examples](#monitoring-and-observability-examples)
9. [Client Integration Examples](#client-integration-examples)
10. [Advanced Configuration Examples](#advanced-configuration-examples)

## Basic ICAP Server Setup

### 1. Simple Server Configuration

```yaml
# config/g3icap/g3icap.yaml
server:
  listen_address: "0.0.0.0:1344"
  max_connections: 1000
  request_timeout_secs: 30
  keep_alive_timeout_secs: 60

auditors:
  - name: "content_filter"
    type: "content_filter"
    enabled: true
    blocked_domains:
      - "malicious-site.com"
      - "phishing-site.com"
    blocked_urls:
      - ".*/malware/.*"
      - ".*/virus/.*"
    log_blocked_requests: true
    log_allowed_requests: false

  - name: "antivirus_scanner"
    type: "antivirus"
    enabled: true
    scan_timeout_secs: 10
    log_clean_requests: false
    log_infected_requests: true

  - name: "audit_logger"
    type: "logging"
    enabled: true
    log_level: "info"
    log_format: "json"
```

### 2. Starting the Server

```bash
# Basic startup
./target/debug/g3icap -c config/g3icap/g3icap.yaml

# With debug logging
RUST_LOG=debug ./target/debug/g3icap -c config/g3icap/g3icap.yaml

# With custom log file
RUST_LOG=debug ./target/debug/g3icap -c config/g3icap/g3icap.yaml 2>&1 | tee g3icap.log
```

### 3. Health Check

```bash
# Check server health
curl -X GET http://localhost:1344/health

# Check readiness
curl -X GET http://localhost:1344/ready

# Check liveness
curl -X GET http://localhost:1344/live
```

## REQMOD Method Examples

### 1. Basic REQMOD Request

```bash
# Test REQMOD with curl
curl -X REQMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: req-hdr=0, null-body=75" \
  -H "Allow: 204" \
  --data-binary "GET / HTTP/1.1
Host: example.com
User-Agent: curl/8.7.1
Accept: */*

" \
  icap://127.0.0.1:1344/reqmod
```

### 2. REQMOD with Content Filtering

```bash
# Test blocked domain
curl -X REQMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: req-hdr=0, null-body=85" \
  -H "Allow: 204" \
  --data-binary "GET / HTTP/1.1
Host: malicious-site.com
User-Agent: curl/8.7.1
Accept: */*

" \
  icap://127.0.0.1:1344/reqmod
```

### 3. REQMOD with Large Request

```bash
# Test with large request body
curl -X REQMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: req-hdr=0, req-body=100" \
  -H "Allow: 204" \
  -H "Content-Length: 200" \
  --data-binary "POST /upload HTTP/1.1
Host: example.com
User-Agent: curl/8.7.1
Content-Type: application/octet-stream
Content-Length: 100

$(dd if=/dev/zero bs=100 count=1 2>/dev/null)" \
  icap://127.0.0.1:1344/reqmod
```

## RESPMOD Method Examples

### 1. Basic RESPMOD Request

```bash
# Test RESPMOD with curl
curl -X RESPMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: res-hdr=0, null-body=120" \
  -H "Allow: 204" \
  --data-binary "HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 50
Server: nginx/1.18.0

<html><body>Hello World</body></html>" \
  icap://127.0.0.1:1344/respmod
```

### 2. RESPMOD with Virus Scanning

```bash
# Test virus scanning
curl -X RESPMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: res-hdr=0, res-body=100" \
  -H "Allow: 204" \
  --data-binary "HTTP/1.1 200 OK
Content-Type: application/octet-stream
Content-Length: 50

$(echo -n "X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*")" \
  icap://127.0.0.1:1344/respmod
```

### 3. RESPMOD with Content Modification

```bash
# Test content modification
curl -X RESPMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: res-hdr=0, res-body=100" \
  -H "Allow: 204" \
  --data-binary "HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 100

<html><body><script>alert('XSS')</script>Hello World</body></html>" \
  icap://127.0.0.1:1344/respmod
```

## OPTIONS Method Examples

### 1. Basic OPTIONS Request

```bash
# Get server capabilities
curl -X OPTIONS \
  -H "Host: 127.0.0.1:1344" \
  icap://127.0.0.1:1344/options
```

### 2. OPTIONS with Service Discovery

```bash
# Get specific service information
curl -X OPTIONS \
  -H "Host: 127.0.0.1:1344" \
  -H "Service: content_filter" \
  icap://127.0.0.1:1344/options
```

### 3. OPTIONS with Health Check

```bash
# Check service health
curl -X OPTIONS \
  -H "Host: 127.0.0.1:1344" \
  -H "Service: health" \
  icap://127.0.0.1:1344/options
```

## Error Handling Examples

### 1. Invalid Request Format

```bash
# Test invalid ICAP request
curl -X INVALID \
  -H "Host: 127.0.0.1:1344" \
  icap://127.0.0.1:1344/reqmod
```

### 2. Missing Required Headers

```bash
# Test missing Encapsulated header
curl -X REQMOD \
  -H "Host: 127.0.0.1:1344" \
  --data-binary "GET / HTTP/1.1
Host: example.com

" \
  icap://127.0.0.1:1344/reqmod
```

### 3. Request Timeout

```bash
# Test request timeout
timeout 5 curl -X REQMOD \
  -H "Host: 127.0.0.1:1344" \
  -H "Encapsulated: req-hdr=0, null-body=75" \
  --data-binary "GET / HTTP/1.1
Host: example.com

" \
  icap://127.0.0.1:1344/reqmod
```

## Security Configuration Examples

### 1. Basic Authentication

```yaml
# config/g3icap/g3icap.yaml
security:
  authentication:
    enabled: true
    methods:
      - "basic"
      - "bearer"
    basic:
      users:
        - username: "admin"
          password: "secret123"
        - username: "user"
          password: "password456"
    bearer:
      tokens:
        - "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
        - "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

  authorization:
    enabled: true
    rules:
      - path: "/reqmod"
        methods: ["REQMOD"]
        users: ["admin", "user"]
        permissions: ["read", "write"]
      - path: "/respmod"
        methods: ["RESPMOD"]
        users: ["admin"]
        permissions: ["read", "write", "delete"]

  rate_limiting:
    enabled: true
    requests_per_minute: 100
    burst_size: 20
```

### 2. JWT Authentication

```yaml
# config/g3icap/g3icap.yaml
security:
  authentication:
    enabled: true
    methods:
      - "jwt"
    jwt:
      secret: "your-secret-key"
      algorithm: "HS256"
      issuer: "g3icap"
      audience: "icap-clients"
      expiration: 3600
```

### 3. API Key Authentication

```yaml
# config/g3icap/g3icap.yaml
security:
  authentication:
    enabled: true
    methods:
      - "api_key"
    api_key:
      header: "X-API-Key"
      keys:
        - "api-key-12345"
        - "api-key-67890"
```

## Performance Tuning Examples

### 1. Connection Pooling

```yaml
# config/g3icap/g3icap.yaml
performance:
  connection_pool:
    enabled: true
    max_connections: 1000
    min_connections: 10
    max_idle_time_secs: 300
    connection_timeout_secs: 30
    keep_alive_timeout_secs: 60

  buffer_management:
    enabled: true
    buffer_size: 8192
    max_buffers: 1000
    buffer_timeout_secs: 60

  memory_optimization:
    enabled: true
    gc_interval_secs: 30
    max_memory_mb: 1024
    memory_pool_size: 100
```

### 2. Caching Configuration

```yaml
# config/g3icap/g3icap.yaml
performance:
  caching:
    enabled: true
    cache_type: "lru"
    max_size: 10000
    ttl_secs: 3600
    eviction_policy: "lru"
    compression: true
```

### 3. Load Balancing

```yaml
# config/g3icap/g3icap.yaml
performance:
  load_balancing:
    enabled: true
    algorithm: "round_robin"
    health_check_interval_secs: 30
    max_failures: 3
    backoff_secs: 60
```

## Monitoring and Observability Examples

### 1. Health Check Configuration

```yaml
# config/g3icap/g3icap.yaml
monitoring:
  health_check:
    enabled: true
    interval_secs: 30
    critical_components:
      - "database"
      - "cache"
      - "auditors"
    readiness_threshold_secs: 60

  metrics:
    enabled: true
    push_interval_secs: 15
    exporter_type: "prometheus"
    exporter_url: "http://localhost:9090/metrics"

  tracing:
    enabled: true
    service_name: "g3icap-service"
    exporter_type: "jaeger"
    exporter_url: "http://localhost:14268/api/traces"
    sample_rate: 0.1
```

### 2. Dashboard Configuration

```yaml
# config/g3icap/g3icap.yaml
monitoring:
  dashboard:
    enabled: true
    refresh_interval_secs: 5
    expose_metrics_endpoint: true
    metrics_endpoint_path: "/metrics"
    max_data_points: 100
```

### 3. Alerting Configuration

```yaml
# config/g3icap/g3icap.yaml
monitoring:
  alerts:
    enabled: true
    evaluation_interval_secs: 60
    notification_channels:
      - name: "email"
        channel_type: "email"
        endpoint: "admin@example.com"
        enabled: true
        severity_threshold: "warning"
      - name: "slack"
        channel_type: "slack"
        endpoint: "https://hooks.slack.com/services/..."
        enabled: true
        severity_threshold: "critical"
    alert_rules:
      - id: "high_cpu_usage"
        name: "High CPU Usage"
        description: "CPU usage is above 80%"
        severity: "critical"
        condition:
          type: "threshold"
          metric_name: "cpu_usage"
          operator: "greater_than"
          threshold_value: 80.0
        enabled: true
        notification_channels: ["email", "slack"]
        evaluation_interval_secs: 60
        cooldown_interval_secs: 300
```

## Client Integration Examples

### 1. Python ICAP Client

```python
import socket
import ssl

class IcapClient:
    def __init__(self, host, port):
        self.host = host
        self.port = port
    
    def send_reqmod(self, http_request):
        # Create ICAP request
        icap_request = f"""REQMOD icap://{self.host}:{self.port}/reqmod ICAP/1.0
Host: {self.host}:{self.port}
Encapsulated: req-hdr=0, null-body={len(http_request)}
Allow: 204

{http_request}"""
        
        # Send request
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.host, self.port))
            s.send(icap_request.encode())
            response = s.recv(4096)
            return response.decode()

# Usage
client = IcapClient("127.0.0.1", 1344)
http_request = """GET / HTTP/1.1
Host: example.com
User-Agent: Python-ICAP-Client

"""
response = client.send_reqmod(http_request)
print(response)
```

### 2. Go ICAP Client

```go
package main

import (
    "fmt"
    "net"
    "strings"
)

type IcapClient struct {
    host string
    port string
}

func NewIcapClient(host, port string) *IcapClient {
    return &IcapClient{host: host, port: port}
}

func (c *IcapClient) SendReqmod(httpRequest string) (string, error) {
    // Create ICAP request
    icapRequest := fmt.Sprintf("REQMOD icap://%s:%s/reqmod ICAP/1.0\r\n", c.host, c.port)
    icapRequest += fmt.Sprintf("Host: %s:%s\r\n", c.host, c.port)
    icapRequest += fmt.Sprintf("Encapsulated: req-hdr=0, null-body=%d\r\n", len(httpRequest))
    icapRequest += "Allow: 204\r\n"
    icapRequest += "\r\n"
    icapRequest += httpRequest
    
    // Send request
    conn, err := net.Dial("tcp", c.host+":"+c.port)
    if err != nil {
        return "", err
    }
    defer conn.Close()
    
    _, err = conn.Write([]byte(icapRequest))
    if err != nil {
        return "", err
    }
    
    buffer := make([]byte, 4096)
    n, err := conn.Read(buffer)
    if err != nil {
        return "", err
    }
    
    return string(buffer[:n]), nil
}

func main() {
    client := NewIcapClient("127.0.0.1", "1344")
    httpRequest := "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Go-ICAP-Client\r\n\r\n"
    response, err := client.SendReqmod(httpRequest)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
        return
    }
    fmt.Println(response)
}
```

### 3. JavaScript ICAP Client

```javascript
class IcapClient {
    constructor(host, port) {
        this.host = host;
        this.port = port;
    }
    
    async sendReqmod(httpRequest) {
        // Create ICAP request
        const icapRequest = `REQMOD icap://${this.host}:${this.port}/reqmod ICAP/1.0\r\n` +
            `Host: ${this.host}:${this.port}\r\n` +
            `Encapsulated: req-hdr=0, null-body=${httpRequest.length}\r\n` +
            `Allow: 204\r\n` +
            `\r\n` +
            httpRequest;
        
        // Send request using fetch (for browser) or node-fetch (for Node.js)
        const response = await fetch(`http://${this.host}:${this.port}`, {
            method: 'REQMOD',
            headers: {
                'Host': `${this.host}:${this.port}`,
                'Encapsulated': `req-hdr=0, null-body=${httpRequest.length}`,
                'Allow': '204'
            },
            body: httpRequest
        });
        
        return await response.text();
    }
}

// Usage
const client = new IcapClient('127.0.0.1', '1344');
const httpRequest = 'GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: JavaScript-ICAP-Client\r\n\r\n';
client.sendReqmod(httpRequest).then(response => {
    console.log(response);
}).catch(error => {
    console.error('Error:', error);
});
```

## Advanced Configuration Examples

### 1. Multi-Auditor Configuration

```yaml
# config/g3icap/g3icap.yaml
auditors:
  - name: "malware_filter"
    type: "content_filter"
    enabled: true
    blocked_extensions:
      - ".exe"
      - ".bat"
      - ".cmd"
      - ".scr"
      - ".pif"
      - ".com"
      - ".vbs"
      - ".js"
      - ".jar"
    blocked_mime_types:
      - "application/x-executable"
      - "application/x-msdownload"
      - "application/x-msdos-program"
    log_blocked_requests: true
    log_allowed_requests: false

  - name: "social_media_filter"
    type: "content_filter"
    enabled: true
    blocked_domains:
      - "facebook.com"
      - "www.facebook.com"
      - "m.facebook.com"
      - "fb.com"
      - "www.fb.com"
      - "instagram.com"
      - "www.instagram.com"
      - "twitter.com"
      - "www.twitter.com"
      - "x.com"
      - "www.x.com"
      - "tiktok.com"
      - "www.tiktok.com"
      - "snapchat.com"
      - "www.snapchat.com"
      - "linkedin.com"
      - "www.linkedin.com"
    blocked_urls:
      - ".*/facebook/.*"
      - ".*/fb/.*"
      - ".*/instagram/.*"
      - ".*/twitter/.*"
      - ".*/x\\.com/.*"
      - ".*/tiktok/.*"
      - ".*/snapchat/.*"
    log_blocked_requests: true
    log_allowed_requests: false

  - name: "antivirus_scanner"
    type: "antivirus"
    enabled: true
    scan_timeout_secs: 10
    log_clean_requests: false
    log_infected_requests: true

  - name: "audit_logger"
    type: "logging"
    enabled: true
    log_level: "info"
    log_format: "json"
```

### 2. SSL/TLS Configuration

```yaml
# config/g3icap/g3icap.yaml
server:
  ssl:
    enabled: true
    cert_file: "/path/to/cert.pem"
    key_file: "/path/to/key.pem"
    ca_file: "/path/to/ca.pem"
    verify_client: false
    cipher_suites:
      - "TLS_AES_256_GCM_SHA384"
      - "TLS_CHACHA20_POLY1305_SHA256"
      - "TLS_AES_128_GCM_SHA256"
    min_tls_version: "1.2"
    max_tls_version: "1.3"
```

### 3. High Availability Configuration

```yaml
# config/g3icap/g3icap.yaml
server:
  ha:
    enabled: true
    mode: "active_passive"
    heartbeat_interval_secs: 5
    heartbeat_timeout_secs: 15
    failover_timeout_secs: 30
    shared_storage:
      enabled: true
      path: "/shared/g3icap"
      lock_file: "/shared/g3icap/.lock"
```

## Troubleshooting Examples

### 1. Debug Logging

```bash
# Enable debug logging
RUST_LOG=debug ./target/debug/g3icap -c config/g3icap/g3icap.yaml

# Enable specific module logging
RUST_LOG=g3icap::audit=debug ./target/debug/g3icap -c config/g3icap/g3icap.yaml

# Enable trace logging
RUST_LOG=trace ./target/debug/g3icap -c config/g3icap/g3icap.yaml
```

### 2. Performance Monitoring

```bash
# Monitor server performance
curl -X GET http://localhost:1344/metrics

# Check connection statistics
curl -X GET http://localhost:1344/stats

# View health status
curl -X GET http://localhost:1344/health
```

### 3. Configuration Validation

```bash
# Validate configuration
./target/debug/g3icap --validate-config -c config/g3icap/g3icap.yaml

# Test configuration
./target/debug/g3icap --test-config -c config/g3icap/g3icap.yaml
```

## Best Practices

### 1. Security Best Practices

- Always enable authentication and authorization
- Use strong passwords and API keys
- Enable rate limiting to prevent abuse
- Regularly rotate credentials
- Monitor for suspicious activity

### 2. Performance Best Practices

- Configure appropriate connection pool sizes
- Enable caching for frequently accessed content
- Monitor memory usage and garbage collection
- Use load balancing for high availability
- Optimize auditor configurations

### 3. Monitoring Best Practices

- Enable comprehensive logging
- Set up health checks and alerts
- Monitor performance metrics
- Use distributed tracing for debugging
- Regular backup of configurations

### 4. Deployment Best Practices

- Use configuration management
- Implement proper error handling
- Test thoroughly before deployment
- Use version control for configurations
- Document all changes

## Conclusion

These examples demonstrate the comprehensive capabilities of G3ICAP and its strong RFC 3507 compliance. The server provides a robust, high-performance platform for content adaptation with extensive security, monitoring, and performance features.

For more detailed information, refer to the [RFC 3507 Compliance Documentation](compliance-overview.md) and [Implementation Details](implemented-features.md).

## References

- [RFC 3507](https://tools.ietf.org/html/rfc3507) - Internet Content Adaptation Protocol
- [G3ICAP Source Code](https://github.com/ByteDance/Arcus/tree/main/g3icap)
- [Configuration Reference](config-reference.md)
- [API Documentation](api-documentation.md)