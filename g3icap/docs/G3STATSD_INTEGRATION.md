# G3ICAP G3StatsD Integration

This document describes how G3ICAP integrates with G3StatsD for metrics collection and monitoring.

## Overview

G3ICAP uses G3StatsD (the same statsd system used by other G3 services) to emit metrics for monitoring, alerting, and observability. This integration follows the same patterns used by G3Proxy, G3StatsD, and other G3 services.

## Architecture

```
G3ICAP Server
    ↓ (metrics)
G3StatsD Client
    ↓ (UDP/TCP)
G3StatsD Server
    ↓ (aggregation)
InfluxDB/Prometheus
    ↓ (visualization)
Grafana/Dashboard
```

## Metrics Emitted

### Counter Metrics

| Metric Name | Description | Tags |
|-------------|-------------|------|
| `g3icap.requests.total` | Total ICAP requests processed | `method`, `service` |
| `g3icap.requests.reqmod` | REQMOD requests | `service` |
| `g3icap.requests.respmod` | RESPMOD requests | `service` |
| `g3icap.requests.options` | OPTIONS requests | `service` |
| `g3icap.responses.successful` | Successful responses | `method`, `service` |
| `g3icap.responses.error` | Error responses | `method`, `service`, `error_type` |
| `g3icap.requests.blocked` | Blocked requests | `reason` |
| `g3icap.bytes.total` | Total bytes processed | `direction` |
| `g3icap.connections.total` | Total connections accepted | `client_ip` |
| `g3icap.connections.error` | Connection errors | `error_type` |
| `g3icap.processing_time.total` | Total processing time | - |

### Gauge Metrics

| Metric Name | Description | Tags |
|-------------|-------------|------|
| `g3icap.connections.active` | Current active connections | - |

### Timing Metrics

| Metric Name | Description | Tags |
|-------------|-------------|------|
| `g3icap.processing_time.avg` | Average processing time | `method`, `service` |

## Configuration

### Command Line Options

```bash
# Enable StatsD metrics emission
g3icap --statsd

# Configure StatsD server
g3icap --statsd \
       --statsd-server 127.0.0.1 \
       --statsd-port 8125 \
       --statsd-prefix g3icap \
       --statsd-interval 10
```

### Configuration File

```yaml
# statsd section in g3icap.yaml
statsd:
  server: "127.0.0.1"
  port: 8125
  prefix: "g3icap"
  emit_interval: 10  # seconds
  buffer_size: 1024
  udp_enabled: true
  tcp_enabled: false
```

### Environment Variables

```bash
export G3ICAP_STATSD_ENABLED=true
export G3ICAP_STATSD_SERVER=127.0.0.1
export G3ICAP_STATSD_PORT=8125
export G3ICAP_STATSD_PREFIX=g3icap
export G3ICAP_STATSD_INTERVAL=10
```

## Usage Examples

### Basic Usage

```bash
# Start G3ICAP with StatsD enabled
g3icap --statsd --statsd-server 127.0.0.1 --statsd-port 8125
```

### With G3StatsD Server

```bash
# Terminal 1: Start G3StatsD server
g3statsd --config /etc/g3statsd/g3statsd.yaml

# Terminal 2: Start G3ICAP with StatsD
g3icap --statsd --statsd-server 127.0.0.1 --statsd-port 8125
```

### Docker Compose

```yaml
version: '3.8'
services:
  g3statsd:
    image: g3statsd:latest
    ports:
      - "8125:8125/udp"
    volumes:
      - ./g3statsd.yaml:/etc/g3statsd/g3statsd.yaml
    command: --config /etc/g3statsd/g3statsd.yaml

  g3icap:
    image: g3icap:latest
    ports:
      - "1344:1344"
    environment:
      - G3ICAP_STATSD_ENABLED=true
      - G3ICAP_STATSD_SERVER=g3statsd
      - G3ICAP_STATSD_PORT=8125
    depends_on:
      - g3statsd
```

## Metrics Format

### StatsD Protocol

Metrics are emitted in StatsD format:

```
# Counter metrics
g3icap.requests.total:1234|c
g3icap.responses.successful:1200|c
g3icap.responses.error:34|c

# Gauge metrics
g3icap.connections.active:56|g

# Timing metrics
g3icap.processing_time.avg:150|ms
```

### With Tags

```
# Tagged metrics
g3icap.requests.total:1234|c|#method:REQMOD,service:reqmod
g3icap.responses.error:5|c|#method:RESPMOD,service:respmod,error_type:timeout
g3icap.processing_time.avg:200|ms|#method:OPTIONS,service:options
```

