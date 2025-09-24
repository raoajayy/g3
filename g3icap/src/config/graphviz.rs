/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Graphviz configuration visualization
//!
//! This module provides Graphviz-based configuration visualization
//! following g3proxy patterns.

use std::collections::HashMap;

use anyhow::Result;
use g3_types::metrics::NodeName;

use crate::config::audit::AuditorConfig;
use crate::config::auth::UserGroupConfig;
use crate::config::server::icap_server::IcapServerConfig;

/// Generate Graphviz graph for configuration visualization
pub fn graphviz_graph() -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("digraph G3ICAP {\n");
    graph.push_str("  rankdir=TB;\n");
    graph.push_str("  node [shape=box, style=filled];\n");
    graph.push_str("  edge [color=gray];\n\n");

    // Add servers
    graph.push_str("  subgraph cluster_servers {\n");
    graph.push_str("    label=\"ICAP Servers\";\n");
    graph.push_str("    style=filled;\n");
    graph.push_str("    color=lightblue;\n");
    
    // Add server nodes (placeholder - would be populated from actual config)
    graph.push_str("    server_main [label=\"Main ICAP Server\", fillcolor=lightgreen];\n");
    graph.push_str("  }\n\n");

    // Add audit modules
    graph.push_str("  subgraph cluster_audit {\n");
    graph.push_str("    label=\"Audit Modules\";\n");
    graph.push_str("    style=filled;\n");
    graph.push_str("    color=lightyellow;\n");
    
    // Add audit nodes (placeholder - would be populated from actual config)
    graph.push_str("    audit_content_filter [label=\"Content Filter\", fillcolor=orange];\n");
    graph.push_str("    audit_antivirus [label=\"Antivirus Scanner\", fillcolor=orange];\n");
    graph.push_str("  }\n\n");

    // Add authentication modules
    graph.push_str("  subgraph cluster_auth {\n");
    graph.push_str("    label=\"Authentication\";\n");
    graph.push_str("    style=filled;\n");
    graph.push_str("    color=lightcoral;\n");
    
    // Add auth nodes (placeholder - would be populated from actual config)
    graph.push_str("    auth_basic [label=\"Basic Auth\", fillcolor=pink];\n");
    graph.push_str("    auth_ldap [label=\"LDAP Auth\", fillcolor=pink];\n");
    graph.push_str("  }\n\n");

    // Add connections
    graph.push_str("  // Server connections\n");
    graph.push_str("  server_main -> audit_content_filter;\n");
    graph.push_str("  server_main -> audit_antivirus;\n");
    graph.push_str("  server_main -> auth_basic;\n");
    graph.push_str("  server_main -> auth_ldap;\n\n");

    // Add legend
    graph.push_str("  subgraph cluster_legend {\n");
    graph.push_str("    label=\"Legend\";\n");
    graph.push_str("    style=filled;\n");
    graph.push_str("    color=lightgray;\n");
    graph.push_str("    legend_server [label=\"Server\", fillcolor=lightgreen];\n");
    graph.push_str("    legend_audit [label=\"Audit Module\", fillcolor=orange];\n");
    graph.push_str("    legend_auth [label=\"Auth Module\", fillcolor=pink];\n");
    graph.push_str("  }\n");

    graph.push_str("}\n");

    Ok(graph)
}

/// Generate detailed Graphviz graph with actual configuration data
pub fn graphviz_graph_detailed(
    servers: &HashMap<NodeName, IcapServerConfig>,
    auditors: &HashMap<NodeName, AuditorConfig>,
    user_groups: &HashMap<NodeName, UserGroupConfig>,
) -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("digraph G3ICAP {\n");
    graph.push_str("  rankdir=TB;\n");
    graph.push_str("  node [shape=box, style=filled];\n");
    graph.push_str("  edge [color=gray];\n\n");

    // Add servers
    if !servers.is_empty() {
        graph.push_str("  subgraph cluster_servers {\n");
        graph.push_str("    label=\"ICAP Servers\";\n");
        graph.push_str("    style=filled;\n");
        graph.push_str("    color=lightblue;\n");
        
        for (name, _config) in servers {
            graph.push_str(&format!(
                "    server_{} [label=\"{}\", fillcolor=lightgreen];\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  }\n\n");
    }

    // Add audit modules
    if !auditors.is_empty() {
        graph.push_str("  subgraph cluster_audit {\n");
        graph.push_str("    label=\"Audit Modules\";\n");
        graph.push_str("    style=filled;\n");
        graph.push_str("    color=lightyellow;\n");
        
        for (name, _config) in auditors {
            graph.push_str(&format!(
                "    audit_{} [label=\"{}\", fillcolor=orange];\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  }\n\n");
    }

    // Add user groups
    if !user_groups.is_empty() {
        graph.push_str("  subgraph cluster_auth {\n");
        graph.push_str("    label=\"User Groups\";\n");
        graph.push_str("    style=filled;\n");
        graph.push_str("    color=lightcoral;\n");
        
        for (name, _config) in user_groups {
            graph.push_str(&format!(
                "    auth_{} [label=\"{}\", fillcolor=pink];\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  }\n\n");
    }

    // Add connections (simplified - would be more complex in reality)
    for (server_name, _server_config) in servers {
        for (audit_name, _audit_config) in auditors {
            graph.push_str(&format!(
                "  server_{} -> audit_{};\n",
                server_name.as_str().replace('-', "_"),
                audit_name.as_str().replace('-', "_")
            ));
        }
    }

    graph.push_str("}\n");

    Ok(graph)
}
