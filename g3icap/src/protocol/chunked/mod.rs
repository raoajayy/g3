//! Chunked Transfer Encoding Parser for ICAP Protocol
//! 
//! This module implements RFC 3507 compliant chunked transfer encoding
//! for ICAP encapsulated HTTP bodies. All encapsulated HTTP bodies MUST
//! use chunked transfer encoding according to the ICAP specification.

use crate::error::IcapError;
use bytes::Bytes;
use std::str;

/// Chunked transfer encoding parser with state machine
#[derive(Debug, Clone)]
pub struct ChunkedParser {
    state: ChunkState,
    current_chunk_size: usize,
    current_chunk_read: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum ChunkState {
    ReadingSize,
    ReadingChunk,
    ReadingTrailers,
    Complete,
}

/// Error types for chunked parsing
#[derive(Debug, Clone, thiserror::Error)]
pub enum ChunkedParseError {
    #[error("Invalid chunk size encoding: {0}")]
    InvalidChunkSize(String),
    
    #[error("Invalid chunked encoding format")]
    InvalidEncoding,
    
    #[error("Unexpected end of data")]
    UnexpectedEnd,
    
    #[error("Chunk size too large: {0}")]
    ChunkSizeTooLarge(usize),
    
    #[error("Invalid trailer format")]
    InvalidTrailer,
}

impl From<ChunkedParseError> for IcapError {
    fn from(err: ChunkedParseError) -> Self {
        IcapError::protocol_error(&err.to_string(), "CHUNKED")
    }
}

impl ChunkedParser {
    /// Create a new chunked parser
    pub fn new() -> Self {
        Self {
            state: ChunkState::ReadingSize,
            current_chunk_size: 0,
            current_chunk_read: 0,
        }
    }
    
    /// Parse chunked data from input buffer
    /// Returns (decoded_data, bytes_consumed)
    pub fn parse_chunk(&mut self, input: &[u8]) -> Result<(Vec<u8>, usize), ChunkedParseError> {
        let mut output = Vec::new();
        let mut consumed = 0;
        let mut pos = 0;
        
        while pos < input.len() {
            match self.state {
                ChunkState::ReadingSize => {
                    if let Some(crlf_pos) = find_crlf(&input[pos..]) {
                        let size_str = str::from_utf8(&input[pos..pos + crlf_pos])
                            .map_err(|_| ChunkedParseError::InvalidEncoding)?;
                        
                        // Parse hexadecimal chunk size
                        self.current_chunk_size = usize::from_str_radix(size_str.trim(), 16)
                            .map_err(|e| ChunkedParseError::InvalidChunkSize(e.to_string()))?;
                        
                        // Validate chunk size (prevent excessive memory usage)
                        if self.current_chunk_size > 1024 * 1024 * 1024 { // 1GB limit
                            return Err(ChunkedParseError::ChunkSizeTooLarge(self.current_chunk_size));
                        }
                        
                        pos += crlf_pos + 2; // Skip CRLF
                        consumed = pos;
                        
                        if self.current_chunk_size == 0 {
                            self.state = ChunkState::ReadingTrailers;
                        } else {
                            self.state = ChunkState::ReadingChunk;
                            self.current_chunk_read = 0;
                        }
                    } else {
                        break; // Need more data
                    }
                },
                
                ChunkState::ReadingChunk => {
                    let remaining_in_chunk = self.current_chunk_size - self.current_chunk_read;
                    let available = input.len() - pos;
                    let to_read = std::cmp::min(remaining_in_chunk, available);
                    
                    output.extend_from_slice(&input[pos..pos + to_read]);
                    pos += to_read;
                    self.current_chunk_read += to_read;
                    
                    if self.current_chunk_read == self.current_chunk_size {
                        // Need to read trailing CRLF
                        if pos + 1 < input.len() && &input[pos..pos + 2] == b"\r\n" {
                            pos += 2;
                            consumed = pos;
                            self.state = ChunkState::ReadingSize;
                        } else {
                            break; // Need more data for trailing CRLF
                        }
                    }
                },
                
                ChunkState::ReadingTrailers => {
                    if let Some(end_pos) = find_double_crlf(&input[pos..]) {
                        pos += end_pos + 4; // Skip trailers and final CRLF
                        consumed = pos;
                        self.state = ChunkState::Complete;
                        break;
                    } else {
                        break; // Need more data
                    }
                },
                
                ChunkState::Complete => {
                    break; // No more processing needed
                }
            }
        }
        
        Ok((output, consumed))
    }
    
    /// Check if parsing is complete
    pub fn is_complete(&self) -> bool {
        self.state == ChunkState::Complete
    }
    
    /// Reset parser to initial state
    pub fn reset(&mut self) {
        self.state = ChunkState::ReadingSize;
        self.current_chunk_size = 0;
        self.current_chunk_read = 0;
    }
    
