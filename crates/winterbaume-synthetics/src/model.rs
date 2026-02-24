//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-synthetics

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCanaryRequest {
    #[serde(rename = "ArtifactConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<ArtifactConfigInput>,
    #[serde(rename = "ArtifactS3Location")]
    #[serde(default)]
    pub artifact_s3_location: String,
    #[serde(rename = "BrowserConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_configs: Option<Vec<BrowserConfig>>,
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: CanaryCodeInput,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "FailureRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_retention_period_in_days: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ProvisionedResourceCleanup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_resource_cleanup: Option<String>,
    #[serde(rename = "ResourcesToReplicateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_to_replicate_tags: Option<Vec<String>>,
    #[serde(rename = "RunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<CanaryRunConfigInput>,
    #[serde(rename = "RuntimeVersion")]
    #[serde(default)]
    pub runtime_version: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: CanaryScheduleInput,
    #[serde(rename = "SuccessRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_retention_period_in_days: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactConfigInput {
    #[serde(rename = "S3Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption: Option<S3EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3EncryptionConfig {
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrowserConfig {
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryCodeInput {
    #[serde(rename = "BlueprintTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_types: Option<Vec<String>>,
    #[serde(rename = "Dependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_version: Option<String>,
    #[serde(rename = "ZipFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dependency {
    #[serde(rename = "Reference")]
    #[serde(default)]
    pub reference: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryRunConfigInput {
    #[serde(rename = "ActiveTracing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tracing: Option<bool>,
    #[serde(rename = "EnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<i32>,
    #[serde(rename = "MemoryInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_in_m_b: Option<i32>,
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryScheduleInput {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "RetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_config: Option<RetryConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryConfigInput {
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    pub max_retries: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigInput {
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
pub struct CreateCanaryResponse {
    #[serde(rename = "Canary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary: Option<Canary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Canary {
    #[serde(rename = "ArtifactConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<ArtifactConfigOutput>,
    #[serde(rename = "ArtifactS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_s3_location: Option<String>,
    #[serde(rename = "BrowserConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_configs: Option<Vec<BrowserConfig>>,
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CanaryCodeOutput>,
    #[serde(rename = "DryRunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_config: Option<DryRunConfigOutput>,
    #[serde(rename = "EngineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_arn: Option<String>,
    #[serde(rename = "EngineConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_configs: Option<Vec<EngineConfig>>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "FailureRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_retention_period_in_days: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProvisionedResourceCleanup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_resource_cleanup: Option<String>,
    #[serde(rename = "RunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<CanaryRunConfigOutput>,
    #[serde(rename = "RuntimeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CanaryScheduleOutput>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CanaryStatus>,
    #[serde(rename = "SuccessRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_retention_period_in_days: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<CanaryTimeline>,
    #[serde(rename = "VisualReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_reference: Option<VisualReferenceOutput>,
    #[serde(rename = "VisualReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_references: Option<Vec<VisualReferenceOutput>>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactConfigOutput {
    #[serde(rename = "S3Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption: Option<S3EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryCodeOutput {
    #[serde(rename = "BlueprintTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_types: Option<Vec<String>>,
    #[serde(rename = "Dependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "SourceLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DryRunConfigOutput {
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "LastDryRunExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_dry_run_execution_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineConfig {
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    #[serde(rename = "EngineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryRunConfigOutput {
    #[serde(rename = "ActiveTracing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tracing: Option<bool>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<i32>,
    #[serde(rename = "MemoryInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_in_m_b: Option<i32>,
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryScheduleOutput {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "RetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_config: Option<RetryConfigOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryConfigOutput {
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryStatus {
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryTimeline {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "LastStarted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started: Option<f64>,
    #[serde(rename = "LastStopped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stopped: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualReferenceOutput {
    #[serde(rename = "BaseCanaryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_canary_run_id: Option<String>,
    #[serde(rename = "BaseScreenshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_screenshots: Option<Vec<BaseScreenshot>>,
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseScreenshot {
    #[serde(rename = "IgnoreCoordinates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_coordinates: Option<Vec<String>>,
    #[serde(rename = "ScreenshotName")]
    #[serde(default)]
    pub screenshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigOutput {
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
pub struct CreateGroupRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCanaryRequest {
    #[serde(rename = "DeleteLambda")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_lambda: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCanaryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCanariesLastRunRequest {
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCanariesLastRunResponse {
    #[serde(rename = "CanariesLastRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canaries_last_run: Option<Vec<CanaryLastRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryLastRun {
    #[serde(rename = "CanaryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_name: Option<String>,
    #[serde(rename = "LastRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<CanaryRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryRun {
    #[serde(rename = "ArtifactS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_s3_location: Option<String>,
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    #[serde(rename = "DryRunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_config: Option<CanaryDryRunConfigOutput>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetryAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempt: Option<i32>,
    #[serde(rename = "ScheduledRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_run_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CanaryRunStatus>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<CanaryRunTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryDryRunConfigOutput {
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryRunStatus {
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
    #[serde(rename = "TestResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryRunTimeline {
    #[serde(rename = "Completed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<f64>,
    #[serde(rename = "MetricTimestampForRunAndRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_timestamp_for_run_and_retries: Option<f64>,
    #[serde(rename = "Started")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCanariesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCanariesResponse {
    #[serde(rename = "Canaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canaries: Option<Vec<Canary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuntimeVersionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuntimeVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RuntimeVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_versions: Option<Vec<RuntimeVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeVersion {
    #[serde(rename = "DeprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCanaryRequest {
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCanaryResponse {
    #[serde(rename = "Canary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary: Option<Canary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCanaryRunsRequest {
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RunType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCanaryRunsResponse {
    #[serde(rename = "CanaryRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_runs: Option<Vec<CanaryRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupResourcesRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupResourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCanaryDryRunRequest {
    #[serde(rename = "ArtifactConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<ArtifactConfigInput>,
    #[serde(rename = "ArtifactS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_s3_location: Option<String>,
    #[serde(rename = "BrowserConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_configs: Option<Vec<BrowserConfig>>,
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CanaryCodeInput>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "FailureRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_retention_period_in_days: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ProvisionedResourceCleanup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_resource_cleanup: Option<String>,
    #[serde(rename = "RunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<CanaryRunConfigInput>,
    #[serde(rename = "RuntimeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[serde(rename = "SuccessRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_retention_period_in_days: Option<i32>,
    #[serde(rename = "VisualReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_reference: Option<VisualReferenceInput>,
    #[serde(rename = "VisualReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_references: Option<Vec<VisualReferenceInput>>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualReferenceInput {
    #[serde(rename = "BaseCanaryRunId")]
    #[serde(default)]
    pub base_canary_run_id: String,
    #[serde(rename = "BaseScreenshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_screenshots: Option<Vec<BaseScreenshot>>,
    #[serde(rename = "BrowserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCanaryDryRunResponse {
    #[serde(rename = "DryRunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_config: Option<DryRunConfigOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCanaryRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCanaryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCanaryRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCanaryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCanaryRequest {
    #[serde(rename = "ArtifactConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<ArtifactConfigInput>,
    #[serde(rename = "ArtifactS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_s3_location: Option<String>,
    #[serde(rename = "BrowserConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_configs: Option<Vec<BrowserConfig>>,
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CanaryCodeInput>,
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "FailureRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_retention_period_in_days: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ProvisionedResourceCleanup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_resource_cleanup: Option<String>,
    #[serde(rename = "RunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<CanaryRunConfigInput>,
    #[serde(rename = "RuntimeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CanaryScheduleInput>,
    #[serde(rename = "SuccessRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_retention_period_in_days: Option<i32>,
    #[serde(rename = "VisualReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_reference: Option<VisualReferenceInput>,
    #[serde(rename = "VisualReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_references: Option<Vec<VisualReferenceInput>>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCanaryResponse {}
