/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2025 ByteDance and/or its affiliates.
 */

//! Configuration generator for translating policies to G3proxy config

use std::collections::HashMap;
use anyhow::{Result, anyhow};
use tracing::{info, debug, error};

use crate::policy::{SecurityPolicy, PolicyCollection, PolicyPriority};
use super::{ConfigContext, GeneratedConfig, RuntimeConfig, LogConfig, StatConfig, ResolverConfig, AuditorConfig, ServerConfig, ServerListen, TlsServerConfig, CertPair, LogOutput, StatTarget};

/// Configuration generator
pub struct ConfigGenerator {
    context: ConfigContext,
}

impl ConfigGenerator {
    pub fn new(context: ConfigContext) -> Self {
        Self { context }
    }

    /// Generate complete G3proxy configuration from policy collection
    pub fn generate_config(&self, policies: &PolicyCollection) -> Result<GeneratedConfig> {
        info!("Generating G3proxy configuration from {} policies", policies.policies.len());
        
        let mut config = GeneratedConfig {
            runtime: self.generate_runtime_config(),
            log: self.generate_log_config(),
            stat: self.generate_stat_config(),
            resolver: self.generate_resolver_config(),
            escaper: Vec::new(),
            user_group: Vec::new(),
            auditor: Vec::new(),
            server: Vec::new(),
        };

        // Generate escapers based on policies
        config.escaper = self.generate_escaper_chain(policies)?;
        
        // Generate user groups
        config.user_group = self.generate_user_groups(policies)?;
        
        // Generate auditors
        config.auditor = self.generate_auditor_config();
        
        // Generate servers
        config.server = self.generate_server_config(policies)?;

        info!("Generated configuration with {} escapers, {} user groups, {} auditors, {} servers",
              config.escaper.len(), config.user_group.len(), config.auditor.len(), config.server.len());

        Ok(config)
    }

    fn generate_runtime_config(&self) -> RuntimeConfig {
        RuntimeConfig {
            thread_number: self.context.proxy_instances * 2, // 2 threads per instance
        }
    }

    fn generate_log_config(&self) -> LogConfig {
        LogConfig {
            level: "info".to_string(),
            format: Some("json".to_string()),
            output: vec![
                LogOutput {
                    output_type: "file".to_string(),
                    path: Some("/var/log/g3proxy/g3proxy.log".to_string()),
                    level: Some("info".to_string()),
                    max_size: Some("100M".to_string()),
                    max_files: Some(10),
                    facility: None,
                    tag: None,
                },
                LogOutput {
                    output_type: "file".to_string(),
                    path: Some("/var/log/g3proxy/g3proxy_error.log".to_string()),
                    level: Some("error".to_string()),
                    max_size: Some("50M".to_string()),
                    max_files: Some(5),
                    facility: None,
                    tag: None,
                },
                LogOutput {
                    output_type: "syslog".to_string(),
                    path: None,
                    level: None,
                    max_size: None,
                    max_files: None,
                    facility: Some("local0".to_string()),
                    tag: Some("g3proxy".to_string()),
                },
            ],
        }
    }

    fn generate_stat_config(&self) -> StatConfig {
        StatConfig {
            target: StatTarget {
                udp: "127.0.0.1:8125".to_string(),
            },
        }
    }

    fn generate_resolver_config(&self) -> Vec<ResolverConfig> {
        vec![
            ResolverConfig {
                name: "default".to_string(),
                resolver_type: "c-ares".to_string(),
                server: vec!["127.0.0.1".to_string()],
            }
        ]
    }

    fn generate_auditor_config(&self) -> Vec<AuditorConfig> {
        vec![
            AuditorConfig {
                name: "default".to_string(),
                protocol_inspection: HashMap::new(),
                tls_cert_generator: HashMap::new(),
                tls_ticketer: HashMap::new(),
                tls_stream_dump: HashMap::new(),
            }
        ]
    }

