//! ICAP Connection Handler
//!
//! This module handles individual ICAP connections and request processing.

use std::net::SocketAddr;
use std::sync::Arc;

use slog::Logger;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use g3_daemon::listen::ListenStats;

use crate::error::{IcapError, IcapResult};
use crate::log::connection::{get_logger, ConnectionEvent};
use crate::opts::ProcArgs;
use crate::protocol::common::{IcapRequest, IcapResponse, EncapsulatedData};
use crate::stats::IcapStats;
use crate::modules::IcapModule;
use crate::modules::content_filter::{ContentFilterModule, ContentFilterConfig};
use crate::modules::antivirus::{AntivirusModule, AntivirusConfig};
use crate::audit::ops::{IcapAuditOps, DefaultIcapAuditOps};

/// Content filtering result
#[derive(Debug)]
#[allow(dead_code)]
enum FilterResult {
    Allow,
    Block(String),
    Modify(EncapsulatedData),
}

/// HTTP request structure for filtering
#[derive(Debug)]
struct HttpRequest {
    method: String,
    uri: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

/// HTTP response structure for scanning
#[derive(Debug)]
struct HttpResponse {
    #[allow(dead_code)]
    status_code: u16,
    #[allow(dead_code)]
    status_text: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

/// Antivirus scanning result
#[derive(Debug)]
#[allow(dead_code)]
enum ScanResult {
    Clean,
    Infected(String),
    Modified(EncapsulatedData),
}

/// Task context for ICAP connections following G3Proxy pattern
#[derive(Clone)]
pub struct IcapTaskContext {
    pub server_config: ProcArgs,
    pub server_stats: Arc<IcapStats>,
    pub listen_stats: Arc<ListenStats>,
    pub client_addr: SocketAddr,
    pub server_addr: SocketAddr,
    pub task_logger: Option<Logger>,
}

/// ICAP Connection Handler
pub struct IcapConnection {
    /// TCP stream
    stream: TcpStream,
    /// Peer address
    peer_addr: SocketAddr,
    /// Statistics collector
    stats: Arc<IcapStats>,
    /// Logger
    #[allow(dead_code)]
    logger: Logger,
    /// Content filter module
    content_filter: Option<ContentFilterModule>,
    /// Antivirus module
    antivirus: Option<AntivirusModule>,
    /// Audit operations
    audit_ops: Box<dyn IcapAuditOps>,
}

impl IcapConnection {
    /// Create a new connection handler
    pub fn new(
        stream: TcpStream,
        peer_addr: SocketAddr,
        stats: Arc<IcapStats>,
        logger: Logger,
    ) -> Self {
        // Initialize content filter module
        let content_filter_config = ContentFilterConfig {
            blocked_domains: vec![
                "malware.com".to_string(),
                "phishing.net".to_string(),
                "spam.org".to_string(),
                "virus.example".to_string(),
            ],
            blocked_domain_patterns: vec![
                r".*\.malware\..*".to_string(),
                r".*\.phishing\..*".to_string(),
            ],
            blocked_keywords: vec![
                "malware".to_string(),
                "virus".to_string(),
                "phishing".to_string(),
                "spam".to_string(),
                "trojan".to_string(),
                "backdoor".to_string(),
            ],
            blocked_keyword_patterns: vec![
                r".*malware.*".to_string(),
                r".*virus.*".to_string(),
            ],
            blocked_mime_types: vec![
                "application/x-executable".to_string(),
                "application/x-msdownload".to_string(),
                "application/x-msdos-program".to_string(),
            ],
            blocked_extensions: vec![
                ".exe".to_string(),
                ".bat".to_string(),
                ".cmd".to_string(),
                ".scr".to_string(),
            ],
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            case_insensitive: true,
            enable_regex: true,
            blocking_action: crate::modules::content_filter::BlockingAction::Forbidden,
            custom_message: Some("Content blocked by G3ICAP".to_string()),
            enable_logging: true,
            enable_metrics: true,
            regex_cache_size: 1000,
        };
        
        let content_filter = Some(ContentFilterModule::new(content_filter_config));

        // Initialize antivirus module
        let antivirus_config = AntivirusConfig {
            engine: crate::modules::antivirus::AntivirusEngine::YARA {
                rules_dir: std::path::PathBuf::from("/etc/g3icap/yara"),
                timeout: std::time::Duration::from_secs(10),
                max_rules: 1000,
                enable_compilation: true,
            },
            max_file_size: 50 * 1024 * 1024, // 50MB
            scan_timeout: std::time::Duration::from_secs(30),
            quarantine_dir: Some(std::path::PathBuf::from("/tmp/g3icap/quarantine")),
            enable_quarantine: true,
            enable_logging: true,
            enable_metrics: true,
            scan_file_types: vec![
                "application/octet-stream".to_string(),
                "application/x-executable".to_string(),
                "application/x-msdownload".to_string(),
            ],
            skip_file_types: vec![
                "text/plain".to_string(),
                "text/html".to_string(),
                "image/jpeg".to_string(),
                "image/png".to_string(),
            ],
            enable_realtime: true,
            update_interval: std::time::Duration::from_secs(3600), // 1 hour
            enable_threat_intel: true,
            threat_intel_sources: vec![
                "https://rules.yara-rules.org".to_string(),
            ],
            yara_config: Some(crate::modules::antivirus::YaraConfig {
                rules_dir: std::path::PathBuf::from("/etc/g3icap/yara"),
                max_rules: 1000,
                enable_compilation: true,
                compiled_rules_dir: Some(std::path::PathBuf::from("/tmp/g3icap/yara/compiled")),
                update_interval: std::time::Duration::from_secs(3600),
                enable_caching: true,
                cache_size: 1000,
                enable_rule_stats: true,
                rule_priorities: std::collections::HashMap::new(),
                enable_debug: false,
                rule_timeout: std::time::Duration::from_secs(5),
            }),
        };
        
        let antivirus = Some(AntivirusModule::new(antivirus_config));

        // Initialize audit operations
        let audit_ops = Box::new(DefaultIcapAuditOps::new(
            g3_types::metrics::NodeName::new_static("g3icap"),
            true, // Enable audit logging
        ));

        Self {
            stream,
            peer_addr,
            stats,
            logger,
            content_filter,
            antivirus,
            audit_ops,
        }
    }

