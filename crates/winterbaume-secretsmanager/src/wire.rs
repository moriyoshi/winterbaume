//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-secretsmanager

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_secret_value_response(
    result: &BatchGetSecretValueResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_rotate_secret_response(
    result: &CancelRotateSecretResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_secret_response(result: &CreateSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_secret_response(result: &DeleteSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_secret_response(result: &DescribeSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_random_password_response(result: &GetRandomPasswordResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_secret_value_response(result: &GetSecretValueResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_secret_version_ids_response(
    result: &ListSecretVersionIdsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_secrets_response(result: &ListSecretsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_secret_value_response(result: &PutSecretValueResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_regions_from_replication_response(
    result: &RemoveRegionsFromReplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_replicate_secret_to_regions_response(
    result: &ReplicateSecretToRegionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_secret_response(result: &RestoreSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_rotate_secret_response(result: &RotateSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_replication_to_replica_response(
    result: &StopReplicationToReplicaResponse,
) -> MockResponse {
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

/// Serialize response for awsJson protocol.
pub fn serialize_update_secret_response(result: &UpdateSecretResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_secret_version_stage_response(
    result: &UpdateSecretVersionStageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_validate_resource_policy_response(
    result: &ValidateResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_secret_value_request(
    body: &[u8],
) -> Result<BatchGetSecretValueRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetSecretValueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetSecretValue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_rotate_secret_request(
    body: &[u8],
) -> Result<CancelRotateSecretRequest, String> {
    if body.is_empty() {
        return Ok(CancelRotateSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelRotateSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_secret_request(body: &[u8]) -> Result<CreateSecretRequest, String> {
    if body.is_empty() {
        return Ok(CreateSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_secret_request(body: &[u8]) -> Result<DeleteSecretRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_secret_request(body: &[u8]) -> Result<DescribeSecretRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_random_password_request(
    body: &[u8],
) -> Result<GetRandomPasswordRequest, String> {
    if body.is_empty() {
        return Ok(GetRandomPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRandomPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_secret_value_request(body: &[u8]) -> Result<GetSecretValueRequest, String> {
    if body.is_empty() {
        return Ok(GetSecretValueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSecretValue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_secret_version_ids_request(
    body: &[u8],
) -> Result<ListSecretVersionIdsRequest, String> {
    if body.is_empty() {
        return Ok(ListSecretVersionIdsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecretVersionIds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_secrets_request(body: &[u8]) -> Result<ListSecretsRequest, String> {
    if body.is_empty() {
        return Ok(ListSecretsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecrets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_secret_value_request(body: &[u8]) -> Result<PutSecretValueRequest, String> {
    if body.is_empty() {
        return Ok(PutSecretValueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutSecretValue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_regions_from_replication_request(
    body: &[u8],
) -> Result<RemoveRegionsFromReplicationRequest, String> {
    if body.is_empty() {
        return Ok(RemoveRegionsFromReplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveRegionsFromReplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_replicate_secret_to_regions_request(
    body: &[u8],
) -> Result<ReplicateSecretToRegionsRequest, String> {
    if body.is_empty() {
        return Ok(ReplicateSecretToRegionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReplicateSecretToRegions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_secret_request(body: &[u8]) -> Result<RestoreSecretRequest, String> {
    if body.is_empty() {
        return Ok(RestoreSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_rotate_secret_request(body: &[u8]) -> Result<RotateSecretRequest, String> {
    if body.is_empty() {
        return Ok(RotateSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RotateSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_replication_to_replica_request(
    body: &[u8],
) -> Result<StopReplicationToReplicaRequest, String> {
    if body.is_empty() {
        return Ok(StopReplicationToReplicaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopReplicationToReplica request: {e}"))
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
pub fn deserialize_update_secret_request(body: &[u8]) -> Result<UpdateSecretRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_secret_version_stage_request(
    body: &[u8],
) -> Result<UpdateSecretVersionStageRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSecretVersionStageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSecretVersionStage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_validate_resource_policy_request(
    body: &[u8],
) -> Result<ValidateResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(ValidateResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ValidateResourcePolicy request: {e}"))
}
