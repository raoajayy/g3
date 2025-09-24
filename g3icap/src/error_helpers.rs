/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Error helper functions for backward compatibility
//!
//! This module provides helper functions to maintain backward compatibility
//! with the old error handling patterns while using the new structured errors.

use crate::error::IcapError;

/// Helper functions for creating errors with the old API
impl IcapError {
    /// Create a simple protocol error (backward compatibility)
    pub fn protocol_simple(message: impl Into<String>) -> Self {
        Self::Protocol {
            message: message.into(),
            protocol: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple network error (backward compatibility)
    pub fn network_simple(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            address: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple service error (backward compatibility)
    pub fn service_simple(message: impl Into<String>) -> Self {
        Self::Service {
            message: message.into(),
            service: None,
            context: None,
            source: None,
        }
    }

    /// Create a simple config error (backward compatibility)
    pub fn config_simple(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            context: None,
            source: None,
        }
    }
}
