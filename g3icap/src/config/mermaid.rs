/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Mermaid configuration visualization
//!
//! This module provides Mermaid-based configuration visualization
//! following g3proxy patterns.

use std::collections::HashMap;

use anyhow::Result;
use g3_types::metrics::NodeName;

use crate::config::audit::AuditorConfig;
use crate::config::auth::UserGroupConfig;
use crate::config::server::icap_server::IcapServerConfig;

/// Generate Mermaid graph for configuration visualization
pub fn mermaid_graph() -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("graph TB\n");
    graph.push_str("  subgraph \"ICAP Servers\"\n");
    graph.push_str("    server_main[\"Main ICAP Server\"]\n");
    graph.push_str("  end\n\n");
    
    graph.push_str("  subgraph \"Audit Modules\"\n");
    graph.push_str("    audit_content_filter[\"Content Filter\"]\n");
    graph.push_str("    audit_antivirus[\"Antivirus Scanner\"]\n");
    graph.push_str("  end\n\n");
    
    graph.push_str("  subgraph \"Authentication\"\n");
    graph.push_str("    auth_basic[\"Basic Auth\"]\n");
    graph.push_str("    auth_ldap[\"LDAP Auth\"]\n");
    graph.push_str("  end\n\n");
    
    // Add connections
    graph.push_str("  server_main --> audit_content_filter\n");
    graph.push_str("  server_main --> audit_antivirus\n");
    graph.push_str("  server_main --> auth_basic\n");
    graph.push_str("  server_main --> auth_ldap\n");

    Ok(graph)
}

/// Generate detailed Mermaid graph with actual configuration data
pub fn mermaid_graph_detailed(
    servers: &HashMap<NodeName, IcapServerConfig>,
    auditors: &HashMap<NodeName, AuditorConfig>,
    user_groups: &HashMap<NodeName, UserGroupConfig>,
) -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("graph TB\n");
    
    // Add servers
    if !servers.is_empty() {
        graph.push_str("  subgraph \"ICAP Servers\"\n");
        for (name, _config) in servers {
            graph.push_str(&format!(
                "    server_{}[\"{}\"]\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  end\n\n");
    }

    // Add audit modules
    if !auditors.is_empty() {
        graph.push_str("  subgraph \"Audit Modules\"\n");
        for (name, _config) in auditors {
            graph.push_str(&format!(
                "    audit_{}[\"{}\"]\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  end\n\n");
    }

    // Add user groups
    if !user_groups.is_empty() {
        graph.push_str("  subgraph \"User Groups\"\n");
        for (name, _config) in user_groups {
            graph.push_str(&format!(
                "    auth_{}[\"{}\"]\n",
                name.as_str().replace('-', "_"),
                name.as_str()
            ));
        }
        graph.push_str("  end\n\n");
    }

    // Add connections
    for (server_name, _server_config) in servers {
        for (audit_name, _audit_config) in auditors {
            graph.push_str(&format!(
                "  server_{} --> audit_{}\n",
                server_name.as_str().replace('-', "_"),
                audit_name.as_str().replace('-', "_")
            ));
        }
    }

    Ok(graph)
}

/// Generate Mermaid sequence diagram for request flow
pub fn mermaid_sequence_diagram() -> Result<String> {
    let mut diagram = String::new();
    
    diagram.push_str("sequenceDiagram\n");
    diagram.push_str("    participant Client as ICAP Client\n");
    diagram.push_str("    participant Server as ICAP Server\n");
    diagram.push_str("    participant Auth as Auth Module\n");
    diagram.push_str("    participant Filter as Content Filter\n");
    diagram.push_str("    participant AV as Antivirus\n");
    diagram.push_str("    participant Audit as Audit Logger\n\n");
    
    diagram.push_str("    Client->>Server: ICAP REQMOD Request\n");
    diagram.push_str("    Server->>Auth: Authenticate User\n");
    diagram.push_str("    Auth-->>Server: Auth Result\n");
    diagram.push_str("    Server->>Filter: Filter Content\n");
    diagram.push_str("    Filter-->>Server: Filter Result\n");
    diagram.push_str("    Server->>AV: Scan for Viruses\n");
    diagram.push_str("    AV-->>Server: Scan Result\n");
    diagram.push_str("    Server->>Audit: Log Request\n");
    diagram.push_str("    Server-->>Client: ICAP Response\n");

    Ok(diagram)
}
