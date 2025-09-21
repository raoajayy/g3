//! G3ICAP Protocol Definitions
//! 
//! This crate contains the protocol definitions and message structures
//! for the G3ICAP server implementation.

pub mod icap;
pub mod common;

// Re-export commonly used types
pub use icap::*;
pub use common::*;
