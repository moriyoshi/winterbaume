//! Serde-compatible view types for Lambda state snapshots.

use std::collections::{BTreeSet, HashMap};
#[cfg(test)]
use std::future::Future;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{
    BlobBackedService, BlobExportEntry, BlobSource, BlobVisitor, StateChangeNotifier,
    StateViewError, StatefulService,
};

use crate::execution::code_blob_key;

use crate::handlers::LambdaService;
use crate::state::LambdaState;
use crate::types::{
    Alias, CapacityProviderEntry, CodeSigningConfigEntry, CorsConfig, DurableExecution,
    EventSourceMapping, EventSourceMappingSettings, FunctionEventInvokeConfig,
    FunctionRecursionConfig, FunctionScalingConfig, FunctionSettings, FunctionUrlConfig,
    FunctionVersion, FunctionVersionConfig, LambdaFunction, LayerPermission, LayerVersion,
    Permission, ProvisionedConcurrencyConfig, RuntimeManagementConfig,
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
    /// Base64 SHA-256s of the stored ZIP deployment packages ( the
    /// `code_sha256` of the functions and versions that use them ).
    ///
    /// Only the digests travel in the view; the archives are blobs, exported
    /// and imported through [`BlobBackedService`].  On `restore` / `merge`,
    /// digests whose archive is not in the blob store are dropped.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub code_packages: BTreeSet<String>,
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
    /// Store-and-echo configuration ( `VpcConfig`, `Architectures`,
    /// `LoggingConfig`, ... ) in the shape of the Lambda API.  Absent in
    /// snapshots taken before these were tracked.  Those snapshots may instead
    /// carry Terraform-shaped `vpc_config`, `tracing_config`, ... members
    /// that no API ever read; they are ignored on load.
    #[serde(default)]
    pub settings: FunctionSettings,
}

/// Serializable view of a function version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionVersionView {
    pub version: String,
    pub description: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub created_at: String,
    /// Settings frozen at publish time.  Absent in older snapshots.
    #[serde(default)]
    pub settings: FunctionSettings,
    /// Execution configuration frozen by `PublishVersion`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<FunctionVersionConfig>,
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
    /// Store-and-echo settings ( `FunctionResponseTypes`, `ScalingConfig`, ... ).
    /// Absent in snapshots taken before these were tracked.
    #[serde(default)]
    pub settings: EventSourceMappingSettings,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_org_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_url_auth_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoked_via_function_url: Option<bool>,
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
            code_packages: state.code_packages.clone(),
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
            settings: f.settings.clone(),
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
            settings: v.settings.clone(),
            config: v.config.clone(),
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
            settings: esm.settings.clone(),
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
            principal_org_id: p.principal_org_id.clone(),
            event_source_token: p.event_source_token.clone(),
            function_url_auth_type: p.function_url_auth_type.clone(),
            invoked_via_function_url: p.invoked_via_function_url,
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
            code_packages: self.code_packages,
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
            settings: v.settings,
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
            settings: v.settings,
            config: v.config,
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
            settings: v.settings,
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
            principal_org_id: v.principal_org_id,
            event_source_token: v.event_source_token,
            function_url_auth_type: v.function_url_auth_type,
            invoked_via_function_url: v.invoked_via_function_url,
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
        let mut restored = view.into_state_for_scope(account_id, region);
        restored.code_packages = self
            .stored_code_packages(account_id, region, restored.code_packages)
            .await;
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = restored;
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
        let code_packages = self
            .stored_code_packages(account_id, region, view.code_packages)
            .await;
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
            guard.code_packages.extend(code_packages);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

impl LambdaService {
    /// The subset of `digests` whose archive is in the blob store.
    async fn stored_code_packages(
        &self,
        account_id: &str,
        region: &str,
        digests: BTreeSet<String>,
    ) -> BTreeSet<String> {
        let blobs = self.blobs.get(account_id, region);
        let mut stored = BTreeSet::new();
        for sha256 in digests {
            if blobs.exists(&code_blob_key(&sha256)).await {
                stored.insert(sha256);
            } else {
                tracing::warn!(
                    code_sha256 = %sha256,
                    "dropping Lambda code package whose archive is not in the blob store"
                );
            }
        }
        stored
    }
}

