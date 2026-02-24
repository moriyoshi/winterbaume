//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emrserverless

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRunRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    pub job_run_id: String,
    #[serde(rename = "shutdownGracePeriodInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_grace_period_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRunResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "autoStartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start_configuration: Option<AutoStartConfig>,
    #[serde(rename = "autoStopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_stop_configuration: Option<AutoStopConfig>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "diskEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    #[serde(rename = "identityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfigurationInput>,
    #[serde(rename = "imageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfigurationInput>,
    #[serde(rename = "initialCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_capacity: Option<std::collections::HashMap<String, InitialCapacityConfig>>,
    #[serde(rename = "interactiveConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_configuration: Option<InteractiveConfiguration>,
    #[serde(rename = "jobLevelCostAllocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_cost_allocation_configuration: Option<JobLevelCostAllocationConfiguration>,
    #[serde(rename = "maximumCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_capacity: Option<MaximumAllowedResources>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    pub release_label: String,
    #[serde(rename = "runtimeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "schedulerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_configuration: Option<SchedulerConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "workerTypeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type_specifications:
        Option<std::collections::HashMap<String, WorkerTypeSpecificationInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoStartConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoStopConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "idleTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiskEncryptionConfiguration {
    #[serde(rename = "encryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterConfigurationInput {
    #[serde(rename = "identityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
    #[serde(rename = "userBackgroundSessionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_sessions_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfigurationInput {
    #[serde(rename = "imageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitialCapacityConfig {
    #[serde(rename = "workerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_configuration: Option<WorkerResourceConfig>,
    #[serde(rename = "workerCount")]
    #[serde(default)]
    pub worker_count: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkerResourceConfig {
    #[serde(default)]
    pub cpu: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<String>,
    #[serde(rename = "diskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[serde(default)]
    pub memory: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InteractiveConfiguration {
    #[serde(rename = "livyEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livy_endpoint_enabled: Option<bool>,
    #[serde(rename = "sessionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_enabled: Option<bool>,
    #[serde(rename = "studioEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobLevelCostAllocationConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaximumAllowedResources {
    #[serde(default)]
    pub cpu: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<String>,
    #[serde(default)]
    pub memory: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfiguration {
    #[serde(rename = "cloudWatchLoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_configuration: Option<CloudWatchLoggingConfiguration>,
    #[serde(rename = "managedPersistenceMonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_persistence_monitoring_configuration:
        Option<ManagedPersistenceMonitoringConfiguration>,
    #[serde(rename = "prometheusMonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_monitoring_configuration: Option<PrometheusMonitoringConfiguration>,
    #[serde(rename = "s3MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_monitoring_configuration: Option<S3MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingConfiguration {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "logTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedPersistenceMonitoringConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrometheusMonitoringConfiguration {
    #[serde(rename = "remoteWriteUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_write_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3MonitoringConfiguration {
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "logUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default)]
    pub classification: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulerConfiguration {
    #[serde(rename = "maxConcurrentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
    #[serde(rename = "queueTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_timeout_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkerTypeSpecificationInput {
    #[serde(rename = "imageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfigurationInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "autoStartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start_configuration: Option<AutoStartConfig>,
    #[serde(rename = "autoStopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_stop_configuration: Option<AutoStopConfig>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "diskEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    #[serde(rename = "identityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfiguration>,
    #[serde(rename = "imageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfiguration>,
    #[serde(rename = "initialCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_capacity: Option<std::collections::HashMap<String, InitialCapacityConfig>>,
    #[serde(rename = "interactiveConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_configuration: Option<InteractiveConfiguration>,
    #[serde(rename = "jobLevelCostAllocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_cost_allocation_configuration: Option<JobLevelCostAllocationConfiguration>,
    #[serde(rename = "maximumCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_capacity: Option<MaximumAllowedResources>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "runtimeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "schedulerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_configuration: Option<SchedulerConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "workerTypeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type_specifications:
        Option<std::collections::HashMap<String, WorkerTypeSpecification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterConfiguration {
    #[serde(rename = "identityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "identityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
    #[serde(rename = "userBackgroundSessionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_sessions_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfiguration {
    #[serde(rename = "imageUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "resolvedImageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_image_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkerTypeSpecification {
    #[serde(rename = "imageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardForJobRunRequest {
    #[serde(rename = "accessSystemProfileLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_system_profile_logs: Option<bool>,
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    pub job_run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardForJobRunResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRunRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    pub job_run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRunResponse {
    #[serde(rename = "jobRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRun {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "attemptCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_created_at: Option<f64>,
    #[serde(rename = "attemptUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_updated_at: Option<f64>,
    #[serde(rename = "billedResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_resource_utilization: Option<ResourceUtilization>,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "executionIamPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_iam_policy: Option<JobRunExecutionIamPolicy>,
    #[serde(rename = "executionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "executionTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_minutes: Option<i64>,
    #[serde(rename = "jobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_driver: Option<JobDriver>,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "queuedDurationMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_duration_milliseconds: Option<i64>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "retryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "totalExecutionDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_duration_seconds: Option<i32>,
    #[serde(rename = "totalResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resource_utilization: Option<TotalResourceUtilization>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceUtilization {
    #[serde(rename = "memoryGBHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_g_b_hour: Option<f64>,
    #[serde(rename = "storageGBHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_g_b_hour: Option<f64>,
    #[serde(rename = "vCPUHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_c_p_u_hour: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOverrides {
    #[serde(rename = "applicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "diskEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRunExecutionIamPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "policyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDriver {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive: Option<Hive>,
    #[serde(rename = "sparkSubmit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit: Option<SparkSubmit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hive {
    #[serde(rename = "initQueryFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_query_file: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(default)]
    pub query: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkSubmit {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: String,
    #[serde(rename = "entryPointArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point_arguments: Option<Vec<String>>,
    #[serde(rename = "sparkSubmitParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit_parameters: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryPolicy {
    #[serde(rename = "maxAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i32>,
    #[serde(rename = "maxFailedAttemptsPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_attempts_per_hour: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalResourceUtilization {
    #[serde(rename = "memoryGBHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_g_b_hour: Option<f64>,
    #[serde(rename = "storageGBHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_g_b_hour: Option<f64>,
    #[serde(rename = "vCPUHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_c_p_u_hour: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDashboardRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDashboardResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEndpointRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEndpointResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "authToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "authTokenExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_expires_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "billedResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_resource_utilization: Option<ResourceUtilization>,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<SessionConfigurationOverrides>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "idleSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_since: Option<f64>,
    #[serde(rename = "idleTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "totalExecutionDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_duration_seconds: Option<i64>,
    #[serde(rename = "totalResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resource_utilization: Option<TotalResourceUtilization>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionConfigurationOverrides {
    #[serde(rename = "runtimeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<Vec<Configuration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunAttemptsRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    pub job_run_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunAttemptsResponse {
    #[serde(rename = "jobRunAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_attempts: Option<Vec<JobRunAttemptSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRunAttemptSummary {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "executionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "jobCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunsRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "createdAtAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_after: Option<f64>,
    #[serde(rename = "createdAtBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_before: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunsResponse {
    #[serde(rename = "jobRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRunSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRunSummary {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "attemptCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_created_at: Option<f64>,
    #[serde(rename = "attemptUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_updated_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "executionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "createdAtAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_after: Option<f64>,
    #[serde(rename = "createdAtBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_before: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionSummary {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartApplicationRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "executionIamPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_iam_policy: Option<JobRunExecutionIamPolicy>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "executionTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_minutes: Option<i64>,
    #[serde(rename = "jobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_driver: Option<JobDriver>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "retryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "jobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<SessionConfigurationOverrides>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "idleTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopApplicationRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "autoStartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start_configuration: Option<AutoStartConfig>,
    #[serde(rename = "autoStopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_stop_configuration: Option<AutoStopConfig>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "diskEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
    #[serde(rename = "identityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfigurationInput>,
    #[serde(rename = "imageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfigurationInput>,
    #[serde(rename = "initialCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_capacity: Option<std::collections::HashMap<String, InitialCapacityConfig>>,
    #[serde(rename = "interactiveConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_configuration: Option<InteractiveConfiguration>,
    #[serde(rename = "jobLevelCostAllocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_cost_allocation_configuration: Option<JobLevelCostAllocationConfiguration>,
    #[serde(rename = "maximumCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_capacity: Option<MaximumAllowedResources>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "runtimeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "schedulerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_configuration: Option<SchedulerConfiguration>,
    #[serde(rename = "workerTypeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type_specifications:
        Option<std::collections::HashMap<String, WorkerTypeSpecificationInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
}
