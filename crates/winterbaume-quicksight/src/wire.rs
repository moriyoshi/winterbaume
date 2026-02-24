//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-quicksight

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
pub fn serialize_batch_create_topic_reviewed_answer_response(
    result: &BatchCreateTopicReviewedAnswerResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_topic_reviewed_answer_response(
    result: &BatchDeleteTopicReviewedAnswerResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_ingestion_response(result: &CancelIngestionResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_account_customization_response(
    result: &CreateAccountCustomizationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_account_subscription_response(
    result: &CreateAccountSubscriptionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_action_connector_response(
    result: &CreateActionConnectorResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_analysis_response(result: &CreateAnalysisResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_brand_response(result: &CreateBrandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_permissions_response(
    result: &CreateCustomPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_dashboard_response(result: &CreateDashboardResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_set_response(result: &CreateDataSetResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_source_response(result: &CreateDataSourceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_folder_response(result: &CreateFolderResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_folder_membership_response(
    result: &CreateFolderMembershipResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_membership_response(
    result: &CreateGroupMembershipResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_i_a_m_policy_assignment_response(
    result: &CreateIAMPolicyAssignmentResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_ingestion_response(result: &CreateIngestionResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_namespace_response(result: &CreateNamespaceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_refresh_schedule_response(
    result: &CreateRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_role_membership_response(
    result: &CreateRoleMembershipResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_template_response(result: &CreateTemplateResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_template_alias_response(
    result: &CreateTemplateAliasResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_theme_response(result: &CreateThemeResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_theme_alias_response(result: &CreateThemeAliasResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_topic_response(result: &CreateTopicResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_topic_refresh_schedule_response(
    result: &CreateTopicRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_v_p_c_connection_response(
    result: &CreateVPCConnectionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_account_custom_permission_response(
    result: &DeleteAccountCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_account_customization_response(
    result: &DeleteAccountCustomizationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_account_subscription_response(
    result: &DeleteAccountSubscriptionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_action_connector_response(
    result: &DeleteActionConnectorResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_analysis_response(result: &DeleteAnalysisResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_brand_response(result: &DeleteBrandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_brand_assignment_response(
    result: &DeleteBrandAssignmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_permissions_response(
    result: &DeleteCustomPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_dashboard_response(result: &DeleteDashboardResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_set_response(result: &DeleteDataSetResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_set_refresh_properties_response(
    result: &DeleteDataSetRefreshPropertiesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_source_response(result: &DeleteDataSourceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_default_q_business_application_response(
    result: &DeleteDefaultQBusinessApplicationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_folder_response(result: &DeleteFolderResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_folder_membership_response(
    result: &DeleteFolderMembershipResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_membership_response(
    result: &DeleteGroupMembershipResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_i_a_m_policy_assignment_response(
    result: &DeleteIAMPolicyAssignmentResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_identity_propagation_config_response(
    result: &DeleteIdentityPropagationConfigResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_namespace_response(result: &DeleteNamespaceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_refresh_schedule_response(
    result: &DeleteRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_role_custom_permission_response(
    result: &DeleteRoleCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_role_membership_response(
    result: &DeleteRoleMembershipResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_template_response(result: &DeleteTemplateResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_template_alias_response(
    result: &DeleteTemplateAliasResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_theme_response(result: &DeleteThemeResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_theme_alias_response(result: &DeleteThemeAliasResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_topic_response(result: &DeleteTopicResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_topic_refresh_schedule_response(
    result: &DeleteTopicRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_response(result: &DeleteUserResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_by_principal_id_response(
    result: &DeleteUserByPrincipalIdResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_custom_permission_response(
    result: &DeleteUserCustomPermissionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_v_p_c_connection_response(
    result: &DeleteVPCConnectionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_custom_permission_response(
    result: &DescribeAccountCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_customization_response(
    result: &DescribeAccountCustomizationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_settings_response(
    result: &DescribeAccountSettingsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_subscription_response(
    result: &DescribeAccountSubscriptionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_action_connector_response(
    result: &DescribeActionConnectorResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_action_connector_permissions_response(
    result: &DescribeActionConnectorPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_analysis_response(result: &DescribeAnalysisResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_analysis_definition_response(
    result: &DescribeAnalysisDefinitionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_analysis_permissions_response(
    result: &DescribeAnalysisPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_asset_bundle_export_job_response(
    result: &DescribeAssetBundleExportJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_asset_bundle_import_job_response(
    result: &DescribeAssetBundleImportJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_automation_job_response(
    result: &DescribeAutomationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_brand_response(result: &DescribeBrandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_brand_assignment_response(
    result: &DescribeBrandAssignmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_brand_published_version_response(
    result: &DescribeBrandPublishedVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_custom_permissions_response(
    result: &DescribeCustomPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboard_response(result: &DescribeDashboardResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboard_definition_response(
    result: &DescribeDashboardDefinitionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboard_permissions_response(
    result: &DescribeDashboardPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboard_snapshot_job_response(
    result: &DescribeDashboardSnapshotJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboard_snapshot_job_result_response(
    result: &DescribeDashboardSnapshotJobResultResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dashboards_q_a_configuration_response(
    result: &DescribeDashboardsQAConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_set_response(result: &DescribeDataSetResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_set_permissions_response(
    result: &DescribeDataSetPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_set_refresh_properties_response(
    result: &DescribeDataSetRefreshPropertiesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_source_response(
    result: &DescribeDataSourceResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_source_permissions_response(
    result: &DescribeDataSourcePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_default_q_business_application_response(
    result: &DescribeDefaultQBusinessApplicationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_folder_response(result: &DescribeFolderResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_folder_permissions_response(
    result: &DescribeFolderPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_folder_resolved_permissions_response(
    result: &DescribeFolderResolvedPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_group_response(result: &DescribeGroupResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_group_membership_response(
    result: &DescribeGroupMembershipResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_i_a_m_policy_assignment_response(
    result: &DescribeIAMPolicyAssignmentResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_ingestion_response(result: &DescribeIngestionResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_ip_restriction_response(
    result: &DescribeIpRestrictionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_key_registration_response(
    result: &DescribeKeyRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_namespace_response(result: &DescribeNamespaceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_q_personalization_configuration_response(
    result: &DescribeQPersonalizationConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_quick_sight_q_search_configuration_response(
    result: &DescribeQuickSightQSearchConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_refresh_schedule_response(
    result: &DescribeRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_role_custom_permission_response(
    result: &DescribeRoleCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_self_upgrade_configuration_response(
    result: &DescribeSelfUpgradeConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_template_response(result: &DescribeTemplateResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_template_alias_response(
    result: &DescribeTemplateAliasResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_template_definition_response(
    result: &DescribeTemplateDefinitionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_template_permissions_response(
    result: &DescribeTemplatePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_theme_response(result: &DescribeThemeResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_theme_alias_response(
    result: &DescribeThemeAliasResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_theme_permissions_response(
    result: &DescribeThemePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_response(result: &DescribeTopicResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_permissions_response(
    result: &DescribeTopicPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_refresh_response(
    result: &DescribeTopicRefreshResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_topic_refresh_schedule_response(
    result: &DescribeTopicRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_user_response(result: &DescribeUserResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_v_p_c_connection_response(
    result: &DescribeVPCConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_embed_url_for_anonymous_user_response(
    result: &GenerateEmbedUrlForAnonymousUserResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_embed_url_for_registered_user_response(
    result: &GenerateEmbedUrlForRegisteredUserResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_embed_url_for_registered_user_with_identity_response(
    result: &GenerateEmbedUrlForRegisteredUserWithIdentityResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_dashboard_embed_url_response(
    result: &GetDashboardEmbedUrlResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_metadata_response(result: &GetFlowMetadataOutput) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_permissions_response(result: &GetFlowPermissionsOutput) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_identity_context_response(
    result: &GetIdentityContextResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_session_embed_url_response(
    result: &GetSessionEmbedUrlResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_action_connectors_response(
    result: &ListActionConnectorsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_analyses_response(result: &ListAnalysesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_asset_bundle_export_jobs_response(
    result: &ListAssetBundleExportJobsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_asset_bundle_import_jobs_response(
    result: &ListAssetBundleImportJobsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_brands_response(result: &ListBrandsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_permissions_response(
    result: &ListCustomPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_dashboard_versions_response(
    result: &ListDashboardVersionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_dashboards_response(result: &ListDashboardsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_sets_response(result: &ListDataSetsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_sources_response(result: &ListDataSourcesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flows_response(result: &ListFlowsOutput) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_folder_members_response(result: &ListFolderMembersResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_folders_response(result: &ListFoldersResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_folders_for_resource_response(
    result: &ListFoldersForResourceResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_group_memberships_response(
    result: &ListGroupMembershipsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_i_a_m_policy_assignments_response(
    result: &ListIAMPolicyAssignmentsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_i_a_m_policy_assignments_for_user_response(
    result: &ListIAMPolicyAssignmentsForUserResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_identity_propagation_configs_response(
    result: &ListIdentityPropagationConfigsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_ingestions_response(result: &ListIngestionsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_namespaces_response(result: &ListNamespacesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_refresh_schedules_response(
    result: &ListRefreshSchedulesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_role_memberships_response(
    result: &ListRoleMembershipsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_self_upgrades_response(result: &ListSelfUpgradesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_template_aliases_response(
    result: &ListTemplateAliasesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_template_versions_response(
    result: &ListTemplateVersionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_templates_response(result: &ListTemplatesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_theme_aliases_response(result: &ListThemeAliasesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_theme_versions_response(result: &ListThemeVersionsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_themes_response(result: &ListThemesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_topic_refresh_schedules_response(
    result: &ListTopicRefreshSchedulesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_topic_reviewed_answers_response(
    result: &ListTopicReviewedAnswersResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_topics_response(result: &ListTopicsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_groups_response(result: &ListUserGroupsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_v_p_c_connections_response(
    result: &ListVPCConnectionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_predict_q_a_results_response(result: &PredictQAResultsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_data_set_refresh_properties_response(
    result: &PutDataSetRefreshPropertiesResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_user_response(result: &RegisterUserResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_restore_analysis_response(result: &RestoreAnalysisResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_action_connectors_response(
    result: &SearchActionConnectorsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_analyses_response(result: &SearchAnalysesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_dashboards_response(result: &SearchDashboardsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_data_sets_response(result: &SearchDataSetsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_data_sources_response(result: &SearchDataSourcesResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_flows_response(result: &SearchFlowsOutput) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_folders_response(result: &SearchFoldersResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_groups_response(result: &SearchGroupsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_topics_response(result: &SearchTopicsResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_asset_bundle_export_job_response(
    result: &StartAssetBundleExportJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_asset_bundle_import_job_response(
    result: &StartAssetBundleImportJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_automation_job_response(
    result: &StartAutomationJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_dashboard_snapshot_job_response(
    result: &StartDashboardSnapshotJobResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_dashboard_snapshot_job_schedule_response(
    result: &StartDashboardSnapshotJobScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_custom_permission_response(
    result: &UpdateAccountCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_customization_response(
    result: &UpdateAccountCustomizationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_settings_response(
    result: &UpdateAccountSettingsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_action_connector_response(
    result: &UpdateActionConnectorResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_action_connector_permissions_response(
    result: &UpdateActionConnectorPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_analysis_response(result: &UpdateAnalysisResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_analysis_permissions_response(
    result: &UpdateAnalysisPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_application_with_token_exchange_grant_response(
    result: &UpdateApplicationWithTokenExchangeGrantResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_brand_response(result: &UpdateBrandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_brand_assignment_response(
    result: &UpdateBrandAssignmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_brand_published_version_response(
    result: &UpdateBrandPublishedVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_custom_permissions_response(
    result: &UpdateCustomPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dashboard_response(result: &UpdateDashboardResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dashboard_links_response(
    result: &UpdateDashboardLinksResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dashboard_permissions_response(
    result: &UpdateDashboardPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dashboard_published_version_response(
    result: &UpdateDashboardPublishedVersionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dashboards_q_a_configuration_response(
    result: &UpdateDashboardsQAConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_set_response(result: &UpdateDataSetResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_set_permissions_response(
    result: &UpdateDataSetPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_source_response(result: &UpdateDataSourceResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_source_permissions_response(
    result: &UpdateDataSourcePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_default_q_business_application_response(
    result: &UpdateDefaultQBusinessApplicationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_permissions_response(
    result: &UpdateFlowPermissionsOutput,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_folder_response(result: &UpdateFolderResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_folder_permissions_response(
    result: &UpdateFolderPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_i_a_m_policy_assignment_response(
    result: &UpdateIAMPolicyAssignmentResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_identity_propagation_config_response(
    result: &UpdateIdentityPropagationConfigResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_ip_restriction_response(
    result: &UpdateIpRestrictionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_key_registration_response(
    result: &UpdateKeyRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_public_sharing_settings_response(
    result: &UpdatePublicSharingSettingsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_q_personalization_configuration_response(
    result: &UpdateQPersonalizationConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_quick_sight_q_search_configuration_response(
    result: &UpdateQuickSightQSearchConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_refresh_schedule_response(
    result: &UpdateRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_role_custom_permission_response(
    result: &UpdateRoleCustomPermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_s_p_i_c_e_capacity_configuration_response(
    result: &UpdateSPICECapacityConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_self_upgrade_response(result: &UpdateSelfUpgradeResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_self_upgrade_configuration_response(
    result: &UpdateSelfUpgradeConfigurationResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_template_response(result: &UpdateTemplateResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_template_alias_response(
    result: &UpdateTemplateAliasResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_template_permissions_response(
    result: &UpdateTemplatePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_theme_response(result: &UpdateThemeResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_theme_alias_response(result: &UpdateThemeAliasResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_theme_permissions_response(
    result: &UpdateThemePermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_topic_response(result: &UpdateTopicResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_topic_permissions_response(
    result: &UpdateTopicPermissionsResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_topic_refresh_schedule_response(
    result: &UpdateTopicRefreshScheduleResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_response(result: &UpdateUserResponse) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_custom_permission_response(
    result: &UpdateUserCustomPermissionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_v_p_c_connection_response(
    result: &UpdateVPCConnectionResponse,
) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_create_topic_reviewed_answer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchCreateTopicReviewedAnswerRequest, String> {
    let mut input = BatchCreateTopicReviewedAnswerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchCreateTopicReviewedAnswerRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchCreateTopicReviewedAnswer request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_topic_reviewed_answer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteTopicReviewedAnswerRequest, String> {
    let mut input = BatchDeleteTopicReviewedAnswerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteTopicReviewedAnswerRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDeleteTopicReviewedAnswer request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_ingestion_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelIngestionRequest, String> {
    let mut input = CancelIngestionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            "IngestionId" => {
                input.ingestion_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_account_customization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccountCustomizationRequest, String> {
    let mut input = CreateAccountCustomizationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccountCustomizationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateAccountCustomization request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_account_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccountSubscriptionRequest, String> {
    let mut input = CreateAccountSubscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccountSubscriptionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAccountSubscription request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_action_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateActionConnectorRequest, String> {
    let mut input = CreateActionConnectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateActionConnectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateActionConnector request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAnalysisRequest, String> {
    let mut input = CreateAnalysisRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAnalysisRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAnalysis request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_brand_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBrandRequest, String> {
    let mut input = CreateBrandRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBrandRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBrand request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomPermissionsRequest, String> {
    let mut input = CreateCustomPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCustomPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateCustomPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_dashboard_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDashboardRequest, String> {
    let mut input = CreateDashboardRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDashboardRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDashboard request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataSetRequest, String> {
    let mut input = CreateDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataSourceRequest, String> {
    let mut input = CreateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFolderRequest, String> {
    let mut input = CreateFolderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFolderRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFolder request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_folder_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFolderMembershipRequest, String> {
    let mut input = CreateFolderMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            "MemberId" => {
                input.member_id = value.to_string();
            }
            "MemberType" => {
                input.member_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupRequest, String> {
    let mut input = CreateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupMembershipRequest, String> {
    let mut input = CreateGroupMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "MemberName" => {
                input.member_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_i_a_m_policy_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIAMPolicyAssignmentRequest, String> {
    let mut input = CreateIAMPolicyAssignmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIAMPolicyAssignmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateIAMPolicyAssignment request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_ingestion_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIngestionRequest, String> {
    let mut input = CreateIngestionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIngestionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIngestion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            "IngestionId" => {
                input.ingestion_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNamespaceRequest, String> {
    let mut input = CreateNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNamespaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNamespace request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRefreshScheduleRequest, String> {
    let mut input = CreateRefreshScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRefreshScheduleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRefreshSchedule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_role_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRoleMembershipRequest, String> {
    let mut input = CreateRoleMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "MemberName" => {
                input.member_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTemplateRequest, String> {
    let mut input = CreateTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_template_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTemplateAliasRequest, String> {
    let mut input = CreateTemplateAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTemplateAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTemplateAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_theme_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThemeRequest, String> {
    let mut input = CreateThemeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThemeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTheme request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_theme_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThemeAliasRequest, String> {
    let mut input = CreateThemeAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThemeAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThemeAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicRequest, String> {
    let mut input = CreateTopicRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTopicRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTopic request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_topic_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicRefreshScheduleRequest, String> {
    let mut input = CreateTopicRefreshScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTopicRefreshScheduleRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTopicRefreshSchedule request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_v_p_c_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVPCConnectionRequest, String> {
    let mut input = CreateVPCConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVPCConnectionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVPCConnection request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_account_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccountCustomPermissionRequest, String> {
    let mut input = DeleteAccountCustomPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_account_customization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccountCustomizationRequest, String> {
    let mut input = DeleteAccountCustomizationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_account_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccountSubscriptionRequest, String> {
    let mut input = DeleteAccountSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_action_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteActionConnectorRequest, String> {
    let mut input = DeleteActionConnectorRequest::default();
    for (name, value) in labels {
        match *name {
            "ActionConnectorId" => {
                input.action_connector_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAnalysisRequest, String> {
    let mut input = DeleteAnalysisRequest::default();
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force-delete-without-recovery") {
        input.force_delete_without_recovery = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("recovery-window-in-days") {
        input.recovery_window_in_days = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_brand_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBrandRequest, String> {
    let mut input = DeleteBrandRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_brand_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBrandAssignmentRequest, String> {
    let mut input = DeleteBrandAssignmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomPermissionsRequest, String> {
    let mut input = DeleteCustomPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "CustomPermissionsName" => {
                input.custom_permissions_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_dashboard_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDashboardRequest, String> {
    let mut input = DeleteDashboardRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSetRequest, String> {
    let mut input = DeleteDataSetRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_set_refresh_properties_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSetRefreshPropertiesRequest, String> {
    let mut input = DeleteDataSetRefreshPropertiesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSourceRequest, String> {
    let mut input = DeleteDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSourceId" => {
                input.data_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_default_q_business_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDefaultQBusinessApplicationRequest, String> {
    let mut input = DeleteDefaultQBusinessApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFolderRequest, String> {
    let mut input = DeleteFolderRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_folder_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFolderMembershipRequest, String> {
    let mut input = DeleteFolderMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            "MemberId" => {
                input.member_id = value.to_string();
            }
            "MemberType" => {
                input.member_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupRequest, String> {
    let mut input = DeleteGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupMembershipRequest, String> {
    let mut input = DeleteGroupMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "MemberName" => {
                input.member_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_i_a_m_policy_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIAMPolicyAssignmentRequest, String> {
    let mut input = DeleteIAMPolicyAssignmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AssignmentName" => {
                input.assignment_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_identity_propagation_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIdentityPropagationConfigRequest, String> {
    let mut input = DeleteIdentityPropagationConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Service" => {
                input.service = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNamespaceRequest, String> {
    let mut input = DeleteNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRefreshScheduleRequest, String> {
    let mut input = DeleteRefreshScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            "ScheduleId" => {
                input.schedule_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_role_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoleCustomPermissionRequest, String> {
    let mut input = DeleteRoleCustomPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_role_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoleMembershipRequest, String> {
    let mut input = DeleteRoleMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "MemberName" => {
                input.member_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTemplateRequest, String> {
    let mut input = DeleteTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_template_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTemplateAliasRequest, String> {
    let mut input = DeleteTemplateAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_theme_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThemeRequest, String> {
    let mut input = DeleteThemeRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_theme_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThemeAliasRequest, String> {
    let mut input = DeleteThemeAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicRequest, String> {
    let mut input = DeleteTopicRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_topic_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicRefreshScheduleRequest, String> {
    let mut input = DeleteTopicRefreshScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DatasetId" => {
                input.dataset_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserRequest, String> {
    let mut input = DeleteUserRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_by_principal_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserByPrincipalIdRequest, String> {
    let mut input = DeleteUserByPrincipalIdRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "PrincipalId" => {
                input.principal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserCustomPermissionRequest, String> {
    let mut input = DeleteUserCustomPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_v_p_c_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVPCConnectionRequest, String> {
    let mut input = DeleteVPCConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "VPCConnectionId" => {
                input.v_p_c_connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountCustomPermissionRequest, String> {
    let mut input = DescribeAccountCustomPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_customization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountCustomizationRequest, String> {
    let mut input = DescribeAccountCustomizationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("resolved") {
        input.resolved = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountSettingsRequest, String> {
    let mut input = DescribeAccountSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountSubscriptionRequest, String> {
    let mut input = DescribeAccountSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_action_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeActionConnectorRequest, String> {
    let mut input = DescribeActionConnectorRequest::default();
    for (name, value) in labels {
        match *name {
            "ActionConnectorId" => {
                input.action_connector_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_action_connector_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeActionConnectorPermissionsRequest, String> {
    let mut input = DescribeActionConnectorPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ActionConnectorId" => {
                input.action_connector_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAnalysisRequest, String> {
    let mut input = DescribeAnalysisRequest::default();
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_analysis_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAnalysisDefinitionRequest, String> {
    let mut input = DescribeAnalysisDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_analysis_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAnalysisPermissionsRequest, String> {
    let mut input = DescribeAnalysisPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_asset_bundle_export_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAssetBundleExportJobRequest, String> {
    let mut input = DescribeAssetBundleExportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "AssetBundleExportJobId" => {
                input.asset_bundle_export_job_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_asset_bundle_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAssetBundleImportJobRequest, String> {
    let mut input = DescribeAssetBundleImportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "AssetBundleImportJobId" => {
                input.asset_bundle_import_job_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_automation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAutomationJobRequest, String> {
    let mut input = DescribeAutomationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "AutomationGroupId" => {
                input.automation_group_id = value.to_string();
            }
            "AutomationId" => {
                input.automation_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeInputPayload") {
        input.include_input_payload = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("includeOutputPayload") {
        input.include_output_payload = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_brand_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrandRequest, String> {
    let mut input = DescribeBrandRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_brand_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrandAssignmentRequest, String> {
    let mut input = DescribeBrandAssignmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_brand_published_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrandPublishedVersionRequest, String> {
    let mut input = DescribeBrandPublishedVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_custom_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCustomPermissionsRequest, String> {
    let mut input = DescribeCustomPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "CustomPermissionsName" => {
                input.custom_permissions_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboard_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardRequest, String> {
    let mut input = DescribeDashboardRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias-name") {
        input.alias_name = Some(value.to_string());
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboard_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardDefinitionRequest, String> {
    let mut input = DescribeDashboardDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias-name") {
        input.alias_name = Some(value.to_string());
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboard_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardPermissionsRequest, String> {
    let mut input = DescribeDashboardPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboard_snapshot_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardSnapshotJobRequest, String> {
    let mut input = DescribeDashboardSnapshotJobRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            "SnapshotJobId" => {
                input.snapshot_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboard_snapshot_job_result_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardSnapshotJobResultRequest, String> {
    let mut input = DescribeDashboardSnapshotJobResultRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            "SnapshotJobId" => {
                input.snapshot_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dashboards_q_a_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDashboardsQAConfigurationRequest, String> {
    let mut input = DescribeDashboardsQAConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSetRequest, String> {
    let mut input = DescribeDataSetRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_set_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSetPermissionsRequest, String> {
    let mut input = DescribeDataSetPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_set_refresh_properties_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSetRefreshPropertiesRequest, String> {
    let mut input = DescribeDataSetRefreshPropertiesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSourceRequest, String> {
    let mut input = DescribeDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSourceId" => {
                input.data_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_source_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSourcePermissionsRequest, String> {
    let mut input = DescribeDataSourcePermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSourceId" => {
                input.data_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_default_q_business_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDefaultQBusinessApplicationRequest, String> {
    let mut input = DescribeDefaultQBusinessApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFolderRequest, String> {
    let mut input = DescribeFolderRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_folder_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFolderPermissionsRequest, String> {
    let mut input = DescribeFolderPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_folder_resolved_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFolderResolvedPermissionsRequest, String> {
    let mut input = DescribeFolderResolvedPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGroupRequest, String> {
    let mut input = DescribeGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_group_membership_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGroupMembershipRequest, String> {
    let mut input = DescribeGroupMembershipRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "MemberName" => {
                input.member_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_i_a_m_policy_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIAMPolicyAssignmentRequest, String> {
    let mut input = DescribeIAMPolicyAssignmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AssignmentName" => {
                input.assignment_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_ingestion_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIngestionRequest, String> {
    let mut input = DescribeIngestionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            "IngestionId" => {
                input.ingestion_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_ip_restriction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIpRestrictionRequest, String> {
    let mut input = DescribeIpRestrictionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_key_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeKeyRegistrationRequest, String> {
    let mut input = DescribeKeyRegistrationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("default-key-only") {
        input.default_key_only = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNamespaceRequest, String> {
    let mut input = DescribeNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_q_personalization_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQPersonalizationConfigurationRequest, String> {
    let mut input = DescribeQPersonalizationConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_quick_sight_q_search_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQuickSightQSearchConfigurationRequest, String> {
    let mut input = DescribeQuickSightQSearchConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRefreshScheduleRequest, String> {
    let mut input = DescribeRefreshScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            "ScheduleId" => {
                input.schedule_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_role_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRoleCustomPermissionRequest, String> {
    let mut input = DescribeRoleCustomPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_self_upgrade_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSelfUpgradeConfigurationRequest, String> {
    let mut input = DescribeSelfUpgradeConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTemplateRequest, String> {
    let mut input = DescribeTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias-name") {
        input.alias_name = Some(value.to_string());
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_template_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTemplateAliasRequest, String> {
    let mut input = DescribeTemplateAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_template_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTemplateDefinitionRequest, String> {
    let mut input = DescribeTemplateDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias-name") {
        input.alias_name = Some(value.to_string());
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_template_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTemplatePermissionsRequest, String> {
    let mut input = DescribeTemplatePermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_theme_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThemeRequest, String> {
    let mut input = DescribeThemeRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias-name") {
        input.alias_name = Some(value.to_string());
    }
    if let Some(value) = query.get("version-number") {
        input.version_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_theme_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThemeAliasRequest, String> {
    let mut input = DescribeThemeAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_theme_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThemePermissionsRequest, String> {
    let mut input = DescribeThemePermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicRequest, String> {
    let mut input = DescribeTopicRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicPermissionsRequest, String> {
    let mut input = DescribeTopicPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_refresh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicRefreshRequest, String> {
    let mut input = DescribeTopicRefreshRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "RefreshId" => {
                input.refresh_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_topic_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTopicRefreshScheduleRequest, String> {
    let mut input = DescribeTopicRefreshScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DatasetId" => {
                input.dataset_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeUserRequest, String> {
    let mut input = DescribeUserRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_v_p_c_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVPCConnectionRequest, String> {
    let mut input = DescribeVPCConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "VPCConnectionId" => {
                input.v_p_c_connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_embed_url_for_anonymous_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateEmbedUrlForAnonymousUserRequest, String> {
    let mut input = GenerateEmbedUrlForAnonymousUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateEmbedUrlForAnonymousUserRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GenerateEmbedUrlForAnonymousUser request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_embed_url_for_registered_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateEmbedUrlForRegisteredUserRequest, String> {
    let mut input = GenerateEmbedUrlForRegisteredUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateEmbedUrlForRegisteredUserRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize GenerateEmbedUrlForRegisteredUser request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_embed_url_for_registered_user_with_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateEmbedUrlForRegisteredUserWithIdentityRequest, String> {
    let mut input = GenerateEmbedUrlForRegisteredUserWithIdentityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateEmbedUrlForRegisteredUserWithIdentityRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize GenerateEmbedUrlForRegisteredUserWithIdentity request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_dashboard_embed_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDashboardEmbedUrlRequest, String> {
    let mut input = GetDashboardEmbedUrlRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("additional-dashboard-ids") {
        input.additional_dashboard_ids = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("creds-type") {
        input.identity_type = value.to_string();
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("reset-disabled") {
        input.reset_disabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("session-lifetime") {
        input.session_lifetime_in_minutes = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = query.get("state-persistence-enabled") {
        input.state_persistence_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("undo-redo-disabled") {
        input.undo_redo_disabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("user-arn") {
        input.user_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowMetadataInput, String> {
    let mut input = GetFlowMetadataInput::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FlowId" => {
                input.flow_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowPermissionsInput, String> {
    let mut input = GetFlowPermissionsInput::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FlowId" => {
                input.flow_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_identity_context_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityContextRequest, String> {
    let mut input = GetIdentityContextRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetIdentityContextRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetIdentityContext request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_session_embed_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSessionEmbedUrlRequest, String> {
    let mut input = GetSessionEmbedUrlRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("entry-point") {
        input.entry_point = Some(value.to_string());
    }
    if let Some(value) = query.get("session-lifetime") {
        input.session_lifetime_in_minutes = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = query.get("user-arn") {
        input.user_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_action_connectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListActionConnectorsRequest, String> {
    let mut input = ListActionConnectorsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_analyses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnalysesRequest, String> {
    let mut input = ListAnalysesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_asset_bundle_export_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssetBundleExportJobsRequest, String> {
    let mut input = ListAssetBundleExportJobsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_asset_bundle_import_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssetBundleImportJobsRequest, String> {
    let mut input = ListAssetBundleImportJobsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_brands_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBrandsRequest, String> {
    let mut input = ListBrandsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomPermissionsRequest, String> {
    let mut input = ListCustomPermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_dashboard_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDashboardVersionsRequest, String> {
    let mut input = ListDashboardVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_dashboards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDashboardsRequest, String> {
    let mut input = ListDashboardsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataSetsRequest, String> {
    let mut input = ListDataSetsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataSourcesRequest, String> {
    let mut input = ListDataSourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowsInput, String> {
    let mut input = ListFlowsInput::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_folder_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFolderMembersRequest, String> {
    let mut input = ListFolderMembersRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_folders_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFoldersRequest, String> {
    let mut input = ListFoldersRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_folders_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFoldersForResourceRequest, String> {
    let mut input = ListFoldersForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_group_memberships_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupMembershipsRequest, String> {
    let mut input = ListGroupMembershipsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsRequest, String> {
    let mut input = ListGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_i_a_m_policy_assignments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIAMPolicyAssignmentsRequest, String> {
    let mut input = ListIAMPolicyAssignmentsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("assignment-status") {
        input.assignment_status = Some(value.to_string());
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_i_a_m_policy_assignments_for_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIAMPolicyAssignmentsForUserRequest, String> {
    let mut input = ListIAMPolicyAssignmentsForUserRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_identity_propagation_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIdentityPropagationConfigsRequest, String> {
    let mut input = ListIdentityPropagationConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_ingestions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIngestionsRequest, String> {
    let mut input = ListIngestionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_namespaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNamespacesRequest, String> {
    let mut input = ListNamespacesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_refresh_schedules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRefreshSchedulesRequest, String> {
    let mut input = ListRefreshSchedulesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_role_memberships_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoleMembershipsRequest, String> {
    let mut input = ListRoleMembershipsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_self_upgrades_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSelfUpgradesRequest, String> {
    let mut input = ListSelfUpgradesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_template_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTemplateAliasesRequest, String> {
    let mut input = ListTemplateAliasesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-result") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_template_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTemplateVersionsRequest, String> {
    let mut input = ListTemplateVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTemplatesRequest, String> {
    let mut input = ListTemplatesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-result") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_theme_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThemeAliasesRequest, String> {
    let mut input = ListThemeAliasesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-result") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_theme_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThemeVersionsRequest, String> {
    let mut input = ListThemeVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_themes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThemesRequest, String> {
    let mut input = ListThemesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_topic_refresh_schedules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicRefreshSchedulesRequest, String> {
    let mut input = ListTopicRefreshSchedulesRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_topic_reviewed_answers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicReviewedAnswersRequest, String> {
    let mut input = ListTopicReviewedAnswersRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_topics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicsRequest, String> {
    let mut input = ListTopicsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_user_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserGroupsRequest, String> {
    let mut input = ListUserGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_users_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUsersRequest, String> {
    let mut input = ListUsersRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_v_p_c_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVPCConnectionsRequest, String> {
    let mut input = ListVPCConnectionsRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_predict_q_a_results_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PredictQAResultsRequest, String> {
    let mut input = PredictQAResultsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PredictQAResultsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PredictQAResults request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_data_set_refresh_properties_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDataSetRefreshPropertiesRequest, String> {
    let mut input = PutDataSetRefreshPropertiesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDataSetRefreshPropertiesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDataSetRefreshProperties request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterUserRequest, String> {
    let mut input = RegisterUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterUserRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterUser request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_restore_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestoreAnalysisRequest, String> {
    let mut input = RestoreAnalysisRequest::default();
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("restore-to-folders") {
        input.restore_to_folders = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_action_connectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchActionConnectorsRequest, String> {
    let mut input = SearchActionConnectorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchActionConnectorsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchActionConnectors request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_analyses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchAnalysesRequest, String> {
    let mut input = SearchAnalysesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchAnalysesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchAnalyses request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_dashboards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchDashboardsRequest, String> {
    let mut input = SearchDashboardsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchDashboardsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchDashboards request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_data_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchDataSetsRequest, String> {
    let mut input = SearchDataSetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchDataSetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchDataSets request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchDataSourcesRequest, String> {
    let mut input = SearchDataSourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchDataSourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchDataSources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchFlowsInput, String> {
    let mut input = SearchFlowsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchFlowsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchFlows request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_folders_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchFoldersRequest, String> {
    let mut input = SearchFoldersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchFoldersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchFolders request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchGroupsRequest, String> {
    let mut input = SearchGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchGroupsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchGroups request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_topics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchTopicsRequest, String> {
    let mut input = SearchTopicsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchTopicsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchTopics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_asset_bundle_export_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAssetBundleExportJobRequest, String> {
    let mut input = StartAssetBundleExportJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAssetBundleExportJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartAssetBundleExportJob request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_asset_bundle_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAssetBundleImportJobRequest, String> {
    let mut input = StartAssetBundleImportJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAssetBundleImportJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartAssetBundleImportJob request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_automation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAutomationJobRequest, String> {
    let mut input = StartAutomationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAutomationJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartAutomationJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AutomationGroupId" => {
                input.automation_group_id = value.to_string();
            }
            "AutomationId" => {
                input.automation_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_dashboard_snapshot_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDashboardSnapshotJobRequest, String> {
    let mut input = StartDashboardSnapshotJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDashboardSnapshotJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartDashboardSnapshotJob request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_dashboard_snapshot_job_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDashboardSnapshotJobScheduleRequest, String> {
    let mut input = StartDashboardSnapshotJobScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            "ScheduleId" => {
                input.schedule_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("keys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountCustomPermissionRequest, String> {
    let mut input = UpdateAccountCustomPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountCustomPermissionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAccountCustomPermission request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_customization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountCustomizationRequest, String> {
    let mut input = UpdateAccountCustomizationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountCustomizationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAccountCustomization request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountSettingsRequest, String> {
    let mut input = UpdateAccountSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAccountSettings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_action_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateActionConnectorRequest, String> {
    let mut input = UpdateActionConnectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateActionConnectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateActionConnector request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ActionConnectorId" => {
                input.action_connector_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_action_connector_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateActionConnectorPermissionsRequest, String> {
    let mut input = UpdateActionConnectorPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateActionConnectorPermissionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateActionConnectorPermissions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ActionConnectorId" => {
                input.action_connector_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAnalysisRequest, String> {
    let mut input = UpdateAnalysisRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAnalysisRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAnalysis request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_analysis_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAnalysisPermissionsRequest, String> {
    let mut input = UpdateAnalysisPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAnalysisPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAnalysisPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AnalysisId" => {
                input.analysis_id = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_application_with_token_exchange_grant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationWithTokenExchangeGrantRequest, String> {
    let mut input = UpdateApplicationWithTokenExchangeGrantRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_brand_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrandRequest, String> {
    let mut input = UpdateBrandRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrandRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrand request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_brand_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrandAssignmentRequest, String> {
    let mut input = UpdateBrandAssignmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrandAssignmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrandAssignment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_brand_published_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrandPublishedVersionRequest, String> {
    let mut input = UpdateBrandPublishedVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrandPublishedVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateBrandPublishedVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "BrandId" => {
                input.brand_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_custom_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCustomPermissionsRequest, String> {
    let mut input = UpdateCustomPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCustomPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateCustomPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "CustomPermissionsName" => {
                input.custom_permissions_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dashboard_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDashboardRequest, String> {
    let mut input = UpdateDashboardRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDashboardRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDashboard request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dashboard_links_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDashboardLinksRequest, String> {
    let mut input = UpdateDashboardLinksRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDashboardLinksRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDashboardLinks request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dashboard_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDashboardPermissionsRequest, String> {
    let mut input = UpdateDashboardPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDashboardPermissionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDashboardPermissions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dashboard_published_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDashboardPublishedVersionRequest, String> {
    let mut input = UpdateDashboardPublishedVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DashboardId" => {
                input.dashboard_id = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dashboards_q_a_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDashboardsQAConfigurationRequest, String> {
    let mut input = UpdateDashboardsQAConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDashboardsQAConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDashboardsQAConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSetRequest, String> {
    let mut input = UpdateDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_set_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSetPermissionsRequest, String> {
    let mut input = UpdateDataSetPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSetPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDataSetPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSourceRequest, String> {
    let mut input = UpdateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSourceId" => {
                input.data_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_source_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSourcePermissionsRequest, String> {
    let mut input = UpdateDataSourcePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSourcePermissionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDataSourcePermissions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSourceId" => {
                input.data_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_default_q_business_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDefaultQBusinessApplicationRequest, String> {
    let mut input = UpdateDefaultQBusinessApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDefaultQBusinessApplicationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize UpdateDefaultQBusinessApplication request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowPermissionsInput, String> {
    let mut input = UpdateFlowPermissionsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowPermissionsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowPermissions request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FlowId" => {
                input.flow_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFolderRequest, String> {
    let mut input = UpdateFolderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFolderRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFolder request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_folder_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFolderPermissionsRequest, String> {
    let mut input = UpdateFolderPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFolderPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateFolderPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "FolderId" => {
                input.folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupRequest, String> {
    let mut input = UpdateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "GroupName" => {
                input.group_name = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_i_a_m_policy_assignment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIAMPolicyAssignmentRequest, String> {
    let mut input = UpdateIAMPolicyAssignmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIAMPolicyAssignmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateIAMPolicyAssignment request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AssignmentName" => {
                input.assignment_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_identity_propagation_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIdentityPropagationConfigRequest, String> {
    let mut input = UpdateIdentityPropagationConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIdentityPropagationConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateIdentityPropagationConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Service" => {
                input.service = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_ip_restriction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIpRestrictionRequest, String> {
    let mut input = UpdateIpRestrictionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIpRestrictionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIpRestriction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_key_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateKeyRegistrationRequest, String> {
    let mut input = UpdateKeyRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateKeyRegistrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateKeyRegistration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_public_sharing_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePublicSharingSettingsRequest, String> {
    let mut input = UpdatePublicSharingSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePublicSharingSettingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePublicSharingSettings request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_q_personalization_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQPersonalizationConfigurationRequest, String> {
    let mut input = UpdateQPersonalizationConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQPersonalizationConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateQPersonalizationConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_quick_sight_q_search_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQuickSightQSearchConfigurationRequest, String> {
    let mut input = UpdateQuickSightQSearchConfigurationRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<UpdateQuickSightQSearchConfigurationRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize UpdateQuickSightQSearchConfiguration request: {err}"
                    )
                })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRefreshScheduleRequest, String> {
    let mut input = UpdateRefreshScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRefreshScheduleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRefreshSchedule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DataSetId" => {
                input.data_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_role_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoleCustomPermissionRequest, String> {
    let mut input = UpdateRoleCustomPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoleCustomPermissionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRoleCustomPermission request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "Role" => {
                input.role = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_s_p_i_c_e_capacity_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSPICECapacityConfigurationRequest, String> {
    let mut input = UpdateSPICECapacityConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSPICECapacityConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSPICECapacityConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_self_upgrade_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSelfUpgradeRequest, String> {
    let mut input = UpdateSelfUpgradeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSelfUpgradeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSelfUpgrade request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_self_upgrade_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSelfUpgradeConfigurationRequest, String> {
    let mut input = UpdateSelfUpgradeConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSelfUpgradeConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSelfUpgradeConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTemplateRequest, String> {
    let mut input = UpdateTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_template_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTemplateAliasRequest, String> {
    let mut input = UpdateTemplateAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTemplateAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTemplateAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_template_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTemplatePermissionsRequest, String> {
    let mut input = UpdateTemplatePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTemplatePermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateTemplatePermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TemplateId" => {
                input.template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_theme_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThemeRequest, String> {
    let mut input = UpdateThemeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThemeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTheme request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_theme_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThemeAliasRequest, String> {
    let mut input = UpdateThemeAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThemeAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThemeAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AliasName" => {
                input.alias_name = value.to_string();
            }
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_theme_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThemePermissionsRequest, String> {
    let mut input = UpdateThemePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThemePermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateThemePermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "ThemeId" => {
                input.theme_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_topic_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTopicRequest, String> {
    let mut input = UpdateTopicRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTopicRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTopic request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_topic_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTopicPermissionsRequest, String> {
    let mut input = UpdateTopicPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTopicPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateTopicPermissions request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_topic_refresh_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTopicRefreshScheduleRequest, String> {
    let mut input = UpdateTopicRefreshScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTopicRefreshScheduleRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateTopicRefreshSchedule request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "DatasetId" => {
                input.dataset_id = value.to_string();
            }
            "TopicId" => {
                input.topic_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserRequest, String> {
    let mut input = UpdateUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUser request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_custom_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserCustomPermissionRequest, String> {
    let mut input = UpdateUserCustomPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserCustomPermissionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserCustomPermission request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "Namespace" => {
                input.namespace = value.to_string();
            }
            "UserName" => {
                input.user_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_v_p_c_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVPCConnectionRequest, String> {
    let mut input = UpdateVPCConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVPCConnectionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVPCConnection request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AwsAccountId" => {
                input.aws_account_id = value.to_string();
            }
            "VPCConnectionId" => {
                input.v_p_c_connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
