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
    /// Version-specific configuration accepted by `CreateFunction` /
    /// `UpdateFunctionConfiguration` and echoed back on every read.
    pub settings: FunctionSettings,
}

/// Store-and-echo configuration of a function, in the shape of the Lambda
/// API ( not of Terraform's `aws_lambda_function` blocks; the converter
/// translates ).
///
/// `None` / empty means "never supplied".  The documented defaults
/// ( `x86_64`, 512 MB, `PassThrough`, `Zip`, the `/aws/lambda/<name>` log
/// group ) are applied on read through the `effective_*` accessors, so that
/// snapshots written before these members were tracked load unchanged.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// `DeadLetterConfig.TargetArn`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_arn: Option<String>,
    /// `EphemeralStorage.Size`, in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage_size: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_system_configs: Vec<FunctionFileSystemConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<FunctionImageConfig>,
    /// `Code.ImageUri` of a container-image function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<FunctionLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<FunctionLoggingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// `SnapStart.ApplyOn`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_start_apply_on: Option<String>,
    /// `TracingConfig.Mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_mode: Option<String>,
    /// `None` for a function that never had a `VpcConfig`; a value with
    /// empty id lists for a function that was detached from its VPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<FunctionVpcConfig>,
}

pub const DEFAULT_ARCHITECTURE: &str = "x86_64";
pub const DEFAULT_EPHEMERAL_STORAGE_SIZE: i32 = 512;
pub const DEFAULT_TRACING_MODE: &str = "PassThrough";
pub const DEFAULT_PACKAGE_TYPE: &str = "Zip";
pub const DEFAULT_LOG_FORMAT: &str = "Text";
pub const DEFAULT_SNAP_START_APPLY_ON: &str = "None";

impl FunctionSettings {
    pub fn effective_architectures(&self) -> Vec<String> {
        match &self.architectures {
            Some(a) if !a.is_empty() => a.clone(),
            _ => vec![DEFAULT_ARCHITECTURE.to_string()],
        }
    }

    pub fn effective_ephemeral_storage_size(&self) -> i32 {
        self.ephemeral_storage_size
            .unwrap_or(DEFAULT_EPHEMERAL_STORAGE_SIZE)
    }

    pub fn effective_tracing_mode(&self) -> &str {
        self.tracing_mode.as_deref().unwrap_or(DEFAULT_TRACING_MODE)
    }

    pub fn effective_package_type(&self) -> &str {
        self.package_type.as_deref().unwrap_or(DEFAULT_PACKAGE_TYPE)
    }

    pub fn is_image(&self) -> bool {
        self.effective_package_type() == "Image"
    }

    pub fn effective_snap_start_apply_on(&self) -> &str {
        self.snap_start_apply_on
            .as_deref()
            .unwrap_or(DEFAULT_SNAP_START_APPLY_ON)
    }

    /// The logging configuration as Lambda reports it: `Text` format and the
    /// `/aws/lambda/<function name>` log group unless configured otherwise,
    /// and, for the `JSON` format only, log levels that default to `INFO`.
    pub fn effective_logging_config(&self, function_name: &str) -> FunctionLoggingConfig {
        let stored = self.logging_config.clone().unwrap_or_default();
        let log_format = stored
            .log_format
            .unwrap_or_else(|| DEFAULT_LOG_FORMAT.to_string());
        let json = log_format == "JSON";
        let level = |v: Option<String>| json.then(|| v.unwrap_or_else(|| "INFO".to_string()));
        FunctionLoggingConfig {
            application_log_level: level(stored.application_log_level),
            system_log_level: level(stored.system_log_level),
            log_group: Some(
                stored
                    .log_group
                    .unwrap_or_else(|| format!("/aws/lambda/{function_name}")),
            ),
            log_format: Some(log_format),
        }
    }

    /// Apply the members a `CreateFunction` / `UpdateFunctionConfiguration`
    /// request supplied, leaving the rest alone.  `layers` carries the code
    /// size of each layer version, resolved by the caller.
    pub fn apply(&mut self, patch: FunctionSettingsPatch, layers: Option<Vec<FunctionLayer>>) {
        fn set<T>(slot: &mut Option<T>, value: Option<T>) {
            if value.is_some() {
                *slot = value;
            }
        }
        /// An empty string clears the member ( `KMSKeyArn: ""` ).
        fn set_or_clear(slot: &mut Option<String>, value: Option<String>) {
            if let Some(v) = value {
                *slot = (!v.is_empty()).then_some(v);
            }
        }
        if patch.architectures.as_ref().is_some_and(|a| !a.is_empty()) {
            self.architectures = patch.architectures;
        }
        set_or_clear(
            &mut self.dead_letter_target_arn,
            patch.dead_letter_target_arn,
        );
        set(
            &mut self.ephemeral_storage_size,
            patch.ephemeral_storage_size,
        );
        if let Some(configs) = patch.file_system_configs {
            self.file_system_configs = configs;
        }
        if let Some(image_config) = patch.image_config {
            self.image_config = (!image_config.is_empty()).then_some(image_config);
        }
        set_or_clear(&mut self.image_uri, patch.image_uri);
        set_or_clear(&mut self.kms_key_arn, patch.kms_key_arn);
        if let Some(layers) = layers {
            self.layers = layers;
        }
        if let Some(logging) = patch.logging_config {
            let current = self.logging_config.get_or_insert_with(Default::default);
            set(&mut current.log_format, logging.log_format);
            set(&mut current.log_group, logging.log_group);
            set(
                &mut current.application_log_level,
                logging.application_log_level,
            );
            set(&mut current.system_log_level, logging.system_log_level);
        }
        set(&mut self.package_type, patch.package_type);
        set(&mut self.snap_start_apply_on, patch.snap_start_apply_on);
        set(&mut self.tracing_mode, patch.tracing_mode);
        if let Some(vpc) = patch.vpc_config {
            let current = self.vpc_config.take().unwrap_or_default();
            let subnet_ids = vpc.subnet_ids.unwrap_or_else(|| current.subnet_ids.clone());
            // The VPC is a property of the subnets: an id learnt for one set
            // of subnets says nothing about another.
            let vpc_id = current
                .vpc_id
                .filter(|_| same_ids(&subnet_ids, &current.subnet_ids));
            self.vpc_config = Some(FunctionVpcConfig {
                security_group_ids: vpc.security_group_ids.unwrap_or(current.security_group_ids),
                ipv6_allowed_for_dual_stack: vpc
                    .ipv6_allowed_for_dual_stack
                    .unwrap_or(current.ipv6_allowed_for_dual_stack),
                subnet_ids,
                vpc_id,
            });
        }
    }
}

