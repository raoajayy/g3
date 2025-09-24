# URI Validation

This document describes the URI validation functionality in the G3ICAP server, which provides comprehensive validation of ICAP URIs according to RFC 3507 specifications.

## Overview

The URI validation system ensures that all ICAP URIs conform to the standards defined in RFC 3507, providing security, compliance, and proper protocol handling. The system includes both basic validation and comprehensive ICAP-specific validation.

## Features

### Core Functionality

- **RFC 3507 Compliance**: Full compliance with ICAP URI specifications
- **Comprehensive Validation**: Scheme, host, port, service path, and format validation
- **Security Checks**: Malicious pattern detection and security validation
- **Performance Optimization**: Caching and parallel validation support
- **Detailed Error Reporting**: Comprehensive error messages with context

### Validation Components

- **Scheme Validation**: Ensures URI uses "icap" scheme
- **Host Validation**: Validates IP addresses and hostnames
- **Port Validation**: Checks port ranges and reserved ports
- **Service Path Validation**: Validates ICAP service names
- **Format Validation**: Checks URI format and character validity
- **Security Validation**: Detects malicious patterns and security threats

## Configuration

### Basic Configuration

```yaml
# Enable URI validation
enabled: true

# Basic validation settings
validation:
  max_uri_length: 2048
  strict_mode: true
  enable_security_checks: true
```

### Advanced Configuration

```yaml
# ICAP-specific validation
validation:
  allowed_schemes: ["icap"]
  default_port: 1344
  allowed_service_patterns:
    - "reqmod"
    - "respmod"
    - "options"
    - "preview"
    - "custom-*"
  
  # Reserved ports
  reserved_ports: [0, 1, 7, 9, 11, 13, 15, 17, 19, 20, 21, 22, 23, 25, 37, 42, 43, 53, 67, 68, 69, 70, 79, 80, 88, 110, 111, 113, 119, 123, 135, 139, 143, 161, 162, 179, 389, 443, 445, 465, 514, 515, 587, 636, 993, 995]

# Format validation
format:
  allow_query_params: false
  allow_fragments: false
  strict_hostname_validation: true
  allow_private_ips: true
  allow_localhost: true
  allow_ipv6: true
```

## Usage

### Basic Usage

```rust
use g3icap::protocol::input_validation::{InputValidator, ValidationConfig};
use g3icap::protocol::uri_validation::{IcapUriValidator, IcapUriValidation};

// Create input validator
let mut validator = InputValidator::with_defaults();

// Validate ICAP URI
let is_valid = validator.is_valid_icap_uri("icap://127.0.0.1:1344/reqmod");
println!("URI is valid: {}", is_valid);

// Extract components
let service = validator.extract_service_name("icap://127.0.0.1:1344/reqmod")?;
let host = validator.extract_host("icap://127.0.0.1:1344/reqmod")?;
let port = validator.extract_port("icap://127.0.0.1:1344/reqmod")?;
```

### Advanced Usage

```rust
use g3icap::protocol::uri_validation::{IcapUriValidator, IcapUriValidation};

// Create custom validator
let validator = IcapUriValidator::with_config(
    vec!["icap".to_string()],
    8080,  // Default port
    4096,  // Max URI length
    vec!["reqmod".to_string(), "respmod".to_string(), "options".to_string()],
);

// Detailed validation
let validation = validator.validate_uri("icap://example.com:8080/reqmod");
if validation.is_valid {
    let components = validation.components.unwrap();
    println!("Scheme: {}", components.scheme);
    println!("Host: {}", components.host);
    println!("Port: {}", components.port);
    println!("Service: {}", components.service);
} else {
    for error in &validation.errors {
        println!("Validation error: {}", error);
    }
}
```

### Request Validation

```rust
use g3icap::protocol::input_validation::{InputValidator, ValidationConfig};
use g3icap::protocol::common::{IcapRequest, IcapMethod};
use http::HeaderMap;
use bytes::Bytes;

// Create validator
let mut validator = InputValidator::with_defaults();

// Create ICAP request
let mut headers = HeaderMap::new();
headers.insert("host", "127.0.0.1:1344".parse().unwrap());
headers.insert("icap-version", "ICAP/1.0".parse().unwrap());

let request = IcapRequest {
    method: IcapMethod::Reqmod,
    uri: "icap://127.0.0.1:1344/reqmod".to_string(),
    version: http::Version::HTTP_11,
    headers,
    body: Bytes::from("test body"),
    encapsulated: None,
};

// Validate request
let result = validator.validate_request(&request);
if result.is_valid {
    println!("Request is valid");
} else {
    for error in &result.errors {
        println!("Validation error: {:?}", error);
    }
}
```

## API Reference

### InputValidator

The main input validator that integrates URI validation with other validation features.

#### Methods