/// Deployment packages are exported as blobs keyed `code/<hex sha256>`.
impl BlobBackedService for LambdaService {
    async fn snapshot_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        visitor: &mut dyn BlobVisitor,
    ) -> Result<LambdaStateView, StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        let view = LambdaStateView::from(&*guard);
        let keys: Vec<String> = view
            .code_packages
            .iter()
            .map(|s| code_blob_key(s))
            .collect();
        for chunk in keys.chunks(winterbaume_core::DEFAULT_BLOB_BATCH_SIZE) {
            let mut entries = Vec::with_capacity(chunk.len());
            for key in chunk {
                let size = blobs
                    .stat(key)
                    .await
                    .map_err(StateViewError::Blob)?
                    .map(|s| s.size);
                let reader = blobs.get_reader(key).await.map_err(StateViewError::Blob)?;
                entries.push(BlobExportEntry {
                    key: key.clone(),
                    reader,
                    size,
                });
            }
            visitor.visit(entries).await.map_err(StateViewError::Blob)?;
        }
        Ok(view)
    }

    async fn restore_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        view: LambdaStateView,
        source: &mut dyn BlobSource,
    ) -> Result<(), StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        for sha256 in &view.code_packages {
            let key = code_blob_key(sha256);
            let mut reader = source.fetch(key.clone()).await?;
            blobs
                .put_from(&key, &mut reader)
                .await
                .map_err(StateViewError::Blob)?;
        }
        self.restore(account_id, region, view).await
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

    /// Snapshots written before issues #44 / #45 carry neither the extra
    /// permission condition inputs nor the event-source-mapping settings.
    /// They must still load, with those members absent.
    #[test]
    fn legacy_snapshot_without_new_members_still_deserializes() {
        let view: LambdaStateView = serde_json::from_value(serde_json::json!({
            "event_source_mappings": {
                "u-1": {
                    "uuid": "u-1",
                    "event_source_arn": "arn:aws:sqs:us-east-1:123456789012:q",
                    "function_arn": "arn:aws:lambda:us-east-1:123456789012:function:f",
                    "batch_size": 10,
                    "enabled": true,
                    "state": "Enabled",
                    "last_modified": "2026-01-01T00:00:00+00:00",
                    "starting_position": null
                }
            },
            "function_permissions": {
                "f": [{
                    "statement_id": "s",
                    "action": "lambda:InvokeFunction",
                    "principal": "*",
                    "source_arn": null,
                    "source_account": null
                }]
            }
        }))
        .expect("legacy snapshot must deserialize");

        assert_eq!(
            view.event_source_mappings["u-1"].settings,
            EventSourceMappingSettings::default()
        );
        let perm = &view.function_permissions["f"][0];
        assert_eq!(perm.function_url_auth_type, None);
        assert_eq!(perm.invoked_via_function_url, None);
        assert_eq!(perm.principal_org_id, None);
        assert_eq!(perm.event_source_token, None);
    }

    /// The new members survive a snapshot -> JSON -> restore round trip.
    #[tokio::test]
    async fn new_members_round_trip_through_snapshot() {
        let service = LambdaService::new();
        let mut view = LambdaStateView::default();
        view.event_source_mappings.insert(
            "u-1".to_string(),
            EventSourceMappingView {
                uuid: "u-1".to_string(),
                event_source_arn: None,
                function_arn: "arn:aws:lambda:us-east-1:123456789012:function:f".to_string(),
                batch_size: None,
                enabled: true,
                state: "Enabled".to_string(),
                last_modified: "2026-01-01T00:00:00+00:00".to_string(),
                starting_position: None,
                destination_config: None,
                filter_criteria: None,
                self_managed_event_source: None,
                source_access_configuration: None,
                self_managed_kafka_event_source_config: None,
                amazon_managed_kafka_event_source_config: None,
                document_db_event_source_config: None,
                metrics_config: None,
                provisioned_poller_config: None,
                settings: EventSourceMappingSettings {
                    function_response_types: Some(vec!["ReportBatchItemFailures".to_string()]),
                    scaling_config: Some(crate::types::EventSourceMappingScalingSettings {
                        maximum_concurrency: Some(7),
                    }),
                    ..Default::default()
                },
            },
        );
        view.function_permissions.insert(
            "f".to_string(),
            vec![PermissionView {
                statement_id: "s".to_string(),
                action: "lambda:InvokeFunctionUrl".to_string(),
                principal: "*".to_string(),
                source_arn: None,
                source_account: None,
                principal_org_id: Some("o-abcdefghij".to_string()),
                event_source_token: None,
                function_url_auth_type: Some("NONE".to_string()),
                invoked_via_function_url: Some(true),
            }],
        );
        service
            .restore("123456789012", "us-east-1", view)
            .await
            .unwrap();

        let json = serde_json::to_value(service.snapshot("123456789012", "us-east-1").await)
            .expect("snapshot must serialize");
        let back: LambdaStateView = serde_json::from_value(json).unwrap();

        let settings = &back.event_source_mappings["u-1"].settings;
        assert_eq!(
            settings.function_response_types.as_deref(),
            Some(&["ReportBatchItemFailures".to_string()][..])
        );
        assert_eq!(
            settings
                .scaling_config
                .as_ref()
                .and_then(|c| c.maximum_concurrency),
            Some(7)
        );
        let perm = &back.function_permissions["f"][0];
        assert_eq!(perm.principal_org_id.as_deref(), Some("o-abcdefghij"));
        assert_eq!(perm.function_url_auth_type.as_deref(), Some("NONE"));
        assert_eq!(perm.invoked_via_function_url, Some(true));
    }

    /// Snapshots written before issue #54 carry no tags on event source
    /// mappings, code signing configs or capacity providers, and no mapping
    /// ARN.  They must still load: tags empty, ARN derived.
    #[test]
    fn legacy_snapshot_without_mapping_tags_still_deserializes() {
        let view: LambdaStateView = serde_json::from_value(serde_json::json!({
            "event_source_mappings": {
                "u-1": {
                    "uuid": "u-1",
                    "event_source_arn": "arn:aws:sqs:eu-west-1:111111111111:q",
                    "function_arn": "arn:aws:lambda:eu-west-1:111111111111:function:f",
                    "batch_size": 10,
                    "enabled": true,
                    "state": "Enabled",
                    "last_modified": "2026-01-01T00:00:00+00:00",
                    "starting_position": null,
                    "settings": { "maximum_retry_attempts": 3 }
                }
            },
            "code_signing_configs": {
                "csc-1": {
                    "code_signing_config_id": "csc-1",
                    "code_signing_config_arn":
                        "arn:aws:lambda:eu-west-1:111111111111:code-signing-config:csc-1",
                    "description": null,
                    "allowed_publishers": [],
                    "untrusted_artifact_on_deployment": "Warn",
                    "last_modified": "2026-01-01T00:00:00+00:00"
                }
            },
            "capacity_providers": {
                "cp": {
                    "capacity_provider_arn":
                        "arn:aws:lambda:eu-west-1:111111111111:capacity-provider:cp",
                    "state": "Active",
                    "last_modified": "2026-01-01T00:00:00+00:00"
                }
            }
        }))
        .expect("legacy snapshot must deserialize");

        let settings = &view.event_source_mappings["u-1"].settings;
        assert_eq!(settings.maximum_retry_attempts, Some(3));
        assert!(settings.tags.is_empty());
        assert!(view.code_signing_configs["csc-1"].tags.is_empty());
        assert!(view.capacity_providers["cp"].tags.is_empty());

        let state = view.into_state_for_scope("111111111111", "eu-west-1");
        let mapping = &state.event_source_mappings["u-1"];
        assert_eq!(
            mapping.arn(),
            "arn:aws:lambda:eu-west-1:111111111111:event-source-mapping:u-1"
        );
        assert!(state.list_tags(&mapping.arn()).unwrap().is_empty());
    }

    /// Mapping tags survive a snapshot round trip, and an untagged mapping
    /// serializes without a `tags` member.
    #[test]
    fn mapping_tags_round_trip_through_snapshot_json() {
        let mut settings = EventSourceMappingSettings::default();
        assert!(
            serde_json::to_value(&settings)
                .unwrap()
                .get("tags")
                .is_none()
        );
        settings.tags.insert("app".to_string(), "demo".to_string());
        let json = serde_json::to_value(&settings).unwrap();
        assert_eq!(json["tags"], serde_json::json!({"app": "demo"}));
        let back: EventSourceMappingSettings = serde_json::from_value(json).unwrap();
        assert_eq!(back, settings);
    }

    /// Snapshots written before issue #62 carry no `settings` on functions
    /// or versions, and may carry the Terraform-shaped blocks the converter
    /// used to park on the function ( `vpc_config = [{...}]`, ... ).  They
    /// must still load: the legacy blocks are ignored and every setting
    /// reads as its documented default.
    #[test]
    fn legacy_snapshot_with_terraform_shaped_function_blocks_still_deserializes() {
        let view: LambdaStateView = serde_json::from_value(serde_json::json!({
            "functions": {
                "f": {
                    "function_name": "f",
                    "function_arn": "arn:aws:lambda:eu-west-1:111111111111:function:f",
                    "runtime": "python3.12",
                    "handler": "index.handler",
                    "role": "arn:aws:iam::111111111111:role/r",
                    "code_sha256": "abc",
                    "code_size": 10,
                    "memory_size": 128,
                    "timeout": 3,
                    "description": "",
                    "last_modified": "2026-01-01T00:00:00+00:00",
                    "state": "Active",
                    "version": "$LATEST",
                    "versions": [{
                        "version": "1",
                        "description": "",
                        "code_sha256": "abc",
                        "code_size": 10,
                        "created_at": "2026-01-01T00:00:00+00:00"
                    }],
                    "reserved_concurrent_executions": null,
                    "code_signing_config_arn": null,
                    "vpc_config": [{
                        "subnet_ids": ["subnet-1"],
                        "security_group_ids": ["sg-1"],
                        "vpc_id": "vpc-1",
                        "ipv6_allowed_for_dual_stack": false
                    }],
                    "tracing_config": [{"mode": "Active"}],
                    "ephemeral_storage": [{"size": 1024}],
                    "logging_config": [{"log_format": "JSON", "log_group": "/x"}],
                    "dead_letter_config": [{"target_arn": "arn:aws:sqs:eu-west-1:1:q"}],
                    "file_system_config": [],
                    "image_config": [],
                    "snap_start": []
                }
            }
        }))
        .expect("legacy snapshot must deserialize");

        let func = &view.functions["f"];
        assert_eq!(func.settings, FunctionSettings::default());
        assert_eq!(func.versions[0].settings, FunctionSettings::default());

        let state = view.into_state_for_scope("111111111111", "eu-west-1");
        let settings = &state.functions["f"].settings;
        assert_eq!(settings.vpc_config, None);
        assert_eq!(settings.effective_architectures(), ["x86_64"]);
        assert_eq!(settings.effective_ephemeral_storage_size(), 512);
        assert_eq!(settings.effective_tracing_mode(), "PassThrough");
        assert_eq!(settings.effective_package_type(), "Zip");
        let logging = settings.effective_logging_config("f");
        assert_eq!(logging.log_format.as_deref(), Some("Text"));
        assert_eq!(logging.log_group.as_deref(), Some("/aws/lambda/f"));
    }

    /// Function settings, on `$LATEST` and on a published version, survive a
    /// snapshot -> JSON -> restore round trip, and a function without any
    /// serializes an empty `settings` object.
    #[tokio::test]
    async fn function_settings_round_trip_through_snapshot() {
        use crate::types::{FunctionLayer, FunctionLoggingConfig, FunctionVpcConfig};

        let settings = FunctionSettings {
            architectures: Some(vec!["arm64".to_string()]),
            dead_letter_target_arn: Some("arn:aws:sqs:us-east-1:123456789012:q".to_string()),
            ephemeral_storage_size: Some(2048),
            kms_key_arn: Some("arn:aws:kms:us-east-1:123456789012:key/k".to_string()),
            layers: vec![FunctionLayer {
                arn: "arn:aws:lambda:us-east-1:123456789012:layer:l:1".to_string(),
                code_size: 42,
            }],
            logging_config: Some(FunctionLoggingConfig {
                log_format: Some("JSON".to_string()),
                ..Default::default()
            }),
            tracing_mode: Some("Active".to_string()),
            vpc_config: Some(FunctionVpcConfig {
                subnet_ids: vec!["subnet-1".to_string()],
                security_group_ids: vec!["sg-1".to_string()],
                ipv6_allowed_for_dual_stack: true,
                vpc_id: Some("vpc-1".to_string()),
            }),
            ..Default::default()
        };
        let function = |name: &str, settings: FunctionSettings| LambdaFunctionView {
            function_name: name.to_string(),
            function_arn: format!("arn:aws:lambda:us-east-1:123456789012:function:{name}"),
            runtime: "python3.12".to_string(),
            handler: "index.handler".to_string(),
            role: "arn:aws:iam::123456789012:role/r".to_string(),
            code_sha256: "abc".to_string(),
            code_size: 10,
            memory_size: 128,
            timeout: 3,
            environment: HashMap::new(),
            description: String::new(),
            last_modified: "2026-01-01T00:00:00+00:00".to_string(),
            state: "Active".to_string(),
            version: "$LATEST".to_string(),
            revision_id: "r-1".to_string(),
            tags: HashMap::new(),
            versions: vec![FunctionVersionView {
                version: "1".to_string(),
                description: String::new(),
                code_sha256: "abc".to_string(),
                code_size: 10,
                created_at: "2026-01-01T00:00:00+00:00".to_string(),
                settings: settings.clone(),
                config: None,
            }],
            reserved_concurrent_executions: None,
            code_signing_config_arn: None,
            settings,
        };

        let service = LambdaService::new();
        let mut view = LambdaStateView::default();
        view.functions
            .insert("f".to_string(), function("f", settings.clone()));
        view.functions.insert(
            "bare".to_string(),
            function("bare", FunctionSettings::default()),
        );
        service
            .restore("123456789012", "us-east-1", view)
            .await
            .unwrap();

        let json = serde_json::to_value(service.snapshot("123456789012", "us-east-1").await)
            .expect("snapshot must serialize");
        assert_eq!(
            json["functions"]["bare"]["settings"],
            serde_json::json!({}),
            "unset members are not written"
        );
        assert!(json["functions"]["f"].get("vpc_config").is_none());
        let back: LambdaStateView = serde_json::from_value(json).unwrap();
        assert_eq!(back.functions["f"].settings, settings);
        assert_eq!(back.functions["f"].versions[0].settings, settings);
    }

    /// Stored deployment packages and the configuration frozen with a
    /// published version survive a snapshot -> JSON -> restore round trip
    /// through the blob export, and a digest whose archive is missing is
    /// dropped.
    #[tokio::test]
    async fn code_packages_round_trip_through_blob_snapshot() {
        use base64::Engine as _;
        use sha2::{Digest, Sha256};
        use winterbaume_core::VfsError;

        struct Collect(HashMap<String, Vec<u8>>);
        impl BlobVisitor for Collect {
            fn visit(
                &mut self,
                batch: Vec<BlobExportEntry>,
            ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), VfsError>> + Send + '_>>
            {
                Box::pin(async move {
                    for mut entry in batch {
                        let mut bytes = Vec::new();
                        tokio::io::AsyncReadExt::read_to_end(&mut entry.reader, &mut bytes)
                            .await
                            .unwrap();
                        self.0.insert(entry.key, bytes);
                    }
                    Ok(())
                })
            }
        }
        impl BlobSource for Collect {
            fn fetch(
                &mut self,
                key: String,
            ) -> std::pin::Pin<
                Box<
                    dyn Future<
                            Output = Result<Box<dyn tokio::io::AsyncRead + Send + Unpin>, VfsError>,
                        > + Send
                        + '_,
                >,
            > {
                Box::pin(async move {
                    let bytes = self.0.get(&key).cloned().unwrap_or_default();
                    Ok(Box::new(std::io::Cursor::new(bytes))
                        as Box<dyn tokio::io::AsyncRead + Send + Unpin>)
                })
            }
        }

        let service = LambdaService::new();
        let zip = bytes::Bytes::from_static(b"not really a zip");
        let sha256 = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(&zip));
        service
            .blobs
            .get("123456789012", "us-east-1")
            .put(&code_blob_key(&sha256), zip.clone())
            .await
            .unwrap();
        {
            let state = service.state.get("123456789012", "us-east-1");
            let mut guard = state.write().await;
            guard
                .create_function(
                    "f",
                    "nodejs22.x",
                    "index.handler",
                    "arn:aws:iam::123456789012:role/r",
                    "",
                    None,
                    Some(7),
                    HashMap::from([("K".to_string(), "v".to_string())]),
                    "123456789012",
                    "us-east-1",
                    HashMap::new(),
                    &sha256,
                    zip.len() as i64,
                )
                .unwrap();
            guard.put_code_package(&sha256);
            guard.publish_version("f", None, None).unwrap();
        }

        let mut blobs = Collect(HashMap::new());
        let view = service
            .snapshot_with_blobs("123456789012", "us-east-1", &mut blobs)
            .await
            .unwrap();
        assert_eq!(blobs.0[&code_blob_key(&sha256)], zip.to_vec());
        let mut json = serde_json::to_value(&view).expect("snapshot must serialize");
        assert_eq!(json["code_packages"], serde_json::json!([sha256]));
        json["code_packages"] = serde_json::json!([sha256, "bWlzc2luZw=="]);

        let restored = LambdaService::new();
        restored
            .restore_with_blobs(
                "123456789012",
                "us-east-1",
                serde_json::from_value(json).unwrap(),
                &mut blobs,
            )
            .await
            .unwrap();
        // The missing digest was fetched as an empty blob by this source;
        // a plain `restore` of a view without its blobs drops it instead.
        let plain = LambdaService::new();
        plain
            .restore("123456789012", "us-east-1", view.clone())
            .await
            .unwrap();
        assert!(
            plain
                .state
                .get("123456789012", "us-east-1")
                .read()
                .await
                .code_packages
                .is_empty()
        );

        let state = restored.state.get("123456789012", "us-east-1");
        let guard = state.read().await;
        assert!(guard.code_packages.contains(&sha256));
        let target = guard.execution_target("f", "1").expect("version 1 exists");
        assert!(target.has_code);
        assert_eq!(target.timeout, 7);
        assert_eq!(target.environment["K"], "v");
        assert_eq!(
            restored
                .blobs
                .get("123456789012", "us-east-1")
                .get(&code_blob_key(&sha256))
                .await
                .unwrap(),
            zip
        );
    }

    /// A snapshot written before code was stored has neither `code_packages`
    /// nor a per-version `config`; it must still load.
    #[test]
    fn legacy_snapshot_without_code_still_deserializes() {
        let view: LambdaStateView = serde_json::from_value(serde_json::json!({
            "functions": {},
        }))
        .expect("legacy snapshot must deserialize");
        assert!(view.code_packages.is_empty());

        let version: FunctionVersionView = serde_json::from_value(serde_json::json!({
            "version": "1",
            "description": "",
            "code_sha256": "abc",
            "code_size": 1,
            "created_at": "2026-01-01T00:00:00+00:00"
        }))
        .expect("legacy version must deserialize");
        assert_eq!(version.config, None);
    }
}
