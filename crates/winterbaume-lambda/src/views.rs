//! Serde-compatible view types for Lambda state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::LambdaService;
use crate::state::LambdaState;
use crate::types::{
    Alias, CapacityProviderEntry, CodeSigningConfigEntry, CorsConfig, DurableExecution,
    EventSourceMapping, FunctionEventInvokeConfig, FunctionRecursionConfig, FunctionScalingConfig,
    FunctionUrlConfig, FunctionVersion, LambdaFunction, LayerPermission, LayerVersion, Permission,
    ProvisionedConcurrencyConfig, RuntimeManagementConfig,
};

/// Serializable view of the entire Lambda state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LambdaStateView {
    /// Functions keyed by function name.
    #[serde(default)]
    pub functions: HashMap<String, LambdaFunctionView>,
    /// Aliases keyed by "{function_name}:{alias_name}".
    #[serde(default)]
    pub aliases: HashMap<String, AliasView>,
    /// Event source mappings keyed by UUID.
    #[serde(default)]
    pub event_source_mappings: HashMap<String, EventSourceMappingView>,
    /// Layers keyed by layer name, containing ordered versions.
    #[serde(default)]
    pub layers: HashMap<String, Vec<LayerVersionView>>,
    /// Next version numbers keyed by layer name.
    #[serde(default)]
    pub layer_next_version: HashMap<String, i64>,
    /// Function URL configs keyed by function name.
    #[serde(default)]
    pub function_url_configs: HashMap<String, FunctionUrlConfigView>,
    /// Function policies keyed by function name.
    #[serde(default)]
    pub function_permissions: HashMap<String, Vec<PermissionView>>,
    /// Per-function policy revision ids keyed by function name.
    #[serde(default)]
    pub function_policy_revisions: HashMap<String, String>,
    /// Function event invoke configs keyed by function name.
    #[serde(default)]
    pub function_event_invoke_configs: HashMap<String, FunctionEventInvokeConfigView>,
    /// Code signing configs keyed by ID.
    #[serde(default)]
    pub code_signing_configs: HashMap<String, CodeSigningConfigEntry>,
    /// Provisioned concurrency keyed by "functionName:qualifier".
    #[serde(default)]
    pub provisioned_concurrency: HashMap<String, ProvisionedConcurrencyConfig>,
    /// Capacity providers keyed by name.
    #[serde(default)]
    pub capacity_providers: HashMap<String, CapacityProviderEntry>,
    /// Function recursion configs keyed by function name.
    #[serde(default)]
    pub function_recursion_configs: HashMap<String, FunctionRecursionConfig>,
    /// Function scaling configs keyed by function name.
    #[serde(default)]
    pub function_scaling_configs: HashMap<String, FunctionScalingConfig>,
    /// Runtime management configs keyed by function name.
    #[serde(default)]
    pub runtime_management_configs: HashMap<String, RuntimeManagementConfig>,
    /// Counter for CSC IDs.
    #[serde(default)]
    pub csc_id_counter: u64,
    /// Counter for capacity provider IDs.
    #[serde(default)]
    pub cp_id_counter: u64,
    /// Durable executions keyed by execution ARN.
    #[serde(default)]
    pub durable_executions: HashMap<String, DurableExecution>,
}

/// Serializable view of a Lambda function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaFunctionView {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: String,
    pub handler: String,
    pub role: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub memory_size: i32,
    pub timeout: i32,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    pub description: String,
    pub last_modified: String,
    pub state: String,
    pub version: String,
    #[serde(default)]
    pub revision_id: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub versions: Vec<FunctionVersionView>,
    pub reserved_concurrent_executions: Option<i32>,
    pub code_signing_config_arn: Option<String>,
    /// Round-trip storage for nested Terraform blocks not otherwise tracked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_system_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap_start: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<serde_json::Value>,
}

/// Serializable view of a function version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionVersionView {
    pub version: String,
    pub description: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub created_at: String,
}

/// Serializable view of a Lambda alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasView {
    pub name: String,
    pub function_name: String,
    pub function_version: String,
    pub description: String,
    pub alias_arn: String,
    pub revision_id: String,
    pub routing_config: Option<HashMap<String, f64>>,
}

