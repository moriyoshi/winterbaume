use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LambdaFunction {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: String,
    pub handler: String,
    pub role: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub memory_size: i32,
    pub timeout: i32,
    pub environment: HashMap<String, String>,
    pub description: String,
    pub last_modified: DateTime<Utc>,
    pub state: String,
    pub version: String,
    pub tags: HashMap<String, String>,
    pub versions: Vec<FunctionVersion>,
    pub reserved_concurrent_executions: Option<i32>,
    pub code_signing_config_arn: Option<String>,
    /// Round-trip storage for nested Terraform blocks not otherwise tracked.
    pub dead_letter_config: Option<serde_json::Value>,
    pub ephemeral_storage: Option<serde_json::Value>,
    pub file_system_config: Option<serde_json::Value>,
    pub image_config: Option<serde_json::Value>,
    pub logging_config: Option<serde_json::Value>,
    pub snap_start: Option<serde_json::Value>,
    pub tracing_config: Option<serde_json::Value>,
    pub vpc_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct FunctionVersion {
    pub version: String,
    pub description: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub function_name: String,
    pub function_version: String,
    pub description: String,
    pub alias_arn: String,
    pub revision_id: String,
    pub routing_config: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone)]
pub struct EventSourceMapping {
    pub uuid: String,
    pub event_source_arn: Option<String>,
    pub function_arn: String,
    pub batch_size: Option<i32>,
    pub enabled: bool,
    pub state: String,
    pub last_modified: DateTime<Utc>,
    pub starting_position: Option<String>,
    /// Round-trip storage for nested Terraform blocks not otherwise tracked.
    pub destination_config: Option<serde_json::Value>,
    pub filter_criteria: Option<serde_json::Value>,
    pub self_managed_event_source: Option<serde_json::Value>,
    pub source_access_configuration: Option<serde_json::Value>,
    pub self_managed_kafka_event_source_config: Option<serde_json::Value>,
    pub amazon_managed_kafka_event_source_config: Option<serde_json::Value>,
    pub document_db_event_source_config: Option<serde_json::Value>,
    pub metrics_config: Option<serde_json::Value>,
    pub provisioned_poller_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct LayerVersion {
    pub layer_name: String,
    pub version: i64,
    pub layer_arn: String,
    pub layer_version_arn: String,
    pub description: String,
    pub compatible_runtimes: Vec<String>,
    pub compatible_architectures: Vec<String>,
    pub license_info: Option<String>,
    pub created_date: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub permissions: Vec<LayerPermission>,
}

#[derive(Debug, Clone)]
pub struct LayerPermission {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionUrlConfig {
    pub function_name: String,
    pub function_arn: String,
    pub function_url: String,
    pub auth_type: String,
    pub cors: Option<CorsConfig>,
    pub invoke_mode: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allow_credentials: Option<bool>,
    pub allow_headers: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_origins: Option<Vec<String>>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub source_arn: Option<String>,
    pub source_account: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionEventInvokeConfig {
    pub function_name: String,
    pub function_arn: String,
    pub maximum_retry_attempts: Option<i32>,
    pub maximum_event_age_in_seconds: Option<i32>,
    pub on_success_destination: Option<String>,
    pub on_failure_destination: Option<String>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSigningConfigEntry {
    pub code_signing_config_id: String,
    pub code_signing_config_arn: String,
    pub description: Option<String>,
    pub allowed_publishers: Vec<String>, // signing profile ARNs
    pub untrusted_artifact_on_deployment: String,
    pub last_modified: String,
    /// Function names linked to this config
    #[serde(default)]
    pub functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedConcurrencyConfig {
    pub function_arn: String,
    pub requested: i32,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityProviderEntry {
    pub capacity_provider_arn: String,
    pub state: String,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRecursionConfig {
    pub recursive_loop: String, // "Allow" or "Terminate"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionScalingConfig {
    pub maximum_concurrency: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeManagementConfig {
    pub update_runtime_on: String,
    pub runtime_version_arn: Option<String>,
}

/// A durable execution record (preview Lambda feature).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableExecution {
    pub durable_execution_arn: String,
    pub durable_execution_name: String,
    pub function_arn: String,
    pub function_name: String,
    pub status: String,
    pub start_timestamp: Option<f64>,
    pub end_timestamp: Option<f64>,
    pub input_payload: Option<String>,
    pub result: Option<String>,
    pub version: Option<String>,
}
