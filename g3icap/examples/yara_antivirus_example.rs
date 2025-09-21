//! YARA Antivirus Module Example
//!
//! This example demonstrates how to use the G3ICAP antivirus module with YARA rules
//! for advanced threat detection and pattern matching.

use g3icap::modules::antivirus::{
    AntivirusConfig, AntivirusEngine, AntivirusModule, YaraConfig
};
use g3icap::modules::{ModuleConfig, IcapModule};
use g3icap::protocol::common::{IcapMethod, IcapRequest};
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("G3ICAP YARA Antivirus Module Example");
    println!("=====================================");

    // Create YARA configuration
    let yara_config = YaraConfig {
        rules_dir: PathBuf::from("examples/yara_rules"),
        compiled_rules_dir: Some(PathBuf::from("examples/yara_rules/compiled")),
        max_rules: 1000,
        enable_compilation: true,
        update_interval: Duration::from_secs(3600), // 1 hour
        enable_caching: true,
        cache_size: 1000,
        enable_rule_stats: true,
        rule_priorities: HashMap::new(),
        enable_debug: false,
        rule_timeout: Duration::from_secs(30),
    };

    // Create antivirus configuration with YARA engine
    let antivirus_config = AntivirusConfig {
        engine: AntivirusEngine::YARA {
            rules_dir: PathBuf::from("examples/yara_rules"),
            timeout: Duration::from_secs(30),
            max_rules: 1000,
            enable_compilation: true,
        },
        max_file_size: 100 * 1024 * 1024, // 100MB
        scan_timeout: Duration::from_secs(60),
        quarantine_dir: Some(PathBuf::from("/tmp/g3icap_quarantine")),
        enable_quarantine: true,
        enable_logging: true,
        enable_metrics: true,
        scan_file_types: vec![
            "exe".to_string(),
            "dll".to_string(),
            "pdf".to_string(),
            "doc".to_string(),
            "docx".to_string(),
            "xls".to_string(),
            "xlsx".to_string(),
            "ppt".to_string(),
            "pptx".to_string(),
            "zip".to_string(),
            "rar".to_string(),
            "7z".to_string(),
        ],
        skip_file_types: vec![
            "txt".to_string(),
            "log".to_string(),
            "tmp".to_string(),
        ],
        enable_realtime: true,
        update_interval: Duration::from_secs(3600),
        enable_threat_intel: true,
        threat_intel_sources: vec![
            "https://rules.yara-rules.com".to_string(),
            "https://github.com/Yara-Rules/rules".to_string(),
        ],
        yara_config: Some(yara_config),
    };

    // Create the antivirus module
    let mut antivirus_module = AntivirusModule::new(antivirus_config);

    // Initialize the module
    let module_config = ModuleConfig {
        name: "yara_antivirus".to_string(),
        path: std::path::PathBuf::new(),
        version: "1.0.0".to_string(),
        config: serde_json::Value::Object(serde_json::Map::new()),
        dependencies: Vec::new(),
        load_timeout: std::time::Duration::from_secs(30),
        max_memory: 1024 * 1024 * 100, // 100MB
        sandbox: false,
    };

    if let Err(e) = antivirus_module.init(&module_config).await {
        eprintln!("Failed to initialize antivirus module: {}", e);
        return Err(e.into());
    }

    println!("✓ Antivirus module initialized successfully");
    println!("✓ YARA rules loaded and ready for scanning");

    // Test the module with various content types
    test_antivirus_module(&antivirus_module).await?;

    Ok(())
}

