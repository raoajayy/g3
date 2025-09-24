/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! PlantUML configuration visualization
//!
//! This module provides PlantUML-based configuration visualization
//! following g3proxy patterns.

use std::collections::HashMap;

use anyhow::Result;
use g3_types::metrics::NodeName;

use crate::config::audit::AuditorConfig;
use crate::config::auth::UserGroupConfig;
use crate::config::server::icap_server::IcapServerConfig;

/// Generate PlantUML graph for configuration visualization
pub fn plantuml_graph() -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("@startuml G3ICAP\n");
    graph.push_str("!theme plain\n\n");
    
    // Add packages
    graph.push_str("package \"ICAP Servers\" {\n");
    graph.push_str("  [Main ICAP Server] as server_main\n");
    graph.push_str("}\n\n");
    
    graph.push_str("package \"Audit Modules\" {\n");
    graph.push_str("  [Content Filter] as audit_content_filter\n");
    graph.push_str("  [Antivirus Scanner] as audit_antivirus\n");
    graph.push_str("}\n\n");
    
    graph.push_str("package \"Authentication\" {\n");
    graph.push_str("  [Basic Auth] as auth_basic\n");
    graph.push_str("  [LDAP Auth] as auth_ldap\n");
    graph.push_str("}\n\n");
    
    // Add connections
    graph.push_str("server_main --> audit_content_filter\n");
    graph.push_str("server_main --> audit_antivirus\n");
    graph.push_str("server_main --> auth_basic\n");
    graph.push_str("server_main --> auth_ldap\n\n");
    
    graph.push_str("@enduml\n");

    Ok(graph)
}

/// Generate detailed PlantUML graph with actual configuration data
pub fn plantuml_graph_detailed(
    servers: &HashMap<NodeName, IcapServerConfig>,
    auditors: &HashMap<NodeName, AuditorConfig>,
    user_groups: &HashMap<NodeName, UserGroupConfig>,
) -> Result<String> {
    let mut graph = String::new();
    
    // Graph header
    graph.push_str("@startuml G3ICAP\n");
    graph.push_str("!theme plain\n\n");
    
    // Add servers
    if !servers.is_empty() {
        graph.push_str("package \"ICAP Servers\" {\n");
        for (name, _config) in servers {
            graph.push_str(&format!(
                "  [{}] as server_{}\n",
                name.as_str(),
                name.as_str().replace('-', "_")
            ));
        }
        graph.push_str("}\n\n");
    }

    // Add audit modules
    if !auditors.is_empty() {
        graph.push_str("package \"Audit Modules\" {\n");
        for (name, _config) in auditors {
            graph.push_str(&format!(
                "  [{}] as audit_{}\n",
                name.as_str(),
                name.as_str().replace('-', "_")
            ));
        }
        graph.push_str("}\n\n");
    }

    // Add user groups
    if !user_groups.is_empty() {
        graph.push_str("package \"User Groups\" {\n");
        for (name, _config) in user_groups {
            graph.push_str(&format!(
                "  [{}] as auth_{}\n",
                name.as_str(),
                name.as_str().replace('-', "_")
            ));
        }
        graph.push_str("}\n\n");
    }

    // Add connections
    for (server_name, _server_config) in servers {
        for (audit_name, _audit_config) in auditors {
            graph.push_str(&format!(
                "server_{} --> audit_{}\n",
                server_name.as_str().replace('-', "_"),
                audit_name.as_str().replace('-', "_")
            ));
        }
    }

    graph.push_str("\n@enduml\n");

    Ok(graph)
}

/// Generate PlantUML sequence diagram for request flow
pub fn plantuml_sequence_diagram() -> Result<String> {
    let mut diagram = String::new();
    
    diagram.push_str("@startuml ICAP Request Flow\n");
    diagram.push_str("!theme plain\n\n");
    
    diagram.push_str("participant \"ICAP Client\" as Client\n");
    diagram.push_str("participant \"ICAP Server\" as Server\n");
    diagram.push_str("participant \"Auth Module\" as Auth\n");
    diagram.push_str("participant \"Content Filter\" as Filter\n");
    diagram.push_str("participant \"Antivirus\" as AV\n");
    diagram.push_str("participant \"Audit Logger\" as Audit\n\n");
    
    diagram.push_str("Client -> Server: ICAP REQMOD Request\n");
    diagram.push_str("Server -> Auth: Authenticate User\n");
    diagram.push_str("Auth --> Server: Auth Result\n");
    diagram.push_str("Server -> Filter: Filter Content\n");
    diagram.push_str("Filter --> Server: Filter Result\n");
    diagram.push_str("Server -> AV: Scan for Viruses\n");
    diagram.push_str("AV --> Server: Scan Result\n");
    diagram.push_str("Server -> Audit: Log Request\n");
    diagram.push_str("Server --> Client: ICAP Response\n\n");
    
    diagram.push_str("@enduml\n");

    Ok(diagram)
}

/// Generate PlantUML component diagram
pub fn plantuml_component_diagram() -> Result<String> {
    let mut diagram = String::new();
    
    diagram.push_str("@startuml G3ICAP Components\n");
    diagram.push_str("!theme plain\n\n");
    
    diagram.push_str("package \"G3ICAP Server\" {\n");
    diagram.push_str("  [ICAP Protocol Handler] as Protocol\n");
    diagram.push_str("  [Request Router] as Router\n");
    diagram.push_str("  [Response Generator] as Response\n");
    diagram.push_str("}\n\n");
    
    diagram.push_str("package \"Content Processing\" {\n");
    diagram.push_str("  [Content Filter] as Filter\n");
    diagram.push_str("  [Antivirus Scanner] as AV\n");
    diagram.push_str("  [Content Transformer] as Transformer\n");
    diagram.push_str("}\n\n");
    
    diagram.push_str("package \"Security\" {\n");
    diagram.push_str("  [Authentication] as Auth\n");
    diagram.push_str("  [Authorization] as Authz\n");
    diagram.push_str("  [Audit Logger] as Audit\n");
    diagram.push_str("}\n\n");
    
    diagram.push_str("package \"Infrastructure\" {\n");
    diagram.push_str("  [Configuration] as Config\n");
    diagram.push_str("  [Statistics] as Stats\n");
    diagram.push_str("  [Logging] as Logging\n");
    diagram.push_str("}\n\n");
    
    // Add connections
    diagram.push_str("Protocol --> Router\n");
    diagram.push_str("Router --> Filter\n");
    diagram.push_str("Router --> AV\n");
    diagram.push_str("Router --> Transformer\n");
    diagram.push_str("Router --> Auth\n");
    diagram.push_str("Router --> Authz\n");
    diagram.push_str("Router --> Audit\n");
    diagram.push_str("Router --> Response\n");
    diagram.push_str("Config --> Protocol\n");
    diagram.push_str("Stats --> Router\n");
    diagram.push_str("Logging --> Audit\n\n");
    
    diagram.push_str("@enduml\n");

    Ok(diagram)
}
