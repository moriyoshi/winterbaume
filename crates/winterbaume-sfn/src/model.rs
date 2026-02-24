//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sfn

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActivityInput {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "kmsDataKeyReusePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<i32>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
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
pub struct CreateActivityOutput {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_arn: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStateMachineAliasInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    pub routing_configuration: Vec<RoutingConfigurationListItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingConfigurationListItem {
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    pub state_machine_version_arn: String,
    #[serde(default)]
    pub weight: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStateMachineAliasOutput {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStateMachineInput {
    #[serde(default)]
    pub definition: String,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "loggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "tracingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<TracingConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "versionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<LogDestination>>,
    #[serde(rename = "includeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDestination {
    #[serde(rename = "cloudWatchLogsLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsLogGroup {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TracingConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStateMachineOutput {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActivityInput {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    pub activity_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActivityOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineAliasInput {
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    pub state_machine_alias_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineAliasOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineInput {
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineVersionInput {
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    pub state_machine_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStateMachineVersionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActivityInput {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    pub activity_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActivityOutput {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_arn: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionInput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
    #[serde(rename = "includedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<CloudWatchEventsExecutionDataDetails>,
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<CloudWatchEventsExecutionDataDetails>,
    #[serde(rename = "redriveCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_count: Option<i32>,
    #[serde(rename = "redriveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_date: Option<f64>,
    #[serde(rename = "redriveStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_status: Option<String>,
    #[serde(rename = "redriveStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_status_reason: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "traceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchEventsExecutionDataDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMapRunInput {
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    pub map_run_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMapRunOutput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(rename = "executionCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_counts: Option<MapRunExecutionCounts>,
    #[serde(rename = "itemCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_counts: Option<MapRunItemCounts>,
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(rename = "maxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(rename = "redriveCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_count: Option<i32>,
    #[serde(rename = "redriveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_date: Option<f64>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "toleratedFailureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_count: Option<i64>,
    #[serde(rename = "toleratedFailurePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_percentage: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunExecutionCounts {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aborted: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(rename = "failuresNotRedrivable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures_not_redrivable: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "pendingRedrive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_redrive: Option<i64>,
    #[serde(rename = "resultsWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_written: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i64>,
    #[serde(rename = "timedOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunItemCounts {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aborted: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(rename = "failuresNotRedrivable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures_not_redrivable: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "pendingRedrive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_redrive: Option<i64>,
    #[serde(rename = "resultsWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_written: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i64>,
    #[serde(rename = "timedOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineAliasInput {
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    pub state_machine_alias_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineAliasOutput {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<RoutingConfigurationListItem>>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
    #[serde(rename = "updateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineForExecutionInput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
    #[serde(rename = "includedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineForExecutionOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "loggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "tracingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<TracingConfiguration>,
    #[serde(rename = "updateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
    #[serde(rename = "variableReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_references: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineInput {
    #[serde(rename = "includedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_data: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStateMachineOutput {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "loggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tracingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<TracingConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "variableReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_references: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActivityTaskInput {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    pub activity_arn: String,
    #[serde(rename = "workerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActivityTaskOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionHistoryInput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
    #[serde(rename = "includeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionHistoryOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HistoryEvent>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoryEvent {
    #[serde(rename = "activityFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_failed_event_details: Option<ActivityFailedEventDetails>,
    #[serde(rename = "activityScheduleFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_schedule_failed_event_details: Option<ActivityScheduleFailedEventDetails>,
    #[serde(rename = "activityScheduledEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_scheduled_event_details: Option<ActivityScheduledEventDetails>,
    #[serde(rename = "activityStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_started_event_details: Option<ActivityStartedEventDetails>,
    #[serde(rename = "activitySucceededEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_succeeded_event_details: Option<ActivitySucceededEventDetails>,
    #[serde(rename = "activityTimedOutEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timed_out_event_details: Option<ActivityTimedOutEventDetails>,
    #[serde(rename = "evaluationFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_failed_event_details: Option<EvaluationFailedEventDetails>,
    #[serde(rename = "executionAbortedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_aborted_event_details: Option<ExecutionAbortedEventDetails>,
    #[serde(rename = "executionFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_failed_event_details: Option<ExecutionFailedEventDetails>,
    #[serde(rename = "executionRedrivenEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_redriven_event_details: Option<ExecutionRedrivenEventDetails>,
    #[serde(rename = "executionStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_started_event_details: Option<ExecutionStartedEventDetails>,
    #[serde(rename = "executionSucceededEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_succeeded_event_details: Option<ExecutionSucceededEventDetails>,
    #[serde(rename = "executionTimedOutEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timed_out_event_details: Option<ExecutionTimedOutEventDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lambdaFunctionFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_failed_event_details: Option<LambdaFunctionFailedEventDetails>,
    #[serde(rename = "lambdaFunctionScheduleFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_schedule_failed_event_details:
        Option<LambdaFunctionScheduleFailedEventDetails>,
    #[serde(rename = "lambdaFunctionScheduledEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_scheduled_event_details: Option<LambdaFunctionScheduledEventDetails>,
    #[serde(rename = "lambdaFunctionStartFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_start_failed_event_details: Option<LambdaFunctionStartFailedEventDetails>,
    #[serde(rename = "lambdaFunctionSucceededEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_succeeded_event_details: Option<LambdaFunctionSucceededEventDetails>,
    #[serde(rename = "lambdaFunctionTimedOutEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_timed_out_event_details: Option<LambdaFunctionTimedOutEventDetails>,
    #[serde(rename = "mapIterationAbortedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_aborted_event_details: Option<MapIterationEventDetails>,
    #[serde(rename = "mapIterationFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_failed_event_details: Option<MapIterationEventDetails>,
    #[serde(rename = "mapIterationStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_started_event_details: Option<MapIterationEventDetails>,
    #[serde(rename = "mapIterationSucceededEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_succeeded_event_details: Option<MapIterationEventDetails>,
    #[serde(rename = "mapRunFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_failed_event_details: Option<MapRunFailedEventDetails>,
    #[serde(rename = "mapRunRedrivenEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_redriven_event_details: Option<MapRunRedrivenEventDetails>,
    #[serde(rename = "mapRunStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_started_event_details: Option<MapRunStartedEventDetails>,
    #[serde(rename = "mapStateStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_state_started_event_details: Option<MapStateStartedEventDetails>,
    #[serde(rename = "previousEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_event_id: Option<i64>,
    #[serde(rename = "stateEnteredEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_entered_event_details: Option<StateEnteredEventDetails>,
    #[serde(rename = "stateExitedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_exited_event_details: Option<StateExitedEventDetails>,
    #[serde(rename = "taskFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_failed_event_details: Option<TaskFailedEventDetails>,
    #[serde(rename = "taskScheduledEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_scheduled_event_details: Option<TaskScheduledEventDetails>,
    #[serde(rename = "taskStartFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_failed_event_details: Option<TaskStartFailedEventDetails>,
    #[serde(rename = "taskStartedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_started_event_details: Option<TaskStartedEventDetails>,
    #[serde(rename = "taskSubmitFailedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_submit_failed_event_details: Option<TaskSubmitFailedEventDetails>,
    #[serde(rename = "taskSubmittedEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_submitted_event_details: Option<TaskSubmittedEventDetails>,
    #[serde(rename = "taskSucceededEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_succeeded_event_details: Option<TaskSucceededEventDetails>,
    #[serde(rename = "taskTimedOutEventDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_timed_out_event_details: Option<TaskTimedOutEventDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityScheduleFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityScheduledEventDetails {
    #[serde(rename = "heartbeatInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_in_seconds: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "timeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoryEventExecutionDataDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityStartedEventDetails {
    #[serde(rename = "workerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivitySucceededEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTimedOutEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionAbortedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionRedrivenEventDetails {
    #[serde(rename = "redriveCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStartedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionSucceededEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionTimedOutEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionScheduleFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionScheduledEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "taskCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_credentials: Option<TaskCredentials>,
    #[serde(rename = "timeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskCredentials {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionStartFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionSucceededEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionTimedOutEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapIterationEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunRedrivenEventDetails {
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(rename = "redriveCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunStartedEventDetails {
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapStateStartedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateEnteredEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateExitedEventDetails {
    #[serde(rename = "assignedVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "assignedVariablesDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_variables_details: Option<AssignedVariablesDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssignedVariablesDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskScheduledEventDetails {
    #[serde(rename = "heartbeatInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_in_seconds: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "taskCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_credentials: Option<TaskCredentials>,
    #[serde(rename = "timeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskStartFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskStartedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSubmitFailedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSubmittedEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSucceededEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<HistoryEventExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTimedOutEventDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActivitiesInput {
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
pub struct ListActivitiesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<ActivityListItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityListItem {
    #[serde(rename = "activityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_arn: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsInput {
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "redriveFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_filter: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "statusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<Vec<ExecutionListItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionListItem {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(rename = "itemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "redriveCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_count: Option<i32>,
    #[serde(rename = "redriveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_date: Option<f64>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMapRunsInput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
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
pub struct ListMapRunsOutput {
    #[serde(rename = "mapRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_runs: Option<Vec<MapRunListItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapRunListItem {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_run_arn: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStateMachineAliasesInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStateMachineAliasesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachineAliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_aliases: Option<Vec<StateMachineAliasListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateMachineAliasListItem {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStateMachineVersionsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStateMachineVersionsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_versions: Option<Vec<StateMachineVersionListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateMachineVersionListItem {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStateMachinesInput {
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
pub struct ListStateMachinesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machines: Option<Vec<StateMachineListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateMachineListItem {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishStateMachineVersionInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishStateMachineVersionOutput {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedriveExecutionInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedriveExecutionOutput {
    #[serde(rename = "redriveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redrive_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskFailureInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskFailureOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskHeartbeatInput {
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskHeartbeatOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskSuccessInput {
    #[serde(default)]
    pub output: String,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendTaskSuccessOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExecutionInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
    #[serde(rename = "traceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExecutionOutput {
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSyncExecutionInput {
    #[serde(rename = "includedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_data: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
    #[serde(rename = "traceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSyncExecutionOutput {
    #[serde(rename = "billingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "executionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_details: Option<CloudWatchEventsExecutionDataDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "outputDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<CloudWatchEventsExecutionDataDetails>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "traceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BillingDetails {
    #[serde(rename = "billedDurationInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_duration_in_milliseconds: Option<i64>,
    #[serde(rename = "billedMemoryUsedInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_memory_used_in_m_b: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopExecutionInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "executionArn")]
    #[serde(default)]
    pub execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopExecutionOutput {
    #[serde(rename = "stopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestStateInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "inspectionLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspection_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock: Option<MockInput>,
    #[serde(rename = "revealSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_secrets: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_configuration: Option<TestStateConfiguration>,
    #[serde(rename = "stateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MockInput {
    #[serde(rename = "errorOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output: Option<MockErrorOutput>,
    #[serde(rename = "fieldValidationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_validation_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MockErrorOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestStateConfiguration {
    #[serde(rename = "errorCausedByState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_caused_by_state: Option<String>,
    #[serde(rename = "mapItemReaderData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_item_reader_data: Option<String>,
    #[serde(rename = "mapIterationFailureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_failure_count: Option<i32>,
    #[serde(rename = "retrierRetryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrier_retry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestStateOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "inspectionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspection_data: Option<InspectionData>,
    #[serde(rename = "nextState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InspectionData {
    #[serde(rename = "afterArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_arguments: Option<String>,
    #[serde(rename = "afterInputPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_input_path: Option<String>,
    #[serde(rename = "afterItemBatcher")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_item_batcher: Option<String>,
    #[serde(rename = "afterItemSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_item_selector: Option<String>,
    #[serde(rename = "afterItemsPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_items_path: Option<String>,
    #[serde(rename = "afterItemsPointer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_items_pointer: Option<String>,
    #[serde(rename = "afterParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_parameters: Option<String>,
    #[serde(rename = "afterResultPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_result_path: Option<String>,
    #[serde(rename = "afterResultSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_result_selector: Option<String>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<InspectionErrorDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "maxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<InspectionDataRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<InspectionDataResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "toleratedFailureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_count: Option<i32>,
    #[serde(rename = "toleratedFailurePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_percentage: Option<f32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InspectionErrorDetails {
    #[serde(rename = "catchIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_index: Option<i32>,
    #[serde(rename = "retryBackoffIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_backoff_interval_seconds: Option<i32>,
    #[serde(rename = "retryIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InspectionDataRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InspectionDataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMapRunInput {
    #[serde(rename = "mapRunArn")]
    #[serde(default)]
    pub map_run_arn: String,
    #[serde(rename = "maxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(rename = "toleratedFailureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_count: Option<i64>,
    #[serde(rename = "toleratedFailurePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerated_failure_percentage: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMapRunOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStateMachineAliasInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<RoutingConfigurationListItem>>,
    #[serde(rename = "stateMachineAliasArn")]
    #[serde(default)]
    pub state_machine_alias_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStateMachineAliasOutput {
    #[serde(rename = "updateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStateMachineInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "loggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stateMachineArn")]
    #[serde(default)]
    pub state_machine_arn: String,
    #[serde(rename = "tracingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<TracingConfiguration>,
    #[serde(rename = "versionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStateMachineOutput {
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "stateMachineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_version_arn: Option<String>,
    #[serde(rename = "updateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateStateMachineDefinitionInput {
    #[serde(default)]
    pub definition: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateStateMachineDefinitionOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<ValidateStateMachineDefinitionDiagnostic>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateStateMachineDefinitionDiagnostic {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}
