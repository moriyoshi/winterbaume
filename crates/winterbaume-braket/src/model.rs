//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-braket

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobResponse {
    #[serde(rename = "cancellationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_status: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQuantumTaskRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQuantumTaskResponse {
    #[serde(rename = "cancellationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_status: Option<String>,
    #[serde(rename = "quantumTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobRequest {
    #[serde(rename = "algorithmSpecification")]
    #[serde(default)]
    pub algorithm_specification: AlgorithmSpecification,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    #[serde(rename = "checkpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<JobCheckpointConfig>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "deviceConfig")]
    #[serde(default)]
    pub device_config: DeviceConfig,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<InputFileConfig>>,
    #[serde(rename = "instanceConfig")]
    #[serde(default)]
    pub instance_config: InstanceConfig,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    pub output_data_config: JobOutputDataConfig,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "stoppingCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<JobStoppingCondition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlgorithmSpecification {
    #[serde(rename = "containerImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_image: Option<ContainerImage>,
    #[serde(rename = "scriptModeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_mode_config: Option<ScriptModeConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerImage {
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScriptModeConfig {
    #[serde(rename = "compressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: String,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Association {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobCheckpointConfig {
    #[serde(rename = "localPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceConfig {
    #[serde(default)]
    pub device: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputFileConfig {
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    pub data_source: DataSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "s3DataSource")]
    #[serde(default)]
    pub s3_data_source: S3DataSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DataSource {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceConfig {
    #[serde(rename = "instanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "volumeSizeInGb")]
    #[serde(default)]
    pub volume_size_in_gb: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobOutputDataConfig {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "s3Path")]
    #[serde(default)]
    pub s3_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobStoppingCondition {
    #[serde(rename = "maxRuntimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQuantumTaskRequest {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    pub device_arn: String,
    #[serde(rename = "deviceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_parameters: Option<String>,
    #[serde(rename = "experimentalCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_capabilities: Option<ExperimentalCapabilities>,
    #[serde(rename = "jobToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_token: Option<String>,
    #[serde(rename = "outputS3Bucket")]
    #[serde(default)]
    pub output_s3_bucket: String,
    #[serde(rename = "outputS3KeyPrefix")]
    #[serde(default)]
    pub output_s3_key_prefix: String,
    #[serde(default)]
    pub shots: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentalCapabilities {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQuantumTaskResponse {
    #[serde(rename = "quantumTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSpendingLimitRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    pub device_arn: String,
    #[serde(rename = "spendingLimit")]
    #[serde(default)]
    pub spending_limit: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimePeriod {
    #[serde(rename = "endAt")]
    #[serde(default)]
    pub end_at: f64,
    #[serde(rename = "startAt")]
    #[serde(default)]
    pub start_at: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSpendingLimitResponse {
    #[serde(rename = "spendingLimitArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limit_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSpendingLimitRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSpendingLimitResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceResponse {
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "deviceCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_capabilities: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceQueueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_queue_info: Option<Vec<DeviceQueueInfo>>,
    #[serde(rename = "deviceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceQueueInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "queuePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_priority: Option<String>,
    #[serde(rename = "queueSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobResponse {
    #[serde(rename = "algorithmSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_specification: Option<AlgorithmSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    #[serde(rename = "billableDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_duration: Option<i32>,
    #[serde(rename = "checkpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<JobCheckpointConfig>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "deviceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_config: Option<DeviceConfig>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<JobEventDetails>>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<InputFileConfig>>,
    #[serde(rename = "instanceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_config: Option<InstanceConfig>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<JobOutputDataConfig>,
    #[serde(rename = "queueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<HybridJobQueueInfo>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "stoppingCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<JobStoppingCondition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobEventDetails {
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "timeOfEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_event: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridJobQueueInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQuantumTaskRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQuantumTaskResponse {
    #[serde(rename = "actionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_metadata: Option<ActionMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "deviceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_parameters: Option<String>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(rename = "experimentalCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_capabilities: Option<ExperimentalCapabilities>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "numSuccessfulShots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_successful_shots: Option<i64>,
    #[serde(rename = "outputS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket: Option<String>,
    #[serde(rename = "outputS3Directory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_directory: Option<String>,
    #[serde(rename = "quantumTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_task_arn: Option<String>,
    #[serde(rename = "queueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<QuantumTaskQueueInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionMetadata {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "executableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_count: Option<i64>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuantumTaskQueueInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "queuePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_priority: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDevicesRequest {
    #[serde(default)]
    pub filters: Vec<SearchDevicesFilter>,
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
pub struct SearchDevicesFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDevicesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceSummary {
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchJobsRequest {
    #[serde(default)]
    pub filters: Vec<SearchJobsFilter>,
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
pub struct SearchJobsFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQuantumTasksRequest {
    #[serde(default)]
    pub filters: Vec<SearchQuantumTasksFilter>,
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
pub struct SearchQuantumTasksFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQuantumTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "quantumTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_tasks: Option<Vec<QuantumTaskSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuantumTaskSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(rename = "outputS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket: Option<String>,
    #[serde(rename = "outputS3Directory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_directory: Option<String>,
    #[serde(rename = "quantumTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_task_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSpendingLimitsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SearchSpendingLimitsFilter>>,
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
pub struct SearchSpendingLimitsFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSpendingLimitsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "spendingLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimitSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpendingLimitSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "deviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "queuedSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_spend: Option<String>,
    #[serde(rename = "spendingLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<String>,
    #[serde(rename = "spendingLimitArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limit_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
    #[serde(rename = "totalSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spend: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSpendingLimitRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "spendingLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<String>,
    #[serde(rename = "timePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSpendingLimitResponse {}
