//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-keyspaces

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_keyspace_response(result: &CreateKeyspaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_table_response(result: &CreateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_type_response(result: &CreateTypeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_keyspace_response(result: &DeleteKeyspaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_table_response(result: &DeleteTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_type_response(result: &DeleteTypeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_keyspace_response(result: &GetKeyspaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_response(result: &GetTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_auto_scaling_settings_response(
    result: &GetTableAutoScalingSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_type_response(result: &GetTypeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_keyspaces_response(result: &ListKeyspacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tables_response(result: &ListTablesResponse) -> MockResponse {
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
pub fn serialize_list_types_response(result: &ListTypesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_table_response(result: &RestoreTableResponse) -> MockResponse {
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

/// Serialize response for awsJson protocol.
pub fn serialize_update_keyspace_response(result: &UpdateKeyspaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_response(result: &UpdateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_keyspace_request(body: &[u8]) -> Result<CreateKeyspaceRequest, String> {
    if body.is_empty() {
        return Ok(CreateKeyspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateKeyspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_table_request(body: &[u8]) -> Result<CreateTableRequest, String> {
    if body.is_empty() {
        return Ok(CreateTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_type_request(body: &[u8]) -> Result<CreateTypeRequest, String> {
    if body.is_empty() {
        return Ok(CreateTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_keyspace_request(body: &[u8]) -> Result<DeleteKeyspaceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteKeyspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteKeyspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_table_request(body: &[u8]) -> Result<DeleteTableRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_type_request(body: &[u8]) -> Result<DeleteTypeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_keyspace_request(body: &[u8]) -> Result<GetKeyspaceRequest, String> {
    if body.is_empty() {
        return Ok(GetKeyspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetKeyspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_request(body: &[u8]) -> Result<GetTableRequest, String> {
    if body.is_empty() {
        return Ok(GetTableRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_auto_scaling_settings_request(
    body: &[u8],
) -> Result<GetTableAutoScalingSettingsRequest, String> {
    if body.is_empty() {
        return Ok(GetTableAutoScalingSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTableAutoScalingSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_type_request(body: &[u8]) -> Result<GetTypeRequest, String> {
    if body.is_empty() {
        return Ok(GetTypeRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_keyspaces_request(body: &[u8]) -> Result<ListKeyspacesRequest, String> {
    if body.is_empty() {
        return Ok(ListKeyspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListKeyspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tables_request(body: &[u8]) -> Result<ListTablesRequest, String> {
    if body.is_empty() {
        return Ok(ListTablesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTables request: {e}"))
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
pub fn deserialize_list_types_request(body: &[u8]) -> Result<ListTypesRequest, String> {
    if body.is_empty() {
        return Ok(ListTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_table_request(body: &[u8]) -> Result<RestoreTableRequest, String> {
    if body.is_empty() {
        return Ok(RestoreTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreTable request: {e}"))
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
pub fn deserialize_update_keyspace_request(body: &[u8]) -> Result<UpdateKeyspaceRequest, String> {
    if body.is_empty() {
        return Ok(UpdateKeyspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateKeyspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_request(body: &[u8]) -> Result<UpdateTableRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTable request: {e}"))
}
