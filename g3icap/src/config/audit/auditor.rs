/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use anyhow::anyhow;
use yaml_rust::yaml;

use g3_types::metrics::NodeName;
use g3_yaml::YamlDocPosition;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuditorConfig {
    name: NodeName,
    position: Option<YamlDocPosition>,
    pub(crate) enabled: bool,
    pub(crate) log_level: String,
    pub(crate) log_file: Option<String>,
}

impl AuditorConfig {
    pub(crate) fn new(position: Option<YamlDocPosition>) -> Self {
        AuditorConfig {
            name: NodeName::new_static("g3icap-auditor"),
            position,
            enabled: false,
            log_level: "info".to_string(),
            log_file: None,
        }
    }

    pub(crate) fn parse(&mut self, map: &yaml::Hash) -> anyhow::Result<()> {
        g3_yaml::foreach_kv(map, |k, v| {
            match g3_yaml::key::normalize(k).as_str() {
                "name" => {
                    let name_str = g3_yaml::value::as_string(v)?;
                    self.name = unsafe { NodeName::new_unchecked(name_str) };
                }
                "enabled" => {
                    self.enabled = g3_yaml::value::as_bool(v)?;
                }
                "log_level" => {
                    self.log_level = g3_yaml::value::as_string(v)?;
                }
                "log_file" => {
                    self.log_file = Some(g3_yaml::value::as_string(v)?);
                }
                _ => return Err(anyhow!("invalid key {k} in auditor config")),
            }
            Ok(())
        })?;
        Ok(())
    }

    pub(crate) fn name(&self) -> &NodeName {
        &self.name
    }
}
