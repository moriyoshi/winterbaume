//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3files

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPointRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "posixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PosixUser {
    #[serde(default)]
    pub gid: i64,
    #[serde(rename = "secondaryGids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
    #[serde(default)]
    pub uid: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootDirectory {
    #[serde(rename = "creationPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_permissions: Option<CreationPermissions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreationPermissions {
    #[serde(rename = "ownerGid")]
    #[serde(default)]
    pub owner_gid: i64,
    #[serde(rename = "ownerUid")]
    #[serde(default)]
    pub owner_uid: i64,
    #[serde(default)]
    pub permissions: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPointResponse {
    #[serde(rename = "accessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "posixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemRequest {
    #[serde(rename = "acceptBucketWarning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_bucket_warning: Option<bool>,
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "fileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMountTargetRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ipv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMountTargetResponse {
    #[serde(rename = "availabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "ipv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessPointRequest {
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    pub access_point_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemPolicyRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMountTargetRequest {
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPointRequest {
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    pub access_point_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPointResponse {
    #[serde(rename = "accessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "posixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileSystemPolicyRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileSystemPolicyResponse {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileSystemRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFileSystemResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "fileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMountTargetRequest {
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMountTargetResponse {
    #[serde(rename = "availabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "ipv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSynchronizationConfigurationRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSynchronizationConfigurationResponse {
    #[serde(rename = "expirationDataRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_data_rules: Option<Vec<ExpirationDataRule>>,
    #[serde(rename = "importDataRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_data_rules: Option<Vec<ImportDataRule>>,
    #[serde(rename = "latestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpirationDataRule {
    #[serde(rename = "daysAfterLastAccess")]
    #[serde(default)]
    pub days_after_last_access: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportDataRule {
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "sizeLessThan")]
    #[serde(default)]
    pub size_less_than: i64,
    #[serde(default)]
    pub trigger: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPointsRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
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
pub struct ListAccessPointsResponse {
    #[serde(rename = "accessPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<Vec<ListAccessPointsDescription>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPointsDescription {
    #[serde(rename = "accessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "posixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileSystemsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
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
pub struct ListFileSystemsResponse {
    #[serde(rename = "fileSystems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<ListFileSystemsDescription>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileSystemsDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "fileSystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMountTargetsRequest {
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
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
pub struct ListMountTargetsResponse {
    #[serde(rename = "mountTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_targets: Option<Vec<ListMountTargetsDescription>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMountTargetsDescription {
    #[serde(rename = "availabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "ipv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileSystemPolicyRequest {
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFileSystemPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSynchronizationConfigurationRequest {
    #[serde(rename = "expirationDataRules")]
    #[serde(default)]
    pub expiration_data_rules: Vec<ExpirationDataRule>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "importDataRules")]
    #[serde(default)]
    pub import_data_rules: Vec<ImportDataRule>,
    #[serde(rename = "latestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSynchronizationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMountTargetRequest {
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    pub mount_target_id: String,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    pub security_groups: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMountTargetResponse {
    #[serde(rename = "availabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "ipv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "mountTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}
