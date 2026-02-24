//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-workspacesweb

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBrowserSettingsRequest {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    pub browser_settings_arn: String,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBrowserSettingsResponse {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDataProtectionSettingsRequest {
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    pub data_protection_settings_arn: String,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDataProtectionSettingsResponse {
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIpAccessSettingsRequest {
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    pub ip_access_settings_arn: String,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIpAccessSettingsResponse {
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateNetworkSettingsRequest {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    pub network_settings_arn: String,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateNetworkSettingsResponse {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSessionLoggerRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    pub session_logger_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSessionLoggerResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTrustStoreRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTrustStoreResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateUserAccessLoggingSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    pub user_access_logging_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateUserAccessLoggingSettingsResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateUserSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    pub user_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateUserSettingsResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrowserSettingsRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "browserPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_policy: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "webContentFilteringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_content_filtering_policy: Option<WebContentFilteringPolicy>,
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
pub struct WebContentFilteringPolicy {
    #[serde(rename = "allowedUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_urls: Option<Vec<String>>,
    #[serde(rename = "blockedCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<String>>,
    #[serde(rename = "blockedUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrowserSettingsResponse {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataProtectionSettingsRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "inlineRedactionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_redaction_configuration: Option<InlineRedactionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InlineRedactionConfiguration {
    #[serde(rename = "globalConfidenceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_confidence_level: Option<i32>,
    #[serde(rename = "globalEnforcedUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_enforced_urls: Option<Vec<String>>,
    #[serde(rename = "globalExemptUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_exempt_urls: Option<Vec<String>>,
    #[serde(rename = "inlineRedactionPatterns")]
    #[serde(default)]
    pub inline_redaction_patterns: Vec<InlineRedactionPattern>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InlineRedactionPattern {
    #[serde(rename = "builtInPatternId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_pattern_id: Option<String>,
    #[serde(rename = "confidenceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<i32>,
    #[serde(rename = "customPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_pattern: Option<CustomPattern>,
    #[serde(rename = "enforcedUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforced_urls: Option<Vec<String>>,
    #[serde(rename = "exemptUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_urls: Option<Vec<String>>,
    #[serde(rename = "redactionPlaceHolder")]
    #[serde(default)]
    pub redaction_place_holder: RedactionPlaceHolder,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomPattern {
    #[serde(rename = "keywordRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_regex: Option<String>,
    #[serde(rename = "patternDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_description: Option<String>,
    #[serde(rename = "patternName")]
    #[serde(default)]
    pub pattern_name: String,
    #[serde(rename = "patternRegex")]
    #[serde(default)]
    pub pattern_regex: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedactionPlaceHolder {
    #[serde(rename = "redactionPlaceHolderText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_place_holder_text: Option<String>,
    #[serde(rename = "redactionPlaceHolderType")]
    #[serde(default)]
    pub redaction_place_holder_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataProtectionSettingsResponse {
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIdentityProviderRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "identityProviderDetails")]
    #[serde(default)]
    pub identity_provider_details: std::collections::HashMap<String, String>,
    #[serde(rename = "identityProviderName")]
    #[serde(default)]
    pub identity_provider_name: String,
    #[serde(rename = "identityProviderType")]
    #[serde(default)]
    pub identity_provider_type: String,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIdentityProviderResponse {
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIpAccessSettingsRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ipRules")]
    #[serde(default)]
    pub ip_rules: Vec<IpRule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ipRange")]
    #[serde(default)]
    pub ip_range: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIpAccessSettingsResponse {
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkSettingsResponse {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "maxConcurrentSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_sessions: Option<i32>,
    #[serde(rename = "portalCustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_custom_domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSessionLoggerRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "eventFilter")]
    #[serde(default)]
    pub event_filter: EventFilter,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    pub log_configuration: LogConfiguration,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3LogConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogConfiguration {
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "bucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner: Option<String>,
    #[serde(rename = "folderStructure")]
    #[serde(default)]
    pub folder_structure: String,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(rename = "logFileFormat")]
    #[serde(default)]
    pub log_file_format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSessionLoggerResponse {
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustStoreRequest {
    #[serde(rename = "certificateList")]
    #[serde(default)]
    pub certificate_list: Vec<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustStoreResponse {
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserAccessLoggingSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "kinesisStreamArn")]
    #[serde(default)]
    pub kinesis_stream_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserAccessLoggingSettingsResponse {
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserSettingsRequest {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "brandingConfigurationInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_configuration_input: Option<BrandingConfigurationCreateInput>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "cookieSynchronizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_synchronization_configuration: Option<CookieSynchronizationConfiguration>,
    #[serde(rename = "copyAllowed")]
    #[serde(default)]
    pub copy_allowed: String,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "deepLinkAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link_allowed: Option<String>,
    #[serde(rename = "disconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "downloadAllowed")]
    #[serde(default)]
    pub download_allowed: String,
    #[serde(rename = "idleDisconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "pasteAllowed")]
    #[serde(default)]
    pub paste_allowed: String,
    #[serde(rename = "printAllowed")]
    #[serde(default)]
    pub print_allowed: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "toolbarConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar_configuration: Option<ToolbarConfiguration>,
    #[serde(rename = "uploadAllowed")]
    #[serde(default)]
    pub upload_allowed: String,
    #[serde(rename = "webAuthnAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_allowed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandingConfigurationCreateInput {
    #[serde(rename = "colorTheme")]
    #[serde(default)]
    pub color_theme: String,
    #[serde(default)]
    pub favicon: IconImageInput,
    #[serde(rename = "localizedStrings")]
    #[serde(default)]
    pub localized_strings: std::collections::HashMap<String, LocalizedBrandingStrings>,
    #[serde(default)]
    pub logo: IconImageInput,
    #[serde(rename = "termsOfService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpaper: Option<WallpaperImageInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IconImageInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalizedBrandingStrings {
    #[serde(rename = "browserTabTitle")]
    #[serde(default)]
    pub browser_tab_title: String,
    #[serde(rename = "contactButtonText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_button_text: Option<String>,
    #[serde(rename = "contactLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,
    #[serde(rename = "loadingText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_text: Option<String>,
    #[serde(rename = "loginButtonText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_button_text: Option<String>,
    #[serde(rename = "loginDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_description: Option<String>,
    #[serde(rename = "loginTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_title: Option<String>,
    #[serde(rename = "welcomeText")]
    #[serde(default)]
    pub welcome_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WallpaperImageInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CookieSynchronizationConfiguration {
    #[serde(default)]
    pub allowlist: Vec<CookieSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist: Option<Vec<CookieSpecification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CookieSpecification {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolbarConfiguration {
    #[serde(rename = "hiddenToolbarItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_toolbar_items: Option<Vec<String>>,
    #[serde(rename = "maxDisplayResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_display_resolution: Option<String>,
    #[serde(rename = "toolbarType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar_type: Option<String>,
    #[serde(rename = "visualMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserSettingsResponse {
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrowserSettingsRequest {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    pub browser_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrowserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataProtectionSettingsRequest {
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    pub data_protection_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataProtectionSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityProviderRequest {
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    pub identity_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityProviderResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIpAccessSettingsRequest {
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    pub ip_access_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIpAccessSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkSettingsRequest {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    pub network_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortalRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortalResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSessionLoggerRequest {
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    pub session_logger_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSessionLoggerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustStoreRequest {
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserAccessLoggingSettingsRequest {
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    pub user_access_logging_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserAccessLoggingSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserSettingsRequest {
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    pub user_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBrowserSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBrowserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDataProtectionSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDataProtectionSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIpAccessSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIpAccessSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateNetworkSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateNetworkSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSessionLoggerRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSessionLoggerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTrustStoreRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTrustStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateUserAccessLoggingSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateUserAccessLoggingSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateUserSettingsRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateUserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpireSessionRequest {
    #[serde(rename = "portalId")]
    #[serde(default)]
    pub portal_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpireSessionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBrowserSettingsRequest {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    pub browser_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBrowserSettingsResponse {
    #[serde(rename = "browserSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings: Option<BrowserSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrowserSettings {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "browserPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_policy: Option<String>,
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "webContentFilteringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_content_filtering_policy: Option<WebContentFilteringPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataProtectionSettingsRequest {
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    pub data_protection_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataProtectionSettingsResponse {
    #[serde(rename = "dataProtectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings: Option<DataProtectionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProtectionSettings {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "inlineRedactionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_redaction_configuration: Option<InlineRedactionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityProviderRequest {
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    pub identity_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityProviderResponse {
    #[serde(rename = "identityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProvider {
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_arn: Option<String>,
    #[serde(rename = "identityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "identityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "identityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIpAccessSettingsRequest {
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    pub ip_access_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIpAccessSettingsResponse {
    #[serde(rename = "ipAccessSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings: Option<IpAccessSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAccessSettings {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
    #[serde(rename = "ipRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<Vec<IpRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkSettingsRequest {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    pub network_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkSettingsResponse {
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<NetworkSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkSettings {
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
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
pub struct GetPortalRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<Portal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Portal {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
    #[serde(rename = "browserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
    #[serde(rename = "maxConcurrentSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_sessions: Option<i32>,
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalCustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_custom_domain: Option<String>,
    #[serde(rename = "portalEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_endpoint: Option<String>,
    #[serde(rename = "portalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_status: Option<String>,
    #[serde(rename = "rendererType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalServiceProviderMetadataRequest {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalServiceProviderMetadataResponse {
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "serviceProviderSamlMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_saml_metadata: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionLoggerRequest {
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    pub session_logger_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionLoggerResponse {
    #[serde(rename = "sessionLogger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger: Option<SessionLogger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionLogger {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "eventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionRequest {
    #[serde(rename = "portalId")]
    #[serde(default)]
    pub portal_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "clientIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustStoreCertificateRequest {
    #[serde(default)]
    pub thumbprint: String,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustStoreCertificateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "notValidAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_after: Option<f64>,
    #[serde(rename = "notValidBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_before: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustStoreRequest {
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrustStoreResponse {
    #[serde(rename = "trustStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store: Option<TrustStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStore {
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAccessLoggingSettingsRequest {
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    pub user_access_logging_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAccessLoggingSettingsResponse {
    #[serde(rename = "userAccessLoggingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings: Option<UserAccessLoggingSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAccessLoggingSettings {
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "kinesisStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserSettingsRequest {
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    pub user_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserSettingsResponse {
    #[serde(rename = "userSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSettings {
    #[serde(rename = "additionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associatedPortalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_portal_arns: Option<Vec<String>>,
    #[serde(rename = "brandingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_configuration: Option<BrandingConfiguration>,
    #[serde(rename = "cookieSynchronizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_synchronization_configuration: Option<CookieSynchronizationConfiguration>,
    #[serde(rename = "copyAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_allowed: Option<String>,
    #[serde(rename = "customerManagedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key: Option<String>,
    #[serde(rename = "deepLinkAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link_allowed: Option<String>,
    #[serde(rename = "disconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "downloadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_allowed: Option<String>,
    #[serde(rename = "idleDisconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "pasteAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paste_allowed: Option<String>,
    #[serde(rename = "printAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_allowed: Option<String>,
    #[serde(rename = "toolbarConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar_configuration: Option<ToolbarConfiguration>,
    #[serde(rename = "uploadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_allowed: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
    #[serde(rename = "webAuthnAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_allowed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandingConfiguration {
    #[serde(rename = "colorTheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<ImageMetadata>,
    #[serde(rename = "localizedStrings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_strings: Option<std::collections::HashMap<String, LocalizedBrandingStrings>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ImageMetadata>,
    #[serde(rename = "termsOfService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpaper: Option<ImageMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageMetadata {
    #[serde(rename = "fileExtension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(rename = "lastUploadTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_upload_timestamp: Option<f64>,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBrowserSettingsRequest {
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
pub struct ListBrowserSettingsResponse {
    #[serde(rename = "browserSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings: Option<Vec<BrowserSettingsSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrowserSettingsSummary {
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataProtectionSettingsRequest {
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
pub struct ListDataProtectionSettingsResponse {
    #[serde(rename = "dataProtectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings: Option<Vec<DataProtectionSettingsSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProtectionSettingsSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityProvidersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityProvidersResponse {
    #[serde(rename = "identityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_providers: Option<Vec<IdentityProviderSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProviderSummary {
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_arn: Option<String>,
    #[serde(rename = "identityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "identityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIpAccessSettingsRequest {
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
pub struct ListIpAccessSettingsResponse {
    #[serde(rename = "ipAccessSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings: Option<Vec<IpAccessSettingsSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAccessSettingsSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNetworkSettingsRequest {
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
pub struct ListNetworkSettingsResponse {
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<Vec<NetworkSettingsSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkSettingsSummary {
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortalsRequest {
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
pub struct ListPortalsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portals: Option<Vec<PortalSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortalSummary {
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings_arn: Option<String>,
    #[serde(rename = "browserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings_arn: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings_arn: Option<String>,
    #[serde(rename = "maxConcurrentSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_sessions: Option<i32>,
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings_arn: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalCustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_custom_domain: Option<String>,
    #[serde(rename = "portalEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_endpoint: Option<String>,
    #[serde(rename = "portalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_status: Option<String>,
    #[serde(rename = "rendererType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionLoggersRequest {
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
pub struct ListSessionLoggersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sessionLoggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_loggers: Option<Vec<SessionLoggerSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionLoggerSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "portalId")]
    #[serde(default)]
    pub portal_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionSummary {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
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
pub struct ListTrustStoreCertificatesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrustStoreCertificatesResponse {
    #[serde(rename = "certificateList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list: Option<Vec<CertificateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "notValidAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_after: Option<f64>,
    #[serde(rename = "notValidBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_before: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrustStoresRequest {
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
pub struct ListTrustStoresResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "trustStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_stores: Option<Vec<TrustStoreSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreSummary {
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserAccessLoggingSettingsRequest {
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
pub struct ListUserAccessLoggingSettingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "userAccessLoggingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings: Option<Vec<UserAccessLoggingSettingsSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAccessLoggingSettingsSummary {
    #[serde(rename = "kinesisStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserSettingsRequest {
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
pub struct ListUserSettingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "userSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSettingsSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSettingsSummary {
    #[serde(rename = "brandingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_configuration: Option<BrandingConfiguration>,
    #[serde(rename = "cookieSynchronizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_synchronization_configuration: Option<CookieSynchronizationConfiguration>,
    #[serde(rename = "copyAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_allowed: Option<String>,
    #[serde(rename = "deepLinkAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link_allowed: Option<String>,
    #[serde(rename = "disconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "downloadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_allowed: Option<String>,
    #[serde(rename = "idleDisconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "pasteAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paste_allowed: Option<String>,
    #[serde(rename = "printAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_allowed: Option<String>,
    #[serde(rename = "toolbarConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar_configuration: Option<ToolbarConfiguration>,
    #[serde(rename = "uploadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_allowed: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings_arn: Option<String>,
    #[serde(rename = "webAuthnAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_allowed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

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
pub struct UpdateBrowserSettingsRequest {
    #[serde(rename = "browserPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_policy: Option<String>,
    #[serde(rename = "browserSettingsArn")]
    #[serde(default)]
    pub browser_settings_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "webContentFilteringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_content_filtering_policy: Option<WebContentFilteringPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrowserSettingsResponse {
    #[serde(rename = "browserSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_settings: Option<BrowserSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataProtectionSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataProtectionSettingsArn")]
    #[serde(default)]
    pub data_protection_settings_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "inlineRedactionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_redaction_configuration: Option<InlineRedactionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataProtectionSettingsResponse {
    #[serde(rename = "dataProtectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_settings: Option<DataProtectionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityProviderRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "identityProviderArn")]
    #[serde(default)]
    pub identity_provider_arn: String,
    #[serde(rename = "identityProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "identityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "identityProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityProviderResponse {
    #[serde(rename = "identityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIpAccessSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ipAccessSettingsArn")]
    #[serde(default)]
    pub ip_access_settings_arn: String,
    #[serde(rename = "ipRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<Vec<IpRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIpAccessSettingsResponse {
    #[serde(rename = "ipAccessSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_settings: Option<IpAccessSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNetworkSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "networkSettingsArn")]
    #[serde(default)]
    pub network_settings_arn: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
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
pub struct UpdateNetworkSettingsResponse {
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<NetworkSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalRequest {
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "maxConcurrentSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_sessions: Option<i32>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    pub portal_arn: String,
    #[serde(rename = "portalCustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_custom_domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<Portal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSessionLoggerRequest {
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "eventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "sessionLoggerArn")]
    #[serde(default)]
    pub session_logger_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSessionLoggerResponse {
    #[serde(rename = "sessionLogger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_logger: Option<SessionLogger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustStoreRequest {
    #[serde(rename = "certificatesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_to_add: Option<Vec<String>>,
    #[serde(rename = "certificatesToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_to_delete: Option<Vec<String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustStoreResponse {
    #[serde(rename = "trustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserAccessLoggingSettingsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "kinesisStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_arn: Option<String>,
    #[serde(rename = "userAccessLoggingSettingsArn")]
    #[serde(default)]
    pub user_access_logging_settings_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserAccessLoggingSettingsResponse {
    #[serde(rename = "userAccessLoggingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_logging_settings: Option<UserAccessLoggingSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserSettingsRequest {
    #[serde(rename = "brandingConfigurationInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_configuration_input: Option<BrandingConfigurationUpdateInput>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "cookieSynchronizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_synchronization_configuration: Option<CookieSynchronizationConfiguration>,
    #[serde(rename = "copyAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_allowed: Option<String>,
    #[serde(rename = "deepLinkAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link_allowed: Option<String>,
    #[serde(rename = "disconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "downloadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_allowed: Option<String>,
    #[serde(rename = "idleDisconnectTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    #[serde(rename = "pasteAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paste_allowed: Option<String>,
    #[serde(rename = "printAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_allowed: Option<String>,
    #[serde(rename = "toolbarConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolbar_configuration: Option<ToolbarConfiguration>,
    #[serde(rename = "uploadAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_allowed: Option<String>,
    #[serde(rename = "userSettingsArn")]
    #[serde(default)]
    pub user_settings_arn: String,
    #[serde(rename = "webAuthnAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_allowed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandingConfigurationUpdateInput {
    #[serde(rename = "colorTheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<IconImageInput>,
    #[serde(rename = "localizedStrings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_strings: Option<std::collections::HashMap<String, LocalizedBrandingStrings>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<IconImageInput>,
    #[serde(rename = "termsOfService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpaper: Option<WallpaperImageInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserSettingsResponse {
    #[serde(rename = "userSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}
