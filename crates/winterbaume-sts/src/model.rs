//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sts

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRootResult")]
pub struct AssumeRootResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "SourceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Credentials")]
pub struct Credentials {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSessionTokenResult")]
pub struct GetSessionTokenResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleRequest")]
pub struct AssumeRoleRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arns: Option<policyDescriptorListType>,
    #[serde(rename = "ProvidedContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_contexts: Option<ProvidedContextsListType>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "RoleSessionName")]
    #[serde(default)]
    pub role_session_name: String,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SourceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "TokenCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_code: Option<String>,
    #[serde(rename = "TransitiveTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitive_tag_keys: Option<tagKeyListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyDescriptorListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyDescriptorType>,
}
impl From<Vec<PolicyDescriptorType>> for policyDescriptorListType {
    fn from(v: Vec<PolicyDescriptorType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyDescriptorType> for policyDescriptorListType {
    fn from_iter<I: IntoIterator<Item = PolicyDescriptorType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyDescriptorType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyDescriptorTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyDescriptorType>,
}

impl From<Vec<PolicyDescriptorType>> for XmlPolicyDescriptorTypeList {
    fn from(v: Vec<PolicyDescriptorType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyDescriptorType> for XmlPolicyDescriptorTypeList {
    fn from_iter<I: IntoIterator<Item = PolicyDescriptorType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyDescriptorType")]
pub struct PolicyDescriptorType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvidedContextsListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ProvidedContext>,
}
impl From<Vec<ProvidedContext>> for ProvidedContextsListType {
    fn from(v: Vec<ProvidedContext>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ProvidedContext> for ProvidedContextsListType {
    fn from_iter<I: IntoIterator<Item = ProvidedContext>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ProvidedContext>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlProvidedContextList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ProvidedContext>,
}

impl From<Vec<ProvidedContext>> for XmlProvidedContextList {
    fn from(v: Vec<ProvidedContext>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ProvidedContext> for XmlProvidedContextList {
    fn from_iter<I: IntoIterator<Item = ProvidedContext>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProvidedContext")]
pub struct ProvidedContext {
    #[serde(rename = "ContextAssertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_assertion: Option<String>,
    #[serde(rename = "ProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct tagListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for tagListType {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for tagListType {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct tagKeyListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for tagKeyListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for tagKeyListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSessionTokenRequest")]
pub struct GetSessionTokenRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "TokenCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRootRequest")]
pub struct AssumeRootRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "TargetPrincipal")]
    #[serde(default)]
    pub target_principal: String,
    #[serde(rename = "TaskPolicyArn")]
    #[serde(default)]
    pub task_policy_arn: PolicyDescriptorType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecodeAuthorizationMessageResult")]
pub struct DecodeAuthorizationMessageResponse {
    #[serde(rename = "DecodedMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCallerIdentityRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFederationTokenRequest")]
pub struct GetFederationTokenRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arns: Option<policyDescriptorListType>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleWithWebIdentityResult")]
pub struct AssumeRoleWithWebIdentityResponse {
    #[serde(rename = "AssumedRoleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumedRoleUser>,
    #[serde(rename = "Audience")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "PackedPolicySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packed_policy_size: Option<i32>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "SourceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
    #[serde(rename = "SubjectFromWebIdentityToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_from_web_identity_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumedRoleUser")]
pub struct AssumedRoleUser {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssumedRoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleWithSAMLResult")]
pub struct AssumeRoleWithSAMLResponse {
    #[serde(rename = "AssumedRoleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumedRoleUser>,
    #[serde(rename = "Audience")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "NameQualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_qualifier: Option<String>,
    #[serde(rename = "PackedPolicySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packed_policy_size: Option<i32>,
    #[serde(rename = "SourceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "SubjectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleResult")]
pub struct AssumeRoleResponse {
    #[serde(rename = "AssumedRoleUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role_user: Option<AssumedRoleUser>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "PackedPolicySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packed_policy_size: Option<i32>,
    #[serde(rename = "SourceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleWithWebIdentityRequest")]
pub struct AssumeRoleWithWebIdentityRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arns: Option<policyDescriptorListType>,
    #[serde(rename = "ProviderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "RoleSessionName")]
    #[serde(default)]
    pub role_session_name: String,
    #[serde(rename = "WebIdentityToken")]
    #[serde(default)]
    pub web_identity_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDelegatedAccessTokenRequest")]
pub struct GetDelegatedAccessTokenRequest {
    #[serde(rename = "TradeInToken")]
    #[serde(default)]
    pub trade_in_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssumeRoleWithSAMLRequest")]
pub struct AssumeRoleWithSAMLRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arns: Option<policyDescriptorListType>,
    #[serde(rename = "PrincipalArn")]
    #[serde(default)]
    pub principal_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SAMLAssertion")]
    #[serde(default)]
    pub s_a_m_l_assertion: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCallerIdentityResult")]
pub struct GetCallerIdentityResponse {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDelegatedAccessTokenResult")]
pub struct GetDelegatedAccessTokenResponse {
    #[serde(rename = "AssumedPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_principal: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "PackedPolicySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packed_policy_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessKeyInfoResult")]
pub struct GetAccessKeyInfoResponse {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecodeAuthorizationMessageRequest")]
pub struct DecodeAuthorizationMessageRequest {
    #[serde(rename = "EncodedMessage")]
    #[serde(default)]
    pub encoded_message: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFederationTokenResult")]
pub struct GetFederationTokenResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "FederatedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_user: Option<FederatedUser>,
    #[serde(rename = "PackedPolicySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packed_policy_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FederatedUser")]
pub struct FederatedUser {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FederatedUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetWebIdentityTokenRequest")]
pub struct GetWebIdentityTokenRequest {
    #[serde(rename = "Audience")]
    #[serde(default)]
    pub audience: webIdentityTokenAudienceListType,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    pub signing_algorithm: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct webIdentityTokenAudienceListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for webIdentityTokenAudienceListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for webIdentityTokenAudienceListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetWebIdentityTokenResult")]
pub struct GetWebIdentityTokenResponse {
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "WebIdentityToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_identity_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessKeyInfoRequest")]
pub struct GetAccessKeyInfoRequest {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
}