- `new(config: ValidationConfig) -> Self`: Create a new input validator
- `with_defaults() -> Self`: Create validator with default configuration
- `validate_request(request: &IcapRequest) -> ValidationResult`: Validate ICAP request
- `validate_uri_detailed(uri: &str) -> IcapUriValidation`: Get detailed URI validation
- `is_valid_icap_uri(uri: &str) -> bool`: Check if URI is valid ICAP URI
- `extract_service_name(uri: &str) -> Result<String, ValidationError>`: Extract service name
- `extract_host(uri: &str) -> Result<String, ValidationError>`: Extract host
- `extract_port(uri: &str) -> Result<u16, ValidationError>`: Extract port

### IcapUriValidator

The comprehensive ICAP URI validator.

#### Methods

- `new() -> Self`: Create a new validator with default settings
- `with_config(schemes, port, max_length, patterns) -> Self`: Create custom validator
- `new_strict() -> Self`: Create strict validator (no private IPs, no localhost)
- `new_permissive() -> Self`: Create permissive validator (allows private IPs, localhost)
- `validate_uri(uri: &str) -> IcapUriValidation`: Validate ICAP URI
- `get_stats() -> IcapUriValidationStats`: Get validation statistics

### IcapUriValidation

The result of URI validation.

#### Properties

- `is_valid: bool`: Whether the URI is valid
- `errors: Vec<IcapUriError>`: List of validation errors
- `components: Option<IcapUriComponents>`: Parsed URI components

### IcapUriComponents

Parsed URI components.

#### Properties

- `scheme: String`: URI scheme (should be "icap")
- `host: String`: Host (IP address or hostname)
- `port: u16`: Port number
- `service: String`: Service path
- `uri: String`: Full URI string

## Error Types

### IcapUriError

Comprehensive error types for URI validation:

- `InvalidFormat(String)`: Invalid URI format
- `InvalidScheme(String)`: Invalid scheme (not "icap")
- `MissingHost`: Missing host component
- `InvalidHost(String)`: Invalid host format
- `InvalidPort(String)`: Invalid port format
- `MissingService`: Missing service path
- `InvalidServicePath(String)`: Invalid service path
- `PortOutOfRange(u16)`: Port out of valid range
- `UnsupportedIPv6`: Unsupported IPv6 format
- `InvalidCharacters(String)`: Invalid characters in URI
- `UriTooLong(usize, usize)`: URI too long
- `InvalidQueryParameters(String)`: Query parameters not allowed
- `InvalidFragment(String)`: Fragment not allowed
- `UnsupportedHostnameFormat(String)`: Unsupported hostname format
- `ReservedPort(u16)`: Reserved port used
- `InvalidServiceName(String)`: Invalid service name
- `ServiceNotAllowed(String)`: Service not allowed

## Security Features

### Malicious Pattern Detection

The system includes comprehensive malicious pattern detection:

- Path traversal attacks (`../`, `%2e%2e%2f`)
- XSS attempts (`<script`, `javascript:`)
- Code injection (`eval(`, `vbscript:`)
- Event handler injection (`onload=`, `onerror=`)
- Data URI abuse (`data:text/html`)

### Security Validation

- Hostname validation against blocked patterns
- Service name validation against blocked services
- Port validation against reserved ports
- Character validation for security threats

## Performance Features

### Caching

- Validation result caching
- Configurable cache size and TTL
- Performance optimization for repeated validations

### Parallel Processing

- Concurrent validation support
- Configurable concurrency limits
- Performance metrics collection

## Monitoring

### Metrics

- Validation success/failure rates
- Performance metrics (validation time)
- Error rate tracking
- Cache hit/miss ratios

### Export Formats

- JSON format for programmatic access
- Prometheus format for monitoring systems
- StatsD format for metrics collection

## Compliance

### RFC 3507 Compliance

The URI validation system is fully compliant with RFC 3507:

- Proper ICAP URI format validation
- Scheme validation ("icap" only)
- Host and port validation
- Service path validation
- Character set validation

### Security Compliance

- OWASP security guidelines
- Input validation best practices
- Malicious pattern detection
- Security header validation

## Examples

See the `examples/uri_validation_example.rs` file for comprehensive usage examples.

## Troubleshooting

### Common Issues

1. **Invalid scheme errors**: Ensure URI uses "icap://" scheme
2. **Missing service errors**: Include service path in URI (e.g., "/reqmod")
3. **Port validation errors**: Use valid port numbers (1-65535)
4. **Hostname validation errors**: Use valid hostnames or IP addresses
5. **Reserved port errors**: Avoid using reserved ports

### Debug Mode

Enable debug mode for detailed validation information:

```yaml
debug:
  enabled: true
  verbose_logging: true
  trace_validation: true
```

## Support

For issues, questions, or contributions related to URI validation, please refer to the main G3ICAP documentation or contact the development team.
