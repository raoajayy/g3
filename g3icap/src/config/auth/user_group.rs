/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

use anyhow::anyhow;
use yaml_rust::yaml;

use g3_types::metrics::NodeName;
use g3_yaml::YamlDocPosition;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UserGroupConfig {
    name: NodeName,
    position: Option<YamlDocPosition>,
    pub(crate) enabled: bool,
    pub(crate) auth_method: String,
    pub(crate) auth_source: Option<String>,
}

impl UserGroupConfig {
    pub(crate) fn new(position: Option<YamlDocPosition>) -> Self {
        UserGroupConfig {
            name: NodeName::new_static("g3icap-users"),
            position,
            enabled: false,
            auth_method: "none".to_string(),
            auth_source: None,
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
                "auth_method" => {
                    self.auth_method = g3_yaml::value::as_string(v)?;
                }
                "auth_source" => {
                    self.auth_source = Some(g3_yaml::value::as_string(v)?);
                }
                _ => return Err(anyhow!("invalid key {k} in user group config")),
            }
            Ok(())
        })?;
        Ok(())
    }

    pub(crate) fn name(&self) -> &NodeName {
        &self.name
    }
}
