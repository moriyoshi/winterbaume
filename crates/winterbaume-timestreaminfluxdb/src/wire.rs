//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestream-influxdb

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_db_cluster_response(result: &CreateDbClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_db_instance_response(result: &CreateDbInstanceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_db_parameter_group_response(
    result: &CreateDbParameterGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_db_cluster_response(result: &DeleteDbClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_db_instance_response(result: &DeleteDbInstanceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_db_cluster_response(result: &GetDbClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_db_instance_response(result: &GetDbInstanceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_db_parameter_group_response(
    result: &GetDbParameterGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_db_clusters_response(result: &ListDbClustersOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_db_instances_response(result: &ListDbInstancesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_db_instances_for_cluster_response(
    result: &ListDbInstancesForClusterOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_db_parameter_groups_response(
    result: &ListDbParameterGroupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reboot_db_cluster_response(result: &RebootDbClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reboot_db_instance_response(result: &RebootDbInstanceOutput) -> MockResponse {
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
pub fn serialize_update_db_cluster_response(result: &UpdateDbClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_db_instance_response(result: &UpdateDbInstanceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_db_cluster_request(body: &[u8]) -> Result<CreateDbClusterInput, String> {
    if body.is_empty() {
        return Ok(CreateDbClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDbCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_db_instance_request(
    body: &[u8],
) -> Result<CreateDbInstanceInput, String> {
    if body.is_empty() {
        return Ok(CreateDbInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDbInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_db_parameter_group_request(
    body: &[u8],
) -> Result<CreateDbParameterGroupInput, String> {
    if body.is_empty() {
        return Ok(CreateDbParameterGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDbParameterGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_db_cluster_request(body: &[u8]) -> Result<DeleteDbClusterInput, String> {
    if body.is_empty() {
        return Ok(DeleteDbClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDbCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_db_instance_request(
    body: &[u8],
) -> Result<DeleteDbInstanceInput, String> {
    if body.is_empty() {
        return Ok(DeleteDbInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDbInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_db_cluster_request(body: &[u8]) -> Result<GetDbClusterInput, String> {
    if body.is_empty() {
        return Ok(GetDbClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDbCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_db_instance_request(body: &[u8]) -> Result<GetDbInstanceInput, String> {
    if body.is_empty() {
        return Ok(GetDbInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDbInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_db_parameter_group_request(
    body: &[u8],
) -> Result<GetDbParameterGroupInput, String> {
    if body.is_empty() {
        return Ok(GetDbParameterGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDbParameterGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_db_clusters_request(body: &[u8]) -> Result<ListDbClustersInput, String> {
    if body.is_empty() {
        return Ok(ListDbClustersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDbClusters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_db_instances_request(body: &[u8]) -> Result<ListDbInstancesInput, String> {
    if body.is_empty() {
        return Ok(ListDbInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDbInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_db_instances_for_cluster_request(
    body: &[u8],
) -> Result<ListDbInstancesForClusterInput, String> {
    if body.is_empty() {
        return Ok(ListDbInstancesForClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDbInstancesForCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_db_parameter_groups_request(
    body: &[u8],
) -> Result<ListDbParameterGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListDbParameterGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDbParameterGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reboot_db_cluster_request(body: &[u8]) -> Result<RebootDbClusterInput, String> {
    if body.is_empty() {
        return Ok(RebootDbClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebootDbCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reboot_db_instance_request(
    body: &[u8],
) -> Result<RebootDbInstanceInput, String> {
    if body.is_empty() {
        return Ok(RebootDbInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebootDbInstance request: {e}"))
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
pub fn deserialize_update_db_cluster_request(body: &[u8]) -> Result<UpdateDbClusterInput, String> {
    if body.is_empty() {
        return Ok(UpdateDbClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDbCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_db_instance_request(
    body: &[u8],
) -> Result<UpdateDbInstanceInput, String> {
    if body.is_empty() {
        return Ok(UpdateDbInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDbInstance request: {e}"))
}
