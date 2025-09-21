# YARA Antivirus Module Documentation

## Overview

The G3ICAP YARA Antivirus Module provides advanced threat detection capabilities using YARA (Yet Another Recursive Acronym) rules for pattern matching and malware detection. This module integrates seamlessly with the G3ICAP content adaptation platform to provide real-time scanning of HTTP requests and responses.

## Features

### Core Capabilities
- **YARA Rule Engine**: Full support for YARA rule-based pattern matching
- **Multiple Threat Detection**: Detection of malware, ransomware, phishing, APT, and more
- **Real-time Scanning**: High-performance scanning of HTTP content
- **Rule Management**: Dynamic rule loading, compilation, and caching
- **Priority-based Matching**: Rule priority system for threat classification
- **Comprehensive Logging**: Detailed logging and metrics collection
- **Quarantine Support**: Automatic quarantine of detected threats

### YARA-Specific Features
- **Rule Compilation**: Automatic compilation of YARA rules for performance
- **Rule Caching**: Intelligent caching of frequently used rules
- **Rule Statistics**: Performance metrics and usage statistics
- **Rule Debugging**: Debug mode for rule development and troubleshooting
- **Rule Updates**: Automatic rule updates from external sources
- **Namespace Support**: Organized rule management with namespaces
- **Metadata Extraction**: Rich metadata from rule matches

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    YARA Antivirus Module                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │   YARA Client   │  │  Rule Manager   │  │   Scanner   │  │
│  │                 │  │                 │  │             │  │
│  │ • Rule Loading  │  │ • Rule Cache    │  │ • Content   │  │
│  │ • Rule Parsing  │  │ • Rule Stats    │  │   Scanning  │  │
│  │ • Rule Compile  │  │ • Rule Update   │  │ • Pattern   │  │
│  │ • Rule Execute  │  │ • Rule Priority │  │   Matching  │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │  Quarantine     │  │   Statistics    │  │   Logging   │  │
│  │  Manager        │  │   Collector     │  │   System    │  │
│  │                 │  │                 │  │             │  │
│  │ • File Storage  │  │ • Scan Metrics  │  │ • Structured│  │
│  │ • Metadata      │  │ • Rule Metrics  │  │   Logging   │  │
│  │ • Cleanup       │  │ • Performance   │  │ • Metrics   │  │
│  │ • Recovery      │  │   Stats         │  │   Emission  │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Content Reception**: HTTP content received via ICAP protocol
2. **Rule Loading**: YARA rules loaded from configured directory
3. **Rule Compilation**: Rules compiled for optimal performance
4. **Content Scanning**: Content scanned against all active rules
5. **Pattern Matching**: YARA patterns matched against content
6. **Threat Detection**: Matches evaluated for threat classification
7. **Response Generation**: Appropriate response generated based on results
8. **Quarantine**: Detected threats quarantined if enabled
9. **Logging**: Results logged and metrics collected

## Configuration

### Basic Configuration

```yaml
# Antivirus module configuration
antivirus:
  engine: YARA
  rules_dir: "/etc/g3icap/yara_rules"
  max_file_size: 104857600  # 100MB
  scan_timeout: 60
  enable_quarantine: true
  quarantine_dir: "/var/quarantine/g3icap"
  enable_logging: true
  enable_metrics: true
```

### Advanced YARA Configuration

```yaml
# YARA-specific configuration
yara_config:
  rules_dir: "/etc/g3icap/yara_rules"
  compiled_rules_dir: "/etc/g3icap/yara_rules/compiled"
  max_rules: 1000
  enable_compilation: true
  update_interval: 3600  # 1 hour
  enable_caching: true
  cache_size: 1000
  enable_rule_stats: true
  rule_priorities:
    "malware": 8
    "phishing": 7
    "ransomware": 9
    "apt": 9
  enable_debug: false
  rule_timeout: 30
```

### Engine Configuration

```yaml
# YARA engine configuration
engine:
  type: YARA
  rules_dir: "/etc/g3icap/yara_rules"
  timeout: 30
  max_rules: 1000
  enable_compilation: true
```

## YARA Rules

### Rule Structure

YARA rules follow the standard YARA syntax with additional metadata support:

