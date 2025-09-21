/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Service management system for G3ICAP
//! 
//! This module provides service management capabilities inspired by c-icap server,
//! including service registration, health monitoring, and load balancing.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use anyhow::Result;
// use async_trait::async_trait;

use crate::protocol::common::{IcapMethod, IcapRequest, IcapResponse};
use crate::modules::{IcapModule, ModuleError};

/// Service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service path
    pub path: String,
    /// Supported ICAP methods
    pub methods: Vec<IcapMethod>,
    /// Preview size limit
    pub preview_size: usize,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum connections
    pub max_connections: usize,
    /// Enable health checks
    pub health_check_enabled: bool,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
}

/// Load balancing strategies
#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    /// Round-robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin(Vec<u32>),
    /// Random
    Random,
}

/// Service metrics
#[derive(Debug, Clone, Default)]
pub struct ServiceMetrics {
    /// Total requests processed
    pub requests_total: u64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time
    pub average_response_time: Duration,
    /// Error rate
    pub error_rate: f64,
    /// Active connections
    pub active_connections: usize,
    /// Total connections
    pub total_connections: u64,
    /// Connection errors
    pub connection_errors: u64,
    /// Memory usage
    pub memory_usage: usize,
    /// CPU usage
    pub cpu_usage: f64,
    /// Last activity
    pub last_activity: Option<Instant>,
    /// Health status
    pub is_healthy: bool,
}

/// Service instance
pub struct ServiceInstance {
    /// Service ID
    pub id: String,
    /// Service configuration
    pub config: ServiceConfig,
    /// Service module
    pub module: Box<dyn IcapModule>,
    /// Service metrics
    pub metrics: ServiceMetrics,
    /// Last health check
    pub last_health_check: Option<Instant>,
    /// Connection count
    pub connection_count: usize,
}

/// Service manager
#[derive(Clone)]
pub struct ServiceManager {
    services: Arc<RwLock<HashMap<String, ServiceInstance>>>,
    health_checker: HealthChecker,
    #[allow(dead_code)]
    load_balancer: LoadBalancer,
}

impl ServiceManager {
    /// Create new service manager
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            health_checker: HealthChecker::new(),
            load_balancer: LoadBalancer::new(),
        }
    }
    
    /// Register a service
    pub async fn register_service(
        &self,
        config: ServiceConfig,
        module: Box<dyn IcapModule>,
    ) -> Result<(), ServiceError> {
        let service_id = format!("{}-{}", config.name, uuid::Uuid::new_v4());
        let instance = ServiceInstance {
            id: service_id,
            config: config.clone(),
            module,
            metrics: ServiceMetrics::default(),
            last_health_check: None,
            connection_count: 0,
        };
        
        let mut services = self.services.write().unwrap();
        services.insert(config.name.clone(), instance);
        
        // Start health checking if enabled
        if config.health_check_enabled {
            self.health_checker.start_health_check(&config.name, config.health_check_interval).await?;
        }
        
        Ok(())
    }
    
    /// Unregister a service
    pub async fn unregister_service(&self, name: &str) -> Result<(), ServiceError> {
        let mut services = self.services.write().unwrap();
        if let Some(_instance) = services.remove(name) {
            // Stop health checking
            self.health_checker.stop_health_check(name).await;
            
            // Cleanup module
            // Note: This is a simplified cleanup - in practice, you'd want to ensure
            // all pending requests are completed before cleanup
            Ok(())
        } else {
            Err(ServiceError::ServiceNotFound(name.to_string()))
        }
    }
    
    /// Get service by name
    pub fn get_service(&self, name: &str) -> Option<ServiceInstance> {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
    
    /// List all services
    pub fn list_services(&self) -> Vec<String> {
        let services = self.services.read().unwrap();
        services.keys().cloned().collect()
    }
    
    /// Handle ICAP request
    pub async fn handle_request(&self, request: &IcapRequest) -> Result<IcapResponse, ServiceError> {
        // Find appropriate service based on path
        let service_name = self.find_service_by_path(&request.uri.path())?;
        
        // Get service instance
        let services = self.services.read().unwrap();
        let service = services.get(&service_name)
            .ok_or_else(|| ServiceError::ServiceNotFound(service_name.clone()))?;
        
        // Check if service supports the method
        if !service.config.methods.contains(&request.method) {
            return Err(ServiceError::MethodNotSupported(request.method.to_string()));
        }
        
        // Check connection limits
        if service.connection_count >= service.config.max_connections {
            return Err(ServiceError::TooManyConnections);
        }
        
        // Handle request based on method
        let response = match request.method {
            IcapMethod::Reqmod => service.module.handle_reqmod(request).await,
            IcapMethod::Respmod => service.module.handle_respmod(request).await,
            IcapMethod::Options => service.module.handle_options(request).await,
        };
        
        // Update metrics
        self.update_service_metrics(&service_name, &response).await;
        
        response.map_err(|e| ServiceError::ModuleError(e))
    }
    
    /// Get service metrics
    pub fn get_service_metrics(&self, name: &str) -> Option<ServiceMetrics> {
        let services = self.services.read().unwrap();
        services.get(name).map(|s| s.metrics.clone())
    }
    
    /// Get all service metrics
    pub fn get_all_metrics(&self) -> HashMap<String, ServiceMetrics> {
        let services = self.services.read().unwrap();
        services.iter().map(|(name, service)| (name.clone(), service.metrics.clone())).collect()
    }
    
    /// Check if service is healthy
    pub fn is_service_healthy(&self, name: &str) -> bool {
        self.health_checker.is_healthy(name)
    }
    
    /// Find service by path
    fn find_service_by_path(&self, path: &str) -> Result<String, ServiceError> {
        let services = self.services.read().unwrap();
        for (name, service) in services.iter() {
            if service.config.path == path {
                return Ok(name.clone());
            }
        }
        Err(ServiceError::ServiceNotFound(path.to_string()))
    }
    
    /// Update service metrics
    async fn update_service_metrics(&self, service_name: &str, response: &Result<IcapResponse, ModuleError>) {
        let mut services = self.services.write().unwrap();
        if let Some(service) = services.get_mut(service_name) {
            service.metrics.requests_total += 1;
            service.metrics.last_activity = Some(Instant::now());
            
            // Update error rate
            if response.is_err() {
                service.metrics.connection_errors += 1;
            }
            
            // Calculate error rate
            if service.metrics.requests_total > 0 {
                service.metrics.error_rate = service.metrics.connection_errors as f64 / service.metrics.requests_total as f64;
            }
        }
    }
}

