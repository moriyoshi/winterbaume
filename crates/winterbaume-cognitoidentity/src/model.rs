//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cognitoidentity

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIdentityPoolInput {
    #[serde(rename = "AllowClassicFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_classic_flow: Option<bool>,
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    #[serde(default)]
    pub allow_unauthenticated_identities: bool,
    #[serde(rename = "CognitoIdentityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
    #[serde(rename = "DeveloperProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<String>,
    #[serde(rename = "IdentityPoolName")]
    #[serde(default)]
    pub identity_pool_name: String,
    #[serde(rename = "IdentityPoolTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "OpenIdConnectProviderARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "SamlProviderARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "SupportedLoginProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoIdentityProvider {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "ServerSideTokenCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_token_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentitiesInput {
    #[serde(rename = "IdentityIdsToDelete")]
    #[serde(default)]
    pub identity_ids_to_delete: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentitiesResponse {
    #[serde(rename = "UnprocessedIdentityIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_identity_ids: Option<Vec<UnprocessedIdentityId>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedIdentityId {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityPoolInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    pub identity_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityPoolInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCredentialsForIdentityInput {
    #[serde(rename = "CustomRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    pub identity_id: String,
    #[serde(rename = "Logins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCredentialsForIdentityResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "SecretKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdInput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "Logins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdResponse {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityPoolRolesInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityPoolRolesResponse {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "RoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<std::collections::HashMap<String, RoleMapping>>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleMapping {
    #[serde(rename = "AmbiguousRoleResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambiguous_role_resolution: Option<String>,
    #[serde(rename = "RulesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_configuration: Option<RulesConfigurationType>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RulesConfigurationType {
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<MappingRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappingRule {
    #[serde(rename = "Claim")]
    #[serde(default)]
    pub claim: String,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    pub match_type: String,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpenIdTokenForDeveloperIdentityInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "Logins")]
    #[serde(default)]
    pub logins: std::collections::HashMap<String, String>,
    #[serde(rename = "PrincipalTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TokenDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_duration: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpenIdTokenForDeveloperIdentityResponse {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpenIdTokenInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    pub identity_id: String,
    #[serde(rename = "Logins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpenIdTokenResponse {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPrincipalTagAttributeMapInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "IdentityProviderName")]
    #[serde(default)]
    pub identity_provider_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPrincipalTagAttributeMapResponse {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "IdentityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "PrincipalTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UseDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_defaults: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityDescription {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Logins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityPool {
    #[serde(rename = "AllowClassicFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_classic_flow: Option<bool>,
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    #[serde(default)]
    pub allow_unauthenticated_identities: bool,
    #[serde(rename = "CognitoIdentityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
    #[serde(rename = "DeveloperProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "IdentityPoolName")]
    #[serde(default)]
    pub identity_pool_name: String,
    #[serde(rename = "IdentityPoolTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "OpenIdConnectProviderARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "SamlProviderARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "SupportedLoginProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentitiesInput {
    #[serde(rename = "HideDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_disabled: Option<bool>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    pub max_results: i32,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentitiesResponse {
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<IdentityDescription>>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityPoolsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    pub max_results: i32,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityPoolsResponse {
    #[serde(rename = "IdentityPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pools: Option<Vec<IdentityPoolShortDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityPoolShortDescription {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "IdentityPoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupDeveloperIdentityInput {
    #[serde(rename = "DeveloperUserIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_user_identifier: Option<String>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
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
pub struct LookupDeveloperIdentityResponse {
    #[serde(rename = "DeveloperUserIdentifierList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_user_identifier_list: Option<Vec<String>>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeDeveloperIdentitiesInput {
    #[serde(rename = "DestinationUserIdentifier")]
    #[serde(default)]
    pub destination_user_identifier: String,
    #[serde(rename = "DeveloperProviderName")]
    #[serde(default)]
    pub developer_provider_name: String,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "SourceUserIdentifier")]
    #[serde(default)]
    pub source_user_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeDeveloperIdentitiesResponse {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityPoolRolesInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "RoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<std::collections::HashMap<String, RoleMapping>>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    pub roles: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetPrincipalTagAttributeMapInput {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
    #[serde(rename = "IdentityProviderName")]
    #[serde(default)]
    pub identity_provider_name: String,
    #[serde(rename = "PrincipalTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UseDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_defaults: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetPrincipalTagAttributeMapResponse {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "IdentityProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "PrincipalTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UseDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_defaults: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnlinkDeveloperIdentityInput {
    #[serde(rename = "DeveloperProviderName")]
    #[serde(default)]
    pub developer_provider_name: String,
    #[serde(rename = "DeveloperUserIdentifier")]
    #[serde(default)]
    pub developer_user_identifier: String,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    pub identity_id: String,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    pub identity_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnlinkIdentityInput {
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    pub identity_id: String,
    #[serde(rename = "Logins")]
    #[serde(default)]
    pub logins: std::collections::HashMap<String, String>,
    #[serde(rename = "LoginsToRemove")]
    #[serde(default)]
    pub logins_to_remove: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