```yara
rule Malware_Generic {
    meta:
        description = "Generic malware detection rule"
        author = "G3ICAP Team"
        priority = 8
        threat_level = "high"
        category = "malware"
    
    strings:
        $malware_strings = {
            "malware", "virus", "trojan", "worm", "backdoor",
            "rootkit", "spyware", "adware", "ransomware"
        }
        
        $suspicious_patterns = {
            "cmd.exe", "powershell", "regsvr32", "rundll32",
            "wscript", "cscript", "certutil", "bitsadmin"
        }
    
    condition:
        2 of ($malware_strings) or 3 of ($suspicious_patterns)
}
```

### Rule Metadata

The module supports additional metadata fields:

- `priority`: Rule priority (1-10, higher = more important)
- `threat_level`: Threat severity (low, medium, high, critical)
- `category`: Threat category (malware, phishing, ransomware, etc.)
- `author`: Rule author
- `description`: Rule description
- `version`: Rule version
- `last_updated`: Last update timestamp

### Rule Categories

#### 1. Malware Detection
- Generic malware patterns
- Specific malware families
- Behavioral indicators
- File structure analysis

#### 2. Phishing Detection
- Social engineering patterns
- Suspicious domain patterns
- Urgency indicators
- Credential harvesting

#### 3. Ransomware Detection
- Encryption indicators
- Ransom notes
- File extension changes
- Payment demands

#### 4. APT Detection
- Advanced persistent threat indicators
- Nation-state attack patterns
- Zero-day exploit indicators
- Custom tool signatures

#### 5. Network Security
- Suspicious network activity
- Data exfiltration patterns
- Command and control indicators
- Lateral movement patterns

## Usage Examples

### Basic Usage

```rust
use g3icap::modules::antivirus::{AntivirusConfig, AntivirusEngine, AntivirusModule};
use g3icap::modules::ModuleConfig;

// Create YARA configuration
let config = AntivirusConfig {
    engine: AntivirusEngine::YARA {
        rules_dir: PathBuf::from("/etc/g3icap/yara_rules"),
        timeout: Duration::from_secs(30),
        max_rules: 1000,
        enable_compilation: true,
    },
    max_file_size: 100 * 1024 * 1024, // 100MB
    scan_timeout: Duration::from_secs(60),
    enable_quarantine: true,
    quarantine_dir: Some(PathBuf::from("/var/quarantine")),
    enable_logging: true,
    enable_metrics: true,
    // ... other configuration options
};

// Create and initialize module
let mut module = AntivirusModule::new(config);
let module_config = ModuleConfig {
    name: "yara_antivirus".to_string(),
    version: "1.0.0".to_string(),
    config: serde_json::Value::Object(serde_json::Map::new()),
};

module.init(&module_config).await?;
```

### Advanced Usage with Custom Rules

```rust
// Load custom YARA rules
let yara_config = YaraConfig {
    rules_dir: PathBuf::from("/custom/yara/rules"),
    compiled_rules_dir: Some(PathBuf::from("/custom/yara/compiled")),
    max_rules: 5000,
    enable_compilation: true,
    update_interval: Duration::from_secs(1800), // 30 minutes
    enable_caching: true,
    cache_size: 2000,
    enable_rule_stats: true,
    rule_priorities: {
        let mut priorities = HashMap::new();
        priorities.insert("custom_malware".to_string(), 9);
        priorities.insert("phishing".to_string(), 7);
        priorities
    },
    enable_debug: true,
    rule_timeout: Duration::from_secs(45),
};

let config = AntivirusConfig {
    engine: AntivirusEngine::YARA {
        rules_dir: PathBuf::from("/custom/yara/rules"),
        timeout: Duration::from_secs(45),
        max_rules: 5000,
        enable_compilation: true,
    },
    yara_config: Some(yara_config),
    // ... other configuration
};
```

## Performance Optimization

### Rule Compilation
- Rules are automatically compiled for better performance
- Compiled rules are cached to avoid recompilation
- Compilation happens in background to avoid blocking

### Caching Strategy
- Frequently used rules are cached in memory
- Cache size is configurable based on available memory
- LRU (Least Recently Used) eviction policy
- Cache statistics for monitoring

