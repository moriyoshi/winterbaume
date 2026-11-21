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
    /// Resource-level configuration revision id.  AWS bumps this on every
    /// successful UpdateFunctionConfiguration / UpdateFunctionCode and uses
    /// it as the optimistic-concurrency token across those ops plus
    /// PublishVersion.
    pub revision_id: String,
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
    /// Scalar and list settings accepted by Create/UpdateEventSourceMapping
    /// and echoed back verbatim on every read.
    pub settings: EventSourceMappingSettings,
}

/// Store-and-echo settings of an event source mapping.
///
/// Every member is optional: `None` means "never supplied", so the member is
/// omitted from responses.  The same struct doubles as the patch applied by
/// `UpdateEventSourceMapping`, where `None` means "leave unchanged".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventSourceMappingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<EventSourceMappingLoggingSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<EventSourceMappingScalingSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i32>,
}

impl EventSourceMappingSettings {
    /// Overwrite every member that `patch` supplies, leaving the rest alone.
    pub fn merge(&mut self, patch: EventSourceMappingSettings) {
        fn set<T>(slot: &mut Option<T>, value: Option<T>) {
            if value.is_some() {
                *slot = value;
            }
        }
        set(
            &mut self.bisect_batch_on_function_error,
            patch.bisect_batch_on_function_error,
        );
        set(
            &mut self.function_response_types,
            patch.function_response_types,
        );
        set(&mut self.kms_key_arn, patch.kms_key_arn);
        set(&mut self.logging_config, patch.logging_config);
        set(
            &mut self.maximum_batching_window_in_seconds,
            patch.maximum_batching_window_in_seconds,
        );
        set(
            &mut self.maximum_record_age_in_seconds,
            patch.maximum_record_age_in_seconds,
        );
        set(
            &mut self.maximum_retry_attempts,
            patch.maximum_retry_attempts,
        );
        set(
            &mut self.parallelization_factor,
            patch.parallelization_factor,
        );
        set(&mut self.queues, patch.queues);
        set(&mut self.scaling_config, patch.scaling_config);
        set(
            &mut self.starting_position_timestamp,
            patch.starting_position_timestamp,
        );
        set(&mut self.topics, patch.topics);
        set(
            &mut self.tumbling_window_in_seconds,
            patch.tumbling_window_in_seconds,
        );
    }
}

/// `ScalingConfig` of an event source mapping.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventSourceMappingScalingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_concurrency: Option<i32>,
}

/// `LoggingConfig` of an event source mapping.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventSourceMappingLoggingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log_level: Option<String>,
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
    /// Layer-version policy revision id.  Empty until the first
    /// `AddLayerVersionPermission` populates it; bumped on every successful
    /// add or remove so callers can use the modelled `RevisionId`
    /// optimistic-concurrency check.
    pub policy_revision_id: String,
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

#[derive(Debug, Clone, Default)]
pub struct Permission {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub source_arn: Option<String>,
    pub source_account: Option<String>,
    pub principal_org_id: Option<String>,
    pub event_source_token: Option<String>,
    pub function_url_auth_type: Option<String>,
    pub invoked_via_function_url: Option<bool>,
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
