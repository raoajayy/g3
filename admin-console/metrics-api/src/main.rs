use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::Filter;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
struct MetricValue {
    value: f64,
    timestamp: u64,
}

#[derive(Clone, Debug, Serialize)]
struct Metric {
    name: String,
    r#type: String,
    tags: HashMap<String, String>,
    values: Vec<MetricValue>,
}

#[derive(Clone, Debug, Serialize)]
struct MetricsResponse {
    metrics: Vec<Metric>,
    total_count: usize,
}

// Policy structures
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PolicyMetadata {
    name: String,
    version: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    created_by: String,
    tags: Vec<String>,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PolicySpec {
    priority: String,
    enabled: bool,
    targets: PolicyTargets,
    url_filtering: Option<UrlFilteringPolicy>,
    content_security: Option<ContentSecurityPolicy>,
    traffic_control: Option<TrafficControlPolicy>,
    https_inspection: Option<HttpsInspectionPolicy>,
    audit: Option<AuditPolicy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PolicyTargets {
    user_groups: Vec<String>,
    users: Vec<String>,
    source_networks: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UrlFilteringPolicy {
    categories: CategoryFiltering,
    custom_rules: Vec<CustomRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CategoryFiltering {
    block: Vec<String>,
    warn: Vec<String>,
    allow: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CustomRule {
    name: String,
    action: String,
    pattern: Option<String>,
    patterns: Option<Vec<String>>,
    rule_type: String,
    message: Option<String>,
    priority: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ContentSecurityPolicy {
    malware_scanning: Option<MalwareScanningConfig>,
    data_loss_prevention: Option<DataLossPreventionConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MalwareScanningConfig {
    enabled: bool,
    icap_server: Option<String>,
    action: String,
    timeout: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DataLossPreventionConfig {
    enabled: bool,
    scan_uploads: bool,
    scan_downloads: bool,
    sensitive_data_patterns: Vec<SensitiveDataPattern>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SensitiveDataPattern {
    name: String,
    pattern: Option<String>,
    keywords: Option<Vec<String>>,
    action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TrafficControlPolicy {
    bandwidth_limits: Option<BandwidthLimits>,
    quotas: Option<QuotaLimits>,
    time_restrictions: Option<TimeRestrictions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BandwidthLimits {
    per_user: Option<String>,
    total: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct QuotaLimits {
    daily_data_per_user: Option<String>,
    monthly_data_per_user: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimeRestrictions {
    work_hours: Option<TimePolicy>,
    after_hours: Option<TimePolicy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimePolicy {
    days: Vec<String>,
    time_range: String,
    timezone: String,
    policies: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HttpsInspectionPolicy {
    enabled: bool,
    mode: String,
    certificate_generation: String,
    ca_certificate: Option<String>,
    ca_private_key: Option<String>,
    bypass_domains: Vec<String>,
    inspect_domains: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AuditPolicy {
    enabled: bool,
    log_level: String,
    retention: String,
    export_targets: Vec<ExportTarget>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportTarget {
    target_type: String,
    endpoint: String,
    format: Option<String>,
    authentication: Option<ExportAuth>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportAuth {
    auth_type: String,
    token: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecurityPolicy {
    api_version: String,
    kind: String,
    metadata: PolicyMetadata,
    spec: PolicySpec,
}

#[derive(Clone, Debug, Serialize)]
struct PolicyResponse {
    policies: Vec<SecurityPolicy>,
    total_count: usize,
}

// User structures
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
    groups: Vec<String>,
    status: String,
    last_login: String,
    created: String,
    role: String,
    bandwidth_limit: Option<String>,
    daily_quota: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct UserResponse {
    users: Vec<User>,
    total_count: usize,
}

type MetricsStore = Arc<Mutex<HashMap<String, Metric>>>;
type PolicyStore = Arc<Mutex<HashMap<String, SecurityPolicy>>>;
type UserStore = Arc<Mutex<HashMap<String, User>>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let metrics_store: MetricsStore = Arc::new(Mutex::new(HashMap::new()));
    let policy_store: PolicyStore = Arc::new(Mutex::new(HashMap::new()));
    let user_store: UserStore = Arc::new(Mutex::new(HashMap::new()));
    
    // Initialize with sample data
    initialize_sample_data(policy_store.clone(), user_store.clone());
    
    // Start background thread to simulate realistic metrics data
    let store_clone = metrics_store.clone();
    thread::spawn(move || {
        let mut request_count = 150.0;
        let mut connection_count = 42.0;
        let mut response_time = 125.5;
        let mut error_count = 5.0;
        let mut bytes_sent = 1024000.0;
        let mut bytes_received = 2048000.0;
        
        loop {
            thread::sleep(Duration::from_secs(2));
            
            let mut store = store_clone.lock().unwrap();
            let now = current_timestamp();
            
            // Simulate realistic request patterns
            request_count += fastrand::f64() * 10.0; // Random increase 0-10 requests
            
            // Simulate connection fluctuations
            connection_count = (connection_count + (fastrand::f64() - 0.5) * 5.0).max(10.0).min(100.0);
            
            // Simulate response time variations
            response_time = (response_time + (fastrand::f64() - 0.5) * 20.0).max(50.0).min(500.0);
            
            // Simulate occasional errors
            if fastrand::f64() < 0.1 {
                error_count += 1.0;
            }
            
            // Simulate data transfer
            bytes_sent += fastrand::f64() * 50000.0;
            bytes_received += fastrand::f64() * 100000.0;
            
            // Update counter metrics
            let mut requests_metric = Metric {
                name: "requests_total".to_string(),
                r#type: "counter".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            requests_metric.tags.insert("method".to_string(), "GET".to_string());
            requests_metric.tags.insert("status".to_string(), "200".to_string());
            requests_metric.values.push(MetricValue {
                value: request_count,
                timestamp: now,
            });
            store.insert("requests_total_get_200".to_string(), requests_metric);
            
            // Error counter
            let mut errors_metric = Metric {
                name: "errors_total".to_string(),
                r#type: "counter".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            errors_metric.tags.insert("type".to_string(), "http_error".to_string());
            errors_metric.values.push(MetricValue {
                value: error_count,
                timestamp: now,
            });
            store.insert("errors_total_http".to_string(), errors_metric);
            
            // Update gauge metrics
            let mut connections_metric = Metric {
                name: "active_connections".to_string(),
                r#type: "gauge".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            connections_metric.tags.insert("server".to_string(), "g3proxy".to_string());
            connections_metric.values.push(MetricValue {
                value: connection_count,
                timestamp: now,
            });
            store.insert("active_connections_g3proxy".to_string(), connections_metric);
            
            // Response time metric
            let mut response_time_metric = Metric {
                name: "response_time_ms".to_string(),
                r#type: "gauge".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            response_time_metric.tags.insert("endpoint".to_string(), "/api/metrics".to_string());
            response_time_metric.values.push(MetricValue {
                value: response_time,
                timestamp: now,
            });
            store.insert("response_time_ms_api_metrics".to_string(), response_time_metric);
            
            // Data transfer metrics
            let mut bytes_sent_metric = Metric {
                name: "bytes_sent_total".to_string(),
                r#type: "counter".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            bytes_sent_metric.tags.insert("direction".to_string(), "outbound".to_string());
            bytes_sent_metric.values.push(MetricValue {
                value: bytes_sent,
                timestamp: now,
            });
            store.insert("bytes_sent_total".to_string(), bytes_sent_metric);
            
            let mut bytes_received_metric = Metric {
                name: "bytes_received_total".to_string(),
                r#type: "counter".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            bytes_received_metric.tags.insert("direction".to_string(), "inbound".to_string());
            bytes_received_metric.values.push(MetricValue {
                value: bytes_received,
                timestamp: now,
            });
            store.insert("bytes_received_total".to_string(), bytes_received_metric);
            
            // CPU usage simulation
            let cpu_usage = 20.0 + fastrand::f64() * 40.0; // 20-60% CPU
            let mut cpu_metric = Metric {
                name: "cpu_usage_percent".to_string(),
                r#type: "gauge".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            cpu_metric.tags.insert("server".to_string(), "g3proxy".to_string());
            cpu_metric.values.push(MetricValue {
                value: cpu_usage,
                timestamp: now,
            });
            store.insert("cpu_usage_percent".to_string(), cpu_metric);
            
            // Memory usage simulation
            let memory_usage = 128.0 + fastrand::f64() * 64.0; // 128-192 MB
            let mut memory_metric = Metric {
                name: "memory_usage_mb".to_string(),
                r#type: "gauge".to_string(),
                tags: HashMap::new(),
                values: vec![],
            };
            memory_metric.tags.insert("server".to_string(), "g3proxy".to_string());
            memory_metric.values.push(MetricValue {
                value: memory_usage,
                timestamp: now,
            });
            store.insert("memory_usage_mb".to_string(), memory_metric);
        }
    });
    
    // CORS headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);
    
    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));
    
    // Metrics endpoints
    let metrics = warp::path("metrics")
        .and(warp::get())
        .and(with_metrics(metrics_store.clone()))
        .and_then(get_metrics);
    
    let metric_by_name = warp::path("metrics")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_metrics(metrics_store.clone()))
        .and_then(get_metric_by_name);
    
    // Policy endpoints
    let policies = warp::path("policies")
        .and(warp::get())
        .and(with_policies(policy_store.clone()))
        .and_then(get_policies);
    
    let policy_by_id = warp::path("policies")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_policies(policy_store.clone()))
        .and_then(get_policy_by_id);
    
    let create_policy = warp::path("policies")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_policies(policy_store.clone()))
        .and_then(create_policy_handler);
    
    let update_policy = warp::path("policies")
        .and(warp::path::param::<String>())
        .and(warp::put())
        .and(warp::body::json())
        .and(with_policies(policy_store.clone()))
        .and_then(update_policy_handler);
    
    let delete_policy = warp::path("policies")
        .and(warp::path::param::<String>())
        .and(warp::delete())
        .and(with_policies(policy_store.clone()))
        .and_then(delete_policy_handler);
    
    // User endpoints
    let users = warp::path("users")
        .and(warp::get())
        .and(with_users(user_store.clone()))
        .and_then(get_users);
    
    let user_by_id = warp::path("users")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_users(user_store.clone()))
        .and_then(get_user_by_id);
    
    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users(user_store.clone()))
        .and_then(create_user_handler);
    
    let update_user = warp::path("users")
        .and(warp::path::param::<String>())
        .and(warp::put())
        .and(warp::body::json())
        .and(with_users(user_store.clone()))
        .and_then(update_user_handler);
    
    let delete_user = warp::path("users")
        .and(warp::path::param::<String>())
        .and(warp::delete())
        .and(with_users(user_store.clone()))
        .and_then(delete_user_handler);
    
    let routes = health
        .or(metrics)
        .or(metric_by_name)
        .or(policies)
        .or(policy_by_id)
        .or(create_policy)
        .or(update_policy)
        .or(delete_policy)
        .or(users)
        .or(user_by_id)
        .or(create_user)
        .or(update_user)
        .or(delete_user)
        .with(cors);
    
    println!("Starting Arcus Admin API on http://localhost:3001");
    println!("Available endpoints:");
    println!("  GET /health - Health check");
    println!("  GET /metrics - Get all metrics");
    println!("  GET /metrics/{{name}} - Get specific metric");
    println!("  GET /policies - Get all policies");
    println!("  GET /policies/{{id}} - Get specific policy");
    println!("  POST /policies - Create policy");
    println!("  PUT /policies/{{id}} - Update policy");
    println!("  DELETE /policies/{{id}} - Delete policy");
    println!("  GET /users - Get all users");
    println!("  GET /users/{{id}} - Get specific user");
    println!("  POST /users - Create user");
    println!("  PUT /users/{{id}} - Update user");
    println!("  DELETE /users/{{id}} - Delete user");
    
    let port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3001);
    
    println!("Starting Arcus Admin API on http://localhost:{}", port);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

fn with_metrics(metrics: MetricsStore) -> impl Filter<Extract = (MetricsStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || metrics.clone())
}

fn with_policies(policies: PolicyStore) -> impl Filter<Extract = (PolicyStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || policies.clone())
}

fn with_users(users: UserStore) -> impl Filter<Extract = (UserStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}

async fn get_metrics(metrics: MetricsStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = metrics.lock().unwrap();
    let metrics_vec: Vec<Metric> = store.values().cloned().collect();
    
    let response = MetricsResponse {
        total_count: metrics_vec.len(),
        metrics: metrics_vec,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

async fn get_metric_by_name(name: String, metrics: MetricsStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = metrics.lock().unwrap();
    
    // Find metrics that match the name (partial match)
    let matching_metrics: Vec<Metric> = store
        .values()
        .filter(|metric| metric.name.contains(&name))
        .cloned()
        .collect();
    
    if matching_metrics.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Metric not found"})),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }
    
    let response = MetricsResponse {
        total_count: matching_metrics.len(),
        metrics: matching_metrics,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

// Policy handlers
async fn get_policies(policies: PolicyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = policies.lock().unwrap();
    let policies_vec: Vec<SecurityPolicy> = store.values().cloned().collect();
    
    let response = PolicyResponse {
        total_count: policies_vec.len(),
        policies: policies_vec,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

async fn get_policy_by_id(id: String, policies: PolicyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = policies.lock().unwrap();
    
    if let Some(policy) = store.get(&id) {
        Ok(warp::reply::with_status(
            warp::reply::json(policy),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Policy not found"})),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

async fn create_policy_handler(policy: SecurityPolicy, policies: PolicyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let id = Uuid::new_v4().to_string();
    let mut store = policies.lock().unwrap();
    store.insert(id.clone(), policy);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "created"})),
        warp::http::StatusCode::CREATED,
    ))
}

async fn update_policy_handler(id: String, policy: SecurityPolicy, policies: PolicyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = policies.lock().unwrap();
    store.insert(id.clone(), policy);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "updated"})),
        warp::http::StatusCode::OK,
    ))
}

async fn delete_policy_handler(id: String, policies: PolicyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = policies.lock().unwrap();
    store.remove(&id);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "deleted"})),
        warp::http::StatusCode::OK,
    ))
}

// User handlers
async fn get_users(users: UserStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = users.lock().unwrap();
    let users_vec: Vec<User> = store.values().cloned().collect();
    
    let response = UserResponse {
        total_count: users_vec.len(),
        users: users_vec,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

async fn get_user_by_id(id: String, users: UserStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = users.lock().unwrap();
    
    if let Some(user) = store.get(&id) {
        Ok(warp::reply::with_status(
            warp::reply::json(user),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "User not found"})),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

async fn create_user_handler(user: User, users: UserStore) -> Result<impl warp::Reply, warp::Rejection> {
    let id = Uuid::new_v4().to_string();
    let mut store = users.lock().unwrap();
    store.insert(id.clone(), user);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "created"})),
        warp::http::StatusCode::CREATED,
    ))
}

async fn update_user_handler(id: String, user: User, users: UserStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = users.lock().unwrap();
    store.insert(id.clone(), user);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "updated"})),
        warp::http::StatusCode::OK,
    ))
}

async fn delete_user_handler(id: String, users: UserStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = users.lock().unwrap();
    store.remove(&id);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"id": id, "status": "deleted"})),
        warp::http::StatusCode::OK,
    ))
}

