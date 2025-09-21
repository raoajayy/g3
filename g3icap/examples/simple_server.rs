//! Simple ICAP Server Example
//!
//! This example demonstrates how to create and run a basic ICAP server.

use g3icap::opts::ProcArgs;
use g3icap::server::IcapServer;
use g3icap::version::VERSION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting G3 ICAP Server Example");

    // Create configuration
    let config = ProcArgs {
        daemon_config: g3_daemon::opts::DaemonArgs::new("g3icap"),
        config: Some("examples/basic.yaml".into()),
        port: 1344,
        host: "127.0.0.1".to_string(),
        max_connections: 100,
        connection_timeout: 30,
        request_timeout: 60,
        tls: false,
        tls_cert: None,
        tls_key: None,
        stats: true,
        stats_port: 8080,
        metrics: true,
        metrics_port: 9090,
    };

    // Create and start server
    let mut server = IcapServer::new(config)?;
    
    println!("ICAP Server started on 127.0.0.1:1344");
    println!("Statistics available at http://127.0.0.1:8080/stats");
    println!("Metrics available at http://127.0.0.1:9090/metrics");
    println!("Press Ctrl+C to stop");

    // Start the server
    server.start().await?;

    Ok(())
}
