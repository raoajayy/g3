# G3ICAP Configuration Directory

This directory contains all configuration files for the G3ICAP service.

## Directory Structure

```
config/g3icap/
├── README.md                    # This file
├── g3icap.yaml                  # Basic G3ICAP configuration
├── yara_config.yaml             # YARA antivirus configuration
├── yara_rules/                  # YARA rules directory
│   └── malware_detection.yar    # Example YARA rules
├── quarantine/                  # Quarantine directory (created at runtime)
└── logs/                        # Log directory (created at runtime)
```

## Configuration Files

### g3icap.yaml
Basic G3ICAP configuration with:
- Server settings (host, port, timeouts)
- Logging configuration
- Statistics and metrics settings
- StatsD integration

### yara_config.yaml
Advanced YARA antivirus configuration with:
- YARA engine settings
- Antivirus module configuration
- File type filtering
- Quarantine settings
- Threat intelligence integration

## YARA Rules

### yara_rules/malware_detection.yar
Example YARA rules for detecting:
- Generic malware patterns
- Phishing attempts
- Ransomware indicators
- APT (Advanced Persistent Threat) patterns
- PowerShell-based attacks
- Fileless malware
- Network security threats
- Data exfiltration attempts
- Social engineering tactics

## Usage

### Starting G3ICAP with Basic Configuration
```bash
./start-services.sh icap-only
```

### Starting All G3 Services (including G3ICAP)
```bash
./start-services.sh start
```

### Configuration Management
- All configuration files use relative paths
- Logs are written to `config/g3icap/logs/`
- Quarantined files are stored in `config/g3icap/quarantine/`
- YARA rules are loaded from `config/g3icap/yara_rules/`

## Customization

### Adding Custom YARA Rules
1. Create your YARA rule file in `config/g3icap/yara_rules/`
2. Use the `.yar` or `.yara` extension
3. Follow YARA rule syntax
4. Restart G3ICAP to load new rules

### Modifying Configuration
1. Edit the appropriate YAML file
2. Use relative paths for all directories
3. Restart the service to apply changes

## Security Notes

- Quarantine directory should have restricted permissions
- Log files may contain sensitive information
- YARA rules should be validated before deployment
- Regular cleanup of quarantine and log directories is recommended

## Troubleshooting

### Check Service Status
```bash
./start-services.sh status
```

### View Logs
```bash
tail -f config/g3icap/logs/g3icap.log
```

### Verify YARA Rules
```bash
ls -la config/g3icap/yara_rules/
```

## Integration

G3ICAP integrates with:
- G3Proxy for HTTP/HTTPS content adaptation
- G3StatsD for metrics collection
- InfluxDB for time series data storage
- Admin Console for web-based management
