//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudcontrol

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_resource_request_response(
    result: &CancelResourceRequestOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resource_response(result: &CreateResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_response(result: &DeleteResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_response(result: &GetResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_request_status_response(
    result: &GetResourceRequestStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_requests_response(
    result: &ListResourceRequestsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resources_response(result: &ListResourcesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resource_response(result: &UpdateResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_resource_request_request(
    body: &[u8],
) -> Result<CancelResourceRequestInput, String> {
    if body.is_empty() {
        return Ok(CancelResourceRequestInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelResourceRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resource_request(body: &[u8]) -> Result<CreateResourceInput, String> {
    if body.is_empty() {
        return Ok(CreateResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_request(body: &[u8]) -> Result<DeleteResourceInput, String> {
    if body.is_empty() {
        return Ok(DeleteResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_request(body: &[u8]) -> Result<GetResourceInput, String> {
    if body.is_empty() {
        return Ok(GetResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_request_status_request(
    body: &[u8],
) -> Result<GetResourceRequestStatusInput, String> {
    if body.is_empty() {
        return Ok(GetResourceRequestStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceRequestStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_requests_request(
    body: &[u8],
) -> Result<ListResourceRequestsInput, String> {
    if body.is_empty() {
        return Ok(ListResourceRequestsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceRequests request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resources_request(body: &[u8]) -> Result<ListResourcesInput, String> {
    if body.is_empty() {
        return Ok(ListResourcesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resource_request(body: &[u8]) -> Result<UpdateResourceInput, String> {
    if body.is_empty() {
        return Ok(UpdateResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResource request: {e}"))
}
