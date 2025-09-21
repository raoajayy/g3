//! Service management for G3 ICAP Server
//!
//! This module provides service discovery, registration, and management.

use crate::error::IcapResult;
use crate::protocol::common::{IcapMethod, IcapService};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Service registry for ICAP services
pub struct ServiceRegistry {
    /// Registered services
    services: Arc<RwLock<HashMap<String, IcapService>>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service
    pub async fn register_service(&self, name: String, service: IcapService) -> IcapResult<()> {
        let mut services = self.services.write().await;
        services.insert(name, service);
        Ok(())
    }

    /// Get a service by name
    pub async fn get_service(&self, name: &str) -> Option<IcapService> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    /// List all services
    pub async fn list_services(&self) -> Vec<IcapService> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    /// Unregister a service
    pub async fn unregister_service(&self, name: &str) -> Option<IcapService> {
        let mut services = self.services.write().await;
        services.remove(name)
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Service manager for ICAP services
pub struct ServiceManager {
    /// Service registry
    registry: ServiceRegistry,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new() -> Self {
        Self {
            registry: ServiceRegistry::new(),
        }
    }

    /// Initialize default services
    pub async fn initialize_default_services(&self) -> IcapResult<()> {
        // Register OPTIONS service
        let options_service = IcapService {
            name: "options".to_string(),
            description: "ICAP OPTIONS service".to_string(),
            methods: vec![IcapMethod::Options],
            options: HashMap::new(),
        };
        self.registry.register_service("options".to_string(), options_service).await?;

        // Register default REQMOD service
        let reqmod_service = IcapService {
            name: "reqmod".to_string(),
            description: "ICAP REQMOD service".to_string(),
            methods: vec![IcapMethod::Reqmod],
            options: HashMap::new(),
        };
        self.registry.register_service("reqmod".to_string(), reqmod_service).await?;

        // Register default RESPMOD service
        let respmod_service = IcapService {
            name: "respmod".to_string(),
            description: "ICAP RESPMOD service".to_string(),
            methods: vec![IcapMethod::Respmod],
            options: HashMap::new(),
        };
        self.registry.register_service("respmod".to_string(), respmod_service).await?;

        Ok(())
    }

    /// Get service registry
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}
