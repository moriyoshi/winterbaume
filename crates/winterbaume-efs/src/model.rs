//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-efs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPointDescription {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "LifeCycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "PosixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "RootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PosixUser {
    #[serde(rename = "Gid")]
    #[serde(default)]
    pub gid: i64,
    #[serde(rename = "SecondaryGids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    pub uid: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootDirectory {
    #[serde(rename = "CreationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_info: Option<CreationInfo>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreationInfo {
    #[serde(rename = "OwnerGid")]
    #[serde(default)]
    pub owner_gid: i64,
    #[serde(rename = "OwnerUid")]
    #[serde(default)]
    pub owner_uid: i64,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: String,
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
pub struct BackupPolicyDescription {
    #[serde(rename = "BackupPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupPolicy {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPointRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "PosixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "RootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemRequest {
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<bool>,
    #[serde(rename = "CreationToken")]
    #[serde(default)]
    pub creation_token: String,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "PerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ThroughputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMountTargetRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationConfigurationRequest {
    #[serde(rename = "Destinations")]
    #[serde(default)]
    pub destinations: Vec<DestinationToCreate>,
    #[serde(rename = "SourceFileSystemId")]
    #[serde(default)]
    pub source_file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationToCreate {
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessPointRequest {
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    pub access_point_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemPolicyRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMountTargetRequest {
    #[serde(rename = "MountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationConfigurationRequest {
    #[serde(rename = "DeletionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_mode: Option<String>,
    #[serde(rename = "SourceFileSystemId")]
    #[serde(default)]
    pub source_file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccessPointsRequest {
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
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
pub struct DescribeAccessPointsResponse {
    #[serde(rename = "AccessPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<Vec<AccessPointDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountPreferencesRequest {
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
pub struct DescribeAccountPreferencesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIdPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_preference: Option<ResourceIdPreference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdPreference {
    #[serde(rename = "ResourceIdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_type: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupPolicyRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileSystemPolicyRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileSystemsRequest {
    #[serde(rename = "CreationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileSystemsResponse {
    #[serde(rename = "FileSystems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystemDescription>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemDescription {
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CreationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_token: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "FileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "FileSystemProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_protection: Option<FileSystemProtectionDescription>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LifeCycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfMountTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_mount_targets: Option<i32>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "PerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<FileSystemSize>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ThroughputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemProtectionDescription {
    #[serde(rename = "ReplicationOverwriteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_overwrite_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemSize {
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(rename = "ValueInArchive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_archive: Option<i64>,
    #[serde(rename = "ValueInIA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_i_a: Option<i64>,
    #[serde(rename = "ValueInStandard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_standard: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLifecycleConfigurationRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMountTargetSecurityGroupsRequest {
    #[serde(rename = "MountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMountTargetSecurityGroupsResponse {
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMountTargetsRequest {
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "MountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMountTargetsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MountTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_targets: Option<Vec<MountTargetDescription>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MountTargetDescription {
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "LifeCycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<String>,
    #[serde(rename = "MountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationConfigurationsRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
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
pub struct DescribeReplicationConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Replications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replications: Option<Vec<ReplicationConfigurationDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationConfigurationDescription {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "OriginalSourceFileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_source_file_system_arn: Option<String>,
    #[serde(rename = "SourceFileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_arn: Option<String>,
    #[serde(rename = "SourceFileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_id: Option<String>,
    #[serde(rename = "SourceFileSystemOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_owner_id: Option<String>,
    #[serde(rename = "SourceFileSystemRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "LastReplicatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_replicated_timestamp: Option<f64>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemPolicyDescription {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleConfigurationDescription {
    #[serde(rename = "LifecyclePolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policies: Option<Vec<LifecyclePolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicy {
    #[serde(rename = "TransitionToArchive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_archive: Option<String>,
    #[serde(rename = "TransitionToIA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_i_a: Option<String>,
    #[serde(rename = "TransitionToPrimaryStorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_primary_storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
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
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyMountTargetSecurityGroupsRequest {
    #[serde(rename = "MountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountPreferencesRequest {
    #[serde(rename = "ResourceIdType")]
    #[serde(default)]
    pub resource_id_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountPreferencesResponse {
    #[serde(rename = "ResourceIdPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_preference: Option<ResourceIdPreference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBackupPolicyRequest {
    #[serde(rename = "BackupPolicy")]
    #[serde(default)]
    pub backup_policy: BackupPolicy,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileSystemPolicyRequest {
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecycleConfigurationRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "LifecyclePolicies")]
    #[serde(default)]
    pub lifecycle_policies: Vec<LifecyclePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemProtectionRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "ReplicationOverwriteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_overwrite_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemRequest {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    #[serde(rename = "ThroughputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}
