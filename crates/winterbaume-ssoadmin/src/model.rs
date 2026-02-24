//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ssoadmin

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddRegionRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddRegionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachCustomerManagedPolicyReferenceToPermissionSetRequest {
    #[serde(rename = "CustomerManagedPolicyReference")]
    #[serde(default)]
    pub customer_managed_policy_reference: CustomerManagedPolicyReference,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerManagedPolicyReference {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachCustomerManagedPolicyReferenceToPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachManagedPolicyToPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "ManagedPolicyArn")]
    #[serde(default)]
    pub managed_policy_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachManagedPolicyToPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountAssignmentRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountAssignmentResponse {
    #[serde(rename = "AccountAssignmentCreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignment_creation_status: Option<AccountAssignmentOperationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAssignmentOperationStatus {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationAssignmentRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationAssignmentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    pub application_provider_arn: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PortalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_options: Option<PortalOptions>,
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
pub struct PortalOptions {
    #[serde(rename = "SignInOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_options: Option<SignInOptions>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignInOptions {
    #[serde(rename = "ApplicationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
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
pub struct CreateApplicationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "IdentityStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceAccessControlAttributeConfigurationRequest {
    #[serde(rename = "InstanceAccessControlAttributeConfiguration")]
    #[serde(default)]
    pub instance_access_control_attribute_configuration:
        InstanceAccessControlAttributeConfiguration,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAccessControlAttributeConfiguration {
    #[serde(rename = "AccessControlAttributes")]
    #[serde(default)]
    pub access_control_attributes: Vec<AccessControlAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlAttribute {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: AccessControlAttributeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlAttributeValue {
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceAccessControlAttributeConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceResponse {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionSetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RelayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_state: Option<String>,
    #[serde(rename = "SessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionSetResponse {
    #[serde(rename = "PermissionSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<PermissionSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionSet {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "RelayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_state: Option<String>,
    #[serde(rename = "SessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustedTokenIssuerRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrustedTokenIssuerConfiguration")]
    #[serde(default)]
    pub trusted_token_issuer_configuration: TrustedTokenIssuerConfiguration,
    #[serde(rename = "TrustedTokenIssuerType")]
    #[serde(default)]
    pub trusted_token_issuer_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedTokenIssuerConfiguration {
    #[serde(rename = "OidcJwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_jwt_configuration: Option<OidcJwtConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OidcJwtConfiguration {
    #[serde(rename = "ClaimAttributePath")]
    #[serde(default)]
    pub claim_attribute_path: String,
    #[serde(rename = "IdentityStoreAttributePath")]
    #[serde(default)]
    pub identity_store_attribute_path: String,
    #[serde(rename = "IssuerUrl")]
    #[serde(default)]
    pub issuer_url: String,
    #[serde(rename = "JwksRetrievalOption")]
    #[serde(default)]
    pub jwks_retrieval_option: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrustedTokenIssuerResponse {
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAssignmentRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAssignmentResponse {
    #[serde(rename = "AccountAssignmentDeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignment_deletion_status: Option<AccountAssignmentOperationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationAccessScopeRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationAssignmentRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationAssignmentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationAuthenticationMethodRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "AuthenticationMethodType")]
    #[serde(default)]
    pub authentication_method_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationGrantRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "GrantType")]
    #[serde(default)]
    pub grant_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInlinePolicyFromPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInlinePolicyFromPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceAccessControlAttributeConfigurationRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceAccessControlAttributeConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionsBoundaryFromPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionsBoundaryFromPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustedTokenIssuerRequest {
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    pub trusted_token_issuer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustedTokenIssuerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAssignmentCreationStatusRequest {
    #[serde(rename = "AccountAssignmentCreationRequestId")]
    #[serde(default)]
    pub account_assignment_creation_request_id: String,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAssignmentCreationStatusResponse {
    #[serde(rename = "AccountAssignmentCreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignment_creation_status: Option<AccountAssignmentOperationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAssignmentDeletionStatusRequest {
    #[serde(rename = "AccountAssignmentDeletionRequestId")]
    #[serde(default)]
    pub account_assignment_deletion_request_id: String,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAssignmentDeletionStatusResponse {
    #[serde(rename = "AccountAssignmentDeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignment_deletion_status: Option<AccountAssignmentOperationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationAssignmentRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationAssignmentResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationProviderRequest {
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    pub application_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationProviderResponse {
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_provider_arn: Option<String>,
    #[serde(rename = "DisplayData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_data: Option<DisplayData>,
    #[serde(rename = "FederationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_protocol: Option<String>,
    #[serde(rename = "ResourceServerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server_config: Option<ResourceServerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayData {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "IconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceServerConfig {
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<std::collections::HashMap<String, ResourceServerScopeDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceServerScopeDetails {
    #[serde(rename = "DetailedTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_title: Option<String>,
    #[serde(rename = "LongDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationResponse {
    #[serde(rename = "ApplicationAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_account: Option<String>,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_provider_arn: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "CreatedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IdentityStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PortalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_options: Option<PortalOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceAccessControlAttributeConfigurationRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceAccessControlAttributeConfigurationResponse {
    #[serde(rename = "InstanceAccessControlAttributeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_access_control_attribute_configuration:
        Option<InstanceAccessControlAttributeConfiguration>,
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
pub struct DescribeInstanceRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceResponse {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "EncryptionConfigurationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration_details: Option<EncryptionConfigurationDetails>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
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
pub struct EncryptionConfigurationDetails {
    #[serde(rename = "EncryptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_status: Option<String>,
    #[serde(rename = "EncryptionStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_status_reason: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePermissionSetProvisioningStatusRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "ProvisionPermissionSetRequestId")]
    #[serde(default)]
    pub provision_permission_set_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePermissionSetProvisioningStatusResponse {
    #[serde(rename = "PermissionSetProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_provisioning_status: Option<PermissionSetProvisioningStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionSetProvisioningStatus {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePermissionSetResponse {
    #[serde(rename = "PermissionSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<PermissionSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionResponse {
    #[serde(rename = "AddedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date: Option<f64>,
    #[serde(rename = "IsPrimaryRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_region: Option<bool>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedTokenIssuerRequest {
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    pub trusted_token_issuer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedTokenIssuerResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_arn: Option<String>,
    #[serde(rename = "TrustedTokenIssuerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_configuration: Option<TrustedTokenIssuerConfiguration>,
    #[serde(rename = "TrustedTokenIssuerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachCustomerManagedPolicyReferenceFromPermissionSetRequest {
    #[serde(rename = "CustomerManagedPolicyReference")]
    #[serde(default)]
    pub customer_managed_policy_reference: CustomerManagedPolicyReference,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachCustomerManagedPolicyReferenceFromPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachManagedPolicyFromPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "ManagedPolicyArn")]
    #[serde(default)]
    pub managed_policy_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachManagedPolicyFromPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAccessScopeRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAccessScopeResponse {
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_targets: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAssignmentConfigurationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAssignmentConfigurationResponse {
    #[serde(rename = "AssignmentRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAuthenticationMethodRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "AuthenticationMethodType")]
    #[serde(default)]
    pub authentication_method_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationAuthenticationMethodResponse {
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<AuthenticationMethod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationMethod {
    #[serde(rename = "Iam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<IamAuthenticationMethod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamAuthenticationMethod {
    #[serde(rename = "ActorPolicy")]
    #[serde(default)]
    pub actor_policy: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationGrantRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "GrantType")]
    #[serde(default)]
    pub grant_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationGrantResponse {
    #[serde(rename = "Grant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant: Option<Grant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Grant {
    #[serde(rename = "AuthorizationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeGrant>,
    #[serde(rename = "JwtBearer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_bearer: Option<JwtBearerGrant>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<RefreshTokenGrant>,
    #[serde(rename = "TokenExchange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_exchange: Option<TokenExchangeGrant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationCodeGrant {
    #[serde(rename = "RedirectUris")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JwtBearerGrant {
    #[serde(rename = "AuthorizedTokenIssuers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_token_issuers: Option<Vec<AuthorizedTokenIssuer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizedTokenIssuer {
    #[serde(rename = "AuthorizedAudiences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_audiences: Option<Vec<String>>,
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTokenGrant {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TokenExchangeGrant {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationSessionConfigurationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationSessionConfigurationResponse {
    #[serde(rename = "UserBackgroundSessionApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_session_application_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInlinePolicyForPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInlinePolicyForPermissionSetResponse {
    #[serde(rename = "InlinePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionsBoundaryForPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionsBoundaryForPermissionSetResponse {
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<PermissionsBoundary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionsBoundary {
    #[serde(rename = "CustomerManagedPolicyReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_policy_reference: Option<CustomerManagedPolicyReference>,
    #[serde(rename = "ManagedPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentCreationStatusRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<OperationStatusFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct OperationStatusFilter {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentCreationStatusResponse {
    #[serde(rename = "AccountAssignmentsCreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignments_creation_status: Option<Vec<AccountAssignmentOperationStatusMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAssignmentOperationStatusMetadata {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentDeletionStatusRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<OperationStatusFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListAccountAssignmentDeletionStatusResponse {
    #[serde(rename = "AccountAssignmentsDeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignments_deletion_status: Option<Vec<AccountAssignmentOperationStatusMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentsForPrincipalRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListAccountAssignmentsFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentsFilter {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentsForPrincipalResponse {
    #[serde(rename = "AccountAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignments: Option<Vec<AccountAssignmentForPrincipal>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAssignmentForPrincipal {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountAssignmentsResponse {
    #[serde(rename = "AccountAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_assignments: Option<Vec<AccountAssignment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAssignment {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsForProvisionedPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "ProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsForProvisionedPermissionSetResponse {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAccessScopesRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
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
pub struct ListApplicationAccessScopesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ScopeDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScopeDetails {
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_targets: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssignmentsForPrincipalRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListApplicationAssignmentsFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssignmentsFilter {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssignmentsForPrincipalResponse {
    #[serde(rename = "ApplicationAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_assignments: Option<Vec<ApplicationAssignmentForPrincipal>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationAssignmentForPrincipal {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssignmentsRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
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
pub struct ListApplicationAssignmentsResponse {
    #[serde(rename = "ApplicationAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_assignments: Option<Vec<ApplicationAssignment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationAssignment {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAuthenticationMethodsRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAuthenticationMethodsResponse {
    #[serde(rename = "AuthenticationMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<Vec<AuthenticationMethodItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationMethodItem {
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<AuthenticationMethod>,
    #[serde(rename = "AuthenticationMethodType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationGrantsRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationGrantsResponse {
    #[serde(rename = "Grants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantItem {
    #[serde(rename = "Grant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant: Option<Grant>,
    #[serde(rename = "GrantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationProvidersRequest {
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
pub struct ListApplicationProvidersResponse {
    #[serde(rename = "ApplicationProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_providers: Option<Vec<ApplicationProvider>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationProvider {
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_provider_arn: Option<String>,
    #[serde(rename = "DisplayData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_data: Option<DisplayData>,
    #[serde(rename = "FederationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_protocol: Option<String>,
    #[serde(rename = "ResourceServerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server_config: Option<ResourceServerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListApplicationsFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListApplicationsFilter {
    #[serde(rename = "ApplicationAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_account: Option<String>,
    #[serde(rename = "ApplicationProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsResponse {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "ApplicationAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_account: Option<String>,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_provider_arn: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "CreatedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IdentityStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PortalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_options: Option<PortalOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomerManagedPolicyReferencesInPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomerManagedPolicyReferencesInPermissionSetResponse {
    #[serde(rename = "CustomerManagedPolicyReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_policy_references: Option<Vec<CustomerManagedPolicyReference>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesRequest {
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
pub struct ListInstancesResponse {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceMetadata {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
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
pub struct ListManagedPoliciesInPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedPoliciesInPermissionSetResponse {
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AttachedManagedPolicy>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachedManagedPolicy {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionSetProvisioningStatusRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<OperationStatusFilter>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListPermissionSetProvisioningStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSetsProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_sets_provisioning_status: Option<Vec<PermissionSetProvisioningStatusMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionSetProvisioningStatusMetadata {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionSetsProvisionedToAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionSetsProvisionedToAccountResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_sets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionSetsRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListPermissionSetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_sets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegionsRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListRegionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<RegionMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionMetadata {
    #[serde(rename = "AddedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date: Option<f64>,
    #[serde(rename = "IsPrimaryRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_region: Option<bool>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
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
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrustedTokenIssuersRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
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
pub struct ListTrustedTokenIssuersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrustedTokenIssuers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuers: Option<Vec<TrustedTokenIssuerMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedTokenIssuerMetadata {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_arn: Option<String>,
    #[serde(rename = "TrustedTokenIssuerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionPermissionSetResponse {
    #[serde(rename = "PermissionSetProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_provisioning_status: Option<PermissionSetProvisioningStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationAccessScopeRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_targets: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationAssignmentConfigurationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "AssignmentRequired")]
    #[serde(default)]
    pub assignment_required: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationAssignmentConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationAuthenticationMethodRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    pub authentication_method: AuthenticationMethod,
    #[serde(rename = "AuthenticationMethodType")]
    #[serde(default)]
    pub authentication_method_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationGrantRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "Grant")]
    #[serde(default)]
    pub grant: Grant,
    #[serde(rename = "GrantType")]
    #[serde(default)]
    pub grant_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationSessionConfigurationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "UserBackgroundSessionApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_session_application_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApplicationSessionConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInlinePolicyToPermissionSetRequest {
    #[serde(rename = "InlinePolicy")]
    #[serde(default)]
    pub inline_policy: String,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInlinePolicyToPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPermissionsBoundaryToPermissionSetRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    pub permissions_boundary: PermissionsBoundary,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPermissionsBoundaryToPermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
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
pub struct UpdateApplicationRequest {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PortalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_options: Option<UpdateApplicationPortalOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationPortalOptions {
    #[serde(rename = "SignInOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_options: Option<SignInOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceAccessControlAttributeConfigurationRequest {
    #[serde(rename = "InstanceAccessControlAttributeConfiguration")]
    #[serde(default)]
    pub instance_access_control_attribute_configuration:
        InstanceAccessControlAttributeConfiguration,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceAccessControlAttributeConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceRequest {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KeyType")]
    #[serde(default)]
    pub key_type: String,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePermissionSetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "PermissionSetArn")]
    #[serde(default)]
    pub permission_set_arn: String,
    #[serde(rename = "RelayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_state: Option<String>,
    #[serde(rename = "SessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePermissionSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustedTokenIssuerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    pub trusted_token_issuer_arn: String,
    #[serde(rename = "TrustedTokenIssuerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_configuration: Option<TrustedTokenIssuerUpdateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedTokenIssuerUpdateConfiguration {
    #[serde(rename = "OidcJwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_jwt_configuration: Option<OidcJwtUpdateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OidcJwtUpdateConfiguration {
    #[serde(rename = "ClaimAttributePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_attribute_path: Option<String>,
    #[serde(rename = "IdentityStoreAttributePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_attribute_path: Option<String>,
    #[serde(rename = "JwksRetrievalOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_retrieval_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrustedTokenIssuerResponse {}
