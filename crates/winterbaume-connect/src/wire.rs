//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-connect

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
pub fn serialize_activate_evaluation_form_response(
    result: &ActivateEvaluationFormResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_analytics_data_set_response(
    result: &AssociateAnalyticsDataSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_approved_origin_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_bot_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_contact_with_user_response(
    result: &AssociateContactWithUserResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_default_vocabulary_response(
    result: &AssociateDefaultVocabularyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_email_address_alias_response(
    result: &AssociateEmailAddressAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_flow_response(result: &AssociateFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_hours_of_operations_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_instance_storage_config_response(
    result: &AssociateInstanceStorageConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_lambda_function_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_lex_bot_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_phone_number_contact_flow_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_queue_email_addresses_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_queue_quick_connects_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_routing_profile_queues_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_security_key_response(
    result: &AssociateSecurityKeyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_security_profiles_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_traffic_distribution_group_user_response(
    result: &AssociateTrafficDistributionGroupUserResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_associate_user_proficiencies_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_workspace_response(result: &AssociateWorkspaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_associate_analytics_data_set_response(
    result: &BatchAssociateAnalyticsDataSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_create_data_table_value_response(
    result: &BatchCreateDataTableValueResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_data_table_value_response(
    result: &BatchDeleteDataTableValueResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_describe_data_table_value_response(
    result: &BatchDescribeDataTableValueResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_disassociate_analytics_data_set_response(
    result: &BatchDisassociateAnalyticsDataSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_attached_file_metadata_response(
    result: &BatchGetAttachedFileMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_flow_association_response(
    result: &BatchGetFlowAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_put_contact_response(result: &BatchPutContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_data_table_value_response(
    result: &BatchUpdateDataTableValueResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_claim_phone_number_response(result: &ClaimPhoneNumberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_complete_attached_file_upload_response(
    result: &CompleteAttachedFileUploadResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_agent_status_response(result: &CreateAgentStatusResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_response(result: &CreateContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_flow_response(result: &CreateContactFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_flow_module_response(
    result: &CreateContactFlowModuleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_flow_module_alias_response(
    result: &CreateContactFlowModuleAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_flow_module_version_response(
    result: &CreateContactFlowModuleVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_flow_version_response(
    result: &CreateContactFlowVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_table_response(result: &CreateDataTableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_table_attribute_response(
    result: &CreateDataTableAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_email_address_response(
    result: &CreateEmailAddressResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_evaluation_form_response(
    result: &CreateEvaluationFormResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_hours_of_operation_response(
    result: &CreateHoursOfOperationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_hours_of_operation_override_response(
    result: &CreateHoursOfOperationOverrideResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_instance_response(result: &CreateInstanceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_integration_association_response(
    result: &CreateIntegrationAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_notification_response(result: &CreateNotificationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_participant_response(result: &CreateParticipantResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_persistent_contact_association_response(
    result: &CreatePersistentContactAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_predefined_attribute_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_prompt_response(result: &CreatePromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_push_notification_registration_response(
    result: &CreatePushNotificationRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_queue_response(result: &CreateQueueResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_quick_connect_response(
    result: &CreateQuickConnectResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_routing_profile_response(
    result: &CreateRoutingProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_rule_response(result: &CreateRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_security_profile_response(
    result: &CreateSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_task_template_response(
    result: &CreateTaskTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_test_case_response(result: &CreateTestCaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_traffic_distribution_group_response(
    result: &CreateTrafficDistributionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_use_case_response(result: &CreateUseCaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_user_hierarchy_group_response(
    result: &CreateUserHierarchyGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_view_response(result: &CreateViewResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_view_version_response(result: &CreateViewVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vocabulary_response(result: &CreateVocabularyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_workspace_response(result: &CreateWorkspaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_workspace_page_response(
    result: &CreateWorkspacePageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deactivate_evaluation_form_response(
    result: &DeactivateEvaluationFormResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_attached_file_response(
    result: &DeleteAttachedFileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_contact_evaluation_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_flow_response(result: &DeleteContactFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_flow_module_response(
    result: &DeleteContactFlowModuleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_flow_module_alias_response(
    result: &DeleteContactFlowModuleAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_flow_module_version_response(
    result: &DeleteContactFlowModuleVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_flow_version_response(
    result: &DeleteContactFlowVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_table_response(result: &DeleteDataTableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_table_attribute_response(
    result: &DeleteDataTableAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_address_response(
    result: &DeleteEmailAddressResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_evaluation_form_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_hours_of_operation_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_hours_of_operation_override_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_instance_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_integration_association_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_notification_response(result: &DeleteNotificationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_predefined_attribute_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_prompt_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_push_notification_registration_response(
    result: &DeletePushNotificationRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_queue_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_quick_connect_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_routing_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_security_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_task_template_response(
    result: &DeleteTaskTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_test_case_response(result: &DeleteTestCaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_traffic_distribution_group_response(
    result: &DeleteTrafficDistributionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_use_case_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_user_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_user_hierarchy_group_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_view_response(result: &DeleteViewResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_view_version_response(result: &DeleteViewVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vocabulary_response(result: &DeleteVocabularyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_workspace_response(result: &DeleteWorkspaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_workspace_media_response(
    result: &DeleteWorkspaceMediaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_workspace_page_response(
    result: &DeleteWorkspacePageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_agent_status_response(
    result: &DescribeAgentStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_attached_files_configuration_response(
    result: &DescribeAttachedFilesConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_authentication_profile_response(
    result: &DescribeAuthenticationProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_contact_response(result: &DescribeContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_contact_evaluation_response(
    result: &DescribeContactEvaluationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_contact_flow_response(
    result: &DescribeContactFlowResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_contact_flow_module_response(
    result: &DescribeContactFlowModuleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_contact_flow_module_alias_response(
    result: &DescribeContactFlowModuleAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_table_response(result: &DescribeDataTableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_data_table_attribute_response(
    result: &DescribeDataTableAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_email_address_response(
    result: &DescribeEmailAddressResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_evaluation_form_response(
    result: &DescribeEvaluationFormResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_hours_of_operation_response(
    result: &DescribeHoursOfOperationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_hours_of_operation_override_response(
    result: &DescribeHoursOfOperationOverrideResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_instance_response(result: &DescribeInstanceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_instance_attribute_response(
    result: &DescribeInstanceAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_instance_storage_config_response(
    result: &DescribeInstanceStorageConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_notification_response(
    result: &DescribeNotificationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_phone_number_response(
    result: &DescribePhoneNumberResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_predefined_attribute_response(
    result: &DescribePredefinedAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_prompt_response(result: &DescribePromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_queue_response(result: &DescribeQueueResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_quick_connect_response(
    result: &DescribeQuickConnectResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_routing_profile_response(
    result: &DescribeRoutingProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_rule_response(result: &DescribeRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_security_profile_response(
    result: &DescribeSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_test_case_response(result: &DescribeTestCaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_traffic_distribution_group_response(
    result: &DescribeTrafficDistributionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_user_response(result: &DescribeUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_user_hierarchy_group_response(
    result: &DescribeUserHierarchyGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_user_hierarchy_structure_response(
    result: &DescribeUserHierarchyStructureResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_view_response(result: &DescribeViewResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_vocabulary_response(result: &DescribeVocabularyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_workspace_response(result: &DescribeWorkspaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_analytics_data_set_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_approved_origin_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_bot_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_email_address_alias_response(
    result: &DisassociateEmailAddressAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_flow_response(result: &DisassociateFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_hours_of_operations_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_instance_storage_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_lambda_function_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_lex_bot_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_phone_number_contact_flow_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_queue_email_addresses_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_queue_quick_connects_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_routing_profile_queues_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_security_key_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_security_profiles_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_traffic_distribution_group_user_response(
    result: &DisassociateTrafficDistributionGroupUserResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_user_proficiencies_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_workspace_response(
    result: &DisassociateWorkspaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_dismiss_user_contact_response(
    result: &DismissUserContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_evaluate_data_table_values_response(
    result: &EvaluateDataTableValuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_attached_file_response(result: &GetAttachedFileResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_contact_attributes_response(
    result: &GetContactAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_contact_metrics_response(result: &GetContactMetricsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_current_metric_data_response(
    result: &GetCurrentMetricDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_current_user_data_response(
    result: &GetCurrentUserDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_effective_hours_of_operations_response(
    result: &GetEffectiveHoursOfOperationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_federation_token_response(
    result: &GetFederationTokenResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_association_response(
    result: &GetFlowAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_metric_data_response(result: &GetMetricDataResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_metric_data_v2_response(result: &GetMetricDataV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_prompt_file_response(result: &GetPromptFileResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_task_template_response(result: &GetTaskTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_test_case_execution_summary_response(
    result: &GetTestCaseExecutionSummaryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_traffic_distribution_response(
    result: &GetTrafficDistributionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_phone_number_response(result: &ImportPhoneNumberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_workspace_media_response(
    result: &ImportWorkspaceMediaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_statuses_response(result: &ListAgentStatusResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_analytics_data_associations_response(
    result: &ListAnalyticsDataAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_analytics_data_lake_data_sets_response(
    result: &ListAnalyticsDataLakeDataSetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_approved_origins_response(
    result: &ListApprovedOriginsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_associated_contacts_response(
    result: &ListAssociatedContactsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attached_files_configurations_response(
    result: &ListAttachedFilesConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_authentication_profiles_response(
    result: &ListAuthenticationProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bots_response(result: &ListBotsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_child_hours_of_operations_response(
    result: &ListChildHoursOfOperationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_evaluations_response(
    result: &ListContactEvaluationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_flow_module_aliases_response(
    result: &ListContactFlowModuleAliasesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_flow_module_versions_response(
    result: &ListContactFlowModuleVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_flow_modules_response(
    result: &ListContactFlowModulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_flow_versions_response(
    result: &ListContactFlowVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_flows_response(result: &ListContactFlowsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_references_response(
    result: &ListContactReferencesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_table_attributes_response(
    result: &ListDataTableAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_table_primary_values_response(
    result: &ListDataTablePrimaryValuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_table_values_response(
    result: &ListDataTableValuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_tables_response(result: &ListDataTablesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_default_vocabularies_response(
    result: &ListDefaultVocabulariesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_entity_security_profiles_response(
    result: &ListEntitySecurityProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_evaluation_form_versions_response(
    result: &ListEvaluationFormVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_evaluation_forms_response(
    result: &ListEvaluationFormsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flow_associations_response(
    result: &ListFlowAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_hours_of_operation_overrides_response(
    result: &ListHoursOfOperationOverridesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_hours_of_operations_response(
    result: &ListHoursOfOperationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_instance_attributes_response(
    result: &ListInstanceAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_instance_storage_configs_response(
    result: &ListInstanceStorageConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_instances_response(result: &ListInstancesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_integration_associations_response(
    result: &ListIntegrationAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_lambda_functions_response(
    result: &ListLambdaFunctionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_lex_bots_response(result: &ListLexBotsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_notifications_response(result: &ListNotificationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_phone_numbers_response(result: &ListPhoneNumbersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_phone_numbers_v2_response(
    result: &ListPhoneNumbersV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_predefined_attributes_response(
    result: &ListPredefinedAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_prompts_response(result: &ListPromptsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_queue_email_addresses_response(
    result: &ListQueueEmailAddressesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_queue_quick_connects_response(
    result: &ListQueueQuickConnectsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_queues_response(result: &ListQueuesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_quick_connects_response(result: &ListQuickConnectsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_realtime_contact_analysis_segments_v2_response(
    result: &ListRealtimeContactAnalysisSegmentsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_routing_profile_manual_assignment_queues_response(
    result: &ListRoutingProfileManualAssignmentQueuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_routing_profile_queues_response(
    result: &ListRoutingProfileQueuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_routing_profiles_response(
    result: &ListRoutingProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_rules_response(result: &ListRulesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_keys_response(result: &ListSecurityKeysResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_profile_applications_response(
    result: &ListSecurityProfileApplicationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_profile_flow_modules_response(
    result: &ListSecurityProfileFlowModulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_profile_permissions_response(
    result: &ListSecurityProfilePermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_profiles_response(
    result: &ListSecurityProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_task_templates_response(result: &ListTaskTemplatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_case_execution_records_response(
    result: &ListTestCaseExecutionRecordsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_case_executions_response(
    result: &ListTestCaseExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_cases_response(result: &ListTestCasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_traffic_distribution_group_users_response(
    result: &ListTrafficDistributionGroupUsersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_traffic_distribution_groups_response(
    result: &ListTrafficDistributionGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_use_cases_response(result: &ListUseCasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_hierarchy_groups_response(
    result: &ListUserHierarchyGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_notifications_response(
    result: &ListUserNotificationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_proficiencies_response(
    result: &ListUserProficienciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_view_versions_response(result: &ListViewVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_views_response(result: &ListViewsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_workspace_media_response(
    result: &ListWorkspaceMediaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_workspace_pages_response(
    result: &ListWorkspacePagesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_workspaces_response(result: &ListWorkspacesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_monitor_contact_response(result: &MonitorContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_pause_contact_response(result: &PauseContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_user_status_response(result: &PutUserStatusResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_release_phone_number_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_replicate_instance_response(result: &ReplicateInstanceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_resume_contact_response(result: &ResumeContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_resume_contact_recording_response(
    result: &ResumeContactRecordingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_agent_statuses_response(
    result: &SearchAgentStatusesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_available_phone_numbers_response(
    result: &SearchAvailablePhoneNumbersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_contact_evaluations_response(
    result: &SearchContactEvaluationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_contact_flow_modules_response(
    result: &SearchContactFlowModulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_contact_flows_response(
    result: &SearchContactFlowsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_contacts_response(result: &SearchContactsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_data_tables_response(result: &SearchDataTablesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_email_addresses_response(
    result: &SearchEmailAddressesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_evaluation_forms_response(
    result: &SearchEvaluationFormsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_hours_of_operation_overrides_response(
    result: &SearchHoursOfOperationOverridesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_hours_of_operations_response(
    result: &SearchHoursOfOperationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_notifications_response(
    result: &SearchNotificationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_predefined_attributes_response(
    result: &SearchPredefinedAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_prompts_response(result: &SearchPromptsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_queues_response(result: &SearchQueuesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_quick_connects_response(
    result: &SearchQuickConnectsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_resource_tags_response(
    result: &SearchResourceTagsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_routing_profiles_response(
    result: &SearchRoutingProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_security_profiles_response(
    result: &SearchSecurityProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_test_cases_response(result: &SearchTestCasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_user_hierarchy_groups_response(
    result: &SearchUserHierarchyGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_users_response(result: &SearchUsersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_views_response(result: &SearchViewsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_vocabularies_response(result: &SearchVocabulariesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_workspace_associations_response(
    result: &SearchWorkspaceAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_workspaces_response(result: &SearchWorkspacesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_chat_integration_event_response(
    result: &SendChatIntegrationEventResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_outbound_email_response(result: &SendOutboundEmailResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_attached_file_upload_response(
    result: &StartAttachedFileUploadResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_chat_contact_response(result: &StartChatContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_contact_evaluation_response(
    result: &StartContactEvaluationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_contact_media_processing_response(
    result: &StartContactMediaProcessingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_contact_recording_response(
    result: &StartContactRecordingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_contact_streaming_response(
    result: &StartContactStreamingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_email_contact_response(result: &StartEmailContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_outbound_chat_contact_response(
    result: &StartOutboundChatContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_outbound_email_contact_response(
    result: &StartOutboundEmailContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_outbound_voice_contact_response(
    result: &StartOutboundVoiceContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_screen_sharing_response(
    result: &StartScreenSharingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_task_contact_response(result: &StartTaskContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_test_case_execution_response(
    result: &StartTestCaseExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_web_r_t_c_contact_response(
    result: &StartWebRTCContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_contact_response(result: &StopContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_contact_media_processing_response(
    result: &StopContactMediaProcessingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_contact_recording_response(
    result: &StopContactRecordingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_contact_streaming_response(
    result: &StopContactStreamingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_test_case_execution_response(
    result: &StopTestCaseExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_submit_contact_evaluation_response(
    result: &SubmitContactEvaluationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_suspend_contact_recording_response(
    result: &SuspendContactRecordingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_contact_response(result: &TagContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_transfer_contact_response(result: &TransferContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_contact_response(result: &UntagContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_agent_status_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_attached_files_configuration_response(
    result: &UpdateAttachedFilesConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_authentication_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_response(result: &UpdateContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_attributes_response(
    result: &UpdateContactAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_evaluation_response(
    result: &UpdateContactEvaluationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_content_response(
    result: &UpdateContactFlowContentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_metadata_response(
    result: &UpdateContactFlowMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_module_alias_response(
    result: &UpdateContactFlowModuleAliasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_module_content_response(
    result: &UpdateContactFlowModuleContentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_module_metadata_response(
    result: &UpdateContactFlowModuleMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_flow_name_response(
    result: &UpdateContactFlowNameResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_routing_data_response(
    result: &UpdateContactRoutingDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_schedule_response(
    result: &UpdateContactScheduleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_table_attribute_response(
    result: &UpdateDataTableAttributeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_table_metadata_response(
    result: &UpdateDataTableMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_table_primary_values_response(
    result: &UpdateDataTablePrimaryValuesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_email_address_metadata_response(
    result: &UpdateEmailAddressMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_evaluation_form_response(
    result: &UpdateEvaluationFormResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_hours_of_operation_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_hours_of_operation_override_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_instance_attribute_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_instance_storage_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_notification_content_response(
    result: &UpdateNotificationContentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_participant_authentication_response(
    result: &UpdateParticipantAuthenticationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_participant_role_config_response(
    result: &UpdateParticipantRoleConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_phone_number_response(result: &UpdatePhoneNumberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_phone_number_metadata_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_predefined_attribute_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_prompt_response(result: &UpdatePromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_hours_of_operation_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_max_contacts_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_outbound_caller_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_outbound_email_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_queue_status_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_quick_connect_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_quick_connect_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_routing_profile_agent_availability_timer_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_routing_profile_concurrency_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_routing_profile_default_outbound_queue_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_routing_profile_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_routing_profile_queues_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_security_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_task_template_response(
    result: &UpdateTaskTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_test_case_response(result: &UpdateTestCaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_traffic_distribution_response(
    result: &UpdateTrafficDistributionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_hierarchy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_hierarchy_group_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_hierarchy_structure_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_identity_info_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_notification_status_response(
    result: &UpdateUserNotificationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_phone_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_proficiencies_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_routing_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_user_security_profiles_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_view_content_response(result: &UpdateViewContentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_view_metadata_response(
    result: &UpdateViewMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_workspace_metadata_response(
    result: &UpdateWorkspaceMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_workspace_page_response(
    result: &UpdateWorkspacePageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_workspace_theme_response(
    result: &UpdateWorkspaceThemeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_workspace_visibility_response(
    result: &UpdateWorkspaceVisibilityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_activate_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ActivateEvaluationFormRequest, String> {
    let mut input = ActivateEvaluationFormRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ActivateEvaluationFormRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ActivateEvaluationForm request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_analytics_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAnalyticsDataSetRequest, String> {
    let mut input = AssociateAnalyticsDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateAnalyticsDataSetRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateAnalyticsDataSet request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_approved_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateApprovedOriginRequest, String> {
    let mut input = AssociateApprovedOriginRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateApprovedOriginRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateApprovedOrigin request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateBotRequest, String> {
    let mut input = AssociateBotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateBotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateBot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_contact_with_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateContactWithUserRequest, String> {
    let mut input = AssociateContactWithUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateContactWithUserRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateContactWithUser request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_default_vocabulary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateDefaultVocabularyRequest, String> {
    let mut input = AssociateDefaultVocabularyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateDefaultVocabularyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateDefaultVocabulary request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "LanguageCode" => {
                input.language_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_email_address_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateEmailAddressAliasRequest, String> {
    let mut input = AssociateEmailAddressAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateEmailAddressAliasRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateEmailAddressAlias request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailAddressId" => {
                input.email_address_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateFlowRequest, String> {
    let mut input = AssociateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateFlow request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateHoursOfOperationsRequest, String> {
    let mut input = AssociateHoursOfOperationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateHoursOfOperationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateHoursOfOperations request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_instance_storage_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateInstanceStorageConfigRequest, String> {
    let mut input = AssociateInstanceStorageConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateInstanceStorageConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateInstanceStorageConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_lambda_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateLambdaFunctionRequest, String> {
    let mut input = AssociateLambdaFunctionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateLambdaFunctionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateLambdaFunction request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_lex_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateLexBotRequest, String> {
    let mut input = AssociateLexBotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateLexBotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateLexBot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_phone_number_contact_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociatePhoneNumberContactFlowRequest, String> {
    let mut input = AssociatePhoneNumberContactFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociatePhoneNumberContactFlowRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociatePhoneNumberContactFlow request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_queue_email_addresses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateQueueEmailAddressesRequest, String> {
    let mut input = AssociateQueueEmailAddressesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateQueueEmailAddressesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateQueueEmailAddresses request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_queue_quick_connects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateQueueQuickConnectsRequest, String> {
    let mut input = AssociateQueueQuickConnectsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateQueueQuickConnectsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateQueueQuickConnects request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_routing_profile_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateRoutingProfileQueuesRequest, String> {
    let mut input = AssociateRoutingProfileQueuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateRoutingProfileQueuesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateRoutingProfileQueues request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_security_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateSecurityKeyRequest, String> {
    let mut input = AssociateSecurityKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateSecurityKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateSecurityKey request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateSecurityProfilesRequest, String> {
    let mut input = AssociateSecurityProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateSecurityProfilesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateSecurityProfiles request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_traffic_distribution_group_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateTrafficDistributionGroupUserRequest, String> {
    let mut input = AssociateTrafficDistributionGroupUserRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<AssociateTrafficDistributionGroupUserRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize AssociateTrafficDistributionGroupUser request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "TrafficDistributionGroupId" => {
                input.traffic_distribution_group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_user_proficiencies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateUserProficienciesRequest, String> {
    let mut input = AssociateUserProficienciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateUserProficienciesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateUserProficiencies request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateWorkspaceRequest, String> {
    let mut input = AssociateWorkspaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateWorkspaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateWorkspace request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_associate_analytics_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchAssociateAnalyticsDataSetRequest, String> {
    let mut input = BatchAssociateAnalyticsDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchAssociateAnalyticsDataSetRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchAssociateAnalyticsDataSet request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_create_data_table_value_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchCreateDataTableValueRequest, String> {
    let mut input = BatchCreateDataTableValueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchCreateDataTableValueRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchCreateDataTableValue request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_data_table_value_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteDataTableValueRequest, String> {
    let mut input = BatchDeleteDataTableValueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteDataTableValueRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchDeleteDataTableValue request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_describe_data_table_value_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDescribeDataTableValueRequest, String> {
    let mut input = BatchDescribeDataTableValueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDescribeDataTableValueRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDescribeDataTableValue request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_disassociate_analytics_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDisassociateAnalyticsDataSetRequest, String> {
    let mut input = BatchDisassociateAnalyticsDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDisassociateAnalyticsDataSetRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize BatchDisassociateAnalyticsDataSet request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_attached_file_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetAttachedFileMetadataRequest, String> {
    let mut input = BatchGetAttachedFileMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetAttachedFileMetadataRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchGetAttachedFileMetadata request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedResourceArn") {
        input.associated_resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_flow_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetFlowAssociationRequest, String> {
    let mut input = BatchGetFlowAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetFlowAssociationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchGetFlowAssociation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_put_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchPutContactRequest, String> {
    let mut input = BatchPutContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchPutContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchPutContact request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_data_table_value_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateDataTableValueRequest, String> {
    let mut input = BatchUpdateDataTableValueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateDataTableValueRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchUpdateDataTableValue request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_claim_phone_number_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ClaimPhoneNumberRequest, String> {
    let mut input = ClaimPhoneNumberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ClaimPhoneNumberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ClaimPhoneNumber request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_complete_attached_file_upload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CompleteAttachedFileUploadRequest, String> {
    let mut input = CompleteAttachedFileUploadRequest::default();
    for (name, value) in labels {
        match *name {
            "FileId" => {
                input.file_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedResourceArn") {
        input.associated_resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_agent_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAgentStatusRequest, String> {
    let mut input = CreateAgentStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAgentStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAgentStatus request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactRequest, String> {
    let mut input = CreateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactFlowRequest, String> {
    let mut input = CreateContactFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateContactFlow request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_flow_module_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactFlowModuleRequest, String> {
    let mut input = CreateContactFlowModuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactFlowModuleRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateContactFlowModule request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_flow_module_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactFlowModuleAliasRequest, String> {
    let mut input = CreateContactFlowModuleAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactFlowModuleAliasRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateContactFlowModuleAlias request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_flow_module_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactFlowModuleVersionRequest, String> {
    let mut input = CreateContactFlowModuleVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactFlowModuleVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateContactFlowModuleVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_flow_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactFlowVersionRequest, String> {
    let mut input = CreateContactFlowVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactFlowVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateContactFlowVersion request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataTableRequest, String> {
    let mut input = CreateDataTableRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataTableRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataTable request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_table_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataTableAttributeRequest, String> {
    let mut input = CreateDataTableAttributeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataTableAttributeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDataTableAttribute request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_email_address_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEmailAddressRequest, String> {
    let mut input = CreateEmailAddressRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEmailAddressRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEmailAddress request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEvaluationFormRequest, String> {
    let mut input = CreateEvaluationFormRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEvaluationFormRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEvaluationForm request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_hours_of_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHoursOfOperationRequest, String> {
    let mut input = CreateHoursOfOperationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateHoursOfOperationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateHoursOfOperation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_hours_of_operation_override_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHoursOfOperationOverrideRequest, String> {
    let mut input = CreateHoursOfOperationOverrideRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateHoursOfOperationOverrideRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateHoursOfOperationOverride request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInstanceRequest, String> {
    let mut input = CreateInstanceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInstanceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateInstance request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_integration_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIntegrationAssociationRequest, String> {
    let mut input = CreateIntegrationAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIntegrationAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateIntegrationAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_notification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNotificationRequest, String> {
    let mut input = CreateNotificationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNotificationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNotification request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_participant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateParticipantRequest, String> {
    let mut input = CreateParticipantRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateParticipantRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateParticipant request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_persistent_contact_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePersistentContactAssociationRequest, String> {
    let mut input = CreatePersistentContactAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePersistentContactAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePersistentContactAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InitialContactId" => {
                input.initial_contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_predefined_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePredefinedAttributeRequest, String> {
    let mut input = CreatePredefinedAttributeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePredefinedAttributeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreatePredefinedAttribute request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePromptRequest, String> {
    let mut input = CreatePromptRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePromptRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePrompt request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_push_notification_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePushNotificationRegistrationRequest, String> {
    let mut input = CreatePushNotificationRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePushNotificationRegistrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePushNotificationRegistration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateQueueRequest, String> {
    let mut input = CreateQueueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateQueueRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateQueue request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_quick_connect_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateQuickConnectRequest, String> {
    let mut input = CreateQuickConnectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateQuickConnectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateQuickConnect request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_routing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRoutingProfileRequest, String> {
    let mut input = CreateRoutingProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRoutingProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRoutingProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRuleRequest, String> {
    let mut input = CreateRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSecurityProfileRequest, String> {
    let mut input = CreateSecurityProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSecurityProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSecurityProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_task_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTaskTemplateRequest, String> {
    let mut input = CreateTaskTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTaskTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTaskTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTestCaseRequest, String> {
    let mut input = CreateTestCaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTestCaseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTestCase request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-region")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_region = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-time")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_time = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-resource-id")
        .and_then(|value| value.to_str().ok())
    {
        input.test_case_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_traffic_distribution_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrafficDistributionGroupRequest, String> {
    let mut input = CreateTrafficDistributionGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTrafficDistributionGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTrafficDistributionGroup request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_use_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUseCaseRequest, String> {
    let mut input = CreateUseCaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUseCaseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUseCase request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "IntegrationAssociationId" => {
                input.integration_association_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUserRequest, String> {
    let mut input = CreateUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUserRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUser request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_user_hierarchy_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUserHierarchyGroupRequest, String> {
    let mut input = CreateUserHierarchyGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUserHierarchyGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateUserHierarchyGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_view_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateViewRequest, String> {
    let mut input = CreateViewRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateViewRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateView request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_view_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateViewVersionRequest, String> {
    let mut input = CreateViewVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateViewVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateViewVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vocabulary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVocabularyRequest, String> {
    let mut input = CreateVocabularyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVocabularyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVocabulary request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateWorkspaceRequest, String> {
    let mut input = CreateWorkspaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateWorkspaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateWorkspace request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_workspace_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateWorkspacePageRequest, String> {
    let mut input = CreateWorkspacePageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateWorkspacePageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateWorkspacePage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deactivate_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeactivateEvaluationFormRequest, String> {
    let mut input = DeactivateEvaluationFormRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeactivateEvaluationFormRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeactivateEvaluationForm request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_attached_file_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAttachedFileRequest, String> {
    let mut input = DeleteAttachedFileRequest::default();
    for (name, value) in labels {
        match *name {
            "FileId" => {
                input.file_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedResourceArn") {
        input.associated_resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_evaluation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactEvaluationRequest, String> {
    let mut input = DeleteContactEvaluationRequest::default();
    for (name, value) in labels {
        match *name {
            "EvaluationId" => {
                input.evaluation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactFlowRequest, String> {
    let mut input = DeleteContactFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_flow_module_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactFlowModuleRequest, String> {
    let mut input = DeleteContactFlowModuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_flow_module_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactFlowModuleAliasRequest, String> {
    let mut input = DeleteContactFlowModuleAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasId" => {
                input.alias_id = value.to_string();
            }
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_flow_module_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactFlowModuleVersionRequest, String> {
    let mut input = DeleteContactFlowModuleVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "ContactFlowModuleVersion" => {
                input.contact_flow_module_version = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_flow_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactFlowVersionRequest, String> {
    let mut input = DeleteContactFlowVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "ContactFlowVersion" => {
                input.contact_flow_version = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataTableRequest, String> {
    let mut input = DeleteDataTableRequest::default();
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_table_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataTableAttributeRequest, String> {
    let mut input = DeleteDataTableAttributeRequest::default();
    for (name, value) in labels {
        match *name {
            "AttributeName" => {
                input.attribute_name = value.to_string();
            }
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_email_address_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEmailAddressRequest, String> {
    let mut input = DeleteEmailAddressRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailAddressId" => {
                input.email_address_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEvaluationFormRequest, String> {
    let mut input = DeleteEvaluationFormRequest::default();
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version") {
        input.evaluation_form_version = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_hours_of_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteHoursOfOperationRequest, String> {
    let mut input = DeleteHoursOfOperationRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_hours_of_operation_override_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteHoursOfOperationOverrideRequest, String> {
    let mut input = DeleteHoursOfOperationOverrideRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "HoursOfOperationOverrideId" => {
                input.hours_of_operation_override_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInstanceRequest, String> {
    let mut input = DeleteInstanceRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_integration_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntegrationAssociationRequest, String> {
    let mut input = DeleteIntegrationAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "IntegrationAssociationId" => {
                input.integration_association_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_notification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNotificationRequest, String> {
    let mut input = DeleteNotificationRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "NotificationId" => {
                input.notification_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_predefined_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePredefinedAttributeRequest, String> {
    let mut input = DeletePredefinedAttributeRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePromptRequest, String> {
    let mut input = DeletePromptRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "PromptId" => {
                input.prompt_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_push_notification_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePushNotificationRegistrationRequest, String> {
    let mut input = DeletePushNotificationRegistrationRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RegistrationId" => {
                input.registration_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("contactId") {
        input.contact_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQueueRequest, String> {
    let mut input = DeleteQueueRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_quick_connect_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQuickConnectRequest, String> {
    let mut input = DeleteQuickConnectRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QuickConnectId" => {
                input.quick_connect_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_routing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoutingProfileRequest, String> {
    let mut input = DeleteRoutingProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRuleRequest, String> {
    let mut input = DeleteRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RuleId" => {
                input.rule_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSecurityProfileRequest, String> {
    let mut input = DeleteSecurityProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_task_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTaskTemplateRequest, String> {
    let mut input = DeleteTaskTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TaskTemplateId" => {
                input.task_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTestCaseRequest, String> {
    let mut input = DeleteTestCaseRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_traffic_distribution_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrafficDistributionGroupRequest, String> {
    let mut input = DeleteTrafficDistributionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "TrafficDistributionGroupId" => {
                input.traffic_distribution_group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_use_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUseCaseRequest, String> {
    let mut input = DeleteUseCaseRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "IntegrationAssociationId" => {
                input.integration_association_id = value.to_string();
            }
            "UseCaseId" => {
                input.use_case_id = value.to_string();
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
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_hierarchy_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserHierarchyGroupRequest, String> {
    let mut input = DeleteUserHierarchyGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "HierarchyGroupId" => {
                input.hierarchy_group_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_view_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteViewRequest, String> {
    let mut input = DeleteViewRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_view_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteViewVersionRequest, String> {
    let mut input = DeleteViewVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            "ViewVersion" => {
                input.view_version = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vocabulary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVocabularyRequest, String> {
    let mut input = DeleteVocabularyRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "VocabularyId" => {
                input.vocabulary_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteWorkspaceRequest, String> {
    let mut input = DeleteWorkspaceRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_workspace_media_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteWorkspaceMediaRequest, String> {
    let mut input = DeleteWorkspaceMediaRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("mediaType") {
        input.media_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_workspace_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteWorkspacePageRequest, String> {
    let mut input = DeleteWorkspacePageRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "Page" => {
                input.page = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_agent_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAgentStatusRequest, String> {
    let mut input = DescribeAgentStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "AgentStatusId" => {
                input.agent_status_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_attached_files_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAttachedFilesConfigurationRequest, String> {
    let mut input = DescribeAttachedFilesConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentScope" => {
                input.attachment_scope = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_authentication_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuthenticationProfileRequest, String> {
    let mut input = DescribeAuthenticationProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "AuthenticationProfileId" => {
                input.authentication_profile_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeContactRequest, String> {
    let mut input = DescribeContactRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_contact_evaluation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeContactEvaluationRequest, String> {
    let mut input = DescribeContactEvaluationRequest::default();
    for (name, value) in labels {
        match *name {
            "EvaluationId" => {
                input.evaluation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_contact_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeContactFlowRequest, String> {
    let mut input = DescribeContactFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_contact_flow_module_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeContactFlowModuleRequest, String> {
    let mut input = DescribeContactFlowModuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_contact_flow_module_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeContactFlowModuleAliasRequest, String> {
    let mut input = DescribeContactFlowModuleAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "AliasId" => {
                input.alias_id = value.to_string();
            }
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataTableRequest, String> {
    let mut input = DescribeDataTableRequest::default();
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_data_table_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataTableAttributeRequest, String> {
    let mut input = DescribeDataTableAttributeRequest::default();
    for (name, value) in labels {
        match *name {
            "AttributeName" => {
                input.attribute_name = value.to_string();
            }
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_email_address_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEmailAddressRequest, String> {
    let mut input = DescribeEmailAddressRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailAddressId" => {
                input.email_address_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEvaluationFormRequest, String> {
    let mut input = DescribeEvaluationFormRequest::default();
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version") {
        input.evaluation_form_version = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_hours_of_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeHoursOfOperationRequest, String> {
    let mut input = DescribeHoursOfOperationRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_hours_of_operation_override_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeHoursOfOperationOverrideRequest, String> {
    let mut input = DescribeHoursOfOperationOverrideRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "HoursOfOperationOverrideId" => {
                input.hours_of_operation_override_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstanceRequest, String> {
    let mut input = DescribeInstanceRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_instance_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstanceAttributeRequest, String> {
    let mut input = DescribeInstanceAttributeRequest::default();
    for (name, value) in labels {
        match *name {
            "AttributeType" => {
                input.attribute_type = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_instance_storage_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstanceStorageConfigRequest, String> {
    let mut input = DescribeInstanceStorageConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "AssociationId" => {
                input.association_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_notification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNotificationRequest, String> {
    let mut input = DescribeNotificationRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "NotificationId" => {
                input.notification_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_phone_number_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePhoneNumberRequest, String> {
    let mut input = DescribePhoneNumberRequest::default();
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_predefined_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePredefinedAttributeRequest, String> {
    let mut input = DescribePredefinedAttributeRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePromptRequest, String> {
    let mut input = DescribePromptRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "PromptId" => {
                input.prompt_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQueueRequest, String> {
    let mut input = DescribeQueueRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_quick_connect_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQuickConnectRequest, String> {
    let mut input = DescribeQuickConnectRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QuickConnectId" => {
                input.quick_connect_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_routing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRoutingProfileRequest, String> {
    let mut input = DescribeRoutingProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRuleRequest, String> {
    let mut input = DescribeRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RuleId" => {
                input.rule_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSecurityProfileRequest, String> {
    let mut input = DescribeSecurityProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTestCaseRequest, String> {
    let mut input = DescribeTestCaseRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_traffic_distribution_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTrafficDistributionGroupRequest, String> {
    let mut input = DescribeTrafficDistributionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "TrafficDistributionGroupId" => {
                input.traffic_distribution_group_id = value.to_string();
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
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_user_hierarchy_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeUserHierarchyGroupRequest, String> {
    let mut input = DescribeUserHierarchyGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "HierarchyGroupId" => {
                input.hierarchy_group_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_user_hierarchy_structure_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeUserHierarchyStructureRequest, String> {
    let mut input = DescribeUserHierarchyStructureRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_view_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeViewRequest, String> {
    let mut input = DescribeViewRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_vocabulary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVocabularyRequest, String> {
    let mut input = DescribeVocabularyRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "VocabularyId" => {
                input.vocabulary_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeWorkspaceRequest, String> {
    let mut input = DescribeWorkspaceRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_analytics_data_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAnalyticsDataSetRequest, String> {
    let mut input = DisassociateAnalyticsDataSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateAnalyticsDataSetRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateAnalyticsDataSet request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_approved_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateApprovedOriginRequest, String> {
    let mut input = DisassociateApprovedOriginRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("origin") {
        input.origin = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateBotRequest, String> {
    let mut input = DisassociateBotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateBotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisassociateBot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_email_address_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateEmailAddressAliasRequest, String> {
    let mut input = DisassociateEmailAddressAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateEmailAddressAliasRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateEmailAddressAlias request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailAddressId" => {
                input.email_address_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateFlowRequest, String> {
    let mut input = DisassociateFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ResourceId" => {
                input.resource_id = value.to_string();
            }
            "ResourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateHoursOfOperationsRequest, String> {
    let mut input = DisassociateHoursOfOperationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateHoursOfOperationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateHoursOfOperations request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_instance_storage_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateInstanceStorageConfigRequest, String> {
    let mut input = DisassociateInstanceStorageConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "AssociationId" => {
                input.association_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_lambda_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateLambdaFunctionRequest, String> {
    let mut input = DisassociateLambdaFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("functionArn") {
        input.function_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_lex_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateLexBotRequest, String> {
    let mut input = DisassociateLexBotRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("botName") {
        input.bot_name = value.to_string();
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("lexRegion") {
        input.lex_region = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_phone_number_contact_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociatePhoneNumberContactFlowRequest, String> {
    let mut input = DisassociatePhoneNumberContactFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("instanceId") {
        input.instance_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_queue_email_addresses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateQueueEmailAddressesRequest, String> {
    let mut input = DisassociateQueueEmailAddressesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateQueueEmailAddressesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateQueueEmailAddresses request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_queue_quick_connects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateQueueQuickConnectsRequest, String> {
    let mut input = DisassociateQueueQuickConnectsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateQueueQuickConnectsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateQueueQuickConnects request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_routing_profile_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateRoutingProfileQueuesRequest, String> {
    let mut input = DisassociateRoutingProfileQueuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateRoutingProfileQueuesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateRoutingProfileQueues request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_security_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateSecurityKeyRequest, String> {
    let mut input = DisassociateSecurityKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "AssociationId" => {
                input.association_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateSecurityProfilesRequest, String> {
    let mut input = DisassociateSecurityProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateSecurityProfilesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateSecurityProfiles request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_traffic_distribution_group_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateTrafficDistributionGroupUserRequest, String> {
    let mut input = DisassociateTrafficDistributionGroupUserRequest::default();
    for (name, value) in labels {
        match *name {
            "TrafficDistributionGroupId" => {
                input.traffic_distribution_group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("InstanceId") {
        input.instance_id = value.to_string();
    }
    if let Some(value) = query.get("UserId") {
        input.user_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_user_proficiencies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateUserProficienciesRequest, String> {
    let mut input = DisassociateUserProficienciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateUserProficienciesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateUserProficiencies request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateWorkspaceRequest, String> {
    let mut input = DisassociateWorkspaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateWorkspaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisassociateWorkspace request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_dismiss_user_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DismissUserContactRequest, String> {
    let mut input = DismissUserContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DismissUserContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DismissUserContact request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_evaluate_data_table_values_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EvaluateDataTableValuesRequest, String> {
    let mut input = EvaluateDataTableValuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EvaluateDataTableValuesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize EvaluateDataTableValues request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_attached_file_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAttachedFileRequest, String> {
    let mut input = GetAttachedFileRequest::default();
    for (name, value) in labels {
        match *name {
            "FileId" => {
                input.file_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedResourceArn") {
        input.associated_resource_arn = value.to_string();
    }
    if let Some(value) = query.get("urlExpiryInSeconds") {
        input.url_expiry_in_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_contact_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContactAttributesRequest, String> {
    let mut input = GetContactAttributesRequest::default();
    for (name, value) in labels {
        match *name {
            "InitialContactId" => {
                input.initial_contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_contact_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContactMetricsRequest, String> {
    let mut input = GetContactMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetContactMetricsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetContactMetrics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_current_metric_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCurrentMetricDataRequest, String> {
    let mut input = GetCurrentMetricDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCurrentMetricDataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCurrentMetricData request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_current_user_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCurrentUserDataRequest, String> {
    let mut input = GetCurrentUserDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCurrentUserDataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCurrentUserData request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_effective_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEffectiveHoursOfOperationsRequest, String> {
    let mut input = GetEffectiveHoursOfOperationsRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("fromDate") {
        input.from_date = value.to_string();
    }
    if let Some(value) = query.get("toDate") {
        input.to_date = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_federation_token_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFederationTokenRequest, String> {
    let mut input = GetFederationTokenRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowAssociationRequest, String> {
    let mut input = GetFlowAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ResourceId" => {
                input.resource_id = value.to_string();
            }
            "ResourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_metric_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMetricDataRequest, String> {
    let mut input = GetMetricDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMetricDataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetMetricData request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_metric_data_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMetricDataV2Request, String> {
    let mut input = GetMetricDataV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMetricDataV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize GetMetricDataV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_prompt_file_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPromptFileRequest, String> {
    let mut input = GetPromptFileRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "PromptId" => {
                input.prompt_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_task_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTaskTemplateRequest, String> {
    let mut input = GetTaskTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TaskTemplateId" => {
                input.task_template_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("snapshotVersion") {
        input.snapshot_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_test_case_execution_summary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTestCaseExecutionSummaryRequest, String> {
    let mut input = GetTestCaseExecutionSummaryRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseExecutionId" => {
                input.test_case_execution_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_traffic_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrafficDistributionRequest, String> {
    let mut input = GetTrafficDistributionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_phone_number_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportPhoneNumberRequest, String> {
    let mut input = ImportPhoneNumberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportPhoneNumberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportPhoneNumber request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_workspace_media_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportWorkspaceMediaRequest, String> {
    let mut input = ImportWorkspaceMediaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportWorkspaceMediaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportWorkspaceMedia request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_statuses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentStatusRequest, String> {
    let mut input = ListAgentStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("AgentStatusTypes") {
        input.agent_status_types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_analytics_data_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnalyticsDataAssociationsRequest, String> {
    let mut input = ListAnalyticsDataAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("DataSetId") {
        input.data_set_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_analytics_data_lake_data_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnalyticsDataLakeDataSetsRequest, String> {
    let mut input = ListAnalyticsDataLakeDataSetsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_approved_origins_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApprovedOriginsRequest, String> {
    let mut input = ListApprovedOriginsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_associated_contacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssociatedContactsRequest, String> {
    let mut input = ListAssociatedContactsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("contactId") {
        input.contact_id = value.to_string();
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attached_files_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedFilesConfigurationsRequest, String> {
    let mut input = ListAttachedFilesConfigurationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_authentication_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuthenticationProfilesRequest, String> {
    let mut input = ListAuthenticationProfilesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bots_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotsRequest, String> {
    let mut input = ListBotsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("lexVersion") {
        input.lex_version = value.to_string();
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_child_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListChildHoursOfOperationsRequest, String> {
    let mut input = ListChildHoursOfOperationsRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_evaluations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactEvaluationsRequest, String> {
    let mut input = ListContactEvaluationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("contactId") {
        input.contact_id = value.to_string();
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_flow_module_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactFlowModuleAliasesRequest, String> {
    let mut input = ListContactFlowModuleAliasesRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_flow_module_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactFlowModuleVersionsRequest, String> {
    let mut input = ListContactFlowModuleVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_flow_modules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactFlowModulesRequest, String> {
    let mut input = ListContactFlowModulesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("state") {
        input.contact_flow_module_state = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_flow_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactFlowVersionsRequest, String> {
    let mut input = ListContactFlowVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactFlowsRequest, String> {
    let mut input = ListContactFlowsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("contactFlowTypes") {
        input.contact_flow_types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_references_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactReferencesRequest, String> {
    let mut input = ListContactReferencesRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("referenceTypes") {
        input.reference_types = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_table_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataTableAttributesRequest, String> {
    let mut input = ListDataTableAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDataTableAttributesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListDataTableAttributes request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_table_primary_values_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataTablePrimaryValuesRequest, String> {
    let mut input = ListDataTablePrimaryValuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDataTablePrimaryValuesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListDataTablePrimaryValues request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_table_values_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataTableValuesRequest, String> {
    let mut input = ListDataTableValuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDataTableValuesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListDataTableValues request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_tables_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataTablesRequest, String> {
    let mut input = ListDataTablesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_default_vocabularies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDefaultVocabulariesRequest, String> {
    let mut input = ListDefaultVocabulariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDefaultVocabulariesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListDefaultVocabularies request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_entity_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEntitySecurityProfilesRequest, String> {
    let mut input = ListEntitySecurityProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListEntitySecurityProfilesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListEntitySecurityProfiles request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_evaluation_form_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEvaluationFormVersionsRequest, String> {
    let mut input = ListEvaluationFormVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_evaluation_forms_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEvaluationFormsRequest, String> {
    let mut input = ListEvaluationFormsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flow_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowAssociationsRequest, String> {
    let mut input = ListFlowAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_hours_of_operation_overrides_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHoursOfOperationOverridesRequest, String> {
    let mut input = ListHoursOfOperationOverridesRequest::default();
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHoursOfOperationsRequest, String> {
    let mut input = ListHoursOfOperationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_instance_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceAttributesRequest, String> {
    let mut input = ListInstanceAttributesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_instance_storage_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceStorageConfigsRequest, String> {
    let mut input = ListInstanceStorageConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_instances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInstancesRequest, String> {
    let mut input = ListInstancesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_integration_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIntegrationAssociationsRequest, String> {
    let mut input = ListIntegrationAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("integrationArn") {
        input.integration_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("integrationType") {
        input.integration_type = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_lambda_functions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLambdaFunctionsRequest, String> {
    let mut input = ListLambdaFunctionsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_lex_bots_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLexBotsRequest, String> {
    let mut input = ListLexBotsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNotificationsRequest, String> {
    let mut input = ListNotificationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_phone_numbers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPhoneNumbersRequest, String> {
    let mut input = ListPhoneNumbersRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("phoneNumberCountryCodes") {
        input.phone_number_country_codes = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("phoneNumberTypes") {
        input.phone_number_types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_phone_numbers_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPhoneNumbersV2Request, String> {
    let mut input = ListPhoneNumbersV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPhoneNumbersV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPhoneNumbersV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_predefined_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPredefinedAttributesRequest, String> {
    let mut input = ListPredefinedAttributesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_prompts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPromptsRequest, String> {
    let mut input = ListPromptsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_queue_email_addresses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQueueEmailAddressesRequest, String> {
    let mut input = ListQueueEmailAddressesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_queue_quick_connects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQueueQuickConnectsRequest, String> {
    let mut input = ListQueueQuickConnectsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQueuesRequest, String> {
    let mut input = ListQueuesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("queueTypes") {
        input.queue_types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_quick_connects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQuickConnectsRequest, String> {
    let mut input = ListQuickConnectsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("QuickConnectTypes") {
        input.quick_connect_types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_realtime_contact_analysis_segments_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRealtimeContactAnalysisSegmentsV2Request, String> {
    let mut input = ListRealtimeContactAnalysisSegmentsV2Request::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<ListRealtimeContactAnalysisSegmentsV2Request>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize ListRealtimeContactAnalysisSegmentsV2 request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_routing_profile_manual_assignment_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoutingProfileManualAssignmentQueuesRequest, String> {
    let mut input = ListRoutingProfileManualAssignmentQueuesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_routing_profile_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoutingProfileQueuesRequest, String> {
    let mut input = ListRoutingProfileQueuesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_routing_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoutingProfilesRequest, String> {
    let mut input = ListRoutingProfilesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRulesRequest, String> {
    let mut input = ListRulesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("eventSourceName") {
        input.event_source_name = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("publishStatus") {
        input.publish_status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityKeysRequest, String> {
    let mut input = ListSecurityKeysRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_profile_applications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityProfileApplicationsRequest, String> {
    let mut input = ListSecurityProfileApplicationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_profile_flow_modules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityProfileFlowModulesRequest, String> {
    let mut input = ListSecurityProfileFlowModulesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_profile_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityProfilePermissionsRequest, String> {
    let mut input = ListSecurityProfilePermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityProfilesRequest, String> {
    let mut input = ListSecurityProfilesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_task_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTaskTemplatesRequest, String> {
    let mut input = ListTaskTemplatesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_case_execution_records_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestCaseExecutionRecordsRequest, String> {
    let mut input = ListTestCaseExecutionRecordsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseExecutionId" => {
                input.test_case_execution_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_case_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestCaseExecutionsRequest, String> {
    let mut input = ListTestCaseExecutionsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("endTime") {
        input.end_time = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("startTime") {
        input.start_time = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    if let Some(value) = query.get("testCaseId") {
        input.test_case_id = Some(value.to_string());
    }
    if let Some(value) = query.get("testCaseName") {
        input.test_case_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_cases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestCasesRequest, String> {
    let mut input = ListTestCasesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_traffic_distribution_group_users_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficDistributionGroupUsersRequest, String> {
    let mut input = ListTrafficDistributionGroupUsersRequest::default();
    for (name, value) in labels {
        match *name {
            "TrafficDistributionGroupId" => {
                input.traffic_distribution_group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_traffic_distribution_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficDistributionGroupsRequest, String> {
    let mut input = ListTrafficDistributionGroupsRequest::default();
    if let Some(value) = query.get("instanceId") {
        input.instance_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_use_cases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUseCasesRequest, String> {
    let mut input = ListUseCasesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "IntegrationAssociationId" => {
                input.integration_association_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_user_hierarchy_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserHierarchyGroupsRequest, String> {
    let mut input = ListUserHierarchyGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_user_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserNotificationsRequest, String> {
    let mut input = ListUserNotificationsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_user_proficiencies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserProficienciesRequest, String> {
    let mut input = ListUserProficienciesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
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
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_view_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListViewVersionsRequest, String> {
    let mut input = ListViewVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_views_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListViewsRequest, String> {
    let mut input = ListViewsRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_workspace_media_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListWorkspaceMediaRequest, String> {
    let mut input = ListWorkspaceMediaRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_workspace_pages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListWorkspacePagesRequest, String> {
    let mut input = ListWorkspacePagesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_workspaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListWorkspacesRequest, String> {
    let mut input = ListWorkspacesRequest::default();
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_monitor_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<MonitorContactRequest, String> {
    let mut input = MonitorContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<MonitorContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize MonitorContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_pause_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PauseContactRequest, String> {
    let mut input = PauseContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PauseContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PauseContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_user_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutUserStatusRequest, String> {
    let mut input = PutUserStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutUserStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutUserStatus request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_release_phone_number_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReleasePhoneNumberRequest, String> {
    let mut input = ReleasePhoneNumberRequest::default();
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_replicate_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReplicateInstanceRequest, String> {
    let mut input = ReplicateInstanceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ReplicateInstanceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ReplicateInstance request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_resume_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResumeContactRequest, String> {
    let mut input = ResumeContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResumeContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ResumeContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_resume_contact_recording_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResumeContactRecordingRequest, String> {
    let mut input = ResumeContactRecordingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResumeContactRecordingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ResumeContactRecording request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_agent_statuses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchAgentStatusesRequest, String> {
    let mut input = SearchAgentStatusesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchAgentStatusesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchAgentStatuses request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_available_phone_numbers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchAvailablePhoneNumbersRequest, String> {
    let mut input = SearchAvailablePhoneNumbersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchAvailablePhoneNumbersRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SearchAvailablePhoneNumbers request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_contact_evaluations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchContactEvaluationsRequest, String> {
    let mut input = SearchContactEvaluationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchContactEvaluationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchContactEvaluations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_contact_flow_modules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchContactFlowModulesRequest, String> {
    let mut input = SearchContactFlowModulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchContactFlowModulesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchContactFlowModules request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_contact_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchContactFlowsRequest, String> {
    let mut input = SearchContactFlowsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchContactFlowsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchContactFlows request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_contacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchContactsRequest, String> {
    let mut input = SearchContactsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchContactsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchContacts request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_data_tables_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchDataTablesRequest, String> {
    let mut input = SearchDataTablesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchDataTablesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchDataTables request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_email_addresses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchEmailAddressesRequest, String> {
    let mut input = SearchEmailAddressesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchEmailAddressesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchEmailAddresses request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_evaluation_forms_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchEvaluationFormsRequest, String> {
    let mut input = SearchEvaluationFormsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchEvaluationFormsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchEvaluationForms request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_hours_of_operation_overrides_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchHoursOfOperationOverridesRequest, String> {
    let mut input = SearchHoursOfOperationOverridesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchHoursOfOperationOverridesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SearchHoursOfOperationOverrides request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_hours_of_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchHoursOfOperationsRequest, String> {
    let mut input = SearchHoursOfOperationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchHoursOfOperationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchHoursOfOperations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchNotificationsRequest, String> {
    let mut input = SearchNotificationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchNotificationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchNotifications request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_predefined_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchPredefinedAttributesRequest, String> {
    let mut input = SearchPredefinedAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchPredefinedAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SearchPredefinedAttributes request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_prompts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchPromptsRequest, String> {
    let mut input = SearchPromptsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchPromptsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchPrompts request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchQueuesRequest, String> {
    let mut input = SearchQueuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchQueuesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchQueues request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_quick_connects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchQuickConnectsRequest, String> {
    let mut input = SearchQuickConnectsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchQuickConnectsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchQuickConnects request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_resource_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchResourceTagsRequest, String> {
    let mut input = SearchResourceTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchResourceTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchResourceTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_routing_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchRoutingProfilesRequest, String> {
    let mut input = SearchRoutingProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchRoutingProfilesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchRoutingProfiles request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchSecurityProfilesRequest, String> {
    let mut input = SearchSecurityProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchSecurityProfilesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchSecurityProfiles request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_test_cases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchTestCasesRequest, String> {
    let mut input = SearchTestCasesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchTestCasesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchTestCases request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_user_hierarchy_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchUserHierarchyGroupsRequest, String> {
    let mut input = SearchUserHierarchyGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchUserHierarchyGroupsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchUserHierarchyGroups request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_users_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchUsersRequest, String> {
    let mut input = SearchUsersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchUsersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchUsers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_views_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchViewsRequest, String> {
    let mut input = SearchViewsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchViewsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchViews request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_vocabularies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchVocabulariesRequest, String> {
    let mut input = SearchVocabulariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchVocabulariesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchVocabularies request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_workspace_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchWorkspaceAssociationsRequest, String> {
    let mut input = SearchWorkspaceAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchWorkspaceAssociationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SearchWorkspaceAssociations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_workspaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchWorkspacesRequest, String> {
    let mut input = SearchWorkspacesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchWorkspacesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchWorkspaces request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_chat_integration_event_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendChatIntegrationEventRequest, String> {
    let mut input = SendChatIntegrationEventRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendChatIntegrationEventRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SendChatIntegrationEvent request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_outbound_email_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendOutboundEmailRequest, String> {
    let mut input = SendOutboundEmailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendOutboundEmailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SendOutboundEmail request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_attached_file_upload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAttachedFileUploadRequest, String> {
    let mut input = StartAttachedFileUploadRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAttachedFileUploadRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartAttachedFileUpload request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedResourceArn") {
        input.associated_resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_chat_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartChatContactRequest, String> {
    let mut input = StartChatContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartChatContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartChatContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_contact_evaluation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartContactEvaluationRequest, String> {
    let mut input = StartContactEvaluationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartContactEvaluationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartContactEvaluation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_contact_media_processing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartContactMediaProcessingRequest, String> {
    let mut input = StartContactMediaProcessingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartContactMediaProcessingRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartContactMediaProcessing request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_contact_recording_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartContactRecordingRequest, String> {
    let mut input = StartContactRecordingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartContactRecordingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartContactRecording request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_contact_streaming_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartContactStreamingRequest, String> {
    let mut input = StartContactStreamingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartContactStreamingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartContactStreaming request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_email_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartEmailContactRequest, String> {
    let mut input = StartEmailContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartEmailContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartEmailContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_outbound_chat_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartOutboundChatContactRequest, String> {
    let mut input = StartOutboundChatContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartOutboundChatContactRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartOutboundChatContact request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_outbound_email_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartOutboundEmailContactRequest, String> {
    let mut input = StartOutboundEmailContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartOutboundEmailContactRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartOutboundEmailContact request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_outbound_voice_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartOutboundVoiceContactRequest, String> {
    let mut input = StartOutboundVoiceContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartOutboundVoiceContactRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartOutboundVoiceContact request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_screen_sharing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartScreenSharingRequest, String> {
    let mut input = StartScreenSharingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartScreenSharingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartScreenSharing request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_task_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTaskContactRequest, String> {
    let mut input = StartTaskContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTaskContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartTaskContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_test_case_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTestCaseExecutionRequest, String> {
    let mut input = StartTestCaseExecutionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTestCaseExecutionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartTestCaseExecution request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_web_r_t_c_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartWebRTCContactRequest, String> {
    let mut input = StartWebRTCContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartWebRTCContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartWebRTCContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopContactRequest, String> {
    let mut input = StopContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_contact_media_processing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopContactMediaProcessingRequest, String> {
    let mut input = StopContactMediaProcessingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopContactMediaProcessingRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StopContactMediaProcessing request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_contact_recording_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopContactRecordingRequest, String> {
    let mut input = StopContactRecordingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopContactRecordingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopContactRecording request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_contact_streaming_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopContactStreamingRequest, String> {
    let mut input = StopContactStreamingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopContactStreamingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopContactStreaming request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_test_case_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopTestCaseExecutionRequest, String> {
    let mut input = StopTestCaseExecutionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopTestCaseExecutionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopTestCaseExecution request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseExecutionId" => {
                input.test_case_execution_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_submit_contact_evaluation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SubmitContactEvaluationRequest, String> {
    let mut input = SubmitContactEvaluationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SubmitContactEvaluationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SubmitContactEvaluation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EvaluationId" => {
                input.evaluation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_suspend_contact_recording_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SuspendContactRecordingRequest, String> {
    let mut input = SuspendContactRecordingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SuspendContactRecordingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SuspendContactRecording request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagContactRequest, String> {
    let mut input = TagContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagContact request: {err}"))?;
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_transfer_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TransferContactRequest, String> {
    let mut input = TransferContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TransferContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TransferContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagContactRequest, String> {
    let mut input = UntagContactRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("TagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentStatusRequest, String> {
    let mut input = UpdateAgentStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAgentStatus request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AgentStatusId" => {
                input.agent_status_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_attached_files_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAttachedFilesConfigurationRequest, String> {
    let mut input = UpdateAttachedFilesConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAttachedFilesConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAttachedFilesConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AttachmentScope" => {
                input.attachment_scope = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_authentication_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAuthenticationProfileRequest, String> {
    let mut input = UpdateAuthenticationProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAuthenticationProfileRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAuthenticationProfile request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AuthenticationProfileId" => {
                input.authentication_profile_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactRequest, String> {
    let mut input = UpdateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateContact request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactAttributesRequest, String> {
    let mut input = UpdateContactAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactAttributesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateContactAttributes request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_evaluation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactEvaluationRequest, String> {
    let mut input = UpdateContactEvaluationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactEvaluationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateContactEvaluation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EvaluationId" => {
                input.evaluation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_content_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowContentRequest, String> {
    let mut input = UpdateContactFlowContentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowContentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateContactFlowContent request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowMetadataRequest, String> {
    let mut input = UpdateContactFlowMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowMetadataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateContactFlowMetadata request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_module_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowModuleAliasRequest, String> {
    let mut input = UpdateContactFlowModuleAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowModuleAliasRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateContactFlowModuleAlias request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AliasId" => {
                input.alias_id = value.to_string();
            }
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_module_content_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowModuleContentRequest, String> {
    let mut input = UpdateContactFlowModuleContentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowModuleContentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateContactFlowModuleContent request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_module_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowModuleMetadataRequest, String> {
    let mut input = UpdateContactFlowModuleMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowModuleMetadataRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateContactFlowModuleMetadata request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowModuleId" => {
                input.contact_flow_module_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_flow_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactFlowNameRequest, String> {
    let mut input = UpdateContactFlowNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactFlowNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateContactFlowName request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactFlowId" => {
                input.contact_flow_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_routing_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactRoutingDataRequest, String> {
    let mut input = UpdateContactRoutingDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactRoutingDataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateContactRoutingData request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactScheduleRequest, String> {
    let mut input = UpdateContactScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactScheduleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateContactSchedule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_table_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataTableAttributeRequest, String> {
    let mut input = UpdateDataTableAttributeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataTableAttributeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDataTableAttribute request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AttributeName" => {
                input.attribute_name = value.to_string();
            }
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_table_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataTableMetadataRequest, String> {
    let mut input = UpdateDataTableMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataTableMetadataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDataTableMetadata request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_table_primary_values_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataTablePrimaryValuesRequest, String> {
    let mut input = UpdateDataTablePrimaryValuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataTablePrimaryValuesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDataTablePrimaryValues request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DataTableId" => {
                input.data_table_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_email_address_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEmailAddressMetadataRequest, String> {
    let mut input = UpdateEmailAddressMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEmailAddressMetadataRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateEmailAddressMetadata request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailAddressId" => {
                input.email_address_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_evaluation_form_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEvaluationFormRequest, String> {
    let mut input = UpdateEvaluationFormRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEvaluationFormRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateEvaluationForm request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "EvaluationFormId" => {
                input.evaluation_form_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_hours_of_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateHoursOfOperationRequest, String> {
    let mut input = UpdateHoursOfOperationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateHoursOfOperationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateHoursOfOperation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_hours_of_operation_override_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateHoursOfOperationOverrideRequest, String> {
    let mut input = UpdateHoursOfOperationOverrideRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateHoursOfOperationOverrideRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateHoursOfOperationOverride request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HoursOfOperationId" => {
                input.hours_of_operation_id = value.to_string();
            }
            "HoursOfOperationOverrideId" => {
                input.hours_of_operation_override_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_instance_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInstanceAttributeRequest, String> {
    let mut input = UpdateInstanceAttributeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInstanceAttributeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateInstanceAttribute request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AttributeType" => {
                input.attribute_type = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_instance_storage_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInstanceStorageConfigRequest, String> {
    let mut input = UpdateInstanceStorageConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInstanceStorageConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateInstanceStorageConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AssociationId" => {
                input.association_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_notification_content_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNotificationContentRequest, String> {
    let mut input = UpdateNotificationContentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNotificationContentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateNotificationContent request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "NotificationId" => {
                input.notification_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_participant_authentication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateParticipantAuthenticationRequest, String> {
    let mut input = UpdateParticipantAuthenticationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateParticipantAuthenticationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateParticipantAuthentication request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_participant_role_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateParticipantRoleConfigRequest, String> {
    let mut input = UpdateParticipantRoleConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateParticipantRoleConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateParticipantRoleConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ContactId" => {
                input.contact_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_phone_number_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePhoneNumberRequest, String> {
    let mut input = UpdatePhoneNumberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePhoneNumberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePhoneNumber request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_phone_number_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePhoneNumberMetadataRequest, String> {
    let mut input = UpdatePhoneNumberMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePhoneNumberMetadataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdatePhoneNumberMetadata request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "PhoneNumberId" => {
                input.phone_number_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_predefined_attribute_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePredefinedAttributeRequest, String> {
    let mut input = UpdatePredefinedAttributeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePredefinedAttributeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdatePredefinedAttribute request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePromptRequest, String> {
    let mut input = UpdatePromptRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePromptRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePrompt request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "PromptId" => {
                input.prompt_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_hours_of_operation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueHoursOfOperationRequest, String> {
    let mut input = UpdateQueueHoursOfOperationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueHoursOfOperationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateQueueHoursOfOperation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_max_contacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueMaxContactsRequest, String> {
    let mut input = UpdateQueueMaxContactsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueMaxContactsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateQueueMaxContacts request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueNameRequest, String> {
    let mut input = UpdateQueueNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateQueueName request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_outbound_caller_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueOutboundCallerConfigRequest, String> {
    let mut input = UpdateQueueOutboundCallerConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueOutboundCallerConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateQueueOutboundCallerConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_outbound_email_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueOutboundEmailConfigRequest, String> {
    let mut input = UpdateQueueOutboundEmailConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueOutboundEmailConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateQueueOutboundEmailConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_queue_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueueStatusRequest, String> {
    let mut input = UpdateQueueStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueueStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateQueueStatus request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QueueId" => {
                input.queue_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_quick_connect_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQuickConnectConfigRequest, String> {
    let mut input = UpdateQuickConnectConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQuickConnectConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateQuickConnectConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QuickConnectId" => {
                input.quick_connect_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_quick_connect_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQuickConnectNameRequest, String> {
    let mut input = UpdateQuickConnectNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQuickConnectNameRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateQuickConnectName request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "QuickConnectId" => {
                input.quick_connect_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_routing_profile_agent_availability_timer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoutingProfileAgentAvailabilityTimerRequest, String> {
    let mut input = UpdateRoutingProfileAgentAvailabilityTimerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoutingProfileAgentAvailabilityTimerRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize UpdateRoutingProfileAgentAvailabilityTimer request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_routing_profile_concurrency_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoutingProfileConcurrencyRequest, String> {
    let mut input = UpdateRoutingProfileConcurrencyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoutingProfileConcurrencyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRoutingProfileConcurrency request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_routing_profile_default_outbound_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoutingProfileDefaultOutboundQueueRequest, String> {
    let mut input = UpdateRoutingProfileDefaultOutboundQueueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoutingProfileDefaultOutboundQueueRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize UpdateRoutingProfileDefaultOutboundQueue request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_routing_profile_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoutingProfileNameRequest, String> {
    let mut input = UpdateRoutingProfileNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoutingProfileNameRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateRoutingProfileName request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_routing_profile_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoutingProfileQueuesRequest, String> {
    let mut input = UpdateRoutingProfileQueuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoutingProfileQueuesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRoutingProfileQueues request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RoutingProfileId" => {
                input.routing_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRuleRequest, String> {
    let mut input = UpdateRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "RuleId" => {
                input.rule_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSecurityProfileRequest, String> {
    let mut input = UpdateSecurityProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSecurityProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSecurityProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "SecurityProfileId" => {
                input.security_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_task_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTaskTemplateRequest, String> {
    let mut input = UpdateTaskTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTaskTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTaskTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TaskTemplateId" => {
                input.task_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTestCaseRequest, String> {
    let mut input = UpdateTestCaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTestCaseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTestCase request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "TestCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-region")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_region = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-time")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_time = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_traffic_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrafficDistributionRequest, String> {
    let mut input = UpdateTrafficDistributionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTrafficDistributionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateTrafficDistribution request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserConfigRequest, String> {
    let mut input = UpdateUserConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUserConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_hierarchy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserHierarchyRequest, String> {
    let mut input = UpdateUserHierarchyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserHierarchyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUserHierarchy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_hierarchy_group_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserHierarchyGroupNameRequest, String> {
    let mut input = UpdateUserHierarchyGroupNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserHierarchyGroupNameRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserHierarchyGroupName request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HierarchyGroupId" => {
                input.hierarchy_group_id = value.to_string();
            }
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_hierarchy_structure_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserHierarchyStructureRequest, String> {
    let mut input = UpdateUserHierarchyStructureRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserHierarchyStructureRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserHierarchyStructure request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_identity_info_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserIdentityInfoRequest, String> {
    let mut input = UpdateUserIdentityInfoRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserIdentityInfoRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateUserIdentityInfo request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_notification_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserNotificationStatusRequest, String> {
    let mut input = UpdateUserNotificationStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserNotificationStatusRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserNotificationStatus request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "NotificationId" => {
                input.notification_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-region")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_region = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-last-modified-time")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified_time = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_phone_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserPhoneConfigRequest, String> {
    let mut input = UpdateUserPhoneConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserPhoneConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUserPhoneConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_proficiencies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserProficienciesRequest, String> {
    let mut input = UpdateUserProficienciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserProficienciesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateUserProficiencies request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_routing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserRoutingProfileRequest, String> {
    let mut input = UpdateUserRoutingProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserRoutingProfileRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateUserRoutingProfile request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_security_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserSecurityProfilesRequest, String> {
    let mut input = UpdateUserSecurityProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserSecurityProfilesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserSecurityProfiles request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "UserId" => {
                input.user_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_view_content_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateViewContentRequest, String> {
    let mut input = UpdateViewContentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateViewContentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateViewContent request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_view_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateViewMetadataRequest, String> {
    let mut input = UpdateViewMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateViewMetadataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateViewMetadata request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "ViewId" => {
                input.view_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspaceMetadataRequest, String> {
    let mut input = UpdateWorkspaceMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspaceMetadataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateWorkspaceMetadata request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspacePageRequest, String> {
    let mut input = UpdateWorkspacePageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspacePageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateWorkspacePage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "Page" => {
                input.page = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_theme_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspaceThemeRequest, String> {
    let mut input = UpdateWorkspaceThemeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspaceThemeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateWorkspaceTheme request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_visibility_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspaceVisibilityRequest, String> {
    let mut input = UpdateWorkspaceVisibilityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspaceVisibilityRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateWorkspaceVisibility request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InstanceId" => {
                input.instance_id = value.to_string();
            }
            "WorkspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