    fn generate_escaper_chain(&self, policies: &PolicyCollection) -> Result<Vec<super::EscaperConfig>> {
        debug!("Generating escaper chain for {} policies", policies.policies.len());
        
        let mut escapers = Vec::new();
        
        // Main router escaper
        escapers.push(super::EscaperConfig {
            name: "main-router".to_string(),
            escaper_type: "route_upstream".to_string(),
            rules: self.generate_routing_rules(policies)?,
            default_next: "internet_access".to_string(),
            resolver: Some("default".to_string()),
            ..Default::default()
        });

        // Content inspection escaper
        if self.has_content_inspection(policies) {
            escapers.push(super::EscaperConfig {
                name: "content_inspection".to_string(),
                escaper_type: "proxy_http".to_string(),
                upstream: Some("icap://malware-scanner.company.com:1344".to_string()),
                icap_service: Some("/scan".to_string()),
                next: Some("internet_access".to_string()),
                ..Default::default()
            });
        }

        // Malware scanning escaper
        if self.has_malware_scanning(policies) {
            escapers.push(super::EscaperConfig {
                name: "malware_scan".to_string(),
                escaper_type: "proxy_http".to_string(),
                upstream: Some("icap://malware-scanner.company.com:1344".to_string()),
                icap_service: Some("/scan".to_string()),
                next: Some("internet_access".to_string()),
                ..Default::default()
            });
        }

        // Company resources escaper
        escapers.push(super::EscaperConfig {
            name: "company_resources".to_string(),
            escaper_type: "direct_http".to_string(),
            ..Default::default()
        });

        // Warning page escaper
        escapers.push(super::EscaperConfig {
            name: "warn_and_allow".to_string(),
            escaper_type: "proxy_http".to_string(),
            upstream: Some("http://warning-page.company.com/social-media-warning".to_string()),
            next: Some("internet_access".to_string()),
            ..Default::default()
        });

        // Deny access escaper
        escapers.push(super::EscaperConfig {
            name: "deny_access_security".to_string(),
            escaper_type: "deny".to_string(),
            message: Some("Access denied by company security policy".to_string()),
            ..Default::default()
        });

        // Internet access escaper
        escapers.push(super::EscaperConfig {
            name: "internet_access".to_string(),
            escaper_type: "direct_http".to_string(),
            ..Default::default()
        });

        Ok(escapers)
    }

    fn generate_routing_rules(&self, policies: &PolicyCollection) -> Result<Vec<super::RoutingRule>> {
        let mut rules = Vec::new();

        // Sort policies by priority
        let mut sorted_policies: Vec<_> = policies.policies.values().collect();
        sorted_policies.sort_by_key(|p| std::cmp::Reverse(p.spec.priority as u32));

        for policy in sorted_policies {
            if !policy.spec.enabled {
                continue;
            }

            if let Some(url_filtering) = &policy.spec.url_filtering {
                // Generate rules for blocked categories
                for category in &url_filtering.categories.block {
                    rules.push(super::RoutingRule {
                        rule_type: "regex_match".to_string(),
                        pattern: Some(format!(".*({}).*", category)),
                        next: "deny_access_security".to_string(),
                        priority: policy.spec.priority as u32,
                    });
                }

                // Generate rules for custom patterns
                for custom_rule in &url_filtering.custom_rules {
                    if let Some(pattern) = &custom_rule.pattern {
                        let next = match custom_rule.action {
                            crate::policy::PolicyAction::Block => "deny_access_security".to_string(),
                            crate::policy::PolicyAction::Warn => "warn_and_allow".to_string(),
                            crate::policy::PolicyAction::Inspect => "content_inspection".to_string(),
                            _ => "internet_access".to_string(),
                        };

                        rules.push(super::RoutingRule {
                            rule_type: match custom_rule.rule_type {
                                crate::policy::RuleType::Wildcard => "child_match".to_string(),
                                crate::policy::RuleType::Regex => "regex_match".to_string(),
                                crate::policy::RuleType::Exact => "exact_match".to_string(),
                                crate::policy::RuleType::Domain => "child_match".to_string(),
                                crate::policy::RuleType::Suffix => "suffix_match".to_string(),
                            },
                            pattern: Some(pattern.clone()),
                            next,
                            priority: custom_rule.priority.unwrap_or(policy.spec.priority as u32),
                        });
                    }
                }
            }
        }

        // Add default rule
        rules.push(super::RoutingRule {
            rule_type: "default".to_string(),
            pattern: None,
            next: "malware_scan".to_string(),
            priority: 0,
        });

        Ok(rules)
    }

