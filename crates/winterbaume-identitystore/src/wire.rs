//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-identitystore

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_group_membership_response(
    result: &CreateGroupMembershipResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_group_membership_response(
    result: &DeleteGroupMembershipResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_response(result: &DeleteUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_group_response(result: &DescribeGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_group_membership_response(
    result: &DescribeGroupMembershipResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_response(result: &DescribeUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_group_id_response(result: &GetGroupIdResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_group_membership_id_response(
    result: &GetGroupMembershipIdResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_id_response(result: &GetUserIdResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_is_member_in_groups_response(result: &IsMemberInGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_group_memberships_response(
    result: &ListGroupMembershipsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_group_memberships_for_member_response(
    result: &ListGroupMembershipsForMemberResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_response(result: &UpdateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_group_request(body: &[u8]) -> Result<CreateGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_group_membership_request(
    body: &[u8],
) -> Result<CreateGroupMembershipRequest, String> {
    if body.is_empty() {
        return Ok(CreateGroupMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGroupMembership request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_request(body: &[u8]) -> Result<CreateUserRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_group_request(body: &[u8]) -> Result<DeleteGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_group_membership_request(
    body: &[u8],
) -> Result<DeleteGroupMembershipRequest, String> {
    if body.is_empty() {
        return Ok(DeleteGroupMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteGroupMembership request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_request(body: &[u8]) -> Result<DeleteUserRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_group_request(body: &[u8]) -> Result<DescribeGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_group_membership_request(
    body: &[u8],
) -> Result<DescribeGroupMembershipRequest, String> {
    if body.is_empty() {
        return Ok(DescribeGroupMembershipRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeGroupMembership request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_user_request(body: &[u8]) -> Result<DescribeUserRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_group_id_request(body: &[u8]) -> Result<GetGroupIdRequest, String> {
    if body.is_empty() {
        return Ok(GetGroupIdRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetGroupId request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_group_membership_id_request(
    body: &[u8],
) -> Result<GetGroupMembershipIdRequest, String> {
    if body.is_empty() {
        return Ok(GetGroupMembershipIdRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetGroupMembershipId request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_id_request(body: &[u8]) -> Result<GetUserIdRequest, String> {
    if body.is_empty() {
        return Ok(GetUserIdRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserId request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_is_member_in_groups_request(
    body: &[u8],
) -> Result<IsMemberInGroupsRequest, String> {
    if body.is_empty() {
        return Ok(IsMemberInGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize IsMemberInGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_group_memberships_request(
    body: &[u8],
) -> Result<ListGroupMembershipsRequest, String> {
    if body.is_empty() {
        return Ok(ListGroupMembershipsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGroupMemberships request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_group_memberships_for_member_request(
    body: &[u8],
) -> Result<ListGroupMembershipsForMemberRequest, String> {
    if body.is_empty() {
        return Ok(ListGroupMembershipsForMemberRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGroupMembershipsForMember request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_groups_request(body: &[u8]) -> Result<ListGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_users_request(body: &[u8]) -> Result<ListUsersRequest, String> {
    if body.is_empty() {
        return Ok(ListUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_group_request(body: &[u8]) -> Result<UpdateGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_request(body: &[u8]) -> Result<UpdateUserRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUser request: {e}"))
}
