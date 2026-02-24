//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-datasync

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTaskExecutionRequest {
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    pub task_execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTaskExecutionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentRequest {
    #[serde(rename = "ActivationKey")]
    #[serde(default)]
    pub activation_key: String,
    #[serde(rename = "AgentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    #[serde(rename = "SubnetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arns: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagListEntry {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentResponse {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationAzureBlobRequest {
    #[serde(rename = "AccessTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "BlobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "ContainerUrl")]
    #[serde(default)]
    pub container_url: String,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "SasConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sas_configuration: Option<AzureBlobSasConfiguration>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmkSecretConfig {
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomSecretConfig {
    #[serde(rename = "SecretAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_role_arn: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AzureBlobSasConfiguration {
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationAzureBlobResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationEfsRequest {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Ec2Config")]
    #[serde(default)]
    pub ec2_config: Ec2Config,
    #[serde(rename = "EfsFilesystemArn")]
    #[serde(default)]
    pub efs_filesystem_arn: String,
    #[serde(rename = "FileSystemAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_access_role_arn: Option<String>,
    #[serde(rename = "InTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_encryption: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Config {
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    pub subnet_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationEfsResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxLustreRequest {
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(default)]
    pub fsx_filesystem_arn: String,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxLustreResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxOntapRequest {
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: FsxProtocol,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "StorageVirtualMachineArn")]
    #[serde(default)]
    pub storage_virtual_machine_arn: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FsxProtocol {
    #[serde(rename = "NFS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_f_s: Option<FsxProtocolNfs>,
    #[serde(rename = "SMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_b: Option<FsxProtocolSmb>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FsxProtocolNfs {
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NfsMountOptions {
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FsxProtocolSmb {
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    pub user: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedSecretConfig {
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmbMountOptions {
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxOntapResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxOpenZfsRequest {
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(default)]
    pub fsx_filesystem_arn: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: FsxProtocol,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxOpenZfsResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxWindowsRequest {
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(default)]
    pub fsx_filesystem_arn: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    #[serde(rename = "User")]
    #[serde(default)]
    pub user: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationFsxWindowsResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationHdfsRequest {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    pub agent_arns: Vec<String>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "KerberosKeytab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<String>,
    #[serde(rename = "KerberosKrb5Conf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_krb5_conf: Option<String>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "KmsKeyProviderUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_provider_uri: Option<String>,
    #[serde(rename = "NameNodes")]
    #[serde(default)]
    pub name_nodes: Vec<HdfsNameNode>,
    #[serde(rename = "QopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qop_configuration: Option<QopConfiguration>,
    #[serde(rename = "ReplicationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,
    #[serde(rename = "SimpleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_user: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HdfsNameNode {
    #[serde(rename = "Hostname")]
    #[serde(default)]
    pub hostname: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QopConfiguration {
    #[serde(rename = "DataTransferProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_protection: Option<String>,
    #[serde(rename = "RpcProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationHdfsResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationNfsRequest {
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    #[serde(rename = "OnPremConfig")]
    #[serde(default)]
    pub on_prem_config: OnPremConfig,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    pub server_hostname: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    pub subdirectory: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnPremConfig {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    pub agent_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationNfsResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationObjectStorageRequest {
    #[serde(rename = "AccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "SecretKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "ServerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    pub server_hostname: String,
    #[serde(rename = "ServerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "ServerProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationObjectStorageResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationS3Request {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "S3BucketArn")]
    #[serde(default)]
    pub s3_bucket_arn: String,
    #[serde(rename = "S3Config")]
    #[serde(default)]
    pub s3_config: S3Config,
    #[serde(rename = "S3StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Config {
    #[serde(rename = "BucketAccessRoleArn")]
    #[serde(default)]
    pub bucket_access_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationS3Response {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationSmbRequest {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    pub agent_arns: Vec<String>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "DnsIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "KerberosKeytab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<String>,
    #[serde(rename = "KerberosKrb5Conf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_krb5_conf: Option<String>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    pub server_hostname: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    pub subdirectory: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLocationSmbResponse {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskRequest {
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "DestinationLocationArn")]
    #[serde(default)]
    pub destination_location_arn: String,
    #[serde(rename = "Excludes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "ManifestConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_config: Option<ManifestConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    #[serde(rename = "SourceLocationArn")]
    #[serde(default)]
    pub source_location_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    #[serde(rename = "TaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
    #[serde(rename = "TaskReportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_report_config: Option<TaskReportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterRule {
    #[serde(rename = "FilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManifestConfig {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceManifestConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceManifestConfig {
    #[serde(rename = "S3")]
    #[serde(default)]
    pub s3: S3ManifestConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ManifestConfig {
    #[serde(rename = "BucketAccessRoleArn")]
    #[serde(default)]
    pub bucket_access_role_arn: String,
    #[serde(rename = "ManifestObjectPath")]
    #[serde(default)]
    pub manifest_object_path: String,
    #[serde(rename = "ManifestObjectVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_object_version_id: Option<String>,
    #[serde(rename = "S3BucketArn")]
    #[serde(default)]
    pub s3_bucket_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Options {
    #[serde(rename = "Atime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atime: Option<String>,
    #[serde(rename = "BytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_per_second: Option<i64>,
    #[serde(rename = "Gid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "Mtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    #[serde(rename = "ObjectTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_tags: Option<String>,
    #[serde(rename = "OverwriteMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_mode: Option<String>,
    #[serde(rename = "PosixPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_permissions: Option<String>,
    #[serde(rename = "PreserveDeletedFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_deleted_files: Option<String>,
    #[serde(rename = "PreserveDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_devices: Option<String>,
    #[serde(rename = "SecurityDescriptorCopyFlags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_descriptor_copy_flags: Option<String>,
    #[serde(rename = "TaskQueueing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queueing: Option<String>,
    #[serde(rename = "TransferMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<String>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "VerifyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSchedule {
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskReportConfig {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ReportDestination>,
    #[serde(rename = "ObjectVersionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version_ids: Option<String>,
    #[serde(rename = "OutputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "Overrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ReportOverrides>,
    #[serde(rename = "ReportLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDestination {
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<ReportDestinationS3>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDestinationS3 {
    #[serde(rename = "BucketAccessRoleArn")]
    #[serde(default)]
    pub bucket_access_role_arn: String,
    #[serde(rename = "S3BucketArn")]
    #[serde(default)]
    pub s3_bucket_arn: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportOverrides {
    #[serde(rename = "Deleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<ReportOverride>,
    #[serde(rename = "Skipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<ReportOverride>,
    #[serde(rename = "Transferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred: Option<ReportOverride>,
    #[serde(rename = "Verified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<ReportOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportOverride {
    #[serde(rename = "ReportLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskResponse {
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentRequest {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    pub agent_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLocationRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLocationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskRequest {
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    pub task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentRequest {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    pub agent_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentResponse {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "LastConnectionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(rename = "PrivateLinkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_config: Option<PrivateLinkConfig>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Platform {
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateLinkConfig {
    #[serde(rename = "PrivateLinkEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_endpoint: Option<String>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    #[serde(rename = "SubnetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arns: Option<Vec<String>>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationAzureBlobRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationAzureBlobResponse {
    #[serde(rename = "AccessTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "BlobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationEfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationEfsResponse {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Ec2Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_config: Option<Ec2Config>,
    #[serde(rename = "FileSystemAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_access_role_arn: Option<String>,
    #[serde(rename = "InTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_encryption: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxLustreRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxLustreResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxOntapRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxOntapResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_filesystem_arn: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<FsxProtocol>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    #[serde(rename = "StorageVirtualMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxOpenZfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxOpenZfsResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<FsxProtocol>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxWindowsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationFsxWindowsResponse {
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
    #[serde(rename = "SecurityGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationHdfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationHdfsResponse {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "KmsKeyProviderUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_provider_uri: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
    #[serde(rename = "NameNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_nodes: Option<Vec<HdfsNameNode>>,
    #[serde(rename = "QopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qop_configuration: Option<QopConfiguration>,
    #[serde(rename = "ReplicationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,
    #[serde(rename = "SimpleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationNfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationNfsResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    #[serde(rename = "OnPremConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_prem_config: Option<OnPremConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationObjectStorageRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationObjectStorageResponse {
    #[serde(rename = "AccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
    #[serde(rename = "ServerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "ServerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "ServerProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationS3Request {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationS3Response {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "S3Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    #[serde(rename = "S3StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationSmbRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLocationSmbResponse {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "DnsIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "ManagedSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_secret_config: Option<ManagedSecretConfig>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskExecutionRequest {
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    pub task_execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskExecutionResponse {
    #[serde(rename = "BytesCompressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_compressed: Option<i64>,
    #[serde(rename = "BytesTransferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    #[serde(rename = "BytesWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EstimatedBytesToTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_bytes_to_transfer: Option<i64>,
    #[serde(rename = "EstimatedFilesToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_files_to_delete: Option<i64>,
    #[serde(rename = "EstimatedFilesToTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_files_to_transfer: Option<i64>,
    #[serde(rename = "EstimatedFoldersToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_folders_to_delete: Option<i64>,
    #[serde(rename = "EstimatedFoldersToTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_folders_to_transfer: Option<i64>,
    #[serde(rename = "Excludes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    #[serde(rename = "FilesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_deleted: Option<i64>,
    #[serde(rename = "FilesFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_failed: Option<TaskExecutionFilesFailedDetail>,
    #[serde(rename = "FilesListed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_listed: Option<TaskExecutionFilesListedDetail>,
    #[serde(rename = "FilesPrepared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_prepared: Option<i64>,
    #[serde(rename = "FilesSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_skipped: Option<i64>,
    #[serde(rename = "FilesTransferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_transferred: Option<i64>,
    #[serde(rename = "FilesVerified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_verified: Option<i64>,
    #[serde(rename = "FoldersDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_deleted: Option<i64>,
    #[serde(rename = "FoldersFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_failed: Option<TaskExecutionFoldersFailedDetail>,
    #[serde(rename = "FoldersListed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_listed: Option<TaskExecutionFoldersListedDetail>,
    #[serde(rename = "FoldersPrepared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_prepared: Option<i64>,
    #[serde(rename = "FoldersSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_skipped: Option<i64>,
    #[serde(rename = "FoldersTransferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_transferred: Option<i64>,
    #[serde(rename = "FoldersVerified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_verified: Option<i64>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "LaunchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    #[serde(rename = "ManifestConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_config: Option<ManifestConfig>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[serde(rename = "ReportResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_result: Option<ReportResult>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TaskExecutionResultDetail>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
    #[serde(rename = "TaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
    #[serde(rename = "TaskReportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_report_config: Option<TaskReportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionFilesFailedDetail {
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<i64>,
    #[serde(rename = "Prepare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare: Option<i64>,
    #[serde(rename = "Transfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<i64>,
    #[serde(rename = "Verify")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionFilesListedDetail {
    #[serde(rename = "AtDestinationForDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_destination_for_delete: Option<i64>,
    #[serde(rename = "AtSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_source: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionFoldersFailedDetail {
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<i64>,
    #[serde(rename = "List")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<i64>,
    #[serde(rename = "Prepare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare: Option<i64>,
    #[serde(rename = "Transfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<i64>,
    #[serde(rename = "Verify")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionFoldersListedDetail {
    #[serde(rename = "AtDestinationForDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_destination_for_delete: Option<i64>,
    #[serde(rename = "AtSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_source: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportResult {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionResultDetail {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "PrepareDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_duration: Option<i64>,
    #[serde(rename = "PrepareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_status: Option<String>,
    #[serde(rename = "TotalDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[serde(rename = "TransferDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_duration: Option<i64>,
    #[serde(rename = "TransferStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
    #[serde(rename = "VerifyDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_duration: Option<i64>,
    #[serde(rename = "VerifyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskRequest {
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    pub task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskResponse {
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentTaskExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_task_execution_arn: Option<String>,
    #[serde(rename = "DestinationLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location_arn: Option<String>,
    #[serde(rename = "DestinationNetworkInterfaceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_network_interface_arns: Option<Vec<String>>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "Excludes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "ManifestConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_config: Option<ManifestConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    #[serde(rename = "ScheduleDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_details: Option<TaskScheduleDetails>,
    #[serde(rename = "SourceLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_arn: Option<String>,
    #[serde(rename = "SourceNetworkInterfaceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_network_interface_arns: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
    #[serde(rename = "TaskReportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_report_config: Option<TaskReportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskScheduleDetails {
    #[serde(rename = "DisabledBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_by: Option<String>,
    #[serde(rename = "DisabledReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "StatusUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentsRequest {
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
pub struct ListAgentsResponse {
    #[serde(rename = "Agents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<AgentListEntry>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentListEntry {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLocationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<LocationFilter>>,
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
pub struct LocationFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLocationsResponse {
    #[serde(rename = "Locations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<LocationListEntry>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocationListEntry {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
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
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
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
    pub tags: Option<Vec<TagListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskExecutionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskExecutionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_executions: Option<Vec<TaskExecutionListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskExecutionListEntry {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
    #[serde(rename = "TaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TaskFilter>>,
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
pub struct TaskFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskListEntry {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskExecutionRequest {
    #[serde(rename = "Excludes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "ManifestConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_config: Option<ManifestConfig>,
    #[serde(rename = "OverrideOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_options: Option<Options>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    pub task_arn: String,
    #[serde(rename = "TaskReportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_report_config: Option<TaskReportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskExecutionResponse {
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<TagListEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "Keys")]
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentRequest {
    #[serde(rename = "AgentArn")]
    #[serde(default)]
    pub agent_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationAzureBlobRequest {
    #[serde(rename = "AccessTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "BlobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "SasConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sas_configuration: Option<AzureBlobSasConfiguration>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationAzureBlobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationEfsRequest {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "FileSystemAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_access_role_arn: Option<String>,
    #[serde(rename = "InTransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_encryption: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationEfsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxLustreRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxLustreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxOntapRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<FsxUpdateProtocol>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FsxUpdateProtocol {
    #[serde(rename = "NFS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_f_s: Option<FsxProtocolNfs>,
    #[serde(rename = "SMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_b: Option<FsxUpdateProtocolSmb>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FsxUpdateProtocolSmb {
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxOntapResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxOpenZfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<FsxProtocol>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxOpenZfsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxWindowsRequest {
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationFsxWindowsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationHdfsRequest {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "KerberosKeytab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<String>,
    #[serde(rename = "KerberosKrb5Conf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_krb5_conf: Option<String>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "KmsKeyProviderUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_provider_uri: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "NameNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_nodes: Option<Vec<HdfsNameNode>>,
    #[serde(rename = "QopConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qop_configuration: Option<QopConfiguration>,
    #[serde(rename = "ReplicationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<i32>,
    #[serde(rename = "SimpleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_user: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationHdfsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationNfsRequest {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    #[serde(rename = "OnPremConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_prem_config: Option<OnPremConfig>,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_hostname: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationNfsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationObjectStorageRequest {
    #[serde(rename = "AccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "SecretKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "ServerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_hostname: Option<String>,
    #[serde(rename = "ServerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "ServerProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationObjectStorageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationS3Request {
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "S3Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    #[serde(rename = "S3StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationS3Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationSmbRequest {
    #[serde(rename = "AgentArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "CmkSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_secret_config: Option<CmkSecretConfig>,
    #[serde(rename = "CustomSecretConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_secret_config: Option<CustomSecretConfig>,
    #[serde(rename = "DnsIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "KerberosKeytab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<String>,
    #[serde(rename = "KerberosKrb5Conf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_krb5_conf: Option<String>,
    #[serde(rename = "KerberosPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_principal: Option<String>,
    #[serde(rename = "LocationArn")]
    #[serde(default)]
    pub location_arn: String,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ServerHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_hostname: Option<String>,
    #[serde(rename = "Subdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLocationSmbResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskExecutionRequest {
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: Options,
    #[serde(rename = "TaskExecutionArn")]
    #[serde(default)]
    pub task_execution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskExecutionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskRequest {
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "Excludes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "ManifestConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_config: Option<ManifestConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    pub task_arn: String,
    #[serde(rename = "TaskReportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_report_config: Option<TaskReportConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskResponse {}
