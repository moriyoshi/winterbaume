#[derive(Debug, Clone)]
pub struct AppRunnerService {
    pub service_id: String,
    pub service_name: String,
    pub service_arn: String,
    pub service_url: String,
    pub status: String,
    pub created_at: f64,
    pub updated_at: f64,
    pub tags: Vec<(String, String)>,
    pub encryption_configuration: Option<serde_json::Value>,
    pub health_check_configuration: Option<serde_json::Value>,
    pub instance_configuration: Option<serde_json::Value>,
    pub network_configuration: Option<serde_json::Value>,
    pub observability_configuration: Option<serde_json::Value>,
    pub source_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_name: String,
    pub connection_arn: String,
    pub provider_type: String,
    pub status: String,
    pub created_at: f64,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct AutoScalingConfig {
    pub name: String,
    pub arn: String,
    pub revision: i32,
    pub status: String,
    pub is_default: bool,
    pub latest: bool,
    pub min_size: i32,
    pub max_size: i32,
    pub max_concurrency: i32,
    pub created_at: f64,
    pub tags: Vec<(String, String)>,
}
