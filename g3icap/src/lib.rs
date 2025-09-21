//! G3 ICAP Server Library
//!
//! This library provides a high-performance ICAP (Internet Content Adaptation Protocol)
//! server implementation for content filtering, virus scanning, and content adaptation.

// #![deny(missing_docs)] // Temporarily disabled for testing
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

pub mod audit;
pub mod auth;
pub mod config;
pub mod control;
pub mod error;
pub mod log;
pub mod opts;
pub mod protocol;
pub mod server;
pub mod serve;
pub mod service;
pub mod signal;
pub mod stats;
pub mod stat;
pub mod modules;
pub mod services;
pub mod pipeline;
pub mod version;

// Tests are located in the tests/ directory

pub use error::{IcapError, IcapResult};
