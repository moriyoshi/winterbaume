//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codebuild

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteBuildsInput {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteBuildsOutput {
    #[serde(rename = "buildsDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_deleted: Option<Vec<String>>,
    #[serde(rename = "buildsNotDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_not_deleted: Option<Vec<BuildNotDeleted>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildNotDeleted {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBuildBatchesInput {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBuildBatchesOutput {
    #[serde(rename = "buildBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batches: Option<Vec<BuildBatch>>,
    #[serde(rename = "buildBatchesNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batches_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildBatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<BuildArtifacts>,
    #[serde(rename = "buildBatchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,
    #[serde(rename = "buildBatchNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_number: Option<i64>,
    #[serde(rename = "buildBatchStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_status: Option<String>,
    #[serde(rename = "buildGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_groups: Option<Vec<BuildGroup>>,
    #[serde(rename = "buildTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timeout_in_minutes: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(rename = "currentPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
    #[serde(rename = "debugSessionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_session_enabled: Option<bool>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogsConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<BuildBatchPhase>>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "reportArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arns: Option<Vec<String>>,
    #[serde(rename = "resolvedSourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_source_version: Option<String>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<BuildArtifacts>>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildArtifacts {
    #[serde(rename = "artifactIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_identifier: Option<String>,
    #[serde(rename = "bucketOwnerAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner_access: Option<String>,
    #[serde(rename = "encryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
    #[serde(rename = "overrideArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_artifact_name: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256sum: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectBuildBatchConfig {
    #[serde(rename = "batchReportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_report_mode: Option<String>,
    #[serde(rename = "combineArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_artifacts: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BatchRestrictions>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "timeoutInMins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_mins: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRestrictions {
    #[serde(rename = "computeTypesAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_types_allowed: Option<Vec<String>>,
    #[serde(rename = "fleetsAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets_allowed: Option<Vec<String>>,
    #[serde(rename = "maximumBuildsAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_builds_allowed: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildGroup {
    #[serde(rename = "currentBuildSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_build_summary: Option<BuildSummary>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "ignoreFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(rename = "priorBuildSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_build_summary_list: Option<Vec<BuildSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "buildStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
    #[serde(rename = "primaryArtifact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_artifact: Option<ResolvedArtifact>,
    #[serde(rename = "requestedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_on: Option<f64>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ResolvedArtifact>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedArtifact {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectCache {
    #[serde(rename = "cacheNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectEnvironment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "computeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_configuration: Option<ComputeConfiguration>,
    #[serde(rename = "computeType")]
    #[serde(default)]
    pub compute_type: String,
    #[serde(rename = "dockerServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_server: Option<DockerServer>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<ProjectFleet>,
    #[serde(default)]
    pub image: String,
    #[serde(rename = "imagePullCredentialsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type: Option<String>,
    #[serde(rename = "privilegedMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode: Option<bool>,
    #[serde(rename = "registryCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<RegistryCredential>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "machineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "vCpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_cpu: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DockerServer {
    #[serde(rename = "computeType")]
    #[serde(default)]
    pub compute_type: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DockerServerStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DockerServerStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentVariable {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectFleet {
    #[serde(rename = "fleetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistryCredential {
    #[serde(default)]
    pub credential: String,
    #[serde(rename = "credentialProvider")]
    #[serde(default)]
    pub credential_provider: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectFileSystemLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "mountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
    #[serde(rename = "mountPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogsConfig {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3LogsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsConfig {
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "streamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogsConfig {
    #[serde(rename = "bucketOwnerAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner_access: Option<String>,
    #[serde(rename = "encryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildBatchPhase {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<PhaseContext>>,
    #[serde(rename = "durationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "phaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_status: Option<String>,
    #[serde(rename = "phaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhaseContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectSourceVersion {
    #[serde(rename = "sourceIdentifier")]
    #[serde(default)]
    pub source_identifier: String,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    pub source_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectSource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<SourceAuth>,
    #[serde(rename = "buildStatusConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status_config: Option<BuildStatusConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec: Option<String>,
    #[serde(rename = "gitCloneDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i32>,
    #[serde(rename = "gitSubmodulesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_submodules_config: Option<GitSubmodulesConfig>,
    #[serde(rename = "insecureSsl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "reportBuildStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_build_status: Option<bool>,
    #[serde(rename = "sourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceAuth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildStatusConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "targetUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitSubmodulesConfig {
    #[serde(rename = "fetchSubmodules")]
    #[serde(default)]
    pub fetch_submodules: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBuildsInput {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBuildsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<Build>>,
    #[serde(rename = "buildsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Build {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<BuildArtifacts>,
    #[serde(rename = "autoRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_config: Option<AutoRetryConfig>,
    #[serde(rename = "buildBatchArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_arn: Option<String>,
    #[serde(rename = "buildComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_complete: Option<bool>,
    #[serde(rename = "buildNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i64>,
    #[serde(rename = "buildStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    #[serde(rename = "currentPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
    #[serde(rename = "debugSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_session: Option<DebugSession>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    #[serde(rename = "exportedEnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported_environment_variables: Option<Vec<ExportedEnvironmentVariable>>,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsLocation>,
    #[serde(rename = "networkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<BuildPhase>>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "reportArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arns: Option<Vec<String>>,
    #[serde(rename = "resolvedSourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_source_version: Option<String>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<BuildArtifacts>>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoRetryConfig {
    #[serde(rename = "autoRetryLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_limit: Option<i32>,
    #[serde(rename = "autoRetryNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_number: Option<i32>,
    #[serde(rename = "nextAutoRetry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_auto_retry: Option<String>,
    #[serde(rename = "previousAutoRetry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_auto_retry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DebugSession {
    #[serde(rename = "sessionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_enabled: Option<bool>,
    #[serde(rename = "sessionTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportedEnvironmentVariable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogsLocation {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,
    #[serde(rename = "cloudWatchLogsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_arn: Option<String>,
    #[serde(rename = "deepLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "s3DeepLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_deep_link: Option<String>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3LogsConfig>,
    #[serde(rename = "s3LogsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs_arn: Option<String>,
    #[serde(rename = "streamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildPhase {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<PhaseContext>>,
    #[serde(rename = "durationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "phaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_status: Option<String>,
    #[serde(rename = "phaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCommandExecutionsInput {
    #[serde(rename = "commandExecutionIds")]
    #[serde(default)]
    pub command_execution_ids: Vec<String>,
    #[serde(rename = "sandboxId")]
    #[serde(default)]
    pub sandbox_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCommandExecutionsOutput {
    #[serde(rename = "commandExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_executions: Option<Vec<CommandExecution>>,
    #[serde(rename = "commandExecutionsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_executions_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandExecution {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsLocation>,
    #[serde(rename = "sandboxArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_arn: Option<String>,
    #[serde(rename = "sandboxId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "standardErrContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_err_content: Option<String>,
    #[serde(rename = "standardOutputContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_content: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFleetsInput {
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFleetsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets: Option<Vec<Fleet>>,
    #[serde(rename = "fleetsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Fleet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "baseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_capacity: Option<i32>,
    #[serde(rename = "computeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_configuration: Option<ComputeConfiguration>,
    #[serde(rename = "computeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "environmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<String>,
    #[serde(rename = "fleetServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "overflowBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_behavior: Option<String>,
    #[serde(rename = "proxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "scalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfigurationOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FleetStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfiguration {
    #[serde(rename = "defaultBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_behavior: Option<String>,
    #[serde(rename = "orderedProxyRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_proxy_rules: Option<Vec<FleetProxyRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FleetProxyRule {
    #[serde(default)]
    pub effect: String,
    #[serde(default)]
    pub entities: Vec<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingConfigurationOutput {
    #[serde(rename = "desiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "maxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "scalingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
    #[serde(rename = "targetTrackingScalingConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_configs: Option<Vec<TargetTrackingScalingConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingScalingConfiguration {
    #[serde(rename = "metricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "targetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FleetStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetProjectsInput {
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetProjectsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
    #[serde(rename = "projectsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Project {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ProjectArtifacts>,
    #[serde(rename = "autoRetryLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<ProjectBadge>,
    #[serde(rename = "buildBatchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    #[serde(rename = "concurrentBuildLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_build_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "logsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_visibility: Option<String>,
    #[serde(rename = "publicProjectAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_project_alias: Option<String>,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "resourceAccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_role: Option<String>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectArtifacts {
    #[serde(rename = "artifactIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_identifier: Option<String>,
    #[serde(rename = "bucketOwnerAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner_access: Option<String>,
    #[serde(rename = "encryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<String>,
    #[serde(rename = "overrideArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_artifact_name: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectBadge {
    #[serde(rename = "badgeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    #[serde(rename = "badgeRequestUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_request_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Webhook {
    #[serde(rename = "branchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(rename = "buildType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_type: Option<String>,
    #[serde(rename = "filterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    #[serde(rename = "lastModifiedSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_secret: Option<f64>,
    #[serde(rename = "manualCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_creation: Option<bool>,
    #[serde(rename = "payloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    #[serde(rename = "pullRequestBuildPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_build_policy: Option<PullRequestBuildPolicy>,
    #[serde(rename = "scopeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_configuration: Option<ScopeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebhookFilter {
    #[serde(rename = "excludeMatchedPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_matched_pattern: Option<bool>,
    #[serde(default)]
    pub pattern: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullRequestBuildPolicy {
    #[serde(rename = "approverRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_roles: Option<Vec<String>>,
    #[serde(rename = "requiresCommentApproval")]
    #[serde(default)]
    pub requires_comment_approval: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScopeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetReportGroupsInput {
    #[serde(rename = "reportGroupArns")]
    #[serde(default)]
    pub report_group_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetReportGroupsOutput {
    #[serde(rename = "reportGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<ReportGroup>>,
    #[serde(rename = "reportGroupsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "exportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportExportConfig {
    #[serde(rename = "exportConfigType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config_type: Option<String>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3ReportExportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReportExportConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "bucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner: Option<String>,
    #[serde(rename = "encryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetReportsInput {
    #[serde(rename = "reportArns")]
    #[serde(default)]
    pub report_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetReportsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<Report>>,
    #[serde(rename = "reportsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Report {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "codeCoverageSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_coverage_summary: Option<CodeCoverageReportSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<f64>,
    #[serde(rename = "exportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "reportGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "testSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<TestReportSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeCoverageReportSummary {
    #[serde(rename = "branchCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_coverage_percentage: Option<f64>,
    #[serde(rename = "branchesCovered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_covered: Option<i32>,
    #[serde(rename = "branchesMissed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_missed: Option<i32>,
    #[serde(rename = "lineCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_coverage_percentage: Option<f64>,
    #[serde(rename = "linesCovered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_covered: Option<i32>,
    #[serde(rename = "linesMissed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_missed: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestReportSummary {
    #[serde(rename = "durationInNanoSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_nano_seconds: Option<i64>,
    #[serde(rename = "statusCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSandboxesInput {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSandboxesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandboxes: Option<Vec<Sandbox>>,
    #[serde(rename = "sandboxesNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandboxes_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sandbox {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "currentSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_session: Option<SandboxSession>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogsConfig>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "requestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<f64>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SandboxSession {
    #[serde(rename = "currentPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsLocation>,
    #[serde(rename = "networkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterface>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<SandboxSessionPhase>>,
    #[serde(rename = "resolvedSourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_source_version: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SandboxSessionPhase {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<PhaseContext>>,
    #[serde(rename = "durationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "phaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_status: Option<String>,
    #[serde(rename = "phaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetInput {
    #[serde(rename = "baseCapacity")]
    #[serde(default)]
    pub base_capacity: i32,
    #[serde(rename = "computeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_configuration: Option<ComputeConfiguration>,
    #[serde(rename = "computeType")]
    #[serde(default)]
    pub compute_type: String,
    #[serde(rename = "environmentType")]
    #[serde(default)]
    pub environment_type: String,
    #[serde(rename = "fleetServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_service_role: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "overflowBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_behavior: Option<String>,
    #[serde(rename = "proxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "scalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfigurationInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingConfigurationInput {
    #[serde(rename = "maxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "scalingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
    #[serde(rename = "targetTrackingScalingConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_configs: Option<Vec<TargetTrackingScalingConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectInput {
    #[serde(default)]
    pub artifacts: ProjectArtifacts,
    #[serde(rename = "autoRetryLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_limit: Option<i32>,
    #[serde(rename = "badgeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    #[serde(rename = "buildBatchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    #[serde(rename = "concurrentBuildLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_build_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    pub environment: ProjectEnvironment,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(rename = "logsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    pub service_role: String,
    #[serde(default)]
    pub source: ProjectSource,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReportGroupInput {
    #[serde(rename = "exportConfig")]
    #[serde(default)]
    pub export_config: ReportExportConfig,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReportGroupOutput {
    #[serde(rename = "reportGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group: Option<ReportGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebhookInput {
    #[serde(rename = "branchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(rename = "buildType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_type: Option<String>,
    #[serde(rename = "filterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    #[serde(rename = "manualCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_creation: Option<bool>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "pullRequestBuildPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_build_policy: Option<PullRequestBuildPolicy>,
    #[serde(rename = "scopeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_configuration: Option<ScopeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebhookOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBuildBatchInput {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBuildBatchOutput {
    #[serde(rename = "buildsDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_deleted: Option<Vec<String>>,
    #[serde(rename = "buildsNotDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_not_deleted: Option<Vec<BuildNotDeleted>>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFleetInput {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFleetOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportGroupInput {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "deleteReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reports: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportGroupOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportInput {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSourceCredentialsInput {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSourceCredentialsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookInput {
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCodeCoveragesInput {
    #[serde(rename = "maxLineCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_line_coverage_percentage: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "minLineCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_line_coverage_percentage: Option<f64>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportArn")]
    #[serde(default)]
    pub report_arn: String,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCodeCoveragesOutput {
    #[serde(rename = "codeCoverages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_coverages: Option<Vec<CodeCoverage>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeCoverage {
    #[serde(rename = "branchCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_coverage_percentage: Option<f64>,
    #[serde(rename = "branchesCovered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_covered: Option<i32>,
    #[serde(rename = "branchesMissed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_missed: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<f64>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lineCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_coverage_percentage: Option<f64>,
    #[serde(rename = "linesCovered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_covered: Option<i32>,
    #[serde(rename = "linesMissed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_missed: Option<i32>,
    #[serde(rename = "reportARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestCasesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TestCaseFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportArn")]
    #[serde(default)]
    pub report_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestCasesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_cases: Option<Vec<TestCase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCase {
    #[serde(rename = "durationInNanoSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_nano_seconds: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "reportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "testRawDataPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_raw_data_path: Option<String>,
    #[serde(rename = "testSuiteName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_suite_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportGroupTrendInput {
    #[serde(rename = "numOfReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_of_reports: Option<i32>,
    #[serde(rename = "reportGroupArn")]
    #[serde(default)]
    pub report_group_arn: String,
    #[serde(rename = "trendField")]
    #[serde(default)]
    pub trend_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportGroupTrendOutput {
    #[serde(rename = "rawData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<Vec<ReportWithRawData>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<ReportGroupTrendStats>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportWithRawData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "reportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportGroupTrendStats {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSourceCredentialsInput {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "serverType")]
    #[serde(default)]
    pub server_type: String,
    #[serde(rename = "shouldOverwrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_overwrite: Option<bool>,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSourceCredentialsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvalidateProjectCacheInput {
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvalidateProjectCacheOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildBatchesForProjectInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<BuildBatchFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildBatchFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildBatchesForProjectOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildBatchesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<BuildBatchFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildBatchesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildsForProjectInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildsForProjectOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildsInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuildsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandExecutionsForSandboxInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sandboxId")]
    #[serde(default)]
    pub sandbox_id: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandExecutionsForSandboxOutput {
    #[serde(rename = "commandExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_executions: Option<Vec<CommandExecution>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCuratedEnvironmentImagesInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCuratedEnvironmentImagesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<EnvironmentPlatform>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentPlatform {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<EnvironmentLanguage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentLanguage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<EnvironmentImage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentImage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFleetsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFleetsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProjectsInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProjectsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportGroupsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportGroupsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsForReportGroupInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportGroupArn")]
    #[serde(default)]
    pub report_group_arn: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsForReportGroupOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSandboxesForProjectInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSandboxesForProjectOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSandboxesInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSandboxesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedProjectsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedProjectsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedReportGroupsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedReportGroupsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceCredentialsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceCredentialsOutput {
    #[serde(rename = "sourceCredentialsInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_credentials_infos: Option<Vec<SourceCredentialsInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceCredentialsInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "serverType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyInput {
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyOutput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryBuildBatchInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "idempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "retryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryBuildBatchOutput {
    #[serde(rename = "buildBatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch: Option<BuildBatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryBuildInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "idempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryBuildOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBuildBatchInput {
    #[serde(rename = "artifactsOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_override: Option<ProjectArtifacts>,
    #[serde(rename = "buildBatchConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_config_override: Option<ProjectBuildBatchConfig>,
    #[serde(rename = "buildTimeoutInMinutesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timeout_in_minutes_override: Option<i32>,
    #[serde(rename = "buildspecOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec_override: Option<String>,
    #[serde(rename = "cacheOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_override: Option<ProjectCache>,
    #[serde(rename = "certificateOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_override: Option<String>,
    #[serde(rename = "computeTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_override: Option<String>,
    #[serde(rename = "debugSessionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_session_enabled: Option<bool>,
    #[serde(rename = "encryptionKeyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_override: Option<String>,
    #[serde(rename = "environmentTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type_override: Option<String>,
    #[serde(rename = "environmentVariablesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables_override: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "gitCloneDepthOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth_override: Option<i32>,
    #[serde(rename = "gitSubmodulesConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_submodules_config_override: Option<GitSubmodulesConfig>,
    #[serde(rename = "idempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "imageOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_override: Option<String>,
    #[serde(rename = "imagePullCredentialsTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type_override: Option<String>,
    #[serde(rename = "insecureSslOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl_override: Option<bool>,
    #[serde(rename = "logsConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config_override: Option<LogsConfig>,
    #[serde(rename = "privilegedModeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode_override: Option<bool>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "queuedTimeoutInMinutesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes_override: Option<i32>,
    #[serde(rename = "registryCredentialOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential_override: Option<RegistryCredential>,
    #[serde(rename = "reportBuildBatchStatusOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_build_batch_status_override: Option<bool>,
    #[serde(rename = "secondaryArtifactsOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts_override: Option<Vec<ProjectArtifacts>>,
    #[serde(rename = "secondarySourcesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_override: Option<Vec<ProjectSource>>,
    #[serde(rename = "secondarySourcesVersionOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_version_override: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "serviceRoleOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_override: Option<String>,
    #[serde(rename = "sourceAuthOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_auth_override: Option<SourceAuth>,
    #[serde(rename = "sourceLocationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_override: Option<String>,
    #[serde(rename = "sourceTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type_override: Option<String>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBuildBatchOutput {
    #[serde(rename = "buildBatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch: Option<BuildBatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBuildInput {
    #[serde(rename = "artifactsOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_override: Option<ProjectArtifacts>,
    #[serde(rename = "autoRetryLimitOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_limit_override: Option<i32>,
    #[serde(rename = "buildStatusConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status_config_override: Option<BuildStatusConfig>,
    #[serde(rename = "buildspecOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildspec_override: Option<String>,
    #[serde(rename = "cacheOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_override: Option<ProjectCache>,
    #[serde(rename = "certificateOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_override: Option<String>,
    #[serde(rename = "computeTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_override: Option<String>,
    #[serde(rename = "debugSessionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_session_enabled: Option<bool>,
    #[serde(rename = "encryptionKeyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_override: Option<String>,
    #[serde(rename = "environmentTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type_override: Option<String>,
    #[serde(rename = "environmentVariablesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables_override: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "fleetOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_override: Option<ProjectFleet>,
    #[serde(rename = "gitCloneDepthOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth_override: Option<i32>,
    #[serde(rename = "gitSubmodulesConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_submodules_config_override: Option<GitSubmodulesConfig>,
    #[serde(rename = "idempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "imageOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_override: Option<String>,
    #[serde(rename = "imagePullCredentialsTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type_override: Option<String>,
    #[serde(rename = "insecureSslOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl_override: Option<bool>,
    #[serde(rename = "logsConfigOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config_override: Option<LogsConfig>,
    #[serde(rename = "privilegedModeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode_override: Option<bool>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "queuedTimeoutInMinutesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes_override: Option<i32>,
    #[serde(rename = "registryCredentialOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential_override: Option<RegistryCredential>,
    #[serde(rename = "reportBuildStatusOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_build_status_override: Option<bool>,
    #[serde(rename = "secondaryArtifactsOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts_override: Option<Vec<ProjectArtifacts>>,
    #[serde(rename = "secondarySourcesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_override: Option<Vec<ProjectSource>>,
    #[serde(rename = "secondarySourcesVersionOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources_version_override: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "serviceRoleOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_override: Option<String>,
    #[serde(rename = "sourceAuthOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_auth_override: Option<SourceAuth>,
    #[serde(rename = "sourceLocationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_override: Option<String>,
    #[serde(rename = "sourceTypeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type_override: Option<String>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "timeoutInMinutesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes_override: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBuildOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCommandExecutionInput {
    #[serde(default)]
    pub command: String,
    #[serde(rename = "sandboxId")]
    #[serde(default)]
    pub sandbox_id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCommandExecutionOutput {
    #[serde(rename = "commandExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_execution: Option<CommandExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSandboxConnectionInput {
    #[serde(rename = "sandboxId")]
    #[serde(default)]
    pub sandbox_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSandboxConnectionOutput {
    #[serde(rename = "ssmSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_session: Option<SSMSession>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSMSession {
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "streamUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    #[serde(rename = "tokenValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSandboxInput {
    #[serde(rename = "idempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSandboxOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<Sandbox>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBuildBatchInput {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBuildBatchOutput {
    #[serde(rename = "buildBatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch: Option<BuildBatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBuildInput {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBuildOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSandboxInput {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSandboxOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<Sandbox>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFleetInput {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "baseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_capacity: Option<i32>,
    #[serde(rename = "computeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_configuration: Option<ComputeConfiguration>,
    #[serde(rename = "computeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<String>,
    #[serde(rename = "environmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<String>,
    #[serde(rename = "fleetServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_service_role: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "overflowBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_behavior: Option<String>,
    #[serde(rename = "proxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "scalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfigurationInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFleetOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Fleet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ProjectArtifacts>,
    #[serde(rename = "autoRetryLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_retry_limit: Option<i32>,
    #[serde(rename = "badgeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    #[serde(rename = "buildBatchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<ProjectCache>,
    #[serde(rename = "concurrentBuildLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_build_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ProjectEnvironment>,
    #[serde(rename = "fileSystemLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    #[serde(rename = "logsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<LogsConfig>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "queuedTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_timeout_in_minutes: Option<i32>,
    #[serde(rename = "secondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<ProjectArtifacts>>,
    #[serde(rename = "secondarySourceVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    #[serde(rename = "secondarySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sources: Option<Vec<ProjectSource>>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,
    #[serde(rename = "sourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectVisibilityInput {
    #[serde(rename = "projectArn")]
    #[serde(default)]
    pub project_arn: String,
    #[serde(rename = "projectVisibility")]
    #[serde(default)]
    pub project_visibility: String,
    #[serde(rename = "resourceAccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectVisibilityOutput {
    #[serde(rename = "projectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    #[serde(rename = "projectVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_visibility: Option<String>,
    #[serde(rename = "publicProjectAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_project_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportGroupInput {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "exportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_config: Option<ReportExportConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportGroupOutput {
    #[serde(rename = "reportGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_group: Option<ReportGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebhookInput {
    #[serde(rename = "branchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(rename = "buildType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_type: Option<String>,
    #[serde(rename = "filterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<Vec<WebhookFilter>>>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "pullRequestBuildPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_build_policy: Option<PullRequestBuildPolicy>,
    #[serde(rename = "rotateSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_secret: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebhookOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}
