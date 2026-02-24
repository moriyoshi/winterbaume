//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloud9

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_environment_e_c2_response(
    result: &CreateEnvironmentEC2Result,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_environment_membership_response(
    result: &CreateEnvironmentMembershipResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_environment_response(result: &DeleteEnvironmentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_environment_membership_response(
    result: &DeleteEnvironmentMembershipResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_environment_memberships_response(
    result: &DescribeEnvironmentMembershipsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_environment_status_response(
    result: &DescribeEnvironmentStatusResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_environments_response(
    result: &DescribeEnvironmentsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_environments_response(result: &ListEnvironmentsResult) -> MockResponse {
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
pub fn serialize_update_environment_response(result: &UpdateEnvironmentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_environment_membership_response(
    result: &UpdateEnvironmentMembershipResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_environment_e_c2_request(
    body: &[u8],
) -> Result<CreateEnvironmentEC2Request, String> {
    if body.is_empty() {
        return Ok(CreateEnvironmentEC2Request::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEnvironmentEC2 request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_environment_membership_request(
    body: &[u8],
) -> Result<CreateEnvironmentMembershipRequest, String> {
    if body.is_empty() {
        return Ok(CreateEnvironmentMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEnvironmentMembership request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_environment_request(
    body: &[u8],
) -> Result<DeleteEnvironmentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEnvironmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEnvironment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_environment_membership_request(
    body: &[u8],
) -> Result<DeleteEnvironmentMembershipRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEnvironmentMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEnvironmentMembership request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_environment_memberships_request(
    body: &[u8],
) -> Result<DescribeEnvironmentMembershipsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEnvironmentMembershipsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEnvironmentMemberships request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_environment_status_request(
    body: &[u8],
) -> Result<DescribeEnvironmentStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEnvironmentStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEnvironmentStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_environments_request(
    body: &[u8],
) -> Result<DescribeEnvironmentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEnvironmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEnvironments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_environments_request(
    body: &[u8],
) -> Result<ListEnvironmentsRequest, String> {
    if body.is_empty() {
        return Ok(ListEnvironmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEnvironments request: {e}"))
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
pub fn deserialize_update_environment_request(
    body: &[u8],
) -> Result<UpdateEnvironmentRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEnvironmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEnvironment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_environment_membership_request(
    body: &[u8],
) -> Result<UpdateEnvironmentMembershipRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEnvironmentMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEnvironmentMembership request: {e}"))
}
