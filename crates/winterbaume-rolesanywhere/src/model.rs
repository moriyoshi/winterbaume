//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rolesanywhere

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProfileRequest {
    #[serde(rename = "acceptRoleSessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_role_session_name: Option<bool>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "managedPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requireInstanceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_instance_properties: Option<bool>,
    #[serde(rename = "roleArns")]
    #[serde(default)]
    pub role_arns: Vec<String>,
    #[serde(rename = "sessionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustAnchorRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "notificationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<Vec<NotificationSetting>>,
    #[serde(default)]
    pub source: Source,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "sourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data: Option<SourceData>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceData {
    #[serde(rename = "acmPcaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_pca_arn: Option<String>,
    #[serde(rename = "x509CertificateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrlDetailResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl: Option<CrlDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrlDetail {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "crlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_arn: Option<String>,
    #[serde(rename = "crlData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_data: Option<String>,
    #[serde(rename = "crlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "trustAnchorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttributeMappingRequest {
    #[serde(rename = "certificateField")]
    #[serde(default)]
    pub certificate_field: String,
    #[serde(rename = "profileId")]
    #[serde(default)]
    pub profile_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttributeMappingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProfileDetail {
    #[serde(rename = "acceptRoleSessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_role_session_name: Option<bool>,
    #[serde(rename = "attributeMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mappings: Option<Vec<AttributeMapping>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "managedPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    #[serde(rename = "profileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "requireInstanceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_instance_properties: Option<bool>,
    #[serde(rename = "roleArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arns: Option<Vec<String>>,
    #[serde(rename = "sessionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_policy: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeMapping {
    #[serde(rename = "certificateField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_field: Option<String>,
    #[serde(rename = "mappingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_rules: Option<Vec<MappingRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappingRule {
    #[serde(default)]
    pub specifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCrlRequest {
    #[serde(rename = "crlData")]
    #[serde(default)]
    pub crl_data: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "trustAnchorArn")]
    #[serde(default)]
    pub trust_anchor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCrlsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crls: Option<Vec<CrlDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProfilesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<ProfileDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubjectsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<SubjectSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(rename = "subjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_arn: Option<String>,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "x509Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_subject: Option<String>,
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
pub struct ListTrustAnchorsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "trustAnchors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchors: Option<Vec<TrustAnchorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustAnchorDetail {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<Vec<NotificationSettingDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "trustAnchorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_arn: Option<String>,
    #[serde(rename = "trustAnchorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSettingDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "configuredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProfileDetailResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttributeMappingRequest {
    #[serde(rename = "certificateField")]
    #[serde(default)]
    pub certificate_field: String,
    #[serde(rename = "mappingRules")]
    #[serde(default)]
    pub mapping_rules: Vec<MappingRule>,
    #[serde(rename = "profileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttributeMappingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutNotificationSettingsRequest {
    #[serde(rename = "notificationSettings")]
    #[serde(default)]
    pub notification_settings: Vec<NotificationSetting>,
    #[serde(rename = "trustAnchorId")]
    #[serde(default)]
    pub trust_anchor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutNotificationSettingsResponse {
    #[serde(rename = "trustAnchor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor: Option<TrustAnchorDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetNotificationSettingsRequest {
    #[serde(rename = "notificationSettingKeys")]
    #[serde(default)]
    pub notification_setting_keys: Vec<NotificationSettingKey>,
    #[serde(rename = "trustAnchorId")]
    #[serde(default)]
    pub trust_anchor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSettingKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default)]
    pub event: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetNotificationSettingsResponse {
    #[serde(rename = "trustAnchor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor: Option<TrustAnchorDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalarCrlRequest {
    #[serde(rename = "crlId")]
    #[serde(default)]
    pub crl_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalarProfileRequest {
    #[serde(rename = "profileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalarSubjectRequest {
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScalarTrustAnchorRequest {
    #[serde(rename = "trustAnchorId")]
    #[serde(default)]
    pub trust_anchor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectDetailResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectDetail {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<CredentialSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "instanceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_properties: Option<Vec<InstanceProperty>>,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(rename = "subjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_arn: Option<String>,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "x509Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CredentialSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "seenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "x509CertificateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificate_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "seenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
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
pub struct TrustAnchorDetailResponse {
    #[serde(rename = "trustAnchor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor: Option<TrustAnchorDetail>,
}

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
pub struct UpdateCrlRequest {
    #[serde(rename = "crlData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_data: Option<String>,
    #[serde(rename = "crlId")]
    #[serde(default)]
    pub crl_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProfileRequest {
    #[serde(rename = "acceptRoleSessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_role_session_name: Option<bool>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "managedPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profileId")]
    #[serde(default)]
    pub profile_id: String,
    #[serde(rename = "roleArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arns: Option<Vec<String>>,
    #[serde(rename = "sessionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustAnchorRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "trustAnchorId")]
    #[serde(default)]
    pub trust_anchor_id: String,
}