    fn generate_user_groups(&self, policies: &PolicyCollection) -> Result<Vec<super::UserGroupConfig>> {
        let mut user_groups = Vec::new();

        // Extract unique user groups from policies
        let mut group_names = std::collections::HashSet::new();
        for policy in policies.policies.values() {
            for group in &policy.spec.targets.user_groups {
                group_names.insert(group.clone());
            }
        }

        // Generate user group configurations
        for group_name in group_names {
            user_groups.push(super::UserGroupConfig {
                name: group_name.clone(),
                source: super::UserGroupSource {
                    source_type: "file".to_string(),
                    path: format!("/config/users_{}.json", group_name),
                },
                description: Some(format!("User group: {}", group_name)),
            });
        }

        // Add default user group if none specified
        if user_groups.is_empty() {
            user_groups.push(super::UserGroupConfig {
                name: "default".to_string(),
                source: super::UserGroupSource {
                    source_type: "file".to_string(),
                    path: "/config/users.json".to_string(),
                },
                description: Some("Default user group".to_string()),
            });
        }

        Ok(user_groups)
    }

    fn generate_server_config(&self, policies: &PolicyCollection) -> Result<Vec<ServerConfig>> {
        let mut servers = Vec::new();

        // HTTP proxy server
        servers.push(ServerConfig {
            name: "http".to_string(),
            server_type: "http_proxy".to_string(),
            escaper: "main-router".to_string(),
            auditor: Some("default".to_string()),
            user_group: Some("default".to_string()),
            listen: ServerListen {
                address: "0.0.0.0:8080".to_string(),
            },
            tls_client: Some(HashMap::new()),
            tls_server: None,
        });

        // SOCKS proxy server
        servers.push(ServerConfig {
            name: "socks".to_string(),
            server_type: "socks_proxy".to_string(),
            escaper: "main-router".to_string(),
            auditor: Some("default".to_string()),
            user_group: Some("default".to_string()),
            listen: ServerListen {
                address: "0.0.0.0:1080".to_string(),
            },
            tls_client: None,
            tls_server: None,
        });

        // HTTPS proxy server if HTTPS inspection is enabled
        if self.has_https_inspection(policies) {
            servers.push(ServerConfig {
                name: "https".to_string(),
                server_type: "http_proxy".to_string(),
                escaper: "main-router".to_string(),
                auditor: Some("default".to_string()),
                user_group: Some("default".to_string()),
                listen: ServerListen {
                    address: "0.0.0.0:8443".to_string(),
                },
                tls_client: Some(HashMap::new()),
                tls_server: Some(TlsServerConfig {
                    cert_pairs: CertPair {
                        certificate: "/etc/ssl/proxy-cert.pem".to_string(),
                        private_key: "/etc/ssl/proxy-key.pem".to_string(),
                    },
                }),
            });
        }

        Ok(servers)
    }

    fn has_content_inspection(&self, policies: &PolicyCollection) -> bool {
        policies.policies.values().any(|p| {
            p.spec.content_security.as_ref()
                .map(|cs| cs.data_loss_prevention.as_ref().map(|dlp| dlp.enabled).unwrap_or(false))
                .unwrap_or(false)
        })
    }

    fn has_malware_scanning(&self, policies: &PolicyCollection) -> bool {
        policies.policies.values().any(|p| {
            p.spec.content_security.as_ref()
                .map(|cs| cs.malware_scanning.as_ref().map(|ms| ms.enabled).unwrap_or(false))
                .unwrap_or(false)
        })
    }

    fn has_https_inspection(&self, policies: &PolicyCollection) -> bool {
        policies.policies.values().any(|p| {
            p.spec.https_inspection.as_ref()
                .map(|hi| hi.enabled)
                .unwrap_or(false)
        })
    }
}