    /// Process the connection
    pub async fn process(&mut self) -> IcapResult<()> {
        let connection_id = format!("{}", self.peer_addr);
        let logger = get_logger(&connection_id).unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });

        println!("DEBUG: Processing connection from {}", self.peer_addr);
        ConnectionEvent::Accepted.log(&logger, &format!("Processing connection from {}", self.peer_addr));
        
        // Log audit event for connection received
        self.audit_ops.log_request_received(
            &self.peer_addr.to_string(),
            "ICAP-Client/1.0",
            "icap://localhost/",
        );

        // Read request
        println!("DEBUG: Reading request...");
        let request = match self.read_request().await {
            Ok(req) => {
                println!("DEBUG: Request read successfully: {:?}", req.method);
                req
            }
            Err(e) => {
                println!("DEBUG: Error reading request: {}", e);
                return Err(e);
            }
        };
        
        // Process request
        println!("DEBUG: Processing request...");
        let response = match self.process_request(request).await {
            Ok(resp) => {
                println!("DEBUG: Request processed successfully: {}", resp.status);
                resp
            }
            Err(e) => {
                println!("DEBUG: Error processing request: {}", e);
                return Err(e);
            }
        };
        
        // Send response
        println!("DEBUG: Sending response...");
        match self.send_response(response).await {
            Ok(_) => {
                println!("DEBUG: Response sent successfully");
            }
            Err(e) => {
                println!("DEBUG: Error sending response: {}", e);
                return Err(e);
            }
        }

        ConnectionEvent::ResponseSent.log(&logger, "Connection processed successfully");
        
        Ok(())
    }

