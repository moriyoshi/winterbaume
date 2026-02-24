//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kms

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_key_deletion_response(result: &CancelKeyDeletionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_connect_custom_key_store_response(
    result: &ConnectCustomKeyStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_alias_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_custom_key_store_response(
    result: &CreateCustomKeyStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_grant_response(result: &CreateGrantResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_key_response(result: &CreateKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_decrypt_response(result: &DecryptResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_alias_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_custom_key_store_response(
    result: &DeleteCustomKeyStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_imported_key_material_response(
    result: &DeleteImportedKeyMaterialResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_derive_shared_secret_response(
    result: &DeriveSharedSecretResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_custom_key_stores_response(
    result: &DescribeCustomKeyStoresResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_key_response(result: &DescribeKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_key_rotation_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_disconnect_custom_key_store_response(
    result: &DisconnectCustomKeyStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_key_rotation_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_encrypt_response(result: &EncryptResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_data_key_response(result: &GenerateDataKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_data_key_pair_response(
    result: &GenerateDataKeyPairResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_data_key_pair_without_plaintext_response(
    result: &GenerateDataKeyPairWithoutPlaintextResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_data_key_without_plaintext_response(
    result: &GenerateDataKeyWithoutPlaintextResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_mac_response(result: &GenerateMacResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_random_response(result: &GenerateRandomResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_key_last_usage_response(result: &GetKeyLastUsageResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_key_policy_response(result: &GetKeyPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_key_rotation_status_response(
    result: &GetKeyRotationStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_parameters_for_import_response(
    result: &GetParametersForImportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_public_key_response(result: &GetPublicKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_key_material_response(result: &ImportKeyMaterialResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_aliases_response(result: &ListAliasesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_grants_response(result: &ListGrantsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_key_policies_response(result: &ListKeyPoliciesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_key_rotations_response(result: &ListKeyRotationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_keys_response(result: &ListKeysResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_tags_response(result: &ListResourceTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_retirable_grants_response(result: &ListGrantsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_key_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_re_encrypt_response(result: &ReEncryptResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_replicate_key_response(result: &ReplicateKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_retire_grant_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_revoke_grant_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_rotate_key_on_demand_response(result: &RotateKeyOnDemandResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_schedule_key_deletion_response(
    result: &ScheduleKeyDeletionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_sign_response(result: &SignResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_alias_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_custom_key_store_response(
    result: &UpdateCustomKeyStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_key_description_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_primary_region_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_verify_response(result: &VerifyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_verify_mac_response(result: &VerifyMacResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_key_deletion_request(
    body: &[u8],
) -> Result<CancelKeyDeletionRequest, String> {
    if body.is_empty() {
        return Ok(CancelKeyDeletionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelKeyDeletion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_connect_custom_key_store_request(
    body: &[u8],
) -> Result<ConnectCustomKeyStoreRequest, String> {
    if body.is_empty() {
        return Ok(ConnectCustomKeyStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConnectCustomKeyStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_alias_request(body: &[u8]) -> Result<CreateAliasRequest, String> {
    if body.is_empty() {
        return Ok(CreateAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_custom_key_store_request(
    body: &[u8],
) -> Result<CreateCustomKeyStoreRequest, String> {
    if body.is_empty() {
        return Ok(CreateCustomKeyStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCustomKeyStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_grant_request(body: &[u8]) -> Result<CreateGrantRequest, String> {
    if body.is_empty() {
        return Ok(CreateGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_key_request(body: &[u8]) -> Result<CreateKeyRequest, String> {
    if body.is_empty() {
        return Ok(CreateKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_decrypt_request(body: &[u8]) -> Result<DecryptRequest, String> {
    if body.is_empty() {
        return Ok(DecryptRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Decrypt request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_alias_request(body: &[u8]) -> Result<DeleteAliasRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_custom_key_store_request(
    body: &[u8],
) -> Result<DeleteCustomKeyStoreRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCustomKeyStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCustomKeyStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_imported_key_material_request(
    body: &[u8],
) -> Result<DeleteImportedKeyMaterialRequest, String> {
    if body.is_empty() {
        return Ok(DeleteImportedKeyMaterialRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteImportedKeyMaterial request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_derive_shared_secret_request(
    body: &[u8],
) -> Result<DeriveSharedSecretRequest, String> {
    if body.is_empty() {
        return Ok(DeriveSharedSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeriveSharedSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_custom_key_stores_request(
    body: &[u8],
) -> Result<DescribeCustomKeyStoresRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCustomKeyStoresRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCustomKeyStores request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_key_request(body: &[u8]) -> Result<DescribeKeyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_key_request(body: &[u8]) -> Result<DisableKeyRequest, String> {
    if body.is_empty() {
        return Ok(DisableKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_key_rotation_request(
    body: &[u8],
) -> Result<DisableKeyRotationRequest, String> {
    if body.is_empty() {
        return Ok(DisableKeyRotationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableKeyRotation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disconnect_custom_key_store_request(
    body: &[u8],
) -> Result<DisconnectCustomKeyStoreRequest, String> {
    if body.is_empty() {
        return Ok(DisconnectCustomKeyStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisconnectCustomKeyStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_key_request(body: &[u8]) -> Result<EnableKeyRequest, String> {
    if body.is_empty() {
        return Ok(EnableKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_key_rotation_request(
    body: &[u8],
) -> Result<EnableKeyRotationRequest, String> {
    if body.is_empty() {
        return Ok(EnableKeyRotationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableKeyRotation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_encrypt_request(body: &[u8]) -> Result<EncryptRequest, String> {
    if body.is_empty() {
        return Ok(EncryptRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Encrypt request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_data_key_request(
    body: &[u8],
) -> Result<GenerateDataKeyRequest, String> {
    if body.is_empty() {
        return Ok(GenerateDataKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateDataKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_data_key_pair_request(
    body: &[u8],
) -> Result<GenerateDataKeyPairRequest, String> {
    if body.is_empty() {
        return Ok(GenerateDataKeyPairRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateDataKeyPair request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_data_key_pair_without_plaintext_request(
    body: &[u8],
) -> Result<GenerateDataKeyPairWithoutPlaintextRequest, String> {
    if body.is_empty() {
        return Ok(GenerateDataKeyPairWithoutPlaintextRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GenerateDataKeyPairWithoutPlaintext request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_data_key_without_plaintext_request(
    body: &[u8],
) -> Result<GenerateDataKeyWithoutPlaintextRequest, String> {
    if body.is_empty() {
        return Ok(GenerateDataKeyWithoutPlaintextRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateDataKeyWithoutPlaintext request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_mac_request(body: &[u8]) -> Result<GenerateMacRequest, String> {
    if body.is_empty() {
        return Ok(GenerateMacRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateMac request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_random_request(body: &[u8]) -> Result<GenerateRandomRequest, String> {
    if body.is_empty() {
        return Ok(GenerateRandomRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateRandom request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_key_last_usage_request(
    body: &[u8],
) -> Result<GetKeyLastUsageRequest, String> {
    if body.is_empty() {
        return Ok(GetKeyLastUsageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetKeyLastUsage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_key_policy_request(body: &[u8]) -> Result<GetKeyPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetKeyPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetKeyPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_key_rotation_status_request(
    body: &[u8],
) -> Result<GetKeyRotationStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetKeyRotationStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetKeyRotationStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_parameters_for_import_request(
    body: &[u8],
) -> Result<GetParametersForImportRequest, String> {
    if body.is_empty() {
        return Ok(GetParametersForImportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetParametersForImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_public_key_request(body: &[u8]) -> Result<GetPublicKeyRequest, String> {
    if body.is_empty() {
        return Ok(GetPublicKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPublicKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_key_material_request(
    body: &[u8],
) -> Result<ImportKeyMaterialRequest, String> {
    if body.is_empty() {
        return Ok(ImportKeyMaterialRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportKeyMaterial request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_aliases_request(body: &[u8]) -> Result<ListAliasesRequest, String> {
    if body.is_empty() {
        return Ok(ListAliasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAliases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_grants_request(body: &[u8]) -> Result<ListGrantsRequest, String> {
    if body.is_empty() {
        return Ok(ListGrantsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGrants request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_key_policies_request(
    body: &[u8],
) -> Result<ListKeyPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListKeyPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListKeyPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_key_rotations_request(
    body: &[u8],
) -> Result<ListKeyRotationsRequest, String> {
    if body.is_empty() {
        return Ok(ListKeyRotationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListKeyRotations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_keys_request(body: &[u8]) -> Result<ListKeysRequest, String> {
    if body.is_empty() {
        return Ok(ListKeysRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_tags_request(
    body: &[u8],
) -> Result<ListResourceTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListResourceTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_retirable_grants_request(
    body: &[u8],
) -> Result<ListRetirableGrantsRequest, String> {
    if body.is_empty() {
        return Ok(ListRetirableGrantsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRetirableGrants request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_key_policy_request(body: &[u8]) -> Result<PutKeyPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutKeyPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutKeyPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_re_encrypt_request(body: &[u8]) -> Result<ReEncryptRequest, String> {
    if body.is_empty() {
        return Ok(ReEncryptRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReEncrypt request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_replicate_key_request(body: &[u8]) -> Result<ReplicateKeyRequest, String> {
    if body.is_empty() {
        return Ok(ReplicateKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReplicateKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_retire_grant_request(body: &[u8]) -> Result<RetireGrantRequest, String> {
    if body.is_empty() {
        return Ok(RetireGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RetireGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_revoke_grant_request(body: &[u8]) -> Result<RevokeGrantRequest, String> {
    if body.is_empty() {
        return Ok(RevokeGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RevokeGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_rotate_key_on_demand_request(
    body: &[u8],
) -> Result<RotateKeyOnDemandRequest, String> {
    if body.is_empty() {
        return Ok(RotateKeyOnDemandRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RotateKeyOnDemand request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_schedule_key_deletion_request(
    body: &[u8],
) -> Result<ScheduleKeyDeletionRequest, String> {
    if body.is_empty() {
        return Ok(ScheduleKeyDeletionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ScheduleKeyDeletion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_sign_request(body: &[u8]) -> Result<SignRequest, String> {
    if body.is_empty() {
        return Ok(SignRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Sign request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_alias_request(body: &[u8]) -> Result<UpdateAliasRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_custom_key_store_request(
    body: &[u8],
) -> Result<UpdateCustomKeyStoreRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCustomKeyStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCustomKeyStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_key_description_request(
    body: &[u8],
) -> Result<UpdateKeyDescriptionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateKeyDescriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateKeyDescription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_primary_region_request(
    body: &[u8],
) -> Result<UpdatePrimaryRegionRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePrimaryRegionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePrimaryRegion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_verify_request(body: &[u8]) -> Result<VerifyRequest, String> {
    if body.is_empty() {
        return Ok(VerifyRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Verify request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_verify_mac_request(body: &[u8]) -> Result<VerifyMacRequest, String> {
    if body.is_empty() {
        return Ok(VerifyMacRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize VerifyMac request: {e}"))
}
