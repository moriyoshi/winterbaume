//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kms

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelKeyDeletionRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelKeyDeletionResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectCustomKeyStoreRequest {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    pub custom_key_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectCustomKeyStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "TargetKeyId")]
    #[serde(default)]
    pub target_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomKeyStoreRequest {
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(default)]
    pub custom_key_store_name: String,
    #[serde(rename = "CustomKeyStoreType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_type: Option<String>,
    #[serde(rename = "KeyStorePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_password: Option<String>,
    #[serde(rename = "TrustAnchorCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_certificate: Option<String>,
    #[serde(rename = "XksProxyAuthenticationCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_authentication_credential: Option<XksProxyAuthenticationCredentialType>,
    #[serde(rename = "XksProxyConnectivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_connectivity: Option<String>,
    #[serde(rename = "XksProxyUriEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_uri_endpoint: Option<String>,
    #[serde(rename = "XksProxyUriPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_uri_path: Option<String>,
    #[serde(rename = "XksProxyVpcEndpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_vpc_endpoint_service_name: Option<String>,
    #[serde(rename = "XksProxyVpcEndpointServiceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_vpc_endpoint_service_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XksProxyAuthenticationCredentialType {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    #[serde(rename = "RawSecretAccessKey")]
    #[serde(default)]
    pub raw_secret_access_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomKeyStoreResponse {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGrantRequest {
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "GranteePrincipal")]
    #[serde(default)]
    pub grantee_principal: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    pub operations: Vec<String>,
    #[serde(rename = "RetiringPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantConstraints {
    #[serde(rename = "EncryptionContextEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EncryptionContextSubset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGrantResponse {
    #[serde(rename = "GrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    #[serde(rename = "GrantToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeyRequest {
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    #[serde(rename = "MultiRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<bool>,
    #[serde(rename = "Origin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "XksKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValue")]
    #[serde(default)]
    pub tag_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "KeyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyMetadata {
    #[serde(rename = "AWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_account_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CurrentKeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_key_material_id: Option<String>,
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EncryptionAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    #[serde(rename = "ExpirationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    #[serde(rename = "KeyAgreementAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement_algorithms: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    #[serde(rename = "KeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    #[serde(rename = "KeyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    #[serde(rename = "MacAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_algorithms: Option<Vec<String>>,
    #[serde(rename = "MultiRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<bool>,
    #[serde(rename = "MultiRegionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_configuration: Option<MultiRegionConfiguration>,
    #[serde(rename = "Origin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "PendingDeletionWindowInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion_window_in_days: Option<i32>,
    #[serde(rename = "SigningAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
    #[serde(rename = "ValidTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
    #[serde(rename = "XksKeyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_key_configuration: Option<XksKeyConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionConfiguration {
    #[serde(rename = "MultiRegionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_key_type: Option<String>,
    #[serde(rename = "PrimaryKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<MultiRegionKey>,
    #[serde(rename = "ReplicaKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_keys: Option<Vec<MultiRegionKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionKey {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XksKeyConfigurationType {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecryptRequest {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "DryRunModifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_modifiers: Option<Vec<String>>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipientInfo {
    #[serde(rename = "AttestationDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_document: Option<String>,
    #[serde(rename = "KeyEncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_encryption_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecryptResponse {
    #[serde(rename = "CiphertextForRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_for_recipient: Option<String>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "Plaintext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomKeyStoreRequest {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    pub custom_key_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomKeyStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportedKeyMaterialRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportedKeyMaterialResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeriveSharedSecretRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyAgreementAlgorithm")]
    #[serde(default)]
    pub key_agreement_algorithm: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    pub public_key: String,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeriveSharedSecretResponse {
    #[serde(rename = "CiphertextForRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_for_recipient: Option<String>,
    #[serde(rename = "KeyAgreementAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement_algorithm: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_origin: Option<String>,
    #[serde(rename = "SharedSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomKeyStoresRequest {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomKeyStoresResponse {
    #[serde(rename = "CustomKeyStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_stores: Option<Vec<CustomKeyStoresListEntry>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomKeyStoresListEntry {
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    #[serde(rename = "ConnectionErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_error_code: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    #[serde(rename = "CustomKeyStoreType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_type: Option<String>,
    #[serde(rename = "TrustAnchorCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_certificate: Option<String>,
    #[serde(rename = "XksProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_configuration: Option<XksProxyConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XksProxyConfigurationType {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Connectivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<String>,
    #[serde(rename = "UriEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_endpoint: Option<String>,
    #[serde(rename = "UriPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_path: Option<String>,
    #[serde(rename = "VpcEndpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
    #[serde(rename = "VpcEndpointServiceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyRequest {
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyResponse {
    #[serde(rename = "KeyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableKeyRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableKeyRotationRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectCustomKeyStoreRequest {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    pub custom_key_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectCustomKeyStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableKeyRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableKeyRotationRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "RotationPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Plaintext")]
    #[serde(default)]
    pub plaintext: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptResponse {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyPairRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeyPairSpec")]
    #[serde(default)]
    pub key_pair_spec: String,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyPairResponse {
    #[serde(rename = "CiphertextForRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_for_recipient: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "KeyPairSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    #[serde(rename = "PrivateKeyCiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<String>,
    #[serde(rename = "PrivateKeyPlaintext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_plaintext: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyPairWithoutPlaintextRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeyPairSpec")]
    #[serde(default)]
    pub key_pair_spec: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyPairWithoutPlaintextResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "KeyPairSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    #[serde(rename = "PrivateKeyCiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    #[serde(rename = "NumberOfBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyResponse {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "CiphertextForRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_for_recipient: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "Plaintext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    #[serde(rename = "NumberOfBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateMacRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "MacAlgorithm")]
    #[serde(default)]
    pub mac_algorithm: String,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateMacResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Mac")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "MacAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateRandomRequest {
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    #[serde(rename = "NumberOfBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i32>,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateRandomResponse {
    #[serde(rename = "CiphertextForRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_for_recipient: Option<String>,
    #[serde(rename = "Plaintext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyLastUsageRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyLastUsageResponse {
    #[serde(rename = "KeyCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_creation_date: Option<f64>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyLastUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_last_usage: Option<KeyLastUsageData>,
    #[serde(rename = "TrackingStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyLastUsageData {
    #[serde(rename = "CloudTrailEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_event_id: Option<String>,
    #[serde(rename = "KmsRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_request_id: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyPolicyRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyRotationStatusRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyRotationStatusResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyRotationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,
    #[serde(rename = "NextRotationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<f64>,
    #[serde(rename = "OnDemandRotationStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_rotation_start_date: Option<f64>,
    #[serde(rename = "RotationPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersForImportRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "WrappingAlgorithm")]
    #[serde(default)]
    pub wrapping_algorithm: String,
    #[serde(rename = "WrappingKeySpec")]
    #[serde(default)]
    pub wrapping_key_spec: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersForImportResponse {
    #[serde(rename = "ImportToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_token: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "ParametersValidTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_valid_to: Option<f64>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPublicKeyRequest {
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPublicKeyResponse {
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    #[serde(rename = "EncryptionAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    #[serde(rename = "KeyAgreementAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement_algorithms: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeySpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "SigningAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportKeyMaterialRequest {
    #[serde(rename = "EncryptedKeyMaterial")]
    #[serde(default)]
    pub encrypted_key_material: String,
    #[serde(rename = "ExpirationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    #[serde(rename = "ImportToken")]
    #[serde(default)]
    pub import_token: String,
    #[serde(rename = "ImportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_type: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "KeyMaterialDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_description: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "ValidTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportKeyMaterialResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAliasesRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAliasesResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasListEntry>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasListEntry {
    #[serde(rename = "AliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(rename = "TargetKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGrantsRequest {
    #[serde(rename = "GrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    #[serde(rename = "GranteePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGrantsResponse {
    #[serde(rename = "Grants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantListEntry>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantListEntry {
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "GrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    #[serde(rename = "GranteePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    #[serde(rename = "IssuingAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_account: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(rename = "RetiringPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyPoliciesRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyPoliciesResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyRotationsRequest {
    #[serde(rename = "IncludeKeyMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_key_material: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyRotationsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Rotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotations: Option<Vec<RotationsListEntry>>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotationsListEntry {
    #[serde(rename = "ExpirationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    #[serde(rename = "ImportState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_state: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyMaterialDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_description: Option<String>,
    #[serde(rename = "KeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_id: Option<String>,
    #[serde(rename = "KeyMaterialState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_material_state: Option<String>,
    #[serde(rename = "RotationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_date: Option<f64>,
    #[serde(rename = "RotationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_type: Option<String>,
    #[serde(rename = "ValidTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeysRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeysResponse {
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyListEntry>>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyListEntry {
    #[serde(rename = "KeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTagsRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTagsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRetirableGrantsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "RetiringPrincipal")]
    #[serde(default)]
    pub retiring_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutKeyPolicyRequest {
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReEncryptRequest {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "DestinationEncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    #[serde(rename = "DestinationEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DestinationKeyId")]
    #[serde(default)]
    pub destination_key_id: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "DryRunModifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_modifiers: Option<Vec<String>>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "SourceEncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    #[serde(rename = "SourceEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SourceKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReEncryptResponse {
    #[serde(rename = "CiphertextBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<String>,
    #[serde(rename = "DestinationEncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    #[serde(rename = "DestinationKeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_key_material_id: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "SourceEncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    #[serde(rename = "SourceKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
    #[serde(rename = "SourceKeyMaterialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_material_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateKeyRequest {
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ReplicaRegion")]
    #[serde(default)]
    pub replica_region: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateKeyResponse {
    #[serde(rename = "ReplicaKeyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_key_metadata: Option<KeyMetadata>,
    #[serde(rename = "ReplicaPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_policy: Option<String>,
    #[serde(rename = "ReplicaTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetireGrantRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    #[serde(rename = "GrantToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeGrantRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantId")]
    #[serde(default)]
    pub grant_id: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateKeyOnDemandRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateKeyOnDemandResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleKeyDeletionRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "PendingWindowInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleKeyDeletionResponse {
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    #[serde(rename = "PendingWindowInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    pub signing_algorithm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Signature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "TargetKeyId")]
    #[serde(default)]
    pub target_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomKeyStoreRequest {
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(default)]
    pub custom_key_store_id: String,
    #[serde(rename = "KeyStorePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_password: Option<String>,
    #[serde(rename = "NewCustomKeyStoreName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_custom_key_store_name: Option<String>,
    #[serde(rename = "XksProxyAuthenticationCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_authentication_credential: Option<XksProxyAuthenticationCredentialType>,
    #[serde(rename = "XksProxyConnectivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_connectivity: Option<String>,
    #[serde(rename = "XksProxyUriEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_uri_endpoint: Option<String>,
    #[serde(rename = "XksProxyUriPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_uri_path: Option<String>,
    #[serde(rename = "XksProxyVpcEndpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_vpc_endpoint_service_name: Option<String>,
    #[serde(rename = "XksProxyVpcEndpointServiceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xks_proxy_vpc_endpoint_service_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomKeyStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeyDescriptionRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePrimaryRegionRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    pub primary_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyMacRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Mac")]
    #[serde(default)]
    pub mac: String,
    #[serde(rename = "MacAlgorithm")]
    #[serde(default)]
    pub mac_algorithm: String,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyMacResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "MacAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_algorithm: Option<String>,
    #[serde(rename = "MacValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_valid: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyRequest {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "GrantTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "Signature")]
    #[serde(default)]
    pub signature: String,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    pub signing_algorithm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyResponse {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "SignatureValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_valid: Option<bool>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}
