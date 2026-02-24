//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestreamquery

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_query_response(result: &CancelQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_scheduled_query_response(
    result: &CreateScheduledQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_scheduled_query_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_settings_response(
    result: &DescribeAccountSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoints_response(result: &DescribeEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scheduled_query_response(
    result: &DescribeScheduledQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_execute_scheduled_query_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_scheduled_queries_response(
    result: &ListScheduledQueriesResponse,
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
pub fn serialize_prepare_query_response(result: &PrepareQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_query_response(result: &QueryResponse) -> MockResponse {
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
pub fn serialize_update_account_settings_response(
    result: &UpdateAccountSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_scheduled_query_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_query_request(body: &[u8]) -> Result<CancelQueryRequest, String> {
    if body.is_empty() {
        return Ok(CancelQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_scheduled_query_request(
    body: &[u8],
) -> Result<CreateScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(CreateScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_scheduled_query_request(
    body: &[u8],
) -> Result<DeleteScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_settings_request(
    body: &[u8],
) -> Result<DescribeAccountSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccountSettings request: {e}"))
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
pub fn deserialize_describe_scheduled_query_request(
    body: &[u8],
) -> Result<DescribeScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_scheduled_query_request(
    body: &[u8],
) -> Result<ExecuteScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(ExecuteScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_scheduled_queries_request(
    body: &[u8],
) -> Result<ListScheduledQueriesRequest, String> {
    if body.is_empty() {
        return Ok(ListScheduledQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListScheduledQueries request: {e}"))
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
pub fn deserialize_prepare_query_request(body: &[u8]) -> Result<PrepareQueryRequest, String> {
    if body.is_empty() {
        return Ok(PrepareQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PrepareQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_query_request(body: &[u8]) -> Result<QueryRequest, String> {
    if body.is_empty() {
        return Ok(QueryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Query request: {e}"))
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
pub fn deserialize_update_account_settings_request(
    body: &[u8],
) -> Result<UpdateAccountSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAccountSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_scheduled_query_request(
    body: &[u8],
) -> Result<UpdateScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(UpdateScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateScheduledQuery request: {e}"))
}
