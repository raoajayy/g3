//! Streaming Support for Large Content Processing
//! 
//! This module provides streaming capabilities for processing large ICAP content
//! without loading everything into memory. It supports both request and response
//! streaming with proper backpressure handling.

use crate::error::IcapError;
use crate::protocol::chunked::ChunkedParser;
use bytes::{Bytes, BytesMut, Buf};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

/// Streaming ICAP content processor
pub struct StreamingProcessor {
    chunked_parser: ChunkedParser,
    buffer: BytesMut,
    max_buffer_size: usize,
    is_complete: bool,
}

impl StreamingProcessor {
    /// Create a new streaming processor
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            chunked_parser: ChunkedParser::new(),
            buffer: BytesMut::with_capacity(8192), // 8KB initial buffer
            max_buffer_size,
            is_complete: false,
        }
    }
    
    /// Process streaming data and return decoded chunks
    pub async fn process_chunk<R>(&mut self, reader: &mut R) -> Result<Option<Bytes>, IcapError>
    where
        R: AsyncRead + Unpin,
    {
        if self.is_complete {
            return Ok(None);
        }
        
        // Read data into buffer
        let mut temp_buffer = vec![0u8; 4096];
        let bytes_read = reader.read(&mut temp_buffer).await
            .map_err(|e| IcapError::Io(e))?;
        
        if bytes_read == 0 {
            // No more data available
            if self.buffer.is_empty() {
                return Ok(None);
            }
            // Process remaining buffer
            return self.process_buffer().await;
        }
        
        // Add to buffer
        self.buffer.extend_from_slice(&temp_buffer[..bytes_read]);
        
        // Check buffer size limit
        if self.buffer.len() > self.max_buffer_size {
            return Err(IcapError::protocol_error(
                &format!("Buffer size exceeded limit: {} bytes", self.max_buffer_size),
                "STREAMING"
            ));
        }
        
        // Process buffer
        self.process_buffer().await
    }
    
    /// Process the internal buffer
    async fn process_buffer(&mut self) -> Result<Option<Bytes>, IcapError> {
        if self.buffer.is_empty() {
            return Ok(None);
        }
        
        // Try to parse chunked data
        match self.chunked_parser.parse_chunk(&self.buffer) {
            Ok((decoded_data, consumed)) => {
                // Remove consumed data from buffer
                self.buffer.advance(consumed);
                
                // Check if parsing is complete
                if self.chunked_parser.is_complete() {
                    self.is_complete = true;
                }
                
                if decoded_data.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(Bytes::from(decoded_data)))
                }
            }
            Err(e) => {
                // If we can't parse yet, we need more data
                if self.buffer.len() < self.max_buffer_size {
                    Ok(None) // Need more data
                } else {
                    Err(IcapError::from(e))
                }
            }
        }
    }
    
    /// Check if processing is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
    
    /// Reset the processor for reuse
    pub fn reset(&mut self) {
        self.chunked_parser.reset();
        self.buffer.clear();
        self.is_complete = false;
    }
    
    /// Get current buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

/// Streaming ICAP request processor
pub struct StreamingRequestProcessor {
    processor: StreamingProcessor,
    content_filter: Option<Box<dyn ContentFilter + Send + Sync>>,
}

impl StreamingRequestProcessor {
    /// Create a new streaming request processor
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            processor: StreamingProcessor::new(max_buffer_size),
            content_filter: None,
        }
    }
    
    /// Set content filter for the processor
    pub fn set_content_filter<F>(&mut self, filter: F)
    where
        F: ContentFilter + Send + Sync + 'static,
    {
        self.content_filter = Some(Box::new(filter));
    }
    
    /// Process streaming request data
    pub async fn process_request_chunk<R>(&mut self, reader: &mut R) -> Result<Option<Bytes>, IcapError>
    where
        R: AsyncRead + Unpin,
    {
        match self.processor.process_chunk(reader).await? {
            Some(data) => {
                // Apply content filter if available
                if let Some(ref filter) = self.content_filter {
                    match filter.filter_request_data(&data).await {
                        Ok(filtered_data) => Ok(Some(filtered_data)),
                        Err(e) => Err(IcapError::content_filter_error(&e.to_string())),
                    }
                } else {
                    Ok(Some(data))
                }
            }
            None => Ok(None),
        }
    }
    
    /// Check if processing is complete
    pub fn is_complete(&self) -> bool {
        self.processor.is_complete()
    }
}

