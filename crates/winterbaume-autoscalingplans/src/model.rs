//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-autoscalingplans

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScalingPlanRequest {
    #[serde(rename = "ApplicationSource")]
    #[serde(default)]
    pub application_source: ApplicationSource,
    #[serde(rename = "ScalingInstructions")]
    #[serde(default)]
    pub scaling_instructions: Vec<ScalingInstruction>,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    pub scaling_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSource {
    #[serde(rename = "CloudFormationStackARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_a_r_n: Option<String>,
    #[serde(rename = "TagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingInstruction {
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_load_metric_specification: Option<CustomizedLoadMetricSpecification>,
    #[serde(rename = "DisableDynamicScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_dynamic_scaling: Option<bool>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    pub max_capacity: i32,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    pub min_capacity: i32,
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_load_metric_specification: Option<PredefinedLoadMetricSpecification>,
    #[serde(rename = "PredictiveScalingMaxCapacityBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_max_capacity_behavior: Option<String>,
    #[serde(rename = "PredictiveScalingMaxCapacityBuffer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_max_capacity_buffer: Option<i32>,
    #[serde(rename = "PredictiveScalingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_mode: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ScalingPolicyUpdateBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy_update_behavior: Option<String>,
    #[serde(rename = "ScheduledActionBufferTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_buffer_time: Option<i32>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "TargetTrackingConfigurations")]
    #[serde(default)]
    pub target_tracking_configurations: Vec<TargetTrackingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomizedLoadMetricSpecification {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    pub statistic: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedLoadMetricSpecification {
    #[serde(rename = "PredefinedLoadMetricType")]
    #[serde(default)]
    pub predefined_load_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingConfiguration {
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_scaling_metric_specification: Option<CustomizedScalingMetricSpecification>,
    #[serde(rename = "DisableScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i32>,
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_scaling_metric_specification: Option<PredefinedScalingMetricSpecification>,
    #[serde(rename = "ScaleInCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i32>,
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i32>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomizedScalingMetricSpecification {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    pub statistic: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedScalingMetricSpecification {
    #[serde(rename = "PredefinedScalingMetricType")]
    #[serde(default)]
    pub predefined_scaling_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScalingPlanResponse {
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScalingPlanRequest {
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    pub scaling_plan_name: String,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    pub scaling_plan_version: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScalingPlanResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPlanResourcesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    pub scaling_plan_name: String,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    pub scaling_plan_version: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPlanResourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPlanResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_resources: Option<Vec<ScalingPlanResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingPlanResource {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_name: Option<String>,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
    #[serde(rename = "ScalingStatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_status_code: Option<String>,
    #[serde(rename = "ScalingStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_status_message: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingPolicy {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "TargetTrackingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPlansRequest {
    #[serde(rename = "ApplicationSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_sources: Option<Vec<ApplicationSource>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPlanNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_names: Option<Vec<String>>,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPlansResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plans: Option<Vec<ScalingPlan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingPlan {
    #[serde(rename = "ApplicationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_source: Option<ApplicationSource>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ScalingInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_instructions: Option<Vec<ScalingInstruction>>,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_name: Option<String>,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StatusStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScalingPlanResourceForecastDataRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "ForecastDataType")]
    #[serde(default)]
    pub forecast_data_type: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    pub scaling_plan_name: String,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    pub scaling_plan_version: i64,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScalingPlanResourceForecastDataResponse {
    #[serde(rename = "Datapoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints: Option<Vec<Datapoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Datapoint {
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScalingPlanRequest {
    #[serde(rename = "ApplicationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_source: Option<ApplicationSource>,
    #[serde(rename = "ScalingInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_instructions: Option<Vec<ScalingInstruction>>,
    #[serde(rename = "ScalingPlanName")]
    #[serde(default)]
    pub scaling_plan_name: String,
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(default)]
    pub scaling_plan_version: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScalingPlanResponse {}
