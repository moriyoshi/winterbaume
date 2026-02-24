//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudhsm-v2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyBackupToRegionRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
    #[serde(rename = "DestinationRegion")]
    #[serde(default)]
    pub destination_region: String,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
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
pub struct CopyBackupToRegionResponse {
    #[serde(rename = "DestinationBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup: Option<DestinationBackup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationBackup {
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "SourceBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    #[serde(rename = "SourceCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "BackupRetentionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    #[serde(rename = "HsmType")]
    #[serde(default)]
    pub hsm_type: String,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "SourceBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupRetentionPolicy {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "BackupPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<String>,
    #[serde(rename = "BackupRetentionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Certificates>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "HsmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    #[serde(rename = "HsmTypeRollbackExpiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type_rollback_expiration: Option<f64>,
    #[serde(rename = "Hsms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms: Option<Vec<Hsm>>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PreCoPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_co_password: Option<String>,
    #[serde(rename = "SecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    #[serde(rename = "SourceBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    #[serde(rename = "SubnetMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mapping: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificates {
    #[serde(rename = "AwsHardwareCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_hardware_certificate: Option<String>,
    #[serde(rename = "ClusterCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    #[serde(rename = "ClusterCsr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_csr: Option<String>,
    #[serde(rename = "HsmCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_certificate: Option<String>,
    #[serde(rename = "ManufacturerHardwareCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_hardware_certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hsm {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "EniId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    #[serde(rename = "EniIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    #[serde(rename = "EniIpV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip_v6: Option<String>,
    #[serde(rename = "HsmId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
    #[serde(rename = "HsmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHsmRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    pub availability_zone: String,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHsmResponse {
    #[serde(rename = "Hsm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm: Option<Hsm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupResponse {
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Backup {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<String>,
    #[serde(rename = "BackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[serde(rename = "BackupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_state: Option<String>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "CopyTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_timestamp: Option<f64>,
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "DeleteTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_timestamp: Option<f64>,
    #[serde(rename = "HsmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "NeverExpires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    #[serde(rename = "SourceBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    #[serde(rename = "SourceCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHsmRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "EniId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    #[serde(rename = "EniIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    #[serde(rename = "HsmId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHsmResponse {
    #[serde(rename = "HsmId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Shared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "SortAscending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupsResponse {
    #[serde(rename = "Backups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClustersRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
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
pub struct DescribeClustersResponse {
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitializeClusterRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "SignedCert")]
    #[serde(default)]
    pub signed_cert: String,
    #[serde(rename = "TrustAnchor")]
    #[serde(default)]
    pub trust_anchor: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitializeClusterResponse {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
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
pub struct ListTagsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyBackupAttributesRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
    #[serde(rename = "NeverExpires")]
    #[serde(default)]
    pub never_expires: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyBackupAttributesResponse {
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClusterRequest {
    #[serde(rename = "BackupRetentionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "HsmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreBackupRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreBackupResponse {
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagList")]
    #[serde(default)]
    pub tag_list: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeyList")]
    #[serde(default)]
    pub tag_key_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