/// Serializable view of an event source mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSourceMappingView {
    pub uuid: String,
    pub event_source_arn: Option<String>,
    pub function_arn: String,
    pub batch_size: Option<i32>,
    pub enabled: bool,
    pub state: String,
    pub last_modified: String,
    pub starting_position: Option<String>,
    /// Round-trip storage for nested Terraform blocks not otherwise tracked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_access_configuration: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_db_event_source_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned_poller_config: Option<serde_json::Value>,
}

/// Serializable view of a layer version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerVersionView {
    pub layer_name: String,
    pub version: i64,
    pub layer_arn: String,
    pub layer_version_arn: String,
    pub description: String,
    #[serde(default)]
    pub compatible_runtimes: Vec<String>,
    #[serde(default)]
    pub compatible_architectures: Vec<String>,
    pub license_info: Option<String>,
    pub created_date: String,
    pub code_sha256: String,
    pub code_size: i64,
    #[serde(default)]
    pub permissions: Vec<LayerPermissionView>,
    #[serde(default)]
    pub policy_revision_id: String,
}

/// Serializable view of a layer permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerPermissionView {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub organization_id: Option<String>,
}

/// Serializable view of a function URL config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionUrlConfigView {
    pub function_name: String,
    pub function_arn: String,
    pub function_url: String,
    pub auth_type: String,
    pub cors: Option<CorsConfigView>,
    pub invoke_mode: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
}

/// Serializable view of a CORS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfigView {
    pub allow_credentials: Option<bool>,
    pub allow_headers: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_origins: Option<Vec<String>>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<i32>,
}

/// Serializable view of a function permission statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionView {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub source_arn: Option<String>,
    pub source_account: Option<String>,
}

/// Serializable view of a function event invoke config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionEventInvokeConfigView {
    pub function_name: String,
    pub function_arn: String,
    pub maximum_retry_attempts: Option<i32>,
    pub maximum_event_age_in_seconds: Option<i32>,
    pub on_success_destination: Option<String>,
    pub on_failure_destination: Option<String>,
    pub last_modified: String,
}

// --- From internal types to view types ---

impl From<&LambdaState> for LambdaStateView {
    fn from(state: &LambdaState) -> Self {
        LambdaStateView {
            functions: state
                .functions
                .iter()
                .map(|(k, v)| (k.clone(), LambdaFunctionView::from(v)))
                .collect(),
            aliases: state
                .aliases
                .iter()
                .map(|(k, v)| (k.clone(), AliasView::from(v)))
                .collect(),
            event_source_mappings: state
                .event_source_mappings
                .iter()
                .map(|(k, v)| (k.clone(), EventSourceMappingView::from(v)))
                .collect(),
            layers: state
                .layers
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(LayerVersionView::from).collect()))
                .collect(),
            layer_next_version: state.layer_next_version.clone(),
            function_url_configs: state
                .function_url_configs
                .iter()
                .map(|(k, v)| (k.clone(), FunctionUrlConfigView::from(v)))
                .collect(),
            function_permissions: state
                .function_permissions
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(PermissionView::from).collect()))
                .collect(),
            function_policy_revisions: state.function_policy_revisions.clone(),
            function_event_invoke_configs: state
                .function_event_invoke_configs
                .iter()
                .map(|(k, v)| (k.clone(), FunctionEventInvokeConfigView::from(v)))
                .collect(),
            code_signing_configs: state.code_signing_configs.clone(),
            provisioned_concurrency: state.provisioned_concurrency.clone(),
            capacity_providers: state.capacity_providers.clone(),
            function_recursion_configs: state.function_recursion_configs.clone(),
            function_scaling_configs: state.function_scaling_configs.clone(),
            runtime_management_configs: state.runtime_management_configs.clone(),
            csc_id_counter: state.csc_id_counter,
            cp_id_counter: state.cp_id_counter,
            durable_executions: state.durable_executions.clone(),
        }
    }
}

