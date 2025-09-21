/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Authentication operations for ICAP server

use std::sync::Arc;

use super::{AuthContext, AuthResult, IcapAuthHandler};

/// Authentication operations trait
pub trait IcapAuthOps {
    /// Get authentication handler
    fn get_auth_handler(&self) -> Option<Arc<dyn IcapAuthHandler>>;
    
    /// Authenticate user
    fn authenticate(&self, context: &AuthContext) -> AuthResult {
        if let Some(handler) = self.get_auth_handler() {
            handler.authenticate(context)
        } else {
            // No authentication required
            AuthResult::success("anonymous".to_string(), Vec::new())
        }
    }
    
    /// Check authorization
    fn is_authorized(&self, username: &str, action: &str) -> bool {
        if let Some(handler) = self.get_auth_handler() {
            handler.is_authorized(username, action)
        } else {
            // No authorization required
            true
        }
    }
}

/// Default authentication operations (no-op)
pub struct DefaultIcapAuthOps;

impl IcapAuthOps for DefaultIcapAuthOps {
    fn get_auth_handler(&self) -> Option<Arc<dyn IcapAuthHandler>> {
        None
    }
}
