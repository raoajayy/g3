/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use std::path::Path;

use anyhow::anyhow;
use yaml_rust::{Yaml, yaml};

use g3_yaml::{HybridParser, YamlDocPosition};

use crate::opts::ProcArgs;

pub(crate) mod icap_server;

mod registry;
pub(crate) use registry::clear;

/// Any server configuration following G3Proxy pattern
#[derive(Debug, Clone)]
pub enum AnyServerConfig {
    Icap(ProcArgs),
}

impl AnyServerConfig {
    pub fn name(&self) -> &str {
        match self {
            AnyServerConfig::Icap(_) => "g3icap",
        }
    }
}

pub(crate) fn load_all(v: &Yaml, conf_dir: &Path) -> anyhow::Result<()> {
    let parser = HybridParser::new(conf_dir, g3_daemon::opts::config_file_extension());
    parser.foreach_map(v, |map, position| {
        let server = load_server(map, position)?;
        if let Some(old_server) = registry::add(server) {
            Err(anyhow!(
                "server with name {} already exists",
                old_server.name()
            ))
        } else {
            Ok(())
        }
    })?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn load_at_position(position: &YamlDocPosition) -> anyhow::Result<AnyServerConfig> {
    let doc = g3_yaml::load_doc(position)?;
    if let Yaml::Hash(map) = doc {
        let server = load_server(&map, Some(position.clone()))?;
        let old_server = registry::add(server.clone());
        if old_server.is_some() {
            return Err(anyhow!("server with name {} already exists", server.name()));
        }
        Ok(server)
    } else {
        Err(anyhow!("yaml doc {position} is not a map"))
    }
}

fn load_server(
    map: &yaml::Hash,
    position: Option<YamlDocPosition>,
) -> anyhow::Result<AnyServerConfig> {
    let server_type = g3_yaml::key::normalize(
        g3_yaml::hash_get_required_str(map, "type")?
    );
    
    match server_type.as_str() {
        "icapserver" => {
            let mut config = icap_server::IcapServerConfig::new(position);
            // Remove the "type" key from the map before parsing
            let mut filtered_map = map.clone();
            filtered_map.remove(&Yaml::String("type".to_string()));
            config.parse(&filtered_map)?;
            Ok(AnyServerConfig::Icap(config.to_proc_args()))
        }
        _ => Err(anyhow!("unsupported server type: {server_type}")),
    }
}