impl From<&LambdaFunction> for LambdaFunctionView {
    fn from(f: &LambdaFunction) -> Self {
        LambdaFunctionView {
            function_name: f.function_name.clone(),
            function_arn: f.function_arn.clone(),
            runtime: f.runtime.clone(),
            handler: f.handler.clone(),
            role: f.role.clone(),
            code_sha256: f.code_sha256.clone(),
            code_size: f.code_size,
            memory_size: f.memory_size,
            timeout: f.timeout,
            environment: f.environment.clone(),
            description: f.description.clone(),
            last_modified: f.last_modified.to_rfc3339(),
            state: f.state.clone(),
            version: f.version.clone(),
            revision_id: f.revision_id.clone(),
            tags: f.tags.clone(),
            versions: f.versions.iter().map(FunctionVersionView::from).collect(),
            reserved_concurrent_executions: f.reserved_concurrent_executions,
            code_signing_config_arn: f.code_signing_config_arn.clone(),
            dead_letter_config: f.dead_letter_config.clone(),
            ephemeral_storage: f.ephemeral_storage.clone(),
            file_system_config: f.file_system_config.clone(),
            image_config: f.image_config.clone(),
            logging_config: f.logging_config.clone(),
            snap_start: f.snap_start.clone(),
            tracing_config: f.tracing_config.clone(),
            vpc_config: f.vpc_config.clone(),
        }
    }
}

impl From<&FunctionVersion> for FunctionVersionView {
    fn from(v: &FunctionVersion) -> Self {
        FunctionVersionView {
            version: v.version.clone(),
            description: v.description.clone(),
            code_sha256: v.code_sha256.clone(),
            code_size: v.code_size,
            created_at: v.created_at.to_rfc3339(),
        }
    }
}

impl From<&Alias> for AliasView {
    fn from(a: &Alias) -> Self {
        AliasView {
            name: a.name.clone(),
            function_name: a.function_name.clone(),
            function_version: a.function_version.clone(),
            description: a.description.clone(),
            alias_arn: a.alias_arn.clone(),
            revision_id: a.revision_id.clone(),
            routing_config: a.routing_config.clone(),
        }
    }
}

impl From<&EventSourceMapping> for EventSourceMappingView {
    fn from(esm: &EventSourceMapping) -> Self {
        EventSourceMappingView {
            uuid: esm.uuid.clone(),
            event_source_arn: esm.event_source_arn.clone(),
            function_arn: esm.function_arn.clone(),
            batch_size: esm.batch_size,
            enabled: esm.enabled,
            state: esm.state.clone(),
            last_modified: esm.last_modified.to_rfc3339(),
            starting_position: esm.starting_position.clone(),
            destination_config: esm.destination_config.clone(),
            filter_criteria: esm.filter_criteria.clone(),
            self_managed_event_source: esm.self_managed_event_source.clone(),
            source_access_configuration: esm.source_access_configuration.clone(),
            self_managed_kafka_event_source_config: esm
                .self_managed_kafka_event_source_config
                .clone(),
            amazon_managed_kafka_event_source_config: esm
                .amazon_managed_kafka_event_source_config
                .clone(),
            document_db_event_source_config: esm.document_db_event_source_config.clone(),
            metrics_config: esm.metrics_config.clone(),
            provisioned_poller_config: esm.provisioned_poller_config.clone(),
        }
    }
}

impl From<&LayerVersion> for LayerVersionView {
    fn from(lv: &LayerVersion) -> Self {
        LayerVersionView {
            layer_name: lv.layer_name.clone(),
            version: lv.version,
            layer_arn: lv.layer_arn.clone(),
            layer_version_arn: lv.layer_version_arn.clone(),
            description: lv.description.clone(),
            compatible_runtimes: lv.compatible_runtimes.clone(),
            compatible_architectures: lv.compatible_architectures.clone(),
            license_info: lv.license_info.clone(),
            created_date: lv.created_date.clone(),
            code_sha256: lv.code_sha256.clone(),
            code_size: lv.code_size,
            permissions: lv
                .permissions
                .iter()
                .map(LayerPermissionView::from)
                .collect(),
            policy_revision_id: lv.policy_revision_id.clone(),
        }
    }
}

impl From<&LayerPermission> for LayerPermissionView {
    fn from(lp: &LayerPermission) -> Self {
        LayerPermissionView {
            statement_id: lp.statement_id.clone(),
            action: lp.action.clone(),
            principal: lp.principal.clone(),
            organization_id: lp.organization_id.clone(),
        }
    }
}