    /// Get current parsing state
    pub fn state(&self) -> &ChunkState {
        &self.state
    }
}

/// Encode data as chunked transfer encoding
pub fn encode_chunked(data: &[u8]) -> Bytes {
    if data.is_empty() {
        return Bytes::from("0\r\n\r\n");
    }
    
    let mut result = Vec::new();
    
    // Split data into chunks (max 8KB per chunk for efficiency)
    const CHUNK_SIZE: usize = 8192;
    let mut pos = 0;
    
    while pos < data.len() {
        let chunk_end = std::cmp::min(pos + CHUNK_SIZE, data.len());
        let chunk_data = &data[pos..chunk_end];
        
        // Write chunk size in hexadecimal
        let size_hex = format!("{:x}\r\n", chunk_data.len());
        result.extend_from_slice(size_hex.as_bytes());
        
        // Write chunk data
        result.extend_from_slice(chunk_data);
        result.extend_from_slice(b"\r\n");
        
        pos = chunk_end;
    }
    
    // Write final zero-length chunk
    result.extend_from_slice(b"0\r\n\r\n");
    
    Bytes::from(result)
}

/// Find CRLF sequence in data
fn find_crlf(data: &[u8]) -> Option<usize> {
    data.windows(2).position(|window| window == b"\r\n")
}

/// Find double CRLF sequence in data
fn find_double_crlf(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|window| window == b"\r\n\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunked_parsing() {
        let chunked_data = b"1a\r\nThis is the first chunk\r\n15\r\nSecond chunk here\r\n0\r\n\r\n";
        let mut parser = ChunkedParser::new();
        
        let (decoded, consumed) = parser.parse_chunk(chunked_data).unwrap();
        assert_eq!(decoded, b"This is the first chunkSecond chunk here");
        assert_eq!(consumed, chunked_data.len());
        assert!(parser.is_complete());
    }
    
    #[test]
    fn test_empty_chunked() {
        let chunked_data = b"0\r\n\r\n";
        let mut parser = ChunkedParser::new();
        
        let (decoded, consumed) = parser.parse_chunk(chunked_data).unwrap();
        assert_eq!(decoded, b"");
        assert_eq!(consumed, chunked_data.len());
        assert!(parser.is_complete());
    }
    
    #[test]
    fn test_large_chunk() {
        let large_data = "x".repeat(10000);
        let chunk_size = format!("{:x}", large_data.len());
        let chunked_data = format!("{}\r\n{}\r\n0\r\n\r\n", chunk_size, large_data);
        
        let mut parser = ChunkedParser::new();
        let (decoded, consumed) = parser.parse_chunk(chunked_data.as_bytes()).unwrap();
        
        assert_eq!(decoded, large_data.as_bytes());
        assert_eq!(consumed, chunked_data.len());
        assert!(parser.is_complete());
    }
    
    #[test]
    fn test_chunked_encoding() {
        let data = b"Hello, World!";
        let encoded = encode_chunked(data);
        let expected = b"d\r\nHello, World!\r\n0\r\n\r\n";
        
        assert_eq!(encoded.as_ref(), expected);
    }
    
    #[test]
    fn test_incremental_parsing() {
        let chunked_data = b"1a\r\nThis is the first chunk\r\n15\r\nSecond chunk here\r\n0\r\n\r\n";
        let mut parser = ChunkedParser::new();
        
        // Parse first part
        let (decoded1, consumed1) = parser.parse_chunk(&chunked_data[..20]).unwrap();
        assert_eq!(decoded1, b"This is the first chunk");
        assert_eq!(consumed1, 20);
        assert!(!parser.is_complete());
        
        // Parse remaining part
        let (decoded2, consumed2) = parser.parse_chunk(&chunked_data[20..]).unwrap();
        assert_eq!(decoded2, b"Second chunk here");
        assert_eq!(consumed2, chunked_data.len() - 20);
        assert!(parser.is_complete());
    }
    
    #[test]
    fn test_invalid_chunk_size() {
        let invalid_data = b"invalid\r\nchunk data\r\n0\r\n\r\n";
        let mut parser = ChunkedParser::new();
        
        let result = parser.parse_chunk(invalid_data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ChunkedParseError::InvalidChunkSize(_)));
    }
    
    #[test]
    fn test_chunk_size_too_large() {
        let large_size = "1000000000"; // 1GB in hex
        let data = format!("{}\r\n{}\r\n0\r\n\r\n", large_size, "x".repeat(1000));
        let mut parser = ChunkedParser::new();
        
        let result = parser.parse_chunk(data.as_bytes());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ChunkedParseError::ChunkSizeTooLarge(_)));
    }
}
