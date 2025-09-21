/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::time::Duration;

use anyhow::anyhow;
use yaml_rust::yaml;

use g3_types::metrics::NodeName;
use g3_types::net::TcpListenConfig;
use g3_yaml::YamlDocPosition;

use crate::opts::ProcArgs;

const SERVER_CONFIG_TYPE: &str = "IcapServer";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IcapServerConfig {
    name: NodeName,
    position: Option<YamlDocPosition>,
    pub(crate) listen: TcpListenConfig,
    pub(crate) listen_in_worker: bool,
    pub(crate) max_connections: u32,
    pub(crate) connection_timeout: Duration,
    pub(crate) request_timeout: Duration,
    pub(crate) enable_tls: bool,
    pub(crate) tls_cert_file: Option<String>,
    pub(crate) tls_key_file: Option<String>,
    pub(crate) enable_stats: bool,
    pub(crate) stats_port: u16,
    pub(crate) enable_metrics: bool,
    pub(crate) metrics_port: u16,
}

impl IcapServerConfig {
    pub(crate) fn new(position: Option<YamlDocPosition>) -> Self {
        IcapServerConfig {
            name: NodeName::new_static("g3icap"),
            position,
            listen: TcpListenConfig::default(),
            listen_in_worker: false,
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            enable_tls: false,
            tls_cert_file: None,
            tls_key_file: None,
            enable_stats: false,
            stats_port: 8080,
            enable_metrics: false,
            metrics_port: 9090,
        }
    }

    pub(crate) fn parse(&mut self, map: &yaml::Hash) -> anyhow::Result<()> {
        g3_yaml::foreach_kv(map, |k, v| {
            match g3_yaml::key::normalize(k).as_str() {
                "name" => {
                    let name_str = g3_yaml::value::as_string(v)?;
                    self.name = unsafe { NodeName::new_unchecked(name_str) };
                }
                "listen" => {
                    self.listen = g3_yaml::value::as_tcp_listen_config(v)?;
                }
                "listen_in_worker" => {
                    self.listen_in_worker = g3_yaml::value::as_bool(v)?;
                }
                "max_connections" => {
                    self.max_connections = g3_yaml::value::as_u32(v)?;
                }
                "connection_timeout" => {
                    self.connection_timeout = Duration::from_secs(g3_yaml::value::as_u64(v)?);
                }
                "request_timeout" => {
                    self.request_timeout = Duration::from_secs(g3_yaml::value::as_u64(v)?);
                }
                "enable_tls" => {
                    self.enable_tls = g3_yaml::value::as_bool(v)?;
                }
                "tls_cert_file" => {
                    self.tls_cert_file = Some(g3_yaml::value::as_string(v)?);
                }
                "tls_key_file" => {
                    self.tls_key_file = Some(g3_yaml::value::as_string(v)?);
                }
                "enable_stats" => {
                    self.enable_stats = g3_yaml::value::as_bool(v)?;
                }
                "stats_port" => {
                    self.stats_port = g3_yaml::value::as_u16(v)?;
                }
                "enable_metrics" => {
                    self.enable_metrics = g3_yaml::value::as_bool(v)?;
                }
                "metrics_port" => {
                    self.metrics_port = g3_yaml::value::as_u16(v)?;
                }
                _ => return Err(anyhow!("invalid key {k} in {SERVER_CONFIG_TYPE} config")),
            }
            Ok(())
        })?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &NodeName {
        &self.name
    }

    pub(crate) fn to_proc_args(&self) -> ProcArgs {
        ProcArgs {
            daemon_config: g3_daemon::opts::DaemonArgs::new("g3icap"),
            config: None,
            port: self.listen.address().port(),
            host: self.listen.address().ip().to_string(),
            max_connections: self.max_connections,
            connection_timeout: self.connection_timeout.as_secs(),
            request_timeout: self.request_timeout.as_secs(),
            tls: self.enable_tls,
            tls_cert: self.tls_cert_file.as_ref().map(|s| std::path::PathBuf::from(s)),
            tls_key: self.tls_key_file.as_ref().map(|s| std::path::PathBuf::from(s)),
            stats: self.enable_stats,
            stats_port: self.stats_port,
            metrics: self.enable_metrics,
            metrics_port: self.metrics_port,
        }
    }
}
