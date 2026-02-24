//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-backupgateway

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateGatewayToServerInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
    #[serde(rename = "ServerArn")]
    #[serde(default)]
    pub server_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateGatewayToServerOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGatewayInput {
    #[serde(rename = "ActivationKey")]
    #[serde(default)]
    pub activation_key: String,
    #[serde(rename = "GatewayDisplayName")]
    #[serde(default)]
    pub gateway_display_name: String,
    #[serde(rename = "GatewayType")]
    #[serde(default)]
    pub gateway_type: String,
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
pub struct CreateGatewayOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHypervisorInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHypervisorOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateGatewayFromServerInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateGatewayFromServerOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBandwidthRateLimitScheduleInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBandwidthRateLimitScheduleOutput {
    #[serde(rename = "BandwidthRateLimitIntervals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_rate_limit_intervals: Option<Vec<BandwidthRateLimitInterval>>,
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BandwidthRateLimitInterval {
    #[serde(rename = "AverageUploadRateLimitInBitsPerSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(rename = "DaysOfWeek")]
    #[serde(default)]
    pub days_of_week: Vec<i32>,
    #[serde(rename = "EndHourOfDay")]
    #[serde(default)]
    pub end_hour_of_day: i32,
    #[serde(rename = "EndMinuteOfHour")]
    #[serde(default)]
    pub end_minute_of_hour: i32,
    #[serde(rename = "StartHourOfDay")]
    #[serde(default)]
    pub start_hour_of_day: i32,
    #[serde(rename = "StartMinuteOfHour")]
    #[serde(default)]
    pub start_minute_of_hour: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGatewayInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGatewayOutput {
    #[serde(rename = "Gateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<GatewayDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayDetails {
    #[serde(rename = "DeprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "GatewayDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_display_name: Option<String>,
    #[serde(rename = "GatewayType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    #[serde(rename = "HypervisorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(rename = "LastSeenTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_time: Option<f64>,
    #[serde(rename = "MaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<MaintenanceStartTime>,
    #[serde(rename = "NextUpdateAvailabilityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update_availability_time: Option<f64>,
    #[serde(rename = "SoftwareVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
    #[serde(rename = "VpcEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceStartTime {
    #[serde(rename = "DayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    #[serde(rename = "DayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
    #[serde(rename = "HourOfDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_of_day: Option<i32>,
    #[serde(rename = "MinuteOfHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_of_hour: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHypervisorInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHypervisorOutput {
    #[serde(rename = "Hypervisor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<HypervisorDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HypervisorDetails {
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LastSuccessfulMetadataSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_metadata_sync_time: Option<f64>,
    #[serde(rename = "LatestMetadataSyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_metadata_sync_status: Option<String>,
    #[serde(rename = "LatestMetadataSyncStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_metadata_sync_status_message: Option<String>,
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
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
pub struct GetHypervisorPropertyMappingsInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHypervisorPropertyMappingsOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "VmwareToAwsTagMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmware_to_aws_tag_mappings: Option<Vec<VmwareToAwsTagMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VmwareToAwsTagMapping {
    #[serde(rename = "AwsTagKey")]
    #[serde(default)]
    pub aws_tag_key: String,
    #[serde(rename = "AwsTagValue")]
    #[serde(default)]
    pub aws_tag_value: String,
    #[serde(rename = "VmwareCategory")]
    #[serde(default)]
    pub vmware_category: String,
    #[serde(rename = "VmwareTagName")]
    #[serde(default)]
    pub vmware_tag_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVirtualMachineInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVirtualMachineOutput {
    #[serde(rename = "VirtualMachine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<VirtualMachineDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualMachineDetails {
    #[serde(rename = "HostName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "HypervisorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(rename = "LastBackupDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "VmwareTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmware_tags: Option<Vec<VmwareTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VmwareTag {
    #[serde(rename = "VmwareCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmware_category: Option<String>,
    #[serde(rename = "VmwareTagDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmware_tag_description: Option<String>,
    #[serde(rename = "VmwareTagName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmware_tag_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportHypervisorConfigurationInput {
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportHypervisorConfigurationOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGatewaysInput {
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
pub struct ListGatewaysOutput {
    #[serde(rename = "Gateways")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<Gateway>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Gateway {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "GatewayDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_display_name: Option<String>,
    #[serde(rename = "GatewayType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    #[serde(rename = "HypervisorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(rename = "LastSeenTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHypervisorsInput {
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
pub struct ListHypervisorsOutput {
    #[serde(rename = "Hypervisors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisors: Option<Vec<Hypervisor>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hypervisor {
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
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
pub struct ListTagsForResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualMachinesInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
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
pub struct ListVirtualMachinesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VirtualMachines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machines: Option<Vec<VirtualMachine>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualMachine {
    #[serde(rename = "HostName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "HypervisorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(rename = "LastBackupDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBandwidthRateLimitScheduleInput {
    #[serde(rename = "BandwidthRateLimitIntervals")]
    #[serde(default)]
    pub bandwidth_rate_limit_intervals: Vec<BandwidthRateLimitInterval>,
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBandwidthRateLimitScheduleOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutHypervisorPropertyMappingsInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "VmwareToAwsTagMappings")]
    #[serde(default)]
    pub vmware_to_aws_tag_mappings: Vec<VmwareToAwsTagMapping>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutHypervisorPropertyMappingsOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMaintenanceStartTimeInput {
    #[serde(rename = "DayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    #[serde(rename = "DayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
    #[serde(rename = "HourOfDay")]
    #[serde(default)]
    pub hour_of_day: i32,
    #[serde(rename = "MinuteOfHour")]
    #[serde(default)]
    pub minute_of_hour: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMaintenanceStartTimeOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartVirtualMachinesMetadataSyncInput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartVirtualMachinesMetadataSyncOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestHypervisorConfigurationInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
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
pub struct TestHypervisorConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayInformationInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
    #[serde(rename = "GatewayDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayInformationOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewaySoftwareNowInput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    pub gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewaySoftwareNowOutput {
    #[serde(rename = "GatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHypervisorInput {
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    pub hypervisor_arn: String,
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct UpdateHypervisorOutput {
    #[serde(rename = "HypervisorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_arn: Option<String>,
}