### Scanning Optimization
- Parallel rule execution where possible
- Early termination on high-priority matches
- Content preprocessing for common patterns
- Memory-efficient scanning for large files

## Monitoring and Metrics

### Statistics Collected

#### Scan Statistics
- Total scans performed
- Clean files detected
- Infected files detected
- Quarantined files
- Scan errors
- Average scan time
- Scan throughput

#### Rule Statistics
- Total rules loaded
- Active rules
- Compiled rules
- Rule match counts
- Rule performance metrics
- Top matching rules

#### System Statistics
- Memory usage
- CPU usage
- Disk usage (quarantine)
- Network I/O
- Error rates

### Metrics Export

The module exports metrics in multiple formats:

- **StatsD**: Real-time metrics to StatsD server
- **Prometheus**: Metrics endpoint for Prometheus scraping
- **JSON**: Structured logging with metrics
- **Custom**: Custom metrics format

### Logging

Structured logging includes:

```json
{
  "timestamp": "2024-01-20T10:30:00Z",
  "level": "INFO",
  "module": "antivirus",
  "event": "scan_completed",
  "file_size": 1024,
  "scan_duration_ms": 150,
  "threats_detected": 1,
  "rule_matches": ["Malware_Generic"],
  "quarantined": true,
  "quarantine_id": "q_12345"
}
```

## Security Considerations

### Rule Security
- Rules are validated before loading
- Malicious rules are rejected
- Rule sources are verified
- Rule integrity is checked

### Quarantine Security
- Quarantined files are encrypted
- Access controls on quarantine directory
- Regular cleanup of old quarantined files
- Audit trail for quarantine operations

### Network Security
- Encrypted communication with external sources
- Certificate validation for rule updates
- Rate limiting for rule downloads
- Secure storage of sensitive rules

## Troubleshooting

### Common Issues

#### Rule Loading Failures
- Check rule syntax with YARA compiler
- Verify rule file permissions
- Check available memory for rule compilation
- Review rule directory configuration

#### Performance Issues
- Monitor rule cache hit rates
- Check for inefficient rules
- Review scan timeout settings
- Monitor memory usage

#### False Positives
- Review rule logic and conditions
- Adjust rule priorities
- Update rule metadata
- Test rules with known clean content

### Debug Mode

Enable debug mode for detailed logging:

```yaml
yara_config:
  enable_debug: true
  rule_timeout: 60  # Increased timeout for debugging
```

Debug logs include:
- Rule compilation details
- Pattern matching process
- Rule execution timing
- Memory usage information

## Integration

### G3ICAP Integration
- Seamless integration with ICAP protocol
- Support for REQMOD and RESPMOD methods
- Integration with content filtering pipeline
- Statistics and metrics collection

### External Integrations
- Threat intelligence feeds
- SIEM systems
- Incident response platforms
- Security orchestration tools

## Best Practices

### Rule Development
1. Use descriptive rule names
2. Include comprehensive metadata
3. Test rules thoroughly
4. Document rule purpose and logic
5. Use appropriate priority levels

### Performance Tuning
1. Compile rules for production
2. Enable caching for frequently used rules
3. Monitor and optimize slow rules
4. Use appropriate timeout values
5. Regular rule updates

### Security
1. Validate all rule sources
2. Use secure rule storage
3. Implement access controls
4. Regular security audits
5. Monitor for rule tampering

## Future Enhancements

### Planned Features
- Machine learning integration
- Behavioral analysis
- Cloud-based rule updates
- Advanced threat intelligence
- Real-time rule modification

### Performance Improvements
- GPU-accelerated scanning
- Distributed rule processing
- Advanced caching strategies
- Optimized pattern matching

## Conclusion

The G3ICAP YARA Antivirus Module provides a powerful, flexible, and high-performance solution for threat detection using YARA rules. With its comprehensive feature set, excellent performance characteristics, and seamless integration with the G3ICAP platform, it offers an ideal solution for organizations requiring advanced content security capabilities.

For more information, see the [G3ICAP Documentation](../README.md) and [Configuration Guide](CONFIGURATION_GUIDE.md).