## Monitoring and Alerting

### Key Metrics to Monitor

1. **Request Rate**: `g3icap.requests.total`
2. **Error Rate**: `g3icap.responses.error / g3icap.requests.total`
3. **Active Connections**: `g3icap.connections.active`
4. **Processing Time**: `g3icap.processing_time.avg`
5. **Connection Errors**: `g3icap.connections.error`

### Grafana Dashboard Queries

```promql
# Request rate per second
rate(g3icap_requests_total[5m])

# Error rate percentage
rate(g3icap_responses_error[5m]) / rate(g3icap_requests_total[5m]) * 100

# Average processing time
g3icap_processing_time_avg

# Active connections
g3icap_connections_active
```

### Alerting Rules

```yaml
# High error rate alert
- alert: G3ICAPHighErrorRate
  expr: rate(g3icap_responses_error[5m]) / rate(g3icap_requests_total[5m]) > 0.05
  for: 2m
  labels:
    severity: warning
  annotations:
    summary: "G3ICAP error rate is high"
    description: "Error rate is {{ $value | humanizePercentage }}"

# High connection count alert
- alert: G3ICAPHighConnectionCount
  expr: g3icap_connections_active > 1000
  for: 1m
  labels:
    severity: critical
  annotations:
    summary: "G3ICAP has too many active connections"
    description: "Active connections: {{ $value }}"
```

## Performance Considerations

### StatsD Client Configuration

```yaml
statsd:
  buffer_size: 1024      # Buffer size for batching
  emit_interval: 10      # Emission interval in seconds
  udp_enabled: true      # Use UDP for better performance
  tcp_enabled: false     # Disable TCP unless needed
```

### High-Volume Scenarios

For high-volume deployments:

1. **Increase buffer size**: `buffer_size: 4096`
2. **Reduce emission interval**: `emit_interval: 5`
3. **Use UDP only**: `tcp_enabled: false`
4. **Monitor StatsD server**: Ensure it can handle the load

### Memory Usage

- **StatsD Client**: ~1MB per instance
- **Metrics Buffer**: ~4KB per 1000 requests
- **Thread Overhead**: Minimal (single background thread)

## Troubleshooting

### Common Issues

1. **Metrics not appearing**
   - Check StatsD server is running
   - Verify network connectivity
   - Check firewall rules

2. **High memory usage**
   - Reduce buffer size
   - Increase emission interval
   - Check for memory leaks

3. **Performance impact**
   - Use UDP instead of TCP
   - Increase emission interval
   - Monitor StatsD server performance

### Debug Mode

```bash
# Enable debug logging for StatsD
RUST_LOG=debug g3icap --statsd --statsd-server 127.0.0.1
```

### Verification

```bash
# Check if metrics are being emitted
tcpdump -i lo -n port 8125

# Test StatsD server
echo "g3icap.test:1|c" | nc -u 127.0.0.1 8125
```

## Integration with G3 Ecosystem

### G3StatsD Server

G3ICAP integrates with the same G3StatsD server used by other G3 services:

- **G3Proxy**: HTTP proxy metrics
- **G3StatsD**: StatsD server metrics
- **G3ICAP**: ICAP server metrics

### Shared Configuration

All G3 services can share the same StatsD configuration:

```yaml
# Shared G3StatsD configuration
g3statsd:
  server: "statsd.internal.company.com"
  port: 8125
  prefix: "g3"
  tags:
    environment: "production"
    datacenter: "us-west-2"
```

### Metric Namespacing

G3ICAP metrics are namespaced under `g3icap.*` to avoid conflicts:

- `g3icap.requests.total`
- `g3proxy.requests.total`
- `g3statsd.metrics.received`

## Best Practices

1. **Use consistent naming**: Follow G3 naming conventions
2. **Tag appropriately**: Add relevant tags for filtering
3. **Monitor key metrics**: Set up alerts for critical metrics
4. **Test in staging**: Verify metrics work before production
5. **Document metrics**: Keep metric documentation up to date

## Migration from Custom Metrics

If migrating from a custom metrics system:

1. **Map existing metrics**: Create mapping to G3StatsD format
2. **Update dashboards**: Modify Grafana queries
3. **Test thoroughly**: Verify all metrics are working
4. **Monitor during transition**: Watch for any issues

## Conclusion

G3ICAP's G3StatsD integration provides comprehensive metrics collection and monitoring capabilities, following the same patterns as other G3 services. This ensures consistency across the G3 ecosystem and enables effective monitoring and alerting.