impl From<&FunctionUrlConfig> for FunctionUrlConfigView {
    fn from(fuc: &FunctionUrlConfig) -> Self {
        FunctionUrlConfigView {
            function_name: fuc.function_name.clone(),
            function_arn: fuc.function_arn.clone(),
            function_url: fuc.function_url.clone(),
            auth_type: fuc.auth_type.clone(),
            cors: fuc.cors.as_ref().map(CorsConfigView::from),
            invoke_mode: fuc.invoke_mode.clone(),
            creation_time: fuc.creation_time.clone(),
            last_modified_time: fuc.last_modified_time.clone(),
        }
    }
}

impl From<&CorsConfig> for CorsConfigView {
    fn from(c: &CorsConfig) -> Self {
        CorsConfigView {
            allow_credentials: c.allow_credentials,
            allow_headers: c.allow_headers.clone(),
            allow_methods: c.allow_methods.clone(),
            allow_origins: c.allow_origins.clone(),
            expose_headers: c.expose_headers.clone(),
            max_age: c.max_age,
        }
    }
}

impl From<&Permission> for PermissionView {
    fn from(p: &Permission) -> Self {
        PermissionView {
            statement_id: p.statement_id.clone(),
            action: p.action.clone(),
            principal: p.principal.clone(),
            source_arn: p.source_arn.clone(),
            source_account: p.source_account.clone(),
        }
    }
}

impl From<&FunctionEventInvokeConfig> for FunctionEventInvokeConfigView {
    fn from(feic: &FunctionEventInvokeConfig) -> Self {
        FunctionEventInvokeConfigView {
            function_name: feic.function_name.clone(),
            function_arn: feic.function_arn.clone(),
            maximum_retry_attempts: feic.maximum_retry_attempts,
            maximum_event_age_in_seconds: feic.maximum_event_age_in_seconds,
            on_success_destination: feic.on_success_destination.clone(),
            on_failure_destination: feic.on_failure_destination.clone(),
            last_modified: feic.last_modified.to_rfc3339(),
        }
    }
}

// --- From view types to internal types ---

impl LambdaStateView {
    fn into_state_for_scope(self, account_id: &str, region: &str) -> LambdaState {
        LambdaState {
            functions: self
                .functions
                .into_iter()
                .map(|(k, v)| (k, LambdaFunction::from(v)))
                .collect(),
            aliases: self
                .aliases
                .into_iter()
                .map(|(k, v)| (k, Alias::from(v)))
                .collect(),
            event_source_mappings: self
                .event_source_mappings
                .into_iter()
                .map(|(k, v)| (k, EventSourceMapping::from(v)))
                .collect(),
            layers: self
                .layers
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(LayerVersion::from).collect()))
                .collect(),
            layer_next_version: self.layer_next_version,
            function_url_configs: self
                .function_url_configs
                .into_iter()
                .map(|(k, v)| (k, FunctionUrlConfig::from(v)))
                .collect(),
            function_permissions: self
                .function_permissions
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(Permission::from).collect()))
                .collect(),
            function_policy_revisions: self.function_policy_revisions,
            function_event_invoke_configs: self
                .function_event_invoke_configs
                .into_iter()
                .map(|(k, v)| (k, FunctionEventInvokeConfig::from(v)))
                .collect(),
            account_id: account_id.to_string(),
            region: region.to_string(),
            code_signing_configs: self.code_signing_configs,
            provisioned_concurrency: self.provisioned_concurrency,
            capacity_providers: self.capacity_providers,
            function_recursion_configs: self.function_recursion_configs,
            function_scaling_configs: self.function_scaling_configs,
            runtime_management_configs: self.runtime_management_configs,
            csc_id_counter: self.csc_id_counter,
            cp_id_counter: self.cp_id_counter,
            empty_tags: HashMap::new(),
            durable_executions: self.durable_executions,
        }
    }
}

