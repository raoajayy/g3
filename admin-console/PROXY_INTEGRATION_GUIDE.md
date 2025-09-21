# Proxy Integration Guide

This guide explains the comprehensive integration between the Arcus Admin Console and the G3Proxy system, including URL filtering, hot reload capabilities, and real-time testing.

## Overview

The proxy integration system provides:

- **Policy-to-Proxy Configuration Generation**: Converts security policies into G3Proxy YAML configurations
- **Hot Reload System**: Real-time policy updates without proxy restart
- **Real-time Testing**: Test policies against URLs with live feedback
- **Health Monitoring**: Continuous proxy status and performance monitoring
- **URL Categorization**: Integration with external URL category databases

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Admin Console │    │  Hot Reload      │    │   G3Proxy       │
│                 │    │  Manager         │    │                 │
│ ┌─────────────┐ │    │                  │    │ ┌─────────────┐ │
│ │ Policies    │ │───▶│ ┌──────────────┐ │───▶│ │ Config      │ │
│ │ Management  │ │    │ │ Config       │ │    │ │ Engine      │ │
│ └─────────────┘ │    │ │ Generator    │ │    │ └─────────────┘ │
│                 │    │ └──────────────┘ │    │                 │
│ ┌─────────────┐ │    │                  │    │ ┌─────────────┐ │
│ │ Testing     │ │───▶│ ┌──────────────┐ │───▶│ │ URL         │ │
│ │ Framework   │ │    │ │ Policy       │ │    │ │ Filtering   │ │
│ └─────────────┘ │    │ │ Evaluator    │ │    │ └─────────────┘ │
│                 │    │ └──────────────┘ │    │                 │
│ ┌─────────────┐ │    │                  │    │ ┌─────────────┐ │
│ │ Monitoring  │ │◀───│ ┌──────────────┐ │◀───│ │ Health      │ │
│ │ Dashboard   │ │    │ │ Health       │ │    │ │ Status      │ │
│ └─────────────┘ │    │ │ Monitor      │ │    │ └─────────────┘ │
└─────────────────┘    │ └──────────────┘ │    └─────────────────┘
                       └──────────────────┘
```

## Components

### 1. Proxy Configuration Generator (`proxy-config-generator.ts`)

Converts policy configurations into G3Proxy YAML format:

```typescript
const generator = new ProxyConfigGenerator();
const config = generator.generateConfig(policies);
```

**Features:**
- Policy-to-YAML conversion
- URL filtering rule generation
- Escaper configuration
- Configuration validation
- Test configuration generation

### 2. Hot Reload Manager (`hot-reload-manager.ts`)

Manages real-time policy updates and proxy configuration reloading:

```typescript
const manager = new HotReloadManager();
await manager.initialize();
await manager.applyPolicyChanges(policies);
```

**Features:**
- Real-time configuration updates
- Event-driven architecture
- Health monitoring
- Policy testing
- Error handling and recovery

### 3. API Endpoints

#### `/api/proxy/config`
- `GET`: Retrieve current proxy configuration
- `POST`: Apply new configuration to proxy
- `PUT`: Generate configuration from policies

#### `/api/proxy/status`
- `GET`: Get proxy health and status
- `POST`: Execute proxy actions (start/stop/restart/reload)

#### `/api/proxy/test`
- `POST`: Test policy against specific URL

### 4. UI Components

#### Proxy Integration Tester (`proxy-integration-tester.tsx`)
- Real-time policy testing
- Proxy status monitoring
- Configuration management
- Activity logging

#### Proxy Integration Page (`proxy-integration-page.tsx`)
- Comprehensive integration dashboard
- Tabbed interface for different functions
- Policy selection and testing
- Monitoring and preview

## Usage

### 1. Basic Integration

```typescript
import { HotReloadManager } from '@/lib/hot-reload-manager';
import { ProxyConfigGenerator } from '@/lib/proxy-config-generator';

// Initialize hot reload manager
const manager = new HotReloadManager();
await manager.initialize();

// Apply policy changes
const policies = await loadPolicies();
await manager.applyPolicyChanges(policies);
```

### 2. Policy Testing

```typescript
// Test a policy against a URL
const result = await manager.testPolicy(policy, 'https://example.com');
console.log(result.action); // 'allow', 'block', 'warn', 'inspect'
console.log(result.reason);
console.log(result.matchedRules);
```

### 3. Configuration Generation

```typescript
const generator = new ProxyConfigGenerator();
const config = generator.generateConfig(policies);

// Validate configuration
const validation = generator.validateConfig(config);
if (!validation.isValid) {
  console.error('Configuration errors:', validation.errors);
}

// Convert to YAML
const yamlConfig = generator.toYaml(config);
```

### 4. Health Monitoring

```typescript
// Check proxy status
const status = await manager.getProxyStatus();
console.log('Proxy running:', status.isRunning);
console.log('Health:', status.health);
console.log('Config version:', status.configVersion);

