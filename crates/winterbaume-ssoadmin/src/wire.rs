//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ssoadmin

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_region_response(result: &AddRegionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_attach_customer_managed_policy_reference_to_permission_set_response(
    result: &AttachCustomerManagedPolicyReferenceToPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_attach_managed_policy_to_permission_set_response(
    result: &AttachManagedPolicyToPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_account_assignment_response(
    result: &CreateAccountAssignmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_assignment_response(
    result: &CreateApplicationAssignmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_instance_response(result: &CreateInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_instance_access_control_attribute_configuration_response(
    result: &CreateInstanceAccessControlAttributeConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_permission_set_response(
    result: &CreatePermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_trusted_token_issuer_response(
    result: &CreateTrustedTokenIssuerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_account_assignment_response(
    result: &DeleteAccountAssignmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_application_access_scope_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_assignment_response(
    result: &DeleteApplicationAssignmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_application_authentication_method_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_application_grant_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_inline_policy_from_permission_set_response(
    result: &DeleteInlinePolicyFromPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_instance_response(result: &DeleteInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_instance_access_control_attribute_configuration_response(
    result: &DeleteInstanceAccessControlAttributeConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_permission_set_response(
    result: &DeletePermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_permissions_boundary_from_permission_set_response(
    result: &DeletePermissionsBoundaryFromPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_trusted_token_issuer_response(
    result: &DeleteTrustedTokenIssuerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_assignment_creation_status_response(
    result: &DescribeAccountAssignmentCreationStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_assignment_deletion_status_response(
    result: &DescribeAccountAssignmentDeletionStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_response(
    result: &DescribeApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_assignment_response(
    result: &DescribeApplicationAssignmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_provider_response(
    result: &DescribeApplicationProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_response(result: &DescribeInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_access_control_attribute_configuration_response(
    result: &DescribeInstanceAccessControlAttributeConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_permission_set_response(
    result: &DescribePermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_permission_set_provisioning_status_response(
    result: &DescribePermissionSetProvisioningStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_region_response(result: &DescribeRegionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusted_token_issuer_response(
    result: &DescribeTrustedTokenIssuerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detach_customer_managed_policy_reference_from_permission_set_response(
    result: &DetachCustomerManagedPolicyReferenceFromPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detach_managed_policy_from_permission_set_response(
    result: &DetachManagedPolicyFromPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_access_scope_response(
    result: &GetApplicationAccessScopeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_assignment_configuration_response(
    result: &GetApplicationAssignmentConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_authentication_method_response(
    result: &GetApplicationAuthenticationMethodResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_grant_response(
    result: &GetApplicationGrantResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_session_configuration_response(
    result: &GetApplicationSessionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_inline_policy_for_permission_set_response(
    result: &GetInlinePolicyForPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_permissions_boundary_for_permission_set_response(
    result: &GetPermissionsBoundaryForPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_assignment_creation_status_response(
    result: &ListAccountAssignmentCreationStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_assignment_deletion_status_response(
    result: &ListAccountAssignmentDeletionStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_assignments_response(
    result: &ListAccountAssignmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_assignments_for_principal_response(
    result: &ListAccountAssignmentsForPrincipalResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accounts_for_provisioned_permission_set_response(
    result: &ListAccountsForProvisionedPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_access_scopes_response(
    result: &ListApplicationAccessScopesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_assignments_response(
    result: &ListApplicationAssignmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_assignments_for_principal_response(
    result: &ListApplicationAssignmentsForPrincipalResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_authentication_methods_response(
    result: &ListApplicationAuthenticationMethodsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_grants_response(
    result: &ListApplicationGrantsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_providers_response(
    result: &ListApplicationProvidersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_customer_managed_policy_references_in_permission_set_response(
    result: &ListCustomerManagedPolicyReferencesInPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_instances_response(result: &ListInstancesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_managed_policies_in_permission_set_response(
    result: &ListManagedPoliciesInPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_permission_set_provisioning_status_response(
    result: &ListPermissionSetProvisioningStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_permission_sets_response(
    result: &ListPermissionSetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_permission_sets_provisioned_to_account_response(
    result: &ListPermissionSetsProvisionedToAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_regions_response(result: &ListRegionsResponse) -> MockResponse {
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
pub fn serialize_list_trusted_token_issuers_response(
    result: &ListTrustedTokenIssuersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_provision_permission_set_response(
    result: &ProvisionPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_application_access_scope_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_application_assignment_configuration_response(
    result: &PutApplicationAssignmentConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_application_authentication_method_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_application_grant_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_application_session_configuration_response(
    result: &PutApplicationSessionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_inline_policy_to_permission_set_response(
    result: &PutInlinePolicyToPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_permissions_boundary_to_permission_set_response(
    result: &PutPermissionsBoundaryToPermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_region_response(result: &RemoveRegionResponse) -> MockResponse {
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
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_instance_response(result: &UpdateInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_instance_access_control_attribute_configuration_response(
    result: &UpdateInstanceAccessControlAttributeConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_permission_set_response(
    result: &UpdatePermissionSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_trusted_token_issuer_response(
    result: &UpdateTrustedTokenIssuerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_region_request(body: &[u8]) -> Result<AddRegionRequest, String> {
    if body.is_empty() {
        return Ok(AddRegionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddRegion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_attach_customer_managed_policy_reference_to_permission_set_request(
    body: &[u8],
) -> Result<AttachCustomerManagedPolicyReferenceToPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(AttachCustomerManagedPolicyReferenceToPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize AttachCustomerManagedPolicyReferenceToPermissionSet request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_attach_managed_policy_to_permission_set_request(
    body: &[u8],
) -> Result<AttachManagedPolicyToPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(AttachManagedPolicyToPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AttachManagedPolicyToPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_account_assignment_request(
    body: &[u8],
) -> Result<CreateAccountAssignmentRequest, String> {
    if body.is_empty() {
        return Ok(CreateAccountAssignmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAccountAssignment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_request(
    body: &[u8],
) -> Result<CreateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_assignment_request(
    body: &[u8],
) -> Result<CreateApplicationAssignmentRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationAssignmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplicationAssignment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_instance_request(body: &[u8]) -> Result<CreateInstanceRequest, String> {
    if body.is_empty() {
        return Ok(CreateInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_instance_access_control_attribute_configuration_request(
    body: &[u8],
) -> Result<CreateInstanceAccessControlAttributeConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(CreateInstanceAccessControlAttributeConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize CreateInstanceAccessControlAttributeConfiguration request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_permission_set_request(
    body: &[u8],
) -> Result<CreatePermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(CreatePermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_trusted_token_issuer_request(
    body: &[u8],
) -> Result<CreateTrustedTokenIssuerRequest, String> {
    if body.is_empty() {
        return Ok(CreateTrustedTokenIssuerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTrustedTokenIssuer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_account_assignment_request(
    body: &[u8],
) -> Result<DeleteAccountAssignmentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccountAssignmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccountAssignment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_request(
    body: &[u8],
) -> Result<DeleteApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_access_scope_request(
    body: &[u8],
) -> Result<DeleteApplicationAccessScopeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationAccessScopeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplicationAccessScope request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_assignment_request(
    body: &[u8],
) -> Result<DeleteApplicationAssignmentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationAssignmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplicationAssignment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_authentication_method_request(
    body: &[u8],
) -> Result<DeleteApplicationAuthenticationMethodRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationAuthenticationMethodRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteApplicationAuthenticationMethod request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_grant_request(
    body: &[u8],
) -> Result<DeleteApplicationGrantRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplicationGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_inline_policy_from_permission_set_request(
    body: &[u8],
) -> Result<DeleteInlinePolicyFromPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteInlinePolicyFromPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteInlinePolicyFromPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_instance_request(body: &[u8]) -> Result<DeleteInstanceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_instance_access_control_attribute_configuration_request(
    body: &[u8],
) -> Result<DeleteInstanceAccessControlAttributeConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteInstanceAccessControlAttributeConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize DeleteInstanceAccessControlAttributeConfiguration request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_permission_set_request(
    body: &[u8],
) -> Result<DeletePermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DeletePermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_permissions_boundary_from_permission_set_request(
    body: &[u8],
) -> Result<DeletePermissionsBoundaryFromPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DeletePermissionsBoundaryFromPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeletePermissionsBoundaryFromPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_trusted_token_issuer_request(
    body: &[u8],
) -> Result<DeleteTrustedTokenIssuerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTrustedTokenIssuerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTrustedTokenIssuer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_assignment_creation_status_request(
    body: &[u8],
) -> Result<DescribeAccountAssignmentCreationStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountAssignmentCreationStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAccountAssignmentCreationStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_assignment_deletion_status_request(
    body: &[u8],
) -> Result<DescribeAccountAssignmentDeletionStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountAssignmentDeletionStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAccountAssignmentDeletionStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_request(
    body: &[u8],
) -> Result<DescribeApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_assignment_request(
    body: &[u8],
) -> Result<DescribeApplicationAssignmentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationAssignmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationAssignment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_provider_request(
    body: &[u8],
) -> Result<DescribeApplicationProviderRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_request(
    body: &[u8],
) -> Result<DescribeInstanceRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_access_control_attribute_configuration_request(
    body: &[u8],
) -> Result<DescribeInstanceAccessControlAttributeConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstanceAccessControlAttributeConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize DescribeInstanceAccessControlAttributeConfiguration request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_permission_set_request(
    body: &[u8],
) -> Result<DescribePermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DescribePermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_permission_set_provisioning_status_request(
    body: &[u8],
) -> Result<DescribePermissionSetProvisioningStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribePermissionSetProvisioningStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribePermissionSetProvisioningStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_region_request(body: &[u8]) -> Result<DescribeRegionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRegionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRegion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusted_token_issuer_request(
    body: &[u8],
) -> Result<DescribeTrustedTokenIssuerRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustedTokenIssuerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTrustedTokenIssuer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detach_customer_managed_policy_reference_from_permission_set_request(
    body: &[u8],
) -> Result<DetachCustomerManagedPolicyReferenceFromPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DetachCustomerManagedPolicyReferenceFromPermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetachCustomerManagedPolicyReferenceFromPermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detach_managed_policy_from_permission_set_request(
    body: &[u8],
) -> Result<DetachManagedPolicyFromPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(DetachManagedPolicyFromPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DetachManagedPolicyFromPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_access_scope_request(
    body: &[u8],
) -> Result<GetApplicationAccessScopeRequest, String> {
    if body.is_empty() {
        return Ok(GetApplicationAccessScopeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApplicationAccessScope request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_assignment_configuration_request(
    body: &[u8],
) -> Result<GetApplicationAssignmentConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetApplicationAssignmentConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetApplicationAssignmentConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_authentication_method_request(
    body: &[u8],
) -> Result<GetApplicationAuthenticationMethodRequest, String> {
    if body.is_empty() {
        return Ok(GetApplicationAuthenticationMethodRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetApplicationAuthenticationMethod request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_grant_request(
    body: &[u8],
) -> Result<GetApplicationGrantRequest, String> {
    if body.is_empty() {
        return Ok(GetApplicationGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApplicationGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_session_configuration_request(
    body: &[u8],
) -> Result<GetApplicationSessionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetApplicationSessionConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetApplicationSessionConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_inline_policy_for_permission_set_request(
    body: &[u8],
) -> Result<GetInlinePolicyForPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(GetInlinePolicyForPermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInlinePolicyForPermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_permissions_boundary_for_permission_set_request(
    body: &[u8],
) -> Result<GetPermissionsBoundaryForPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(GetPermissionsBoundaryForPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetPermissionsBoundaryForPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_assignment_creation_status_request(
    body: &[u8],
) -> Result<ListAccountAssignmentCreationStatusRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountAssignmentCreationStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAccountAssignmentCreationStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_assignment_deletion_status_request(
    body: &[u8],
) -> Result<ListAccountAssignmentDeletionStatusRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountAssignmentDeletionStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAccountAssignmentDeletionStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_assignments_request(
    body: &[u8],
) -> Result<ListAccountAssignmentsRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountAssignmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccountAssignments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_assignments_for_principal_request(
    body: &[u8],
) -> Result<ListAccountAssignmentsForPrincipalRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountAssignmentsForPrincipalRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAccountAssignmentsForPrincipal request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accounts_for_provisioned_permission_set_request(
    body: &[u8],
) -> Result<ListAccountsForProvisionedPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountsForProvisionedPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAccountsForProvisionedPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_access_scopes_request(
    body: &[u8],
) -> Result<ListApplicationAccessScopesRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationAccessScopesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationAccessScopes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_assignments_request(
    body: &[u8],
) -> Result<ListApplicationAssignmentsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationAssignmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationAssignments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_assignments_for_principal_request(
    body: &[u8],
) -> Result<ListApplicationAssignmentsForPrincipalRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationAssignmentsForPrincipalRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListApplicationAssignmentsForPrincipal request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_authentication_methods_request(
    body: &[u8],
) -> Result<ListApplicationAuthenticationMethodsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationAuthenticationMethodsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListApplicationAuthenticationMethods request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_grants_request(
    body: &[u8],
) -> Result<ListApplicationGrantsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationGrantsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationGrants request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_providers_request(
    body: &[u8],
) -> Result<ListApplicationProvidersRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationProvidersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationProviders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_applications_request(
    body: &[u8],
) -> Result<ListApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_customer_managed_policy_references_in_permission_set_request(
    body: &[u8],
) -> Result<ListCustomerManagedPolicyReferencesInPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(ListCustomerManagedPolicyReferencesInPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize ListCustomerManagedPolicyReferencesInPermissionSet request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_instances_request(body: &[u8]) -> Result<ListInstancesRequest, String> {
    if body.is_empty() {
        return Ok(ListInstancesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_managed_policies_in_permission_set_request(
    body: &[u8],
) -> Result<ListManagedPoliciesInPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(ListManagedPoliciesInPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListManagedPoliciesInPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_permission_set_provisioning_status_request(
    body: &[u8],
) -> Result<ListPermissionSetProvisioningStatusRequest, String> {
    if body.is_empty() {
        return Ok(ListPermissionSetProvisioningStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListPermissionSetProvisioningStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_permission_sets_request(
    body: &[u8],
) -> Result<ListPermissionSetsRequest, String> {
    if body.is_empty() {
        return Ok(ListPermissionSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPermissionSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_permission_sets_provisioned_to_account_request(
    body: &[u8],
) -> Result<ListPermissionSetsProvisionedToAccountRequest, String> {
    if body.is_empty() {
        return Ok(ListPermissionSetsProvisionedToAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListPermissionSetsProvisionedToAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_regions_request(body: &[u8]) -> Result<ListRegionsRequest, String> {
    if body.is_empty() {
        return Ok(ListRegionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRegions request: {e}"))
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
pub fn deserialize_list_trusted_token_issuers_request(
    body: &[u8],
) -> Result<ListTrustedTokenIssuersRequest, String> {
    if body.is_empty() {
        return Ok(ListTrustedTokenIssuersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTrustedTokenIssuers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_provision_permission_set_request(
    body: &[u8],
) -> Result<ProvisionPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(ProvisionPermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ProvisionPermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_application_access_scope_request(
    body: &[u8],
) -> Result<PutApplicationAccessScopeRequest, String> {
    if body.is_empty() {
        return Ok(PutApplicationAccessScopeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutApplicationAccessScope request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_application_assignment_configuration_request(
    body: &[u8],
) -> Result<PutApplicationAssignmentConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutApplicationAssignmentConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutApplicationAssignmentConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_application_authentication_method_request(
    body: &[u8],
) -> Result<PutApplicationAuthenticationMethodRequest, String> {
    if body.is_empty() {
        return Ok(PutApplicationAuthenticationMethodRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutApplicationAuthenticationMethod request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_application_grant_request(
    body: &[u8],
) -> Result<PutApplicationGrantRequest, String> {
    if body.is_empty() {
        return Ok(PutApplicationGrantRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutApplicationGrant request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_application_session_configuration_request(
    body: &[u8],
) -> Result<PutApplicationSessionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutApplicationSessionConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutApplicationSessionConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_inline_policy_to_permission_set_request(
    body: &[u8],
) -> Result<PutInlinePolicyToPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(PutInlinePolicyToPermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutInlinePolicyToPermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_permissions_boundary_to_permission_set_request(
    body: &[u8],
) -> Result<PutPermissionsBoundaryToPermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(PutPermissionsBoundaryToPermissionSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutPermissionsBoundaryToPermissionSet request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_region_request(body: &[u8]) -> Result<RemoveRegionRequest, String> {
    if body.is_empty() {
        return Ok(RemoveRegionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveRegion request: {e}"))
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
pub fn deserialize_update_application_request(
    body: &[u8],
) -> Result<UpdateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_instance_request(body: &[u8]) -> Result<UpdateInstanceRequest, String> {
    if body.is_empty() {
        return Ok(UpdateInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_instance_access_control_attribute_configuration_request(
    body: &[u8],
) -> Result<UpdateInstanceAccessControlAttributeConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateInstanceAccessControlAttributeConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize UpdateInstanceAccessControlAttributeConfiguration request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_permission_set_request(
    body: &[u8],
) -> Result<UpdatePermissionSetRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePermissionSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePermissionSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_trusted_token_issuer_request(
    body: &[u8],
) -> Result<UpdateTrustedTokenIssuerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTrustedTokenIssuerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTrustedTokenIssuer request: {e}"))
}