fn same_ids(a: &[String], b: &[String]) -> bool {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    a.sort();
    b.sort();
    a == b
}

/// The settings a `CreateFunction` / `UpdateFunctionConfiguration` request
/// carries.  `None` means the request did not mention the member.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunctionSettingsPatch {
    pub architectures: Option<Vec<String>>,
    /// `Some("")` removes the dead-letter queue.
    pub dead_letter_target_arn: Option<String>,
    pub ephemeral_storage_size: Option<i32>,
    pub file_system_configs: Option<Vec<FunctionFileSystemConfig>>,
    /// An image configuration with no member set removes the overrides.
    pub image_config: Option<FunctionImageConfig>,
    pub image_uri: Option<String>,
    /// `Some("")` reverts to the AWS-owned key.
    pub kms_key_arn: Option<String>,
    /// Layer version ARNs; `Some(vec![])` removes all layers.
    pub layers: Option<Vec<String>>,
    pub logging_config: Option<FunctionLoggingConfig>,
    pub package_type: Option<String>,
    /// `Role`, the one member here that is not part of [`FunctionSettings`].
    pub role: Option<String>,
    pub snap_start_apply_on: Option<String>,
    pub tracing_mode: Option<String>,
    pub vpc_config: Option<FunctionVpcConfigPatch>,
}

/// `VpcConfig` as stored.  Both id lists empty means "not connected to a VPC".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionVpcConfig {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub ipv6_allowed_for_dual_stack: bool,
    /// Lambda derives `VpcId` from the subnets.  This crate has no view of
    /// EC2, so the id is known only when something that does know it supplied
    /// it ( a Terraform state import ); otherwise it is not reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl FunctionVpcConfig {
    pub fn is_attached(&self) -> bool {
        !self.subnet_ids.is_empty() || !self.security_group_ids.is_empty()
    }
}

/// `VpcConfig` of a request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunctionVpcConfigPatch {
    pub subnet_ids: Option<Vec<String>>,
    pub security_group_ids: Option<Vec<String>>,
    pub ipv6_allowed_for_dual_stack: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionFileSystemConfig {
    pub arn: String,
    pub local_mount_path: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionImageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl FunctionImageConfig {
    pub fn is_empty(&self) -> bool {
        self.command.as_ref().is_none_or(|c| c.is_empty())
            && self.entry_point.as_ref().is_none_or(|e| e.is_empty())
            && self.working_directory.as_ref().is_none_or(|w| w.is_empty())
    }
}

/// A layer attached to a function, as reported in `Layers`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionLayer {
    pub arn: String,
    /// Size of the layer archive; 0 when the layer version is not one this
    /// account and region holds.
    pub code_size: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FunctionLoggingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_log_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log_level: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionVersion {
    pub version: String,
    pub description: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub created_at: DateTime<Utc>,
    /// The settings the version was published with.  A published version is
    /// immutable: later `UpdateFunctionConfiguration` calls change `$LATEST`
    /// only.
    pub settings: FunctionSettings,
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

impl EventSourceMapping {
    /// `EventSourceMappingArn` as returned by every mapping operation.
    pub fn arn(&self) -> String {
        event_source_mapping_arn(&self.function_arn, &self.uuid)
    }
}

/// Derive `arn:aws:lambda:<region>:<account>:event-source-mapping:<uuid>`.
///
/// The ARN is never stored: a mapping always lives in the account and region
/// of the function it targets, so both are read off the function ARN.  That
/// keeps snapshots written before the ARN was reported loadable as they are.
pub fn event_source_mapping_arn(function_arn: &str, uuid: &str) -> String {
    let parts: Vec<&str> = function_arn.split(':').collect();
    let (partition, region, account_id) = match parts.as_slice() {
        ["arn", partition, "lambda", region, account_id, ..]
            if !region.is_empty() && !account_id.is_empty() =>
        {
            (*partition, *region, *account_id)
        }
        _ => ("aws", "us-east-1", "123456789012"),
    };
    format!("arn:{partition}:lambda:{region}:{account_id}:event-source-mapping:{uuid}")
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
    /// Resource tags of the mapping ( `Tags` on create, then `TagResource` /
    /// `UntagResource` ).  Kept here rather than on the mapping view so that
    /// older snapshots load unchanged; [`Self::merge`] never touches them,
    /// because `UpdateEventSourceMapping` cannot change tags.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl EventSourceMappingSettings {
    /// Overwrite every member that `patch` supplies, leaving the rest alone.
    /// `tags` are not a patchable setting and are left as they are.
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
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
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
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
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
