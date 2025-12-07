/// E1104: Overly large struct (too many fields)
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: This struct has too many fields (more than 20-25), making it difficult to
/// understand, construct, and maintain. Large structs often indicate that the type is doing too
/// many things and should be split into smaller, focused types. It's like a class with dozens
/// of member variables - it becomes hard to keep track of what each field means and how they
/// relate to each other. Fix by grouping related fields into smaller structs, or splitting the
/// large struct into multiple types that each have a single responsibility.
///
/// Mitigation: Use `#![warn(clippy::struct_excessive_bools)]` to catch structs with too many
/// boolean fields. Limit structs to 20-25 fields maximum. Group related fields into nested
/// structs. Consider if the struct is trying to do too much - apply Single Responsibility
/// Principle. Use builder pattern for structs with many optional fields.
// PROBLEM E1104: Struct with too many fields (30+ fields)

pub struct E1104OversizedConfig {
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub max_connections: usize,
    pub enable_tls: bool,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    pub enable_compression: bool,
    pub compression_level: u8,
    pub buffer_size: usize,
    pub max_buffer_size: usize,
    pub enable_logging: bool,
    pub log_level: String,
    pub log_file_path: String,
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub enable_tracing: bool,
    pub trace_sample_rate: f64,
    pub user_agent: String,
    pub api_key: String,
    pub api_secret: String,
    pub enable_auth: bool,
    pub auth_timeout_ms: u64,
    pub session_duration_sec: u64,
    pub enable_rate_limiting: bool,
    pub rate_limit_requests: u32,
    pub rate_limit_window_sec: u32,
    pub enable_caching: bool,
    pub cache_ttl_sec: u64,
    pub cache_max_size: usize,
}

impl E1104OversizedConfig {
    // PROBLEM E1104: Constructor is unwieldy with so many fields
    pub fn e1104_overly_large_struct() -> Self {
        Self {
            host: String::from("localhost"),
            port: 8080,
            timeout_ms: 5000,
            retry_count: 3,
            max_connections: 100,
            enable_tls: false,
            tls_cert_path: String::new(),
            tls_key_path: String::new(),
            enable_compression: true,
            compression_level: 6,
            buffer_size: 8192,
            max_buffer_size: 65536,
            enable_logging: true,
            log_level: String::from("info"),
            log_file_path: String::from("/var/log/app.log"),
            enable_metrics: false,
            metrics_port: 9090,
            enable_tracing: false,
            trace_sample_rate: 0.1,
            user_agent: String::from("MyApp/1.0"),
            api_key: String::new(),
            api_secret: String::new(),
            enable_auth: true,
            auth_timeout_ms: 30000,
            session_duration_sec: 3600,
            enable_rate_limiting: true,
            rate_limit_requests: 100,
            rate_limit_window_sec: 60,
            enable_caching: true,
            cache_ttl_sec: 300,
            cache_max_size: 1000,
        }
    }
}

pub fn e1104_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = E1104OversizedConfig::e1104_overly_large_struct();
    Ok(())
}
