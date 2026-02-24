//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-directory

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptSharedDirectoryRequest {
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    pub shared_directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptSharedDirectoryResult {
    #[serde(rename = "SharedDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory: Option<SharedDirectory>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharedDirectory {
    #[serde(rename = "CreatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "OwnerDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_id: Option<String>,
    #[serde(rename = "ShareMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    #[serde(rename = "ShareNotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    #[serde(rename = "SharedAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_account_id: Option<String>,
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddIpRoutesRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "IpRoutes")]
    #[serde(default)]
    pub ip_routes: Vec<IpRoute>,
    #[serde(rename = "UpdateSecurityGroupForDirectoryControllers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_security_group_for_directory_controllers: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpRoute {
    #[serde(rename = "CidrIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    #[serde(rename = "CidrIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddIpRoutesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddRegionRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
    #[serde(rename = "VPCSettings")]
    #[serde(default)]
    pub v_p_c_settings: DirectoryVpcSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryVpcSettings {
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddRegionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToResourceRequest {
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
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelSchemaExtensionRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "SchemaExtensionId")]
    #[serde(default)]
    pub schema_extension_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelSchemaExtensionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectDirectoryRequest {
    #[serde(rename = "ConnectSettings")]
    #[serde(default)]
    pub connect_settings: DirectoryConnectSettings,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "ShortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryConnectSettings {
    #[serde(rename = "CustomerDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_dns_ips: Option<Vec<String>>,
    #[serde(rename = "CustomerDnsIpsV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_dns_ips_v6: Option<Vec<String>>,
    #[serde(rename = "CustomerUserName")]
    #[serde(default)]
    pub customer_user_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectDirectoryResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAliasRequest {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAliasResult {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComputerRequest {
    #[serde(rename = "ComputerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    pub computer_name: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComputerResult {
    #[serde(rename = "Computer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer: Option<Computer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Computer {
    #[serde(rename = "ComputerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    #[serde(rename = "ComputerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_id: Option<String>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConditionalForwarderRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "DnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    pub remote_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConditionalForwarderResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectoryRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "ShortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectoryResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHybridADRequest {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    pub secret_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHybridADResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogSubscriptionRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogSubscriptionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMicrosoftADRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "ShortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    pub vpc_settings: DirectoryVpcSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMicrosoftADResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotResult {
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustRequest {
    #[serde(rename = "ConditionalForwarderIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarder_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "ConditionalForwarderIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarder_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    pub remote_domain_name: String,
    #[serde(rename = "SelectiveAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    #[serde(rename = "TrustDirection")]
    #[serde(default)]
    pub trust_direction: String,
    #[serde(rename = "TrustPassword")]
    #[serde(default)]
    pub trust_password: String,
    #[serde(rename = "TrustType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustResult {
    #[serde(rename = "TrustId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteADAssessmentRequest {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    pub assessment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteADAssessmentResult {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConditionalForwarderRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    pub remote_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConditionalForwarderResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectoryRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectoryResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogSubscriptionRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogSubscriptionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotRequest {
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotResult {
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustRequest {
    #[serde(rename = "DeleteAssociatedConditionalForwarder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_associated_conditional_forwarder: Option<bool>,
    #[serde(rename = "TrustId")]
    #[serde(default)]
    pub trust_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustResult {
    #[serde(rename = "TrustId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterCertificateResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterEventTopicRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterEventTopicResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeADAssessmentRequest {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    pub assessment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeADAssessmentResult {
    #[serde(rename = "Assessment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
    #[serde(rename = "AssessmentReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_reports: Option<Vec<AssessmentReport>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Assessment {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "CustomerDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_dns_ips: Option<Vec<String>>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
    #[serde(rename = "ReportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SelfManagedInstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_instance_ids: Option<Vec<String>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentReport {
    #[serde(rename = "DomainControllerIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_ip: Option<String>,
    #[serde(rename = "Validations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<AssessmentValidation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentValidation {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCAEnrollmentPolicyRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCAEnrollmentPolicyResult {
    #[serde(rename = "CaEnrollmentPolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_enrollment_policy_status: Option<String>,
    #[serde(rename = "CaEnrollmentPolicyStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_enrollment_policy_status_reason: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "PcaConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pca_connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateResult {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "ClientCertAuthSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_auth_settings: Option<ClientCertAuthSettings>,
    #[serde(rename = "CommonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[serde(rename = "ExpiryDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<f64>,
    #[serde(rename = "RegisteredDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCertAuthSettings {
    #[serde(rename = "OCSPUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_c_s_p_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientAuthenticationSettingsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientAuthenticationSettingsResult {
    #[serde(rename = "ClientAuthenticationSettingsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication_settings_info: Option<Vec<ClientAuthenticationSettingInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientAuthenticationSettingInfo {
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConditionalForwardersRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RemoteDomainNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConditionalForwardersResult {
    #[serde(rename = "ConditionalForwarders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarders: Option<Vec<ConditionalForwarder>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalForwarder {
    #[serde(rename = "DnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    #[serde(rename = "ReplicationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectoriesRequest {
    #[serde(rename = "DirectoryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectoriesResult {
    #[serde(rename = "DirectoryDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_descriptions: Option<Vec<DirectoryDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryDescription {
    #[serde(rename = "AccessUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_url: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "ConnectSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_settings: Option<DirectoryConnectSettingsDescription>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DesiredNumberOfDomainControllers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_of_domain_controllers: Option<i32>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "HybridSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_settings: Option<HybridSettingsDescription>,
    #[serde(rename = "LaunchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OsVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "OwnerDirectoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_description: Option<OwnerDirectoryDescription>,
    #[serde(rename = "RadiusSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    #[serde(rename = "RadiusStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    #[serde(rename = "RegionsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_info: Option<RegionsInfo>,
    #[serde(rename = "ShareMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    #[serde(rename = "ShareNotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    #[serde(rename = "ShortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "SsoEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(rename = "StageLastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_last_updated_date_time: Option<f64>,
    #[serde(rename = "StageReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryConnectSettingsDescription {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "ConnectIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_ips: Option<Vec<String>>,
    #[serde(rename = "ConnectIpsV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_ips_v6: Option<Vec<String>>,
    #[serde(rename = "CustomerUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
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
pub struct HybridSettingsDescription {
    #[serde(rename = "SelfManagedDnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "SelfManagedInstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_instance_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OwnerDirectoryDescription {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "RadiusSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    #[serde(rename = "RadiusStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadiusSettings {
    #[serde(rename = "AuthenticationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_protocol: Option<String>,
    #[serde(rename = "DisplayLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
    #[serde(rename = "RadiusPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_port: Option<i32>,
    #[serde(rename = "RadiusRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_retries: Option<i32>,
    #[serde(rename = "RadiusServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_servers: Option<Vec<String>>,
    #[serde(rename = "RadiusServersIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_servers_ipv6: Option<Vec<String>>,
    #[serde(rename = "RadiusTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_timeout: Option<i32>,
    #[serde(rename = "SharedSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    #[serde(rename = "UseSameUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_same_username: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryVpcSettingsDescription {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
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
pub struct RegionsInfo {
    #[serde(rename = "AdditionalRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_regions: Option<Vec<String>>,
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectoryDataAccessRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectoryDataAccessResult {
    #[serde(rename = "DataAccessStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainControllersRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "DomainControllerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_ids: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainControllersResult {
    #[serde(rename = "DomainControllers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controllers: Option<Vec<DomainController>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainController {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DnsIpAddr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addr: Option<String>,
    #[serde(rename = "DnsIpv6Addr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addr: Option<String>,
    #[serde(rename = "DomainControllerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_id: Option<String>,
    #[serde(rename = "LaunchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusLastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_last_updated_date_time: Option<f64>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
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
pub struct DescribeEventTopicsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "TopicNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventTopicsResult {
    #[serde(rename = "EventTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_topics: Option<Vec<EventTopic>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventTopic {
    #[serde(rename = "CreatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHybridADUpdateRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHybridADUpdateResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdateActivities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_activities: Option<HybridUpdateActivities>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridUpdateActivities {
    #[serde(rename = "HybridAdministratorAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_administrator_account: Option<Vec<HybridUpdateInfoEntry>>,
    #[serde(rename = "SelfManagedInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_instances: Option<Vec<HybridUpdateInfoEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridUpdateInfoEntry {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "InitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "NewValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<HybridUpdateValue>,
    #[serde(rename = "PreviousValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<HybridUpdateValue>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridUpdateValue {
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLDAPSSettingsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLDAPSSettingsResult {
    #[serde(rename = "LDAPSSettingsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_d_a_p_s_settings_info: Option<Vec<LDAPSSettingInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LDAPSSettingInfo {
    #[serde(rename = "LDAPSStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_d_a_p_s_status: Option<String>,
    #[serde(rename = "LDAPSStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_d_a_p_s_status_reason: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionsDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_description: Option<Vec<RegionDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionDescription {
    #[serde(rename = "DesiredNumberOfDomainControllers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_of_domain_controllers: Option<i32>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "LaunchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "RegionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusLastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_last_updated_date_time: Option<f64>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSettingsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSettingsResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SettingEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_entries: Option<Vec<SettingEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SettingEntry {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "AppliedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_value: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "LastRequestedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_requested_date_time: Option<f64>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestDetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_detailed_status: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<String>,
    #[serde(rename = "RequestStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status_message: Option<String>,
    #[serde(rename = "RequestedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_value: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSharedDirectoriesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OwnerDirectoryId")]
    #[serde(default)]
    pub owner_directory_id: String,
    #[serde(rename = "SharedDirectoryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSharedDirectoriesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SharedDirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directories: Option<Vec<SharedDirectory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSnapshotsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SnapshotIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSnapshotsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Snapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Snapshot {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrustIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Trusts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusts: Option<Vec<Trust>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Trust {
    #[serde(rename = "CreatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    #[serde(rename = "SelectiveAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    #[serde(rename = "StateLastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_last_updated_date_time: Option<f64>,
    #[serde(rename = "TrustDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_direction: Option<String>,
    #[serde(rename = "TrustId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
    #[serde(rename = "TrustState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<String>,
    #[serde(rename = "TrustStateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state_reason: Option<String>,
    #[serde(rename = "TrustType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUpdateDirectoryRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "UpdateType")]
    #[serde(default)]
    pub update_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUpdateDirectoryResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdateActivities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_activities: Option<Vec<UpdateInfoEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInfoEntry {
    #[serde(rename = "InitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "NewValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<UpdateValue>,
    #[serde(rename = "PreviousValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<UpdateValue>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateValue {
    #[serde(rename = "OSUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_s_update_settings: Option<OSUpdateSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OSUpdateSettings {
    #[serde(rename = "OSVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_s_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableCAEnrollmentPolicyRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableCAEnrollmentPolicyResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableClientAuthenticationRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableClientAuthenticationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDirectoryDataAccessRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDirectoryDataAccessResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableLDAPSRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableLDAPSResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableRadiusRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableRadiusResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSsoRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSsoResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableCAEnrollmentPolicyRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "PcaConnectorArn")]
    #[serde(default)]
    pub pca_connector_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableCAEnrollmentPolicyResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableClientAuthenticationRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableClientAuthenticationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDirectoryDataAccessRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDirectoryDataAccessResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableLDAPSRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableLDAPSResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableRadiusRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RadiusSettings")]
    #[serde(default)]
    pub radius_settings: RadiusSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableRadiusResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSsoRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSsoResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectoryLimitsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectoryLimitsResult {
    #[serde(rename = "DirectoryLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_limits: Option<DirectoryLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectoryLimits {
    #[serde(rename = "CloudOnlyDirectoriesCurrentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_current_count: Option<i32>,
    #[serde(rename = "CloudOnlyDirectoriesLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit: Option<i32>,
    #[serde(rename = "CloudOnlyDirectoriesLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit_reached: Option<bool>,
    #[serde(rename = "CloudOnlyMicrosoftADCurrentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_a_d_current_count: Option<i32>,
    #[serde(rename = "CloudOnlyMicrosoftADLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_a_d_limit: Option<i32>,
    #[serde(rename = "CloudOnlyMicrosoftADLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_a_d_limit_reached: Option<bool>,
    #[serde(rename = "ConnectedDirectoriesCurrentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_current_count: Option<i32>,
    #[serde(rename = "ConnectedDirectoriesLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit: Option<i32>,
    #[serde(rename = "ConnectedDirectoriesLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit_reached: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSnapshotLimitsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSnapshotLimitsResult {
    #[serde(rename = "SnapshotLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_limits: Option<SnapshotLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotLimits {
    #[serde(rename = "ManualSnapshotsCurrentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_current_count: Option<i32>,
    #[serde(rename = "ManualSnapshotsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit: Option<i32>,
    #[serde(rename = "ManualSnapshotsLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit_reached: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListADAssessmentsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListADAssessmentsResult {
    #[serde(rename = "Assessments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessments: Option<Vec<AssessmentSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentSummary {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "CustomerDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_dns_ips: Option<Vec<String>>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
    #[serde(rename = "ReportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
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
pub struct ListCertificatesRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesResult {
    #[serde(rename = "CertificatesInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_info: Option<Vec<CertificateInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateInfo {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "CommonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[serde(rename = "ExpiryDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIpRoutesRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIpRoutesResult {
    #[serde(rename = "IpRoutesInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_routes_info: Option<Vec<IpRouteInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpRouteInfo {
    #[serde(rename = "AddedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date_time: Option<f64>,
    #[serde(rename = "CidrIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    #[serde(rename = "CidrIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "IpRouteStatusMsg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_msg: Option<String>,
    #[serde(rename = "IpRouteStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogSubscriptionsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogSubscriptionsResult {
    #[serde(rename = "LogSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscriptions: Option<Vec<LogSubscription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogSubscription {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "SubscriptionCreatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_created_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemaExtensionsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemaExtensionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaExtensionsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extensions_info: Option<Vec<SchemaExtensionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaExtensionInfo {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "SchemaExtensionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
    #[serde(rename = "SchemaExtensionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status: Option<String>,
    #[serde(rename = "SchemaExtensionStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status_reason: Option<String>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResult {
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
pub struct RegisterCertificateRequest {
    #[serde(rename = "CertificateData")]
    #[serde(default)]
    pub certificate_data: String,
    #[serde(rename = "ClientCertAuthSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_auth_settings: Option<ClientCertAuthSettings>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCertificateResult {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterEventTopicRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterEventTopicResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectSharedDirectoryRequest {
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    pub shared_directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectSharedDirectoryResult {
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveIpRoutesRequest {
    #[serde(rename = "CidrIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ips: Option<Vec<String>>,
    #[serde(rename = "CidrIpv6s")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6s: Option<Vec<String>>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveIpRoutesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetUserPasswordRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "NewPassword")]
    #[serde(default)]
    pub new_password: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetUserPasswordResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreFromSnapshotRequest {
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreFromSnapshotResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareDirectoryRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "ShareMethod")]
    #[serde(default)]
    pub share_method: String,
    #[serde(rename = "ShareNotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    #[serde(rename = "ShareTarget")]
    #[serde(default)]
    pub share_target: ShareTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareTarget {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareDirectoryResult {
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartADAssessmentRequest {
    #[serde(rename = "AssessmentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_configuration: Option<AssessmentConfiguration>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentConfiguration {
    #[serde(rename = "CustomerDnsIps")]
    #[serde(default)]
    pub customer_dns_ips: Vec<String>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    pub dns_name: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "VpcSettings")]
    #[serde(default)]
    pub vpc_settings: DirectoryVpcSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartADAssessmentResult {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaExtensionRequest {
    #[serde(rename = "CreateSnapshotBeforeSchemaExtension")]
    #[serde(default)]
    pub create_snapshot_before_schema_extension: bool,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "LdifContent")]
    #[serde(default)]
    pub ldif_content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaExtensionResult {
    #[serde(rename = "SchemaExtensionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnshareDirectoryRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "UnshareTarget")]
    #[serde(default)]
    pub unshare_target: UnshareTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnshareTarget {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnshareDirectoryResult {
    #[serde(rename = "SharedDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConditionalForwarderRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "DnsIpAddrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addrs: Option<Vec<String>>,
    #[serde(rename = "RemoteDomainName")]
    #[serde(default)]
    pub remote_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConditionalForwarderResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectorySetupRequest {
    #[serde(rename = "CreateSnapshotBeforeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_snapshot_before_update: Option<bool>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "DirectorySizeUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_size_update_settings: Option<DirectorySizeUpdateSettings>,
    #[serde(rename = "NetworkUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_update_settings: Option<NetworkUpdateSettings>,
    #[serde(rename = "OSUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_s_update_settings: Option<OSUpdateSettings>,
    #[serde(rename = "UpdateType")]
    #[serde(default)]
    pub update_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectorySizeUpdateSettings {
    #[serde(rename = "DirectorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkUpdateSettings {
    #[serde(rename = "CustomerDnsIpsV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_dns_ips_v6: Option<Vec<String>>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectorySetupResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHybridADRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "HybridAdministratorAccountUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_administrator_account_update: Option<HybridAdministratorAccountUpdate>,
    #[serde(rename = "SelfManagedInstancesSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_instances_settings: Option<HybridCustomerInstancesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridAdministratorAccountUpdate {
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HybridCustomerInstancesSettings {
    #[serde(rename = "CustomerDnsIps")]
    #[serde(default)]
    pub customer_dns_ips: Vec<String>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHybridADResult {
    #[serde(rename = "AssessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNumberOfDomainControllersRequest {
    #[serde(rename = "DesiredNumber")]
    #[serde(default)]
    pub desired_number: i32,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNumberOfDomainControllersResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRadiusRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "RadiusSettings")]
    #[serde(default)]
    pub radius_settings: RadiusSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRadiusResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSettingsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: Vec<Setting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Setting {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSettingsResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustRequest {
    #[serde(rename = "SelectiveAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    #[serde(rename = "TrustId")]
    #[serde(default)]
    pub trust_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustResult {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "TrustId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyTrustRequest {
    #[serde(rename = "TrustId")]
    #[serde(default)]
    pub trust_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyTrustResult {
    #[serde(rename = "TrustId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}
