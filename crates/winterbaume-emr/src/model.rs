//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emr

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddInstanceFleetInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "InstanceFleet")]
    #[serde(default)]
    pub instance_fleet: InstanceFleetConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetConfig {
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "InstanceFleetType")]
    #[serde(default)]
    pub instance_fleet_type: String,
    #[serde(rename = "InstanceTypeConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    #[serde(rename = "LaunchSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResizeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_specifications: Option<InstanceFleetResizingSpecifications>,
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i32>,
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTypeConfig {
    #[serde(rename = "BidPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsConfiguration {
    #[serde(rename = "EbsBlockDeviceConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsBlockDeviceConfig {
    #[serde(rename = "VolumeSpecification")]
    #[serde(default)]
    pub volume_specification: VolumeSpecification,
    #[serde(rename = "VolumesPerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_per_instance: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeSpecification {
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "SizeInGB")]
    #[serde(default)]
    pub size_in_g_b: i32,
    #[serde(rename = "Throughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    pub volume_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetProvisioningSpecifications {
    #[serde(rename = "OnDemandSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_specification: Option<OnDemandProvisioningSpecification>,
    #[serde(rename = "SpotSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_specification: Option<SpotProvisioningSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandProvisioningSpecification {
    #[serde(rename = "AllocationStrategy")]
    #[serde(default)]
    pub allocation_strategy: String,
    #[serde(rename = "CapacityReservationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_options: Option<OnDemandCapacityReservationOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandCapacityReservationOptions {
    #[serde(rename = "CapacityReservationPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_preference: Option<String>,
    #[serde(rename = "CapacityReservationResourceGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_resource_group_arn: Option<String>,
    #[serde(rename = "UsageStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpotProvisioningSpecification {
    #[serde(rename = "AllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "BlockDurationMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_duration_minutes: Option<i32>,
    #[serde(rename = "TimeoutAction")]
    #[serde(default)]
    pub timeout_action: String,
    #[serde(rename = "TimeoutDurationMinutes")]
    #[serde(default)]
    pub timeout_duration_minutes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetResizingSpecifications {
    #[serde(rename = "OnDemandResizeSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_resize_specification: Option<OnDemandResizingSpecification>,
    #[serde(rename = "SpotResizeSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_resize_specification: Option<SpotResizingSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandResizingSpecification {
    #[serde(rename = "AllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "CapacityReservationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_options: Option<OnDemandCapacityReservationOptions>,
    #[serde(rename = "TimeoutDurationMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_duration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpotResizingSpecification {
    #[serde(rename = "AllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "TimeoutDurationMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_duration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddInstanceFleetOutput {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "InstanceFleetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddInstanceGroupsInput {
    #[serde(rename = "InstanceGroups")]
    #[serde(default)]
    pub instance_groups: Vec<InstanceGroupConfig>,
    #[serde(rename = "JobFlowId")]
    #[serde(default)]
    pub job_flow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupConfig {
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    #[serde(rename = "BidPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    pub instance_count: i32,
    #[serde(rename = "InstanceRole")]
    #[serde(default)]
    pub instance_role: String,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "Market")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicy {
    #[serde(rename = "Constraints")]
    #[serde(default)]
    pub constraints: ScalingConstraints,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<ScalingRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingConstraints {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    pub max_capacity: i32,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    pub min_capacity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: ScalingAction,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Trigger")]
    #[serde(default)]
    pub trigger: ScalingTrigger,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingAction {
    #[serde(rename = "Market")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    #[serde(default)]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleScalingPolicyConfiguration {
    #[serde(rename = "AdjustmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "CoolDown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cool_down: Option<i32>,
    #[serde(rename = "ScalingAdjustment")]
    #[serde(default)]
    pub scaling_adjustment: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingTrigger {
    #[serde(rename = "CloudWatchAlarmDefinition")]
    #[serde(default)]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchAlarmDefinition {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "EvaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    pub threshold: f64,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimension {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddInstanceGroupsOutput {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "InstanceGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_ids: Option<Vec<String>>,
    #[serde(rename = "JobFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddJobFlowStepsInput {
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "JobFlowId")]
    #[serde(default)]
    pub job_flow_id: String,
    #[serde(rename = "Steps")]
    #[serde(default)]
    pub steps: Vec<StepConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepConfig {
    #[serde(rename = "ActionOnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    #[serde(rename = "HadoopJarStep")]
    #[serde(default)]
    pub hadoop_jar_step: HadoopJarStepConfig,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StepMonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_monitoring_configuration: Option<StepMonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HadoopJarStepConfig {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Jar")]
    #[serde(default)]
    pub jar: String,
    #[serde(rename = "MainClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<KeyValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValue {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepMonitoringConfiguration {
    #[serde(rename = "S3MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_monitoring_configuration: Option<S3MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3MonitoringConfiguration {
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddJobFlowStepsOutput {
    #[serde(rename = "StepIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsInput {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStepsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "StepCancellationOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_cancellation_option: Option<String>,
    #[serde(rename = "StepIds")]
    #[serde(default)]
    pub step_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStepsOutput {
    #[serde(rename = "CancelStepsInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_steps_info_list: Option<Vec<CancelStepsInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStepsInfo {
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StepId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePersistentAppUIInput {
    #[serde(rename = "EMRContainersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_m_r_containers_config: Option<EMRContainersConfig>,
    #[serde(rename = "ProfilerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiler_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetResourceArn")]
    #[serde(default)]
    pub target_resource_arn: String,
    #[serde(rename = "XReferer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_referer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EMRContainersConfig {
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePersistentAppUIOutput {
    #[serde(rename = "PersistentAppUIId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i_id: Option<String>,
    #[serde(rename = "RuntimeRoleEnabledCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_role_enabled_cluster: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    pub security_configuration: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationOutput {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStudioInput {
    #[serde(rename = "AuthMode")]
    #[serde(default)]
    pub auth_mode: String,
    #[serde(rename = "DefaultS3Location")]
    #[serde(default)]
    pub default_s3_location: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EngineSecurityGroupId")]
    #[serde(default)]
    pub engine_security_group_id: String,
    #[serde(rename = "IdcInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_instance_arn: Option<String>,
    #[serde(rename = "IdcUserAssignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_user_assignment: Option<String>,
    #[serde(rename = "IdpAuthUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_auth_url: Option<String>,
    #[serde(rename = "IdpRelayStateParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_relay_state_parameter_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    pub service_role: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrustedIdentityPropagationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_identity_propagation_enabled: Option<bool>,
    #[serde(rename = "UserRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(default)]
    pub workspace_security_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStudioOutput {
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStudioSessionMappingInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "SessionPolicyArn")]
    #[serde(default)]
    pub session_policy_arn: String,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigurationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStudioInput {
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStudioSessionMappingInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterOutput {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "AutoScalingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[serde(rename = "AutoTerminate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminate: Option<bool>,
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsRootVolumeIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_iops: Option<i32>,
    #[serde(rename = "EbsRootVolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_size: Option<i32>,
    #[serde(rename = "EbsRootVolumeThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_throughput: Option<i32>,
    #[serde(rename = "Ec2InstanceAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_attributes: Option<Ec2InstanceAttributes>,
    #[serde(rename = "ExtendedSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_support: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceCollectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_collection_type: Option<String>,
    #[serde(rename = "KerberosAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_attributes: Option<KerberosAttributes>,
    #[serde(rename = "LogEncryptionKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_encryption_kms_key_id: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "MasterPublicDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_public_dns_name: Option<String>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i32>,
    #[serde(rename = "OSReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_s_release_label: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "PlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_groups: Option<Vec<PlacementGroupConfig>>,
    #[serde(rename = "ReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "RepoUpgradeOnBoot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    #[serde(rename = "RequestedAmiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ami_version: Option<String>,
    #[serde(rename = "RunningAmiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_ami_version: Option<String>,
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TerminationProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
    #[serde(rename = "UnhealthyNodeReplacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_node_replacement: Option<bool>,
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2InstanceAttributes {
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    #[serde(rename = "Ec2AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_availability_zone: Option<String>,
    #[serde(rename = "Ec2KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_name: Option<String>,
    #[serde(rename = "Ec2SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_subnet_id: Option<String>,
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    #[serde(rename = "RequestedEc2AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ec2_availability_zones: Option<Vec<String>>,
    #[serde(rename = "RequestedEc2SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ec2_subnet_ids: Option<Vec<String>>,
    #[serde(rename = "ServiceAccessSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_security_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KerberosAttributes {
    #[serde(rename = "ADDomainJoinPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_domain_join_password: Option<String>,
    #[serde(rename = "ADDomainJoinUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_domain_join_user: Option<String>,
    #[serde(rename = "CrossRealmTrustPrincipalPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_realm_trust_principal_password: Option<String>,
    #[serde(rename = "KdcAdminPassword")]
    #[serde(default)]
    pub kdc_admin_password: String,
    #[serde(rename = "Realm")]
    #[serde(default)]
    pub realm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfiguration {
    #[serde(rename = "CloudWatchLogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_configuration: Option<CloudWatchLogConfiguration>,
    #[serde(rename = "S3LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logging_configuration: Option<S3LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "LogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LoggingConfiguration {
    #[serde(rename = "LogTypeUploadPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type_upload_policy: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementGroupConfig {
    #[serde(rename = "InstanceRole")]
    #[serde(default)]
    pub instance_role: String,
    #[serde(rename = "PlacementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterStatus {
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ClusterStateChangeReason>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<ClusterTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_data: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterTimeline {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobFlowsInput {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_ids: Option<Vec<String>>,
    #[serde(rename = "JobFlowStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobFlowsOutput {
    #[serde(rename = "JobFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flows: Option<Vec<JobFlowDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobFlowDetail {
    #[serde(rename = "AmiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_version: Option<String>,
    #[serde(rename = "AutoScalingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[serde(rename = "BootstrapActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionDetail>>,
    #[serde(rename = "ExecutionStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status_detail: Option<JobFlowExecutionStatusDetail>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<JobFlowInstancesDetail>,
    #[serde(rename = "JobFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_id: Option<String>,
    #[serde(rename = "JobFlowRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_role: Option<String>,
    #[serde(rename = "LogEncryptionKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_encryption_kms_key_id: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepDetail>>,
    #[serde(rename = "SupportedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BootstrapActionDetail {
    #[serde(rename = "BootstrapActionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_action_config: Option<BootstrapActionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BootstrapActionConfig {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ScriptBootstrapAction")]
    #[serde(default)]
    pub script_bootstrap_action: ScriptBootstrapActionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScriptBootstrapActionConfig {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobFlowExecutionStatusDetail {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "LastStateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobFlowInstancesDetail {
    #[serde(rename = "Ec2KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_name: Option<String>,
    #[serde(rename = "Ec2SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_subnet_id: Option<String>,
    #[serde(rename = "HadoopVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hadoop_version: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupDetail>>,
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    #[serde(rename = "MasterInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_id: Option<String>,
    #[serde(rename = "MasterInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_type: Option<String>,
    #[serde(rename = "MasterPublicDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_public_dns_name: Option<String>,
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i32>,
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementType>,
    #[serde(rename = "SlaveInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_instance_type: Option<String>,
    #[serde(rename = "TerminationProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
    #[serde(rename = "UnhealthyNodeReplacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_node_replacement: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupDetail {
    #[serde(rename = "BidPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    #[serde(rename = "InstanceRequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_request_count: Option<i32>,
    #[serde(rename = "InstanceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    #[serde(rename = "InstanceRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_running_count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LastStateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[serde(rename = "Market")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementType {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepDetail {
    #[serde(rename = "ExecutionStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status_detail: Option<StepExecutionStatusDetail>,
    #[serde(rename = "StepConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_config: Option<StepConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepExecutionStatusDetail {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "LastStateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotebookExecutionInput {
    #[serde(rename = "NotebookExecutionId")]
    #[serde(default)]
    pub notebook_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotebookExecutionOutput {
    #[serde(rename = "NotebookExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution: Option<NotebookExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookExecution {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "EditorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExecutionEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine: Option<ExecutionEngineConfig>,
    #[serde(rename = "LastStateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[serde(rename = "NotebookExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_id: Option<String>,
    #[serde(rename = "NotebookExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_name: Option<String>,
    #[serde(rename = "NotebookInstanceSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_security_group_id: Option<String>,
    #[serde(rename = "NotebookParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_params: Option<String>,
    #[serde(rename = "NotebookS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_s3_location: Option<NotebookS3LocationForOutput>,
    #[serde(rename = "OutputNotebookFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_notebook_format: Option<String>,
    #[serde(rename = "OutputNotebookS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_notebook_s3_location: Option<OutputNotebookS3LocationForOutput>,
    #[serde(rename = "OutputNotebookURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_notebook_u_r_i: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionEngineConfig {
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "MasterInstanceSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_security_group_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookS3LocationForOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputNotebookS3LocationForOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePersistentAppUIInput {
    #[serde(rename = "PersistentAppUIId")]
    #[serde(default)]
    pub persistent_app_u_i_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePersistentAppUIOutput {
    #[serde(rename = "PersistentAppUI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i: Option<PersistentAppUI>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersistentAppUI {
    #[serde(rename = "AuthorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LastStateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[serde(rename = "PersistentAppUIId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i_id: Option<String>,
    #[serde(rename = "PersistentAppUIStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i_status: Option<String>,
    #[serde(rename = "PersistentAppUITypeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i_type_list: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReleaseLabelInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReleaseLabelOutput {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<SimplifiedApplication>>,
    #[serde(rename = "AvailableOSReleases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_o_s_releases: Option<Vec<OSRelease>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimplifiedApplication {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OSRelease {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityConfigurationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityConfigurationOutput {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStepInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "StepId")]
    #[serde(default)]
    pub step_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStepOutput {
    #[serde(rename = "Step")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<Step>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Step {
    #[serde(rename = "ActionOnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StepStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HadoopStepConfig {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Jar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jar: Option<String>,
    #[serde(rename = "MainClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepStatus {
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<StepStateChangeReason>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<StepTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureDetails {
    #[serde(rename = "LogFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepTimeline {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStudioInput {
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStudioOutput {
    #[serde(rename = "Studio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<Studio>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Studio {
    #[serde(rename = "AuthMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DefaultS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_s3_location: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EngineSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_security_group_id: Option<String>,
    #[serde(rename = "IdcInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_instance_arn: Option<String>,
    #[serde(rename = "IdcUserAssignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_user_assignment: Option<String>,
    #[serde(rename = "IdpAuthUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_auth_url: Option<String>,
    #[serde(rename = "IdpRelayStateParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_relay_state_parameter_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "StudioArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_arn: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrustedIdentityPropagationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_identity_propagation_enabled: Option<bool>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UserRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoTerminationPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoTerminationPolicyOutput {
    #[serde(rename = "AutoTerminationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_termination_policy: Option<AutoTerminationPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTerminationPolicy {
    #[serde(rename = "IdleTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlockPublicAccessConfigurationInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlockPublicAccessConfigurationOutput {
    #[serde(rename = "BlockPublicAccessConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access_configuration: Option<BlockPublicAccessConfiguration>,
    #[serde(rename = "BlockPublicAccessConfigurationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access_configuration_metadata: Option<BlockPublicAccessConfigurationMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockPublicAccessConfiguration {
    #[serde(rename = "BlockPublicSecurityGroupRules")]
    #[serde(default)]
    pub block_public_security_group_rules: bool,
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "PermittedPublicSecurityGroupRuleRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted_public_security_group_rule_ranges: Option<Vec<PortRange>>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRange {
    #[serde(rename = "MaxRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range: Option<i32>,
    #[serde(rename = "MinRange")]
    #[serde(default)]
    pub min_range: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockPublicAccessConfigurationMetadata {
    #[serde(rename = "CreatedByArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_arn: Option<String>,
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterSessionCredentialsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterSessionCredentialsOutput {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "ExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    #[serde(rename = "UsernamePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_password: Option<UsernamePassword>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsernamePassword {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedScalingPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedScalingPolicyOutput {
    #[serde(rename = "ManagedScalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling_policy: Option<ManagedScalingPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedScalingPolicy {
    #[serde(rename = "ComputeLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_limits: Option<ComputeLimits>,
    #[serde(rename = "ScalingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_strategy: Option<String>,
    #[serde(rename = "UtilizationPerformanceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_performance_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeLimits {
    #[serde(rename = "MaximumCapacityUnits")]
    #[serde(default)]
    pub maximum_capacity_units: i32,
    #[serde(rename = "MaximumCoreCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_core_capacity_units: Option<i32>,
    #[serde(rename = "MaximumOnDemandCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_on_demand_capacity_units: Option<i32>,
    #[serde(rename = "MinimumCapacityUnits")]
    #[serde(default)]
    pub minimum_capacity_units: i32,
    #[serde(rename = "UnitType")]
    #[serde(default)]
    pub unit_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOnClusterAppUIPresignedURLInput {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "OnClusterAppUIType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_cluster_app_u_i_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOnClusterAppUIPresignedURLOutput {
    #[serde(rename = "PresignedURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_u_r_l: Option<String>,
    #[serde(rename = "PresignedURLReady")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_u_r_l_ready: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersistentAppUIPresignedURLInput {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AuthProxyCall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_proxy_call: Option<bool>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "PersistentAppUIId")]
    #[serde(default)]
    pub persistent_app_u_i_id: String,
    #[serde(rename = "PersistentAppUIType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersistentAppUIPresignedURLOutput {
    #[serde(rename = "PresignedURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_u_r_l: Option<String>,
    #[serde(rename = "PresignedURLReady")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_u_r_l_ready: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStudioSessionMappingInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStudioSessionMappingOutput {
    #[serde(rename = "SessionMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_mapping: Option<SessionMappingDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionMappingDetail {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "SessionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_policy_arn: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBootstrapActionsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBootstrapActionsOutput {
    #[serde(rename = "BootstrapActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<Command>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Command {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ScriptPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersInput {
    #[serde(rename = "ClusterStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_states: Option<Vec<String>>,
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersOutput {
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ClusterSummary>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSummary {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i32>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceFleetsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceFleetsOutput {
    #[serde(rename = "InstanceFleets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleet>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleet {
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceFleetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_type: Option<String>,
    #[serde(rename = "InstanceTypeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_specifications: Option<Vec<InstanceTypeSpecification>>,
    #[serde(rename = "LaunchSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProvisionedOnDemandCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_on_demand_capacity: Option<i32>,
    #[serde(rename = "ProvisionedSpotCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_spot_capacity: Option<i32>,
    #[serde(rename = "ResizeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_specifications: Option<InstanceFleetResizingSpecifications>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceFleetStatus>,
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i32>,
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTypeSpecification {
    #[serde(rename = "BidPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsBlockDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsBlockDevice {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "VolumeSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_specification: Option<VolumeSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetStatus {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceFleetStateChangeReason>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceFleetTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetTimeline {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceGroupsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceGroupsOutput {
    #[serde(rename = "InstanceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroup>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroup {
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    #[serde(rename = "BidPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "ConfigurationsVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations_version: Option<i64>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsBlockDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceGroupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_type: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LastSuccessfullyAppliedConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successfully_applied_configurations: Option<Vec<Configuration>>,
    #[serde(rename = "LastSuccessfullyAppliedConfigurationsVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successfully_applied_configurations_version: Option<i64>,
    #[serde(rename = "Market")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestedInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_instance_count: Option<i32>,
    #[serde(rename = "RunningInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_instance_count: Option<i32>,
    #[serde(rename = "ShrinkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceGroupStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicyDescription {
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<ScalingConstraints>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ScalingRule>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutoScalingPolicyStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicyStatus {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<AutoScalingPolicyStateChangeReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicyStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShrinkPolicy {
    #[serde(rename = "DecommissionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decommission_timeout: Option<i32>,
    #[serde(rename = "InstanceResizePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_resize_policy: Option<InstanceResizePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceResizePolicy {
    #[serde(rename = "InstanceTerminationTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_termination_timeout: Option<i32>,
    #[serde(rename = "InstancesToProtect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_protect: Option<Vec<String>>,
    #[serde(rename = "InstancesToTerminate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_terminate: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupStatus {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceGroupStateChangeReason>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceGroupTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupTimeline {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "InstanceFleetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
    #[serde(rename = "InstanceFleetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_type: Option<String>,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    #[serde(rename = "InstanceGroupTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_types: Option<Vec<String>>,
    #[serde(rename = "InstanceStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_states: Option<Vec<String>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesOutput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instance {
    #[serde(rename = "EbsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<Vec<EbsVolume>>,
    #[serde(rename = "Ec2InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceFleetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Market")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "PrivateDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "PublicDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    #[serde(rename = "PublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsVolume {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceStatus {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceStateChangeReason>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceStateChangeReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTimeline {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "ReadyDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookExecutionsInput {
    #[serde(rename = "EditorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<String>,
    #[serde(rename = "ExecutionEngineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine_id: Option<String>,
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookExecutionsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NotebookExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_executions: Option<Vec<NotebookExecutionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookExecutionSummary {
    #[serde(rename = "EditorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ExecutionEngineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine_id: Option<String>,
    #[serde(rename = "NotebookExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_id: Option<String>,
    #[serde(rename = "NotebookExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_name: Option<String>,
    #[serde(rename = "NotebookS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_s3_location: Option<NotebookS3LocationForOutput>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReleaseLabelsInput {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ReleaseLabelFilter>,
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
pub struct ReleaseLabelFilter {
    #[serde(rename = "Application")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReleaseLabelsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReleaseLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityConfigurationsInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityConfigurationsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SecurityConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfigurationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfigurationSummary {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStepsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "StepIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_ids: Option<Vec<String>>,
    #[serde(rename = "StepStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStepsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepSummary {
    #[serde(rename = "ActionOnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StepStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStudioSessionMappingsInput {
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStudioSessionMappingsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SessionMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_mappings: Option<Vec<SessionMappingSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionMappingSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "SessionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_policy_arn: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStudiosInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStudiosOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Studios")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<StudioSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StudioSummary {
    #[serde(rename = "AuthMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSupportedInstanceTypesInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReleaseLabel")]
    #[serde(default)]
    pub release_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSupportedInstanceTypesOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SupportedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_instance_types: Option<Vec<SupportedInstanceType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedInstanceType {
    #[serde(rename = "Architecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "EbsOptimizedAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized_available: Option<bool>,
    #[serde(rename = "EbsOptimizedByDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized_by_default: Option<bool>,
    #[serde(rename = "EbsStorageOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_storage_only: Option<bool>,
    #[serde(rename = "InstanceFamilyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family_id: Option<String>,
    #[serde(rename = "Is64BitsOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is64_bits_only: Option<bool>,
    #[serde(rename = "MemoryGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_g_b: Option<f32>,
    #[serde(rename = "NumberOfDisks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_disks: Option<i32>,
    #[serde(rename = "StorageGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_g_b: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VCPU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_c_p_u: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClusterInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "ExtendedSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_support: Option<bool>,
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClusterOutput {
    #[serde(rename = "ExtendedSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_support: Option<bool>,
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyInstanceFleetInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "InstanceFleet")]
    #[serde(default)]
    pub instance_fleet: InstanceFleetModifyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceFleetModifyConfig {
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "InstanceFleetId")]
    #[serde(default)]
    pub instance_fleet_id: String,
    #[serde(rename = "InstanceTypeConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    #[serde(rename = "ResizeSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_specifications: Option<InstanceFleetResizingSpecifications>,
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i32>,
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyInstanceGroupsInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "InstanceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupModifyConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGroupModifyConfig {
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "EC2InstanceIdsToTerminate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_instance_ids_to_terminate: Option<Vec<String>>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    pub instance_group_id: String,
    #[serde(rename = "ReconfigurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconfiguration_type: Option<String>,
    #[serde(rename = "ShrinkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAutoScalingPolicyInput {
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(default)]
    pub auto_scaling_policy: AutoScalingPolicy,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    pub instance_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAutoScalingPolicyOutput {
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAutoTerminationPolicyInput {
    #[serde(rename = "AutoTerminationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_termination_policy: Option<AutoTerminationPolicy>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAutoTerminationPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBlockPublicAccessConfigurationInput {
    #[serde(rename = "BlockPublicAccessConfiguration")]
    #[serde(default)]
    pub block_public_access_configuration: BlockPublicAccessConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBlockPublicAccessConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedScalingPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "ManagedScalingPolicy")]
    #[serde(default)]
    pub managed_scaling_policy: ManagedScalingPolicy,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedScalingPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAutoScalingPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "InstanceGroupId")]
    #[serde(default)]
    pub instance_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAutoScalingPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAutoTerminationPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAutoTerminationPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveManagedScalingPolicyInput {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveManagedScalingPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsInput {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunJobFlowInput {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "AmiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_version: Option<String>,
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "AutoScalingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[serde(rename = "AutoTerminationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_termination_policy: Option<AutoTerminationPolicy>,
    #[serde(rename = "BootstrapActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionConfig>>,
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "CustomAmiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "EbsRootVolumeIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_iops: Option<i32>,
    #[serde(rename = "EbsRootVolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_size: Option<i32>,
    #[serde(rename = "EbsRootVolumeThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_throughput: Option<i32>,
    #[serde(rename = "ExtendedSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_support: Option<bool>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    pub instances: JobFlowInstancesConfig,
    #[serde(rename = "JobFlowRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_role: Option<String>,
    #[serde(rename = "KerberosAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_attributes: Option<KerberosAttributes>,
    #[serde(rename = "LogEncryptionKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_encryption_kms_key_id: Option<String>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "ManagedScalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling_policy: Option<ManagedScalingPolicy>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NewSupportedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_supported_products: Option<Vec<SupportedProductConfig>>,
    #[serde(rename = "OSReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_s_release_label: Option<String>,
    #[serde(rename = "PlacementGroupConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_configs: Option<Vec<PlacementGroupConfig>>,
    #[serde(rename = "ReleaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "RepoUpgradeOnBoot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i32>,
    #[serde(rename = "StepExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_execution_role_arn: Option<String>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepConfig>>,
    #[serde(rename = "SupportedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobFlowInstancesConfig {
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    #[serde(rename = "Ec2KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_name: Option<String>,
    #[serde(rename = "Ec2SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_subnet_id: Option<String>,
    #[serde(rename = "Ec2SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_subnet_ids: Option<Vec<String>>,
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    #[serde(rename = "HadoopVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hadoop_version: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceFleets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleetConfig>>,
    #[serde(rename = "InstanceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupConfig>>,
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    #[serde(rename = "MasterInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_type: Option<String>,
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementType>,
    #[serde(rename = "ServiceAccessSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_security_group: Option<String>,
    #[serde(rename = "SlaveInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_instance_type: Option<String>,
    #[serde(rename = "TerminationProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
    #[serde(rename = "UnhealthyNodeReplacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_node_replacement: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedProductConfig {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunJobFlowOutput {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "JobFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetKeepJobFlowAliveWhenNoStepsInput {
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    #[serde(default)]
    pub keep_job_flow_alive_when_no_steps: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetTerminationProtectionInput {
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
    #[serde(rename = "TerminationProtected")]
    #[serde(default)]
    pub termination_protected: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUnhealthyNodeReplacementInput {
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
    #[serde(rename = "UnhealthyNodeReplacement")]
    #[serde(default)]
    pub unhealthy_node_replacement: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetVisibleToAllUsersInput {
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(default)]
    pub visible_to_all_users: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartNotebookExecutionInput {
    #[serde(rename = "EditorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_id: Option<String>,
    #[serde(rename = "EnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExecutionEngine")]
    #[serde(default)]
    pub execution_engine: ExecutionEngineConfig,
    #[serde(rename = "NotebookExecutionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_name: Option<String>,
    #[serde(rename = "NotebookInstanceSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_security_group_id: Option<String>,
    #[serde(rename = "NotebookParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_params: Option<String>,
    #[serde(rename = "NotebookS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_s3_location: Option<NotebookS3LocationFromInput>,
    #[serde(rename = "OutputNotebookFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_notebook_format: Option<String>,
    #[serde(rename = "OutputNotebookS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_notebook_s3_location: Option<OutputNotebookS3LocationFromInput>,
    #[serde(rename = "RelativePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    pub service_role: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookS3LocationFromInput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputNotebookS3LocationFromInput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartNotebookExecutionOutput {
    #[serde(rename = "NotebookExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopNotebookExecutionInput {
    #[serde(rename = "NotebookExecutionId")]
    #[serde(default)]
    pub notebook_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateJobFlowsInput {
    #[serde(rename = "JobFlowIds")]
    #[serde(default)]
    pub job_flow_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStudioInput {
    #[serde(rename = "DefaultS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_s3_location: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStudioSessionMappingInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "SessionPolicyArn")]
    #[serde(default)]
    pub session_policy_arn: String,
    #[serde(rename = "StudioId")]
    #[serde(default)]
    pub studio_id: String,
}
