/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! # Arcus Policy Framework
//! 
//! A comprehensive policy management system for G3 Secure Web Gateway.
//! This framework provides YAML-based policy configuration with automatic
//! G3proxy configuration generation.

pub mod policy;
pub mod config;
pub mod engine;
pub mod filtering;
pub mod security;
pub mod traffic;
pub mod user;
pub mod monitoring;
pub mod integration;

pub use policy::PolicyManager;
pub use config::ConfigGenerator;
pub use engine::PolicyEngine;

/// Policy framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Policy API version
pub const API_VERSION: &str = "arcus.v1";

/// Default policy schema version
pub const SCHEMA_VERSION: &str = "1.0";