async fn test_antivirus_module(module: &AntivirusModule) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTesting Antivirus Module");
    println!("========================");

    // Test 1: Clean content
    println!("\n1. Testing clean content...");
    let clean_content = b"This is a clean document with no malicious content.";
    let clean_request = create_test_request(clean_content, Some("clean.txt"));
    
    match module.handle_reqmod(&clean_request).await {
        Ok(response) => {
            if response.status == 200 {
                println!("✓ Clean content passed - no threats detected");
            } else {
                println!("✗ Clean content was blocked unexpectedly");
            }
        }
        Err(e) => {
            println!("✗ Error processing clean content: {}", e);
        }
    }

    // Test 2: Malware content
    println!("\n2. Testing malware content...");
    let malware_content = b"This file contains malware and virus code for testing purposes.";
    let malware_request = create_test_request(malware_content, Some("malware.exe"));
    
    match module.handle_reqmod(&malware_request).await {
        Ok(response) => {
            if response.status == 403 {
                println!("✓ Malware content blocked successfully");
            } else {
                println!("✗ Malware content was not blocked");
            }
        }
        Err(e) => {
            println!("✗ Error processing malware content: {}", e);
        }
    }

    // Test 3: Phishing content
    println!("\n3. Testing phishing content...");
    let phishing_content = b"Urgent: Verify your account immediately. Click here to confirm your identity.";
    let phishing_request = create_test_request(phishing_content, Some("phishing.html"));
    
    match module.handle_reqmod(&phishing_request).await {
        Ok(response) => {
            if response.status == 403 {
                println!("✓ Phishing content blocked successfully");
            } else {
                println!("✗ Phishing content was not blocked");
            }
        }
        Err(e) => {
            println!("✗ Error processing phishing content: {}", e);
        }
    }

    // Test 4: Ransomware content
    println!("\n4. Testing ransomware content...");
    let ransomware_content = b"Your files have been encrypted. Pay the ransom to decrypt your files.";
    let ransomware_request = create_test_request(ransomware_content, Some("ransomware.txt"));
    
    match module.handle_reqmod(&ransomware_request).await {
        Ok(response) => {
            if response.status == 403 {
                println!("✓ Ransomware content blocked successfully");
            } else {
                println!("✗ Ransomware content was not blocked");
            }
        }
        Err(e) => {
            println!("✗ Error processing ransomware content: {}", e);
        }
    }

    // Test 5: PowerShell script
    println!("\n5. Testing suspicious PowerShell script...");
    let powershell_content = b"powershell.exe -WindowStyle Hidden -EncodedCommand Invoke-Expression";
    let powershell_request = create_test_request(powershell_content, Some("script.ps1"));
    
    match module.handle_reqmod(&powershell_request).await {
        Ok(response) => {
            if response.status == 403 {
                println!("✓ Suspicious PowerShell script blocked successfully");
            } else {
                println!("✗ Suspicious PowerShell script was not blocked");
            }
        }
        Err(e) => {
            println!("✗ Error processing PowerShell script: {}", e);
        }
    }

    // Display module statistics
    println!("\nModule Statistics");
    println!("=================");
    let metrics = module.get_metrics();
    println!("Total requests processed: {}", metrics.requests_total);
    println!("Requests per second: {}", metrics.requests_per_second);
    println!("Average response time: {}ms", metrics.average_response_time.as_millis());
    println!("Error rate: {:.2}%", metrics.error_rate * 100.0);

    Ok(())
}

fn create_test_request(content: &[u8], filename: Option<&str>) -> IcapRequest {
    use g3icap::protocol::common::IcapRequest;
    use http::HeaderMap;
    use bytes::Bytes;

    let mut headers = HeaderMap::new();
    headers.insert("Host", "example.com".parse().unwrap());
    headers.insert("User-Agent", "G3ICAP-Test-Client/1.0".parse().unwrap());
    
    if let Some(fname) = filename {
        headers.insert("Content-Disposition", format!("attachment; filename=\"{}\"", fname).parse().unwrap());
    }

    IcapRequest {
        method: IcapMethod::Reqmod,
        uri: "/reqmod".parse().unwrap(),
        version: http::Version::HTTP_11,
        headers,
        body: Bytes::from(content.to_vec()),
        encapsulated: None,
    }
}
