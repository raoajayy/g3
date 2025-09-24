//! ICAP Preview mode implementation
//!
//! This module handles ICAP preview mode for large content processing.

use crate::error::{IcapError, IcapResult};
use crate::protocol::common::{IcapRequest, IcapResponse};
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};

/// Preview mode handler
pub struct PreviewHandler {
    /// Maximum preview size
    #[allow(dead_code)]
    max_preview_size: usize,
}

impl PreviewHandler {
    /// Create a new preview handler
    pub fn new(max_preview_size: usize) -> Self {
        Self { max_preview_size }
    }

    /// Handle preview request
    pub async fn handle_preview(&self, request: &IcapRequest) -> IcapResult<PreviewResponse> {
        // Check if this is a preview request
        let preview_size = self.get_preview_size(request)?;
        
        if preview_size == 0 {
            // No preview requested
            return Ok(PreviewResponse::NoPreview);
        }

        // Check if content is small enough for immediate processing
        let content_size = self.get_content_size(request)?;
        
        if content_size <= preview_size {
            // Content is small enough, process immediately
            Ok(PreviewResponse::ProcessImmediately)
        } else {
            // Content is too large, need to use preview mode
            Ok(PreviewResponse::UsePreview {
                preview_size,
                content_size,
            })
        }
    }

    /// Get preview size from request headers
    fn get_preview_size(&self, request: &IcapRequest) -> IcapResult<usize> {
        if let Some(preview_header) = request.headers.get("preview") {
            let preview_str = preview_header.to_str()
                .map_err(|e| IcapError::protocol_simple(format!("Invalid preview header: {}", e)))?;
            
            preview_str.parse::<usize>()
                .map_err(|e| IcapError::protocol_simple(format!("Invalid preview size: {}", e)))
        } else {
            Ok(0)
        }
    }

    /// Get content size from request
    fn get_content_size(&self, request: &IcapRequest) -> IcapResult<usize> {
        // Check Content-Length header
        if let Some(length_header) = request.headers.get("content-length") {
            let length_str = length_header.to_str()
                .map_err(|e| IcapError::protocol_simple(format!("Invalid content-length header: {}", e)))?;
            
            return length_str.parse::<usize>()
                .map_err(|e| IcapError::protocol_simple(format!("Invalid content length: {}", e)));
        }

        // Check encapsulated body size
        if let Some(encapsulated) = &request.encapsulated {
            let mut total_size = 0;
            
            if let Some(req_body) = &encapsulated.req_body {
                total_size += req_body.len();
            }
            
            if let Some(res_body) = &encapsulated.res_body {
                total_size += res_body.len();
            }
            
            return Ok(total_size);
        }

        // Default to 0 if no size information available
        Ok(0)
    }

    /// Create preview response
    pub fn create_preview_response(&self, preview_size: usize) -> IcapResponse {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-preview\"".parse().unwrap());
        headers.insert("Preview", preview_size.to_string().parse().unwrap());
        headers.insert("Connection", "close".parse().unwrap());

        IcapResponse {
            status: StatusCode::CONTINUE,
            version: Version::HTTP_11,
            headers,
            body: Bytes::new(),
            encapsulated: None,
        }
    }

    /// Create final response after preview
    pub fn create_final_response(&self, status: StatusCode, body: Option<Bytes>) -> IcapResponse {
        let mut headers = HeaderMap::new();
        headers.insert("ISTag", "\"g3icap-final\"".parse().unwrap());

        IcapResponse {
            status,
            version: Version::HTTP_11,
            headers,
            body: body.unwrap_or_default(),
            encapsulated: None,
        }
    }
}

/// Preview response types
#[derive(Debug)]
pub enum PreviewResponse {
    /// No preview requested
    NoPreview,
    /// Process content immediately (small enough)
    ProcessImmediately,
    /// Use preview mode
    UsePreview {
        /// Size of preview to request
        preview_size: usize,
        /// Total content size
        content_size: usize,
    },
}
