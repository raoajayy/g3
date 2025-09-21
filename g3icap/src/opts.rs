/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::path::PathBuf;

use g3_daemon::opts::{DaemonArgs, DaemonArgsExt};
use clap::{Arg, ArgAction, Command, ValueHint, value_parser};

use crate::version::VERSION;

/// Command line arguments for G3 ICAP Server
#[derive(Debug)]
pub struct ProcArgs {
    pub daemon_config: DaemonArgs,
    
    /// Configuration file path
    pub config: Option<PathBuf>,
    
    /// Server port
    pub port: u16,
    
    /// Server host
    pub host: String,
    
    /// Maximum connections
    pub max_connections: u32,
    
    /// Connection timeout
    pub connection_timeout: u64,
    
    /// Request timeout
    pub request_timeout: u64,
    
    /// Enable TLS
    pub tls: bool,
    
    /// TLS certificate file
    pub tls_cert: Option<PathBuf>,
    
    /// TLS key file
    pub tls_key: Option<PathBuf>,
    
    /// Enable statistics
    pub stats: bool,
    
    /// Statistics port
    pub stats_port: u16,
    
    /// Enable metrics
    pub metrics: bool,
    
    /// Metrics port
    pub metrics_port: u16,
}

impl Default for ProcArgs {
    fn default() -> Self {
        Self {
            daemon_config: DaemonArgs::new("g3icap"),
            config: None,
            port: 1344,
            host: "0.0.0.0".to_string(),
            max_connections: 1000,
            connection_timeout: 30,
            request_timeout: 60,
            tls: false,
            tls_cert: None,
            tls_key: None,
            stats: false,
            stats_port: 8080,
            metrics: false,
            metrics_port: 9090,
        }
    }
}

impl ProcArgs {
    /// Parse command line arguments
    pub fn parse() -> Option<Self> {
        let matches = Command::new("g3icap")
            .version(VERSION)
            .about("G3 ICAP Server")
            .append_daemon_args()
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .help("Configuration file path")
                    .value_hint(ValueHint::FilePath)
            )
            .arg(
                Arg::new("port")
                    .short('P')
                    .long("port")
                    .value_name("PORT")
                    .help("Server port")
                    .default_value("1344")
                    .value_parser(value_parser!(u16))
            )
            .arg(
                Arg::new("host")
                    .short('H')
                    .long("host")
                    .value_name("HOST")
                    .help("Server host")
                    .default_value("0.0.0.0")
            )
            .arg(
                Arg::new("max-connections")
                    .long("max-connections")
                    .value_name("NUM")
                    .help("Maximum connections")
                    .default_value("1000")
                    .value_parser(value_parser!(u32))
            )
            .arg(
                Arg::new("connection-timeout")
                    .long("connection-timeout")
                    .value_name("SECS")
                    .help("Connection timeout")
                    .default_value("30")
                    .value_parser(value_parser!(u64))
            )
            .arg(
                Arg::new("request-timeout")
                    .long("request-timeout")
                    .value_name("SECS")
                    .help("Request timeout")
                    .default_value("60")
                    .value_parser(value_parser!(u64))
            )
            .arg(
                Arg::new("tls")
                    .long("tls")
                    .help("Enable TLS")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("tls-cert")
                    .long("tls-cert")
                    .value_name("FILE")
                    .help("TLS certificate file")
                    .value_hint(ValueHint::FilePath)
            )
            .arg(
                Arg::new("tls-key")
                    .long("tls-key")
                    .value_name("FILE")
                    .help("TLS key file")
                    .value_hint(ValueHint::FilePath)
            )
            .arg(
                Arg::new("stats")
                    .long("stats")
                    .help("Enable statistics")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("stats-port")
                    .long("stats-port")
                    .value_name("PORT")
                    .help("Statistics port")
                    .default_value("8080")
                    .value_parser(value_parser!(u16))
            )
            .arg(
                Arg::new("metrics")
                    .long("metrics")
                    .help("Enable metrics")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("metrics-port")
                    .long("metrics-port")
                    .value_name("PORT")
                    .help("Metrics port")
                    .default_value("9090")
                    .value_parser(value_parser!(u16))
            )
            .get_matches();

        let daemon_config = DaemonArgs::new("g3icap");
        
        // Set config file if provided
        if let Some(config_file) = matches.get_one::<String>("config") {
            g3_daemon::opts::validate_and_set_config_file(
                std::path::Path::new(config_file), 
                "g3icap"
            ).map_err(|e| {
                eprintln!("Failed to set config file: {}", e);
                e
            }).ok();
        }
        
        Some(Self {
            daemon_config,
            config: matches.get_one::<String>("config").map(|s| PathBuf::from(s)),
            port: *matches.get_one::<u16>("port").unwrap_or(&1344),
            host: matches.get_one::<String>("host").unwrap_or(&"0.0.0.0".to_string()).clone(),
            max_connections: *matches.get_one::<u32>("max-connections").unwrap_or(&1000),
            connection_timeout: *matches.get_one::<u64>("connection-timeout").unwrap_or(&30),
            request_timeout: *matches.get_one::<u64>("request-timeout").unwrap_or(&60),
            tls: matches.get_flag("tls"),
            tls_cert: matches.get_one::<String>("tls-cert").map(|s| PathBuf::from(s)),
            tls_key: matches.get_one::<String>("tls-key").map(|s| PathBuf::from(s)),
            stats: matches.get_flag("stats"),
            stats_port: *matches.get_one::<u16>("stats-port").unwrap_or(&8080),
            metrics: matches.get_flag("metrics"),
            metrics_port: *matches.get_one::<u16>("metrics-port").unwrap_or(&9090),
        })
    }
}

impl Clone for ProcArgs {
    fn clone(&self) -> Self {
        Self {
            daemon_config: DaemonArgs::new(self.daemon_config.process_name),
            config: self.config.clone(),
            host: self.host.clone(),
            port: self.port,
            max_connections: self.max_connections,
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            tls: self.tls,
            tls_cert: self.tls_cert.clone(),
            tls_key: self.tls_key.clone(),
            stats: self.stats,
            stats_port: self.stats_port,
            metrics: self.metrics,
            metrics_port: self.metrics_port,
        }
    }
}

/// Get daemon group name
pub fn daemon_group() -> &'static str {
    "g3icap"
}