/// Streaming ICAP response processor
pub struct StreamingResponseProcessor {
    processor: StreamingProcessor,
    content_filter: Option<Box<dyn ContentFilter + Send + Sync>>,
}

impl StreamingResponseProcessor {
    /// Create a new streaming response processor
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            processor: StreamingProcessor::new(max_buffer_size),
            content_filter: None,
        }
    }
    
    /// Set content filter for the processor
    pub fn set_content_filter<F>(&mut self, filter: F)
    where
        F: ContentFilter + Send + Sync + 'static,
    {
        self.content_filter = Some(Box::new(filter));
    }
    
    /// Process streaming response data
    pub async fn process_response_chunk<R>(&mut self, reader: &mut R) -> Result<Option<Bytes>, IcapError>
    where
        R: AsyncRead + Unpin,
    {
        match self.processor.process_chunk(reader).await? {
            Some(data) => {
                // Apply content filter if available
                if let Some(ref filter) = self.content_filter {
                    match filter.filter_response_data(&data).await {
                        Ok(filtered_data) => Ok(Some(filtered_data)),
                        Err(e) => Err(IcapError::content_filter_error(&e.to_string())),
                    }
                } else {
                    Ok(Some(data))
                }
            }
            None => Ok(None),
        }
    }
    
    /// Check if processing is complete
    pub fn is_complete(&self) -> bool {
        self.processor.is_complete()
    }
}

/// Content filter trait for streaming data
#[async_trait::async_trait]
pub trait ContentFilter: Send + Sync {
    /// Filter request data
    async fn filter_request_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Filter response data
    async fn filter_response_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;
}

/// Simple pass-through content filter
pub struct PassThroughFilter;

#[async_trait::async_trait]
impl ContentFilter for PassThroughFilter {
    async fn filter_request_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Bytes::from(data.to_vec()))
    }
    
    async fn filter_response_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Bytes::from(data.to_vec()))
    }
}

/// Keyword-based content filter
pub struct KeywordFilter {
    blocked_keywords: Vec<String>,
    replacement: String,
}

impl KeywordFilter {
    /// Create a new keyword filter
    pub fn new(blocked_keywords: Vec<String>, replacement: String) -> Self {
        Self {
            blocked_keywords,
            replacement,
        }
    }
}

#[async_trait::async_trait]
impl ContentFilter for KeywordFilter {
    async fn filter_request_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        let mut content = String::from_utf8(data.to_vec())?;
        
        for keyword in &self.blocked_keywords {
            content = content.replace(keyword, &self.replacement);
        }
        
        Ok(Bytes::from(content.into_bytes()))
    }
    
    async fn filter_response_data(&self, data: &[u8]) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        let mut content = String::from_utf8(data.to_vec())?;
        
        for keyword in &self.blocked_keywords {
            content = content.replace(keyword, &self.replacement);
        }
        
        Ok(Bytes::from(content.into_bytes()))
    }
}

/// Streaming ICAP connection handler
pub struct StreamingConnectionHandler {
    request_processor: StreamingRequestProcessor,
    response_processor: StreamingResponseProcessor,
    max_connections: usize,
    active_connections: usize,
}

impl StreamingConnectionHandler {
    /// Create a new streaming connection handler
    pub fn new(max_buffer_size: usize, max_connections: usize) -> Self {
        Self {
            request_processor: StreamingRequestProcessor::new(max_buffer_size),
            response_processor: StreamingResponseProcessor::new(max_buffer_size),
            max_connections,
            active_connections: 0,
        }
    }
    
