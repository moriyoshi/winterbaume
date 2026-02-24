//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-guardduty

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAdministratorInvitationRequest {
    #[serde(rename = "administratorId")]
    #[serde(default)]
    pub administrator_id: String,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    pub invitation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAdministratorInvitationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    pub invitation_id: String,
    #[serde(rename = "masterId")]
    #[serde(default)]
    pub master_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveFindingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingIds")]
    #[serde(default)]
    pub finding_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveFindingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDetectorRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<DetectorFeatureConfiguration>>,
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceConfigurations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<KubernetesConfiguration>,
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<MalwareProtectionConfiguration>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3LogsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesConfiguration {
    #[serde(rename = "auditLogs")]
    #[serde(default)]
    pub audit_logs: KubernetesAuditLogsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesAuditLogsConfiguration {
    #[serde(default)]
    pub enable: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionConfiguration {
    #[serde(rename = "scanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings: Option<ScanEc2InstanceWithFindings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanEc2InstanceWithFindings {
    #[serde(rename = "ebsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogsConfiguration {
    #[serde(default)]
    pub enable: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectorFeatureConfiguration {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<DetectorAdditionalConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectorAdditionalConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDetectorResponse {
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(rename = "unprocessedDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_data_sources: Option<UnprocessedDataSourcesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedDataSourcesResult {
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<MalwareProtectionConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionConfigurationResult {
    #[serde(rename = "scanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings: Option<ScanEc2InstanceWithFindingsResult>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanEc2InstanceWithFindingsResult {
    #[serde(rename = "ebsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<EbsVolumesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsVolumesResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    pub finding_criteria: FindingCriteria,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<std::collections::HashMap<String, Condition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,
    #[serde(rename = "greaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<i64>,
    #[serde(rename = "greaterThanOrEqual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i32>,
    #[serde(rename = "lessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<i64>,
    #[serde(rename = "lessThanOrEqual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
    #[serde(rename = "notEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,
    #[serde(rename = "notMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_matches: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIPSetRequest {
    #[serde(default)]
    pub activate: bool,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIPSetResponse {
    #[serde(rename = "ipSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMalwareProtectionPlanRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<MalwareProtectionPlanActions>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "protectedResource")]
    #[serde(default)]
    pub protected_resource: CreateProtectedResource,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionPlanActions {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<MalwareProtectionPlanTaggingAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionPlanTaggingAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProtectedResource {
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<CreateS3BucketResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateS3BucketResource {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "objectPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_prefixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMalwareProtectionPlanResponse {
    #[serde(rename = "malwareProtectionPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection_plan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMembersRequest {
    #[serde(rename = "accountDetails")]
    #[serde(default)]
    pub account_details: Vec<AccountDetail>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountDetail {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub email: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePublishingDestinationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "destinationProperties")]
    #[serde(default)]
    pub destination_properties: DestinationProperties,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    pub destination_type: String,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationProperties {
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePublishingDestinationResponse {
    #[serde(rename = "destinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSampleFindingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSampleFindingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThreatEntitySetRequest {
    #[serde(default)]
    pub activate: bool,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThreatEntitySetResponse {
    #[serde(rename = "threatEntitySetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_entity_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThreatIntelSetRequest {
    #[serde(default)]
    pub activate: bool,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThreatIntelSetResponse {
    #[serde(rename = "threatIntelSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustedEntitySetRequest {
    #[serde(default)]
    pub activate: bool,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustedEntitySetResponse {
    #[serde(rename = "trustedEntitySetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_entity_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDetectorRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDetectorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFilterRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "FilterName")]
    #[serde(default)]
    pub filter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIPSetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "IpSetId")]
    #[serde(default)]
    pub ip_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIPSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMalwareProtectionPlanRequest {
    #[serde(rename = "MalwareProtectionPlanId")]
    #[serde(default)]
    pub malware_protection_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePublishingDestinationRequest {
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    pub destination_id: String,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePublishingDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThreatEntitySetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "ThreatEntitySetId")]
    #[serde(default)]
    pub threat_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThreatEntitySetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThreatIntelSetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "ThreatIntelSetId")]
    #[serde(default)]
    pub threat_intel_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThreatIntelSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustedEntitySetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "TrustedEntitySetId")]
    #[serde(default)]
    pub trusted_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustedEntitySetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMalwareScansRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriteria {
    #[serde(rename = "filterCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criterion: Option<Vec<FilterCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriterion {
    #[serde(rename = "criterionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion_key: Option<String>,
    #[serde(rename = "filterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition: Option<FilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCondition {
    #[serde(rename = "equalsValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals_value: Option<String>,
    #[serde(rename = "greaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<i64>,
    #[serde(rename = "lessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortCriteria {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMalwareScansResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scans: Option<Vec<Scan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scan {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "adminDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_detector_id: Option<String>,
    #[serde(rename = "attachedVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_volumes: Option<Vec<VolumeDetail>>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "fileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
    #[serde(rename = "resourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
    #[serde(rename = "scanEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_end_time: Option<f64>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "scanResultDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_details: Option<ScanResultDetails>,
    #[serde(rename = "scanStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_start_time: Option<f64>,
    #[serde(rename = "scanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "totalBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
    #[serde(rename = "triggerDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_details: Option<TriggerDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeDetail {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "snapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "volumeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    #[serde(rename = "volumeSizeInGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_g_b: Option<i32>,
    #[serde(rename = "volumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanResultDetails {
    #[serde(rename = "scanResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "guardDutyFindingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_duty_finding_id: Option<String>,
    #[serde(rename = "triggerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct DescribeOrganizationConfigurationResponse {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
    #[serde(rename = "autoEnableOrganizationMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_organization_members: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<OrganizationDataSourceConfigurationsResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<OrganizationFeatureConfigurationResult>>,
    #[serde(rename = "memberAccountLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_limit_reached: Option<bool>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDataSourceConfigurationsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<OrganizationKubernetesConfigurationResult>,
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<OrganizationMalwareProtectionConfigurationResult>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<OrganizationS3LogsConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationKubernetesConfigurationResult {
    #[serde(rename = "auditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<OrganizationKubernetesAuditLogsConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationKubernetesAuditLogsConfigurationResult {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationMalwareProtectionConfigurationResult {
    #[serde(rename = "scanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings: Option<OrganizationScanEc2InstanceWithFindingsResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationScanEc2InstanceWithFindingsResult {
    #[serde(rename = "ebsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<OrganizationEbsVolumesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationEbsVolumesResult {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationS3LogsConfigurationResult {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationFeatureConfigurationResult {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<OrganizationAdditionalConfigurationResult>>,
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationAdditionalConfigurationResult {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePublishingDestinationRequest {
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    pub destination_id: String,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePublishingDestinationResponse {
    #[serde(rename = "destinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_properties: Option<DestinationProperties>,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "publishingFailureStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_failure_start_timestamp: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator: Option<Administrator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Administrator {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoverageStatisticsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CoverageFilterCriteria>,
    #[serde(rename = "statisticsType")]
    #[serde(default)]
    pub statistics_type: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageFilterCriteria {
    #[serde(rename = "filterCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criterion: Option<Vec<CoverageFilterCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageFilterCriterion {
    #[serde(rename = "criterionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion_key: Option<String>,
    #[serde(rename = "filterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition: Option<CoverageFilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageFilterCondition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,
    #[serde(rename = "notEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoverageStatisticsResponse {
    #[serde(rename = "coverageStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_statistics: Option<CoverageStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageStatistics {
    #[serde(rename = "countByCoverageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_coverage_status: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "countByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_resource_type: Option<std::collections::HashMap<String, i64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDetectorRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDetectorResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurationsResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<DetectorFeatureConfigurationResult>>,
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceConfigurationsResult {
    #[serde(rename = "cloudTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail: Option<CloudTrailConfigurationResult>,
    #[serde(rename = "dnsLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_logs: Option<DNSLogsConfigurationResult>,
    #[serde(rename = "flowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs: Option<FlowLogsConfigurationResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<KubernetesConfigurationResult>,
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<MalwareProtectionConfigurationResult>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3LogsConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudTrailConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DNSLogsConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowLogsConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesConfigurationResult {
    #[serde(rename = "auditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<KubernetesAuditLogsConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesAuditLogsConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogsConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectorFeatureConfigurationResult {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<DetectorAdditionalConfigurationResult>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectorAdditionalConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFilterRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "FilterName")]
    #[serde(default)]
    pub filter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingIds")]
    #[serde(default)]
    pub finding_ids: Vec<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Finding {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedAttackSequenceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_attack_sequence_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "accessKeyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_details: Option<AccessKeyDetails>,
    #[serde(rename = "containerDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_details: Option<Container>,
    #[serde(rename = "ebsSnapshotDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot_details: Option<EbsSnapshotDetails>,
    #[serde(rename = "ebsVolumeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volume_details: Option<EbsVolumeDetails>,
    #[serde(rename = "ec2ImageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_image_details: Option<Ec2ImageDetails>,
    #[serde(rename = "ecsClusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_details: Option<EcsClusterDetails>,
    #[serde(rename = "eksClusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_cluster_details: Option<EksClusterDetails>,
    #[serde(rename = "instanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    #[serde(rename = "kubernetesDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_details: Option<KubernetesDetails>,
    #[serde(rename = "lambdaDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_details: Option<LambdaDetails>,
    #[serde(rename = "rdsDbInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_instance_details: Option<RdsDbInstanceDetails>,
    #[serde(rename = "rdsDbUserDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_user_details: Option<RdsDbUserDetails>,
    #[serde(rename = "rdsLimitlessDbDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_limitless_db_details: Option<RdsLimitlessDbDetails>,
    #[serde(rename = "recoveryPointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_details: Option<RecoveryPointDetails>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "s3BucketDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_details: Option<Vec<S3BucketDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessKeyDetails {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Container {
    #[serde(rename = "containerRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_runtime: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imagePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,
    #[serde(rename = "volumeMounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<VolumeMount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityContext {
    #[serde(rename = "allowPrivilegeEscalation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeMount {
    #[serde(rename = "mountPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsSnapshotDetails {
    #[serde(rename = "snapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsVolumeDetails {
    #[serde(rename = "scannedVolumeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_volume_details: Option<Vec<VolumeDetail>>,
    #[serde(rename = "skippedVolumeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_volume_details: Option<Vec<VolumeDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2ImageDetails {
    #[serde(rename = "imageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsClusterDetails {
    #[serde(rename = "activeServicesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "registeredContainerInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i32>,
    #[serde(rename = "runningTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "taskDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_details: Option<EcsTaskDetails>,
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
pub struct EcsTaskDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(rename = "definitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(rename = "startedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Volume {
    #[serde(rename = "hostPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPath>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostPath {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksClusterDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceDetails {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "iamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,
    #[serde(rename = "imageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_description: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "launchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<String>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "outpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "productCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<ProductCode>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamInstanceProfile {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "ipv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "privateDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    #[serde(rename = "privateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressDetails>>,
    #[serde(rename = "publicDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    #[serde(rename = "publicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
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
pub struct PrivateIpAddressDetails {
    #[serde(rename = "privateDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    #[serde(rename = "privateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroup {
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductCode {
    #[serde(rename = "productCodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "productCodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesDetails {
    #[serde(rename = "kubernetesUserDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_user_details: Option<KubernetesUserDetails>,
    #[serde(rename = "kubernetesWorkloadDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_workload_details: Option<KubernetesWorkloadDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesUserDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "impersonatedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impersonated_user: Option<ImpersonatedUser>,
    #[serde(rename = "sessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImpersonatedUser {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesWorkloadDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(rename = "hostIPC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_i_p_c: Option<bool>,
    #[serde(rename = "hostNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    #[serde(rename = "hostPID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_p_i_d: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "serviceAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "functionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<f64>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbInstanceDetails {
    #[serde(rename = "dbClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    #[serde(rename = "dbInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_arn: Option<String>,
    #[serde(rename = "dbInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    #[serde(rename = "dbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbUserDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(rename = "authMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsLimitlessDbDetails {
    #[serde(rename = "dbClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    #[serde(rename = "dbShardGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_shard_group_arn: Option<String>,
    #[serde(rename = "dbShardGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_shard_group_identifier: Option<String>,
    #[serde(rename = "dbShardGroupResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_shard_group_resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointDetails {
    #[serde(rename = "backupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "recoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "defaultServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_server_side_encryption: Option<DefaultServerSideEncryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "publicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<PublicAccess>,
    #[serde(rename = "s3ObjectDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_details: Option<Vec<S3ObjectDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultServerSideEncryption {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "kmsMasterKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Owner {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicAccess {
    #[serde(rename = "effectivePermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_permission: Option<String>,
    #[serde(rename = "permissionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_configuration: Option<PermissionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionConfiguration {
    #[serde(rename = "accountLevelPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_level_permissions: Option<AccountLevelPermissions>,
    #[serde(rename = "bucketLevelPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_level_permissions: Option<BucketLevelPermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLevelPermissions {
    #[serde(rename = "blockPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockPublicAccess {
    #[serde(rename = "blockPublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "blockPublicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "ignorePublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "restrictPublicBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketLevelPermissions {
    #[serde(rename = "accessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<AccessControlList>,
    #[serde(rename = "blockPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
    #[serde(rename = "bucketPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy: Option<BucketPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlList {
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketPolicy {
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ObjectDetail {
    #[serde(rename = "eTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "objectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_arn: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<ServiceAdditionalInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection: Option<Detection>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(rename = "ebsVolumeScanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volume_scan_details: Option<EbsVolumeScanDetails>,
    #[serde(rename = "eventFirstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_first_seen: Option<String>,
    #[serde(rename = "eventLastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_seen: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
    #[serde(rename = "featureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "malwareScanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scan_details: Option<MalwareScanDetails>,
    #[serde(rename = "resourceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    #[serde(rename = "runtimeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_details: Option<RuntimeDetails>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "userFeedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "awsApiCallAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_call_action: Option<AwsApiCallAction>,
    #[serde(rename = "dnsRequestAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_request_action: Option<DnsRequestAction>,
    #[serde(rename = "kubernetesApiCallAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_api_call_action: Option<KubernetesApiCallAction>,
    #[serde(rename = "kubernetesPermissionCheckedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_permission_checked_details: Option<KubernetesPermissionCheckedDetails>,
    #[serde(rename = "kubernetesRoleBindingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_role_binding_details: Option<KubernetesRoleBindingDetails>,
    #[serde(rename = "kubernetesRoleDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_role_details: Option<KubernetesRoleDetails>,
    #[serde(rename = "networkConnectionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connection_action: Option<NetworkConnectionAction>,
    #[serde(rename = "portProbeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_action: Option<PortProbeAction>,
    #[serde(rename = "rdsLoginAttemptAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_login_attempt_action: Option<RdsLoginAttemptAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiCallAction {
    #[serde(rename = "affectedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resources: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "callerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_type: Option<String>,
    #[serde(rename = "domainDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<DomainDetails>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "remoteAccountDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_account_details: Option<RemoteAccountDetails>,
    #[serde(rename = "remoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteAccountDetails {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteIpDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(rename = "geoLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    #[serde(rename = "ipAddressV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    #[serde(rename = "ipAddressV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v6: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct City {
    #[serde(rename = "cityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Country {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "countryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Organization {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    #[serde(rename = "asnOrg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsRequestAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "domainWithSuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_with_suffix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "vpcOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesApiCallAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "remoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    #[serde(rename = "requestUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "sourceIPs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesPermissionCheckedDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesRoleBindingDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleRefKind")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref_kind: Option<String>,
    #[serde(rename = "roleRefName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesRoleDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConnectionAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "connectionDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_direction: Option<String>,
    #[serde(rename = "localIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<LocalIpDetails>,
    #[serde(rename = "localNetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    #[serde(rename = "localPortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "remoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    #[serde(rename = "remotePortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port_details: Option<RemotePortDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalIpDetails {
    #[serde(rename = "ipAddressV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    #[serde(rename = "ipAddressV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v6: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalPortDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "portName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemotePortDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "portName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortProbeAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "portProbeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_details: Option<Vec<PortProbeDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortProbeDetail {
    #[serde(rename = "localIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<LocalIpDetails>,
    #[serde(rename = "localPortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    #[serde(rename = "remoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsLoginAttemptAction {
    #[serde(rename = "LoginAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_attributes: Option<Vec<LoginAttribute>>,
    #[serde(rename = "remoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoginAttribute {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(rename = "failedLoginAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_login_attempts: Option<i32>,
    #[serde(rename = "successfulLoginAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_login_attempts: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceAdditionalInfo {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Detection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly: Option<Anomaly>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Anomaly {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, Vec<AnomalyObject>>>,
    >,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unusual: Option<AnomalyUnusual>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyObject {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Observations>,
    #[serde(rename = "profileSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_subtype: Option<String>,
    #[serde(rename = "profileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Observations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyUnusual {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, AnomalyObject>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sequence {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<Actor>>,
    #[serde(rename = "additionalSequenceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_sequence_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<NetworkEndpoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceV2>>,
    #[serde(rename = "sequenceIndicators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_indicators: Option<Vec<Indicator>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<Signal>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Actor {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ActorProcess>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActorProcess {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "mfaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
    #[serde(rename = "credentialUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_uid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkEndpoint {
    #[serde(rename = "autonomousSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_system: Option<AutonomousSystem>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<NetworkConnection>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<NetworkGeoLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutonomousSystem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConnection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkGeoLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "lat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "lon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceV2 {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "cloudPartition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_partition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResourceData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceData {
    #[serde(rename = "accessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<AccessKey>,
    #[serde(rename = "autoscalingAutoScalingGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaling_auto_scaling_group: Option<AutoscalingAutoScalingGroup>,
    #[serde(rename = "cloudformationStack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_stack: Option<CloudformationStack>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerFindingResource>,
    #[serde(rename = "ec2Image")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_image: Option<Ec2Image>,
    #[serde(rename = "ec2Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance: Option<Ec2Instance>,
    #[serde(rename = "ec2LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_launch_template: Option<Ec2LaunchTemplate>,
    #[serde(rename = "ec2NetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_network_interface: Option<Ec2NetworkInterface>,
    #[serde(rename = "ec2Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_vpc: Option<Ec2Vpc>,
    #[serde(rename = "ecsCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster: Option<EcsCluster>,
    #[serde(rename = "ecsTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_task: Option<EcsTask>,
    #[serde(rename = "eksCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_cluster: Option<EksCluster>,
    #[serde(rename = "iamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfileV2>,
    #[serde(rename = "kubernetesWorkload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_workload: Option<KubernetesWorkload>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "s3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessKey {
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoscalingAutoScalingGroup {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudformationStack {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerFindingResource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Image {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Instance {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ec2NetworkInterfaceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_network_interface_uids: Option<Vec<String>>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,
    #[serde(rename = "imageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_description: Option<String>,
    #[serde(rename = "instanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "outpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "productCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<ProductCode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2LaunchTemplate {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2NetworkInterface {
    #[serde(rename = "ipv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
    #[serde(rename = "privateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressDetails>>,
    #[serde(rename = "publicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
    #[serde(rename = "subNetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_net_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Vpc {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsCluster {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsTask {
    #[serde(rename = "containerUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_uids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "launchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "taskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksCluster {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamInstanceProfileV2 {
    #[serde(rename = "ec2InstanceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesWorkload {
    #[serde(rename = "containerUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_uids: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_resources_types: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Bucket {
    #[serde(rename = "accountPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_public_access: Option<PublicAccessConfiguration>,
    #[serde(rename = "bucketPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_public_access: Option<PublicAccessConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "effectivePermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_permission: Option<String>,
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "publicReadAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_read_access: Option<String>,
    #[serde(rename = "publicWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_write_access: Option<String>,
    #[serde(rename = "s3ObjectUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicAccessConfiguration {
    #[serde(rename = "publicAclAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_acl_access: Option<String>,
    #[serde(rename = "publicAclIgnoreBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_acl_ignore_behavior: Option<String>,
    #[serde(rename = "publicBucketRestrictBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_bucket_restrict_behavior: Option<String>,
    #[serde(rename = "publicPolicyAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_policy_access: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Object {
    #[serde(rename = "eTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Indicator {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Signal {
    #[serde(rename = "actorIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "endpointIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(rename = "firstSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<f64>,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[serde(rename = "signalIndicators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_indicators: Option<Vec<Indicator>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsVolumeScanDetails {
    #[serde(rename = "scanCompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_completed_at: Option<f64>,
    #[serde(rename = "scanDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_detections: Option<ScanDetections>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "scanStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_started_at: Option<f64>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(rename = "triggerFindingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_finding_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanDetections {
    #[serde(rename = "highestSeverityThreatDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_severity_threat_details: Option<HighestSeverityThreatDetails>,
    #[serde(rename = "scannedItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_item_count: Option<ScannedItemCount>,
    #[serde(rename = "threatDetectedByName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detected_by_name: Option<ThreatDetectedByName>,
    #[serde(rename = "threatsDetectedItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threats_detected_item_count: Option<ThreatsDetectedItemCount>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HighestSeverityThreatDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "threatName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScannedItemCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<i32>,
    #[serde(rename = "totalGb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_gb: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThreatDetectedByName {
    #[serde(rename = "itemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortened: Option<bool>,
    #[serde(rename = "threatNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_names: Option<Vec<ScanThreatName>>,
    #[serde(rename = "uniqueThreatNameCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_threat_name_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanThreatName {
    #[serde(rename = "filePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<ScanFilePath>>,
    #[serde(rename = "itemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanFilePath {
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "volumeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThreatsDetectedItemCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(rename = "threatIntelligenceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intelligence_details: Option<Vec<ThreatIntelligenceDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThreatIntelligenceDetail {
    #[serde(rename = "threatFileSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_file_sha256: Option<String>,
    #[serde(rename = "threatListName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_list_name: Option<String>,
    #[serde(rename = "threatNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareScanDetails {
    #[serde(rename = "scanCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_category: Option<String>,
    #[serde(rename = "scanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration: Option<MalwareProtectionFindingsScanConfiguration>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threats: Option<Vec<Threat>>,
    #[serde(rename = "uniqueThreatCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_threat_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionFindingsScanConfiguration {
    #[serde(rename = "incrementalScanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_scan_details: Option<IncrementalScanDetails>,
    #[serde(rename = "triggerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncrementalScanDetails {
    #[serde(rename = "baselineResourceArn")]
    #[serde(default)]
    pub baseline_resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Threat {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "itemDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_details: Option<Vec<ItemDetails>>,
    #[serde(rename = "itemPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_paths: Option<Vec<ItemPath>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemDetails {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "itemPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalInfo {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemPath {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "nestedItemPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_item_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<RuntimeContext>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeContext {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "commandLineExample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line_example: Option<String>,
    #[serde(rename = "fileSystemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(rename = "ianaProtocolNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iana_protocol_number: Option<i32>,
    #[serde(rename = "ldPreloadValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ld_preload_value: Option<String>,
    #[serde(rename = "libraryPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_path: Option<String>,
    #[serde(rename = "memoryRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_regions: Option<Vec<String>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "modifyingProcess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifying_process: Option<ProcessDetails>,
    #[serde(rename = "moduleFilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_file_path: Option<String>,
    #[serde(rename = "moduleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(rename = "moduleSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_sha256: Option<String>,
    #[serde(rename = "mountSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_source: Option<String>,
    #[serde(rename = "mountTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target: Option<String>,
    #[serde(rename = "releaseAgentPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_agent_path: Option<String>,
    #[serde(rename = "runcBinaryPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runc_binary_path: Option<String>,
    #[serde(rename = "scriptPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_path: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "shellHistoryFilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_history_file_path: Option<String>,
    #[serde(rename = "socketPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket_path: Option<String>,
    #[serde(rename = "targetProcess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_process: Option<ProcessDetails>,
    #[serde(rename = "threatFilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_file_path: Option<String>,
    #[serde(rename = "toolCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_category: Option<String>,
    #[serde(rename = "toolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub euid: Option<i32>,
    #[serde(rename = "executablePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_path: Option<String>,
    #[serde(rename = "executableSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_sha256: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Vec<LineageObject>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespacePid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_pid: Option<i32>,
    #[serde(rename = "parentUuid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineageObject {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub euid: Option<i32>,
    #[serde(rename = "executablePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespacePid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_pid: Option<i32>,
    #[serde(rename = "parentUuid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsStatisticsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(rename = "findingStatisticTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_statistic_types: Option<Vec<String>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsStatisticsResponse {
    #[serde(rename = "findingStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_statistics: Option<FindingStatistics>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingStatistics {
    #[serde(rename = "countBySeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_severity: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "groupedByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by_account: Option<Vec<AccountStatistics>>,
    #[serde(rename = "groupedByDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by_date: Option<Vec<DateStatistics>>,
    #[serde(rename = "groupedByFindingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by_finding_type: Option<Vec<FindingTypeStatistics>>,
    #[serde(rename = "groupedByResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by_resource: Option<Vec<ResourceStatistics>>,
    #[serde(rename = "groupedBySeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by_severity: Option<Vec<SeverityStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountStatistics {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "lastGeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<f64>,
    #[serde(rename = "totalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateStatistics {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "lastGeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[serde(rename = "totalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingTypeStatistics {
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "lastGeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<f64>,
    #[serde(rename = "totalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceStatistics {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "lastGeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "totalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityStatistics {
    #[serde(rename = "lastGeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[serde(rename = "totalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIPSetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "IpSetId")]
    #[serde(default)]
    pub ip_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIPSetResponse {
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountResponse {
    #[serde(rename = "invitationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareProtectionPlanRequest {
    #[serde(rename = "MalwareProtectionPlanId")]
    #[serde(default)]
    pub malware_protection_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareProtectionPlanResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<MalwareProtectionPlanActions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "protectedResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource: Option<CreateProtectedResource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<MalwareProtectionPlanStatusReason>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionPlanStatusReason {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareScanRequest {
    #[serde(rename = "ScanId")]
    #[serde(default)]
    pub scan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareScanResponse {
    #[serde(rename = "adminDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_detector_id: Option<String>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(rename = "failedResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_count: Option<i32>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "scanCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_category: Option<String>,
    #[serde(rename = "scanCompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_completed_at: Option<f64>,
    #[serde(rename = "scanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration: Option<ScanConfiguration>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "scanResultDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_details: Option<GetMalwareScanResultDetails>,
    #[serde(rename = "scanStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_started_at: Option<f64>,
    #[serde(rename = "scanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    #[serde(rename = "scanStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status_reason: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "scannedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_resources: Option<Vec<ScannedResource>>,
    #[serde(rename = "scannedResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_resources_count: Option<i32>,
    #[serde(rename = "skippedResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_resources_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanConfiguration {
    #[serde(rename = "incrementalScanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_scan_details: Option<IncrementalScanDetails>,
    #[serde(rename = "recoveryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point: Option<ScanConfigurationRecoveryPoint>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "triggerDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_details: Option<TriggerDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanConfigurationRecoveryPoint {
    #[serde(rename = "backupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareScanResultDetails {
    #[serde(rename = "failedFileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_file_count: Option<i64>,
    #[serde(rename = "scanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_status: Option<String>,
    #[serde(rename = "skippedFileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_file_count: Option<i64>,
    #[serde(rename = "threatFoundFileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_found_file_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threats: Option<Vec<ScanResultThreat>>,
    #[serde(rename = "totalBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
    #[serde(rename = "totalFileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_file_count: Option<i64>,
    #[serde(rename = "uniqueThreatCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_threat_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanResultThreat {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "itemDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_details: Option<Vec<ItemDetails>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScannedResource {
    #[serde(rename = "resourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ScannedResourceDetails>,
    #[serde(rename = "scanStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status_reason: Option<String>,
    #[serde(rename = "scannedResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_resource_arn: Option<String>,
    #[serde(rename = "scannedResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_resource_status: Option<String>,
    #[serde(rename = "scannedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScannedResourceDetails {
    #[serde(rename = "ebsSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot: Option<EbsSnapshot>,
    #[serde(rename = "ebsVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volume: Option<VolumeDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsSnapshot {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareScanSettingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMalwareScanSettingsResponse {
    #[serde(rename = "ebsSnapshotPreservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot_preservation: Option<String>,
    #[serde(rename = "scanResourceCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_resource_criteria: Option<ScanResourceCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanResourceCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<std::collections::HashMap<String, ScanCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<std::collections::HashMap<String, ScanCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanCondition {
    #[serde(rename = "mapEquals")]
    #[serde(default)]
    pub map_equals: Vec<ScanConditionPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanConditionPair {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Master>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Master {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberDetectorsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberDetectorsResponse {
    #[serde(rename = "members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_data_source_configurations: Option<Vec<MemberDataSourceConfiguration>>,
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberDataSourceConfiguration {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurationsResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<MemberFeaturesConfigurationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFeaturesConfigurationResult {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<MemberAdditionalConfigurationResult>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberAdditionalConfigurationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMembersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "administratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_id: Option<String>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "masterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationStatisticsResponse {
    #[serde(rename = "organizationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_details: Option<OrganizationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDetails {
    #[serde(rename = "organizationStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_statistics: Option<OrganizationStatistics>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationStatistics {
    #[serde(rename = "activeAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_accounts_count: Option<i32>,
    #[serde(rename = "countByFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_feature: Option<Vec<OrganizationFeatureStatistics>>,
    #[serde(rename = "enabledAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_accounts_count: Option<i32>,
    #[serde(rename = "memberAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_accounts_count: Option<i32>,
    #[serde(rename = "totalAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_accounts_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationFeatureStatistics {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<OrganizationFeatureStatisticsAdditionalConfiguration>>,
    #[serde(rename = "enabledAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_accounts_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationFeatureStatisticsAdditionalConfiguration {
    #[serde(rename = "enabledAccountsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_accounts_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRemainingFreeTrialDaysRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRemainingFreeTrialDaysResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountFreeTrialInfo>>,
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountFreeTrialInfo {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourcesFreeTrial>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<FreeTrialFeatureConfigurationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourcesFreeTrial {
    #[serde(rename = "cloudTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail: Option<DataSourceFreeTrial>,
    #[serde(rename = "dnsLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_logs: Option<DataSourceFreeTrial>,
    #[serde(rename = "flowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs: Option<DataSourceFreeTrial>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<KubernetesDataSourceFreeTrial>,
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<MalwareProtectionDataSourceFreeTrial>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<DataSourceFreeTrial>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceFreeTrial {
    #[serde(rename = "freeTrialDaysRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_days_remaining: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesDataSourceFreeTrial {
    #[serde(rename = "auditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<DataSourceFreeTrial>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionDataSourceFreeTrial {
    #[serde(rename = "scanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings: Option<DataSourceFreeTrial>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeTrialFeatureConfigurationResult {
    #[serde(rename = "freeTrialDaysRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_days_remaining: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThreatEntitySetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "ThreatEntitySetId")]
    #[serde(default)]
    pub threat_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThreatEntitySetResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThreatIntelSetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "ThreatIntelSetId")]
    #[serde(default)]
    pub threat_intel_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThreatIntelSetResponse {
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustedEntitySetRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "TrustedEntitySetId")]
    #[serde(default)]
    pub trusted_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustedEntitySetResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageStatisticsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
    pub unit: Option<String>,
    #[serde(rename = "usageCriteria")]
    #[serde(default)]
    pub usage_criteria: UsageCriteria,
    #[serde(rename = "usageStatisticsType")]
    #[serde(default)]
    pub usage_statistic_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageCriteria {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageStatisticsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "usageStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_statistics: Option<UsageStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageStatistics {
    #[serde(rename = "sumByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_account: Option<Vec<UsageAccountResult>>,
    #[serde(rename = "sumByDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_data_source: Option<Vec<UsageDataSourceResult>>,
    #[serde(rename = "sumByFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_feature: Option<Vec<UsageFeatureResult>>,
    #[serde(rename = "sumByResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_resource: Option<Vec<UsageResourceResult>>,
    #[serde(rename = "topAccountsByFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_accounts_by_feature: Option<Vec<UsageTopAccountsResult>>,
    #[serde(rename = "topResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_resources: Option<Vec<UsageResourceResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageAccountResult {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Total {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageDataSourceResult {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageFeatureResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageResourceResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageTopAccountsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<UsageTopAccountResult>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageTopAccountResult {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "disableEmailNotification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CoverageFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<CoverageSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageSortCriteria {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CoverageResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageResource {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "coverageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_status: Option<String>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(rename = "resourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<CoverageResourceDetails>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageResourceDetails {
    #[serde(rename = "ec2InstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_details: Option<CoverageEc2InstanceDetails>,
    #[serde(rename = "ecsClusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_details: Option<CoverageEcsClusterDetails>,
    #[serde(rename = "eksClusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_cluster_details: Option<CoverageEksClusterDetails>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageEc2InstanceDetails {
    #[serde(rename = "agentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<AgentDetails>,
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "managementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageEcsClusterDetails {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "containerInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_details: Option<ContainerInstanceDetails>,
    #[serde(rename = "fargateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_details: Option<FargateDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerInstanceDetails {
    #[serde(rename = "compatibleContainerInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_container_instances: Option<i64>,
    #[serde(rename = "coveredContainerInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_container_instances: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargateDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    #[serde(rename = "managementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageEksClusterDetails {
    #[serde(rename = "addonDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_details: Option<AddonDetails>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "compatibleNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_nodes: Option<i64>,
    #[serde(rename = "coveredNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_nodes: Option<i64>,
    #[serde(rename = "managementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonDetails {
    #[serde(rename = "addonStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_status: Option<String>,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDetectorsRequest {
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
pub struct ListDetectorsResponse {
    #[serde(rename = "detectorIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFiltersRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListFiltersResponse {
    #[serde(rename = "filterNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsResponse {
    #[serde(rename = "findingIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIPSetsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListIPSetsResponse {
    #[serde(rename = "ipSetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInvitationsRequest {
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
pub struct ListInvitationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareProtectionPlansRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareProtectionPlansResponse {
    #[serde(rename = "malwareProtectionPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection_plans: Option<Vec<MalwareProtectionPlanSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareProtectionPlanSummary {
    #[serde(rename = "malwareProtectionPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection_plan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareScansRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<ListMalwareScansFilterCriteria>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareScansFilterCriteria {
    #[serde(rename = "filterCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_malware_scans_filter_criterion: Option<Vec<ListMalwareScansFilterCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareScansFilterCriterion {
    #[serde(rename = "filterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition: Option<FilterCondition>,
    #[serde(rename = "criterionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_malware_scans_criterion_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMalwareScansResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scans: Option<Vec<MalwareScan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalwareScan {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "scanCompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_completed_at: Option<f64>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "scanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_status: Option<String>,
    #[serde(rename = "scanStartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_started_at: Option<f64>,
    #[serde(rename = "scanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OnlyAssociated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationAdminAccountsRequest {
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
pub struct ListOrganizationAdminAccountsResponse {
    #[serde(rename = "adminAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_accounts: Option<Vec<AdminAccount>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminAccount {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    #[serde(rename = "adminStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPublishingDestinationsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListPublishingDestinationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "destinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThreatEntitySetsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListThreatEntitySetsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "threatEntitySetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_entity_set_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThreatIntelSetsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListThreatIntelSetsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "threatIntelSetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_set_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrustedEntitySetsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
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
pub struct ListTrustedEntitySetsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "trustedEntitySetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_entity_set_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendObjectMalwareScanRequest {
    #[serde(rename = "s3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3ObjectForSendObjectMalwareScan>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ObjectForSendObjectMalwareScan {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendObjectMalwareScanResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMalwareScanRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "scanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration: Option<StartMalwareScanConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMalwareScanConfiguration {
    #[serde(rename = "incrementalScanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_scan_details: Option<IncrementalScanDetails>,
    #[serde(rename = "recoveryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point: Option<RecoveryPoint>,
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPoint {
    #[serde(rename = "backupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMalwareScanResponse {
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMonitoringMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMonitoringMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMonitoringMembersRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMonitoringMembersResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnarchiveFindingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "findingIds")]
    #[serde(default)]
    pub finding_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnarchiveFindingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDetectorRequest {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<DetectorFeatureConfiguration>>,
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDetectorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFilterRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "FilterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsFeedbackRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(default)]
    pub feedback: String,
    #[serde(rename = "findingIds")]
    #[serde(default)]
    pub finding_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsFeedbackResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIPSetRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IpSetId")]
    #[serde(default)]
    pub ip_set_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIPSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMalwareProtectionPlanRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<MalwareProtectionPlanActions>,
    #[serde(rename = "MalwareProtectionPlanId")]
    #[serde(default)]
    pub malware_protection_plan_id: String,
    #[serde(rename = "protectedResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource: Option<UpdateProtectedResource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProtectedResource {
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<UpdateS3BucketResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateS3BucketResource {
    #[serde(rename = "objectPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_prefixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMalwareScanSettingsRequest {
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "ebsSnapshotPreservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot_preservation: Option<String>,
    #[serde(rename = "scanResourceCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_resource_criteria: Option<ScanResourceCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMalwareScanSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMemberDetectorsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<MemberFeaturesConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFeaturesConfiguration {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<MemberAdditionalConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberAdditionalConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMemberDetectorsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationRequest {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
    #[serde(rename = "autoEnableOrganizationMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_organization_members: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<OrganizationDataSourceConfigurations>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<OrganizationFeatureConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDataSourceConfigurations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<OrganizationKubernetesConfiguration>,
    #[serde(rename = "malwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<OrganizationMalwareProtectionConfiguration>,
    #[serde(rename = "s3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<OrganizationS3LogsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationKubernetesConfiguration {
    #[serde(rename = "auditLogs")]
    #[serde(default)]
    pub audit_logs: OrganizationKubernetesAuditLogsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationKubernetesAuditLogsConfiguration {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    pub auto_enable: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationMalwareProtectionConfiguration {
    #[serde(rename = "scanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings: Option<OrganizationScanEc2InstanceWithFindings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationScanEc2InstanceWithFindings {
    #[serde(rename = "ebsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<OrganizationEbsVolumes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationEbsVolumes {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationS3LogsConfiguration {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    pub auto_enable: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationFeatureConfiguration {
    #[serde(rename = "additionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<Vec<OrganizationAdditionalConfiguration>>,
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationAdditionalConfiguration {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublishingDestinationRequest {
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    pub destination_id: String,
    #[serde(rename = "destinationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_properties: Option<DestinationProperties>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublishingDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThreatEntitySetRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThreatEntitySetId")]
    #[serde(default)]
    pub threat_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThreatEntitySetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThreatIntelSetRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThreatIntelSetId")]
    #[serde(default)]
    pub threat_intel_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThreatIntelSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustedEntitySetRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    #[serde(rename = "DetectorId")]
    #[serde(default)]
    pub detector_id: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrustedEntitySetId")]
    #[serde(default)]
    pub trusted_entity_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustedEntitySetResponse {}
