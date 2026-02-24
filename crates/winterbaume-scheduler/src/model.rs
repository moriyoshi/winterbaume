//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-scheduler

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleGroupInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleGroupOutput {
    #[serde(rename = "ScheduleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleInput {
    #[serde(rename = "ActionAfterCompletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_after_completion: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "FlexibleTimeWindow")]
    #[serde(default)]
    pub flexible_time_window: FlexibleTimeWindow,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlexibleTimeWindow {
    #[serde(rename = "MaximumWindowInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_window_in_minutes: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "EcsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    #[serde(rename = "EventBridgeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_parameters: Option<EventBridgeParameters>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "KinesisParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,
    #[serde(rename = "RetryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SageMakerPipelineParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,
    #[serde(rename = "SqsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsParameters {
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "EnableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "LaunchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "PlacementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "PlacementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "PropagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "TaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    #[serde(rename = "TaskDefinitionArn")]
    #[serde(default)]
    pub task_definition_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderStrategyItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i32>,
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    pub capacity_provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "AssignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementConstraint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementStrategy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeParameters {
    #[serde(rename = "DetailType")]
    #[serde(default)]
    pub detail_type: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisParameters {
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    pub partition_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryPolicy {
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
pub struct SageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqsParameters {
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleOutput {
    #[serde(rename = "ScheduleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleGroupInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleGroupOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduleGroupInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduleGroupOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModificationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduleInput {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduleOutput {
    #[serde(rename = "ActionAfterCompletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_after_completion: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "FlexibleTimeWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_time_window: Option<FlexibleTimeWindow>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LastModificationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduleGroupsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduleGroupsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_groups: Option<Vec<ScheduleGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleGroupSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModificationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchedulesInput {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchedulesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<ScheduleSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "LastModificationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TargetSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduleInput {
    #[serde(rename = "ActionAfterCompletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_after_completion: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "FlexibleTimeWindow")]
    #[serde(default)]
    pub flexible_time_window: FlexibleTimeWindow,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduleOutput {
    #[serde(rename = "ScheduleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_arn: Option<String>,
}
