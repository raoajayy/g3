//! REQMOD (Request Modification) implementation
//!
//! This module handles the REQMOD ICAP method for modifying HTTP requests.

use crate::error::IcapError;
use crate::protocol::common::{IcapRequest, IcapResponse, IcapMethod};
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};
use async_trait::async_trait;

/// REQMOD handler trait
#[async_trait]
pub trait ReqmodHandler: Send + Sync {
    /// Handle REQMOD request
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError>;
}

/// Default REQMOD handler implementation
pub struct DefaultReqmodHandler;

#[async_trait]
impl ReqmodHandler for DefaultReqmodHandler {
    async fn handle_reqmod(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Default implementation - just pass through the request
        Ok(IcapResponse {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: Bytes::new(),
            encapsulated: request.encapsulated.clone(),
        })
    }
}

/// REQMOD service
pub struct ReqmodService {
    handler: Box<dyn ReqmodHandler>,
}

impl ReqmodService {
    /// Create a new REQMOD service
    pub fn new(handler: Box<dyn ReqmodHandler>) -> Self {
        Self { handler }
    }

    /// Process REQMOD request
    pub async fn process_request(&self, request: &IcapRequest) -> Result<IcapResponse, IcapError> {
        // Validate that this is a REQMOD request
        if request.method != IcapMethod::Reqmod {
            return Err(IcapError::protocol_error("Expected REQMOD method", "ICAP"));
        }

        // Call the handler
        self.handler.handle_reqmod(request).await
    }
}