    /// Handle a new connection
    pub async fn handle_connection<R, W>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), IcapError>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        if self.active_connections >= self.max_connections {
            return Err(IcapError::resource_exhausted_simple("Maximum connections exceeded"));
        }
        
        self.active_connections += 1;
        
        // Process request data
        while let Some(data) = self.request_processor.process_request_chunk(reader).await? {
            // Write processed data
            writer.write_all(&data).await
                .map_err(|e| IcapError::Io(e))?;
        }
        
        // Process response data
        while let Some(data) = self.response_processor.process_response_chunk(reader).await? {
            // Write processed data
            writer.write_all(&data).await
                .map_err(|e| IcapError::Io(e))?;
        }
        
        self.active_connections -= 1;
        Ok(())
    }
    
    /// Get current active connections
    pub fn active_connections(&self) -> usize {
        self.active_connections
    }
    
    /// Check if we can accept new connections
    pub fn can_accept_connection(&self) -> bool {
        self.active_connections < self.max_connections
    }
}

/// Simple stream processor for AsyncRead
pub struct AsyncReadProcessor<R> {
    reader: R,
    processor: StreamingProcessor,
}

impl<R> AsyncReadProcessor<R>
where
    R: AsyncRead + Unpin,
{
    /// Create a new stream processor from AsyncRead
    pub fn new(reader: R, max_buffer_size: usize) -> Self {
        Self {
            reader,
            processor: StreamingProcessor::new(max_buffer_size),
        }
    }
    
    /// Process the next chunk from the reader
    pub async fn next_chunk(&mut self) -> Result<Option<Bytes>, IcapError> {
        self.processor.process_chunk(&mut self.reader).await
    }
    
    /// Check if processing is complete
    pub fn is_complete(&self) -> bool {
        self.processor.is_complete()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;
    use std::io::Cursor;
    
    #[tokio::test]
    async fn test_streaming_processor() {
        let mut processor = StreamingProcessor::new(1024);
        let data = b"1a\r\nThis is the first chunk\r\n15\r\nSecond chunk here\r\n0\r\n\r\n";
        let mut cursor = Cursor::new(data);
        
        let mut result = Vec::new();
        while let Some(chunk) = processor.process_chunk(&mut cursor).await.unwrap() {
            result.extend_from_slice(&chunk);
        }
        
        assert_eq!(result, b"This is the first chunkSecond chunk here");
        assert!(processor.is_complete());
    }
    
    #[tokio::test]
    async fn test_keyword_filter() {
        let filter = KeywordFilter::new(
            vec!["badword".to_string(), "blocked".to_string()],
            "[FILTERED]".to_string()
        );
        
        let data = b"This contains badword and blocked content";
        let result = filter.filter_request_data(data).await.unwrap();
        
        assert_eq!(result, Bytes::from("This contains [FILTERED] and [FILTERED] content"));
    }
    
    #[tokio::test]
    async fn test_streaming_connection_handler() {
        let mut handler = StreamingConnectionHandler::new(1024, 10);
        let data = b"1a\r\nThis is test data\r\n0\r\n\r\n";
        let mut reader = Cursor::new(data);
        let mut writer = Vec::new();
        
        handler.handle_connection(&mut reader, &mut writer).await.unwrap();
        
        assert_eq!(writer, b"This is test data");
    }
    
    #[tokio::test]
    async fn test_async_read_processor() {
        let data = b"1a\r\nThis is test data\r\n0\r\n\r\n";
        let reader = Cursor::new(data);
        let mut processor = AsyncReadProcessor::new(reader, 1024);
        
        let mut result = Vec::new();
        while let Some(chunk) = processor.next_chunk().await.unwrap() {
            result.extend_from_slice(&chunk);
        }
        
        assert_eq!(result, b"This is test data");
        assert!(processor.is_complete());
    }
}
