//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lambda

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddLayerVersionPermissionRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    pub statement_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddLayerVersionPermissionResponse {
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Statement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddPermissionRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "EventSourceToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionUrlAuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url_auth_type: Option<String>,
    #[serde(rename = "InvokedViaFunctionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_via_function_url: Option<bool>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "PrincipalOrgID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_org_i_d: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "SourceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddPermissionResponse {
    #[serde(rename = "Statement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasConfiguration {
    #[serde(rename = "AliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FunctionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasRoutingConfiguration {
    #[serde(rename = "AdditionalVersionWeights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_weights: Option<std::collections::HashMap<String, f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointDurableExecutionRequest {
    #[serde(rename = "CheckpointToken")]
    #[serde(default)]
    pub checkpoint_token: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    pub durable_execution_arn: String,
    #[serde(rename = "Updates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<OperationUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationUpdate {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "CallbackOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_options: Option<CallbackOptions>,
    #[serde(rename = "ChainedInvokeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_options: Option<ChainedInvokeOptions>,
    #[serde(rename = "ContextOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_options: Option<ContextOptions>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "StepOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_options: Option<StepOptions>,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "WaitOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_options: Option<WaitOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackOptions {
    #[serde(rename = "HeartbeatTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout_seconds: Option<i32>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeOptions {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextOptions {
    #[serde(rename = "ReplayChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_children: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorObject {
    #[serde(rename = "ErrorData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_data: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(rename = "StackTrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepOptions {
    #[serde(rename = "NextAttemptDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_delay_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitOptions {
    #[serde(rename = "WaitSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointDurableExecutionResponse {
    #[serde(rename = "CheckpointToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_token: Option<String>,
    #[serde(rename = "NewExecutionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_execution_state: Option<CheckpointUpdatedExecutionState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointUpdatedExecutionState {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Operation {
    #[serde(rename = "CallbackDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_details: Option<CallbackDetails>,
    #[serde(rename = "ChainedInvokeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_details: Option<ChainedInvokeDetails>,
    #[serde(rename = "ContextDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_details: Option<ContextDetails>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "ExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_details: Option<StepDetails>,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WaitDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_details: Option<WaitDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackDetails {
    #[serde(rename = "CallbackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "ReplayChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_children: Option<bool>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionDetails {
    #[serde(rename = "InputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepDetails {
    #[serde(rename = "Attempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "NextAttemptTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_timestamp: Option<f64>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitDetails {
    #[serde(rename = "ScheduledEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Concurrency {
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAliasRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionVersion")]
    #[serde(default)]
    pub function_version: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapacityProviderRequest {
    #[serde(rename = "CapacityProviderName")]
    #[serde(default)]
    pub capacity_provider_name: String,
    #[serde(rename = "CapacityProviderScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_scaling_config: Option<CapacityProviderScalingConfig>,
    #[serde(rename = "InstanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<InstanceRequirements>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "PermissionsConfig")]
    #[serde(default)]
    pub permissions_config: CapacityProviderPermissionsConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    pub vpc_config: CapacityProviderVpcConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderScalingConfig {
    #[serde(rename = "MaxVCpuCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_v_cpu_count: Option<i32>,
    #[serde(rename = "ScalingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_mode: Option<String>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<TargetTrackingScalingPolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingScalingPolicy {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceRequirements {
    #[serde(rename = "AllowedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "Architectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "ExcludedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_instance_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderPermissionsConfig {
    #[serde(rename = "CapacityProviderOperatorRoleArn")]
    #[serde(default)]
    pub capacity_provider_operator_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderVpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapacityProviderResponse {
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProvider {
    #[serde(rename = "CapacityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_arn: Option<String>,
    #[serde(rename = "CapacityProviderScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_scaling_config: Option<CapacityProviderScalingConfig>,
    #[serde(rename = "InstanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<InstanceRequirements>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "PermissionsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_config: Option<CapacityProviderPermissionsConfig>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<CapacityProviderVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSigningConfigRequest {
    #[serde(rename = "AllowedPublishers")]
    #[serde(default)]
    pub allowed_publishers: AllowedPublishers,
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedPublishers {
    #[serde(rename = "SigningProfileVersionArns")]
    #[serde(default)]
    pub signing_profile_version_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSigningPolicies {
    #[serde(rename = "UntrustedArtifactOnDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untrusted_artifact_on_deployment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config: Option<CodeSigningConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSigningConfig {
    #[serde(rename = "AllowedPublishers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_publishers: Option<AllowedPublishers>,
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<String>,
    #[serde(rename = "CodeSigningConfigId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_id: Option<String>,
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventSourceMappingRequest {
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "DocumentDBEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_d_b_event_source_config: Option<DocumentDBEventSourceConfig>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "FilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<EventSourceMappingLoggingConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "MetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<EventSourceMappingMetricsConfig>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(rename = "ProvisionedPollerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_poller_config: Option<ProvisionedPollerConfig>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    #[serde(rename = "ScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<ScalingConfig>,
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "SchemaRegistryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_registry_config: Option<KafkaSchemaRegistryConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaSchemaRegistryConfig {
    #[serde(rename = "AccessConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configs: Option<Vec<KafkaSchemaRegistryAccessConfig>>,
    #[serde(rename = "EventRecordFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_record_format: Option<String>,
    #[serde(rename = "SchemaRegistryURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_registry_u_r_i: Option<String>,
    #[serde(rename = "SchemaValidationConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_validation_configs: Option<Vec<KafkaSchemaValidationConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaSchemaRegistryAccessConfig {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "URI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaSchemaValidationConfig {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfig {
    #[serde(rename = "OnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<OnFailure>,
    #[serde(rename = "OnSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success: Option<OnSuccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnFailure {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnSuccess {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentDBEventSourceConfig {
    #[serde(rename = "CollectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "FullDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriteria {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSourceMappingLoggingConfig {
    #[serde(rename = "SystemLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSourceMappingMetricsConfig {
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedPollerConfig {
    #[serde(rename = "MaximumPollers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_pollers: Option<i32>,
    #[serde(rename = "MinimumPollers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_pollers: Option<i32>,
    #[serde(rename = "PollerGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poller_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingConfig {
    #[serde(rename = "MaximumConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_concurrency: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedEventSource {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "SchemaRegistryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_registry_config: Option<KafkaSchemaRegistryConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceAccessConfiguration {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "URI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionRequest {
    #[serde(rename = "Architectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "CapacityProviderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_config: Option<CapacityProviderConfig>,
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: FunctionCode,
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<String>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DurableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_config: Option<DurableConfig>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "ImageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "MemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(rename = "Publish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(rename = "PublishTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_to: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "SnapStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_start: Option<SnapStart>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TenancyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy_config: Option<TenancyConfig>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TracingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderConfig {
    #[serde(rename = "LambdaManagedInstancesCapacityProviderConfig")]
    #[serde(default)]
    pub lambda_managed_instances_capacity_provider_config:
        LambdaManagedInstancesCapacityProviderConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaManagedInstancesCapacityProviderConfig {
    #[serde(rename = "CapacityProviderArn")]
    #[serde(default)]
    pub capacity_provider_arn: String,
    #[serde(rename = "ExecutionEnvironmentMemoryGiBPerVCpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_environment_memory_gi_b_per_v_cpu: Option<f64>,
    #[serde(rename = "PerExecutionEnvironmentMaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_execution_environment_max_concurrency: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionCode {
    #[serde(rename = "ImageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "SourceKMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_k_m_s_key_arn: Option<String>,
    #[serde(rename = "ZipFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeadLetterConfig {
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DurableConfig {
    #[serde(rename = "ExecutionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<i32>,
    #[serde(rename = "RetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EphemeralStorage {
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "LocalMountPath")]
    #[serde(default)]
    pub local_mount_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfig {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[serde(rename = "WorkingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfig {
    #[serde(rename = "ApplicationLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_log_level: Option<String>,
    #[serde(rename = "LogFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "SystemLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapStart {
    #[serde(rename = "ApplyOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TenancyConfig {
    #[serde(rename = "TenantIsolationMode")]
    #[serde(default)]
    pub tenant_isolation_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TracingConfig {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "Ipv6AllowedForDualStack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_allowed_for_dual_stack: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionUrlConfigRequest {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cors {
    #[serde(rename = "AllowCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "AllowHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(rename = "AllowMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(rename = "AllowOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    #[serde(rename = "ExposeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionUrlConfigResponse {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url: Option<String>,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAliasRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityProviderRequest {
    #[serde(rename = "CapacityProviderName")]
    #[serde(default)]
    pub capacity_provider_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityProviderResponse {
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    pub code_signing_config_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSigningConfigResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventSourceMappingRequest {
    #[serde(rename = "UUID")]
    #[serde(default)]
    pub u_u_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionCodeSigningConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionResponse {
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLayerVersionRequest {
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSourceMappingConfiguration {
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "DocumentDBEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_d_b_event_source_config: Option<DocumentDBEventSourceConfig>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "EventSourceMappingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_mapping_arn: Option<String>,
    #[serde(rename = "FilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "FilterCriteriaError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria_error: Option<FilterCriteriaError>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "LastProcessingResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processing_result: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<EventSourceMappingLoggingConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "MetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<EventSourceMappingMetricsConfig>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(rename = "ProvisionedPollerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_poller_config: Option<ProvisionedPollerConfig>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    #[serde(rename = "ScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<ScalingConfig>,
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateTransitionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_reason: Option<String>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i32>,
    #[serde(rename = "UUID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_u_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriteriaError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionConfiguration {
    #[serde(rename = "Architectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "CapacityProviderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_config: Option<CapacityProviderConfig>,
    #[serde(rename = "CodeSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "ConfigSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_sha256: Option<String>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DurableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_config: Option<DurableConfig>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentResponse>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "ImageConfigResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config_response: Option<ImageConfigResponse>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "LastUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    #[serde(rename = "LastUpdateStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason: Option<String>,
    #[serde(rename = "LastUpdateStatusReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason_code: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "MasterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    #[serde(rename = "MemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "RuntimeVersionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version_config: Option<RuntimeVersionConfig>,
    #[serde(rename = "SigningJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
    #[serde(rename = "SnapStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_start: Option<SnapStartResponse>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "StateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_code: Option<String>,
    #[serde(rename = "TenancyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy_config: Option<TenancyConfig>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TracingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfigResponse>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EnvironmentError>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfigResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ImageConfigError>,
    #[serde(rename = "ImageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfigError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Layer {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "SigningJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeVersionConfig {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RuntimeVersionError>,
    #[serde(rename = "RuntimeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeVersionError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapStartResponse {
    #[serde(rename = "ApplyOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on: Option<String>,
    #[serde(rename = "OptimizationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TracingConfigResponse {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigResponse {
    #[serde(rename = "Ipv6AllowedForDualStack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_allowed_for_dual_stack: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionEventInvokeConfig {
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsResponse {
    #[serde(rename = "AccountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_limit: Option<AccountLimit>,
    #[serde(rename = "AccountUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_usage: Option<AccountUsage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLimit {
    #[serde(rename = "CodeSizeUnzipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_unzipped: Option<i64>,
    #[serde(rename = "CodeSizeZipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_zipped: Option<i64>,
    #[serde(rename = "ConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_executions: Option<i32>,
    #[serde(rename = "TotalCodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
    #[serde(rename = "UnreservedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreserved_concurrent_executions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountUsage {
    #[serde(rename = "FunctionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<i64>,
    #[serde(rename = "TotalCodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAliasRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityProviderRequest {
    #[serde(rename = "CapacityProviderName")]
    #[serde(default)]
    pub capacity_provider_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityProviderResponse {
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    pub code_signing_config_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config: Option<CodeSigningConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionHistoryRequest {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    pub durable_execution_arn: String,
    #[serde(rename = "IncludeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ReverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionHistoryResponse {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "CallbackFailedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_failed_details: Option<CallbackFailedDetails>,
    #[serde(rename = "CallbackStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_started_details: Option<CallbackStartedDetails>,
    #[serde(rename = "CallbackSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_succeeded_details: Option<CallbackSucceededDetails>,
    #[serde(rename = "CallbackTimedOutDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_timed_out_details: Option<CallbackTimedOutDetails>,
    #[serde(rename = "ChainedInvokeFailedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_failed_details: Option<ChainedInvokeFailedDetails>,
    #[serde(rename = "ChainedInvokeStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_started_details: Option<ChainedInvokeStartedDetails>,
    #[serde(rename = "ChainedInvokeStoppedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_stopped_details: Option<ChainedInvokeStoppedDetails>,
    #[serde(rename = "ChainedInvokeSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_succeeded_details: Option<ChainedInvokeSucceededDetails>,
    #[serde(rename = "ChainedInvokeTimedOutDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chained_invoke_timed_out_details: Option<ChainedInvokeTimedOutDetails>,
    #[serde(rename = "ContextFailedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_failed_details: Option<ContextFailedDetails>,
    #[serde(rename = "ContextStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_started_details: Option<ContextStartedDetails>,
    #[serde(rename = "ContextSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_succeeded_details: Option<ContextSucceededDetails>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i32>,
    #[serde(rename = "EventTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<f64>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "ExecutionFailedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_failed_details: Option<ExecutionFailedDetails>,
    #[serde(rename = "ExecutionStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_started_details: Option<ExecutionStartedDetails>,
    #[serde(rename = "ExecutionStoppedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_details: Option<ExecutionStoppedDetails>,
    #[serde(rename = "ExecutionSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_succeeded_details: Option<ExecutionSucceededDetails>,
    #[serde(rename = "ExecutionTimedOutDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timed_out_details: Option<ExecutionTimedOutDetails>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InvocationCompletedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_completed_details: Option<InvocationCompletedDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "StepFailedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_failed_details: Option<StepFailedDetails>,
    #[serde(rename = "StepStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_started_details: Option<StepStartedDetails>,
    #[serde(rename = "StepSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_succeeded_details: Option<StepSucceededDetails>,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "WaitCancelledDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_cancelled_details: Option<WaitCancelledDetails>,
    #[serde(rename = "WaitStartedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_started_details: Option<WaitStartedDetails>,
    #[serde(rename = "WaitSucceededDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_succeeded_details: Option<WaitSucceededDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackFailedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventError {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<ErrorObject>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackStartedDetails {
    #[serde(rename = "CallbackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<i32>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackSucceededDetails {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<EventResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventResult {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallbackTimedOutDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeFailedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeStartedDetails {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_arn: Option<String>,
    #[serde(rename = "ExecutedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_version: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<EventInput>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventInput {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeStoppedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeSucceededDetails {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<EventResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChainedInvokeTimedOutDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextFailedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextStartedDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextSucceededDetails {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<EventResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionFailedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStartedDetails {
    #[serde(rename = "ExecutionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<i32>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<EventInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStoppedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionSucceededDetails {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<EventResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionTimedOutDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvocationCompletedDetails {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepFailedDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
    #[serde(rename = "RetryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_details: Option<RetryDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryDetails {
    #[serde(rename = "CurrentAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_attempt: Option<i32>,
    #[serde(rename = "NextAttemptDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_delay_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepStartedDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepSucceededDetails {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<EventResult>,
    #[serde(rename = "RetryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_details: Option<RetryDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitCancelledDetails {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EventError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitStartedDetails {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "ScheduledEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitSucceededDetails {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionRequest {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    pub durable_execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionResponse {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_arn: Option<String>,
    #[serde(rename = "DurableExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_name: Option<String>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "InputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TraceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<TraceHeader>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TraceHeader {
    #[serde(rename = "XAmznTraceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_amzn_trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionStateRequest {
    #[serde(rename = "CheckpointToken")]
    #[serde(default)]
    pub checkpoint_token: String,
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    pub durable_execution_arn: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDurableExecutionStateResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventSourceMappingRequest {
    #[serde(rename = "UUID")]
    #[serde(default)]
    pub u_u_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionCodeSigningConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionConcurrencyResponse {
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionConfigurationRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionRecursionConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionRecursionConfigResponse {
    #[serde(rename = "RecursiveLoop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_loop: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionResponse {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FunctionCodeLocation>,
    #[serde(rename = "Concurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<FunctionConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TagsError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_error: Option<TagsError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionCodeLocation {
    #[serde(rename = "ImageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "RepositoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    #[serde(rename = "ResolvedImageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_image_uri: Option<String>,
    #[serde(rename = "SourceKMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_k_m_s_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagsError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionScalingConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionScalingConfigResponse {
    #[serde(rename = "AppliedFunctionScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_function_scaling_config: Option<FunctionScalingConfig>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "RequestedFunctionScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_function_scaling_config: Option<FunctionScalingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionScalingConfig {
    #[serde(rename = "MaxExecutionEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_execution_environments: Option<i32>,
    #[serde(rename = "MinExecutionEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_execution_environments: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionUrlConfigResponse {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url: Option<String>,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLayerVersionByArnRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLayerVersionPolicyRequest {
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLayerVersionPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLayerVersionRequest {
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLayerVersionResponse {
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<Vec<String>>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    #[serde(rename = "LayerVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    #[serde(rename = "LicenseInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerVersionContentOutput {
    #[serde(rename = "CodeSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "SigningJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedConcurrencyConfigResponse {
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuntimeManagementConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuntimeManagementConfigResponse {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "RuntimeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version_arn: Option<String>,
    #[serde(rename = "UpdateRuntimeOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_runtime_on: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvocationRequest {
    #[serde(rename = "ClientContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    #[serde(rename = "DurableExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_name: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "InvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvocationResponse {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_arn: Option<String>,
    #[serde(rename = "ExecutedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_version: Option<String>,
    #[serde(rename = "FunctionError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_error: Option<String>,
    #[serde(rename = "LogResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_result: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeAsyncRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "InvokeArgs")]
    #[serde(default)]
    pub invoke_args: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeAsyncResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeWithResponseStreamRequest {
    #[serde(rename = "ClientContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "InvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeWithResponseStreamResponse {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<InvokeWithResponseStreamResponseEvent>,
    #[serde(rename = "ExecutedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_version: Option<String>,
    #[serde(rename = "ResponseStreamContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_stream_content_type: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeWithResponseStreamResponseEvent {
    #[serde(rename = "InvokeComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_complete: Option<InvokeWithResponseStreamCompleteEvent>,
    #[serde(rename = "PayloadChunk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_chunk: Option<InvokeResponseStreamUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeWithResponseStreamCompleteEvent {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[serde(rename = "LogResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeResponseStreamUpdate {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAliasesRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAliasesResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasConfiguration>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityProvidersRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityProvidersResponse {
    #[serde(rename = "CapacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<CapacityProvider>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSigningConfigsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSigningConfigsResponse {
    #[serde(rename = "CodeSigningConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_configs: Option<Vec<CodeSigningConfig>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDurableExecutionsByFunctionRequest {
    #[serde(rename = "DurableExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_name: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "ReverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "StartedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    #[serde(rename = "StartedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
    #[serde(rename = "Statuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDurableExecutionsByFunctionResponse {
    #[serde(rename = "DurableExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_executions: Option<Vec<Execution>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Execution {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_arn: Option<String>,
    #[serde(rename = "DurableExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_execution_name: Option<String>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventSourceMappingsRequest {
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventSourceMappingsResponse {
    #[serde(rename = "EventSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_mappings: Option<Vec<EventSourceMappingConfiguration>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionEventInvokeConfigsRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionEventInvokeConfigsResponse {
    #[serde(rename = "FunctionEventInvokeConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_event_invoke_configs: Option<Vec<FunctionEventInvokeConfig>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionUrlConfigsRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionUrlConfigsResponse {
    #[serde(rename = "FunctionUrlConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url_configs: Option<Vec<FunctionUrlConfig>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionUrlConfig {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url: Option<String>,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionVersionsByCapacityProviderRequest {
    #[serde(rename = "CapacityProviderName")]
    #[serde(default)]
    pub capacity_provider_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionVersionsByCapacityProviderResponse {
    #[serde(rename = "CapacityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_arn: Option<String>,
    #[serde(rename = "FunctionVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_versions: Option<Vec<FunctionVersionsByCapacityProviderListItem>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionVersionsByCapacityProviderListItem {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsByCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    pub code_signing_config_arn: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsByCodeSigningConfigResponse {
    #[serde(rename = "FunctionArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arns: Option<Vec<String>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsRequest {
    #[serde(rename = "FunctionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MasterRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_region: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsResponse {
    #[serde(rename = "Functions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<FunctionConfiguration>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLayerVersionsRequest {
    #[serde(rename = "CompatibleArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architecture: Option<String>,
    #[serde(rename = "CompatibleRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<String>,
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLayerVersionsResponse {
    #[serde(rename = "LayerVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_versions: Option<Vec<LayerVersionsListItem>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerVersionsListItem {
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<Vec<String>>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LayerVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    #[serde(rename = "LicenseInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLayersRequest {
    #[serde(rename = "CompatibleArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architecture: Option<String>,
    #[serde(rename = "CompatibleRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLayersResponse {
    #[serde(rename = "Layers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<LayersListItem>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayersListItem {
    #[serde(rename = "LatestMatchingVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_matching_version: Option<LayerVersionsListItem>,
    #[serde(rename = "LayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    #[serde(rename = "LayerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedConcurrencyConfigsRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedConcurrencyConfigsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "ProvisionedConcurrencyConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_concurrency_configs: Option<Vec<ProvisionedConcurrencyConfigListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedConcurrencyConfigListItem {
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVersionsByFunctionRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVersionsByFunctionResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<FunctionConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishLayerVersionRequest {
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<Vec<String>>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: LayerVersionContentInput,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "LicenseInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerVersionContentInput {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "ZipFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishLayerVersionResponse {
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<Vec<String>>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    #[serde(rename = "LayerVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    #[serde(rename = "LicenseInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishVersionRequest {
    #[serde(rename = "CodeSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "PublishTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_to: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    pub code_signing_config_arn: String,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(default)]
    pub reserved_concurrent_executions: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionEventInvokeConfigRequest {
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionRecursionConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "RecursiveLoop")]
    #[serde(default)]
    pub recursive_loop: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionRecursionConfigResponse {
    #[serde(rename = "RecursiveLoop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_loop: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionScalingConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_scaling_config: Option<FunctionScalingConfig>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFunctionScalingConfigResponse {
    #[serde(rename = "FunctionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    #[serde(default)]
    pub provisioned_concurrent_executions: i32,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutProvisionedConcurrencyConfigResponse {
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuntimeManagementConfigRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "RuntimeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version_arn: Option<String>,
    #[serde(rename = "UpdateRuntimeOn")]
    #[serde(default)]
    pub update_runtime_on: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuntimeManagementConfigResponse {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "RuntimeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version_arn: Option<String>,
    #[serde(rename = "UpdateRuntimeOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_runtime_on: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveLayerVersionPermissionRequest {
    #[serde(rename = "LayerName")]
    #[serde(default)]
    pub layer_name: String,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    pub statement_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemovePermissionRequest {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackFailureRequest {
    #[serde(rename = "CallbackId")]
    #[serde(default)]
    pub callback_id: String,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackFailureResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackHeartbeatRequest {
    #[serde(rename = "CallbackId")]
    #[serde(default)]
    pub callback_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackHeartbeatResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackSuccessRequest {
    #[serde(rename = "CallbackId")]
    #[serde(default)]
    pub callback_id: String,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDurableExecutionCallbackSuccessResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDurableExecutionRequest {
    #[serde(rename = "DurableExecutionArn")]
    #[serde(default)]
    pub durable_execution_arn: String,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDurableExecutionResponse {
    #[serde(rename = "StopTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAliasRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "FunctionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapacityProviderRequest {
    #[serde(rename = "CapacityProviderName")]
    #[serde(default)]
    pub capacity_provider_name: String,
    #[serde(rename = "CapacityProviderScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_scaling_config: Option<CapacityProviderScalingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapacityProviderResponse {
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSigningConfigRequest {
    #[serde(rename = "AllowedPublishers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_publishers: Option<AllowedPublishers>,
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(default)]
    pub code_signing_config_arn: String,
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config: Option<CodeSigningConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventSourceMappingRequest {
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "DocumentDBEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_d_b_event_source_config: Option<DocumentDBEventSourceConfig>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "FilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<EventSourceMappingLoggingConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "MetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<EventSourceMappingMetricsConfig>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(rename = "ProvisionedPollerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_poller_config: Option<ProvisionedPollerConfig>,
    #[serde(rename = "ScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<ScalingConfig>,
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i32>,
    #[serde(rename = "UUID")]
    #[serde(default)]
    pub u_u_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionCodeRequest {
    #[serde(rename = "Architectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "ImageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "Publish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(rename = "PublishTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_to: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "SourceKMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_k_m_s_key_arn: Option<String>,
    #[serde(rename = "ZipFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionConfigurationRequest {
    #[serde(rename = "CapacityProviderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_config: Option<CapacityProviderConfig>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DurableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_config: Option<DurableConfig>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "ImageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "MemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "SnapStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_start: Option<SnapStart>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TracingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionEventInvokeConfigRequest {
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionUrlConfigRequest {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionUrlConfigResponse {
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "Cors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url: Option<String>,
    #[serde(rename = "InvokeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}
