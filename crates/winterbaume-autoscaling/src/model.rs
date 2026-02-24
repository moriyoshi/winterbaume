//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-autoscaling

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExitStandbyResult")]
pub struct ExitStandbyAnswer {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Activities>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Activities {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Activity>,
}
impl From<Vec<Activity>> for Activities {
    fn from(v: Vec<Activity>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Activity> for Activities {
    fn from_iter<I: IntoIterator<Item = Activity>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Activity>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlActivityList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Activity>,
}

impl From<Vec<Activity>> for XmlActivityList {
    fn from(v: Vec<Activity>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Activity> for XmlActivityList {
    fn from_iter<I: IntoIterator<Item = Activity>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Activity")]
pub struct Activity {
    #[serde(rename = "ActivityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "AutoScalingGroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_a_r_n: Option<String>,
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "AutoScalingGroupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_state: Option<String>,
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
    pub end_time: Option<String>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
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
#[serde(rename = "LaunchConfigurationNameType")]
pub struct LaunchConfigurationNameType {
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    pub launch_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteScheduledActionType")]
pub struct DeleteScheduledActionType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTagsResult")]
pub struct TagsType {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagDescriptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TagDescription>,
}
impl From<Vec<TagDescription>> for TagDescriptionList {
    fn from(v: Vec<TagDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TagDescription> for TagDescriptionList {
    fn from_iter<I: IntoIterator<Item = TagDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TagDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TagDescription>,
}

impl From<Vec<TagDescription>> for XmlTagDescriptionList {
    fn from(v: Vec<TagDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TagDescription> for XmlTagDescriptionList {
    fn from_iter<I: IntoIterator<Item = TagDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagDescription")]
pub struct TagDescription {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "PropagateAtLaunch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_at_launch: Option<bool>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAutoScalingNotificationTypesResult")]
pub struct DescribeAutoScalingNotificationTypesAnswer {
    #[serde(rename = "AutoScalingNotificationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_notification_types: Option<AutoScalingNotificationTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingNotificationTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AutoScalingNotificationTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AutoScalingNotificationTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecycleHookAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartInstanceRefreshType")]
pub struct StartInstanceRefreshType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "DesiredConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_configuration: Option<DesiredConfiguration>,
    #[serde(rename = "Preferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<RefreshPreferences>,
    #[serde(rename = "Strategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DesiredConfiguration")]
pub struct DesiredConfiguration {
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchTemplateSpecification")]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MixedInstancesPolicy")]
pub struct MixedInstancesPolicy {
    #[serde(rename = "InstancesDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_distribution: Option<InstancesDistribution>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstancesDistribution")]
pub struct InstancesDistribution {
    #[serde(rename = "OnDemandAllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_allocation_strategy: Option<String>,
    #[serde(rename = "OnDemandBaseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i32>,
    #[serde(rename = "OnDemandPercentageAboveBaseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i32>,
    #[serde(rename = "SpotAllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_allocation_strategy: Option<String>,
    #[serde(rename = "SpotInstancePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i32>,
    #[serde(rename = "SpotMaxPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_price: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchTemplate")]
pub struct LaunchTemplate {
    #[serde(rename = "LaunchTemplateSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    #[serde(rename = "Overrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Overrides>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Overrides {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LaunchTemplateOverrides>,
}
impl From<Vec<LaunchTemplateOverrides>> for Overrides {
    fn from(v: Vec<LaunchTemplateOverrides>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LaunchTemplateOverrides> for Overrides {
    fn from_iter<I: IntoIterator<Item = LaunchTemplateOverrides>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LaunchTemplateOverrides>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLaunchTemplateOverridesList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LaunchTemplateOverrides>,
}

impl From<Vec<LaunchTemplateOverrides>> for XmlLaunchTemplateOverridesList {
    fn from(v: Vec<LaunchTemplateOverrides>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LaunchTemplateOverrides> for XmlLaunchTemplateOverridesList {
    fn from_iter<I: IntoIterator<Item = LaunchTemplateOverrides>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchTemplateOverrides")]
pub struct LaunchTemplateOverrides {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<InstanceRequirements>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LaunchTemplateSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceRequirements")]
pub struct InstanceRequirements {
    #[serde(rename = "AcceleratorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_count: Option<AcceleratorCountRequest>,
    #[serde(rename = "AcceleratorManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_manufacturers: Option<AcceleratorManufacturers>,
    #[serde(rename = "AcceleratorNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_names: Option<AcceleratorNames>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,
    #[serde(rename = "AcceleratorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<AcceleratorTypes>,
    #[serde(rename = "AllowedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_instance_types: Option<AllowedInstanceTypes>,
    #[serde(rename = "BareMetal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bare_metal: Option<String>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,
    #[serde(rename = "BaselinePerformanceFactors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_performance_factors: Option<BaselinePerformanceFactorsRequest>,
    #[serde(rename = "BurstablePerformance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "CpuManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_manufacturers: Option<CpuManufacturers>,
    #[serde(rename = "ExcludedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_instance_types: Option<ExcludedInstanceTypes>,
    #[serde(rename = "InstanceGenerations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_generations: Option<InstanceGenerations>,
    #[serde(rename = "LocalStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<String>,
    #[serde(rename = "LocalStorageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage_types: Option<LocalStorageTypes>,
    #[serde(rename = "MaxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_gi_b_per_v_cpu: Option<MemoryGiBPerVCpuRequest>,
    #[serde(rename = "MemoryMiB")]
    #[serde(default)]
    pub memory_mi_b: MemoryMiBRequest,
    #[serde(rename = "NetworkBandwidthGbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,
    #[serde(rename = "NetworkInterfaceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "RequireHibernateSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "TotalLocalStorageGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_local_storage_g_b: Option<TotalLocalStorageGBRequest>,
    #[serde(rename = "VCpuCount")]
    #[serde(default)]
    pub v_cpu_count: VCpuCountRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AcceleratorCountRequest")]
pub struct AcceleratorCountRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceleratorManufacturers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AcceleratorManufacturers {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AcceleratorManufacturers {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceleratorNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AcceleratorNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AcceleratorNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AcceleratorTotalMemoryMiBRequest")]
pub struct AcceleratorTotalMemoryMiBRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceleratorTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AcceleratorTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AcceleratorTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedInstanceTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AllowedInstanceTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AllowedInstanceTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BaselineEbsBandwidthMbpsRequest")]
pub struct BaselineEbsBandwidthMbpsRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BaselinePerformanceFactorsRequest")]
pub struct BaselinePerformanceFactorsRequest {
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuPerformanceFactorRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CpuPerformanceFactorRequest")]
pub struct CpuPerformanceFactorRequest {
    #[serde(rename = "Reference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<PerformanceFactorReferenceSetRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PerformanceFactorReferenceSetRequest {
    #[serde(rename = "item", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PerformanceFactorReferenceRequest>,
}
impl From<Vec<PerformanceFactorReferenceRequest>> for PerformanceFactorReferenceSetRequest {
    fn from(v: Vec<PerformanceFactorReferenceRequest>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PerformanceFactorReferenceRequest> for PerformanceFactorReferenceSetRequest {
    fn from_iter<I: IntoIterator<Item = PerformanceFactorReferenceRequest>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PerformanceFactorReferenceRequest>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPerformanceFactorReferenceRequestList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PerformanceFactorReferenceRequest>,
}

impl From<Vec<PerformanceFactorReferenceRequest>> for XmlPerformanceFactorReferenceRequestList {
    fn from(v: Vec<PerformanceFactorReferenceRequest>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PerformanceFactorReferenceRequest> for XmlPerformanceFactorReferenceRequestList {
    fn from_iter<I: IntoIterator<Item = PerformanceFactorReferenceRequest>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PerformanceFactorReferenceRequest")]
pub struct PerformanceFactorReferenceRequest {
    #[serde(rename = "InstanceFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CpuManufacturers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CpuManufacturers {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CpuManufacturers {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExcludedInstanceTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ExcludedInstanceTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ExcludedInstanceTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceGenerations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for InstanceGenerations {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for InstanceGenerations {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalStorageTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LocalStorageTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LocalStorageTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MemoryGiBPerVCpuRequest")]
pub struct MemoryGiBPerVCpuRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MemoryMiBRequest")]
pub struct MemoryMiBRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    pub min: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NetworkBandwidthGbpsRequest")]
pub struct NetworkBandwidthGbpsRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NetworkInterfaceCountRequest")]
pub struct NetworkInterfaceCountRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TotalLocalStorageGBRequest")]
pub struct TotalLocalStorageGBRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VCpuCountRequest")]
pub struct VCpuCountRequest {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    pub min: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RefreshPreferences")]
pub struct RefreshPreferences {
    #[serde(rename = "AlarmSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_specification: Option<AlarmSpecification>,
    #[serde(rename = "AutoRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback: Option<bool>,
    #[serde(rename = "BakeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake_time: Option<i32>,
    #[serde(rename = "CheckpointDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_delay: Option<i32>,
    #[serde(rename = "CheckpointPercentages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_percentages: Option<CheckpointPercentages>,
    #[serde(rename = "InstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_warmup: Option<i32>,
    #[serde(rename = "MaxHealthyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_healthy_percentage: Option<i32>,
    #[serde(rename = "MinHealthyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_healthy_percentage: Option<i32>,
    #[serde(rename = "ScaleInProtectedInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_protected_instances: Option<String>,
    #[serde(rename = "SkipMatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_matching: Option<bool>,
    #[serde(rename = "StandbyInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_instances: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AlarmSpecification")]
pub struct AlarmSpecification {
    #[serde(rename = "Alarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<AlarmList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AlarmList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AlarmList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointPercentages {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<i32>,
}
impl From<Vec<i32>> for CheckpointPercentages {
    fn from(v: Vec<i32>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<i32> for CheckpointPercentages {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<i32>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmli32List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<i32>,
}

impl From<Vec<i32>> for Xmli32List {
    fn from(v: Vec<i32>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<i32> for Xmli32List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInstanceRefreshesType")]
pub struct DescribeInstanceRefreshesType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceRefreshIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_ids: Option<InstanceRefreshIds>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceRefreshIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for InstanceRefreshIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for InstanceRefreshIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInstanceRefreshesResult")]
pub struct DescribeInstanceRefreshesAnswer {
    #[serde(rename = "InstanceRefreshes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refreshes: Option<InstanceRefreshes>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceRefreshes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<InstanceRefresh>,
}
impl From<Vec<InstanceRefresh>> for InstanceRefreshes {
    fn from(v: Vec<InstanceRefresh>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InstanceRefresh> for InstanceRefreshes {
    fn from_iter<I: IntoIterator<Item = InstanceRefresh>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InstanceRefresh>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceRefreshList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InstanceRefresh>,
}

impl From<Vec<InstanceRefresh>> for XmlInstanceRefreshList {
    fn from(v: Vec<InstanceRefresh>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InstanceRefresh> for XmlInstanceRefreshList {
    fn from_iter<I: IntoIterator<Item = InstanceRefresh>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceRefresh")]
pub struct InstanceRefresh {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "DesiredConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_configuration: Option<DesiredConfiguration>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "InstanceRefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_id: Option<String>,
    #[serde(rename = "InstancesToUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_update: Option<i32>,
    #[serde(rename = "PercentageComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<i32>,
    #[serde(rename = "Preferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<RefreshPreferences>,
    #[serde(rename = "ProgressDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_details: Option<InstanceRefreshProgressDetails>,
    #[serde(rename = "RollbackDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_details: Option<RollbackDetails>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "Strategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceRefreshProgressDetails")]
pub struct InstanceRefreshProgressDetails {
    #[serde(rename = "LivePoolProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pool_progress: Option<InstanceRefreshLivePoolProgress>,
    #[serde(rename = "WarmPoolProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_progress: Option<InstanceRefreshWarmPoolProgress>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceRefreshLivePoolProgress")]
pub struct InstanceRefreshLivePoolProgress {
    #[serde(rename = "InstancesToUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_update: Option<i32>,
    #[serde(rename = "PercentageComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceRefreshWarmPoolProgress")]
pub struct InstanceRefreshWarmPoolProgress {
    #[serde(rename = "InstancesToUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_update: Option<i32>,
    #[serde(rename = "PercentageComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackDetails")]
pub struct RollbackDetails {
    #[serde(rename = "InstancesToUpdateOnRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_update_on_rollback: Option<i32>,
    #[serde(rename = "PercentageCompleteOnRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete_on_rollback: Option<i32>,
    #[serde(rename = "ProgressDetailsOnRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_details_on_rollback: Option<InstanceRefreshProgressDetails>,
    #[serde(rename = "RollbackReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_reason: Option<String>,
    #[serde(rename = "RollbackStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutScalingPolicyResult")]
pub struct PolicyARNType {
    #[serde(rename = "Alarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Alarms>,
    #[serde(rename = "PolicyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alarms {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Alarm>,
}
impl From<Vec<Alarm>> for Alarms {
    fn from(v: Vec<Alarm>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Alarm> for Alarms {
    fn from_iter<I: IntoIterator<Item = Alarm>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Alarm>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAlarmList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Alarm>,
}

impl From<Vec<Alarm>> for XmlAlarmList {
    fn from(v: Vec<Alarm>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Alarm> for XmlAlarmList {
    fn from_iter<I: IntoIterator<Item = Alarm>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Alarm")]
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
#[serde(rename = "DescribePoliciesType")]
pub struct DescribePoliciesType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<PolicyNames>,
    #[serde(rename = "PolicyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<PolicyTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PolicyNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PolicyNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PolicyTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PolicyTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TerminateInstanceInAutoScalingGroupType")]
pub struct TerminateInstanceInAutoScalingGroupType {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ShouldDecrementDesiredCapacity")]
    #[serde(default)]
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancersRequest")]
pub struct DescribeLoadBalancersRequest {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachTrafficSourcesResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutNotificationConfigurationType")]
pub struct PutNotificationConfigurationType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "NotificationTypes")]
    #[serde(default)]
    pub notification_types: AutoScalingNotificationTypes,
    #[serde(rename = "TopicARN")]
    #[serde(default)]
    pub topic_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutScalingPolicyType")]
pub struct PutScalingPolicyType {
    #[serde(rename = "AdjustmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "Cooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i32>,
    #[serde(rename = "MetricAggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<i32>,
    #[serde(rename = "MinAdjustmentStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_step: Option<i32>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "PredictiveScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_configuration: Option<PredictiveScalingConfiguration>,
    #[serde(rename = "ScalingAdjustment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<i32>,
    #[serde(rename = "StepAdjustments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<StepAdjustments>,
    #[serde(rename = "TargetTrackingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingConfiguration")]
pub struct PredictiveScalingConfiguration {
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
    pub metric_specifications: PredictiveScalingMetricSpecifications,
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
pub struct PredictiveScalingMetricSpecifications {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PredictiveScalingMetricSpecification>,
}
impl From<Vec<PredictiveScalingMetricSpecification>> for PredictiveScalingMetricSpecifications {
    fn from(v: Vec<PredictiveScalingMetricSpecification>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PredictiveScalingMetricSpecification> for PredictiveScalingMetricSpecifications {
    fn from_iter<I: IntoIterator<Item = PredictiveScalingMetricSpecification>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PredictiveScalingMetricSpecification>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPredictiveScalingMetricSpecificationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PredictiveScalingMetricSpecification>,
}

impl From<Vec<PredictiveScalingMetricSpecification>>
    for XmlPredictiveScalingMetricSpecificationList
{
    fn from(v: Vec<PredictiveScalingMetricSpecification>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PredictiveScalingMetricSpecification>
    for XmlPredictiveScalingMetricSpecificationList
{
    fn from_iter<I: IntoIterator<Item = PredictiveScalingMetricSpecification>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingMetricSpecification")]
pub struct PredictiveScalingMetricSpecification {
    #[serde(rename = "CustomizedCapacityMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_capacity_metric_specification: Option<PredictiveScalingCustomizedCapacityMetric>,
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_load_metric_specification: Option<PredictiveScalingCustomizedLoadMetric>,
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_scaling_metric_specification: Option<PredictiveScalingCustomizedScalingMetric>,
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_load_metric_specification: Option<PredictiveScalingPredefinedLoadMetric>,
    #[serde(rename = "PredefinedMetricPairSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_metric_pair_specification: Option<PredictiveScalingPredefinedMetricPair>,
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_scaling_metric_specification: Option<PredictiveScalingPredefinedScalingMetric>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingCustomizedCapacityMetric")]
pub struct PredictiveScalingCustomizedCapacityMetric {
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    pub metric_data_queries: MetricDataQueries,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataQueries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetricDataQuery>,
}
impl From<Vec<MetricDataQuery>> for MetricDataQueries {
    fn from(v: Vec<MetricDataQuery>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetricDataQuery> for MetricDataQueries {
    fn from_iter<I: IntoIterator<Item = MetricDataQuery>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetricDataQuery>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricDataQueryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetricDataQuery>,
}

impl From<Vec<MetricDataQuery>> for XmlMetricDataQueryList {
    fn from(v: Vec<MetricDataQuery>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetricDataQuery> for XmlMetricDataQueryList {
    fn from_iter<I: IntoIterator<Item = MetricDataQuery>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricDataQuery")]
pub struct MetricDataQuery {
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
    pub metric_stat: Option<MetricStat>,
    #[serde(rename = "ReturnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricStat")]
pub struct MetricStat {
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: Metric,
    #[serde(rename = "Stat")]
    #[serde(default)]
    pub stat: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Metric")]
pub struct Metric {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<MetricDimensions>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimensions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetricDimension>,
}
impl From<Vec<MetricDimension>> for MetricDimensions {
    fn from(v: Vec<MetricDimension>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetricDimension> for MetricDimensions {
    fn from_iter<I: IntoIterator<Item = MetricDimension>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetricDimension>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricDimensionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetricDimension>,
}

impl From<Vec<MetricDimension>> for XmlMetricDimensionList {
    fn from(v: Vec<MetricDimension>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetricDimension> for XmlMetricDimensionList {
    fn from_iter<I: IntoIterator<Item = MetricDimension>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricDimension")]
pub struct MetricDimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingCustomizedLoadMetric")]
pub struct PredictiveScalingCustomizedLoadMetric {
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    pub metric_data_queries: MetricDataQueries,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingCustomizedScalingMetric")]
pub struct PredictiveScalingCustomizedScalingMetric {
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    pub metric_data_queries: MetricDataQueries,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingPredefinedLoadMetric")]
pub struct PredictiveScalingPredefinedLoadMetric {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingPredefinedMetricPair")]
pub struct PredictiveScalingPredefinedMetricPair {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredictiveScalingPredefinedScalingMetric")]
pub struct PredictiveScalingPredefinedScalingMetric {
    #[serde(rename = "PredefinedMetricType")]
    #[serde(default)]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepAdjustments {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StepAdjustment>,
}
impl From<Vec<StepAdjustment>> for StepAdjustments {
    fn from(v: Vec<StepAdjustment>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StepAdjustment> for StepAdjustments {
    fn from_iter<I: IntoIterator<Item = StepAdjustment>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StepAdjustment>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStepAdjustmentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StepAdjustment>,
}

impl From<Vec<StepAdjustment>> for XmlStepAdjustmentList {
    fn from(v: Vec<StepAdjustment>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StepAdjustment> for XmlStepAdjustmentList {
    fn from_iter<I: IntoIterator<Item = StepAdjustment>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StepAdjustment")]
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
#[serde(rename = "TargetTrackingConfiguration")]
pub struct TargetTrackingConfiguration {
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
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomizedMetricSpecification")]
pub struct CustomizedMetricSpecification {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<MetricDimensions>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<TargetTrackingMetricDataQueries>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
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
pub struct TargetTrackingMetricDataQueries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetTrackingMetricDataQuery>,
}
impl From<Vec<TargetTrackingMetricDataQuery>> for TargetTrackingMetricDataQueries {
    fn from(v: Vec<TargetTrackingMetricDataQuery>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetTrackingMetricDataQuery> for TargetTrackingMetricDataQueries {
    fn from_iter<I: IntoIterator<Item = TargetTrackingMetricDataQuery>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetTrackingMetricDataQuery>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetTrackingMetricDataQueryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetTrackingMetricDataQuery>,
}

impl From<Vec<TargetTrackingMetricDataQuery>> for XmlTargetTrackingMetricDataQueryList {
    fn from(v: Vec<TargetTrackingMetricDataQuery>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetTrackingMetricDataQuery> for XmlTargetTrackingMetricDataQueryList {
    fn from_iter<I: IntoIterator<Item = TargetTrackingMetricDataQuery>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetTrackingMetricDataQuery")]
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
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "ReturnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetTrackingMetricStat")]
pub struct TargetTrackingMetricStat {
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: Metric,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "Stat")]
    #[serde(default)]
    pub stat: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PredefinedMetricSpecification")]
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
#[serde(rename = "CreateLaunchConfigurationType")]
pub struct CreateLaunchConfigurationType {
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<BlockDeviceMappings>,
    #[serde(rename = "ClassicLinkVPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_v_p_c_id: Option<String>,
    #[serde(rename = "ClassicLinkVPCSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_v_p_c_security_groups: Option<ClassicLinkVPCSecurityGroups>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<InstanceMonitoring>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KernelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    pub launch_configuration_name: String,
    #[serde(rename = "MetadataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<InstanceMetadataOptions>,
    #[serde(rename = "PlacementTenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<String>,
    #[serde(rename = "RamdiskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramdisk_id: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "SpotPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<String>,
    #[serde(rename = "UserData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockDeviceMappings {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BlockDeviceMapping>,
}
impl From<Vec<BlockDeviceMapping>> for BlockDeviceMappings {
    fn from(v: Vec<BlockDeviceMapping>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BlockDeviceMapping> for BlockDeviceMappings {
    fn from_iter<I: IntoIterator<Item = BlockDeviceMapping>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BlockDeviceMapping>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBlockDeviceMappingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BlockDeviceMapping>,
}

impl From<Vec<BlockDeviceMapping>> for XmlBlockDeviceMappingList {
    fn from(v: Vec<BlockDeviceMapping>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BlockDeviceMapping> for XmlBlockDeviceMappingList {
    fn from_iter<I: IntoIterator<Item = BlockDeviceMapping>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BlockDeviceMapping")]
pub struct BlockDeviceMapping {
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    pub device_name: String,
    #[serde(rename = "Ebs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<Ebs>,
    #[serde(rename = "NoDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<bool>,
    #[serde(rename = "VirtualName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Ebs")]
pub struct Ebs {
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Throughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassicLinkVPCSecurityGroups {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ClassicLinkVPCSecurityGroups {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ClassicLinkVPCSecurityGroups {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceMonitoring")]
pub struct InstanceMonitoring {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceMetadataOptions")]
pub struct InstanceMetadataOptions {
    #[serde(rename = "HttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i32>,
    #[serde(rename = "HttpTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroups {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SecurityGroups {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SecurityGroups {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CancelInstanceRefreshResult")]
pub struct CancelInstanceRefreshAnswer {
    #[serde(rename = "InstanceRefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AutoScalingGroupNamesType")]
pub struct AutoScalingGroupNamesType {
    #[serde(rename = "AutoScalingGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_names: Option<AutoScalingGroupNames>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "IncludeInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_instances: Option<bool>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroupNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AutoScalingGroupNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AutoScalingGroupNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Filter>,
}
impl From<Vec<Filter>> for Filters {
    fn from(v: Vec<Filter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Filter> for Filters {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Filter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Filter>,
}

impl From<Vec<Filter>> for XmlFilterList {
    fn from(v: Vec<Filter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Filter> for XmlFilterList {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Filter")]
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Values>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Values {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Values {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Values {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachLoadBalancersType")]
pub struct AttachLoadBalancersType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    pub load_balancer_names: LoadBalancerNames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LoadBalancerNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LoadBalancerNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLifecycleHookTypesResult")]
pub struct DescribeLifecycleHookTypesAnswer {
    #[serde(rename = "LifecycleHookTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_types: Option<AutoScalingNotificationTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancersResult")]
pub struct DescribeLoadBalancersResponse {
    #[serde(rename = "LoadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<LoadBalancerStates>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerStates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancerState>,
}
impl From<Vec<LoadBalancerState>> for LoadBalancerStates {
    fn from(v: Vec<LoadBalancerState>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancerState> for LoadBalancerStates {
    fn from_iter<I: IntoIterator<Item = LoadBalancerState>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancerState>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerStateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancerState>,
}

impl From<Vec<LoadBalancerState>> for XmlLoadBalancerStateList {
    fn from(v: Vec<LoadBalancerState>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancerState> for XmlLoadBalancerStateList {
    fn from_iter<I: IntoIterator<Item = LoadBalancerState>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerState")]
pub struct LoadBalancerState {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLaunchConfigurationsResult")]
pub struct LaunchConfigurationsType {
    #[serde(rename = "LaunchConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configurations: Option<LaunchConfigurations>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchConfigurations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LaunchConfiguration>,
}
impl From<Vec<LaunchConfiguration>> for LaunchConfigurations {
    fn from(v: Vec<LaunchConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LaunchConfiguration> for LaunchConfigurations {
    fn from_iter<I: IntoIterator<Item = LaunchConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LaunchConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLaunchConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LaunchConfiguration>,
}

impl From<Vec<LaunchConfiguration>> for XmlLaunchConfigurationList {
    fn from(v: Vec<LaunchConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LaunchConfiguration> for XmlLaunchConfigurationList {
    fn from_iter<I: IntoIterator<Item = LaunchConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchConfiguration")]
pub struct LaunchConfiguration {
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<BlockDeviceMappings>,
    #[serde(rename = "ClassicLinkVPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_v_p_c_id: Option<String>,
    #[serde(rename = "ClassicLinkVPCSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_v_p_c_security_groups: Option<ClassicLinkVPCSecurityGroups>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<InstanceMonitoring>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KernelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LaunchConfigurationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_a_r_n: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "MetadataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<InstanceMetadataOptions>,
    #[serde(rename = "PlacementTenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<String>,
    #[serde(rename = "RamdiskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramdisk_id: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "SpotPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<String>,
    #[serde(rename = "UserData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeMetricCollectionTypesResult")]
pub struct DescribeMetricCollectionTypesAnswer {
    #[serde(rename = "Granularities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularities: Option<MetricGranularityTypes>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricCollectionTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricGranularityTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetricGranularityType>,
}
impl From<Vec<MetricGranularityType>> for MetricGranularityTypes {
    fn from(v: Vec<MetricGranularityType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetricGranularityType> for MetricGranularityTypes {
    fn from_iter<I: IntoIterator<Item = MetricGranularityType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetricGranularityType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricGranularityTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetricGranularityType>,
}

impl From<Vec<MetricGranularityType>> for XmlMetricGranularityTypeList {
    fn from(v: Vec<MetricGranularityType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetricGranularityType> for XmlMetricGranularityTypeList {
    fn from_iter<I: IntoIterator<Item = MetricGranularityType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricGranularityType")]
pub struct MetricGranularityType {
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricCollectionTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetricCollectionType>,
}
impl From<Vec<MetricCollectionType>> for MetricCollectionTypes {
    fn from(v: Vec<MetricCollectionType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetricCollectionType> for MetricCollectionTypes {
    fn from_iter<I: IntoIterator<Item = MetricCollectionType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetricCollectionType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricCollectionTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetricCollectionType>,
}

impl From<Vec<MetricCollectionType>> for XmlMetricCollectionTypeList {
    fn from(v: Vec<MetricCollectionType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetricCollectionType> for XmlMetricCollectionTypeList {
    fn from_iter<I: IntoIterator<Item = MetricCollectionType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricCollectionType")]
pub struct MetricCollectionType {
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachLoadBalancerTargetGroupsResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachLoadBalancersType")]
pub struct DetachLoadBalancersType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    pub load_balancer_names: LoadBalancerNames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExecutePolicyType")]
pub struct ExecutePolicyType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "BreachThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breach_threshold: Option<f64>,
    #[serde(rename = "HonorCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honor_cooldown: Option<bool>,
    #[serde(rename = "MetricValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<f64>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachTrafficSourcesType")]
pub struct AttachTrafficSourcesType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "SkipZonalShiftValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_zonal_shift_validation: Option<bool>,
    #[serde(rename = "TrafficSources")]
    #[serde(default)]
    pub traffic_sources: TrafficSources,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficSources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrafficSourceIdentifier>,
}
impl From<Vec<TrafficSourceIdentifier>> for TrafficSources {
    fn from(v: Vec<TrafficSourceIdentifier>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrafficSourceIdentifier> for TrafficSources {
    fn from_iter<I: IntoIterator<Item = TrafficSourceIdentifier>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrafficSourceIdentifier>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrafficSourceIdentifierList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrafficSourceIdentifier>,
}

impl From<Vec<TrafficSourceIdentifier>> for XmlTrafficSourceIdentifierList {
    fn from(v: Vec<TrafficSourceIdentifier>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrafficSourceIdentifier> for XmlTrafficSourceIdentifierList {
    fn from_iter<I: IntoIterator<Item = TrafficSourceIdentifier>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficSourceIdentifier")]
pub struct TrafficSourceIdentifier {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScheduledActionsResult")]
pub struct ScheduledActionsType {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledUpdateGroupActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_update_group_actions: Option<ScheduledUpdateGroupActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledUpdateGroupActions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScheduledUpdateGroupAction>,
}
impl From<Vec<ScheduledUpdateGroupAction>> for ScheduledUpdateGroupActions {
    fn from(v: Vec<ScheduledUpdateGroupAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScheduledUpdateGroupAction> for ScheduledUpdateGroupActions {
    fn from_iter<I: IntoIterator<Item = ScheduledUpdateGroupAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScheduledUpdateGroupAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScheduledUpdateGroupActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScheduledUpdateGroupAction>,
}

impl From<Vec<ScheduledUpdateGroupAction>> for XmlScheduledUpdateGroupActionList {
    fn from(v: Vec<ScheduledUpdateGroupAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScheduledUpdateGroupAction> for XmlScheduledUpdateGroupActionList {
    fn from_iter<I: IntoIterator<Item = ScheduledUpdateGroupAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScheduledUpdateGroupAction")]
pub struct ScheduledUpdateGroupAction {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "Recurrence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "ScheduledActionARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_a_r_n: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLifecycleHooksResult")]
pub struct DescribeLifecycleHooksAnswer {
    #[serde(rename = "LifecycleHooks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hooks: Option<LifecycleHooks>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleHooks {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LifecycleHook>,
}
impl From<Vec<LifecycleHook>> for LifecycleHooks {
    fn from(v: Vec<LifecycleHook>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LifecycleHook> for LifecycleHooks {
    fn from_iter<I: IntoIterator<Item = LifecycleHook>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LifecycleHook>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLifecycleHookList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LifecycleHook>,
}

impl From<Vec<LifecycleHook>> for XmlLifecycleHookList {
    fn from(v: Vec<LifecycleHook>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LifecycleHook> for XmlLifecycleHookList {
    fn from_iter<I: IntoIterator<Item = LifecycleHook>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleHook")]
pub struct LifecycleHook {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "DefaultResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_result: Option<String>,
    #[serde(rename = "GlobalTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_timeout: Option<i32>,
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<i32>,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_name: Option<String>,
    #[serde(rename = "LifecycleTransition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition: Option<String>,
    #[serde(rename = "NotificationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<String>,
    #[serde(rename = "NotificationTargetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachLoadBalancerTargetGroupsResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAutoScalingInstancesResult")]
pub struct AutoScalingInstancesType {
    #[serde(rename = "AutoScalingInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_instances: Option<AutoScalingInstances>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingInstances {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AutoScalingInstanceDetails>,
}
impl From<Vec<AutoScalingInstanceDetails>> for AutoScalingInstances {
    fn from(v: Vec<AutoScalingInstanceDetails>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AutoScalingInstanceDetails> for AutoScalingInstances {
    fn from_iter<I: IntoIterator<Item = AutoScalingInstanceDetails>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AutoScalingInstanceDetails>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAutoScalingInstanceDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AutoScalingInstanceDetails>,
}

impl From<Vec<AutoScalingInstanceDetails>> for XmlAutoScalingInstanceDetailsList {
    fn from(v: Vec<AutoScalingInstanceDetails>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AutoScalingInstanceDetails> for XmlAutoScalingInstanceDetailsList {
    fn from_iter<I: IntoIterator<Item = AutoScalingInstanceDetails>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AutoScalingInstanceDetails")]
pub struct AutoScalingInstanceDetails {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "LifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<String>,
    #[serde(rename = "ProtectedFromScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_from_scale_in: Option<bool>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchInstancesRequest")]
pub struct LaunchInstancesRequest {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "AvailabilityZoneIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_ids: Option<AvailabilityZoneIdsLimit1>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZonesLimit1>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "RequestedCapacity")]
    #[serde(default)]
    pub requested_capacity: i32,
    #[serde(rename = "RetryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIdsLimit1>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneIdsLimit1 {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZoneIdsLimit1 {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZoneIdsLimit1 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZonesLimit1 {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZonesLimit1 {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZonesLimit1 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetIdsLimit1 {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SubnetIdsLimit1 {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SubnetIdsLimit1 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CancelInstanceRefreshType")]
pub struct CancelInstanceRefreshType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "WaitForTransitioningInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_transitioning_instances: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOrUpdateTagsType")]
pub struct CreateOrUpdateTagsType {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tags {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for Tags {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for Tags {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PropagateAtLaunch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_at_launch: Option<bool>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteLifecycleHookType")]
pub struct DeleteLifecycleHookType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    pub lifecycle_hook_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAutoScalingGroupType")]
pub struct DeleteAutoScalingGroupType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ForceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScalingActivitiesResult")]
pub struct ActivitiesType {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Activities>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordLifecycleActionHeartbeatAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTerminationPolicyTypesResult")]
pub struct DescribeTerminationPolicyTypesAnswer {
    #[serde(rename = "TerminationPolicyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policy_types: Option<TerminationPolicies>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminationPolicies {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TerminationPolicies {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TerminationPolicies {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachInstancesQuery")]
pub struct DetachInstancesQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
    #[serde(rename = "ShouldDecrementDesiredCapacity")]
    #[serde(default)]
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for InstanceIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for InstanceIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScalingProcessTypesResult")]
pub struct ProcessesType {
    #[serde(rename = "Processes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Processes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Processes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ProcessType>,
}
impl From<Vec<ProcessType>> for Processes {
    fn from(v: Vec<ProcessType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ProcessType> for Processes {
    fn from_iter<I: IntoIterator<Item = ProcessType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ProcessType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlProcessTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ProcessType>,
}

impl From<Vec<ProcessType>> for XmlProcessTypeList {
    fn from(v: Vec<ProcessType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ProcessType> for XmlProcessTypeList {
    fn from_iter<I: IntoIterator<Item = ProcessType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProcessType")]
pub struct ProcessType {
    #[serde(rename = "ProcessName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnterStandbyQuery")]
pub struct EnterStandbyQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
    #[serde(rename = "ShouldDecrementDesiredCapacity")]
    #[serde(default)]
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchPutScheduledUpdateGroupActionType")]
pub struct BatchPutScheduledUpdateGroupActionType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ScheduledUpdateGroupActions")]
    #[serde(default)]
    pub scheduled_update_group_actions: ScheduledUpdateGroupActionRequests,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledUpdateGroupActionRequests {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScheduledUpdateGroupActionRequest>,
}
impl From<Vec<ScheduledUpdateGroupActionRequest>> for ScheduledUpdateGroupActionRequests {
    fn from(v: Vec<ScheduledUpdateGroupActionRequest>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScheduledUpdateGroupActionRequest> for ScheduledUpdateGroupActionRequests {
    fn from_iter<I: IntoIterator<Item = ScheduledUpdateGroupActionRequest>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScheduledUpdateGroupActionRequest>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScheduledUpdateGroupActionRequestList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScheduledUpdateGroupActionRequest>,
}

impl From<Vec<ScheduledUpdateGroupActionRequest>> for XmlScheduledUpdateGroupActionRequestList {
    fn from(v: Vec<ScheduledUpdateGroupActionRequest>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScheduledUpdateGroupActionRequest> for XmlScheduledUpdateGroupActionRequestList {
    fn from_iter<I: IntoIterator<Item = ScheduledUpdateGroupActionRequest>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScheduledUpdateGroupActionRequest")]
pub struct ScheduledUpdateGroupActionRequest {
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "Recurrence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePolicyType")]
pub struct DeletePolicyType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPredictiveScalingForecastType")]
pub struct GetPredictiveScalingForecastType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchConfigurationNamesType")]
pub struct LaunchConfigurationNamesType {
    #[serde(rename = "LaunchConfigurationNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_names: Option<LaunchConfigurationNames>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchConfigurationNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LaunchConfigurationNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LaunchConfigurationNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDeleteScheduledActionResult")]
pub struct BatchDeleteScheduledActionAnswer {
    #[serde(rename = "FailedScheduledActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_scheduled_actions: Option<FailedScheduledUpdateGroupActionRequests>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedScheduledUpdateGroupActionRequests {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<FailedScheduledUpdateGroupActionRequest>,
}
impl From<Vec<FailedScheduledUpdateGroupActionRequest>>
    for FailedScheduledUpdateGroupActionRequests
{
    fn from(v: Vec<FailedScheduledUpdateGroupActionRequest>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<FailedScheduledUpdateGroupActionRequest>
    for FailedScheduledUpdateGroupActionRequests
{
    fn from_iter<I: IntoIterator<Item = FailedScheduledUpdateGroupActionRequest>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<FailedScheduledUpdateGroupActionRequest>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFailedScheduledUpdateGroupActionRequestList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<FailedScheduledUpdateGroupActionRequest>,
}

impl From<Vec<FailedScheduledUpdateGroupActionRequest>>
    for XmlFailedScheduledUpdateGroupActionRequestList
{
    fn from(v: Vec<FailedScheduledUpdateGroupActionRequest>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<FailedScheduledUpdateGroupActionRequest>
    for XmlFailedScheduledUpdateGroupActionRequestList
{
    fn from_iter<I: IntoIterator<Item = FailedScheduledUpdateGroupActionRequest>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailedScheduledUpdateGroupActionRequest")]
pub struct FailedScheduledUpdateGroupActionRequest {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachInstancesResult")]
pub struct DetachInstancesAnswer {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Activities>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchInstancesResult")]
pub struct LaunchInstancesResult {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<LaunchInstancesErrors>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<InstanceCollections>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchInstancesErrors {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LaunchInstancesError>,
}
impl From<Vec<LaunchInstancesError>> for LaunchInstancesErrors {
    fn from(v: Vec<LaunchInstancesError>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LaunchInstancesError> for LaunchInstancesErrors {
    fn from_iter<I: IntoIterator<Item = LaunchInstancesError>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LaunchInstancesError>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLaunchInstancesErrorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LaunchInstancesError>,
}

impl From<Vec<LaunchInstancesError>> for XmlLaunchInstancesErrorList {
    fn from(v: Vec<LaunchInstancesError>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LaunchInstancesError> for XmlLaunchInstancesErrorList {
    fn from_iter<I: IntoIterator<Item = LaunchInstancesError>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchInstancesError")]
pub struct LaunchInstancesError {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "MarketType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceCollections {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<InstanceCollection>,
}
impl From<Vec<InstanceCollection>> for InstanceCollections {
    fn from(v: Vec<InstanceCollection>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InstanceCollection> for InstanceCollections {
    fn from_iter<I: IntoIterator<Item = InstanceCollection>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InstanceCollection>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceCollectionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InstanceCollection>,
}

impl From<Vec<InstanceCollection>> for XmlInstanceCollectionList {
    fn from(v: Vec<InstanceCollection>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InstanceCollection> for XmlInstanceCollectionList {
    fn from_iter<I: IntoIterator<Item = InstanceCollection>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceCollection")]
pub struct InstanceCollection {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "MarketType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrafficSourcesResult")]
pub struct DescribeTrafficSourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrafficSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_sources: Option<TrafficSourceStates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficSourceStates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrafficSourceState>,
}
impl From<Vec<TrafficSourceState>> for TrafficSourceStates {
    fn from(v: Vec<TrafficSourceState>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrafficSourceState> for TrafficSourceStates {
    fn from_iter<I: IntoIterator<Item = TrafficSourceState>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrafficSourceState>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrafficSourceStateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrafficSourceState>,
}

impl From<Vec<TrafficSourceState>> for XmlTrafficSourceStateList {
    fn from(v: Vec<TrafficSourceState>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrafficSourceState> for XmlTrafficSourceStateList {
    fn from_iter<I: IntoIterator<Item = TrafficSourceState>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficSourceState")]
pub struct TrafficSourceState {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TrafficSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_source: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutLifecycleHookType")]
pub struct PutLifecycleHookType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "DefaultResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_result: Option<String>,
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<i32>,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    pub lifecycle_hook_name: String,
    #[serde(rename = "LifecycleTransition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition: Option<String>,
    #[serde(rename = "NotificationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<String>,
    #[serde(rename = "NotificationTargetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWarmPoolAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutScheduledUpdateGroupActionType")]
pub struct PutScheduledUpdateGroupActionType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "Recurrence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecordLifecycleActionHeartbeatType")]
pub struct RecordLifecycleActionHeartbeatType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LifecycleActionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_action_token: Option<String>,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    pub lifecycle_hook_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeNotificationConfigurationsResult")]
pub struct DescribeNotificationConfigurationsAnswer {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotificationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configurations: Option<NotificationConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationConfigurations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NotificationConfiguration>,
}
impl From<Vec<NotificationConfiguration>> for NotificationConfigurations {
    fn from(v: Vec<NotificationConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NotificationConfiguration> for NotificationConfigurations {
    fn from_iter<I: IntoIterator<Item = NotificationConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NotificationConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNotificationConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NotificationConfiguration>,
}

impl From<Vec<NotificationConfiguration>> for XmlNotificationConfigurationList {
    fn from(v: Vec<NotificationConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NotificationConfiguration> for XmlNotificationConfigurationList {
    fn from_iter<I: IntoIterator<Item = NotificationConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NotificationConfiguration")]
pub struct NotificationConfiguration {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
    #[serde(rename = "TopicARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachTrafficSourcesResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAutoScalingGroupType")]
pub struct CreateAutoScalingGroupType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "AvailabilityZoneDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_distribution: Option<AvailabilityZoneDistribution>,
    #[serde(rename = "AvailabilityZoneImpairmentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_impairment_policy: Option<AvailabilityZoneImpairmentPolicy>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "CapacityRebalance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_rebalance: Option<bool>,
    #[serde(rename = "CapacityReservationSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_specification: Option<CapacityReservationSpecification>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "DefaultCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cooldown: Option<i32>,
    #[serde(rename = "DefaultInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_warmup: Option<i32>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "DesiredCapacityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity_type: Option<String>,
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<i32>,
    #[serde(rename = "HealthCheckType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceLifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_lifecycle_policy: Option<InstanceLifecyclePolicy>,
    #[serde(rename = "InstanceMaintenancePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_maintenance_policy: Option<InstanceMaintenancePolicy>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "LifecycleHookSpecificationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_specification_list: Option<LifecycleHookSpecifications>,
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<LoadBalancerNames>,
    #[serde(rename = "MaxInstanceLifetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instance_lifetime: Option<i32>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    pub max_size: i32,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    pub min_size: i32,
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    #[serde(rename = "NewInstancesProtectedFromScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_instances_protected_from_scale_in: Option<bool>,
    #[serde(rename = "PlacementGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    #[serde(rename = "ServiceLinkedRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_linked_role_a_r_n: Option<String>,
    #[serde(rename = "SkipZonalShiftValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_zonal_shift_validation: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TargetGroupARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_a_r_ns: Option<TargetGroupARNs>,
    #[serde(rename = "TerminationPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policies: Option<TerminationPolicies>,
    #[serde(rename = "TrafficSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_sources: Option<TrafficSources>,
    #[serde(rename = "VPCZoneIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_zone_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailabilityZoneDistribution")]
pub struct AvailabilityZoneDistribution {
    #[serde(rename = "CapacityDistributionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_distribution_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailabilityZoneImpairmentPolicy")]
pub struct AvailabilityZoneImpairmentPolicy {
    #[serde(rename = "ImpairedZoneHealthCheckBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impaired_zone_health_check_behavior: Option<String>,
    #[serde(rename = "ZonalShiftEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZones {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZones {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZones {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CapacityReservationSpecification")]
pub struct CapacityReservationSpecification {
    #[serde(rename = "CapacityReservationPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_preference: Option<String>,
    #[serde(rename = "CapacityReservationTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_target: Option<CapacityReservationTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CapacityReservationTarget")]
pub struct CapacityReservationTarget {
    #[serde(rename = "CapacityReservationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_ids: Option<CapacityReservationIds>,
    #[serde(rename = "CapacityReservationResourceGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_resource_group_arns: Option<CapacityReservationResourceGroupArns>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityReservationIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CapacityReservationIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CapacityReservationIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityReservationResourceGroupArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CapacityReservationResourceGroupArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CapacityReservationResourceGroupArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceLifecyclePolicy")]
pub struct InstanceLifecyclePolicy {
    #[serde(rename = "RetentionTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_triggers: Option<RetentionTriggers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RetentionTriggers")]
pub struct RetentionTriggers {
    #[serde(rename = "TerminateHookAbandon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_hook_abandon: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceMaintenancePolicy")]
pub struct InstanceMaintenancePolicy {
    #[serde(rename = "MaxHealthyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_healthy_percentage: Option<i32>,
    #[serde(rename = "MinHealthyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_healthy_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleHookSpecifications {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LifecycleHookSpecification>,
}
impl From<Vec<LifecycleHookSpecification>> for LifecycleHookSpecifications {
    fn from(v: Vec<LifecycleHookSpecification>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LifecycleHookSpecification> for LifecycleHookSpecifications {
    fn from_iter<I: IntoIterator<Item = LifecycleHookSpecification>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LifecycleHookSpecification>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLifecycleHookSpecificationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LifecycleHookSpecification>,
}

impl From<Vec<LifecycleHookSpecification>> for XmlLifecycleHookSpecificationList {
    fn from(v: Vec<LifecycleHookSpecification>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LifecycleHookSpecification> for XmlLifecycleHookSpecificationList {
    fn from_iter<I: IntoIterator<Item = LifecycleHookSpecification>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleHookSpecification")]
pub struct LifecycleHookSpecification {
    #[serde(rename = "DefaultResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_result: Option<String>,
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<i32>,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    pub lifecycle_hook_name: String,
    #[serde(rename = "LifecycleTransition")]
    #[serde(default)]
    pub lifecycle_transition: String,
    #[serde(rename = "NotificationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<String>,
    #[serde(rename = "NotificationTargetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupARNs {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TargetGroupARNs {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TargetGroupARNs {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTagsType")]
pub struct DeleteTagsType {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScheduledActionsType")]
pub struct DescribeScheduledActionsType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledActionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_names: Option<ScheduledActionNames>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledActionNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ScheduledActionNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ScheduledActionNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableMetricsCollectionQuery")]
pub struct EnableMetricsCollectionQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metrics {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Metrics {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Metrics {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnterStandbyResult")]
pub struct EnterStandbyAnswer {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Activities>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLifecycleHooksType")]
pub struct DescribeLifecycleHooksType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "LifecycleHookNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_names: Option<LifecycleHookNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleHookNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LifecycleHookNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LifecycleHookNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteWarmPoolType")]
pub struct DeleteWarmPoolType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ForceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAutoScalingInstancesType")]
pub struct DescribeAutoScalingInstancesType {
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeWarmPoolType")]
pub struct DescribeWarmPoolType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrafficSourcesRequest")]
pub struct DescribeTrafficSourcesRequest {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrafficSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachTrafficSourcesType")]
pub struct DetachTrafficSourcesType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "TrafficSources")]
    #[serde(default)]
    pub traffic_sources: TrafficSources,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetDesiredCapacityType")]
pub struct SetDesiredCapacityType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    pub desired_capacity: i32,
    #[serde(rename = "HonorCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honor_cooldown: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePoliciesResult")]
pub struct PoliciesType {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<ScalingPolicies>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalingPolicies {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScalingPolicy>,
}
impl From<Vec<ScalingPolicy>> for ScalingPolicies {
    fn from(v: Vec<ScalingPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScalingPolicy> for ScalingPolicies {
    fn from_iter<I: IntoIterator<Item = ScalingPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScalingPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScalingPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScalingPolicy>,
}

impl From<Vec<ScalingPolicy>> for XmlScalingPolicyList {
    fn from(v: Vec<ScalingPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScalingPolicy> for XmlScalingPolicyList {
    fn from_iter<I: IntoIterator<Item = ScalingPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScalingPolicy")]
pub struct ScalingPolicy {
    #[serde(rename = "AdjustmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "Alarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Alarms>,
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "Cooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i32>,
    #[serde(rename = "MetricAggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<i32>,
    #[serde(rename = "MinAdjustmentStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_step: Option<i32>,
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
    #[serde(rename = "PredictiveScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_configuration: Option<PredictiveScalingConfiguration>,
    #[serde(rename = "ScalingAdjustment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<i32>,
    #[serde(rename = "StepAdjustments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<StepAdjustments>,
    #[serde(rename = "TargetTrackingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TerminateInstanceInAutoScalingGroupResult")]
pub struct ActivityType {
    #[serde(rename = "Activity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAutoScalingGroupType")]
pub struct UpdateAutoScalingGroupType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "AvailabilityZoneDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_distribution: Option<AvailabilityZoneDistribution>,
    #[serde(rename = "AvailabilityZoneImpairmentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_impairment_policy: Option<AvailabilityZoneImpairmentPolicy>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "CapacityRebalance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_rebalance: Option<bool>,
    #[serde(rename = "CapacityReservationSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_specification: Option<CapacityReservationSpecification>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "DefaultCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cooldown: Option<i32>,
    #[serde(rename = "DefaultInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_warmup: Option<i32>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "DesiredCapacityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity_type: Option<String>,
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<i32>,
    #[serde(rename = "HealthCheckType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    #[serde(rename = "InstanceLifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_lifecycle_policy: Option<InstanceLifecyclePolicy>,
    #[serde(rename = "InstanceMaintenancePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_maintenance_policy: Option<InstanceMaintenancePolicy>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "MaxInstanceLifetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instance_lifetime: Option<i32>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    #[serde(rename = "NewInstancesProtectedFromScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_instances_protected_from_scale_in: Option<bool>,
    #[serde(rename = "PlacementGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    #[serde(rename = "ServiceLinkedRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_linked_role_a_r_n: Option<String>,
    #[serde(rename = "SkipZonalShiftValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_zonal_shift_validation: Option<bool>,
    #[serde(rename = "TerminationPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policies: Option<TerminationPolicies>,
    #[serde(rename = "VPCZoneIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_zone_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackInstanceRefreshResult")]
pub struct RollbackInstanceRefreshAnswer {
    #[serde(rename = "InstanceRefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchPutScheduledUpdateGroupActionResult")]
pub struct BatchPutScheduledUpdateGroupActionAnswer {
    #[serde(rename = "FailedScheduledUpdateGroupActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_scheduled_update_group_actions: Option<FailedScheduledUpdateGroupActionRequests>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerTargetGroupsResult")]
pub struct DescribeLoadBalancerTargetGroupsResponse {
    #[serde(rename = "LoadBalancerTargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target_groups: Option<LoadBalancerTargetGroupStates>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerTargetGroupStates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancerTargetGroupState>,
}
impl From<Vec<LoadBalancerTargetGroupState>> for LoadBalancerTargetGroupStates {
    fn from(v: Vec<LoadBalancerTargetGroupState>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancerTargetGroupState> for LoadBalancerTargetGroupStates {
    fn from_iter<I: IntoIterator<Item = LoadBalancerTargetGroupState>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancerTargetGroupState>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerTargetGroupStateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancerTargetGroupState>,
}

impl From<Vec<LoadBalancerTargetGroupState>> for XmlLoadBalancerTargetGroupStateList {
    fn from(v: Vec<LoadBalancerTargetGroupState>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancerTargetGroupState> for XmlLoadBalancerTargetGroupStateList {
    fn from_iter<I: IntoIterator<Item = LoadBalancerTargetGroupState>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerTargetGroupState")]
pub struct LoadBalancerTargetGroupState {
    #[serde(rename = "LoadBalancerTargetGroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target_group_a_r_n: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutWarmPoolType")]
pub struct PutWarmPoolType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceReusePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_reuse_policy: Option<InstanceReusePolicy>,
    #[serde(rename = "MaxGroupPreparedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_group_prepared_capacity: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "PoolState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceReusePolicy")]
pub struct InstanceReusePolicy {
    #[serde(rename = "ReuseOnScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_on_scale_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExitStandbyQuery")]
pub struct ExitStandbyQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteLifecycleActionAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompleteLifecycleActionType")]
pub struct CompleteLifecycleActionType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LifecycleActionResult")]
    #[serde(default)]
    pub lifecycle_action_result: String,
    #[serde(rename = "LifecycleActionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_action_token: Option<String>,
    #[serde(rename = "LifecycleHookName")]
    #[serde(default)]
    pub lifecycle_hook_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTagsType")]
pub struct DescribeTagsType {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetInstanceHealthQuery")]
pub struct SetInstanceHealthQuery {
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    pub health_status: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ShouldRespectGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_respect_grace_period: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableMetricsCollectionQuery")]
pub struct DisableMetricsCollectionQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPredictiveScalingForecastResult")]
pub struct GetPredictiveScalingForecastAnswer {
    #[serde(rename = "CapacityForecast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_forecast: Option<CapacityForecast>,
    #[serde(rename = "LoadForecast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_forecast: Option<LoadForecasts>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CapacityForecast")]
pub struct CapacityForecast {
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<PredictiveScalingForecastTimestamps>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PredictiveScalingForecastValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingForecastTimestamps {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PredictiveScalingForecastTimestamps {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PredictiveScalingForecastTimestamps {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveScalingForecastValues {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<f64>,
}
impl From<Vec<f64>> for PredictiveScalingForecastValues {
    fn from(v: Vec<f64>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<f64> for PredictiveScalingForecastValues {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<f64>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmlf64List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<f64>,
}

impl From<Vec<f64>> for Xmlf64List {
    fn from(v: Vec<f64>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<f64> for Xmlf64List {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadForecasts {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadForecast>,
}
impl From<Vec<LoadForecast>> for LoadForecasts {
    fn from(v: Vec<LoadForecast>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadForecast> for LoadForecasts {
    fn from_iter<I: IntoIterator<Item = LoadForecast>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadForecast>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadForecastList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadForecast>,
}

impl From<Vec<LoadForecast>> for XmlLoadForecastList {
    fn from(v: Vec<LoadForecast>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadForecast> for XmlLoadForecastList {
    fn from_iter<I: IntoIterator<Item = LoadForecast>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadForecast")]
pub struct LoadForecast {
    #[serde(rename = "MetricSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_specification: Option<PredictiveScalingMetricSpecification>,
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<PredictiveScalingForecastTimestamps>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PredictiveScalingForecastValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackInstanceRefreshType")]
pub struct RollbackInstanceRefreshType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetInstanceProtectionAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecycleHookAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAutoScalingGroupsResult")]
pub struct AutoScalingGroupsType {
    #[serde(rename = "AutoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<AutoScalingGroups>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroups {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AutoScalingGroup>,
}
impl From<Vec<AutoScalingGroup>> for AutoScalingGroups {
    fn from(v: Vec<AutoScalingGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AutoScalingGroup> for AutoScalingGroups {
    fn from_iter<I: IntoIterator<Item = AutoScalingGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AutoScalingGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAutoScalingGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AutoScalingGroup>,
}

impl From<Vec<AutoScalingGroup>> for XmlAutoScalingGroupList {
    fn from(v: Vec<AutoScalingGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AutoScalingGroup> for XmlAutoScalingGroupList {
    fn from_iter<I: IntoIterator<Item = AutoScalingGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AutoScalingGroup")]
pub struct AutoScalingGroup {
    #[serde(rename = "AutoScalingGroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_a_r_n: Option<String>,
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "AvailabilityZoneDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_distribution: Option<AvailabilityZoneDistribution>,
    #[serde(rename = "AvailabilityZoneImpairmentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_impairment_policy: Option<AvailabilityZoneImpairmentPolicy>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "CapacityRebalance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_rebalance: Option<bool>,
    #[serde(rename = "CapacityReservationSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_specification: Option<CapacityReservationSpecification>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DefaultCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cooldown: Option<i32>,
    #[serde(rename = "DefaultInstanceWarmup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_warmup: Option<i32>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DesiredCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i32>,
    #[serde(rename = "DesiredCapacityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity_type: Option<String>,
    #[serde(rename = "EnabledMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_metrics: Option<EnabledMetrics>,
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<i32>,
    #[serde(rename = "HealthCheckType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    #[serde(rename = "InstanceLifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_lifecycle_policy: Option<InstanceLifecyclePolicy>,
    #[serde(rename = "InstanceMaintenancePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_maintenance_policy: Option<InstanceMaintenancePolicy>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<LoadBalancerNames>,
    #[serde(rename = "MaxInstanceLifetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instance_lifetime: Option<i32>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    #[serde(rename = "NewInstancesProtectedFromScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_instances_protected_from_scale_in: Option<bool>,
    #[serde(rename = "PlacementGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    #[serde(rename = "PredictedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_capacity: Option<i32>,
    #[serde(rename = "ServiceLinkedRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_linked_role_a_r_n: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SuspendedProcesses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_processes: Option<SuspendedProcesses>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagDescriptionList>,
    #[serde(rename = "TargetGroupARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_a_r_ns: Option<TargetGroupARNs>,
    #[serde(rename = "TerminationPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policies: Option<TerminationPolicies>,
    #[serde(rename = "TrafficSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_sources: Option<TrafficSources>,
    #[serde(rename = "VPCZoneIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_zone_identifier: Option<String>,
    #[serde(rename = "WarmPoolConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_configuration: Option<WarmPoolConfiguration>,
    #[serde(rename = "WarmPoolSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnabledMetrics {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EnabledMetric>,
}
impl From<Vec<EnabledMetric>> for EnabledMetrics {
    fn from(v: Vec<EnabledMetric>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EnabledMetric> for EnabledMetrics {
    fn from_iter<I: IntoIterator<Item = EnabledMetric>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EnabledMetric>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEnabledMetricList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EnabledMetric>,
}

impl From<Vec<EnabledMetric>> for XmlEnabledMetricList {
    fn from(v: Vec<EnabledMetric>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EnabledMetric> for XmlEnabledMetricList {
    fn from_iter<I: IntoIterator<Item = EnabledMetric>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnabledMetric")]
pub struct EnabledMetric {
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instances {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Instance>,
}
impl From<Vec<Instance>> for Instances {
    fn from(v: Vec<Instance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Instance> for Instances {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Instance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Instance>,
}

impl From<Vec<Instance>> for XmlInstanceList {
    fn from(v: Vec<Instance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Instance> for XmlInstanceList {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Instance")]
pub struct Instance {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "LifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<String>,
    #[serde(rename = "ProtectedFromScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_from_scale_in: Option<bool>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuspendedProcesses {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SuspendedProcess>,
}
impl From<Vec<SuspendedProcess>> for SuspendedProcesses {
    fn from(v: Vec<SuspendedProcess>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SuspendedProcess> for SuspendedProcesses {
    fn from_iter<I: IntoIterator<Item = SuspendedProcess>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SuspendedProcess>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSuspendedProcessList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SuspendedProcess>,
}

impl From<Vec<SuspendedProcess>> for XmlSuspendedProcessList {
    fn from(v: Vec<SuspendedProcess>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SuspendedProcess> for XmlSuspendedProcessList {
    fn from_iter<I: IntoIterator<Item = SuspendedProcess>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SuspendedProcess")]
pub struct SuspendedProcess {
    #[serde(rename = "ProcessName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    #[serde(rename = "SuspensionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspension_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WarmPoolConfiguration")]
pub struct WarmPoolConfiguration {
    #[serde(rename = "InstanceReusePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_reuse_policy: Option<InstanceReusePolicy>,
    #[serde(rename = "MaxGroupPreparedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_group_prepared_capacity: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "PoolState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachLoadBalancersResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDeleteScheduledActionType")]
pub struct BatchDeleteScheduledActionType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ScheduledActionNames")]
    #[serde(default)]
    pub scheduled_action_names: ScheduledActionNames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachInstancesQuery")]
pub struct AttachInstancesQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<InstanceIds>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartInstanceRefreshResult")]
pub struct StartInstanceRefreshAnswer {
    #[serde(rename = "InstanceRefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_refresh_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeWarmPoolResult")]
pub struct DescribeWarmPoolAnswer {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WarmPoolConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_configuration: Option<WarmPoolConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScalingProcessQuery")]
pub struct ScalingProcessQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "ScalingProcesses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_processes: Option<ProcessNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ProcessNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ProcessNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachLoadBalancerTargetGroupsType")]
pub struct AttachLoadBalancerTargetGroupsType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "TargetGroupARNs")]
    #[serde(default)]
    pub target_group_a_r_ns: TargetGroupARNs,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAdjustmentTypesResult")]
pub struct DescribeAdjustmentTypesAnswer {
    #[serde(rename = "AdjustmentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_types: Option<AdjustmentTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdjustmentTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AdjustmentType>,
}
impl From<Vec<AdjustmentType>> for AdjustmentTypes {
    fn from(v: Vec<AdjustmentType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AdjustmentType> for AdjustmentTypes {
    fn from_iter<I: IntoIterator<Item = AdjustmentType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AdjustmentType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAdjustmentTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AdjustmentType>,
}

impl From<Vec<AdjustmentType>> for XmlAdjustmentTypeList {
    fn from(v: Vec<AdjustmentType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AdjustmentType> for XmlAdjustmentTypeList {
    fn from_iter<I: IntoIterator<Item = AdjustmentType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdjustmentType")]
pub struct AdjustmentType {
    #[serde(rename = "AdjustmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetInstanceProtectionQuery")]
pub struct SetInstanceProtectionQuery {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    pub instance_ids: InstanceIds,
    #[serde(rename = "ProtectedFromScaleIn")]
    #[serde(default)]
    pub protected_from_scale_in: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachLoadBalancerTargetGroupsType")]
pub struct DetachLoadBalancerTargetGroupsType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "TargetGroupARNs")]
    #[serde(default)]
    pub target_group_a_r_ns: TargetGroupARNs,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountLimitsResult")]
pub struct DescribeAccountLimitsAnswer {
    #[serde(rename = "MaxNumberOfAutoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_auto_scaling_groups: Option<i32>,
    #[serde(rename = "MaxNumberOfLaunchConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_launch_configurations: Option<i32>,
    #[serde(rename = "NumberOfAutoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_auto_scaling_groups: Option<i32>,
    #[serde(rename = "NumberOfLaunchConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_launch_configurations: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeNotificationConfigurationsType")]
pub struct DescribeNotificationConfigurationsType {
    #[serde(rename = "AutoScalingGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_names: Option<AutoScalingGroupNames>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachLoadBalancersResultType {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutWarmPoolAnswer {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerTargetGroupsRequest")]
pub struct DescribeLoadBalancerTargetGroupsRequest {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteNotificationConfigurationType")]
pub struct DeleteNotificationConfigurationType {
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    pub auto_scaling_group_name: String,
    #[serde(rename = "TopicARN")]
    #[serde(default)]
    pub topic_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScalingActivitiesType")]
pub struct DescribeScalingActivitiesType {
    #[serde(rename = "ActivityIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ids: Option<ActivityIds>,
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "IncludeDeletedGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_groups: Option<bool>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ActivityIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ActivityIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}