/// Health checker
#[derive(Clone)]
pub struct HealthChecker {
    // Health check state
    health_checks: Arc<RwLock<HashMap<String, bool>>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            health_checks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Start health checking for a service
    pub async fn start_health_check(&self, service_name: &str, _interval: Duration) -> Result<(), ServiceError> {
        // In a real implementation, this would spawn a background task
        // to periodically check service health
        let mut health_checks = self.health_checks.write().unwrap();
        health_checks.insert(service_name.to_string(), true);
        Ok(())
    }
    
    /// Stop health checking for a service
    pub async fn stop_health_check(&self, service_name: &str) {
        let mut health_checks = self.health_checks.write().unwrap();
        health_checks.remove(service_name);
    }
    
    /// Check if service is healthy
    pub fn is_healthy(&self, service_name: &str) -> bool {
        let health_checks = self.health_checks.read().unwrap();
        health_checks.get(service_name).copied().unwrap_or(false)
    }
}

/// Load balancer
#[derive(Clone)]
pub struct LoadBalancer {
    // Load balancing state
    round_robin_index: Arc<RwLock<usize>>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            round_robin_index: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Select service instance using load balancing strategy
    pub fn select_service(&self, services: &[String], strategy: &LoadBalancingStrategy) -> Option<String> {
        if services.is_empty() {
            return None;
        }
        
        match strategy {
            LoadBalancingStrategy::RoundRobin => {
                let mut index = self.round_robin_index.write().unwrap();
                let selected = services[*index % services.len()].clone();
                *index = (*index + 1) % services.len();
                Some(selected)
            }
            LoadBalancingStrategy::LeastConnections => {
                // In a real implementation, this would check actual connection counts
                services.first().cloned()
            }
            LoadBalancingStrategy::WeightedRoundRobin(_weights) => {
                // In a real implementation, this would use the weights
                services.first().cloned()
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let mut rng = rand::rng();
                let index = rng.random_range(0..services.len());
                Some(services[index].clone())
            }
        }
    }
}

/// Service errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Method not supported: {0}")]
    MethodNotSupported(String),
    #[error("Too many connections")]
    TooManyConnections,
    #[error("Service unhealthy")]
    ServiceUnhealthy,
    #[error("Module error: {0}")]
    ModuleError(ModuleError),
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("Load balancing error: {0}")]
    LoadBalancingError(String),
}

impl Clone for ServiceInstance {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone implementation
        // In practice, you'd need to handle the module trait object properly
        Self {
            id: self.id.clone(),
            config: self.config.clone(),
            module: Box::new(crate::modules::builtin::EchoModule::new()),
            metrics: self.metrics.clone(),
            last_health_check: self.last_health_check,
            connection_count: self.connection_count,
        }
    }
}
