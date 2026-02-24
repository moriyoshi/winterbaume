//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-eks

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAccessPolicyRequest {
    #[serde(rename = "accessScope")]
    #[serde(default)]
    pub access_scope: AccessScope,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessScope {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAccessPolicyResponse {
    #[serde(rename = "associatedAccessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_access_policy: Option<AssociatedAccessPolicy>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedAccessPolicy {
    #[serde(rename = "accessScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<AccessScope>,
    #[serde(rename = "associatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<f64>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateEncryptionConfigRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    pub encryption_config: Vec<EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Provider {
    #[serde(rename = "keyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateEncryptionConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Update {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<UpdateParam>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParam {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIdentityProviderConfigRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    pub oidc: OidcIdentityProviderConfigRequest,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OidcIdentityProviderConfigRequest {
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "groupsClaim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
    #[serde(rename = "groupsPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    #[serde(rename = "identityProviderConfigName")]
    #[serde(default)]
    pub identity_provider_config_name: String,
    #[serde(rename = "issuerUrl")]
    #[serde(default)]
    pub issuer_url: String,
    #[serde(rename = "requiredClaims")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usernameClaim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    #[serde(rename = "usernamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIdentityProviderConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessEntryRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "kubernetesGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_groups: Option<Vec<String>>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessEntryResponse {
    #[serde(rename = "accessEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_entry: Option<AccessEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessEntry {
    #[serde(rename = "accessEntryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_entry_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "kubernetesGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_groups: Option<Vec<String>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAddonRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    pub addon_name: String,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "configurationValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_values: Option<String>,
    #[serde(rename = "namespaceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_config: Option<AddonNamespaceConfigRequest>,
    #[serde(rename = "podIdentityAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_identity_associations: Option<Vec<AddonPodIdentityAssociations>>,
    #[serde(rename = "resolveConflicts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_conflicts: Option<String>,
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonNamespaceConfigRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonPodIdentityAssociations {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    pub service_account: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAddonResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Addon {
    #[serde(rename = "addonArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_arn: Option<String>,
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "configurationValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_values: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<AddonHealth>,
    #[serde(rename = "marketplaceInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_information: Option<MarketplaceInformation>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "namespaceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_config: Option<AddonNamespaceConfigResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "podIdentityAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_identity_associations: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonHealth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<AddonIssue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonIssue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketplaceInformation {
    #[serde(rename = "productId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "productUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonNamespaceConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapabilityRequest {
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CapabilityConfigurationRequest>,
    #[serde(rename = "deletePropagationPolicy")]
    #[serde(default)]
    pub delete_propagation_policy: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityConfigurationRequest {
    #[serde(rename = "argoCd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_cd: Option<ArgoCdConfigRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdConfigRequest {
    #[serde(rename = "awsIdc")]
    #[serde(default)]
    pub aws_idc: ArgoCdAwsIdcConfigRequest,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "networkAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<ArgoCdNetworkAccessConfigRequest>,
    #[serde(rename = "rbacRoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbac_role_mappings: Option<Vec<ArgoCdRoleMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdAwsIdcConfigRequest {
    #[serde(rename = "idcInstanceArn")]
    #[serde(default)]
    pub idc_instance_arn: String,
    #[serde(rename = "idcRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdNetworkAccessConfigRequest {
    #[serde(rename = "vpceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdRoleMapping {
    #[serde(default)]
    pub identities: Vec<SsoIdentity>,
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SsoIdentity {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<Capability>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capability {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CapabilityConfigurationResponse>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "deletePropagationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_propagation_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<CapabilityHealth>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityConfigurationResponse {
    #[serde(rename = "argoCd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_cd: Option<ArgoCdConfigResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdConfigResponse {
    #[serde(rename = "awsIdc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_idc: Option<ArgoCdAwsIdcConfigResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "networkAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<ArgoCdNetworkAccessConfigResponse>,
    #[serde(rename = "rbacRoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbac_role_mappings: Option<Vec<ArgoCdRoleMapping>>,
    #[serde(rename = "serverUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdAwsIdcConfigResponse {
    #[serde(rename = "idcInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_instance_arn: Option<String>,
    #[serde(rename = "idcManagedApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_managed_application_arn: Option<String>,
    #[serde(rename = "idcRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArgoCdNetworkAccessConfigResponse {
    #[serde(rename = "vpceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityHealth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<CapabilityIssue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityIssue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "accessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_config: Option<CreateAccessConfigRequest>,
    #[serde(rename = "bootstrapSelfManagedAddons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_self_managed_addons: Option<bool>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "computeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_config: Option<ComputeConfigRequest>,
    #[serde(rename = "controlPlaneScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_scaling_config: Option<ControlPlaneScalingConfig>,
    #[serde(rename = "deletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,
    #[serde(rename = "kubernetesNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfigRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outpostConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_config: Option<OutpostConfigRequest>,
    #[serde(rename = "remoteNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_network_config: Option<RemoteNetworkConfigRequest>,
    #[serde(rename = "resourcesVpcConfig")]
    #[serde(default)]
    pub resources_vpc_config: VpcConfigRequest,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "storageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_config: Option<StorageConfigRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "upgradePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicyRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "zonalShiftConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_config: Option<ZonalShiftConfigRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessConfigRequest {
    #[serde(rename = "authenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<String>,
    #[serde(rename = "bootstrapClusterCreatorAdminPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_cluster_creator_admin_permissions: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeConfigRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "nodePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_pools: Option<Vec<String>>,
    #[serde(rename = "nodeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlaneScalingConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesNetworkConfigRequest {
    #[serde(rename = "elasticLoadBalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancing: Option<ElasticLoadBalancing>,
    #[serde(rename = "ipFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_family: Option<String>,
    #[serde(rename = "serviceIpv4Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ipv4_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticLoadBalancing {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Logging {
    #[serde(rename = "clusterLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_logging: Option<Vec<LogSetup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogSetup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutpostConfigRequest {
    #[serde(rename = "controlPlaneInstanceType")]
    #[serde(default)]
    pub control_plane_instance_type: String,
    #[serde(rename = "controlPlanePlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_placement: Option<ControlPlanePlacementRequest>,
    #[serde(rename = "outpostArns")]
    #[serde(default)]
    pub outpost_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlanePlacementRequest {
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteNetworkConfigRequest {
    #[serde(rename = "remoteNodeNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_node_networks: Option<Vec<RemoteNodeNetwork>>,
    #[serde(rename = "remotePodNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_pod_networks: Option<Vec<RemotePodNetwork>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteNodeNetwork {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemotePodNetwork {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigRequest {
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    #[serde(rename = "endpointPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    #[serde(rename = "publicAccessCidrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageConfigRequest {
    #[serde(rename = "blockStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_storage: Option<BlockStorage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockStorage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradePolicyRequest {
    #[serde(rename = "supportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalShiftConfigRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "accessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_config: Option<AccessConfigResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "certificateAuthority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<Certificate>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "computeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_config: Option<ComputeConfigResponse>,
    #[serde(rename = "connectorConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_config: Option<ConnectorConfigResponse>,
    #[serde(rename = "controlPlaneScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_scaling_config: Option<ControlPlaneScalingConfig>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "deletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ClusterHealth>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "kubernetesNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfigResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outpostConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_config: Option<OutpostConfigResponse>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "remoteNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_network_config: Option<RemoteNetworkConfigResponse>,
    #[serde(rename = "resourcesVpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<VpcConfigResponse>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_config: Option<StorageConfigResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "upgradePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicyResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "zonalShiftConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_config: Option<ZonalShiftConfigResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessConfigResponse {
    #[serde(rename = "authenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<String>,
    #[serde(rename = "bootstrapClusterCreatorAdminPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_cluster_creator_admin_permissions: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "nodePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_pools: Option<Vec<String>>,
    #[serde(rename = "nodeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorConfigResponse {
    #[serde(rename = "activationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code: Option<String>,
    #[serde(rename = "activationExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_expiry: Option<f64>,
    #[serde(rename = "activationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterHealth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<ClusterIssue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterIssue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Identity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OIDC>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OIDC {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KubernetesNetworkConfigResponse {
    #[serde(rename = "elasticLoadBalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancing: Option<ElasticLoadBalancing>,
    #[serde(rename = "ipFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_family: Option<String>,
    #[serde(rename = "serviceIpv4Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ipv4_cidr: Option<String>,
    #[serde(rename = "serviceIpv6Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ipv6_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutpostConfigResponse {
    #[serde(rename = "controlPlaneInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_instance_type: Option<String>,
    #[serde(rename = "controlPlanePlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_placement: Option<ControlPlanePlacementResponse>,
    #[serde(rename = "outpostArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlanePlacementResponse {
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteNetworkConfigResponse {
    #[serde(rename = "remoteNodeNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_node_networks: Option<Vec<RemoteNodeNetwork>>,
    #[serde(rename = "remotePodNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_pod_networks: Option<Vec<RemotePodNetwork>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigResponse {
    #[serde(rename = "clusterSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_id: Option<String>,
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    #[serde(rename = "endpointPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    #[serde(rename = "publicAccessCidrs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
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
pub struct StorageConfigResponse {
    #[serde(rename = "blockStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_storage: Option<BlockStorage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradePolicyResponse {
    #[serde(rename = "supportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalShiftConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEksAnywhereSubscriptionRequest {
    #[serde(rename = "autoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "licenseQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_quantity: Option<i32>,
    #[serde(rename = "licenseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub term: EksAnywhereSubscriptionTerm,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksAnywhereSubscriptionTerm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEksAnywhereSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EksAnywhereSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksAnywhereSubscription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "autoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "effectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<f64>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "licenseArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arns: Option<Vec<String>>,
    #[serde(rename = "licenseQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_quantity: Option<i32>,
    #[serde(rename = "licenseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<License>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<EksAnywhereSubscriptionTerm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct License {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFargateProfileRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "fargateProfileName")]
    #[serde(default)]
    pub fargate_profile_name: String,
    #[serde(rename = "podExecutionRoleArn")]
    #[serde(default)]
    pub pod_execution_role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<FargateProfileSelector>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargateProfileSelector {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFargateProfileResponse {
    #[serde(rename = "fargateProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargateProfile {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "fargateProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_arn: Option<String>,
    #[serde(rename = "fargateProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<FargateProfileHealth>,
    #[serde(rename = "podExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<FargateProfileSelector>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargateProfileHealth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<FargateProfileIssue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargateProfileIssue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodegroupRequest {
    #[serde(rename = "amiType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    #[serde(rename = "capacityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "diskSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i32>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "launchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "nodeRepairConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_repair_config: Option<NodeRepairConfig>,
    #[serde(rename = "nodeRole")]
    #[serde(default)]
    pub node_role: String,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    pub nodegroup_name: String,
    #[serde(rename = "releaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(rename = "remoteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access: Option<RemoteAccessConfig>,
    #[serde(rename = "scalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    #[serde(default)]
    pub subnets: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(rename = "updateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "warmPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_config: Option<WarmPoolConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchTemplateSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeRepairConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "maxParallelNodesRepairedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_nodes_repaired_count: Option<i32>,
    #[serde(rename = "maxParallelNodesRepairedPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_nodes_repaired_percentage: Option<i32>,
    #[serde(rename = "maxUnhealthyNodeThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unhealthy_node_threshold_count: Option<i32>,
    #[serde(rename = "maxUnhealthyNodeThresholdPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unhealthy_node_threshold_percentage: Option<i32>,
    #[serde(rename = "nodeRepairConfigOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_repair_config_overrides: Option<Vec<NodeRepairConfigOverrides>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeRepairConfigOverrides {
    #[serde(rename = "minRepairWaitTimeMins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_repair_wait_time_mins: Option<i32>,
    #[serde(rename = "nodeMonitoringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_monitoring_condition: Option<String>,
    #[serde(rename = "nodeUnhealthyReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_unhealthy_reason: Option<String>,
    #[serde(rename = "repairAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoteAccessConfig {
    #[serde(rename = "ec2SshKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_ssh_key: Option<String>,
    #[serde(rename = "sourceSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodegroupScalingConfig {
    #[serde(rename = "desiredSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i32>,
    #[serde(rename = "maxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "minSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Taint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodegroupUpdateConfig {
    #[serde(rename = "maxUnavailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i32>,
    #[serde(rename = "maxUnavailablePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable_percentage: Option<i32>,
    #[serde(rename = "updateStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarmPoolConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "maxGroupPreparedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_group_prepared_capacity: Option<i32>,
    #[serde(rename = "minSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "poolState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_state: Option<String>,
    #[serde(rename = "reuseOnScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_on_scale_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodegroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Nodegroup {
    #[serde(rename = "amiType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    #[serde(rename = "capacityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "diskSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<NodegroupHealth>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "launchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "nodeRepairConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_repair_config: Option<NodeRepairConfig>,
    #[serde(rename = "nodeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[serde(rename = "nodegroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_arn: Option<String>,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
    #[serde(rename = "releaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(rename = "remoteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access: Option<RemoteAccessConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<NodegroupResources>,
    #[serde(rename = "scalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(rename = "updateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "warmPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_config: Option<WarmPoolConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodegroupHealth {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Issue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodegroupResources {
    #[serde(rename = "autoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    #[serde(rename = "remoteAccessSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_security_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePodIdentityAssociationRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "disableSessionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_session_tags: Option<bool>,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    pub service_account: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePodIdentityAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<PodIdentityAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PodIdentityAssociation {
    #[serde(rename = "associationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "disableSessionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_session_tags: Option<bool>,
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "ownerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessEntryRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessEntryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAddonRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    pub addon_name: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAddonResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapabilityRequest {
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<Capability>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEksAnywhereSubscriptionRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEksAnywhereSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EksAnywhereSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFargateProfileRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "fargateProfileName")]
    #[serde(default)]
    pub fargate_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFargateProfileResponse {
    #[serde(rename = "fargateProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodegroupRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    pub nodegroup_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodegroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePodIdentityAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePodIdentityAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<PodIdentityAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterClusterRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccessEntryRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccessEntryResponse {
    #[serde(rename = "accessEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_entry: Option<AccessEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonConfigurationRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    pub addon_name: String,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    pub addon_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonConfigurationResponse {
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    #[serde(rename = "configurationSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_schema: Option<String>,
    #[serde(rename = "podIdentityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_identity_configuration: Option<Vec<AddonPodIdentityConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonPodIdentityConfiguration {
    #[serde(rename = "recommendedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_managed_policies: Option<Vec<String>>,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    pub addon_name: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonVersionsRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "kubernetesVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
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
    pub owners: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAddonVersionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<AddonInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonInfo {
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "addonVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_versions: Option<Vec<AddonVersionInfo>>,
    #[serde(rename = "defaultNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_namespace: Option<String>,
    #[serde(rename = "marketplaceInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_information: Option<MarketplaceInformation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonVersionInfo {
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilities: Option<Vec<Compatibility>>,
    #[serde(rename = "computeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_types: Option<Vec<String>>,
    #[serde(rename = "requiresConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_configuration: Option<bool>,
    #[serde(rename = "requiresIamPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_iam_permissions: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Compatibility {
    #[serde(rename = "clusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "platformVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCapabilityRequest {
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCapabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<Capability>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterVersionsRequest {
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "clusterVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_versions: Option<Vec<String>>,
    #[serde(rename = "defaultOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_only: Option<bool>,
    #[serde(rename = "includeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
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
    pub status: Option<String>,
    #[serde(rename = "versionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterVersionsResponse {
    #[serde(rename = "clusterVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_versions: Option<Vec<ClusterVersionInformation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterVersionInformation {
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "clusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "defaultPlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_platform_version: Option<String>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "endOfExtendedSupportDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_extended_support_date: Option<f64>,
    #[serde(rename = "endOfStandardSupportDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_standard_support_date: Option<f64>,
    #[serde(rename = "kubernetesPatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_patch_version: Option<String>,
    #[serde(rename = "releaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEksAnywhereSubscriptionRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEksAnywhereSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EksAnywhereSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFargateProfileRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "fargateProfileName")]
    #[serde(default)]
    pub fargate_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFargateProfileResponse {
    #[serde(rename = "fargateProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityProviderConfigRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "identityProviderConfig")]
    #[serde(default)]
    pub identity_provider_config: IdentityProviderConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProviderConfig {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityProviderConfigResponse {
    #[serde(rename = "identityProviderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config: Option<IdentityProviderConfigResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProviderConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OidcIdentityProviderConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OidcIdentityProviderConfig {
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "groupsClaim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
    #[serde(rename = "groupsPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    #[serde(rename = "identityProviderConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config_arn: Option<String>,
    #[serde(rename = "identityProviderConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config_name: Option<String>,
    #[serde(rename = "issuerUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    #[serde(rename = "requiredClaims")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usernameClaim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    #[serde(rename = "usernamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<Insight>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insight {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "categorySpecificSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_specific_summary: Option<InsightCategorySpecificSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "insightStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_status: Option<InsightStatus>,
    #[serde(rename = "kubernetesVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[serde(rename = "lastRefreshTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<f64>,
    #[serde(rename = "lastTransitionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<InsightResourceDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightCategorySpecificSummary {
    #[serde(rename = "addonCompatibilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_compatibility_details: Option<Vec<AddonCompatibilityDetail>>,
    #[serde(rename = "deprecationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_details: Option<Vec<DeprecationDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddonCompatibilityDetail {
    #[serde(rename = "compatibleVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_versions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecationDetail {
    #[serde(rename = "clientStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_stats: Option<Vec<ClientStat>>,
    #[serde(rename = "replacedWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_with: Option<String>,
    #[serde(rename = "startServingReplacementVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_serving_replacement_version: Option<String>,
    #[serde(rename = "stopServingVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_serving_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientStat {
    #[serde(rename = "lastRequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_request_time: Option<f64>,
    #[serde(rename = "numberOfRequestsLast30Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_requests_last30_days: Option<i32>,
    #[serde(rename = "userAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightResourceDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "insightStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_status: Option<InsightStatus>,
    #[serde(rename = "kubernetesResourceUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_resource_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightsRefreshRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightsRefreshResponse {
    #[serde(rename = "endedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodegroupRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    pub nodegroup_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodegroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePodIdentityAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePodIdentityAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<PodIdentityAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUpdateRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
    #[serde(rename = "updateId")]
    #[serde(default)]
    pub update_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUpdateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAccessPolicyRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAccessPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIdentityProviderConfigRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "identityProviderConfig")]
    #[serde(default)]
    pub identity_provider_config: IdentityProviderConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIdentityProviderConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessEntriesRequest {
    #[serde(rename = "associatedPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_policy_arn: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListAccessEntriesResponse {
    #[serde(rename = "accessEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_entries: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPoliciesRequest {
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
pub struct ListAccessPoliciesResponse {
    #[serde(rename = "accessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Vec<AccessPolicy>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAddonsRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListAddonsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedAccessPoliciesRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedAccessPoliciesResponse {
    #[serde(rename = "associatedAccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_access_policies: Option<Vec<AssociatedAccessPolicy>>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapabilitiesRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListCapabilitiesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<CapabilitySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilitySummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
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
pub struct ListClustersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEksAnywhereSubscriptionsRequest {
    #[serde(rename = "includeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status: Option<Vec<String>>,
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
pub struct ListEksAnywhereSubscriptionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<EksAnywhereSubscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFargateProfilesRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListFargateProfilesResponse {
    #[serde(rename = "fargateProfileNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityProviderConfigsRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListIdentityProviderConfigsResponse {
    #[serde(rename = "identityProviderConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_configs: Option<Vec<IdentityProviderConfig>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<InsightsFilter>,
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
pub struct InsightsFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "kubernetesVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_versions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Vec<InsightSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "insightStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_status: Option<InsightStatus>,
    #[serde(rename = "kubernetesVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[serde(rename = "lastRefreshTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<f64>,
    #[serde(rename = "lastTransitionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodegroupsRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
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
pub struct ListNodegroupsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPodIdentityAssociationsRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPodIdentityAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<PodIdentityAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PodIdentityAssociationSummary {
    #[serde(rename = "associationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "ownerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    #[serde(rename = "serviceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
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
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUpdatesRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUpdatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "updateIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterClusterRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "connectorConfig")]
    #[serde(default)]
    pub connector_config: ConnectorConfigRequest,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorConfigRequest {
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInsightsRefreshRequest {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInsightsRefreshResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
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
pub struct UpdateAccessEntryRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "kubernetesGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_groups: Option<Vec<String>>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessEntryResponse {
    #[serde(rename = "accessEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_entry: Option<AccessEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAddonRequest {
    #[serde(rename = "addonName")]
    #[serde(default)]
    pub addon_name: String,
    #[serde(rename = "addonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "configurationValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_values: Option<String>,
    #[serde(rename = "podIdentityAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_identity_associations: Option<Vec<AddonPodIdentityAssociations>>,
    #[serde(rename = "resolveConflicts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_conflicts: Option<String>,
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAddonResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapabilityRequest {
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<UpdateCapabilityConfiguration>,
    #[serde(rename = "deletePropagationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_propagation_policy: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapabilityConfiguration {
    #[serde(rename = "argoCd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_cd: Option<UpdateArgoCdConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateArgoCdConfig {
    #[serde(rename = "networkAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<ArgoCdNetworkAccessConfigRequest>,
    #[serde(rename = "rbacRoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbac_role_mappings: Option<UpdateRoleMappings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleMappings {
    #[serde(rename = "addOrUpdateRoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_update_role_mappings: Option<Vec<ArgoCdRoleMapping>>,
    #[serde(rename = "removeRoleMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_role_mappings: Option<Vec<ArgoCdRoleMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterConfigRequest {
    #[serde(rename = "accessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_config: Option<UpdateAccessConfigRequest>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "computeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_config: Option<ComputeConfigRequest>,
    #[serde(rename = "controlPlaneScalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_scaling_config: Option<ControlPlaneScalingConfig>,
    #[serde(rename = "deletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "kubernetesNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfigRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "remoteNetworkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_network_config: Option<RemoteNetworkConfigRequest>,
    #[serde(rename = "resourcesVpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<VpcConfigRequest>,
    #[serde(rename = "storageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_config: Option<StorageConfigRequest>,
    #[serde(rename = "upgradePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicyRequest>,
    #[serde(rename = "zonalShiftConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_config: Option<ZonalShiftConfigRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessConfigRequest {
    #[serde(rename = "authenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterVersionRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEksAnywhereSubscriptionRequest {
    #[serde(rename = "autoRenew")]
    #[serde(default)]
    pub auto_renew: bool,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEksAnywhereSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<EksAnywhereSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodegroupConfigRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<UpdateLabelsPayload>,
    #[serde(rename = "nodeRepairConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_repair_config: Option<NodeRepairConfig>,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    pub nodegroup_name: String,
    #[serde(rename = "scalingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<UpdateTaintsPayload>,
    #[serde(rename = "updateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
    #[serde(rename = "warmPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_pool_config: Option<WarmPoolConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLabelsPayload {
    #[serde(rename = "addOrUpdateLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_update_labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "removeLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaintsPayload {
    #[serde(rename = "addOrUpdateTaints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_update_taints: Option<Vec<Taint>>,
    #[serde(rename = "removeTaints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_taints: Option<Vec<Taint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodegroupConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodegroupVersionRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "launchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "nodegroupName")]
    #[serde(default)]
    pub nodegroup_name: String,
    #[serde(rename = "releaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodegroupVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePodIdentityAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "clusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "disableSessionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_session_tags: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "targetRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePodIdentityAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<PodIdentityAssociation>,
}