impl From<LambdaFunctionView> for LambdaFunction {
    fn from(v: LambdaFunctionView) -> Self {
        let last_modified = DateTime::parse_from_rfc3339(&v.last_modified)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        LambdaFunction {
            function_name: v.function_name,
            function_arn: v.function_arn,
            runtime: v.runtime,
            handler: v.handler,
            role: v.role,
            code_sha256: v.code_sha256,
            code_size: v.code_size,
            memory_size: v.memory_size,
            timeout: v.timeout,
            environment: v.environment,
            description: v.description,
            last_modified,
            state: v.state,
            version: v.version,
            revision_id: if v.revision_id.is_empty() {
                uuid::Uuid::new_v4().to_string()
            } else {
                v.revision_id
            },
            tags: v.tags,
            versions: v.versions.into_iter().map(FunctionVersion::from).collect(),
            reserved_concurrent_executions: v.reserved_concurrent_executions,
            code_signing_config_arn: v.code_signing_config_arn,
            dead_letter_config: v.dead_letter_config,
            ephemeral_storage: v.ephemeral_storage,
            file_system_config: v.file_system_config,
            image_config: v.image_config,
            logging_config: v.logging_config,
            snap_start: v.snap_start,
            tracing_config: v.tracing_config,
            vpc_config: v.vpc_config,
        }
    }
}

