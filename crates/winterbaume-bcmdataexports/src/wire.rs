//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bcmdataexports

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_export_response(result: &CreateExportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_export_response(result: &DeleteExportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_execution_response(result: &GetExecutionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_export_response(result: &GetExportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_response(result: &GetTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_executions_response(result: &ListExecutionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_exports_response(result: &ListExportsResponse) -> MockResponse {
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
pub fn serialize_update_export_response(result: &UpdateExportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_export_request(body: &[u8]) -> Result<CreateExportRequest, String> {
    if body.is_empty() {
        return Ok(CreateExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_export_request(body: &[u8]) -> Result<DeleteExportRequest, String> {
    if body.is_empty() {
        return Ok(DeleteExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_execution_request(body: &[u8]) -> Result<GetExecutionRequest, String> {
    if body.is_empty() {
        return Ok(GetExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_export_request(body: &[u8]) -> Result<GetExportRequest, String> {
    if body.is_empty() {
        return Ok(GetExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_request(body: &[u8]) -> Result<GetTableRequest, String> {
    if body.is_empty() {
        return Ok(GetTableRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_executions_request(body: &[u8]) -> Result<ListExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(ListExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_exports_request(body: &[u8]) -> Result<ListExportsRequest, String> {
    if body.is_empty() {
        return Ok(ListExportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExports request: {e}"))
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
pub fn deserialize_update_export_request(body: &[u8]) -> Result<UpdateExportRequest, String> {
    if body.is_empty() {
        return Ok(UpdateExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateExport request: {e}"))
}