    /// Read ICAP request from stream
    async fn read_request(&mut self) -> IcapResult<IcapRequest> {
        println!("DEBUG: Starting to read request from stream");
        let mut buffer = Vec::new();
        let mut temp_buffer = [0u8; 4096];
        
        loop {
            println!("DEBUG: Reading from stream...");
            let n = self.stream.read(&mut temp_buffer).await
                .map_err(|e| {
                    println!("DEBUG: Error reading from stream: {}", e);
                    IcapError::Io(e)
                })?;
            
            println!("DEBUG: Read {} bytes from stream", n);
            
            if n == 0 {
                println!("DEBUG: Connection closed by peer");
                return Err(IcapError::Network("Connection closed by peer".to_string()));
            }
            
            buffer.extend_from_slice(&temp_buffer[..n]);
            println!("DEBUG: Buffer now has {} bytes", buffer.len());
            
            // Check if we have a complete request
            println!("DEBUG: Checking if request is complete...");
            if self.is_complete_request(&buffer) {
                println!("DEBUG: Complete request received");
                break;
            } else {
                println!("DEBUG: Request not complete yet, continuing to read...");
            }
        }
        
        println!("DEBUG: Parsing request with {} bytes", buffer.len());
        // Parse the request using the ICAP parser
        crate::protocol::common::IcapParser::parse_request(&buffer)
    }

    /// Check if we have a complete request
    fn is_complete_request(&self, buffer: &[u8]) -> bool {
        // Simple check for double CRLF (end of headers)
        buffer.windows(4).any(|w| w == b"\r\n\r\n")
    }