impl From<FunctionVersionView> for FunctionVersion {
    fn from(v: FunctionVersionView) -> Self {
        let created_at = DateTime::parse_from_rfc3339(&v.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        FunctionVersion {
            version: v.version,
            description: v.description,
            code_sha256: v.code_sha256,
            code_size: v.code_size,
            created_at,
        }
    }
}

impl From<AliasView> for Alias {
    fn from(v: AliasView) -> Self {
        Alias {
            name: v.name,
            function_name: v.function_name,
            function_version: v.function_version,
            description: v.description,
            alias_arn: v.alias_arn,
            revision_id: v.revision_id,
            routing_config: v.routing_config,
        }
    }
}

impl From<EventSourceMappingView> for EventSourceMapping {
    fn from(v: EventSourceMappingView) -> Self {
        let last_modified = DateTime::parse_from_rfc3339(&v.last_modified)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        EventSourceMapping {
            uuid: v.uuid,
            event_source_arn: v.event_source_arn,
            function_arn: v.function_arn,
            batch_size: v.batch_size,
            enabled: v.enabled,
            state: v.state,
            last_modified,
            starting_position: v.starting_position,
            destination_config: v.destination_config,
            filter_criteria: v.filter_criteria,
            self_managed_event_source: v.self_managed_event_source,
            source_access_configuration: v.source_access_configuration,
            self_managed_kafka_event_source_config: v.self_managed_kafka_event_source_config,
            amazon_managed_kafka_event_source_config: v.amazon_managed_kafka_event_source_config,
            document_db_event_source_config: v.document_db_event_source_config,
            metrics_config: v.metrics_config,
            provisioned_poller_config: v.provisioned_poller_config,
        }
    }
}

impl From<LayerVersionView> for LayerVersion {
    fn from(v: LayerVersionView) -> Self {
        LayerVersion {
            layer_name: v.layer_name,
            version: v.version,
            layer_arn: v.layer_arn,
            layer_version_arn: v.layer_version_arn,
            description: v.description,
            compatible_runtimes: v.compatible_runtimes,
            compatible_architectures: v.compatible_architectures,
            license_info: v.license_info,
            created_date: v.created_date,
            code_sha256: v.code_sha256,
            code_size: v.code_size,
            permissions: v
                .permissions
                .into_iter()
                .map(LayerPermission::from)
                .collect(),
            policy_revision_id: v.policy_revision_id,
        }
    }
}

impl From<LayerPermissionView> for LayerPermission {
    fn from(v: LayerPermissionView) -> Self {
        LayerPermission {
            statement_id: v.statement_id,
            action: v.action,
            principal: v.principal,
            organization_id: v.organization_id,
        }
    }
}

impl From<FunctionUrlConfigView> for FunctionUrlConfig {
    fn from(v: FunctionUrlConfigView) -> Self {
        FunctionUrlConfig {
            function_name: v.function_name,
            function_arn: v.function_arn,
            function_url: v.function_url,
            auth_type: v.auth_type,
            cors: v.cors.map(CorsConfig::from),
            invoke_mode: v.invoke_mode,
            creation_time: v.creation_time,
            last_modified_time: v.last_modified_time,
        }
    }
}

impl From<CorsConfigView> for CorsConfig {
    fn from(v: CorsConfigView) -> Self {
        CorsConfig {
            allow_credentials: v.allow_credentials,
            allow_headers: v.allow_headers,
            allow_methods: v.allow_methods,
            allow_origins: v.allow_origins,
            expose_headers: v.expose_headers,
            max_age: v.max_age,
        }
    }
}

impl From<PermissionView> for Permission {
    fn from(v: PermissionView) -> Self {
        Permission {
            statement_id: v.statement_id,
            action: v.action,
            principal: v.principal,
            source_arn: v.source_arn,
            source_account: v.source_account,
        }
    }
}

impl From<FunctionEventInvokeConfigView> for FunctionEventInvokeConfig {
    fn from(v: FunctionEventInvokeConfigView) -> Self {
        let last_modified = DateTime::parse_from_rfc3339(&v.last_modified)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        FunctionEventInvokeConfig {
            function_name: v.function_name,
            function_arn: v.function_arn,
            maximum_retry_attempts: v.maximum_retry_attempts,
            maximum_event_age_in_seconds: v.maximum_event_age_in_seconds,
            on_success_destination: v.on_success_destination,
            on_failure_destination: v.on_failure_destination,
            last_modified,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for LambdaService {
    type StateView = LambdaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        LambdaStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = view.into_state_for_scope(account_id, region);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.account_id = account_id.to_string();
            guard.region = region.to_string();
            for (name, func_view) in view.functions {
                guard
                    .functions
                    .insert(name, LambdaFunction::from(func_view));
            }
            for (key, alias_view) in view.aliases {
                guard.aliases.insert(key, Alias::from(alias_view));
            }
            for (uuid, esm_view) in view.event_source_mappings {
                guard
                    .event_source_mappings
                    .insert(uuid, EventSourceMapping::from(esm_view));
            }
            for (name, versions) in view.layers {
                guard
                    .layers
                    .insert(name, versions.into_iter().map(LayerVersion::from).collect());
            }
            for (name, next_ver) in view.layer_next_version {
                guard.layer_next_version.insert(name, next_ver);
            }
            for (name, fuc_view) in view.function_url_configs {
                guard
                    .function_url_configs
                    .insert(name, FunctionUrlConfig::from(fuc_view));
            }
            for (name, perms) in view.function_permissions {
                guard
                    .function_permissions
                    .insert(name, perms.into_iter().map(Permission::from).collect());
            }
            guard
                .function_policy_revisions
                .extend(view.function_policy_revisions);
            for (name, feic_view) in view.function_event_invoke_configs {
                guard
                    .function_event_invoke_configs
                    .insert(name, FunctionEventInvokeConfig::from(feic_view));
            }
            guard.code_signing_configs.extend(view.code_signing_configs);
            guard
                .provisioned_concurrency
                .extend(view.provisioned_concurrency);
            guard.capacity_providers.extend(view.capacity_providers);
            guard
                .function_recursion_configs
                .extend(view.function_recursion_configs);
            guard
                .function_scaling_configs
                .extend(view.function_scaling_configs);
            guard
                .runtime_management_configs
                .extend(view.runtime_management_configs);
            guard.csc_id_counter = guard.csc_id_counter.max(view.csc_id_counter);
            guard.cp_id_counter = guard.cp_id_counter.max(view.cp_id_counter);
            guard.durable_executions.extend(view.durable_executions);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn restore_and_merge_seed_runtime_scope_from_arguments() {
        let service = LambdaService::new();

        service
            .restore("999999999999", "eu-west-1", LambdaStateView::default())
            .await
            .unwrap();
        {
            let state = service.state.get("999999999999", "eu-west-1");
            let guard = state.read().await;
            assert_eq!(guard.account_id, "999999999999");
            assert_eq!(guard.region, "eu-west-1");
        }

        service
            .merge("111111111111", "ap-southeast-2", LambdaStateView::default())
            .await
            .unwrap();
        let state = service.state.get("111111111111", "ap-southeast-2");
        let guard = state.read().await;
        assert_eq!(guard.account_id, "111111111111");
        assert_eq!(guard.region, "ap-southeast-2");
    }

    #[test]
    fn lambda_state_view_does_not_serialize_scope_fields() {
        let value = serde_json::to_value(LambdaStateView::default()).unwrap();
        let object = value.as_object().unwrap();

        assert!(!object.contains_key("account_id"));
        assert!(!object.contains_key("region"));
    }
}