fn initialize_sample_data(policies: PolicyStore, users: UserStore) {
    // Initialize sample policies
    let mut policy_store = policies.lock().unwrap();
    
    let policy1 = SecurityPolicy {
        api_version: "arcus.v1".to_string(),
        kind: "SecurityPolicy".to_string(),
        metadata: PolicyMetadata {
            name: "Block Malware Sites".to_string(),
            version: "1.0".to_string(),
            description: Some("Blocks access to known malware and phishing sites".to_string()),
            created_at: "2024-01-15T10:30:00Z".to_string(),
            updated_at: "2024-01-15T10:30:00Z".to_string(),
            created_by: "admin@company.com".to_string(),
            tags: vec!["security".to_string(), "malware".to_string()],
            status: "active".to_string(),
        },
        spec: PolicySpec {
            priority: "critical".to_string(),
            enabled: true,
            targets: PolicyTargets {
                user_groups: vec!["employees".to_string()],
                users: vec![],
                source_networks: vec!["10.0.0.0/8".to_string()],
            },
            url_filtering: Some(UrlFilteringPolicy {
                categories: CategoryFiltering {
                    block: vec!["malware".to_string(), "phishing".to_string()],
                    warn: vec!["social-media".to_string()],
                    allow: vec!["business-tools".to_string()],
                },
                custom_rules: vec![
                    CustomRule {
                        name: "block-crypto".to_string(),
                        action: "block".to_string(),
                        pattern: Some("*.crypto*".to_string()),
                        patterns: None,
                        rule_type: "wildcard".to_string(),
                        message: Some("Cryptocurrency sites are blocked".to_string()),
                        priority: Some(200),
                    }
                ],
            }),
            content_security: None,
            traffic_control: None,
            https_inspection: None,
            audit: None,
        },
    };
    
    policy_store.insert("policy-1".to_string(), policy1);
    
    // Initialize sample users
    let mut user_store = users.lock().unwrap();
    
    let user1 = User {
        id: "user-1".to_string(),
        name: "John Doe".to_string(),
        email: "john.doe@company.com".to_string(),
        groups: vec!["employees".to_string(), "developers".to_string()],
        status: "active".to_string(),
        last_login: "2024-01-15T09:30:00Z".to_string(),
        created: "2024-01-01T00:00:00Z".to_string(),
        role: "admin".to_string(),
        bandwidth_limit: Some("100Mbps".to_string()),
        daily_quota: Some("5GB".to_string()),
    };
    
    user_store.insert("user-1".to_string(), user1);
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}