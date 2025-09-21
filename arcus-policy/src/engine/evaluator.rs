/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Policy evaluator for processing policy rules

use anyhow::Result;
use regex::Regex;
use tracing::{debug, info, warn};

use crate::policy::{SecurityPolicy, PolicyAction, RuleType};
use super::{PolicyRequest, PolicyDecision};

/// Policy evaluator
pub struct PolicyEvaluator {
    // Compiled regex patterns cache
    regex_cache: std::collections::HashMap<String, Regex>,
}

impl PolicyEvaluator {
    pub fn new() -> Self {
        Self {
            regex_cache: std::collections::HashMap::new(),
        }
    }

    /// Evaluate a policy against a request
    pub async fn evaluate_policy(&mut self, policy: &SecurityPolicy, request: &PolicyRequest) -> Result<PolicyDecision> {
        debug!("Evaluating policy: {} against request: {}", policy.metadata.name, request.url);

        // Check if policy is enabled
        if !policy.spec.enabled {
            return Ok(PolicyDecision::allow());
        }

        // Check URL filtering rules
        if let Some(url_filtering) = &policy.spec.url_filtering {
            let decision = self.evaluate_url_filtering(url_filtering, request).await?;
            if decision.action != PolicyAction::Allow {
                return Ok(decision);
            }
        }

        // Check content security policies
        if let Some(content_security) = &policy.spec.content_security {
            let decision = self.evaluate_content_security(content_security, request).await?;
            if decision.action != PolicyAction::Allow {
                return Ok(decision);
            }
        }

        // Check traffic control policies
        if let Some(traffic_control) = &policy.spec.traffic_control {
            let decision = self.evaluate_traffic_control(traffic_control, request).await?;
            if decision.action != PolicyAction::Allow {
                return Ok(decision);
            }
        }

        // Check HTTPS inspection policies
        if let Some(https_inspection) = &policy.spec.https_inspection {
            let decision = self.evaluate_https_inspection(https_inspection, request).await?;
            if decision.action != PolicyAction::Allow {
                return Ok(decision);
            }
        }

        Ok(PolicyDecision::allow())
    }

    /// Evaluate URL filtering rules
    async fn evaluate_url_filtering(
        &mut self,
        url_filtering: &crate::policy::UrlFilteringPolicy,
        request: &PolicyRequest,
    ) -> Result<PolicyDecision> {
        // Check blocked categories
        for category in &url_filtering.categories.block {
            if self.matches_category(&request.url, category) {
                return Ok(PolicyDecision::block(
                    format!("URL blocked by category: {}", category),
                    "url_filtering".to_string(),
                ));
            }
        }

        // Check custom rules
        for rule in &url_filtering.custom_rules {
            if self.matches_rule(&request.url, rule).await? {
                let decision = match rule.action {
                    PolicyAction::Block => PolicyDecision::block(
                        format!("URL blocked by rule: {}", rule.name),
                        rule.name.clone(),
                    ),
                    PolicyAction::Warn => PolicyDecision::warn(
                        format!("URL warning by rule: {}", rule.name),
                        rule.name.clone(),
                    ),
                    PolicyAction::Inspect => PolicyDecision::inspect(
                        format!("URL requires inspection by rule: {}", rule.name),
                        rule.name.clone(),
                    ),
                    _ => PolicyDecision::allow(),
                };

                if let Some(message) = &rule.message {
                    return Ok(decision.with_message(message.clone()));
                }
                return Ok(decision);
            }
        }

        Ok(PolicyDecision::allow())
    }

    /// Evaluate content security policies
    async fn evaluate_content_security(
        &self,
        content_security: &crate::policy::ContentSecurityPolicy,
        request: &PolicyRequest,
    ) -> Result<PolicyDecision> {
        // Check malware scanning requirements
        if let Some(malware_scanning) = &content_security.malware_scanning {
            if malware_scanning.enabled {
                return Ok(PolicyDecision::inspect(
                    "Content requires malware scanning".to_string(),
                    "malware_scanning".to_string(),
                ));
            }
        }

        // Check DLP requirements
        if let Some(dlp) = &content_security.data_loss_prevention {
            if dlp.enabled {
                return Ok(PolicyDecision::inspect(
                    "Content requires DLP scanning".to_string(),
                    "data_loss_prevention".to_string(),
                ));
            }
        }

        Ok(PolicyDecision::allow())
    }

