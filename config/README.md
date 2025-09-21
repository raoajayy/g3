# G3 Configuration Directory

This directory contains all configuration files for the G3 ecosystem services.

## Directory Structure

```
config/
├── README.md                           # This file
├── g3proxy_monitoring.yaml             # G3Proxy configuration
├── g3statsd_influxdb_config.yaml       # G3StatsD configuration
├── g3fcgen.yaml                        # G3FCGen configuration
├── ajay.key                            # SSL private key
├── ajay.crt                            # SSL certificate
└── g3icap/                             # G3ICAP configuration
    ├── README.md                       # G3ICAP documentation
    ├── g3icap.yaml                     # Basic G3ICAP config
    ├── yara_config.yaml                # YARA antivirus config
    ├── yara_rules/                     # YARA rules directory
    │   └── malware_detection.yar       # Example YARA rules
    ├── quarantine/                     # Quarantine directory
    └── logs/                           # Log directory
```

## Services Configuration

### G3Proxy
- **File**: `g3proxy_monitoring.yaml`
- **Purpose**: HTTP/HTTPS proxy server configuration
- **Features**: Load balancing, SSL termination, monitoring

### G3StatsD
- **File**: `g3statsd_influxdb_config.yaml`
- **Purpose**: Statistics collection and metrics aggregation
- **Features**: InfluxDB integration, real-time metrics

### G3FCGen
- **File**: `g3fcgen.yaml`
- **Purpose**: Fake certificate generator for testing
- **Features**: SSL certificate generation, key management

### G3ICAP
- **Directory**: `g3icap/`
- **Purpose**: ICAP content adaptation server with YARA antivirus
- **Features**: Content filtering, virus scanning, YARA rules

## Usage

### Start All Services
```bash
./start-services.sh start
```

### Start Individual Services
```bash
./start-services.sh icap-only    # G3ICAP only
./start-services.sh status       # Check status
./start-services.sh stop         # Stop all services
```

### Service URLs
- **G3Proxy**: http://localhost:8080
- **G3StatsD**: http://localhost:8125
- **G3ICAP**: http://localhost:1344
- **Admin Console**: http://localhost:3002
- **Grafana**: http://localhost:3001

## Configuration Management

### Adding Custom YARA Rules
1. Create rule file in `g3icap/yara_rules/`
2. Use `.yar` or `.yara` extension
3. Restart G3ICAP service

### Modifying Service Configurations
1. Edit the appropriate YAML file
2. Use relative paths for all directories
3. Restart the service to apply changes

### SSL Certificates
- Private key: `ajay.key`
- Certificate: `ajay.crt`
- Generated automatically if not present

## Security Considerations

- Quarantine directories have restricted permissions
- Log files may contain sensitive information
- YARA rules should be validated before deployment
- Regular cleanup of temporary directories recommended

## Troubleshooting

### Check Service Status
```bash
./start-services.sh status
```

### View Service Logs
```bash
tail -f config/g3icap/logs/g3icap.log
```

### Verify Configuration
```bash
# Check G3ICAP configuration
cat config/g3icap/g3icap.yaml

# Check YARA rules
ls -la config/g3icap/yara_rules/
```

## Integration

The G3 ecosystem provides:
- **Content Adaptation**: G3ICAP with YARA antivirus
- **Proxy Services**: G3Proxy for HTTP/HTTPS traffic
- **Metrics Collection**: G3StatsD for statistics
- **Certificate Management**: G3FCGen for SSL/TLS
- **Monitoring**: InfluxDB + Grafana dashboard
- **Management**: Web-based admin console

All services are configured to work together seamlessly with shared configuration and monitoring.