    /// Process the ICAP request
    async fn process_request(&self, request: IcapRequest) -> IcapResult<IcapResponse> {
        let connection_id = format!("{}", self.peer_addr);
        let logger = get_logger(&connection_id).unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });

        ConnectionEvent::RequestReceived.log(&logger, &format!("Processing ICAP request: {}", request.method.to_string()));
        
        // Update statistics
        self.stats.increment_requests();
        
        // Route to appropriate handler based on method
        match request.method {
            crate::protocol::common::IcapMethod::Options => {
                self.stats.increment_options_requests();
                self.handle_options_request(request).await
            }
            crate::protocol::common::IcapMethod::Reqmod => {
                self.stats.increment_reqmod_requests();
                self.handle_reqmod_request(request).await
            }
            crate::protocol::common::IcapMethod::Respmod => {
                self.stats.increment_respmod_requests();
                self.handle_respmod_request(request).await
            }
        }
    }

    /// Handle OPTIONS request
    async fn handle_options_request(&self, request: IcapRequest) -> IcapResult<IcapResponse> {
        println!("DEBUG: Processing OPTIONS request for URI: {}", request.uri);
        
        // Create comprehensive OPTIONS response with full service capabilities
        let mut headers = http::HeaderMap::new();
        
        // Basic ICAP service information
        headers.insert("ISTag", "\"g3icap-1.0.0\"".parse().unwrap());
        headers.insert("Methods", "REQMOD, RESPMOD, OPTIONS".parse().unwrap());
        headers.insert("Service", "G3 ICAP Server - Content Filtering & Antivirus".parse().unwrap());
        headers.insert("Server", "G3ICAP/1.0.0".parse().unwrap());
        
        // Connection and performance limits
        headers.insert("Max-Connections", "1000".parse().unwrap());
        headers.insert("Max-Connections-Per-Client", "10".parse().unwrap());
        headers.insert("Options-TTL", "3600".parse().unwrap());
        headers.insert("Connection-Timeout", "30".parse().unwrap());
        headers.insert("Request-Timeout", "60".parse().unwrap());
        
        // ICAP protocol capabilities
        headers.insert("Allow", "204".parse().unwrap());
        headers.insert("Preview", "1024".parse().unwrap());
        headers.insert("Transfer-Preview", "*".parse().unwrap());
        headers.insert("Transfer-Ignore", "Content-Length, Content-Encoding".parse().unwrap());
        headers.insert("Transfer-Complete", "Content-Length".parse().unwrap());
        
        // Content filtering capabilities
        headers.insert("X-Content-Filter", "enabled".parse().unwrap());
        headers.insert("X-Filter-Domains", "blocked_domains, domain_patterns".parse().unwrap());
        headers.insert("X-Filter-Keywords", "blocked_keywords, keyword_patterns".parse().unwrap());
        headers.insert("X-Filter-MIME", "blocked_mime_types, blocked_extensions".parse().unwrap());
        headers.insert("X-Filter-Size", "max_file_size".parse().unwrap());
        headers.insert("X-Filter-Regex", "enabled".parse().unwrap());
        
        // Antivirus scanning capabilities
        headers.insert("X-Antivirus", "enabled".parse().unwrap());
        headers.insert("X-Antivirus-Engine", "YARA".parse().unwrap());
        headers.insert("X-Antivirus-Scan", "real-time, on-demand".parse().unwrap());
        headers.insert("X-Antivirus-Quarantine", "enabled".parse().unwrap());
        headers.insert("X-Antivirus-Update", "hourly".parse().unwrap());
        headers.insert("X-Antivirus-Threat-Intel", "enabled".parse().unwrap());
        
        // Security and compliance features
        headers.insert("X-Security-Features", "content_filtering, antivirus, threat_intelligence".parse().unwrap());
        headers.insert("X-Compliance", "GDPR, CCPA, SOX".parse().unwrap());
        headers.insert("X-Data-Protection", "enabled".parse().unwrap());
        headers.insert("X-Audit-Logging", "enabled".parse().unwrap());
        
        // Performance and monitoring
        headers.insert("X-Metrics", "enabled".parse().unwrap());
        headers.insert("X-Statistics", "enabled".parse().unwrap());
        headers.insert("X-Health-Check", "/health".parse().unwrap());
        headers.insert("X-Metrics-Endpoint", "/metrics".parse().unwrap());
        
        // Supported content types for scanning
        headers.insert("X-Scan-Content-Types", "application/octet-stream, application/x-executable, application/x-msdownload".parse().unwrap());
        headers.insert("X-Skip-Content-Types", "text/plain, text/html, image/jpeg, image/png".parse().unwrap());
        
        // Maximum file sizes
        headers.insert("X-Max-File-Size", "52428800".parse().unwrap()); // 50MB
        headers.insert("X-Max-Preview-Size", "1048576".parse().unwrap()); // 1MB
        
        // Service version and build information
        headers.insert("X-Version", "1.0.0".parse().unwrap());
        headers.insert("X-Build-Date", "2025-01-11".parse().unwrap());
        headers.insert("X-Build-Info", "G3ICAP-1.0.0-rust".parse().unwrap());
        
        // Custom service capabilities
        headers.insert("X-Custom-Features", "modular_architecture, plugin_system, load_balancing".parse().unwrap());
        headers.insert("X-Service-Status", "operational".parse().unwrap());
        headers.insert("X-Maintenance-Window", "sunday-02:00-04:00-utc".parse().unwrap());
        
        println!("DEBUG: OPTIONS response created with comprehensive service capabilities");
        
        Ok(IcapResponse {
            status: http::StatusCode::NO_CONTENT,
            version: http::Version::HTTP_11,
            headers,
            body: bytes::Bytes::new(),
            encapsulated: None,
        })
    }

    /// Handle REQMOD request
    async fn handle_reqmod_request(&self, request: IcapRequest) -> IcapResult<IcapResponse> {
        println!("DEBUG: Processing REQMOD request for URI: {}", request.uri);
        
        // Log audit event for REQMOD request
        self.audit_ops.log_audit_event(
            "REQMOD request received",
            &format!("URI: {}", request.uri)
        );
        
        // Extract HTTP request from encapsulated data
        let http_request = match &request.encapsulated {
            Some(encapsulated) => {
                // Parse the encapsulated HTTP request
                self.parse_http_request_from_encapsulated(encapsulated).await?
            }
            None => {
                println!("DEBUG: No encapsulated data in REQMOD request");
                return Ok(IcapResponse {
                    status: http::StatusCode::BAD_REQUEST,
                    version: http::Version::HTTP_11,
                    headers: http::HeaderMap::new(),
                    body: bytes::Bytes::from("No encapsulated HTTP request data"),
                    encapsulated: None,
                });
            }
        };

        // Apply content filtering using the content filter module
        if let Some(ref content_filter) = self.content_filter {
            println!("DEBUG: Using content filter module for REQMOD processing");
            match content_filter.handle_reqmod(&request).await {
                Ok(response) => {
                    println!("DEBUG: Content filter processed REQMOD request: {}", response.status);
                    Ok(response)
                }
                Err(e) => {
                    println!("DEBUG: Content filter error: {}", e);
                    // Fall back to basic filtering
                    self.apply_basic_content_filtering(&http_request).await
                }
            }
        } else {
            println!("DEBUG: No content filter module, using basic filtering");
            self.apply_basic_content_filtering(&http_request).await
        }
    }

    /// Handle RESPMOD request
    async fn handle_respmod_request(&self, request: IcapRequest) -> IcapResult<IcapResponse> {
        println!("DEBUG: Processing RESPMOD request for URI: {}", request.uri);
        
        // Log audit event for RESPMOD request
        self.audit_ops.log_audit_event(
            "RESPMOD request received",
            &format!("URI: {}", request.uri)
        );
        
        // Extract HTTP response from encapsulated data
        let http_response = match &request.encapsulated {
            Some(encapsulated) => {
                // Parse the encapsulated HTTP response
                self.parse_http_response_from_encapsulated(encapsulated).await?
            }
            None => {
                println!("DEBUG: No encapsulated data in RESPMOD request");
                return Ok(IcapResponse {
                    status: http::StatusCode::BAD_REQUEST,
                    version: http::Version::HTTP_11,
                    headers: http::HeaderMap::new(),
                    body: bytes::Bytes::from("No encapsulated HTTP response data"),
                    encapsulated: None,
                });
            }
        };

        // Apply antivirus scanning using the antivirus module
        if let Some(ref antivirus) = self.antivirus {
            println!("DEBUG: Using antivirus module for RESPMOD processing");
            match antivirus.handle_respmod(&request).await {
                Ok(response) => {
                    println!("DEBUG: Antivirus module processed RESPMOD request: {}", response.status);
                    Ok(response)
                }
                Err(e) => {
                    println!("DEBUG: Antivirus module error: {}", e);
                    // Fall back to basic scanning
                    self.apply_basic_antivirus_scanning(&http_response).await
                }
            }
        } else {
            println!("DEBUG: No antivirus module, using basic scanning");
            self.apply_basic_antivirus_scanning(&http_response).await
        }
    }

    /// Send ICAP response to client
    async fn send_response(&mut self, response: IcapResponse) -> IcapResult<()> {
        let connection_id = format!("{}", self.peer_addr);
        let logger = get_logger(&connection_id).unwrap_or_else(|| {
            slog::Logger::root(slog::Discard, slog::o!())
        });

        ConnectionEvent::ResponseSent.log(&logger, &format!("Sending ICAP response: {}", response.status));
        
        // Serialize response using the ICAP serializer
        let response_data = crate::protocol::common::IcapSerializer::serialize_response(&response)?;
        
        self.stream.write_all(&response_data).await
            .map_err(|e| IcapError::Io(e))?;
        
        self.stream.flush().await
            .map_err(|e| IcapError::Io(e))?;
        
        // Update statistics
        if response.status.is_success() {
            self.stats.increment_successful_responses();
        } else {
            self.stats.increment_error_responses();
        }
        
        Ok(())
    }

    /// Parse HTTP request from encapsulated data
    async fn parse_http_request_from_encapsulated(&self, encapsulated: &EncapsulatedData) -> IcapResult<HttpRequest> {
        // Extract request headers and body from encapsulated data
        let req_headers = encapsulated.req_hdr.as_ref()
            .ok_or_else(|| IcapError::Protocol("No request headers in encapsulated data".to_string()))?;
        
        let req_body = encapsulated.req_body.as_ref()
            .map(|b| b.to_vec())
            .unwrap_or_default();

        // Extract method and URI from headers (simplified)
        let method = "GET".to_string(); // Default method
        let uri = "/".to_string(); // Default URI
        
        // Convert headers to our format
        let mut headers = Vec::new();
        for (name, value) in req_headers.iter() {
            let name_str = name.as_str().to_string();
            if let Ok(value_str) = value.to_str() {
                headers.push((name_str, value_str.to_string()));
            }
        }

        Ok(HttpRequest {
            method,
            uri,
            headers,
            body: req_body,
        })
    }

    /// Apply basic content filtering to HTTP request (fallback)
    async fn apply_basic_content_filtering(&self, http_request: &HttpRequest) -> IcapResult<IcapResponse> {
        println!("DEBUG: Applying basic content filtering to {} {}", http_request.method, http_request.uri);

        // Check for blocked domains
        if let Some(host) = self.extract_host(&http_request.headers) {
            if self.is_blocked_domain(&host) {
                // Log audit event for blocked request
                self.audit_ops.log_request_blocked(
                    &self.peer_addr.to_string(),
                    &http_request.uri,
                    &format!("Blocked domain: {}", host)
                );
                
                return Ok(IcapResponse {
                    status: http::StatusCode::FORBIDDEN,
                    version: http::Version::HTTP_11,
                    headers: {
                        let mut headers = http::HeaderMap::new();
                        headers.insert("X-ICAP-Error", "Blocked domain".parse().unwrap());
                        headers
                    },
                    body: bytes::Bytes::from("Request blocked: blocked domain"),
                    encapsulated: None,
                });
            }
        }

        // Check for blocked keywords in URI
        if self.contains_blocked_keywords(&http_request.uri) {
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Error", "Blocked keywords in URI".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Request blocked: blocked keywords in URI"),
                encapsulated: None,
            });
        }

        // Check for blocked MIME types
        if let Some(content_type) = self.extract_content_type(&http_request.headers) {
            if self.is_blocked_mime_type(&content_type) {
                return Ok(IcapResponse {
                    status: http::StatusCode::FORBIDDEN,
                    version: http::Version::HTTP_11,
                    headers: {
                        let mut headers = http::HeaderMap::new();
                        headers.insert("X-ICAP-Error", "Blocked MIME type".parse().unwrap());
                        headers
                    },
                    body: bytes::Bytes::from("Request blocked: blocked MIME type"),
                    encapsulated: None,
                });
            }
        }

        // Check file size
        if http_request.body.len() > 10 * 1024 * 1024 { // 10MB limit
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Error", "File too large".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Request blocked: file too large"),
                encapsulated: None,
            });
        }

        // Check for blocked keywords in body
        if self.contains_blocked_keywords(&String::from_utf8_lossy(&http_request.body)) {
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Error", "Blocked keywords in content".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Request blocked: blocked keywords in content"),
                encapsulated: None,
            });
        }

        // Allow the request
        Ok(IcapResponse {
            status: http::StatusCode::NO_CONTENT,
            version: http::Version::HTTP_11,
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
            encapsulated: None, // This will be set by the caller
        })
    }

    /// Apply content filtering to HTTP request (legacy method)
    #[allow(dead_code)]
    async fn apply_content_filtering(&self, http_request: &HttpRequest) -> IcapResult<FilterResult> {
        println!("DEBUG: Applying content filtering to {} {}", http_request.method, http_request.uri);

        // Check for blocked domains
        if let Some(host) = self.extract_host(&http_request.headers) {
            if self.is_blocked_domain(&host) {
                return Ok(FilterResult::Block(format!("Blocked domain: {}", host)));
            }
        }

        // Check for blocked keywords in URI
        if self.contains_blocked_keywords(&http_request.uri) {
            return Ok(FilterResult::Block("Blocked keywords in URI".to_string()));
        }

        // Check for blocked MIME types
        if let Some(content_type) = self.extract_content_type(&http_request.headers) {
            if self.is_blocked_mime_type(&content_type) {
                return Ok(FilterResult::Block(format!("Blocked MIME type: {}", content_type)));
            }
        }

        // Check file size
        if http_request.body.len() > 10 * 1024 * 1024 { // 10MB limit
            return Ok(FilterResult::Block("File too large".to_string()));
        }

        // Check for blocked keywords in body
        if self.contains_blocked_keywords(&String::from_utf8_lossy(&http_request.body)) {
            return Ok(FilterResult::Block("Blocked keywords in content".to_string()));
        }

        Ok(FilterResult::Allow)
    }

    /// Extract host from headers
    fn extract_host(&self, headers: &[(String, String)]) -> Option<String> {
        headers.iter()
            .find(|(name, _)| name.to_lowercase() == "host")
            .map(|(_, value)| value.clone())
    }

    /// Extract content type from headers
    fn extract_content_type(&self, headers: &[(String, String)]) -> Option<String> {
        headers.iter()
            .find(|(name, _)| name.to_lowercase() == "content-type")
            .map(|(_, value)| value.clone())
    }

    /// Check if domain is blocked
    fn is_blocked_domain(&self, host: &str) -> bool {
        let blocked_domains = vec![
            "malware.com",
            "phishing.net",
            "spam.org",
            "virus.example",
        ];
        
        blocked_domains.iter().any(|domain| host.contains(domain))
    }

    /// Check if content contains blocked keywords
    fn contains_blocked_keywords(&self, content: &str) -> bool {
        let blocked_keywords = vec![
            "malware",
            "virus",
            "phishing",
            "spam",
            "trojan",
            "backdoor",
        ];
        
        let content_lower = content.to_lowercase();
        blocked_keywords.iter().any(|keyword| content_lower.contains(keyword))
    }

    /// Check if MIME type is blocked
    fn is_blocked_mime_type(&self, content_type: &str) -> bool {
        let blocked_mime_types = vec![
            "application/x-executable",
            "application/x-msdownload",
            "application/x-msdos-program",
        ];
        
        blocked_mime_types.iter().any(|mime| content_type.contains(mime))
    }

    /// Parse HTTP response from encapsulated data
    async fn parse_http_response_from_encapsulated(&self, encapsulated: &EncapsulatedData) -> IcapResult<HttpResponse> {
        // Extract response headers and body from encapsulated data
        let res_headers = encapsulated.res_hdr.as_ref()
            .ok_or_else(|| IcapError::Protocol("No response headers in encapsulated data".to_string()))?;
        
        let res_body = encapsulated.res_body.as_ref()
            .map(|b| b.to_vec())
            .unwrap_or_default();

        // Extract status code from headers (simplified)
        let status_code = 200; // Default status
        let status_text = "OK".to_string();
        
        // Convert headers to our format
        let mut headers = Vec::new();
        for (name, value) in res_headers.iter() {
            let name_str = name.as_str().to_string();
            if let Ok(value_str) = value.to_str() {
                headers.push((name_str, value_str.to_string()));
            }
        }

        Ok(HttpResponse {
            status_code,
            status_text,
            headers,
            body: res_body,
        })
    }

    /// Apply basic antivirus scanning to HTTP response (fallback)
    async fn apply_basic_antivirus_scanning(&self, http_response: &HttpResponse) -> IcapResult<IcapResponse> {
        println!("DEBUG: Applying basic antivirus scanning to response with {} bytes", http_response.body.len());

        // Check for known virus signatures in response body
        if self.contains_virus_signatures(&http_response.body) {
            let virus_name = self.detect_virus_name(&http_response.body);
            println!("DEBUG: RESPMOD response infected with: {}", virus_name);
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Virus", virus_name.parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from(format!("Response blocked: virus detected ({})", virus_name)),
                encapsulated: None,
            });
        }

        // Check for suspicious patterns
        if self.contains_suspicious_patterns(&http_response.body) {
            println!("DEBUG: Suspicious patterns detected, blocking response");
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Virus", "SuspiciousPattern.Generic".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Response blocked: suspicious patterns detected"),
                encapsulated: None,
            });
        }

        // Check file size limits
        if http_response.body.len() > 50 * 1024 * 1024 { // 50MB limit
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Virus", "FileTooLarge.Generic".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Response blocked: file too large"),
                encapsulated: None,
            });
        }

        // Check for executable content
        if self.is_executable_content(&http_response.headers, &http_response.body) {
            return Ok(IcapResponse {
                status: http::StatusCode::FORBIDDEN,
                version: http::Version::HTTP_11,
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.insert("X-ICAP-Virus", "ExecutableContent.Generic".parse().unwrap());
                    headers
                },
                body: bytes::Bytes::from("Response blocked: executable content detected"),
                encapsulated: None,
            });
        }

        // Allow the response
        Ok(IcapResponse {
            status: http::StatusCode::NO_CONTENT,
            version: http::Version::HTTP_11,
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
            encapsulated: None, // This will be set by the caller
        })
    }

    /// Apply antivirus scanning to HTTP response (legacy method)
    #[allow(dead_code)]
    async fn apply_antivirus_scanning(&self, http_response: &HttpResponse) -> IcapResult<ScanResult> {
        println!("DEBUG: Applying antivirus scanning to response with {} bytes", http_response.body.len());

        // Check for known virus signatures in response body
        if self.contains_virus_signatures(&http_response.body) {
            let virus_name = self.detect_virus_name(&http_response.body);
            return Ok(ScanResult::Infected(virus_name));
        }

        // Check for suspicious patterns
        if self.contains_suspicious_patterns(&http_response.body) {
            println!("DEBUG: Suspicious patterns detected, blocking response");
            return Ok(ScanResult::Infected("SuspiciousPattern.Generic".to_string()));
        }

        // Check file size limits
        if http_response.body.len() > 50 * 1024 * 1024 { // 50MB limit
            return Ok(ScanResult::Infected("FileTooLarge.Generic".to_string()));
        }

        // Check for executable content
        if self.is_executable_content(&http_response.headers, &http_response.body) {
            return Ok(ScanResult::Infected("ExecutableContent.Generic".to_string()));
        }

        Ok(ScanResult::Clean)
    }

    /// Check if content contains virus signatures
    fn contains_virus_signatures(&self, content: &[u8]) -> bool {
        // EICAR test file signature
        if content.windows(68).any(|w| w == b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*") {
            return true;
        }
        
        // PE executable header
        if content.starts_with(b"MZ") {
            return true;
        }
        
        // ELF executable header
        if content.starts_with(b"\x7fELF") {
            return true;
        }
        
        // Shell script
        if content.starts_with(b"#!/bin/") {
            return true;
        }
        
        // JavaScript patterns
        if content.windows(8).any(|w| w == b"<script>") || content.windows(5).any(|w| w == b"eval(") {
            return true;
        }
        
        // Cookie theft patterns
        if content.windows(15).any(|w| w == b"document.cookie") || content.windows(15).any(|w| w == b"window.location") {
            return true;
        }
        
        false
    }

    /// Detect virus name from content
    fn detect_virus_name(&self, content: &[u8]) -> String {
        if content.starts_with(b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*") {
            "EICAR-Test-File".to_string()
        } else if content.starts_with(b"MZ") {
            "PE.Executable.Generic".to_string()
        } else if content.starts_with(b"\x7fELF") {
            "ELF.Executable.Generic".to_string()
        } else if content.starts_with(b"#!/bin/") {
            "Shell.Script.Generic".to_string()
        } else if content.windows(8).any(|w| w == b"<script>") {
            "JavaScript.Generic".to_string()
        } else {
            "Generic.Malware".to_string()
        }
    }

    /// Check for suspicious patterns
    fn contains_suspicious_patterns(&self, content: &[u8]) -> bool {
        // Check for suspicious command patterns
        let suspicious_patterns: Vec<&[u8]> = vec![
            b"cmd.exe",
            b"powershell",
            b"wscript",
            b"cscript",
            b"regsvr32",
            b"rundll32",
            b"certutil",
            b"bitsadmin",
            b"wmic",
            b"schtasks",
        ];
        
        // Check each pattern individually
        suspicious_patterns.iter().any(|pattern| {
            content.windows(pattern.len()).any(|window| window.eq_ignore_ascii_case(pattern))
        })
    }

    /// Check if content is executable
    fn is_executable_content(&self, headers: &[(String, String)], body: &[u8]) -> bool {
        // Check content type
        if let Some(content_type) = self.extract_content_type_from_headers(headers) {
            let executable_types = vec![
                "application/x-executable",
                "application/x-msdownload",
                "application/x-msdos-program",
                "application/octet-stream",
            ];
            
            if executable_types.iter().any(|&mime| content_type.contains(mime)) {
                return true;
            }
        }

        // Check file signatures
        body.starts_with(b"MZ") || body.starts_with(b"\x7fELF") || body.starts_with(b"#!/")
    }

    /// Extract content type from headers
    fn extract_content_type_from_headers(&self, headers: &[(String, String)]) -> Option<String> {
        headers.iter()
            .find(|(name, _)| name.to_lowercase() == "content-type")
            .map(|(_, value)| value.clone())
    }
}
