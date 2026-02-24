//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationinsights

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddWorkloadRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "WorkloadConfiguration")]
    #[serde(default)]
    pub workload_configuration: WorkloadConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkloadConfiguration {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "WorkloadName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddWorkloadResponse {
    #[serde(rename = "WorkloadConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_configuration: Option<WorkloadConfiguration>,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "AttachMissingPermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_missing_permission: Option<bool>,
    #[serde(rename = "AutoConfigEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_config_enabled: Option<bool>,
    #[serde(rename = "AutoCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    #[serde(rename = "CWEMonitorEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_w_e_monitor_enabled: Option<bool>,
    #[serde(rename = "GroupingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_type: Option<String>,
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_s_n_s_topic_arn: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "SNSNotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_notification_arn: Option<String>,
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
pub struct CreateApplicationResponse {
    #[serde(rename = "ApplicationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationInfo {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AttachMissingPermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_missing_permission: Option<bool>,
    #[serde(rename = "AutoConfigEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_config_enabled: Option<bool>,
    #[serde(rename = "CWEMonitorEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_w_e_monitor_enabled: Option<bool>,
    #[serde(rename = "DiscoveryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[serde(rename = "LifeCycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<String>,
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_s_n_s_topic_arn: Option<String>,
    #[serde(rename = "Remarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "SNSNotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_notification_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComponentRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "ResourceList")]
    #[serde(default)]
    pub resource_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComponentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogPatternRequest {
    #[serde(rename = "Pattern")]
    #[serde(default)]
    pub pattern: String,
    #[serde(rename = "PatternName")]
    #[serde(default)]
    pub pattern_name: String,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    pub pattern_set_name: String,
    #[serde(rename = "Rank")]
    #[serde(default)]
    pub rank: i32,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogPatternResponse {
    #[serde(rename = "LogPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogPattern {
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "PatternName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_name: Option<String>,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    #[serde(rename = "Rank")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteComponentRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteComponentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogPatternRequest {
    #[serde(rename = "PatternName")]
    #[serde(default)]
    pub pattern_name: String,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    pub pattern_set_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogPatternResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationResponse {
    #[serde(rename = "ApplicationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentConfigurationRecommendationRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "RecommendationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "Tier")]
    #[serde(default)]
    pub tier: String,
    #[serde(rename = "WorkloadName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentConfigurationRecommendationResponse {
    #[serde(rename = "ComponentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentConfigurationResponse {
    #[serde(rename = "ComponentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    #[serde(rename = "Monitor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComponentResponse {
    #[serde(rename = "ApplicationComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component: Option<ApplicationComponent>,
    #[serde(rename = "ResourceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationComponent {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "ComponentRemarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_remarks: Option<String>,
    #[serde(rename = "DetectedWorkload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_workload:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "Monitor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    #[serde(rename = "OsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogPatternRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "PatternName")]
    #[serde(default)]
    pub pattern_name: String,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    pub pattern_set_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogPatternResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "LogPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObservationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ObservationId")]
    #[serde(default)]
    pub observation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObservationResponse {
    #[serde(rename = "Observation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation: Option<Observation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Observation {
    #[serde(rename = "CloudWatchEventDetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_detail_type: Option<String>,
    #[serde(rename = "CloudWatchEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_id: Option<String>,
    #[serde(rename = "CloudWatchEventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_source: Option<String>,
    #[serde(rename = "CodeDeployApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_application: Option<String>,
    #[serde(rename = "CodeDeployDeploymentGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_deployment_group: Option<String>,
    #[serde(rename = "CodeDeployDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_deployment_id: Option<String>,
    #[serde(rename = "CodeDeployInstanceGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_instance_group_id: Option<String>,
    #[serde(rename = "CodeDeployState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_state: Option<String>,
    #[serde(rename = "EbsCause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_cause: Option<String>,
    #[serde(rename = "EbsEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_event: Option<String>,
    #[serde(rename = "EbsRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_request_id: Option<String>,
    #[serde(rename = "EbsResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_result: Option<String>,
    #[serde(rename = "Ec2State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_state: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "HealthEventArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_arn: Option<String>,
    #[serde(rename = "HealthEventDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_description: Option<String>,
    #[serde(rename = "HealthEventTypeCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_type_category: Option<String>,
    #[serde(rename = "HealthEventTypeCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_type_code: Option<String>,
    #[serde(rename = "HealthService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_service: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LineTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_time: Option<f64>,
    #[serde(rename = "LogFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "LogText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_text: Option<String>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "RdsEventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_event_categories: Option<String>,
    #[serde(rename = "RdsEventMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_event_message: Option<String>,
    #[serde(rename = "S3EventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_event_name: Option<String>,
    #[serde(rename = "SourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_a_r_n: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "StatesArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_arn: Option<String>,
    #[serde(rename = "StatesExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_execution_arn: Option<String>,
    #[serde(rename = "StatesInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_input: Option<String>,
    #[serde(rename = "StatesStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_status: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "XRayErrorPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_error_percent: Option<i32>,
    #[serde(rename = "XRayFaultPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_fault_percent: Option<i32>,
    #[serde(rename = "XRayNodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_node_name: Option<String>,
    #[serde(rename = "XRayNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_node_type: Option<String>,
    #[serde(rename = "XRayRequestAverageLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_request_average_latency: Option<i64>,
    #[serde(rename = "XRayRequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_request_count: Option<i32>,
    #[serde(rename = "XRayThrottlePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_throttle_percent: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProblemObservationsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ProblemId")]
    #[serde(default)]
    pub problem_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProblemObservationsResponse {
    #[serde(rename = "RelatedObservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<RelatedObservations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedObservations {
    #[serde(rename = "ObservationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_list: Option<Vec<Observation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProblemRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ProblemId")]
    #[serde(default)]
    pub problem_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProblemResponse {
    #[serde(rename = "Problem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<Problem>,
    #[serde(rename = "SNSNotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_notification_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Problem {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AffectedResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resource: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Feedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Insights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<String>,
    #[serde(rename = "LastRecurrenceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recurrence_time: Option<f64>,
    #[serde(rename = "RecurringCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_count: Option<i64>,
    #[serde(rename = "ResolutionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_method: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "SeverityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
    #[serde(rename = "ShortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkloadRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    pub workload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkloadResponse {
    #[serde(rename = "WorkloadConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_configuration: Option<WorkloadConfiguration>,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_id: Option<String>,
    #[serde(rename = "WorkloadRemarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_remarks: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
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
pub struct ListApplicationsResponse {
    #[serde(rename = "ApplicationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info_list: Option<Vec<ApplicationInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComponentsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComponentsResponse {
    #[serde(rename = "ApplicationComponentList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component_list: Option<Vec<ApplicationComponent>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationHistoryRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EventStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationHistoryResponse {
    #[serde(rename = "EventList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_list: Option<Vec<ConfigurationEvent>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationEvent {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EventDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
    #[serde(rename = "EventResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_name: Option<String>,
    #[serde(rename = "EventResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_type: Option<String>,
    #[serde(rename = "EventStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    #[serde(rename = "EventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(rename = "MonitoredResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_resource_a_r_n: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogPatternSetsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogPatternSetsResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "LogPatternSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern_sets: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogPatternsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogPatternsResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "LogPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_patterns: Option<Vec<LogPattern>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProblemsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProblemsResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProblemList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_list: Option<Vec<Problem>>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkloadsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkloadsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkloadList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_list: Option<Vec<Workload>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workload {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "MissingWorkloadConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_workload_config: Option<bool>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_id: Option<String>,
    #[serde(rename = "WorkloadName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_name: Option<String>,
    #[serde(rename = "WorkloadRemarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_remarks: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveWorkloadRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    pub workload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveWorkloadResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "AttachMissingPermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_missing_permission: Option<bool>,
    #[serde(rename = "AutoConfigEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_config_enabled: Option<bool>,
    #[serde(rename = "CWEMonitorEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_w_e_monitor_enabled: Option<bool>,
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_s_n_s_topic_arn: Option<String>,
    #[serde(rename = "RemoveSNSTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_s_n_s_topic: Option<bool>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "SNSNotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_notification_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {
    #[serde(rename = "ApplicationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentConfigurationRequest {
    #[serde(rename = "AutoConfigEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_config_enabled: Option<bool>,
    #[serde(rename = "ComponentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "Monitor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "NewComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_component_name: Option<String>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "ResourceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLogPatternRequest {
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "PatternName")]
    #[serde(default)]
    pub pattern_name: String,
    #[serde(rename = "PatternSetName")]
    #[serde(default)]
    pub pattern_set_name: String,
    #[serde(rename = "Rank")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLogPatternResponse {
    #[serde(rename = "LogPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProblemRequest {
    #[serde(rename = "ProblemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProblemResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkloadRequest {
    #[serde(rename = "ComponentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(rename = "ResourceGroupName")]
    #[serde(default)]
    pub resource_group_name: String,
    #[serde(rename = "WorkloadConfiguration")]
    #[serde(default)]
    pub workload_configuration: WorkloadConfiguration,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkloadResponse {
    #[serde(rename = "WorkloadConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_configuration: Option<WorkloadConfiguration>,
    #[serde(rename = "WorkloadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_id: Option<String>,
}
