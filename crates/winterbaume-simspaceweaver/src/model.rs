//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-simspaceweaver

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotInput {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: S3Destination,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSimulationInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSimulationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppOutput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EndpointInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_info: Option<SimulationAppEndpointInfo>,
    #[serde(rename = "LaunchOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_overrides: Option<LaunchOverrides>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationAppEndpointInfo {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "IngressPortMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_port_mappings: Option<Vec<SimulationAppPortMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationAppPortMapping {
    #[serde(rename = "Actual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<i32>,
    #[serde(rename = "Declared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchOverrides {
    #[serde(rename = "LaunchCommands")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_commands: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSimulationInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSimulationOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "LiveSimulationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_simulation_state: Option<LiveSimulationState>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(rename = "MaximumDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SchemaError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_error: Option<String>,
    #[serde(rename = "SchemaS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_s3_location: Option<S3Location>,
    #[serde(rename = "SnapshotS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_s3_location: Option<S3Location>,
    #[serde(rename = "StartError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_error: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LiveSimulationState {
    #[serde(rename = "Clocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clocks: Option<Vec<SimulationClock>>,
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<Domain>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationClock {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Domain {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfiguration {
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<LogDestination>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDestination {
    #[serde(rename = "CloudWatchLogsLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsLogGroup {
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    pub object_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppsOutput {
    #[serde(rename = "Apps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SimulationAppMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationAppMetadata {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSimulationsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSimulationsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Simulations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulations: Option<Vec<SimulationMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAppInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "LaunchOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_overrides: Option<LaunchOverrides>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAppOutput {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartClockInput {
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartClockOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSimulationInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaximumDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SchemaS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_s3_location: Option<S3Location>,
    #[serde(rename = "SnapshotS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_s3_location: Option<S3Location>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSimulationOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAppInput {
    #[serde(rename = "App")]
    #[serde(default)]
    pub app: String,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAppOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopClockInput {
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopClockOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSimulationInput {
    #[serde(rename = "Simulation")]
    #[serde(default)]
    pub simulation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSimulationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}
