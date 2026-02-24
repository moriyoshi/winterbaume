//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ecs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapacityProviderRequest {
    #[serde(rename = "autoScalingGroupProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_provider: Option<AutoScalingGroupProvider>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "managedInstancesProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_instances_provider: Option<CreateManagedInstancesProviderConfiguration>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroupProvider {
    #[serde(rename = "autoScalingGroupArn")]
    #[serde(default)]
    pub auto_scaling_group_arn: String,
    #[serde(rename = "managedDraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_draining: Option<String>,
    #[serde(rename = "managedScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling: Option<ManagedScaling>,
    #[serde(rename = "managedTerminationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_termination_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedScaling {
    #[serde(rename = "instanceWarmupPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_warmup_period: Option<i32>,
    #[serde(rename = "maximumScalingStepSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scaling_step_size: Option<i32>,
    #[serde(rename = "minimumScalingStepSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_scaling_step_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateManagedInstancesProviderConfiguration {
    #[serde(rename = "autoRepairConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_configuration: Option<AutoRepairConfiguration>,
    #[serde(rename = "infrastructureOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_optimization: Option<InfrastructureOptimization>,
    #[serde(rename = "infrastructureRoleArn")]
    #[serde(default)]
    pub infrastructure_role_arn: String,
    #[serde(rename = "instanceLaunchTemplate")]
    #[serde(default)]
    pub instance_launch_template: InstanceLaunchTemplate,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoRepairConfiguration {
    #[serde(rename = "actionsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InfrastructureOptimization {
    #[serde(rename = "scaleInAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_after: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceLaunchTemplate {
    #[serde(rename = "capacityOptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_option_type: Option<String>,
    #[serde(rename = "capacityReservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservations: Option<CapacityReservationRequest>,
    #[serde(rename = "ec2InstanceProfileArn")]
    #[serde(default)]
    pub ec2_instance_profile_arn: String,
    #[serde(rename = "fipsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fips_enabled: Option<bool>,
    #[serde(rename = "instanceMetadataTagsPropagation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_tags_propagation: Option<bool>,
    #[serde(rename = "instanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,
    #[serde(rename = "localStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage_configuration: Option<ManagedInstancesLocalStorageConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    pub network_configuration: ManagedInstancesNetworkConfiguration,
    #[serde(rename = "storageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<ManagedInstancesStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityReservationRequest {
    #[serde(rename = "reservationGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_group_arn: Option<String>,
    #[serde(rename = "reservationPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_preference: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceRequirementsRequest {
    #[serde(rename = "acceleratorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_count: Option<AcceleratorCountRequest>,
    #[serde(rename = "acceleratorManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_manufacturers: Option<Vec<String>>,
    #[serde(rename = "acceleratorNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "acceleratorTotalMemoryMiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,
    #[serde(rename = "acceleratorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "allowedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "bareMetal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bare_metal: Option<String>,
    #[serde(rename = "baselineEbsBandwidthMbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,
    #[serde(rename = "burstablePerformance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "cpuManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "excludedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "instanceGenerations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_generations: Option<Vec<String>>,
    #[serde(rename = "localStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<String>,
    #[serde(rename = "localStorageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
    #[serde(rename = "memoryGiBPerVCpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_gi_b_per_v_cpu: Option<MemoryGiBPerVCpuRequest>,
    #[serde(rename = "memoryMiB")]
    #[serde(default)]
    pub memory_mi_b: MemoryMiBRequest,
    #[serde(rename = "networkBandwidthGbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,
    #[serde(rename = "networkInterfaceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "requireHibernateSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "totalLocalStorageGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_local_storage_g_b: Option<TotalLocalStorageGBRequest>,
    #[serde(rename = "vCpuCount")]
    #[serde(default)]
    pub v_cpu_count: VCpuCountRangeRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceleratorCountRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceleratorTotalMemoryMiBRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaselineEbsBandwidthMbpsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemoryGiBPerVCpuRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemoryMiBRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    pub min: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkBandwidthGbpsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterfaceCountRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalLocalStorageGBRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VCpuCountRangeRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default)]
    pub min: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedInstancesLocalStorageConfiguration {
    #[serde(rename = "useLocalStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_local_storage: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedInstancesNetworkConfiguration {
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedInstancesStorageConfiguration {
    #[serde(rename = "storageSizeGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size_gi_b: Option<i32>,
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
pub struct CreateCapacityProviderResponse {
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProvider {
    #[serde(rename = "autoScalingGroupProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_provider: Option<AutoScalingGroupProvider>,
    #[serde(rename = "capacityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "managedInstancesProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_instances_provider: Option<ManagedInstancesProvider>,
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
    #[serde(rename = "updateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    #[serde(rename = "updateStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedInstancesProvider {
    #[serde(rename = "autoRepairConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_configuration: Option<AutoRepairConfiguration>,
    #[serde(rename = "infrastructureOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_optimization: Option<InfrastructureOptimization>,
    #[serde(rename = "infrastructureRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_role_arn: Option<String>,
    #[serde(rename = "instanceLaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_launch_template: Option<InstanceLaunchTemplate>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    #[serde(rename = "defaultCapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "serviceConnectDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_defaults: Option<ClusterServiceConnectDefaultsRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterConfiguration {
    #[serde(rename = "executeCommandConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_configuration: Option<ExecuteCommandConfiguration>,
    #[serde(rename = "managedStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_storage_configuration: Option<ManagedStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCommandConfiguration {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<ExecuteCommandLogConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCommandLogConfiguration {
    #[serde(rename = "cloudWatchEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_enabled: Option<bool>,
    #[serde(rename = "cloudWatchLogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "s3EncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_enabled: Option<bool>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedStorageConfiguration {
    #[serde(rename = "fargateEphemeralStorageKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_ephemeral_storage_kms_key_id: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
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
pub struct ClusterServiceConnectDefaultsRequest {
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "activeServicesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "attachmentsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_status: Option<String>,
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    #[serde(rename = "defaultCapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "pendingTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i32>,
    #[serde(rename = "registeredContainerInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i32>,
    #[serde(rename = "runningTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,
    #[serde(rename = "serviceConnectDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_defaults: Option<ClusterServiceConnectDefaults>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValuePair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterServiceConnectDefaults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDaemonRequest {
    #[serde(rename = "capacityProviderArns")]
    #[serde(default)]
    pub capacity_provider_arns: Vec<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "daemonName")]
    #[serde(default)]
    pub daemon_name: String,
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    pub daemon_task_definition_arn: String,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DaemonDeploymentConfiguration>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeploymentConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<DaemonAlarmConfiguration>,
    #[serde(rename = "bakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake_time_in_minutes: Option<i32>,
    #[serde(rename = "drainPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drain_percent: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonAlarmConfiguration {
    #[serde(rename = "alarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDaemonResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "deploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExpressGatewayServiceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "healthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "infrastructureRoleArn")]
    #[serde(default)]
    pub infrastructure_role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<ExpressGatewayServiceNetworkConfiguration>,
    #[serde(rename = "primaryContainer")]
    #[serde(default)]
    pub primary_container: ExpressGatewayContainer,
    #[serde(rename = "scalingTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_target: Option<ExpressGatewayScalingTarget>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayServiceNetworkConfiguration {
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayContainer {
    #[serde(rename = "awsLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logs_configuration: Option<ExpressGatewayServiceAwsLogsConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "containerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    pub image: String,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<ExpressGatewayRepositoryCredentials>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayServiceAwsLogsConfiguration {
    #[serde(rename = "logGroup")]
    #[serde(default)]
    pub log_group: String,
    #[serde(rename = "logStreamPrefix")]
    #[serde(default)]
    pub log_stream_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayRepositoryCredentials {
    #[serde(rename = "credentialsParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_parameter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Secret {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "valueFrom")]
    #[serde(default)]
    pub value_from: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayScalingTarget {
    #[serde(rename = "autoScalingMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_metric: Option<String>,
    #[serde(rename = "autoScalingTargetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_target_value: Option<i32>,
    #[serde(rename = "maxTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_task_count: Option<i32>,
    #[serde(rename = "minTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_task_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExpressGatewayServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ECSExpressGatewayService>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ECSExpressGatewayService {
    #[serde(rename = "activeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_configurations: Option<Vec<ExpressGatewayServiceConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "currentDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deployment: Option<String>,
    #[serde(rename = "infrastructureRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_role_arn: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExpressGatewayServiceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayServiceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "healthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "ingressPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_paths: Option<Vec<IngressPathSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<ExpressGatewayServiceNetworkConfiguration>,
    #[serde(rename = "primaryContainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_container: Option<ExpressGatewayContainer>,
    #[serde(rename = "scalingTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_target: Option<ExpressGatewayScalingTarget>,
    #[serde(rename = "serviceRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_revision_arn: Option<String>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngressPathSummary {
    #[serde(rename = "accessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressGatewayServiceStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceRequest {
    #[serde(rename = "availabilityZoneRebalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_rebalancing: Option<String>,
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "deploymentController")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    #[serde(rename = "desiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i32>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "placementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    #[serde(rename = "serviceConnectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<ServiceVolumeConfiguration>>,
    #[serde(rename = "vpcLatticeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice_configurations: Option<Vec<VpcLatticeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<DeploymentAlarms>,
    #[serde(rename = "bakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake_time_in_minutes: Option<i32>,
    #[serde(rename = "canaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_configuration: Option<CanaryConfiguration>,
    #[serde(rename = "deploymentCircuitBreaker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_circuit_breaker: Option<DeploymentCircuitBreaker>,
    #[serde(rename = "lifecycleHooks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hooks: Option<Vec<DeploymentLifecycleHook>>,
    #[serde(rename = "linearConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_configuration: Option<LinearConfiguration>,
    #[serde(rename = "maximumPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<i32>,
    #[serde(rename = "minimumHealthyPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_percent: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentAlarms {
    #[serde(rename = "alarmNames")]
    #[serde(default)]
    pub alarm_names: Vec<String>,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub rollback: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryConfiguration {
    #[serde(rename = "canaryBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "canaryPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_percent: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentCircuitBreaker {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub rollback: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentLifecycleHook {
    #[serde(rename = "hookDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_details: Option<serde_json::Value>,
    #[serde(rename = "hookTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_target_arn: Option<String>,
    #[serde(rename = "lifecycleStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stages: Option<Vec<String>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinearConfiguration {
    #[serde(rename = "stepBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "stepPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_percent: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentController {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancer {
    #[serde(rename = "advancedConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_configuration: Option<AdvancedConfiguration>,
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "containerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "loadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "targetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedConfiguration {
    #[serde(rename = "alternateTargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_target_group_arn: Option<String>,
    #[serde(rename = "productionListenerRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_listener_rule: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "testListenerRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_listener_rule: Option<String>,
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
    #[serde(rename = "assignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
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
pub struct ServiceConnectConfiguration {
    #[serde(rename = "accessLogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_configuration: Option<ServiceConnectAccessLogConfiguration>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceConnectService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectAccessLogConfiguration {
    #[serde(default)]
    pub format: String,
    #[serde(rename = "includeQueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_query_parameters: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfiguration {
    #[serde(rename = "logDriver")]
    #[serde(default)]
    pub log_driver: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secretOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectService {
    #[serde(rename = "clientAliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_aliases: Option<Vec<ServiceConnectClientAlias>>,
    #[serde(rename = "discoveryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_name: Option<String>,
    #[serde(rename = "ingressPortOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_port_override: Option<i32>,
    #[serde(rename = "portName")]
    #[serde(default)]
    pub port_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeoutConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ServiceConnectTlsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectClientAlias {
    #[serde(rename = "dnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "testTrafficRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_traffic_rules: Option<ServiceConnectTestTrafficRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectTestTrafficRules {
    #[serde(default)]
    pub header: ServiceConnectTestTrafficHeaderRules,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectTestTrafficHeaderRules {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ServiceConnectTestTrafficHeaderMatchRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectTestTrafficHeaderMatchRules {
    #[serde(default)]
    pub exact: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeoutConfiguration {
    #[serde(rename = "idleTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<i32>,
    #[serde(rename = "perRequestTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request_timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectTlsConfiguration {
    #[serde(rename = "issuerCertificateAuthority")]
    #[serde(default)]
    pub issuer_certificate_authority: ServiceConnectTlsCertificateAuthority,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectTlsCertificateAuthority {
    #[serde(rename = "awsPcaAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_pca_authority_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceRegistry {
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "containerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "registryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceVolumeConfiguration {
    #[serde(rename = "managedEBSVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_e_b_s_volume: Option<ServiceManagedEBSVolumeConfiguration>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceManagedEBSVolumeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "filesystemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gi_b: Option<i32>,
    #[serde(rename = "snapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "tagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<Vec<EBSTagSpecification>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "volumeInitializationRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_initialization_rate: Option<i32>,
    #[serde(rename = "volumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSTagSpecification {
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcLatticeConfiguration {
    #[serde(rename = "portName")]
    #[serde(default)]
    pub port_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "targetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "availabilityZoneRebalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_rebalancing: Option<String>,
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "currentServiceDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_service_deployment: Option<String>,
    #[serde(rename = "currentServiceRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_service_revisions: Option<Vec<ServiceCurrentRevisionSummary>>,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "deploymentController")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    #[serde(rename = "desiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ServiceEvent>>,
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i32>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "pendingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "placementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "platformFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "resourceManagementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_management_type: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "runningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    #[serde(rename = "schedulingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "taskSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<TaskSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceCurrentRevisionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "pendingTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_task_count: Option<i32>,
    #[serde(rename = "requestedTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_task_count: Option<i32>,
    #[serde(rename = "runningTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_task_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "desiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[serde(rename = "failedTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tasks: Option<i32>,
    #[serde(rename = "fargateEphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_ephemeral_storage: Option<DeploymentEphemeralStorage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "pendingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "platformFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "rolloutState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state: Option<String>,
    #[serde(rename = "rolloutStateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_state_reason: Option<String>,
    #[serde(rename = "runningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    #[serde(rename = "serviceConnectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,
    #[serde(rename = "serviceConnectResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_resources: Option<Vec<ServiceConnectServiceResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<ServiceVolumeConfiguration>>,
    #[serde(rename = "vpcLatticeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice_configurations: Option<Vec<VpcLatticeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentEphemeralStorage {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceConnectServiceResource {
    #[serde(rename = "discoveryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_arn: Option<String>,
    #[serde(rename = "discoveryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceEvent {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSet {
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "computedDesiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_desired_count: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "fargateEphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_ephemeral_storage: Option<DeploymentEphemeralStorage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "pendingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "platformFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "runningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(rename = "stabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status: Option<String>,
    #[serde(rename = "stabilityStatusAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status_at: Option<f64>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "taskSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scale {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskSetRequest {
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub cluster: String,
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    pub task_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskSetResponse {
    #[serde(rename = "taskSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountSettingRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountSettingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Setting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttributesRequest {
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "targetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttributesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityProviderRequest {
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    pub capacity_provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityProviderResponse {
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(default)]
    pub cluster: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDaemonRequest {
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    pub daemon_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDaemonResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "deploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDaemonTaskDefinitionRequest {
    #[serde(rename = "daemonTaskDefinition")]
    #[serde(default)]
    pub daemon_task_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDaemonTaskDefinitionResponse {
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definition_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExpressGatewayServiceRequest {
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExpressGatewayServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ECSExpressGatewayService>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default)]
    pub service: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskDefinitionsRequest {
    #[serde(rename = "taskDefinitions")]
    #[serde(default)]
    pub task_definitions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskDefinitionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "taskDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Failure {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskDefinition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilities: Option<Vec<String>>,
    #[serde(rename = "containerDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "deleteRequestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_requested_at: Option<f64>,
    #[serde(rename = "deregisteredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistered_at: Option<f64>,
    #[serde(rename = "enableFaultInjection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_fault_injection: Option<bool>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "inferenceAccelerators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    #[serde(rename = "ipcMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "networkMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(rename = "pidMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    #[serde(rename = "proxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    #[serde(rename = "requiresAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_attributes: Option<Vec<Attribute>>,
    #[serde(rename = "requiresCompatibilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerDefinition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "credentialSpecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_specs: Option<Vec<String>>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<ContainerDependency>>,
    #[serde(rename = "disableNetworking")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,
    #[serde(rename = "dnsSearchDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    #[serde(rename = "dnsServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(rename = "dockerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "dockerSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "environmentFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "extraHosts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<HostEntry>>,
    #[serde(rename = "firelensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "memoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "portMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "pseudoTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(rename = "restartPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(rename = "startTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i32>,
    #[serde(rename = "stopTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i32>,
    #[serde(rename = "systemControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<SystemControl>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "versionConsistency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_consistency: Option<String>,
    #[serde(rename = "volumesFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<VolumeFrom>>,
    #[serde(rename = "workingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerDependency {
    #[serde(default)]
    pub condition: String,
    #[serde(rename = "containerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentFile {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostEntry {
    #[serde(default)]
    pub hostname: String,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    pub ip_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirelensConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheck {
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "startPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinuxParameters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KernelCapabilities>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "initProcessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    #[serde(rename = "maxSwap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i32>,
    #[serde(rename = "sharedMemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KernelCapabilities {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "hostPath")]
    #[serde(default)]
    pub host_path: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tmpfs {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    pub container_path: String,
    #[serde(rename = "mountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    #[serde(default)]
    pub size: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MountPoint {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "sourceVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortMapping {
    #[serde(rename = "appProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
    #[serde(rename = "containerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "containerPortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port_range: Option<String>,
    #[serde(rename = "hostPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryCredentials {
    #[serde(rename = "credentialsParameter")]
    #[serde(default)]
    pub credentials_parameter: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRequirement {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerRestartPolicy {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "ignoredExitCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_exit_codes: Option<Vec<i32>>,
    #[serde(rename = "restartAttemptPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_attempt_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SystemControl {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ulimit {
    #[serde(rename = "hardLimit")]
    #[serde(default)]
    pub hard_limit: i32,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "softLimit")]
    #[serde(default)]
    pub soft_limit: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeFrom {
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "sourceContainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EphemeralStorage {
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    pub size_in_gi_b: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceAccelerator {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    pub device_name: String,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    pub device_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskDefinitionPlacementConstraint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfiguration {
    #[serde(rename = "containerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<KeyValuePair>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimePlatform {
    #[serde(rename = "cpuArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_architecture: Option<String>,
    #[serde(rename = "operatingSystemFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Volume {
    #[serde(rename = "configuredAtLaunch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_at_launch: Option<bool>,
    #[serde(rename = "dockerVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,
    #[serde(rename = "efsVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<EFSVolumeConfiguration>,
    #[serde(rename = "fsxWindowsFileServerVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_windows_file_server_volume_configuration:
        Option<FSxWindowsFileServerVolumeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostVolumeProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "s3filesVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3files_volume_configuration: Option<S3FilesVolumeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DockerVolumeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "driverOpts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EFSVolumeConfiguration {
    #[serde(rename = "authorizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<EFSAuthorizationConfig>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    #[serde(rename = "transitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,
    #[serde(rename = "transitEncryptionPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EFSAuthorizationConfig {
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FSxWindowsFileServerVolumeConfiguration {
    #[serde(rename = "authorizationConfig")]
    #[serde(default)]
    pub authorization_config: FSxWindowsFileServerAuthorizationConfig,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    pub root_directory: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FSxWindowsFileServerAuthorizationConfig {
    #[serde(rename = "credentialsParameter")]
    #[serde(default)]
    pub credentials_parameter: String,
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostVolumeProperties {
    #[serde(rename = "sourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3FilesVolumeConfiguration {
    #[serde(rename = "accessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "fileSystemArn")]
    #[serde(default)]
    pub file_system_arn: String,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    #[serde(rename = "transitEncryptionPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskSetRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "taskSet")]
    #[serde(default)]
    pub task_set: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskSetResponse {
    #[serde(rename = "taskSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterContainerInstanceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    pub container_instance: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterContainerInstanceResponse {
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerInstance {
    #[serde(rename = "agentConnected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_connected: Option<bool>,
    #[serde(rename = "agentUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_update_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "capacityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_name: Option<String>,
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(rename = "ec2InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_id: Option<String>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<ContainerInstanceHealthStatus>,
    #[serde(rename = "pendingTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i32>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_resources: Option<Vec<Resource>>,
    #[serde(rename = "remainingResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_resources: Option<Vec<Resource>>,
    #[serde(rename = "runningTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "versionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerInstanceHealthStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<InstanceHealthCheckResult>>,
    #[serde(rename = "overallStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceHealthCheckResult {
    #[serde(rename = "lastStatusChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "doubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "integerValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i32>,
    #[serde(rename = "longValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stringSetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_set_value: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionInfo {
    #[serde(rename = "agentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_hash: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "dockerVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTaskDefinitionRequest {
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    pub task_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTaskDefinitionResponse {
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCapacityProvidersRequest {
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
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
pub struct DescribeCapacityProvidersResponse {
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<CapacityProvider>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClustersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClustersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContainerInstancesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstances")]
    #[serde(default)]
    pub container_instances: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContainerInstancesResponse {
    #[serde(rename = "containerInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonDeploymentsRequest {
    #[serde(rename = "daemonDeploymentArns")]
    #[serde(default)]
    pub daemon_deployment_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonDeploymentsResponse {
    #[serde(rename = "daemonDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_deployments: Option<Vec<DaemonDeployment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeployment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<DaemonDeploymentAlarms>,
    #[serde(rename = "circuitBreaker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_breaker: Option<DaemonCircuitBreaker>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_deployment_arn: Option<String>,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DaemonDeploymentConfiguration>,
    #[serde(rename = "finishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<DaemonRollback>,
    #[serde(rename = "sourceDaemonRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_daemon_revisions: Option<Vec<DaemonDeploymentRevisionDetail>>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    #[serde(rename = "targetDaemonRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_daemon_revision: Option<DaemonDeploymentRevisionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeploymentAlarms {
    #[serde(rename = "alarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "triggeredAlarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarm_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonCircuitBreaker {
    #[serde(rename = "failureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonRollback {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "rollbackCapacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_capacity_providers: Option<Vec<String>>,
    #[serde(rename = "rollbackTargetDaemonRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_target_daemon_revision_arn: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeploymentRevisionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<DaemonDeploymentCapacityProvider>>,
    #[serde(rename = "totalDrainingInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_draining_instance_count: Option<i32>,
    #[serde(rename = "totalRunningInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_instance_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeploymentCapacityProvider {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "drainingInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draining_instance_count: Option<i32>,
    #[serde(rename = "runningInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_instance_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonRequest {
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    pub daemon_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon: Option<DaemonDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDetail {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "currentRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revisions: Option<Vec<DaemonRevisionDetail>>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "deploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonRevisionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<DaemonCapacityProvider>>,
    #[serde(rename = "totalRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonCapacityProvider {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "runningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonRevisionsRequest {
    #[serde(rename = "daemonRevisionArns")]
    #[serde(default)]
    pub daemon_revision_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonRevisionsResponse {
    #[serde(rename = "daemonRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_revisions: Option<Vec<DaemonRevision>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonRevision {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "containerImages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_images: Option<Vec<DaemonContainerImage>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "daemonRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_revision_arn: Option<String>,
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definition_arn: Option<String>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonContainerImage {
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonTaskDefinitionRequest {
    #[serde(rename = "daemonTaskDefinition")]
    #[serde(default)]
    pub daemon_task_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDaemonTaskDefinitionResponse {
    #[serde(rename = "daemonTaskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definition: Option<DaemonTaskDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonTaskDefinition {
    #[serde(rename = "containerDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<DaemonContainerDefinition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definition_arn: Option<String>,
    #[serde(rename = "deleteRequestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_requested_at: Option<f64>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<DaemonVolume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonContainerDefinition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<ContainerDependency>>,
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "environmentFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "firelensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<DaemonLinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "memoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "pseudoTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "restartPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(rename = "startTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i32>,
    #[serde(rename = "stopTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i32>,
    #[serde(rename = "systemControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<SystemControl>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "workingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonLinuxParameters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KernelCapabilities>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "initProcessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonVolume {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostVolumeProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExpressGatewayServiceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExpressGatewayServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ECSExpressGatewayService>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceDeploymentsRequest {
    #[serde(rename = "serviceDeploymentArns")]
    #[serde(default)]
    pub service_deployment_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceDeploymentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "serviceDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_deployments: Option<Vec<ServiceDeployment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDeployment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<ServiceDeploymentAlarms>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "deploymentCircuitBreaker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_circuit_breaker: Option<ServiceDeploymentCircuitBreaker>,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "finishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<Rollback>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_deployment_arn: Option<String>,
    #[serde(rename = "sourceServiceRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_service_revisions: Option<Vec<ServiceRevisionSummary>>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    #[serde(rename = "targetServiceRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_service_revision: Option<ServiceRevisionSummary>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDeploymentAlarms {
    #[serde(rename = "alarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "triggeredAlarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarm_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDeploymentCircuitBreaker {
    #[serde(rename = "failureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rollback {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "serviceRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_revision_arn: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceRevisionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "pendingTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_task_count: Option<i32>,
    #[serde(rename = "requestedProductionTrafficWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_production_traffic_weight: Option<f64>,
    #[serde(rename = "requestedTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_task_count: Option<i32>,
    #[serde(rename = "requestedTestTrafficWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_test_traffic_weight: Option<f64>,
    #[serde(rename = "runningTaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_task_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceRevisionsRequest {
    #[serde(rename = "serviceRevisionArns")]
    #[serde(default)]
    pub service_revision_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceRevisionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "serviceRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_revisions: Option<Vec<ServiceRevision>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceRevision {
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "containerImages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_images: Option<Vec<ContainerImage>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ecsManagedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_managed_resources: Option<ECSManagedResources>,
    #[serde(rename = "fargateEphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_ephemeral_storage: Option<DeploymentEphemeralStorage>,
    #[serde(rename = "guardDutyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_duty_enabled: Option<bool>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "platformFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "resolvedConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_configuration: Option<ResolvedConfiguration>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceConnectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(rename = "serviceRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_revision_arn: Option<String>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<ServiceVolumeConfiguration>>,
    #[serde(rename = "vpcLatticeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice_configurations: Option<Vec<VpcLatticeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerImage {
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ECSManagedResources {
    #[serde(rename = "autoScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<ManagedAutoScaling>,
    #[serde(rename = "ingressPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_paths: Option<Vec<ManagedIngressPath>>,
    #[serde(rename = "logGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_groups: Option<Vec<ManagedLogGroup>>,
    #[serde(rename = "metricAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_alarms: Option<Vec<ManagedMetricAlarm>>,
    #[serde(rename = "serviceSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_security_groups: Option<Vec<ManagedSecurityGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedAutoScaling {
    #[serde(rename = "applicationAutoScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_auto_scaling_policies: Option<Vec<ManagedApplicationAutoScalingPolicy>>,
    #[serde(rename = "scalableTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target: Option<ManagedScalableTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedApplicationAutoScalingPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "policyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "targetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedScalableTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "maxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "minCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedIngressPath {
    #[serde(rename = "accessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ManagedCertificate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<ManagedListener>,
    #[serde(rename = "loadBalancer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<ManagedLoadBalancer>,
    #[serde(rename = "loadBalancerSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_security_groups: Option<Vec<ManagedSecurityGroup>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<ManagedListenerRule>,
    #[serde(rename = "targetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<Vec<ManagedTargetGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedCertificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedListener {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedLoadBalancer {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedSecurityGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedListenerRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedTargetGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "healthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "healthCheckPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedLogGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedMetricAlarm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedConfiguration {
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<ServiceRevisionLoadBalancer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceRevisionLoadBalancer {
    #[serde(rename = "productionListenerRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_listener_rule: Option<String>,
    #[serde(rename = "targetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub services: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskDefinitionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    pub task_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskSetsRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "taskSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskSetsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "taskSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<TaskSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTasksRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTasksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Task {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "capacityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_name: Option<String>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<String>,
    #[serde(rename = "connectivityAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_at: Option<f64>,
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "desiredStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionStoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    #[serde(rename = "fargateEphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_ephemeral_storage: Option<TaskEphemeralStorage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "inferenceAccelerators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[serde(rename = "platformFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_family: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "pullStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    #[serde(rename = "pullStoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(rename = "stopCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_code: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    #[serde(rename = "stoppedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<String>,
    #[serde(rename = "stoppingAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "taskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Container {
    #[serde(rename = "containerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "gpuIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_ids: Option<Vec<String>>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "managedAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_agents: Option<Vec<ManagedAgent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "memoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkBindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runtimeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedAgent {
    #[serde(rename = "lastStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started_at: Option<f64>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkBinding {
    #[serde(rename = "bindIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_i_p: Option<String>,
    #[serde(rename = "containerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "containerPortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port_range: Option<String>,
    #[serde(rename = "hostPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    #[serde(rename = "hostPortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port_range: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "attachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "privateIpv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv4_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskEphemeralStorage {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gi_b: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskOverride {
    #[serde(rename = "containerOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<Vec<ContainerOverride>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "inferenceAcceleratorOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerator_overrides: Option<Vec<InferenceAcceleratorOverride>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "environmentFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "memoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceAcceleratorOverride {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverPollEndpointRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverPollEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "serviceConnectEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_endpoint: Option<String>,
    #[serde(rename = "telemetryEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCommandRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default)]
    pub task: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCommandResponse {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "containerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
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
pub struct GetTaskProtectionRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaskProtectionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "protectedTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_tasks: Option<Vec<ProtectedTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectedTask {
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "protectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_enabled: Option<bool>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountSettingsRequest {
    #[serde(rename = "effectiveSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_settings: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountSettingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttributesRequest {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttributesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersRequest {
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
pub struct ListClustersResponse {
    #[serde(rename = "clusterArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arns: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContainerInstancesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
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
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContainerInstancesResponse {
    #[serde(rename = "containerInstanceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arns: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDaemonDeploymentsRequest {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    pub daemon_arn: String,
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
    pub status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatedAt {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDaemonDeploymentsResponse {
    #[serde(rename = "daemonDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_deployments: Option<Vec<DaemonDeploymentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonDeploymentSummary {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "daemonDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_deployment_arn: Option<String>,
    #[serde(rename = "finishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    #[serde(rename = "targetDaemonRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_daemon_revision_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDaemonTaskDefinitionsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "familyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
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
    pub revision: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDaemonTaskDefinitionsResponse {
    #[serde(rename = "daemonTaskDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definitions: Option<Vec<DaemonTaskDefinitionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonTaskDefinitionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "deleteRequestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_requested_at: Option<f64>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDaemonsRequest {
    #[serde(rename = "capacityProviderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_arns: Option<Vec<String>>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
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
pub struct ListDaemonsResponse {
    #[serde(rename = "daemonSummariesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_summaries_list: Option<Vec<DaemonSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DaemonSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDeploymentsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDeploymentsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_deployments: Option<Vec<ServiceDeploymentBrief>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDeploymentBrief {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "finishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_deployment_arn: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "targetServiceRevisionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_service_revision_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesByNamespaceRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesByNamespaceResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceManagementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_management_type: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
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
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskDefinitionFamiliesRequest {
    #[serde(rename = "familyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
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
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskDefinitionFamiliesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskDefinitionsRequest {
    #[serde(rename = "familyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
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
    pub sort: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskDefinitionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taskDefinitionArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
    #[serde(rename = "daemonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_name: Option<String>,
    #[serde(rename = "desiredStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taskArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingDefaultRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingDefaultResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttributesRequest {
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttributesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterCapacityProvidersRequest {
    #[serde(rename = "capacityProviders")]
    #[serde(default)]
    pub capacity_providers: Vec<String>,
    #[serde(default)]
    pub cluster: String,
    #[serde(rename = "defaultCapacityProviderStrategy")]
    #[serde(default)]
    pub default_capacity_provider_strategy: Vec<CapacityProviderStrategyItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterCapacityProvidersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterContainerInstanceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(rename = "instanceIdentityDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document: Option<String>,
    #[serde(rename = "instanceIdentityDocumentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document_signature: Option<String>,
    #[serde(rename = "platformDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_devices: Option<Vec<PlatformDevice>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "totalResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<Vec<Resource>>,
    #[serde(rename = "versionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformDevice {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterContainerInstanceResponse {
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDaemonTaskDefinitionRequest {
    #[serde(rename = "containerDefinitions")]
    #[serde(default)]
    pub container_definitions: Vec<DaemonContainerDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<DaemonVolume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDaemonTaskDefinitionResponse {
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_task_definition_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTaskDefinitionRequest {
    #[serde(rename = "containerDefinitions")]
    #[serde(default)]
    pub container_definitions: Vec<ContainerDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "enableFaultInjection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_fault_injection: Option<bool>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    pub family: String,
    #[serde(rename = "inferenceAccelerators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    #[serde(rename = "ipcMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "networkMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(rename = "pidMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    #[serde(rename = "proxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "requiresCompatibilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTaskDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunTaskRequest {
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "placementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    pub task_definition: String,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<TaskVolumeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskVolumeConfiguration {
    #[serde(rename = "managedEBSVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_e_b_s_volume: Option<TaskManagedEBSVolumeConfiguration>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskManagedEBSVolumeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "filesystemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gi_b: Option<i32>,
    #[serde(rename = "snapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "tagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<Vec<EBSTagSpecification>>,
    #[serde(rename = "terminationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policy: Option<TaskManagedEBSVolumeTerminationPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "volumeInitializationRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_initialization_rate: Option<i32>,
    #[serde(rename = "volumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskManagedEBSVolumeTerminationPolicy {
    #[serde(rename = "deleteOnTermination")]
    #[serde(default)]
    pub delete_on_termination: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunTaskResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstances")]
    #[serde(default)]
    pub container_instances: Vec<String>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    pub task_definition: String,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<TaskVolumeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopServiceDeploymentRequest {
    #[serde(rename = "serviceDeploymentArn")]
    #[serde(default)]
    pub service_deployment_arn: String,
    #[serde(rename = "stopType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopServiceDeploymentResponse {
    #[serde(rename = "serviceDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_deployment_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTaskRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub task: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTaskResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitAttachmentStateChangesRequest {
    #[serde(default)]
    pub attachments: Vec<AttachmentStateChange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentStateChange {
    #[serde(rename = "attachmentArn")]
    #[serde(default)]
    pub attachment_arn: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitAttachmentStateChangesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitContainerStateChangeRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "networkBindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runtimeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitContainerStateChangeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitTaskStateChangeRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentStateChange>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerStateChange>>,
    #[serde(rename = "executionStoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    #[serde(rename = "managedAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_agents: Option<Vec<ManagedAgentStateChange>>,
    #[serde(rename = "pullStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    #[serde(rename = "pullStoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerStateChange {
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "networkBindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runtimeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedAgentStateChange {
    #[serde(rename = "containerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "managedAgentName")]
    #[serde(default)]
    pub managed_agent_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitTaskStateChangeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

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
pub struct UpdateCapacityProviderRequest {
    #[serde(rename = "autoScalingGroupProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_provider: Option<AutoScalingGroupProviderUpdate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "managedInstancesProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_instances_provider: Option<UpdateManagedInstancesProviderConfiguration>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroupProviderUpdate {
    #[serde(rename = "managedDraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_draining: Option<String>,
    #[serde(rename = "managedScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_scaling: Option<ManagedScaling>,
    #[serde(rename = "managedTerminationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_termination_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedInstancesProviderConfiguration {
    #[serde(rename = "autoRepairConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_configuration: Option<AutoRepairConfiguration>,
    #[serde(rename = "infrastructureOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_optimization: Option<InfrastructureOptimization>,
    #[serde(rename = "infrastructureRoleArn")]
    #[serde(default)]
    pub infrastructure_role_arn: String,
    #[serde(rename = "instanceLaunchTemplate")]
    #[serde(default)]
    pub instance_launch_template: InstanceLaunchTemplateUpdate,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceLaunchTemplateUpdate {
    #[serde(rename = "capacityReservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservations: Option<CapacityReservationRequest>,
    #[serde(rename = "ec2InstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_profile_arn: Option<String>,
    #[serde(rename = "instanceMetadataTagsPropagation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_tags_propagation: Option<bool>,
    #[serde(rename = "instanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,
    #[serde(rename = "localStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage_configuration: Option<ManagedInstancesLocalStorageConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<ManagedInstancesNetworkConfiguration>,
    #[serde(rename = "storageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<ManagedInstancesStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapacityProviderResponse {
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<CapacityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClusterConfiguration>,
    #[serde(rename = "serviceConnectDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_defaults: Option<ClusterServiceConnectDefaultsRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<ClusterSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterSettingsRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    pub settings: Vec<ClusterSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterSettingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContainerAgentRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    pub container_instance: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContainerAgentResponse {
    #[serde(rename = "containerInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContainerInstancesStateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "containerInstances")]
    #[serde(default)]
    pub container_instances: Vec<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContainerInstancesStateResponse {
    #[serde(rename = "containerInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDaemonRequest {
    #[serde(rename = "capacityProviderArns")]
    #[serde(default)]
    pub capacity_provider_arns: Vec<String>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    pub daemon_arn: String,
    #[serde(rename = "daemonTaskDefinitionArn")]
    #[serde(default)]
    pub daemon_task_definition_arn: String,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DaemonDeploymentConfiguration>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDaemonResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "daemonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_arn: Option<String>,
    #[serde(rename = "deploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExpressGatewayServiceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "healthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<ExpressGatewayServiceNetworkConfiguration>,
    #[serde(rename = "primaryContainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_container: Option<ExpressGatewayContainer>,
    #[serde(rename = "scalingTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_target: Option<ExpressGatewayScalingTarget>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    pub service_arn: String,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExpressGatewayServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<UpdatedExpressGatewayService>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatedExpressGatewayService {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExpressGatewayServiceStatus>,
    #[serde(rename = "targetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<ExpressGatewayServiceConfiguration>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServicePrimaryTaskSetRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(rename = "primaryTaskSet")]
    #[serde(default)]
    pub primary_task_set: String,
    #[serde(default)]
    pub service: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServicePrimaryTaskSetResponse {
    #[serde(rename = "taskSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceRequest {
    #[serde(rename = "availabilityZoneRebalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_rebalancing: Option<String>,
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "deploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "deploymentController")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    #[serde(rename = "desiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[serde(rename = "enableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "forceNewDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_deployment: Option<bool>,
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i32>,
    #[serde(rename = "loadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "placementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "placementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "serviceConnectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,
    #[serde(rename = "serviceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    #[serde(rename = "taskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    #[serde(rename = "volumeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<ServiceVolumeConfiguration>>,
    #[serde(rename = "vpcLatticeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice_configurations: Option<Vec<VpcLatticeConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskProtectionRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(rename = "expiresInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_minutes: Option<i32>,
    #[serde(rename = "protectionEnabled")]
    #[serde(default)]
    pub protection_enabled: bool,
    #[serde(default)]
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskProtectionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[serde(rename = "protectedTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_tasks: Option<Vec<ProtectedTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskSetRequest {
    #[serde(default)]
    pub cluster: String,
    #[serde(default)]
    pub scale: Scale,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "taskSet")]
    #[serde(default)]
    pub task_set: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskSetResponse {
    #[serde(rename = "taskSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set: Option<TaskSet>,
}