// Set up health monitoring
manager.startHealthMonitoring(30000); // Check every 30 seconds
```

## Configuration Format

The generated G3Proxy configuration follows this structure:

```yaml
runtime:
  thread_number: 4

log: stdout

auditor:
  - name: default
    protocol_inspection: {}
    tls_cert_generator: {}
    tls_ticketer: {}
    tls_stream_dump: {}
    task_audit_ratio: 1.0

server:
  - name: http
    escaper: policy_escaper
    auditor: default
    type: http_proxy
    listen:
      address: "0.0.0.0:3128"
    tls_client: {}
  - name: socks
    escaper: policy_escaper
    auditor: default
    type: socks_proxy
    listen:
      address: "0.0.0.0:1080"

resolver:
  - name: default
    type: c-ares

escaper:
  - name: policy_escaper
    type: route_upstream
    url_filtering:
      block_categories: ["malware", "phishing"]
      warn_categories: ["social_media"]
      allow_categories: ["technology"]
      custom_rules:
        - name: "facebook_block"
          pattern: "*facebook*"
          action: "block"
          rule_type: "wildcard"
    default_next: direct
  - name: direct
    type: direct_fixed
    resolver: default
```

## Testing

### Running Integration Tests

```bash
# Install dependencies
npm install node-fetch

# Run the integration test
node test-proxy-integration.js
```

The test script will:
1. Check proxy status
2. Create test policies
3. Generate proxy configuration
4. Test policy evaluation
5. Test hot reload functionality
6. Test URL filtering

### Manual Testing

1. **Start the admin console**: `npm run dev`
2. **Navigate to Proxy Integration**: Click "Proxy Integration" in the sidebar
3. **Create policies**: Use the policy management interface
4. **Apply changes**: Click "Apply Policy Changes"
5. **Test URLs**: Enter URLs in the testing interface
6. **Monitor status**: Watch the real-time monitoring dashboard

## Event System

The hot reload manager emits events for monitoring and debugging:

```typescript
manager.on('reloadStarted', (data) => {
  console.log('Reload started:', data.status.message);
});

manager.on('reloadCompleted', (data) => {
  console.log('Reload completed:', data.status.message);
});

manager.on('reloadFailed', (data) => {
  console.error('Reload failed:', data.status.message);
});

manager.on('healthCheck', (data) => {
  console.log('Health check:', data.status.health);
});
```

## Error Handling

The system includes comprehensive error handling:

- **Configuration Validation**: Validates generated configurations before applying
- **Proxy Health Checks**: Monitors proxy status and health
- **Retry Logic**: Automatic retry for failed operations
- **Graceful Degradation**: Fallback mechanisms for failed services
- **Error Logging**: Detailed error logging and reporting

## Performance Considerations

- **Caching**: Configuration and policy results are cached
- **Rate Limiting**: API calls are rate-limited to prevent overload
- **Async Operations**: All operations are asynchronous for better performance
- **Health Monitoring**: Configurable monitoring intervals
- **Resource Cleanup**: Proper cleanup of resources and event listeners

## Security Considerations

- **Input Validation**: All inputs are validated before processing
- **Configuration Sanitization**: Generated configurations are sanitized
- **Access Control**: API endpoints include proper access controls
- **Audit Logging**: All operations are logged for audit purposes
- **Error Information**: Sensitive information is not exposed in error messages

## Troubleshooting

### Common Issues

1. **Proxy Not Responding**
   - Check if proxy is running
   - Verify port configuration
   - Check firewall settings

2. **Configuration Validation Errors**
   - Review policy configuration
   - Check for invalid patterns
   - Validate URL filtering rules

3. **Hot Reload Failures**
   - Check proxy logs
   - Verify configuration syntax
   - Ensure proxy supports hot reload

4. **URL Categorization Issues**
   - Check external API connectivity
   - Verify API keys and configuration
   - Review fallback mechanisms

### Debug Mode

Enable debug logging:

```typescript
// Set debug mode
process.env.DEBUG = 'proxy-integration:*';

// Or enable specific components
process.env.DEBUG = 'hot-reload-manager,proxy-config-generator';
```

## Future Enhancements

- **Distributed Configuration**: Support for multiple proxy instances
- **Advanced Analytics**: Detailed policy enforcement analytics
- **Machine Learning**: ML-based URL categorization
- **API Gateway Integration**: Integration with API gateways
- **Cloud Deployment**: Cloud-native deployment options
- **Advanced Monitoring**: Prometheus/Grafana integration
- **Policy Templates**: Pre-built policy templates
- **Bulk Operations**: Bulk policy management operations

## Contributing

When contributing to the proxy integration system:

1. Follow the existing code patterns
2. Add comprehensive tests
3. Update documentation
4. Consider performance implications
5. Ensure backward compatibility
6. Add proper error handling
7. Include logging and monitoring

## Support

For issues and questions:

1. Check the troubleshooting section
2. Review the API documentation
3. Check the proxy logs
4. Enable debug mode
5. Create an issue with detailed information
