//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-transfer

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessRequest {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HomeDirectoryMapEntry {
    #[serde(rename = "Entry")]
    #[serde(default)]
    pub entry: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PosixProfile {
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
pub struct CreateAccessResponse {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgreementRequest {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    pub access_role: String,
    #[serde(rename = "BaseDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<String>,
    #[serde(rename = "CustomDirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_directories: Option<CustomDirectoriesType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnforceMessageSigning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_message_signing: Option<String>,
    #[serde(rename = "LocalProfileId")]
    #[serde(default)]
    pub local_profile_id: String,
    #[serde(rename = "PartnerProfileId")]
    #[serde(default)]
    pub partner_profile_id: String,
    #[serde(rename = "PreserveFilename")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_filename: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
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
pub struct CustomDirectoriesType {
    #[serde(rename = "FailedFilesDirectory")]
    #[serde(default)]
    pub failed_files_directory: String,
    #[serde(rename = "MdnFilesDirectory")]
    #[serde(default)]
    pub mdn_files_directory: String,
    #[serde(rename = "PayloadFilesDirectory")]
    #[serde(default)]
    pub payload_files_directory: String,
    #[serde(rename = "StatusFilesDirectory")]
    #[serde(default)]
    pub status_files_directory: String,
    #[serde(rename = "TemporaryFilesDirectory")]
    #[serde(default)]
    pub temporary_files_directory: String,
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
pub struct CreateAgreementResponse {
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorRequest {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    pub access_role: String,
    #[serde(rename = "As2Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_config: Option<As2ConnectorConfig>,
    #[serde(rename = "EgressConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<ConnectorEgressConfig>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "SftpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_config: Option<SftpConnectorConfig>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct As2ConnectorConfig {
    #[serde(rename = "AsyncMdnConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_mdn_config: Option<As2AsyncMdnConnectorConfig>,
    #[serde(rename = "BasicAuthSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_secret_id: Option<String>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "LocalProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_profile_id: Option<String>,
    #[serde(rename = "MdnResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdn_response: Option<String>,
    #[serde(rename = "MdnSigningAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdn_signing_algorithm: Option<String>,
    #[serde(rename = "MessageSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_subject: Option<String>,
    #[serde(rename = "PartnerProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_profile_id: Option<String>,
    #[serde(rename = "PreserveContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_content_type: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct As2AsyncMdnConnectorConfig {
    #[serde(rename = "ServerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ids: Option<Vec<String>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorEgressConfig {
    #[serde(rename = "VpcLattice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice: Option<ConnectorVpcLatticeEgressConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorVpcLatticeEgressConfig {
    #[serde(rename = "PortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[serde(rename = "ResourceConfigurationArn")]
    #[serde(default)]
    pub resource_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SftpConnectorConfig {
    #[serde(rename = "MaxConcurrentConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_connections: Option<i32>,
    #[serde(rename = "TrustedHostKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_host_keys: Option<Vec<String>>,
    #[serde(rename = "UserSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorResponse {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProfileRequest {
    #[serde(rename = "As2Id")]
    #[serde(default)]
    pub as2_id: String,
    #[serde(rename = "CertificateIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_ids: Option<Vec<String>>,
    #[serde(rename = "ProfileType")]
    #[serde(default)]
    pub profile_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProfileResponse {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServerRequest {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "HostKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    #[serde(rename = "IdentityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    #[serde(rename = "IdentityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "PostAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_authentication_login_banner: Option<String>,
    #[serde(rename = "PreAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_authentication_login_banner: Option<String>,
    #[serde(rename = "ProtocolDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_details: Option<ProtocolDetails>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "S3StorageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_options: Option<S3StorageOptions>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "StructuredLogDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_log_destinations: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WorkflowDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_details: Option<WorkflowDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointDetails {
    #[serde(rename = "AddressAllocationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_allocation_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProviderDetails {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Function")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(rename = "InvocationRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_role: Option<String>,
    #[serde(rename = "SftpAuthenticationMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_authentication_methods: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtocolDetails {
    #[serde(rename = "As2Transports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_transports: Option<Vec<String>>,
    #[serde(rename = "PassiveIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passive_ip: Option<String>,
    #[serde(rename = "SetStatOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_stat_option: Option<String>,
    #[serde(rename = "TlsSessionResumptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_session_resumption_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3StorageOptions {
    #[serde(rename = "DirectoryListingOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_listing_optimization: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowDetails {
    #[serde(rename = "OnPartialUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_partial_upload: Option<Vec<WorkflowDetail>>,
    #[serde(rename = "OnUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_upload: Option<Vec<WorkflowDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowDetail {
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    pub execution_role: String,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServerResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "SshPublicKeyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_body: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebAppRequest {
    #[serde(rename = "AccessEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[serde(rename = "EndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<WebAppEndpointDetails>,
    #[serde(rename = "IdentityProviderDetails")]
    #[serde(default)]
    pub identity_provider_details: WebAppIdentityProviderDetails,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WebAppEndpointPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_endpoint_policy: Option<String>,
    #[serde(rename = "WebAppUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_units: Option<WebAppUnits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAppEndpointDetails {
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<WebAppVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAppVpcConfig {
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAppIdentityProviderDetails {
    #[serde(rename = "IdentityCenterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_config: Option<IdentityCenterConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterConfig {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAppUnits {
    #[serde(rename = "Provisioned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebAppResponse {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkflowRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OnExceptionSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exception_steps: Option<Vec<WorkflowStep>>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    pub steps: Vec<WorkflowStep>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowStep {
    #[serde(rename = "CopyStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_step_details: Option<CopyStepDetails>,
    #[serde(rename = "CustomStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_step_details: Option<CustomStepDetails>,
    #[serde(rename = "DecryptStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypt_step_details: Option<DecryptStepDetails>,
    #[serde(rename = "DeleteStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_step_details: Option<DeleteStepDetails>,
    #[serde(rename = "TagStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_step_details: Option<TagStepDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyStepDetails {
    #[serde(rename = "DestinationFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_file_location: Option<InputFileLocation>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverwriteExisting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existing: Option<String>,
    #[serde(rename = "SourceFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputFileLocation {
    #[serde(rename = "EfsFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_file_location: Option<EfsFileLocation>,
    #[serde(rename = "S3FileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_file_location: Option<S3InputFileLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EfsFileLocation {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3InputFileLocation {
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
pub struct CustomStepDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecryptStepDetails {
    #[serde(rename = "DestinationFileLocation")]
    #[serde(default)]
    pub destination_file_location: InputFileLocation,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverwriteExisting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existing: Option<String>,
    #[serde(rename = "SourceFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStepDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagStepDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_location: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<S3Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkflowResponse {
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessRequest {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgreementRequest {
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHostKeyRequest {
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    pub host_key_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProfileRequest {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServerRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSshPublicKeyRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "SshPublicKeyId")]
    #[serde(default)]
    pub ssh_public_key_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebAppCustomizationRequest {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebAppRequest {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkflowRequest {
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccessRequest {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccessResponse {
    #[serde(rename = "Access")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<DescribedAccess>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedAccess {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgreementRequest {
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgreementResponse {
    #[serde(rename = "Agreement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement: Option<DescribedAgreement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedAgreement {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_role: Option<String>,
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BaseDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<String>,
    #[serde(rename = "CustomDirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_directories: Option<CustomDirectoriesType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnforceMessageSigning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_message_signing: Option<String>,
    #[serde(rename = "LocalProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_profile_id: Option<String>,
    #[serde(rename = "PartnerProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_profile_id: Option<String>,
    #[serde(rename = "PreserveFilename")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_filename: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
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
pub struct DescribeCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<DescribedCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedCertificate {
    #[serde(rename = "ActiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<f64>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InactiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_date: Option<f64>,
    #[serde(rename = "NotAfterDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after_date: Option<f64>,
    #[serde(rename = "NotBeforeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before_date: Option<f64>,
    #[serde(rename = "Serial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Usage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorResponse {
    #[serde(rename = "Connector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<DescribedConnector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedConnector {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_role: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "As2Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_config: Option<As2ConnectorConfig>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "EgressConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<DescribedConnectorEgressConfig>,
    #[serde(rename = "EgressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_type: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "ServiceManagedEgressIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_egress_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "SftpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_config: Option<SftpConnectorConfig>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedConnectorEgressConfig {
    #[serde(rename = "VpcLattice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice: Option<DescribedConnectorVpcLatticeEgressConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedConnectorVpcLatticeEgressConfig {
    #[serde(rename = "PortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[serde(rename = "ResourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionRequest {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionResponse {
    #[serde(rename = "Execution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<DescribedExecution>,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedExecution {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "InitialFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_file_location: Option<FileLocation>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<ExecutionResults>,
    #[serde(rename = "ServiceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_metadata: Option<ServiceMetadata>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileLocation {
    #[serde(rename = "EfsFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_file_location: Option<EfsFileLocation>,
    #[serde(rename = "S3FileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_file_location: Option<S3FileLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3FileLocation {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Etag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionResults {
    #[serde(rename = "OnExceptionSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exception_steps: Option<Vec<ExecutionStepResult>>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<ExecutionStepResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStepResult {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ExecutionError>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<String>,
    #[serde(rename = "StepType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceMetadata {
    #[serde(rename = "UserDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_details: Option<UserDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserDetails {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHostKeyRequest {
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    pub host_key_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHostKeyResponse {
    #[serde(rename = "HostKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<DescribedHostKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedHostKey {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DateImported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_imported: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HostKeyFingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_fingerprint: Option<String>,
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProfileRequest {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProfileResponse {
    #[serde(rename = "Profile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<DescribedProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedProfile {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "As2Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_id: Option<String>,
    #[serde(rename = "CertificateIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_ids: Option<Vec<String>>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "ProfileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityPolicyRequest {
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    pub security_policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityPolicyResponse {
    #[serde(rename = "SecurityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<DescribedSecurityPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedSecurityPolicy {
    #[serde(rename = "Fips")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fips: Option<bool>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "SshCiphers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_ciphers: Option<Vec<String>>,
    #[serde(rename = "SshHostKeyAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_host_key_algorithms: Option<Vec<String>>,
    #[serde(rename = "SshKexs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_kexs: Option<Vec<String>>,
    #[serde(rename = "SshMacs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_macs: Option<Vec<String>>,
    #[serde(rename = "TlsCiphers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_ciphers: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServerRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServerResponse {
    #[serde(rename = "Server")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<DescribedServer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedServer {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "As2ServiceManagedEgressIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_service_managed_egress_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "HostKeyFingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_fingerprint: Option<String>,
    #[serde(rename = "IdentityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    #[serde(rename = "IdentityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "PostAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_authentication_login_banner: Option<String>,
    #[serde(rename = "PreAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_authentication_login_banner: Option<String>,
    #[serde(rename = "ProtocolDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_details: Option<ProtocolDetails>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "S3StorageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_options: Option<S3StorageOptions>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StructuredLogDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_log_destinations: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    #[serde(rename = "WorkflowDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_details: Option<WorkflowDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<DescribedUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedUser {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SshPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<SshPublicKey>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SshPublicKey {
    #[serde(rename = "DateImported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_imported: Option<f64>,
    #[serde(rename = "SshPublicKeyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_body: Option<String>,
    #[serde(rename = "SshPublicKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWebAppCustomizationRequest {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWebAppCustomizationResponse {
    #[serde(rename = "WebAppCustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_customization: Option<DescribedWebAppCustomization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWebAppCustomization {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FaviconFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_file: Option<String>,
    #[serde(rename = "LogoFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWebAppRequest {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWebAppResponse {
    #[serde(rename = "WebApp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<DescribedWebApp>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWebApp {
    #[serde(rename = "AccessEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DescribedEndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub described_endpoint_details: Option<DescribedWebAppEndpointDetails>,
    #[serde(rename = "DescribedIdentityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub described_identity_provider_details: Option<DescribedWebAppIdentityProviderDetails>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WebAppEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_endpoint: Option<String>,
    #[serde(rename = "WebAppEndpointPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_endpoint_policy: Option<String>,
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
    #[serde(rename = "WebAppUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_units: Option<WebAppUnits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWebAppEndpointDetails {
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<DescribedWebAppVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWebAppVpcConfig {
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWebAppIdentityProviderDetails {
    #[serde(rename = "IdentityCenterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_config: Option<DescribedIdentityCenterConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedIdentityCenterConfig {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkflowRequest {
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkflowResponse {
    #[serde(rename = "Workflow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<DescribedWorkflow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribedWorkflow {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OnExceptionSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exception_steps: Option<Vec<WorkflowStep>>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<WorkflowStep>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateRequest {
    #[serde(rename = "ActiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<f64>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    pub certificate: String,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InactiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_date: Option<f64>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Usage")]
    #[serde(default)]
    pub usage: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateResponse {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportHostKeyRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HostKeyBody")]
    #[serde(default)]
    pub host_key_body: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportHostKeyResponse {
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_id: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSshPublicKeyRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "SshPublicKeyBody")]
    #[serde(default)]
    pub ssh_public_key_body: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSshPublicKeyResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "SshPublicKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessesResponse {
    #[serde(rename = "Accesses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accesses: Option<Vec<ListedAccess>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedAccess {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgreementsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgreementsResponse {
    #[serde(rename = "Agreements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreements: Option<Vec<ListedAgreement>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedAgreement {
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LocalProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_profile_id: Option<String>,
    #[serde(rename = "PartnerProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_profile_id: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesRequest {
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
pub struct ListCertificatesResponse {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<ListedCertificate>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedCertificate {
    #[serde(rename = "ActiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<f64>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InactiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_date: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Usage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsRequest {
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
pub struct ListConnectorsResponse {
    #[serde(rename = "Connectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ListedConnector>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedConnector {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsResponse {
    #[serde(rename = "Executions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<Vec<ListedExecution>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedExecution {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "InitialFileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_file_location: Option<FileLocation>,
    #[serde(rename = "ServiceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_metadata: Option<ServiceMetadata>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileTransferResultsRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransferId")]
    #[serde(default)]
    pub transfer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFileTransferResultsResponse {
    #[serde(rename = "FileTransferResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_transfer_results: Option<Vec<ConnectorFileTransferResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorFileTransferResult {
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHostKeysRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHostKeysResponse {
    #[serde(rename = "HostKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_keys: Option<Vec<ListedHostKey>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedHostKey {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DateImported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_imported: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProfilesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProfileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProfilesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Profiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<ListedProfile>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedProfile {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "As2Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_id: Option<String>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "ProfileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityPoliciesRequest {
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
pub struct ListSecurityPoliciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityPolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServersRequest {
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
pub struct ListServersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Servers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ListedServer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedServer {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "IdentityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UserCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
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
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
pub struct ListUsersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ListedUser>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedUser {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SshPublicKeyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_count: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebAppsRequest {
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
pub struct ListWebAppsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WebApps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_apps: Option<Vec<ListedWebApp>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedWebApp {
    #[serde(rename = "AccessEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "WebAppEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_endpoint: Option<String>,
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkflowsRequest {
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
pub struct ListWorkflowsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Workflows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<ListedWorkflow>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListedWorkflow {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendWorkflowStepStateRequest {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
    #[serde(rename = "WorkflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendWorkflowStepStateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDirectoryListingRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "OutputDirectoryPath")]
    #[serde(default)]
    pub output_directory_path: String,
    #[serde(rename = "RemoteDirectoryPath")]
    #[serde(default)]
    pub remote_directory_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDirectoryListingResponse {
    #[serde(rename = "ListingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,
    #[serde(rename = "OutputFileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFileTransferRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "CustomHttpHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_http_headers: Option<Vec<CustomHttpHeader>>,
    #[serde(rename = "LocalDirectoryPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_directory_path: Option<String>,
    #[serde(rename = "RemoteDirectoryPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_directory_path: Option<String>,
    #[serde(rename = "RetrieveFilePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_file_paths: Option<Vec<String>>,
    #[serde(rename = "SendFilePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_file_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomHttpHeader {
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
pub struct StartFileTransferResponse {
    #[serde(rename = "TransferId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemoteDeleteRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "DeletePath")]
    #[serde(default)]
    pub delete_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemoteDeleteResponse {
    #[serde(rename = "DeleteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemoteMoveRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    pub source_path: String,
    #[serde(rename = "TargetPath")]
    #[serde(default)]
    pub target_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemoteMoveResponse {
    #[serde(rename = "MoveId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartServerRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopServerRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionRequest {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionResponse {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "SftpConnectionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_connection_details: Option<SftpConnectorConnectionDetails>,
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
pub struct SftpConnectorConnectionDetails {
    #[serde(rename = "HostKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestIdentityProviderRequest {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "ServerProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<String>,
    #[serde(rename = "SourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
    #[serde(rename = "UserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestIdentityProviderResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Response")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessRequest {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessResponse {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgreementRequest {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_role: Option<String>,
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(rename = "BaseDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<String>,
    #[serde(rename = "CustomDirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_directories: Option<CustomDirectoriesType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnforceMessageSigning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_message_signing: Option<String>,
    #[serde(rename = "LocalProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_profile_id: Option<String>,
    #[serde(rename = "PartnerProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_profile_id: Option<String>,
    #[serde(rename = "PreserveFilename")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_filename: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgreementResponse {
    #[serde(rename = "AgreementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateRequest {
    #[serde(rename = "ActiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<f64>,
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InactiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateResponse {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorRequest {
    #[serde(rename = "AccessRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_role: Option<String>,
    #[serde(rename = "As2Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as2_config: Option<As2ConnectorConfig>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "EgressConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<UpdateConnectorEgressConfig>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "SftpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_config: Option<SftpConnectorConfig>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorEgressConfig {
    #[serde(rename = "VpcLattice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_lattice: Option<UpdateConnectorVpcLatticeEgressConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorVpcLatticeEgressConfig {
    #[serde(rename = "PortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[serde(rename = "ResourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorResponse {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHostKeyRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    pub host_key_id: String,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHostKeyResponse {
    #[serde(rename = "HostKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_id: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProfileRequest {
    #[serde(rename = "CertificateIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_ids: Option<Vec<String>>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProfileResponse {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServerRequest {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "EndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "HostKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    #[serde(rename = "IdentityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    #[serde(rename = "IdentityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoggingRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    #[serde(rename = "PostAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_authentication_login_banner: Option<String>,
    #[serde(rename = "PreAuthenticationLoginBanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_authentication_login_banner: Option<String>,
    #[serde(rename = "ProtocolDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_details: Option<ProtocolDetails>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "S3StorageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_options: Option<S3StorageOptions>,
    #[serde(rename = "SecurityPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "StructuredLogDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_log_destinations: Option<Vec<String>>,
    #[serde(rename = "WorkflowDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_details: Option<WorkflowDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServerResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "HomeDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    #[serde(rename = "HomeDirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PosixProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    pub server_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserResponse {
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppCustomizationRequest {
    #[serde(rename = "FaviconFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_file: Option<String>,
    #[serde(rename = "LogoFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppCustomizationResponse {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppRequest {
    #[serde(rename = "AccessEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[serde(rename = "EndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<UpdateWebAppEndpointDetails>,
    #[serde(rename = "IdentityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<UpdateWebAppIdentityProviderDetails>,
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    pub web_app_id: String,
    #[serde(rename = "WebAppUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_units: Option<WebAppUnits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppEndpointDetails {
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<UpdateWebAppVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppVpcConfig {
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppIdentityProviderDetails {
    #[serde(rename = "IdentityCenterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_config: Option<UpdateWebAppIdentityCenterConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppIdentityCenterConfig {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebAppResponse {
    #[serde(rename = "WebAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_id: Option<String>,
}
