//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-directory

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_shared_directory_response(
    result: &AcceptSharedDirectoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_ip_routes_response(result: &AddIpRoutesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_region_response(result: &AddRegionResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_to_resource_response(result: &AddTagsToResourceResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_schema_extension_response(
    result: &CancelSchemaExtensionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_connect_directory_response(result: &ConnectDirectoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_alias_response(result: &CreateAliasResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_computer_response(result: &CreateComputerResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_conditional_forwarder_response(
    result: &CreateConditionalForwarderResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_directory_response(result: &CreateDirectoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_hybrid_a_d_response(result: &CreateHybridADResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_log_subscription_response(
    result: &CreateLogSubscriptionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_microsoft_a_d_response(result: &CreateMicrosoftADResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_snapshot_response(result: &CreateSnapshotResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_trust_response(result: &CreateTrustResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_a_d_assessment_response(result: &DeleteADAssessmentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_conditional_forwarder_response(
    result: &DeleteConditionalForwarderResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_directory_response(result: &DeleteDirectoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_log_subscription_response(
    result: &DeleteLogSubscriptionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_snapshot_response(result: &DeleteSnapshotResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_trust_response(result: &DeleteTrustResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_certificate_response(
    result: &DeregisterCertificateResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_event_topic_response(
    result: &DeregisterEventTopicResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_a_d_assessment_response(
    result: &DescribeADAssessmentResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_c_a_enrollment_policy_response(
    result: &DescribeCAEnrollmentPolicyResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificate_response(result: &DescribeCertificateResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_client_authentication_settings_response(
    result: &DescribeClientAuthenticationSettingsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_conditional_forwarders_response(
    result: &DescribeConditionalForwardersResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_directories_response(result: &DescribeDirectoriesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_directory_data_access_response(
    result: &DescribeDirectoryDataAccessResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_domain_controllers_response(
    result: &DescribeDomainControllersResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_event_topics_response(
    result: &DescribeEventTopicsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_hybrid_a_d_update_response(
    result: &DescribeHybridADUpdateResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_l_d_a_p_s_settings_response(
    result: &DescribeLDAPSSettingsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_regions_response(result: &DescribeRegionsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_settings_response(result: &DescribeSettingsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_shared_directories_response(
    result: &DescribeSharedDirectoriesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_snapshots_response(result: &DescribeSnapshotsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusts_response(result: &DescribeTrustsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_update_directory_response(
    result: &DescribeUpdateDirectoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_c_a_enrollment_policy_response(
    result: &DisableCAEnrollmentPolicyResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_client_authentication_response(
    result: &DisableClientAuthenticationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_directory_data_access_response(
    result: &DisableDirectoryDataAccessResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_l_d_a_p_s_response(result: &DisableLDAPSResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_radius_response(result: &DisableRadiusResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_sso_response(result: &DisableSsoResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_c_a_enrollment_policy_response(
    result: &EnableCAEnrollmentPolicyResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_client_authentication_response(
    result: &EnableClientAuthenticationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_directory_data_access_response(
    result: &EnableDirectoryDataAccessResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_l_d_a_p_s_response(result: &EnableLDAPSResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_radius_response(result: &EnableRadiusResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_sso_response(result: &EnableSsoResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_directory_limits_response(result: &GetDirectoryLimitsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_snapshot_limits_response(result: &GetSnapshotLimitsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_a_d_assessments_response(result: &ListADAssessmentsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_certificates_response(result: &ListCertificatesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_ip_routes_response(result: &ListIpRoutesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_subscriptions_response(
    result: &ListLogSubscriptionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_schema_extensions_response(
    result: &ListSchemaExtensionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_certificate_response(result: &RegisterCertificateResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_event_topic_response(result: &RegisterEventTopicResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reject_shared_directory_response(
    result: &RejectSharedDirectoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_ip_routes_response(result: &RemoveIpRoutesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_region_response(result: &RemoveRegionResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_from_resource_response(
    result: &RemoveTagsFromResourceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reset_user_password_response(result: &ResetUserPasswordResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_from_snapshot_response(
    result: &RestoreFromSnapshotResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_share_directory_response(result: &ShareDirectoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_a_d_assessment_response(result: &StartADAssessmentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_schema_extension_response(
    result: &StartSchemaExtensionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_unshare_directory_response(result: &UnshareDirectoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_conditional_forwarder_response(
    result: &UpdateConditionalForwarderResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_directory_setup_response(
    result: &UpdateDirectorySetupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_hybrid_a_d_response(result: &UpdateHybridADResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_number_of_domain_controllers_response(
    result: &UpdateNumberOfDomainControllersResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_radius_response(result: &UpdateRadiusResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_settings_response(result: &UpdateSettingsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_trust_response(result: &UpdateTrustResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_verify_trust_response(result: &VerifyTrustResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_shared_directory_request(
    body: &[u8],
) -> Result<AcceptSharedDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(AcceptSharedDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcceptSharedDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_ip_routes_request(body: &[u8]) -> Result<AddIpRoutesRequest, String> {
    if body.is_empty() {
        return Ok(AddIpRoutesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddIpRoutes request: {e}"))
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
pub fn deserialize_add_tags_to_resource_request(
    body: &[u8],
) -> Result<AddTagsToResourceRequest, String> {
    if body.is_empty() {
        return Ok(AddTagsToResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_schema_extension_request(
    body: &[u8],
) -> Result<CancelSchemaExtensionRequest, String> {
    if body.is_empty() {
        return Ok(CancelSchemaExtensionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelSchemaExtension request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_connect_directory_request(
    body: &[u8],
) -> Result<ConnectDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(ConnectDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConnectDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_alias_request(body: &[u8]) -> Result<CreateAliasRequest, String> {
    if body.is_empty() {
        return Ok(CreateAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_computer_request(body: &[u8]) -> Result<CreateComputerRequest, String> {
    if body.is_empty() {
        return Ok(CreateComputerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateComputer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_conditional_forwarder_request(
    body: &[u8],
) -> Result<CreateConditionalForwarderRequest, String> {
    if body.is_empty() {
        return Ok(CreateConditionalForwarderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConditionalForwarder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_directory_request(body: &[u8]) -> Result<CreateDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(CreateDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_hybrid_a_d_request(body: &[u8]) -> Result<CreateHybridADRequest, String> {
    if body.is_empty() {
        return Ok(CreateHybridADRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateHybridAD request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_log_subscription_request(
    body: &[u8],
) -> Result<CreateLogSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(CreateLogSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLogSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_microsoft_a_d_request(
    body: &[u8],
) -> Result<CreateMicrosoftADRequest, String> {
    if body.is_empty() {
        return Ok(CreateMicrosoftADRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMicrosoftAD request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_snapshot_request(body: &[u8]) -> Result<CreateSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(CreateSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_trust_request(body: &[u8]) -> Result<CreateTrustRequest, String> {
    if body.is_empty() {
        return Ok(CreateTrustRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTrust request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_a_d_assessment_request(
    body: &[u8],
) -> Result<DeleteADAssessmentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteADAssessmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteADAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_conditional_forwarder_request(
    body: &[u8],
) -> Result<DeleteConditionalForwarderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConditionalForwarderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConditionalForwarder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_directory_request(body: &[u8]) -> Result<DeleteDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_log_subscription_request(
    body: &[u8],
) -> Result<DeleteLogSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLogSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLogSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_snapshot_request(body: &[u8]) -> Result<DeleteSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_trust_request(body: &[u8]) -> Result<DeleteTrustRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTrustRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTrust request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_certificate_request(
    body: &[u8],
) -> Result<DeregisterCertificateRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_event_topic_request(
    body: &[u8],
) -> Result<DeregisterEventTopicRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterEventTopicRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterEventTopic request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_a_d_assessment_request(
    body: &[u8],
) -> Result<DescribeADAssessmentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeADAssessmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeADAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_c_a_enrollment_policy_request(
    body: &[u8],
) -> Result<DescribeCAEnrollmentPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCAEnrollmentPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCAEnrollmentPolicy request: {e}"))
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
pub fn deserialize_describe_client_authentication_settings_request(
    body: &[u8],
) -> Result<DescribeClientAuthenticationSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeClientAuthenticationSettingsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeClientAuthenticationSettings request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_conditional_forwarders_request(
    body: &[u8],
) -> Result<DescribeConditionalForwardersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConditionalForwardersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConditionalForwarders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_directories_request(
    body: &[u8],
) -> Result<DescribeDirectoriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDirectories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_directory_data_access_request(
    body: &[u8],
) -> Result<DescribeDirectoryDataAccessRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectoryDataAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDirectoryDataAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_domain_controllers_request(
    body: &[u8],
) -> Result<DescribeDomainControllersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDomainControllersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDomainControllers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_event_topics_request(
    body: &[u8],
) -> Result<DescribeEventTopicsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventTopicsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventTopics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_hybrid_a_d_update_request(
    body: &[u8],
) -> Result<DescribeHybridADUpdateRequest, String> {
    if body.is_empty() {
        return Ok(DescribeHybridADUpdateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeHybridADUpdate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_l_d_a_p_s_settings_request(
    body: &[u8],
) -> Result<DescribeLDAPSSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLDAPSSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLDAPSSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_regions_request(body: &[u8]) -> Result<DescribeRegionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRegionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRegions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_settings_request(
    body: &[u8],
) -> Result<DescribeSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_shared_directories_request(
    body: &[u8],
) -> Result<DescribeSharedDirectoriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSharedDirectoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSharedDirectories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_snapshots_request(
    body: &[u8],
) -> Result<DescribeSnapshotsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSnapshotsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSnapshots request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusts_request(body: &[u8]) -> Result<DescribeTrustsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTrusts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_update_directory_request(
    body: &[u8],
) -> Result<DescribeUpdateDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUpdateDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUpdateDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_c_a_enrollment_policy_request(
    body: &[u8],
) -> Result<DisableCAEnrollmentPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DisableCAEnrollmentPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableCAEnrollmentPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_client_authentication_request(
    body: &[u8],
) -> Result<DisableClientAuthenticationRequest, String> {
    if body.is_empty() {
        return Ok(DisableClientAuthenticationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableClientAuthentication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_directory_data_access_request(
    body: &[u8],
) -> Result<DisableDirectoryDataAccessRequest, String> {
    if body.is_empty() {
        return Ok(DisableDirectoryDataAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableDirectoryDataAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_l_d_a_p_s_request(body: &[u8]) -> Result<DisableLDAPSRequest, String> {
    if body.is_empty() {
        return Ok(DisableLDAPSRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableLDAPS request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_radius_request(body: &[u8]) -> Result<DisableRadiusRequest, String> {
    if body.is_empty() {
        return Ok(DisableRadiusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableRadius request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_sso_request(body: &[u8]) -> Result<DisableSsoRequest, String> {
    if body.is_empty() {
        return Ok(DisableSsoRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableSso request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_c_a_enrollment_policy_request(
    body: &[u8],
) -> Result<EnableCAEnrollmentPolicyRequest, String> {
    if body.is_empty() {
        return Ok(EnableCAEnrollmentPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableCAEnrollmentPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_client_authentication_request(
    body: &[u8],
) -> Result<EnableClientAuthenticationRequest, String> {
    if body.is_empty() {
        return Ok(EnableClientAuthenticationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableClientAuthentication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_directory_data_access_request(
    body: &[u8],
) -> Result<EnableDirectoryDataAccessRequest, String> {
    if body.is_empty() {
        return Ok(EnableDirectoryDataAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableDirectoryDataAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_l_d_a_p_s_request(body: &[u8]) -> Result<EnableLDAPSRequest, String> {
    if body.is_empty() {
        return Ok(EnableLDAPSRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableLDAPS request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_radius_request(body: &[u8]) -> Result<EnableRadiusRequest, String> {
    if body.is_empty() {
        return Ok(EnableRadiusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableRadius request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_sso_request(body: &[u8]) -> Result<EnableSsoRequest, String> {
    if body.is_empty() {
        return Ok(EnableSsoRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableSso request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_directory_limits_request(
    body: &[u8],
) -> Result<GetDirectoryLimitsRequest, String> {
    if body.is_empty() {
        return Ok(GetDirectoryLimitsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDirectoryLimits request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_snapshot_limits_request(
    body: &[u8],
) -> Result<GetSnapshotLimitsRequest, String> {
    if body.is_empty() {
        return Ok(GetSnapshotLimitsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSnapshotLimits request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_a_d_assessments_request(
    body: &[u8],
) -> Result<ListADAssessmentsRequest, String> {
    if body.is_empty() {
        return Ok(ListADAssessmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListADAssessments request: {e}"))
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
pub fn deserialize_list_ip_routes_request(body: &[u8]) -> Result<ListIpRoutesRequest, String> {
    if body.is_empty() {
        return Ok(ListIpRoutesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIpRoutes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_subscriptions_request(
    body: &[u8],
) -> Result<ListLogSubscriptionsRequest, String> {
    if body.is_empty() {
        return Ok(ListLogSubscriptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogSubscriptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_schema_extensions_request(
    body: &[u8],
) -> Result<ListSchemaExtensionsRequest, String> {
    if body.is_empty() {
        return Ok(ListSchemaExtensionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSchemaExtensions request: {e}"))
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
pub fn deserialize_register_certificate_request(
    body: &[u8],
) -> Result<RegisterCertificateRequest, String> {
    if body.is_empty() {
        return Ok(RegisterCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_event_topic_request(
    body: &[u8],
) -> Result<RegisterEventTopicRequest, String> {
    if body.is_empty() {
        return Ok(RegisterEventTopicRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterEventTopic request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reject_shared_directory_request(
    body: &[u8],
) -> Result<RejectSharedDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(RejectSharedDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RejectSharedDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_ip_routes_request(body: &[u8]) -> Result<RemoveIpRoutesRequest, String> {
    if body.is_empty() {
        return Ok(RemoveIpRoutesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveIpRoutes request: {e}"))
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
pub fn deserialize_remove_tags_from_resource_request(
    body: &[u8],
) -> Result<RemoveTagsFromResourceRequest, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTagsFromResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reset_user_password_request(
    body: &[u8],
) -> Result<ResetUserPasswordRequest, String> {
    if body.is_empty() {
        return Ok(ResetUserPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResetUserPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_from_snapshot_request(
    body: &[u8],
) -> Result<RestoreFromSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(RestoreFromSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreFromSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_share_directory_request(body: &[u8]) -> Result<ShareDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(ShareDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ShareDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_a_d_assessment_request(
    body: &[u8],
) -> Result<StartADAssessmentRequest, String> {
    if body.is_empty() {
        return Ok(StartADAssessmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartADAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_schema_extension_request(
    body: &[u8],
) -> Result<StartSchemaExtensionRequest, String> {
    if body.is_empty() {
        return Ok(StartSchemaExtensionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSchemaExtension request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_unshare_directory_request(
    body: &[u8],
) -> Result<UnshareDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(UnshareDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UnshareDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_conditional_forwarder_request(
    body: &[u8],
) -> Result<UpdateConditionalForwarderRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConditionalForwarderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConditionalForwarder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_directory_setup_request(
    body: &[u8],
) -> Result<UpdateDirectorySetupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDirectorySetupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDirectorySetup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_hybrid_a_d_request(body: &[u8]) -> Result<UpdateHybridADRequest, String> {
    if body.is_empty() {
        return Ok(UpdateHybridADRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateHybridAD request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_number_of_domain_controllers_request(
    body: &[u8],
) -> Result<UpdateNumberOfDomainControllersRequest, String> {
    if body.is_empty() {
        return Ok(UpdateNumberOfDomainControllersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateNumberOfDomainControllers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_radius_request(body: &[u8]) -> Result<UpdateRadiusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRadiusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRadius request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_settings_request(body: &[u8]) -> Result<UpdateSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_trust_request(body: &[u8]) -> Result<UpdateTrustRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTrustRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTrust request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_verify_trust_request(body: &[u8]) -> Result<VerifyTrustRequest, String> {
    if body.is_empty() {
        return Ok(VerifyTrustRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize VerifyTrust request: {e}"))
}
