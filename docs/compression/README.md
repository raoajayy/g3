# G3ICAP Compression Documentation

This document provides comprehensive documentation for the compression features in G3ICAP, including configuration, usage examples, and best practices.

## Table of Contents

1. [Overview](#overview)
2. [Supported Algorithms](#supported-algorithms)
3. [Configuration](#configuration)
4. [Usage Examples](#usage-examples)
5. [Streaming Compression](#streaming-compression)
6. [Performance Considerations](#performance-considerations)
7. [Monitoring and Metrics](#monitoring-and-metrics)
8. [Troubleshooting](#troubleshooting)
9. [Best Practices](#best-practices)

## Overview

G3ICAP provides comprehensive compression support for ICAP messages, enabling efficient transmission of large content while maintaining protocol compliance. The compression system supports multiple algorithms, streaming compression, and detailed metrics collection.

### Key Features

- **Multiple Compression Algorithms**: gzip, deflate, brotli, zstd, identity
- **ICAP-Specific Handling**: Proper handling of ICAP headers and encapsulated data
- **Streaming Support**: Efficient compression/decompression of large messages
- **Configurable Thresholds**: Size-based compression decisions
- **Metrics Collection**: Detailed performance and compression statistics
- **Content-Type Awareness**: Different compression settings for different content types

## Supported Algorithms

### Gzip (RFC 1952)
- **Best for**: Text content, HTML, CSS, JavaScript, JSON
- **Compression ratio**: Good (typically 60-80% of original size)
- **Speed**: Fast compression and decompression
- **Browser support**: Universal
- **Default level**: 6

### Deflate (RFC 1951)
- **Best for**: Text content, API responses
- **Compression ratio**: Good (similar to gzip)
- **Speed**: Fast compression and decompression
- **Browser support**: Universal
- **Default level**: 6

### Brotli
- **Best for**: Text content, web assets
- **Compression ratio**: Excellent (typically 50-70% of original size)
- **Speed**: Slower compression, fast decompression
- **Browser support**: Modern browsers
- **Default level**: 6
- **Requires**: `brotli` feature flag

### Zstandard (Zstd)
- **Best for**: Large files, binary content
- **Compression ratio**: Excellent (typically 40-60% of original size)
- **Speed**: Very fast compression and decompression
- **Browser support**: Limited
- **Default level**: 6
- **Requires**: `zstd` feature flag

### Identity
- **Best for**: Already compressed content, small files
- **Compression ratio**: 100% (no compression)
- **Speed**: Instant
- **Use case**: Fallback when compression is not beneficial

## Configuration

### Basic Configuration

```yaml
compression:
  enabled: true
  min_size: 1024
  default_algorithm: gzip
  default_level: 6
  supported_algorithms:
    - gzip
    - deflate
    - identity
```

### Advanced Configuration

```yaml
compression:
  enabled: true
  min_size: 1024
  max_size: 10485760  # 10MB
  default_algorithm: gzip
  default_level: 6
  supported_algorithms:
    - gzip
    - deflate
    - brotli
    - zstd
    - identity
  
  algorithm_settings:
    gzip:
      level: 6
      enabled: true
      parameters:
        window_bits: "15"
        mem_level: "8"
    
    brotli:
      level: 6
      enabled: true
      parameters:
        quality: "6"
        window_size: "22"
  
  quality_settings:
    fast_level: 1
    balanced_level: 6
    best_level: 9
    fast_threshold: 1024
    balanced_threshold: 65536
  
  enable_streaming: true
  stream_buffer_size: 65536
  enable_metrics: true
```

### Content-Type Specific Configuration

```yaml
content_type_compression:
  text:
    enabled: true
    algorithms: [gzip, deflate]
    min_size: 512
  
  html:
    enabled: true
    algorithms: [gzip, deflate]
    min_size: 1024
  
  binary:
    enabled: false
    algorithms: [identity]
```

## Usage Examples

### Basic Compression

```rust
use g3icap::compression::{
    CompressionManager, CompressionConfig, CompressionAlgorithm, CompressionLevel
};

// Create configuration
let config = CompressionConfig::default();
let mut manager = CompressionManager::new(config);

// Compress ICAP request
let compressed_request = manager.compress_request(&request, None)?;

// Decompress ICAP request
let decompressed_request = manager.decompress_request(&compressed_request)?;
```

### Custom Algorithm Selection

```rust
// Compress with specific algorithm
let compressed = manager.compress_request(
    &request, 
    Some(CompressionAlgorithm::Brotli)
)?;

// Compress with custom level
let mut config = CompressionConfig::default();
config.default_level = CompressionLevel::best();
let mut manager = CompressionManager::new(config);
```

### Streaming Compression

```rust
use g3icap::compression::StreamingCompressionManager;

let manager = StreamingCompressionManager::new(config);

// Compress large data
let compressed_data = manager.compress_request_streaming(
    reader,
    CompressionAlgorithm::Gzip,
    CompressionLevel::default(),
)?;

// Decompress large data
let decompressed_data = manager.decompress_response_streaming(
    reader,
    CompressionAlgorithm::Gzip,
)?;
```

### Chunked Compression

```rust
use g3icap::compression::ChunkedCompression;

let chunked = ChunkedCompression::new(
    1024,
    CompressionAlgorithm::Gzip,
    CompressionLevel::default(),
);

// Compress with chunks
let compressed = chunked.compress_with_chunks(reader)?;

// Decompress with chunks
let decompressed = chunked.decompress_with_chunks(reader)?;
```

## Streaming Compression

Streaming compression is designed for handling large ICAP messages efficiently without loading the entire content into memory.

### Features

- **Memory Efficient**: Processes data in chunks
- **Configurable Buffer Size**: Adjustable chunk size for optimal performance
- **Multiple Algorithms**: Support for all compression algorithms
- **Error Handling**: Robust error handling for streaming operations

### Usage

```rust
use g3icap::compression::StreamingCompressionManager;

let config = CompressionConfig {
    enable_streaming: true,
    stream_buffer_size: 64 * 1024, // 64KB chunks
    ..Default::default()
};

let manager = StreamingCompressionManager::new(config);

// Compress large file
let file = std::fs::File::open("large_file.txt")?;
let compressed = manager.compress_request_streaming(
    file,
    CompressionAlgorithm::Gzip,
    CompressionLevel::default(),
)?;
```

## Performance Considerations

### Compression Levels

| Level | Speed | Ratio | Use Case |
|-------|-------|-------|----------|
| 1-3   | Fast  | Low   | Real-time applications |
| 4-6   | Balanced | Good | General purpose |
| 7-9   | Slow  | High  | Batch processing |

### Size Thresholds

- **Min Size**: Don't compress files smaller than this threshold
- **Max Size**: Don't compress files larger than this threshold
- **Quality Thresholds**: Use different compression levels based on file size

### Memory Usage

- **Buffer Size**: Larger buffers use more memory but may improve compression
- **Streaming**: Reduces memory usage for large files
- **Chunking**: Balances memory usage and compression efficiency

### CPU Usage

- **Algorithm Choice**: Brotli and Zstd use more CPU than gzip/deflate
- **Compression Level**: Higher levels use more CPU
- **Threading**: Use multiple threads for parallel compression

## Monitoring and Metrics

### Available Metrics

- **Total Operations**: Number of compression/decompression operations
- **Bytes Processed**: Total bytes compressed/decompressed
- **Compression Ratios**: Average, best, and worst compression ratios
- **Performance**: Compression/decompression times and throughput
- **Algorithm Statistics**: Per-algorithm performance metrics

### Metrics Export

```rust
// Get metrics
let metrics = manager.get_metrics();
let stats = metrics.get_stats();

// Export to JSON
let json = metrics.to_json()?;

// Export to CSV
let csv = metrics.to_csv();

// Export to Prometheus
let exporter = PrometheusMetricsExporter;
let prometheus = exporter.export(&metrics)?;
```

### Prometheus Metrics

```
# HELP icap_compression_operations_total Total number of compression operations
# TYPE icap_compression_operations_total counter
icap_compression_operations_total{operation="compression"} 1000
icap_compression_operations_total{operation="decompression"} 1000

# HELP icap_compression_bytes_total Total bytes processed
# TYPE icap_compression_bytes_total counter
icap_compression_bytes_total{operation="compression"} 1048576
icap_compression_bytes_total{operation="decompression"} 1048576

# HELP icap_compression_ratio_ratio Compression ratio
# TYPE icap_compression_ratio_ratio gauge
icap_compression_ratio_ratio{algorithm="gzip"} 0.65
```

## Troubleshooting

### Common Issues

#### Compression Not Working

**Symptoms**: Files are not being compressed despite configuration

**Solutions**:
1. Check if compression is enabled: `compression.enabled: true`
2. Verify minimum size threshold: `compression.min_size`
3. Ensure algorithm is supported: `compression.supported_algorithms`
4. Check content type settings: `content_type_compression`

#### Poor Compression Ratios

**Symptoms**: Compression ratios are higher than expected

**Solutions**:
1. Increase compression level: `compression.default_level`
2. Use better algorithm: `brotli` or `zstd`
3. Check content type: Some content compresses better than others
4. Verify data is not already compressed

#### High CPU Usage

**Symptoms**: Compression is using too much CPU

**Solutions**:
1. Lower compression level: Use levels 1-3 for speed
2. Use faster algorithms: `gzip` or `deflate`
3. Increase size thresholds: Compress only larger files
4. Use streaming compression: Reduces memory pressure

#### Memory Issues

**Symptoms**: High memory usage during compression

**Solutions**:
1. Enable streaming compression: `compression.enable_streaming: true`
2. Reduce buffer size: `compression.stream_buffer_size`
3. Use chunked compression: Process data in smaller chunks
4. Check for memory leaks in application code

### Debug Logging

Enable debug logging to troubleshoot compression issues:

```yaml
logging:
  level: debug
  log_compression: true
  log_ratios: true
  log_times: true
```

### Performance Profiling

Use metrics to identify performance bottlenecks:

```rust
let metrics = manager.get_metrics();
let stats = metrics.get_stats();

println!("Average compression time: {:.2} μs", stats.average_compression_time);
println!("Average compression ratio: {:.4}", stats.average_compression_ratio);
```

## Best Practices

### Algorithm Selection

1. **Text Content**: Use `gzip` or `deflate` for compatibility
2. **Web Assets**: Use `brotli` for better compression
3. **Large Files**: Use `zstd` for speed and compression
4. **API Responses**: Use `gzip` for universal support

### Size Thresholds

1. **Min Size**: Set to 1KB for text, 512B for JSON
2. **Max Size**: Set to 10MB to avoid compressing very large files
3. **Quality Thresholds**: Use different levels based on file size

### Performance Optimization

1. **Use Streaming**: For files larger than 1MB
2. **Chunk Size**: Use 64KB chunks for optimal performance
3. **Threading**: Use multiple threads for parallel compression
4. **Caching**: Cache compressed content when possible

### Monitoring

1. **Track Ratios**: Monitor compression ratios for effectiveness
2. **Monitor Performance**: Track compression times and throughput
3. **Alert on Issues**: Set up alerts for compression failures
4. **Regular Review**: Review metrics regularly for optimization

### Security Considerations

1. **Validate Input**: Ensure input data is safe before compression
2. **Resource Limits**: Set limits on compression operations
3. **Error Handling**: Handle compression errors gracefully
4. **Logging**: Log compression operations for audit trails

## Examples

See the [compression example](../examples/compression_example.rs) for a complete working example of all compression features.

## API Reference

For detailed API documentation, see the [compression module documentation](../../g3icap/src/compression/).

## Support

For questions and support regarding compression features:

- **Documentation**: This guide and API documentation
- **Issues**: GitHub Issues for bug reports
- **Discussions**: GitHub Discussions for questions
- **Email**: support@g3icap.com
