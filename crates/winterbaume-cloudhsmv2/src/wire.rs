//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudhsm-v2

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_copy_backup_to_region_response(
    result: &CopyBackupToRegionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_hsm_response(result: &CreateHsmResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_backup_response(result: &DeleteBackupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_hsm_response(result: &DeleteHsmResponse) -> MockResponse {
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
pub fn serialize_describe_backups_response(result: &DescribeBackupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_clusters_response(result: &DescribeClustersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_initialize_cluster_response(result: &InitializeClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_backup_attributes_response(
    result: &ModifyBackupAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_cluster_response(result: &ModifyClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_backup_response(result: &RestoreBackupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_backup_to_region_request(
    body: &[u8],
) -> Result<CopyBackupToRegionRequest, String> {
    if body.is_empty() {
        return Ok(CopyBackupToRegionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopyBackupToRegion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_cluster_request(body: &[u8]) -> Result<CreateClusterRequest, String> {
    if body.is_empty() {
        return Ok(CreateClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_hsm_request(body: &[u8]) -> Result<CreateHsmRequest, String> {
    if body.is_empty() {
        return Ok(CreateHsmRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateHsm request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_backup_request(body: &[u8]) -> Result<DeleteBackupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_cluster_request(body: &[u8]) -> Result<DeleteClusterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_hsm_request(body: &[u8]) -> Result<DeleteHsmRequest, String> {
    if body.is_empty() {
        return Ok(DeleteHsmRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteHsm request: {e}"))
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
pub fn deserialize_describe_backups_request(body: &[u8]) -> Result<DescribeBackupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBackupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBackups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_clusters_request(
    body: &[u8],
) -> Result<DescribeClustersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeClustersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeClusters request: {e}"))
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
pub fn deserialize_initialize_cluster_request(
    body: &[u8],
) -> Result<InitializeClusterRequest, String> {
    if body.is_empty() {
        return Ok(InitializeClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize InitializeCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_request(body: &[u8]) -> Result<ListTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_backup_attributes_request(
    body: &[u8],
) -> Result<ModifyBackupAttributesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyBackupAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyBackupAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_cluster_request(body: &[u8]) -> Result<ModifyClusterRequest, String> {
    if body.is_empty() {
        return Ok(ModifyClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyCluster request: {e}"))
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
pub fn deserialize_restore_backup_request(body: &[u8]) -> Result<RestoreBackupRequest, String> {
    if body.is_empty() {
        return Ok(RestoreBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreBackup request: {e}"))
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
