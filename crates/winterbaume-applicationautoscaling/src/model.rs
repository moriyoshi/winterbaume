//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationautoscaling

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScalingPolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScalingPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledActionRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledActionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterScalableTargetRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterScalableTargetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalableTargetsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalableTargetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalableTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_targets: Option<Vec<ScalableTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalableTarget {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
    #[serde(rename = "PredictedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_capacity: Option<i32>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ScalableTargetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_a_r_n: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "SuspendedState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_state: Option<SuspendedState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuspendedState {
    #[serde(rename = "DynamicScalingInSuspended")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_scaling_in_suspended: Option<bool>,
    #[serde(rename = "DynamicScalingOutSuspended")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_scaling_out_suspended: Option<bool>,
    #[serde(rename = "ScheduledScalingSuspended")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_scaling_suspended: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingActivitiesRequest {
    #[serde(rename = "IncludeNotScaledActivities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_not_scaled_activities: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingActivitiesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingActivities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_activities: Option<Vec<ScalingActivity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingActivity {
    #[serde(rename = "ActivityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "Cause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NotScaledReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_scaled_reasons: Option<Vec<NotScaledReason>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotScaledReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "CurrentCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_capacity: Option<i32>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPoliciesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScalingPoliciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingPolicy {
    #[serde(rename = "Alarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "PolicyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_a_r_n: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "PredictiveScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_policy_configuration: Option<PredictiveScalingPolicyConfiguration>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alarm {
    #[serde(rename = "AlarmARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_a_r_n: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingPolicyConfiguration {
    #[serde(rename = "MaxCapacityBreachBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity_breach_behavior: Option<String>,
    #[serde(rename = "MaxCapacityBuffer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity_buffer: Option<i32>,
    #[serde(rename = "MetricSpecifications")]
    #[serde(default)]
    pub metric_specifications: Vec<PredictiveScalingMetricSpecification>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "SchedulingBufferTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_buffer_time: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingMetricSpecification {
    #[serde(rename = "CustomizedCapacityMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_capacity_metric_specification:
        Option<PredictiveScalingCustomizedMetricSpecification>,
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_load_metric_specification:
        Option<PredictiveScalingCustomizedMetricSpecification>,
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_scaling_metric_specification:
        Option<PredictiveScalingCustomizedMetricSpecification>,
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_load_metric_specification:
        Option<PredictiveScalingPredefinedLoadMetricSpecification>,
    #[serde(rename = "PredefinedMetricPairSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_metric_pair_specification:
        Option<PredictiveScalingPredefinedMetricPairSpecification>,
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_scaling_metric_specification:
        Option<PredictiveScalingPredefinedScalingMetricSpecification>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingCustomizedMetricSpecification {
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    pub metric_data_queries: Vec<PredictiveScalingMetricDataQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingMetricDataQuery {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_stat: Option<PredictiveScalingMetricStat>,
    #[serde(rename = "ReturnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingMetricStat {
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: PredictiveScalingMetric,
    #[serde(rename = "Stat")]
    #[serde(default)]
    pub stat: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingMetric {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<PredictiveScalingMetricDimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingMetricDimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingPredefinedLoadMetricSpecification {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingPredefinedMetricPairSpecification {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingPredefinedScalingMetricSpecification {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepScalingPolicyConfiguration {
    #[serde(rename = "AdjustmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "Cooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i32>,
    #[serde(rename = "MetricAggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<i32>,
    #[serde(rename = "StepAdjustments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepAdjustment {
    #[serde(rename = "MetricIntervalLowerBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_interval_lower_bound: Option<f64>,
    #[serde(rename = "MetricIntervalUpperBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_interval_upper_bound: Option<f64>,
    #[serde(rename = "ScalingAdjustment")]
    #[serde(default)]
    pub scaling_adjustment: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingScalingPolicyConfiguration {
    #[serde(rename = "CustomizedMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    #[serde(rename = "DisableScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "PredefinedMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
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
pub struct CustomizedMetricSpecification {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TargetTrackingMetricDataQuery>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
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
pub struct TargetTrackingMetricDataQuery {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_stat: Option<TargetTrackingMetricStat>,
    #[serde(rename = "ReturnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingMetricStat {
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: TargetTrackingMetric,
    #[serde(rename = "Stat")]
    #[serde(default)]
    pub stat: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingMetric {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<TargetTrackingMetricDimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingMetricDimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedMetricSpecification {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledActionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ScheduledActionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_names: Option<Vec<String>>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledActionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<Vec<ScheduledAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledAction {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[serde(rename = "ScalableTargetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduledActionARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_a_r_n: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_name: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalableTargetAction {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPredictiveScalingForecastRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPredictiveScalingForecastResponse {
    #[serde(rename = "CapacityForecast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_forecast: Option<CapacityForecast>,
    #[serde(rename = "LoadForecast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_forecast: Option<Vec<LoadForecast>>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityForecast {
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadForecast {
    #[serde(rename = "MetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_specification: Option<PredictiveScalingMetricSpecification>,
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
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
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutScalingPolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "PredictiveScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_policy_configuration: Option<PredictiveScalingPolicyConfiguration>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutScalingPolicyResponse {
    #[serde(rename = "Alarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    #[serde(rename = "PolicyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutScheduledActionRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ScalableTargetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutScheduledActionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterScalableTargetRequest {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "ScalableDimension")]
    #[serde(default)]
    pub scalable_dimension: String,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
    #[serde(rename = "SuspendedState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_state: Option<SuspendedState>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterScalableTargetResponse {
    #[serde(rename = "ScalableTargetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
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
