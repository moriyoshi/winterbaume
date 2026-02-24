//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-transfer

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_access_response(result: &CreateAccessResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_agreement_response(result: &CreateAgreementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connector_response(result: &CreateConnectorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_profile_response(result: &CreateProfileResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_server_response(result: &CreateServerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_web_app_response(result: &CreateWebAppResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workflow_response(result: &CreateWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_access_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_agreement_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_connector_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_host_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_profile_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_server_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_ssh_public_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_user_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_web_app_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_web_app_customization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_workflow_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_access_response(result: &DescribeAccessResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_agreement_response(result: &DescribeAgreementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificate_response(
    result: &DescribeCertificateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connector_response(result: &DescribeConnectorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_execution_response(result: &DescribeExecutionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_host_key_response(result: &DescribeHostKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_profile_response(result: &DescribeProfileResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_security_policy_response(
    result: &DescribeSecurityPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_server_response(result: &DescribeServerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_response(result: &DescribeUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_web_app_response(result: &DescribeWebAppResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_web_app_customization_response(
    result: &DescribeWebAppCustomizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workflow_response(result: &DescribeWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_certificate_response(result: &ImportCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_host_key_response(result: &ImportHostKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_ssh_public_key_response(
    result: &ImportSshPublicKeyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accesses_response(result: &ListAccessesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_agreements_response(result: &ListAgreementsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_certificates_response(result: &ListCertificatesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_connectors_response(result: &ListConnectorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_executions_response(result: &ListExecutionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_file_transfer_results_response(
    result: &ListFileTransferResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_host_keys_response(result: &ListHostKeysResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_profiles_response(result: &ListProfilesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_security_policies_response(
    result: &ListSecurityPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_servers_response(result: &ListServersResponse) -> MockResponse {
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
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_web_apps_response(result: &ListWebAppsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_workflows_response(result: &ListWorkflowsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_workflow_step_state_response(
    result: &SendWorkflowStepStateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_directory_listing_response(
    result: &StartDirectoryListingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_file_transfer_response(result: &StartFileTransferResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_remote_delete_response(result: &StartRemoteDeleteResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_remote_move_response(result: &StartRemoteMoveResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_start_server_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_server_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_connection_response(result: &TestConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_identity_provider_response(
    result: &TestIdentityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_access_response(result: &UpdateAccessResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_agreement_response(result: &UpdateAgreementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_certificate_response(result: &UpdateCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_connector_response(result: &UpdateConnectorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_host_key_response(result: &UpdateHostKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_profile_response(result: &UpdateProfileResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_server_response(result: &UpdateServerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_response(result: &UpdateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_web_app_response(result: &UpdateWebAppResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_web_app_customization_response(
    result: &UpdateWebAppCustomizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_access_request(body: &[u8]) -> Result<CreateAccessRequest, String> {
    if body.is_empty() {
        return Ok(CreateAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_agreement_request(body: &[u8]) -> Result<CreateAgreementRequest, String> {
    if body.is_empty() {
        return Ok(CreateAgreementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAgreement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connector_request(body: &[u8]) -> Result<CreateConnectorRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_profile_request(body: &[u8]) -> Result<CreateProfileRequest, String> {
    if body.is_empty() {
        return Ok(CreateProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_server_request(body: &[u8]) -> Result<CreateServerRequest, String> {
    if body.is_empty() {
        return Ok(CreateServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateServer request: {e}"))
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
pub fn deserialize_create_web_app_request(body: &[u8]) -> Result<CreateWebAppRequest, String> {
    if body.is_empty() {
        return Ok(CreateWebAppRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWebApp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workflow_request(body: &[u8]) -> Result<CreateWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_access_request(body: &[u8]) -> Result<DeleteAccessRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_agreement_request(body: &[u8]) -> Result<DeleteAgreementRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAgreementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAgreement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_certificate_request(
    body: &[u8],
) -> Result<DeleteCertificateRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connector_request(body: &[u8]) -> Result<DeleteConnectorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_host_key_request(body: &[u8]) -> Result<DeleteHostKeyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteHostKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteHostKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_profile_request(body: &[u8]) -> Result<DeleteProfileRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_server_request(body: &[u8]) -> Result<DeleteServerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_ssh_public_key_request(
    body: &[u8],
) -> Result<DeleteSshPublicKeyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSshPublicKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSshPublicKey request: {e}"))
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
pub fn deserialize_delete_web_app_request(body: &[u8]) -> Result<DeleteWebAppRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWebAppRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebApp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_web_app_customization_request(
    body: &[u8],
) -> Result<DeleteWebAppCustomizationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWebAppCustomizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebAppCustomization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_workflow_request(body: &[u8]) -> Result<DeleteWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_access_request(body: &[u8]) -> Result<DescribeAccessRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_agreement_request(
    body: &[u8],
) -> Result<DescribeAgreementRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAgreementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAgreement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_certificate_request(
    body: &[u8],
) -> Result<DescribeCertificateRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connector_request(
    body: &[u8],
) -> Result<DescribeConnectorRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_execution_request(
    body: &[u8],
) -> Result<DescribeExecutionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_host_key_request(
    body: &[u8],
) -> Result<DescribeHostKeyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeHostKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeHostKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_profile_request(body: &[u8]) -> Result<DescribeProfileRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_security_policy_request(
    body: &[u8],
) -> Result<DescribeSecurityPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSecurityPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSecurityPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_server_request(body: &[u8]) -> Result<DescribeServerRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServer request: {e}"))
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
pub fn deserialize_describe_web_app_request(body: &[u8]) -> Result<DescribeWebAppRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWebAppRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWebApp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_web_app_customization_request(
    body: &[u8],
) -> Result<DescribeWebAppCustomizationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWebAppCustomizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWebAppCustomization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workflow_request(
    body: &[u8],
) -> Result<DescribeWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_certificate_request(
    body: &[u8],
) -> Result<ImportCertificateRequest, String> {
    if body.is_empty() {
        return Ok(ImportCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_host_key_request(body: &[u8]) -> Result<ImportHostKeyRequest, String> {
    if body.is_empty() {
        return Ok(ImportHostKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportHostKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_ssh_public_key_request(
    body: &[u8],
) -> Result<ImportSshPublicKeyRequest, String> {
    if body.is_empty() {
        return Ok(ImportSshPublicKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportSshPublicKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accesses_request(body: &[u8]) -> Result<ListAccessesRequest, String> {
    if body.is_empty() {
        return Ok(ListAccessesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccesses request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_agreements_request(body: &[u8]) -> Result<ListAgreementsRequest, String> {
    if body.is_empty() {
        return Ok(ListAgreementsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAgreements request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_certificates_request(
    body: &[u8],
) -> Result<ListCertificatesRequest, String> {
    if body.is_empty() {
        return Ok(ListCertificatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCertificates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_connectors_request(body: &[u8]) -> Result<ListConnectorsRequest, String> {
    if body.is_empty() {
        return Ok(ListConnectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConnectors request: {e}"))
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
pub fn deserialize_list_file_transfer_results_request(
    body: &[u8],
) -> Result<ListFileTransferResultsRequest, String> {
    if body.is_empty() {
        return Ok(ListFileTransferResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFileTransferResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_host_keys_request(body: &[u8]) -> Result<ListHostKeysRequest, String> {
    if body.is_empty() {
        return Ok(ListHostKeysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListHostKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_profiles_request(body: &[u8]) -> Result<ListProfilesRequest, String> {
    if body.is_empty() {
        return Ok(ListProfilesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProfiles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_security_policies_request(
    body: &[u8],
) -> Result<ListSecurityPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListSecurityPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecurityPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_servers_request(body: &[u8]) -> Result<ListServersRequest, String> {
    if body.is_empty() {
        return Ok(ListServersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServers request: {e}"))
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
pub fn deserialize_list_users_request(body: &[u8]) -> Result<ListUsersRequest, String> {
    if body.is_empty() {
        return Ok(ListUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_web_apps_request(body: &[u8]) -> Result<ListWebAppsRequest, String> {
    if body.is_empty() {
        return Ok(ListWebAppsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWebApps request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_workflows_request(body: &[u8]) -> Result<ListWorkflowsRequest, String> {
    if body.is_empty() {
        return Ok(ListWorkflowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWorkflows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_workflow_step_state_request(
    body: &[u8],
) -> Result<SendWorkflowStepStateRequest, String> {
    if body.is_empty() {
        return Ok(SendWorkflowStepStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendWorkflowStepState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_directory_listing_request(
    body: &[u8],
) -> Result<StartDirectoryListingRequest, String> {
    if body.is_empty() {
        return Ok(StartDirectoryListingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDirectoryListing request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_file_transfer_request(
    body: &[u8],
) -> Result<StartFileTransferRequest, String> {
    if body.is_empty() {
        return Ok(StartFileTransferRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFileTransfer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_remote_delete_request(
    body: &[u8],
) -> Result<StartRemoteDeleteRequest, String> {
    if body.is_empty() {
        return Ok(StartRemoteDeleteRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartRemoteDelete request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_remote_move_request(
    body: &[u8],
) -> Result<StartRemoteMoveRequest, String> {
    if body.is_empty() {
        return Ok(StartRemoteMoveRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartRemoteMove request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_server_request(body: &[u8]) -> Result<StartServerRequest, String> {
    if body.is_empty() {
        return Ok(StartServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_server_request(body: &[u8]) -> Result<StopServerRequest, String> {
    if body.is_empty() {
        return Ok(StopServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopServer request: {e}"))
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
pub fn deserialize_test_connection_request(body: &[u8]) -> Result<TestConnectionRequest, String> {
    if body.is_empty() {
        return Ok(TestConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_identity_provider_request(
    body: &[u8],
) -> Result<TestIdentityProviderRequest, String> {
    if body.is_empty() {
        return Ok(TestIdentityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestIdentityProvider request: {e}"))
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
pub fn deserialize_update_access_request(body: &[u8]) -> Result<UpdateAccessRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_agreement_request(body: &[u8]) -> Result<UpdateAgreementRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAgreementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAgreement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_certificate_request(
    body: &[u8],
) -> Result<UpdateCertificateRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_connector_request(body: &[u8]) -> Result<UpdateConnectorRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_host_key_request(body: &[u8]) -> Result<UpdateHostKeyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateHostKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateHostKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_profile_request(body: &[u8]) -> Result<UpdateProfileRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_server_request(body: &[u8]) -> Result<UpdateServerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_request(body: &[u8]) -> Result<UpdateUserRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_web_app_request(body: &[u8]) -> Result<UpdateWebAppRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWebAppRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWebApp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_web_app_customization_request(
    body: &[u8],
) -> Result<UpdateWebAppCustomizationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWebAppCustomizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWebAppCustomization request: {e}"))
}
