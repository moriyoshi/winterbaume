//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ram

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_accept_resource_share_invitation_response(
    result: &AcceptResourceShareInvitationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_resource_share_response(
    result: &AssociateResourceShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_resource_share_permission_response(
    result: &AssociateResourceSharePermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_permission_response(result: &CreatePermissionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_permission_version_response(
    result: &CreatePermissionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_share_response(
    result: &CreateResourceShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_permission_response(result: &DeletePermissionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_permission_version_response(
    result: &DeletePermissionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_share_response(
    result: &DeleteResourceShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_resource_share_response(
    result: &DisassociateResourceShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_resource_share_permission_response(
    result: &DisassociateResourceSharePermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_sharing_with_aws_organization_response(
    result: &EnableSharingWithAwsOrganizationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_permission_response(result: &GetPermissionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_policies_response(
    result: &GetResourcePoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_share_associations_response(
    result: &GetResourceShareAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_share_invitations_response(
    result: &GetResourceShareInvitationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_shares_response(result: &GetResourceSharesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_pending_invitation_resources_response(
    result: &ListPendingInvitationResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_permission_associations_response(
    result: &ListPermissionAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_permission_versions_response(
    result: &ListPermissionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_permissions_response(result: &ListPermissionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_principals_response(result: &ListPrincipalsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_replace_permission_associations_work_response(
    result: &ListReplacePermissionAssociationsWorkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_share_permissions_response(
    result: &ListResourceSharePermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_types_response(result: &ListResourceTypesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resources_response(result: &ListResourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_source_associations_response(
    result: &ListSourceAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_promote_permission_created_from_policy_response(
    result: &PromotePermissionCreatedFromPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_promote_resource_share_created_from_policy_response(
    result: &PromoteResourceShareCreatedFromPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_resource_share_invitation_response(
    result: &RejectResourceShareInvitationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_replace_permission_associations_response(
    result: &ReplacePermissionAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_set_default_permission_version_response(
    result: &SetDefaultPermissionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_share_response(
    result: &UpdateResourceShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_resource_share_invitation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptResourceShareInvitationRequest, String> {
    let mut input = AcceptResourceShareInvitationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AcceptResourceShareInvitationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AcceptResourceShareInvitation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_resource_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateResourceShareRequest, String> {
    let mut input = AssociateResourceShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateResourceShareRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateResourceShare request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_resource_share_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateResourceSharePermissionRequest, String> {
    let mut input = AssociateResourceSharePermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateResourceSharePermissionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateResourceSharePermission request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePermissionRequest, String> {
    let mut input = CreatePermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePermissionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePermission request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_permission_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePermissionVersionRequest, String> {
    let mut input = CreatePermissionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePermissionVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreatePermissionVersion request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceShareRequest, String> {
    let mut input = CreateResourceShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateResourceShare request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePermissionRequest, String> {
    let mut input = DeletePermissionRequest::default();
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("permissionArn") {
        input.permission_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_permission_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePermissionVersionRequest, String> {
    let mut input = DeletePermissionVersionRequest::default();
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("permissionArn") {
        input.permission_arn = value.to_string();
    }
    if let Some(value) = query.get("permissionVersion") {
        input.permission_version = value
            .parse::<i32>()
            .map_err(|err| format!("failed to parse integer: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceShareRequest, String> {
    let mut input = DeleteResourceShareRequest::default();
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceShareArn") {
        input.resource_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_resource_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateResourceShareRequest, String> {
    let mut input = DisassociateResourceShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateResourceShareRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DisassociateResourceShare request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_resource_share_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateResourceSharePermissionRequest, String> {
    let mut input = DisassociateResourceSharePermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateResourceSharePermissionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateResourceSharePermission request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_sharing_with_aws_organization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableSharingWithAwsOrganizationRequest, String> {
    let input = EnableSharingWithAwsOrganizationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPermissionRequest, String> {
    let mut input = GetPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPermissionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetPermission request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePoliciesRequest, String> {
    let mut input = GetResourcePoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourcePoliciesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetResourcePolicies request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_share_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceShareAssociationsRequest, String> {
    let mut input = GetResourceShareAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourceShareAssociationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetResourceShareAssociations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_share_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceShareInvitationsRequest, String> {
    let mut input = GetResourceShareInvitationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourceShareInvitationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetResourceShareInvitations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_shares_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceSharesRequest, String> {
    let mut input = GetResourceSharesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourceSharesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetResourceShares request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_pending_invitation_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPendingInvitationResourcesRequest, String> {
    let mut input = ListPendingInvitationResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPendingInvitationResourcesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListPendingInvitationResources request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_permission_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPermissionAssociationsRequest, String> {
    let mut input = ListPermissionAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPermissionAssociationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListPermissionAssociations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_permission_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPermissionVersionsRequest, String> {
    let mut input = ListPermissionVersionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPermissionVersionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListPermissionVersions request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPermissionsRequest, String> {
    let mut input = ListPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPermissionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPermissions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_principals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPrincipalsRequest, String> {
    let mut input = ListPrincipalsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPrincipalsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPrincipals request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_replace_permission_associations_work_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReplacePermissionAssociationsWorkRequest, String> {
    let mut input = ListReplacePermissionAssociationsWorkRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<ListReplacePermissionAssociationsWorkRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize ListReplacePermissionAssociationsWork request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_share_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceSharePermissionsRequest, String> {
    let mut input = ListResourceSharePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourceSharePermissionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListResourceSharePermissions request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceTypesRequest, String> {
    let mut input = ListResourceTypesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourceTypesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListResourceTypes request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourcesRequest, String> {
    let mut input = ListResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_source_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSourceAssociationsRequest, String> {
    let mut input = ListSourceAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSourceAssociationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListSourceAssociations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_promote_permission_created_from_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PromotePermissionCreatedFromPolicyRequest, String> {
    let mut input = PromotePermissionCreatedFromPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PromotePermissionCreatedFromPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PromotePermissionCreatedFromPolicy request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_promote_resource_share_created_from_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PromoteResourceShareCreatedFromPolicyRequest, String> {
    let mut input = PromoteResourceShareCreatedFromPolicyRequest::default();
    if let Some(value) = query.get("resourceShareArn") {
        input.resource_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_resource_share_invitation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectResourceShareInvitationRequest, String> {
    let mut input = RejectResourceShareInvitationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RejectResourceShareInvitationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RejectResourceShareInvitation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_replace_permission_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReplacePermissionAssociationsRequest, String> {
    let mut input = ReplacePermissionAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ReplacePermissionAssociationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ReplacePermissionAssociations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_default_permission_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetDefaultPermissionVersionRequest, String> {
    let mut input = SetDefaultPermissionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SetDefaultPermissionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SetDefaultPermissionVersion request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceShareRequest, String> {
    let mut input = UpdateResourceShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResourceShare request: {err}"))?;
    }
    Ok(input)
}
