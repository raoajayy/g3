# G3Proxy and G3ICAP Integration Guide

This guide explains how to configure and use G3Proxy with G3ICAP for content adaptation and security scanning.

## Overview

The integration between G3Proxy and G3ICAP provides:
- **Content Filtering**: Block or modify HTTP requests and responses
- **Virus Scanning**: Scan content for malware using YARA rules
- **Content Adaptation**: Modify content based on policies
- **Real-time Monitoring**: Track and log all content processing

## Architecture

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Client    │───▶│   G3Proxy   │───▶│   G3ICAP    │
│             │    │             │    │             │
│ HTTP/HTTPS  │    │ ICAP Client │    │ ICAP Server │
│ Requests    │    │             │    │ YARA Engine │
└─────────────┘    └─────────────┘    └─────────────┘
                           │
                           ▼
                   ┌─────────────┐
                   │  Backend    │
                   │  Server     │
                   └─────────────┘
```

## Configuration Files

### G3Proxy Configurations

1. **Basic ICAP Integration**: `config/g3proxy_with_icap.yaml`
   - Simple ICAP integration
   - REQMOD and RESPMOD services
   - Basic ICAP settings

2. **Advanced ICAP Integration**: `config/g3proxy_icap_advanced.yaml`
   - Multiple auditor configurations
   - Different ICAP services for different purposes
   - Advanced ICAP settings

3. **Basic Configuration**: `config/g3proxy_monitoring.yaml`
   - Basic proxy without ICAP
   - Monitoring and statistics

### G3ICAP Configurations

1. **Basic Configuration**: `config/g3icap/g3icap.yaml`
   - Basic ICAP server settings
   - Simple logging and statistics

2. **YARA Antivirus Configuration**: `config/g3icap/yara_config.yaml`
   - YARA rule-based antivirus scanning
   - Content filtering and quarantine
   - Advanced threat detection

## ICAP Services

### REQMOD (Request Modification)
- **Purpose**: Modify HTTP requests before forwarding
- **Use Cases**: URL filtering, request header modification, authentication
- **Endpoint**: `icap://127.0.0.1:1344/reqmod`

### RESPMOD (Response Modification)
- **Purpose**: Modify HTTP responses before sending to client
- **Use Cases**: Content filtering, virus scanning, response header modification
- **Endpoint**: `icap://127.0.0.1:1344/respmod`

### OPTIONS (Service Discovery)
- **Purpose**: Discover ICAP service capabilities
- **Use Cases**: Service health checks, capability negotiation
- **Endpoint**: `icap://127.0.0.1:1344/options`

## Usage Examples

### Start All Services with ICAP
```bash
# Start all services with basic ICAP integration
./start-services.sh start

# Start all services with advanced ICAP configuration
./start-services.sh icap-advanced
```

### Start Individual Services
```bash
# Start only G3ICAP
./start-services.sh icap-only

# Start only G3Proxy with ICAP
./start-services.sh proxy-only
```

### Check Service Status
```bash
./start-services.sh status
```

## Service URLs

### G3Proxy Endpoints
- **HTTP Proxy**: `http://127.0.0.1:3129`
- **HTTPS Proxy**: `http://127.0.0.1:3128`
- **SOCKS Proxy**: `socks5://127.0.0.1:1081`

### G3ICAP Endpoints
- **ICAP Server**: `icap://127.0.0.1:1344`
- **Statistics**: `http://127.0.0.1:8080/stats`
- **Metrics**: `http://127.0.0.1:9090/metrics`

### Admin Console
- **Web Interface**: `http://localhost:3002`

## Configuration Examples

### Basic ICAP Integration

```yaml
# G3Proxy configuration
auditor:
  - name: default
    icap_reqmod_service: icap://127.0.0.1:1344/reqmod
    icap_respmod_service: icap://127.0.0.1:1344/respmod
    icap_options_service: icap://127.0.0.1:1344/options
```

### Advanced ICAP Integration

```yaml
# G3Proxy configuration with multiple auditors
auditor:
  - name: content_filter
    icap_reqmod_service: icap://127.0.0.1:1344/reqmod
    icap_respmod_service: icap://127.0.0.1:1344/respmod
  
  - name: antivirus_scan
    icap_reqmod_service: icap://127.0.0.1:1344/reqmod
    icap_respmod_service: icap://127.0.0.1:1344/respmod

server:
  - name: http_filtered
    auditor: content_filter
    # ... server configuration
  
  - name: https_filtered
    auditor: antivirus_scan
    # ... server configuration
```

### YARA Antivirus Configuration