    /// Evaluate traffic control policies
    async fn evaluate_traffic_control(
        &self,
        traffic_control: &crate::policy::TrafficControlPolicy,
        request: &PolicyRequest,
    ) -> Result<PolicyDecision> {
        // Check bandwidth limits
        if let Some(bandwidth_limits) = &traffic_control.bandwidth_limits {
            if bandwidth_limits.per_user.is_some() || bandwidth_limits.total.is_some() {
                return Ok(PolicyDecision::inspect(
                    "Request requires bandwidth control".to_string(),
                    "traffic_control".to_string(),
                ));
            }
        }

        // Check quotas
        if let Some(quotas) = &traffic_control.quotas {
            if quotas.daily_data_per_user.is_some() || quotas.monthly_data_per_user.is_some() {
                return Ok(PolicyDecision::inspect(
                    "Request requires quota control".to_string(),
                    "traffic_control".to_string(),
                ));
            }
        }

        Ok(PolicyDecision::allow())
    }

    /// Evaluate HTTPS inspection policies
    async fn evaluate_https_inspection(
        &self,
        https_inspection: &crate::policy::HttpsInspectionPolicy,
        request: &PolicyRequest,
    ) -> Result<PolicyDecision> {
        if !https_inspection.enabled {
            return Ok(PolicyDecision::allow());
        }

        // Check if domain should be bypassed
        for bypass_domain in &https_inspection.bypass_domains {
            if self.matches_domain(&request.url, bypass_domain) {
                return Ok(PolicyDecision::allow());
            }
        }

        // Check if domain should be inspected
        for inspect_domain in &https_inspection.inspect_domains {
            if self.matches_domain(&request.url, inspect_domain) {
                return Ok(PolicyDecision::inspect(
                    "HTTPS inspection required".to_string(),
                    "https_inspection".to_string(),
                ));
            }
        }

        // Default behavior based on mode
        match https_inspection.mode {
            crate::policy::HttpsMode::Mitm => Ok(PolicyDecision::inspect(
                "HTTPS MITM inspection required".to_string(),
                "https_inspection".to_string(),
            )),
            crate::policy::HttpsMode::Passthrough => Ok(PolicyDecision::allow()),
            crate::policy::HttpsMode::Selective => Ok(PolicyDecision::allow()),
        }
    }

    /// Check if URL matches a category
    fn matches_category(&self, url: &str, category: &str) -> bool {
        // This is a simplified implementation
        // In a real system, you'd have a category database or API
        match category {
            "gambling" => url.contains("gambling") || url.contains("casino") || url.contains("bet"),
            "adult-content" => url.contains("adult") || url.contains("porn"),
            "malware" => url.contains("malware") || url.contains("virus"),
            "phishing" => url.contains("phishing") || url.contains("scam"),
            "peer-to-peer" => url.contains("torrent") || url.contains("p2p"),
            _ => false,
        }
    }

    /// Check if URL matches a rule
    async fn matches_rule(&mut self, url: &str, rule: &crate::policy::CustomRule) -> Result<bool> {
        if let Some(pattern) = &rule.pattern {
            return Ok(self.matches_pattern(url, pattern, &rule.rule_type).await?);
        }

        if let Some(patterns) = &rule.patterns {
            for pattern in patterns {
                if self.matches_pattern(url, pattern, &rule.rule_type).await? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Check if URL matches a pattern
    async fn matches_pattern(&mut self, url: &str, pattern: &str, rule_type: &RuleType) -> Result<bool> {
        match rule_type {
            RuleType::Wildcard => Ok(self.matches_wildcard(url, pattern)),
            RuleType::Regex => Ok(self.matches_regex(url, pattern).await?),
            RuleType::Exact => Ok(url == pattern),
            RuleType::Domain => Ok(self.matches_domain(url, pattern)),
            RuleType::Suffix => Ok(url.ends_with(pattern)),
        }
    }

    /// Check wildcard pattern matching
    fn matches_wildcard(&self, url: &str, pattern: &str) -> bool {
        // Convert wildcard pattern to regex
        let regex_pattern = pattern
            .replace(".", "\\.")
            .replace("*", ".*")
            .replace("?", ".");
        
        match Regex::new(&format!("^{}$", regex_pattern)) {
            Ok(regex) => regex.is_match(url),
            Err(_) => false,
        }
    }

    /// Check regex pattern matching with caching
    async fn matches_regex(&mut self, url: &str, pattern: &str) -> Result<bool> {
        let regex = if let Some(cached) = self.regex_cache.get(pattern) {
            cached
        } else {
            let compiled = Regex::new(pattern)?;
            self.regex_cache.insert(pattern.to_string(), compiled);
            self.regex_cache.get(pattern).unwrap()
        };

        Ok(regex.is_match(url))
    }

    /// Check domain matching
    fn matches_domain(&self, url: &str, domain: &str) -> bool {
        // Extract domain from URL
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                return host == domain || host.ends_with(&format!(".{}", domain));
            }
        }
        false
    }
}
