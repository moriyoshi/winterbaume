//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-organizations

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_handshake_response(result: &AcceptHandshakeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_attach_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_handshake_response(result: &CancelHandshakeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_close_account_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_account_response(result: &CreateAccountResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_gov_cloud_account_response(
    result: &CreateGovCloudAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_organization_response(result: &CreateOrganizationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_organizational_unit_response(
    result: &CreateOrganizationalUnitResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_policy_response(result: &CreatePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_decline_handshake_response(result: &DeclineHandshakeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_organization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_organizational_unit_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deregister_delegated_administrator_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_response(result: &DescribeAccountResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_create_account_status_response(
    result: &DescribeCreateAccountStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_effective_policy_response(
    result: &DescribeEffectivePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_handshake_response(result: &DescribeHandshakeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organization_response(
    result: &DescribeOrganizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organizational_unit_response(
    result: &DescribeOrganizationalUnitResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_policy_response(result: &DescribePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_resource_policy_response(
    result: &DescribeResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_responsibility_transfer_response(
    result: &DescribeResponsibilityTransferResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_detach_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_a_w_s_service_access_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_policy_type_response(result: &DisablePolicyTypeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_a_w_s_service_access_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_all_features_response(result: &EnableAllFeaturesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_policy_type_response(result: &EnablePolicyTypeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_invite_account_to_organization_response(
    result: &InviteAccountToOrganizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_invite_organization_to_transfer_responsibility_response(
    result: &InviteOrganizationToTransferResponsibilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_leave_organization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_a_w_s_service_access_for_organization_response(
    result: &ListAWSServiceAccessForOrganizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accounts_response(result: &ListAccountsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accounts_for_parent_response(
    result: &ListAccountsForParentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accounts_with_invalid_effective_policy_response(
    result: &ListAccountsWithInvalidEffectivePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_children_response(result: &ListChildrenResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_create_account_status_response(
    result: &ListCreateAccountStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_delegated_administrators_response(
    result: &ListDelegatedAdministratorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_delegated_services_for_account_response(
    result: &ListDelegatedServicesForAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_effective_policy_validation_errors_response(
    result: &ListEffectivePolicyValidationErrorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_handshakes_for_account_response(
    result: &ListHandshakesForAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_handshakes_for_organization_response(
    result: &ListHandshakesForOrganizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_inbound_responsibility_transfers_response(
    result: &ListInboundResponsibilityTransfersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_organizational_units_for_parent_response(
    result: &ListOrganizationalUnitsForParentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_outbound_responsibility_transfers_response(
    result: &ListOutboundResponsibilityTransfersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_parents_response(result: &ListParentsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_policies_response(result: &ListPoliciesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_policies_for_target_response(
    result: &ListPoliciesForTargetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_roots_response(result: &ListRootsResponse) -> MockResponse {
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
pub fn serialize_list_targets_for_policy_response(
    result: &ListTargetsForPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_move_account_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_delegated_administrator_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_account_from_organization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_responsibility_transfer_response(
    result: &TerminateResponsibilityTransferResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_organizational_unit_response(
    result: &UpdateOrganizationalUnitResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_policy_response(result: &UpdatePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_responsibility_transfer_response(
    result: &UpdateResponsibilityTransferResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_handshake_request(body: &[u8]) -> Result<AcceptHandshakeRequest, String> {
    if body.is_empty() {
        return Ok(AcceptHandshakeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcceptHandshake request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_attach_policy_request(body: &[u8]) -> Result<AttachPolicyRequest, String> {
    if body.is_empty() {
        return Ok(AttachPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AttachPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_handshake_request(body: &[u8]) -> Result<CancelHandshakeRequest, String> {
    if body.is_empty() {
        return Ok(CancelHandshakeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelHandshake request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_close_account_request(body: &[u8]) -> Result<CloseAccountRequest, String> {
    if body.is_empty() {
        return Ok(CloseAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CloseAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_account_request(body: &[u8]) -> Result<CreateAccountRequest, String> {
    if body.is_empty() {
        return Ok(CreateAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_gov_cloud_account_request(
    body: &[u8],
) -> Result<CreateGovCloudAccountRequest, String> {
    if body.is_empty() {
        return Ok(CreateGovCloudAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGovCloudAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_organization_request(
    body: &[u8],
) -> Result<CreateOrganizationRequest, String> {
    if body.is_empty() {
        return Ok(CreateOrganizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateOrganization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_organizational_unit_request(
    body: &[u8],
) -> Result<CreateOrganizationalUnitRequest, String> {
    if body.is_empty() {
        return Ok(CreateOrganizationalUnitRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateOrganizationalUnit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_policy_request(body: &[u8]) -> Result<CreatePolicyRequest, String> {
    if body.is_empty() {
        return Ok(CreatePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_decline_handshake_request(
    body: &[u8],
) -> Result<DeclineHandshakeRequest, String> {
    if body.is_empty() {
        return Ok(DeclineHandshakeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeclineHandshake request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_organizational_unit_request(
    body: &[u8],
) -> Result<DeleteOrganizationalUnitRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOrganizationalUnitRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteOrganizationalUnit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_policy_request(body: &[u8]) -> Result<DeletePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeletePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_delegated_administrator_request(
    body: &[u8],
) -> Result<DeregisterDelegatedAdministratorRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterDelegatedAdministratorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterDelegatedAdministrator request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_request(body: &[u8]) -> Result<DescribeAccountRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_create_account_status_request(
    body: &[u8],
) -> Result<DescribeCreateAccountStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCreateAccountStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCreateAccountStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_effective_policy_request(
    body: &[u8],
) -> Result<DescribeEffectivePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEffectivePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEffectivePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_handshake_request(
    body: &[u8],
) -> Result<DescribeHandshakeRequest, String> {
    if body.is_empty() {
        return Ok(DescribeHandshakeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeHandshake request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_organizational_unit_request(
    body: &[u8],
) -> Result<DescribeOrganizationalUnitRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOrganizationalUnitRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeOrganizationalUnit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_policy_request(body: &[u8]) -> Result<DescribePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_responsibility_transfer_request(
    body: &[u8],
) -> Result<DescribeResponsibilityTransferRequest, String> {
    if body.is_empty() {
        return Ok(DescribeResponsibilityTransferRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeResponsibilityTransfer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detach_policy_request(body: &[u8]) -> Result<DetachPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DetachPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetachPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_a_w_s_service_access_request(
    body: &[u8],
) -> Result<DisableAWSServiceAccessRequest, String> {
    if body.is_empty() {
        return Ok(DisableAWSServiceAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableAWSServiceAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_policy_type_request(
    body: &[u8],
) -> Result<DisablePolicyTypeRequest, String> {
    if body.is_empty() {
        return Ok(DisablePolicyTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisablePolicyType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_a_w_s_service_access_request(
    body: &[u8],
) -> Result<EnableAWSServiceAccessRequest, String> {
    if body.is_empty() {
        return Ok(EnableAWSServiceAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableAWSServiceAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_all_features_request(
    body: &[u8],
) -> Result<EnableAllFeaturesRequest, String> {
    if body.is_empty() {
        return Ok(EnableAllFeaturesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableAllFeatures request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_policy_type_request(
    body: &[u8],
) -> Result<EnablePolicyTypeRequest, String> {
    if body.is_empty() {
        return Ok(EnablePolicyTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnablePolicyType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_invite_account_to_organization_request(
    body: &[u8],
) -> Result<InviteAccountToOrganizationRequest, String> {
    if body.is_empty() {
        return Ok(InviteAccountToOrganizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize InviteAccountToOrganization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_invite_organization_to_transfer_responsibility_request(
    body: &[u8],
) -> Result<InviteOrganizationToTransferResponsibilityRequest, String> {
    if body.is_empty() {
        return Ok(InviteOrganizationToTransferResponsibilityRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize InviteOrganizationToTransferResponsibility request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_a_w_s_service_access_for_organization_request(
    body: &[u8],
) -> Result<ListAWSServiceAccessForOrganizationRequest, String> {
    if body.is_empty() {
        return Ok(ListAWSServiceAccessForOrganizationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAWSServiceAccessForOrganization request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accounts_request(body: &[u8]) -> Result<ListAccountsRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccounts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accounts_for_parent_request(
    body: &[u8],
) -> Result<ListAccountsForParentRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountsForParentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccountsForParent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accounts_with_invalid_effective_policy_request(
    body: &[u8],
) -> Result<ListAccountsWithInvalidEffectivePolicyRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountsWithInvalidEffectivePolicyRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAccountsWithInvalidEffectivePolicy request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_children_request(body: &[u8]) -> Result<ListChildrenRequest, String> {
    if body.is_empty() {
        return Ok(ListChildrenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListChildren request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_create_account_status_request(
    body: &[u8],
) -> Result<ListCreateAccountStatusRequest, String> {
    if body.is_empty() {
        return Ok(ListCreateAccountStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCreateAccountStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_delegated_administrators_request(
    body: &[u8],
) -> Result<ListDelegatedAdministratorsRequest, String> {
    if body.is_empty() {
        return Ok(ListDelegatedAdministratorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDelegatedAdministrators request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_delegated_services_for_account_request(
    body: &[u8],
) -> Result<ListDelegatedServicesForAccountRequest, String> {
    if body.is_empty() {
        return Ok(ListDelegatedServicesForAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDelegatedServicesForAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_effective_policy_validation_errors_request(
    body: &[u8],
) -> Result<ListEffectivePolicyValidationErrorsRequest, String> {
    if body.is_empty() {
        return Ok(ListEffectivePolicyValidationErrorsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListEffectivePolicyValidationErrors request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_handshakes_for_account_request(
    body: &[u8],
) -> Result<ListHandshakesForAccountRequest, String> {
    if body.is_empty() {
        return Ok(ListHandshakesForAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListHandshakesForAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_handshakes_for_organization_request(
    body: &[u8],
) -> Result<ListHandshakesForOrganizationRequest, String> {
    if body.is_empty() {
        return Ok(ListHandshakesForOrganizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListHandshakesForOrganization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_inbound_responsibility_transfers_request(
    body: &[u8],
) -> Result<ListInboundResponsibilityTransfersRequest, String> {
    if body.is_empty() {
        return Ok(ListInboundResponsibilityTransfersRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListInboundResponsibilityTransfers request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_organizational_units_for_parent_request(
    body: &[u8],
) -> Result<ListOrganizationalUnitsForParentRequest, String> {
    if body.is_empty() {
        return Ok(ListOrganizationalUnitsForParentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOrganizationalUnitsForParent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_outbound_responsibility_transfers_request(
    body: &[u8],
) -> Result<ListOutboundResponsibilityTransfersRequest, String> {
    if body.is_empty() {
        return Ok(ListOutboundResponsibilityTransfersRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListOutboundResponsibilityTransfers request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_parents_request(body: &[u8]) -> Result<ListParentsRequest, String> {
    if body.is_empty() {
        return Ok(ListParentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListParents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_policies_request(body: &[u8]) -> Result<ListPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_policies_for_target_request(
    body: &[u8],
) -> Result<ListPoliciesForTargetRequest, String> {
    if body.is_empty() {
        return Ok(ListPoliciesForTargetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPoliciesForTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_roots_request(body: &[u8]) -> Result<ListRootsRequest, String> {
    if body.is_empty() {
        return Ok(ListRootsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRoots request: {e}"))
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
pub fn deserialize_list_targets_for_policy_request(
    body: &[u8],
) -> Result<ListTargetsForPolicyRequest, String> {
    if body.is_empty() {
        return Ok(ListTargetsForPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTargetsForPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_move_account_request(body: &[u8]) -> Result<MoveAccountRequest, String> {
    if body.is_empty() {
        return Ok(MoveAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MoveAccount request: {e}"))
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
pub fn deserialize_register_delegated_administrator_request(
    body: &[u8],
) -> Result<RegisterDelegatedAdministratorRequest, String> {
    if body.is_empty() {
        return Ok(RegisterDelegatedAdministratorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterDelegatedAdministrator request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_account_from_organization_request(
    body: &[u8],
) -> Result<RemoveAccountFromOrganizationRequest, String> {
    if body.is_empty() {
        return Ok(RemoveAccountFromOrganizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveAccountFromOrganization request: {e}"))
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
pub fn deserialize_terminate_responsibility_transfer_request(
    body: &[u8],
) -> Result<TerminateResponsibilityTransferRequest, String> {
    if body.is_empty() {
        return Ok(TerminateResponsibilityTransferRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateResponsibilityTransfer request: {e}"))
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
pub fn deserialize_update_organizational_unit_request(
    body: &[u8],
) -> Result<UpdateOrganizationalUnitRequest, String> {
    if body.is_empty() {
        return Ok(UpdateOrganizationalUnitRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateOrganizationalUnit request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_policy_request(body: &[u8]) -> Result<UpdatePolicyRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_responsibility_transfer_request(
    body: &[u8],
) -> Result<UpdateResponsibilityTransferRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResponsibilityTransferRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResponsibilityTransfer request: {e}"))
}
