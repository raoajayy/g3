/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! G3 ICAP Server Library
//!
//! This library provides a high-performance ICAP (Internet Content Adaptation Protocol)
//! server implementation for content filtering, virus scanning, and content adaptation.
//!
//! # Features
//!
//! - **High Performance**: Built on tokio async runtime for maximum throughput
//! - **Content Filtering**: Advanced content filtering and virus scanning
//! - **Protocol Compliance**: Full RFC 3507 ICAP protocol compliance
//! - **Modular Architecture**: Pluggable modules for different content adaptation needs
//! - **Comprehensive Logging**: Detailed audit and operational logging
//! - **Configuration Management**: Flexible YAML-based configuration
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use g3icap::{IcapServer, IcapError};
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), IcapError> {
//!     let server = IcapServer::new("0.0.0.0:1344".parse()?)?;
//!     server.start().await?;
//!     Ok(())
//! }
//! ```

#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unreachable)]

// Core modules following g3proxy patterns
pub mod audit;
pub mod auth;
pub mod config;
pub mod control;
pub mod opts;
pub mod protocol;
pub mod server;
pub mod serve;
pub mod signal;
pub mod stat;

// ICAP-specific modules
pub mod modules;
pub mod pipeline;

// Internal modules (not part of public API)
mod error;
mod log;
mod service;
mod services;
mod stats;
mod version;

// Re-export commonly used types
pub use error::{IcapError, IcapResult};
pub use server::IcapServer;
