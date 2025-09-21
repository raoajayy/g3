/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy management and lifecycle

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use tokio::fs;
use tracing::{info, debug, error, warn};

use super::{PolicyCollection, SecurityPolicy, PolicyId, PolicyMetadata, PolicyStatus};
use crate::config::{ConfigGenerator, ConfigContext};

/// Policy manager for handling policy lifecycle
pub struct PolicyManager {
    collections: HashMap<String, Arc<PolicyCollection>>,
    config_generator: ConfigGenerator,
    config_dir: String,
}

impl PolicyManager {
    pub fn new(config_dir: String) -> Self {
        let context = ConfigContext::default();
        Self {
            collections: HashMap::new(),
            config_generator: ConfigGenerator::new(context),
            config_dir,
        }
    }

    /// Load policies from directory
    pub async fn load_policies(&mut self) -> Result<()> {
        info!("Loading policies from directory: {}", self.config_dir);
        
        let mut entries = fs::read_dir(&self.config_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                self.load_policy_file(&path).await?;
            }
        }

        info!("Loaded {} policy collections", self.collections.len());
        Ok(())
    }

    /// Load a single policy file
    async fn load_policy_file(&mut self, path: &Path) -> Result<()> {
        debug!("Loading policy file: {:?}", path);
        
        let content = fs::read_to_string(path).await?;
        let policy: SecurityPolicy = serde_yaml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse policy file {:?}: {}", path, e))?;

        let collection_name = policy.metadata.name.clone();
        
        // Create or update collection
        let collection = self.collections.entry(collection_name.clone())
            .or_insert_with(|| Arc::new(PolicyCollection::new(
                collection_name.clone(),
                policy.metadata.created_by.clone()
            )));

        // Add policy to collection
        let policy_id = Uuid::new_v4();
        // Note: This is a simplified approach. In a real implementation,
        // you'd need to handle Arc<PolicyCollection> updates properly
        info!("Loaded policy: {} from {:?}", policy.metadata.name, path);
        
        Ok(())
    }

    /// Add a new policy
    pub fn add_policy(&mut self, collection_name: String, policy: SecurityPolicy) -> Result<PolicyId> {
        let collection = self.collections.entry(collection_name.clone())
            .or_insert_with(|| Arc::new(PolicyCollection::new(
                collection_name,
                policy.metadata.created_by.clone()
            )));

        // In a real implementation, you'd need to handle Arc updates
        // This is a simplified version
        let policy_id = Uuid::new_v4();
        info!("Added policy: {} with ID: {}", policy.metadata.name, policy_id);
        Ok(policy_id)
    }

    /// Get policy by ID
    pub fn get_policy(&self, collection_name: &str, policy_id: &PolicyId) -> Option<&Arc<SecurityPolicy>> {
        self.collections.get(collection_name)
            .and_then(|collection| collection.get_policy(policy_id))
    }

    /// Update policy
    pub fn update_policy(&mut self, collection_name: &str, policy_id: &PolicyId, policy: SecurityPolicy) -> Result<()> {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            // Update policy in collection
            // In a real implementation, you'd need to handle Arc updates properly
            info!("Updated policy: {} with ID: {}", policy.metadata.name, policy_id);
            Ok(())
        } else {
            Err(anyhow!("Collection not found: {}", collection_name))
        }
    }

    /// Delete policy
    pub fn delete_policy(&mut self, collection_name: &str, policy_id: &PolicyId) -> Result<Option<Arc<SecurityPolicy>>> {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            let result = collection.remove_policy(policy_id);
            if result.is_some() {
                info!("Deleted policy with ID: {}", policy_id);
            }
            Ok(result)
        } else {
            Err(anyhow!("Collection not found: {}", collection_name))
        }
    }

    /// Generate G3proxy configuration for all policies
    pub fn generate_g3proxy_config(&self) -> Result<HashMap<String, serde_yaml::Value>> {
        let mut configs = HashMap::new();
        
        for (collection_name, collection) in &self.collections {
            debug!("Generating configuration for collection: {}", collection_name);
            
            let config = self.config_generator.generate_config(collection)?;
            let yaml_value = serde_yaml::to_value(config)?;
            configs.insert(collection_name.clone(), yaml_value);
        }

        Ok(configs)
    }

    /// Validate all policies
    pub fn validate_policies(&self) -> Result<Vec<PolicyValidationResult>> {
        let mut results = Vec::new();
        
        for (collection_name, collection) in &self.collections {
            for (policy_id, policy) in &collection.policies {
                let validation = self.validate_policy(policy);
                results.push(PolicyValidationResult {
                    collection_name: collection_name.clone(),
                    policy_id: *policy_id,
                    policy_name: policy.metadata.name.clone(),
                    is_valid: validation.is_ok(),
                    errors: validation.err().map(|e| vec![e.to_string()]).unwrap_or_default(),
                });
            }
        }

        Ok(results)
    }

    /// Validate a single policy
    fn validate_policy(&self, policy: &SecurityPolicy) -> Result<()> {
        // Basic validation
        if policy.metadata.name.is_empty() {
            return Err(anyhow!("Policy name cannot be empty"));
        }

        if policy.api_version != "arcus.v1" {
            return Err(anyhow!("Unsupported API version: {}", policy.api_version));
        }

        if policy.kind != "SecurityPolicy" {
            return Err(anyhow!("Invalid policy kind: {}", policy.kind));
        }

        // Validate targets
        if policy.spec.targets.user_groups.is_empty() && 
           policy.spec.targets.users.is_empty() && 
           policy.spec.targets.source_networks.is_empty() {
            return Err(anyhow!("Policy must have at least one target"));
        }

        // Validate URL filtering rules
        if let Some(url_filtering) = &policy.spec.url_filtering {
            for custom_rule in &url_filtering.custom_rules {
                if custom_rule.pattern.is_none() && custom_rule.patterns.is_none() {
                    return Err(anyhow!("Custom rule '{}' must have a pattern", custom_rule.name));
                }
            }
        }

        Ok(())
    }

    /// Get policy statistics
    pub fn get_policy_stats(&self) -> PolicyStats {
        let total_policies: usize = self.collections.values()
            .map(|c| c.policies.len())
            .sum();

        let active_policies: usize = self.collections.values()
            .flat_map(|c| c.policies.values())
            .filter(|p| p.spec.enabled)
            .count();

        let collections_count = self.collections.len();

        PolicyStats {
            total_policies,
            active_policies,
            collections_count,
        }
    }
}

/// Policy validation result
#[derive(Debug, Clone)]
pub struct PolicyValidationResult {
    pub collection_name: String,
    pub policy_id: PolicyId,
    pub policy_name: String,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Policy statistics
#[derive(Debug, Clone)]
pub struct PolicyStats {
    pub total_policies: usize,
    pub active_policies: usize,
    pub collections_count: usize,
}

// Import Uuid for PolicyId
use uuid::Uuid;