```yaml
# G3ICAP YARA configuration
modules:
  - name: "yara_antivirus"
    type: "antivirus"
    config:
      engine:
        type: "YARA"
        rules_dir: "config/g3icap/yara_rules"
        timeout: 30
        max_rules: 1000
        enable_compilation: true
      
      max_file_size: 104857600  # 100MB
      scan_timeout: 60
      enable_quarantine: true
      quarantine_dir: "config/g3icap/quarantine"
```

## Testing the Integration

### Test HTTP Request Filtering
```bash
# Test through G3Proxy
curl -x http://127.0.0.1:3129 http://example.com

# Check G3ICAP logs
tail -f config/g3icap/logs/g3icap.log
```

### Test Virus Scanning
```bash
# Create a test file with malware content
echo "This file contains malware and virus code" > test_malware.txt

# Test through G3Proxy
curl -x http://127.0.0.1:3129 -T test_malware.txt http://httpbin.org/post

# Check quarantine directory
ls -la config/g3icap/quarantine/
```

### Test YARA Rules
```bash
# Add custom YARA rule
cat > config/g3icap/yara_rules/custom_rule.yar << 'EOF'
rule TestRule {
    strings:
        $test_string = "test_malware"
    condition:
        $test_string
}
EOF

# Restart G3ICAP to load new rules
./start-services.sh restart
```

## Monitoring and Logging

### View Service Logs
```bash
# G3ICAP logs
tail -f config/g3icap/logs/g3icap.log

# G3Proxy logs (if configured)
journalctl -u g3proxy -f
```

### Check Statistics
```bash
# G3ICAP statistics
curl http://127.0.0.1:8080/stats

# G3StatsD metrics
curl http://127.0.0.1:9090/metrics
```

### Monitor Quarantine
```bash
# List quarantined files
ls -la config/g3icap/quarantine/

# Check quarantine metadata
find config/g3icap/quarantine/ -name "*.meta" -exec cat {} \;
```

## Troubleshooting

### Common Issues

1. **ICAP Connection Failed**
   - Check if G3ICAP is running: `./start-services.sh status`
   - Verify ICAP port: `netstat -tlnp | grep 1344`
   - Check G3ICAP logs for errors

2. **YARA Rules Not Loading**
   - Verify rules directory: `ls -la config/g3icap/yara_rules/`
   - Check YARA rule syntax
   - Restart G3ICAP after rule changes

3. **Content Not Being Filtered**
   - Verify ICAP configuration in G3Proxy
   - Check G3ICAP service endpoints
   - Enable debug logging

4. **Performance Issues**
   - Check YARA rule complexity
   - Monitor memory usage
   - Adjust scan timeouts

### Debug Commands

```bash
# Check service status
./start-services.sh status

# Test ICAP connectivity
telnet 127.0.0.1 1344

# Check configuration syntax
./target/debug/g3proxy --config-file config/g3proxy_with_icap.yaml --test-config

# View detailed logs
./start-services.sh logs
```

## Security Considerations

### ICAP Security
- Use TLS for ICAP communication in production
- Implement authentication for ICAP services
- Monitor ICAP service health

### YARA Rules Security
- Validate YARA rules before deployment
- Use trusted rule sources
- Regular rule updates

### Quarantine Security
- Restrict access to quarantine directory
- Encrypt quarantined files
- Regular cleanup of old quarantined files

## Performance Tuning

### G3Proxy Tuning
- Adjust thread pool size
- Configure connection limits
- Optimize ICAP timeout settings

### G3ICAP Tuning
- Compile YARA rules for performance
- Use rule caching
- Optimize scan timeouts
- Configure appropriate file size limits

### System Tuning
- Allocate sufficient memory
- Use SSD storage for quarantine
- Monitor disk I/O performance

## Best Practices

1. **Configuration Management**
   - Use version control for configurations
   - Test configurations in staging environment
   - Document configuration changes

2. **Monitoring**
   - Set up alerts for service failures
   - Monitor performance metrics
   - Regular log analysis

3. **Security**
   - Regular security updates
   - Monitor for new threats
   - Implement defense in depth

4. **Maintenance**
   - Regular rule updates
   - Quarantine cleanup
   - Performance monitoring

## Conclusion

The G3Proxy and G3ICAP integration provides a powerful solution for content adaptation and security scanning. With proper configuration and monitoring, it can effectively protect against various threats while maintaining good performance.

For more information, see:
- [G3ICAP Documentation](g3icap/README.md)
- [G3Proxy Documentation](g3proxy/README.md)
- [Configuration Guide](README.md)
