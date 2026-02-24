//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestreamwrite

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_batch_load_task_response(
    result: &CreateBatchLoadTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_database_response(result: &CreateDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_table_response(result: &CreateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_database_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_table_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_batch_load_task_response(
    result: &DescribeBatchLoadTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_database_response(result: &DescribeDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoints_response(result: &DescribeEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_table_response(result: &DescribeTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_batch_load_tasks_response(
    result: &ListBatchLoadTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_databases_response(result: &ListDatabasesResponse) -> MockResponse {
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
pub fn serialize_resume_batch_load_task_response(
    result: &ResumeBatchLoadTaskResponse,
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
pub fn serialize_update_database_response(result: &UpdateDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_response(result: &UpdateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_write_records_response(result: &WriteRecordsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_batch_load_task_request(
    body: &[u8],
) -> Result<CreateBatchLoadTaskRequest, String> {
    if body.is_empty() {
        return Ok(CreateBatchLoadTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBatchLoadTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_database_request(body: &[u8]) -> Result<CreateDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDatabase request: {e}"))
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
pub fn deserialize_delete_database_request(body: &[u8]) -> Result<DeleteDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDatabase request: {e}"))
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
pub fn deserialize_describe_batch_load_task_request(
    body: &[u8],
) -> Result<DescribeBatchLoadTaskRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBatchLoadTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBatchLoadTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_database_request(
    body: &[u8],
) -> Result<DescribeDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoints_request(
    body: &[u8],
) -> Result<DescribeEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_table_request(body: &[u8]) -> Result<DescribeTableRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_batch_load_tasks_request(
    body: &[u8],
) -> Result<ListBatchLoadTasksRequest, String> {
    if body.is_empty() {
        return Ok(ListBatchLoadTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBatchLoadTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_databases_request(body: &[u8]) -> Result<ListDatabasesRequest, String> {
    if body.is_empty() {
        return Ok(ListDatabasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatabases request: {e}"))
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
pub fn deserialize_resume_batch_load_task_request(
    body: &[u8],
) -> Result<ResumeBatchLoadTaskRequest, String> {
    if body.is_empty() {
        return Ok(ResumeBatchLoadTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResumeBatchLoadTask request: {e}"))
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
pub fn deserialize_update_database_request(body: &[u8]) -> Result<UpdateDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_request(body: &[u8]) -> Result<UpdateTableRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_write_records_request(body: &[u8]) -> Result<WriteRecordsRequest, String> {
    if body.is_empty() {
        return Ok(WriteRecordsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize WriteRecords request: {e}"))
}
