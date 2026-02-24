//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-iot

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

/// Serialize void response for restJson protocol.
pub fn serialize_accept_certificate_transfer_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_add_thing_to_billing_group_response(
    result: &AddThingToBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_thing_to_thing_group_response(
    result: &AddThingToThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_sbom_with_package_version_response(
    result: &AssociateSbomWithPackageVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_targets_with_job_response(
    result: &AssociateTargetsWithJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_attach_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_attach_principal_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_security_profile_response(
    result: &AttachSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_thing_principal_response(
    result: &AttachThingPrincipalResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_audit_mitigation_actions_task_response(
    result: &CancelAuditMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_audit_task_response(result: &CancelAuditTaskResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_cancel_certificate_transfer_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_detect_mitigation_actions_task_response(
    result: &CancelDetectMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_job_response(result: &CancelJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_cancel_job_execution_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_clear_default_authorizer_response(
    result: &ClearDefaultAuthorizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_confirm_topic_rule_destination_response(
    result: &ConfirmTopicRuleDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_audit_suppression_response(
    result: &CreateAuditSuppressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_authorizer_response(result: &CreateAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_billing_group_response(
    result: &CreateBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_certificate_from_csr_response(
    result: &CreateCertificateFromCsrResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_certificate_provider_response(
    result: &CreateCertificateProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_command_response(result: &CreateCommandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_metric_response(
    result: &CreateCustomMetricResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_dimension_response(result: &CreateDimensionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_configuration_response(
    result: &CreateDomainConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_dynamic_thing_group_response(
    result: &CreateDynamicThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_fleet_metric_response(result: &CreateFleetMetricResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_job_response(result: &CreateJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_job_template_response(result: &CreateJobTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_keys_and_certificate_response(
    result: &CreateKeysAndCertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_mitigation_action_response(
    result: &CreateMitigationActionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_o_t_a_update_response(result: &CreateOTAUpdateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_package_response(result: &CreatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_package_version_response(
    result: &CreatePackageVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_policy_response(result: &CreatePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_policy_version_response(
    result: &CreatePolicyVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_provisioning_claim_response(
    result: &CreateProvisioningClaimResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_provisioning_template_response(
    result: &CreateProvisioningTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_provisioning_template_version_response(
    result: &CreateProvisioningTemplateVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_role_alias_response(result: &CreateRoleAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_scheduled_audit_response(
    result: &CreateScheduledAuditResponse,
) -> MockResponse {
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
pub fn serialize_create_stream_response(result: &CreateStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_thing_response(result: &CreateThingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_thing_group_response(result: &CreateThingGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_thing_type_response(result: &CreateThingTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_topic_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_topic_rule_destination_response(
    result: &CreateTopicRuleDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_account_audit_configuration_response(
    result: &DeleteAccountAuditConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_audit_suppression_response(
    result: &DeleteAuditSuppressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_authorizer_response(result: &DeleteAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_billing_group_response(
    result: &DeleteBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_c_a_certificate_response(
    result: &DeleteCACertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_certificate_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_certificate_provider_response(
    result: &DeleteCertificateProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_command_response(result: &DeleteCommandResponse) -> MockResponse {
    let status = result.status_code.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_command_execution_response(
    result: &DeleteCommandExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_metric_response(
    result: &DeleteCustomMetricResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_dimension_response(result: &DeleteDimensionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_domain_configuration_response(
    result: &DeleteDomainConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_dynamic_thing_group_response(
    result: &DeleteDynamicThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_fleet_metric_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_job_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_job_execution_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_job_template_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_mitigation_action_response(
    result: &DeleteMitigationActionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_o_t_a_update_response(result: &DeleteOTAUpdateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_package_response(result: &DeletePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_package_version_response(
    result: &DeletePackageVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_policy_version_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_provisioning_template_response(
    result: &DeleteProvisioningTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_provisioning_template_version_response(
    result: &DeleteProvisioningTemplateVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_registration_code_response(
    result: &DeleteRegistrationCodeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_role_alias_response(result: &DeleteRoleAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_scheduled_audit_response(
    result: &DeleteScheduledAuditResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_security_profile_response(
    result: &DeleteSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_stream_response(result: &DeleteStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_thing_response(result: &DeleteThingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_thing_group_response(result: &DeleteThingGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_thing_type_response(result: &DeleteThingTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_topic_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_topic_rule_destination_response(
    result: &DeleteTopicRuleDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_v2_logging_level_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_deprecate_thing_type_response(
    result: &DeprecateThingTypeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_audit_configuration_response(
    result: &DescribeAccountAuditConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_audit_finding_response(
    result: &DescribeAuditFindingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_audit_mitigation_actions_task_response(
    result: &DescribeAuditMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_audit_suppression_response(
    result: &DescribeAuditSuppressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_audit_task_response(result: &DescribeAuditTaskResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_authorizer_response(result: &DescribeAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_billing_group_response(
    result: &DescribeBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_c_a_certificate_response(
    result: &DescribeCACertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_certificate_response(
    result: &DescribeCertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_certificate_provider_response(
    result: &DescribeCertificateProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_custom_metric_response(
    result: &DescribeCustomMetricResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_default_authorizer_response(
    result: &DescribeDefaultAuthorizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_detect_mitigation_actions_task_response(
    result: &DescribeDetectMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dimension_response(result: &DescribeDimensionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_configuration_response(
    result: &DescribeDomainConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_encryption_configuration_response(
    result: &DescribeEncryptionConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_endpoint_response(result: &DescribeEndpointResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_event_configurations_response(
    result: &DescribeEventConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_fleet_metric_response(
    result: &DescribeFleetMetricResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_index_response(result: &DescribeIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_response(result: &DescribeJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_execution_response(
    result: &DescribeJobExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_template_response(
    result: &DescribeJobTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_managed_job_template_response(
    result: &DescribeManagedJobTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_mitigation_action_response(
    result: &DescribeMitigationActionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_provisioning_template_response(
    result: &DescribeProvisioningTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_provisioning_template_version_response(
    result: &DescribeProvisioningTemplateVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_role_alias_response(result: &DescribeRoleAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_scheduled_audit_response(
    result: &DescribeScheduledAuditResponse,
) -> MockResponse {
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
pub fn serialize_describe_stream_response(result: &DescribeStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_thing_response(result: &DescribeThingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_thing_group_response(
    result: &DescribeThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_thing_registration_task_response(
    result: &DescribeThingRegistrationTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_thing_type_response(result: &DescribeThingTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_detach_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_detach_principal_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_detach_security_profile_response(
    result: &DetachSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_detach_thing_principal_response(
    result: &DetachThingPrincipalResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disable_topic_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_sbom_from_package_version_response(
    result: &DisassociateSbomFromPackageVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_enable_topic_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_behavior_model_training_summaries_response(
    result: &GetBehaviorModelTrainingSummariesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_buckets_aggregation_response(
    result: &GetBucketsAggregationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cardinality_response(result: &GetCardinalityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_command_response(result: &GetCommandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_command_execution_response(
    result: &GetCommandExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_effective_policies_response(
    result: &GetEffectivePoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_indexing_configuration_response(
    result: &GetIndexingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_job_document_response(result: &GetJobDocumentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_logging_options_response(result: &GetLoggingOptionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_o_t_a_update_response(result: &GetOTAUpdateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_package_response(result: &GetPackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_package_configuration_response(
    result: &GetPackageConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_package_version_response(result: &GetPackageVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_percentiles_response(result: &GetPercentilesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_policy_response(result: &GetPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_policy_version_response(result: &GetPolicyVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_registration_code_response(
    result: &GetRegistrationCodeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_statistics_response(result: &GetStatisticsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_thing_connectivity_data_response(
    result: &GetThingConnectivityDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_topic_rule_response(result: &GetTopicRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_topic_rule_destination_response(
    result: &GetTopicRuleDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_v2_logging_options_response(
    result: &GetV2LoggingOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_active_violations_response(
    result: &ListActiveViolationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attached_policies_response(
    result: &ListAttachedPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_findings_response(result: &ListAuditFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_mitigation_actions_executions_response(
    result: &ListAuditMitigationActionsExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_mitigation_actions_tasks_response(
    result: &ListAuditMitigationActionsTasksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_suppressions_response(
    result: &ListAuditSuppressionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_tasks_response(result: &ListAuditTasksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_authorizers_response(result: &ListAuthorizersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_billing_groups_response(result: &ListBillingGroupsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_c_a_certificates_response(
    result: &ListCACertificatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_certificate_providers_response(
    result: &ListCertificateProvidersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_certificates_response(result: &ListCertificatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_certificates_by_c_a_response(
    result: &ListCertificatesByCAResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_command_executions_response(
    result: &ListCommandExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_commands_response(result: &ListCommandsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_metrics_response(result: &ListCustomMetricsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_detect_mitigation_actions_executions_response(
    result: &ListDetectMitigationActionsExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_detect_mitigation_actions_tasks_response(
    result: &ListDetectMitigationActionsTasksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_dimensions_response(result: &ListDimensionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_configurations_response(
    result: &ListDomainConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_fleet_metrics_response(result: &ListFleetMetricsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_indices_response(result: &ListIndicesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_job_executions_for_job_response(
    result: &ListJobExecutionsForJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_job_executions_for_thing_response(
    result: &ListJobExecutionsForThingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_job_templates_response(result: &ListJobTemplatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_jobs_response(result: &ListJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_managed_job_templates_response(
    result: &ListManagedJobTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_metric_values_response(result: &ListMetricValuesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_mitigation_actions_response(
    result: &ListMitigationActionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_o_t_a_updates_response(result: &ListOTAUpdatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_outgoing_certificates_response(
    result: &ListOutgoingCertificatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_package_versions_response(
    result: &ListPackageVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_packages_response(result: &ListPackagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_policies_response(result: &ListPoliciesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_policy_principals_response(
    result: &ListPolicyPrincipalsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_policy_versions_response(
    result: &ListPolicyVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_principal_policies_response(
    result: &ListPrincipalPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_principal_things_response(
    result: &ListPrincipalThingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_principal_things_v2_response(
    result: &ListPrincipalThingsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_provisioning_template_versions_response(
    result: &ListProvisioningTemplateVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_provisioning_templates_response(
    result: &ListProvisioningTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_related_resources_for_audit_finding_response(
    result: &ListRelatedResourcesForAuditFindingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_role_aliases_response(result: &ListRoleAliasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sbom_validation_results_response(
    result: &ListSbomValidationResultsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scheduled_audits_response(
    result: &ListScheduledAuditsResponse,
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
pub fn serialize_list_security_profiles_for_target_response(
    result: &ListSecurityProfilesForTargetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_streams_response(result: &ListStreamsResponse) -> MockResponse {
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
pub fn serialize_list_targets_for_policy_response(
    result: &ListTargetsForPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_targets_for_security_profile_response(
    result: &ListTargetsForSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_groups_response(result: &ListThingGroupsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_groups_for_thing_response(
    result: &ListThingGroupsForThingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_principals_response(
    result: &ListThingPrincipalsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_principals_v2_response(
    result: &ListThingPrincipalsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_registration_task_reports_response(
    result: &ListThingRegistrationTaskReportsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_registration_tasks_response(
    result: &ListThingRegistrationTasksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_thing_types_response(result: &ListThingTypesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_things_response(result: &ListThingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_things_in_billing_group_response(
    result: &ListThingsInBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_things_in_thing_group_response(
    result: &ListThingsInThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_topic_rule_destinations_response(
    result: &ListTopicRuleDestinationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_topic_rules_response(result: &ListTopicRulesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_v2_logging_levels_response(
    result: &ListV2LoggingLevelsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_violation_events_response(
    result: &ListViolationEventsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_verification_state_on_violation_response(
    result: &PutVerificationStateOnViolationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_c_a_certificate_response(
    result: &RegisterCACertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_certificate_response(
    result: &RegisterCertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_certificate_without_c_a_response(
    result: &RegisterCertificateWithoutCAResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_thing_response(result: &RegisterThingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_reject_certificate_transfer_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_thing_from_billing_group_response(
    result: &RemoveThingFromBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_thing_from_thing_group_response(
    result: &RemoveThingFromThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_replace_topic_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_search_index_response(result: &SearchIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_set_default_authorizer_response(
    result: &SetDefaultAuthorizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_default_policy_version_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_logging_options_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_v2_logging_level_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_v2_logging_options_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_start_audit_mitigation_actions_task_response(
    result: &StartAuditMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_detect_mitigation_actions_task_response(
    result: &StartDetectMitigationActionsTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_on_demand_audit_task_response(
    result: &StartOnDemandAuditTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_thing_registration_task_response(
    result: &StartThingRegistrationTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_thing_registration_task_response(
    result: &StopThingRegistrationTaskResponse,
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
pub fn serialize_test_authorization_response(result: &TestAuthorizationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_test_invoke_authorizer_response(
    result: &TestInvokeAuthorizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_transfer_certificate_response(
    result: &TransferCertificateResponse,
) -> MockResponse {
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
pub fn serialize_update_account_audit_configuration_response(
    result: &UpdateAccountAuditConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_audit_suppression_response(
    result: &UpdateAuditSuppressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_authorizer_response(result: &UpdateAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_billing_group_response(
    result: &UpdateBillingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_c_a_certificate_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_certificate_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_certificate_provider_response(
    result: &UpdateCertificateProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_command_response(result: &UpdateCommandResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_custom_metric_response(
    result: &UpdateCustomMetricResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dimension_response(result: &UpdateDimensionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_configuration_response(
    result: &UpdateDomainConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_dynamic_thing_group_response(
    result: &UpdateDynamicThingGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_encryption_configuration_response(
    result: &UpdateEncryptionConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_event_configurations_response(
    result: &UpdateEventConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_fleet_metric_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_indexing_configuration_response(
    result: &UpdateIndexingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_job_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_mitigation_action_response(
    result: &UpdateMitigationActionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_package_response(result: &UpdatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_package_configuration_response(
    result: &UpdatePackageConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_package_version_response(
    result: &UpdatePackageVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_provisioning_template_response(
    result: &UpdateProvisioningTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_role_alias_response(result: &UpdateRoleAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_scheduled_audit_response(
    result: &UpdateScheduledAuditResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_security_profile_response(
    result: &UpdateSecurityProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_stream_response(result: &UpdateStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_thing_response(result: &UpdateThingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_thing_group_response(result: &UpdateThingGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_thing_groups_for_thing_response(
    result: &UpdateThingGroupsForThingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_thing_type_response(result: &UpdateThingTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_topic_rule_destination_response(
    result: &UpdateTopicRuleDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_validate_security_profile_behaviors_response(
    result: &ValidateSecurityProfileBehaviorsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_certificate_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptCertificateTransferRequest, String> {
    let mut input = AcceptCertificateTransferRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("setAsActive") {
        input.set_as_active = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_thing_to_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddThingToBillingGroupRequest, String> {
    let mut input = AddThingToBillingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddThingToBillingGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AddThingToBillingGroup request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_thing_to_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddThingToThingGroupRequest, String> {
    let mut input = AddThingToThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddThingToThingGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddThingToThingGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_sbom_with_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateSbomWithPackageVersionRequest, String> {
    let mut input = AssociateSbomWithPackageVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateSbomWithPackageVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateSbomWithPackageVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
pub fn deserialize_associate_targets_with_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateTargetsWithJobRequest, String> {
    let mut input = AssociateTargetsWithJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateTargetsWithJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateTargetsWithJob request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachPolicyRequest, String> {
    let mut input = AttachPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AttachPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AttachPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_principal_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachPrincipalPolicyRequest, String> {
    let mut input = AttachPrincipalPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-iot-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachSecurityProfileRequest, String> {
    let mut input = AttachSecurityProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("securityProfileTargetArn") {
        input.security_profile_target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_thing_principal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachThingPrincipalRequest, String> {
    let mut input = AttachThingPrincipalRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("thingPrincipalType") {
        input.thing_principal_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_audit_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelAuditMitigationActionsTaskRequest, String> {
    let mut input = CancelAuditMitigationActionsTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_audit_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelAuditTaskRequest, String> {
    let mut input = CancelAuditTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_certificate_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelCertificateTransferRequest, String> {
    let mut input = CancelCertificateTransferRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_detect_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelDetectMitigationActionsTaskRequest, String> {
    let mut input = CancelDetectMitigationActionsTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelJobRequest, String> {
    let mut input = CancelJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_job_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelJobExecutionRequest, String> {
    let mut input = CancelJobExecutionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelJobExecutionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelJobExecution request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_clear_default_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ClearDefaultAuthorizerRequest, String> {
    let input = ClearDefaultAuthorizerRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_confirm_topic_rule_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ConfirmTopicRuleDestinationRequest, String> {
    let mut input = ConfirmTopicRuleDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "confirmationToken" => {
                input.confirmation_token = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_audit_suppression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAuditSuppressionRequest, String> {
    let mut input = CreateAuditSuppressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAuditSuppressionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAuditSuppression request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAuthorizerRequest, String> {
    let mut input = CreateAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "authorizerName" => {
                input.authorizer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBillingGroupRequest, String> {
    let mut input = CreateBillingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBillingGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBillingGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "billingGroupName" => {
                input.billing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_certificate_from_csr_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCertificateFromCsrRequest, String> {
    let mut input = CreateCertificateFromCsrRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCertificateFromCsrRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateCertificateFromCsr request: {err}"),
        )?;
    }
    if let Some(value) = query.get("setAsActive") {
        input.set_as_active = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_certificate_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCertificateProviderRequest, String> {
    let mut input = CreateCertificateProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCertificateProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateCertificateProvider request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "certificateProviderName" => {
                input.certificate_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_command_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCommandRequest, String> {
    let mut input = CreateCommandRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCommandRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCommand request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "commandId" => {
                input.command_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomMetricRequest, String> {
    let mut input = CreateCustomMetricRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCustomMetricRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCustomMetric request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_dimension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDimensionRequest, String> {
    let mut input = CreateDimensionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDimensionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDimension request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainConfigurationRequest, String> {
    let mut input = CreateDomainConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainConfigurationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDomainConfiguration request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "domainConfigurationName" => {
                input.domain_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_dynamic_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDynamicThingGroupRequest, String> {
    let mut input = CreateDynamicThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDynamicThingGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDynamicThingGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_fleet_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFleetMetricRequest, String> {
    let mut input = CreateFleetMetricRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFleetMetricRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFleetMetric request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateJobRequest, String> {
    let mut input = CreateJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateJobTemplateRequest, String> {
    let mut input = CreateJobTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateJobTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateJobTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobTemplateId" => {
                input.job_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_keys_and_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateKeysAndCertificateRequest, String> {
    let mut input = CreateKeysAndCertificateRequest::default();
    if let Some(value) = query.get("setAsActive") {
        input.set_as_active = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_mitigation_action_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMitigationActionRequest, String> {
    let mut input = CreateMitigationActionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMitigationActionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateMitigationAction request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "actionName" => {
                input.action_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_o_t_a_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateOTAUpdateRequest, String> {
    let mut input = CreateOTAUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateOTAUpdateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateOTAUpdate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "otaUpdateId" => {
                input.ota_update_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePackageRequest, String> {
    let mut input = CreatePackageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePackageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePackage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
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
pub fn deserialize_create_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePackageVersionRequest, String> {
    let mut input = CreatePackageVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePackageVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePackageVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
pub fn deserialize_create_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePolicyRequest, String> {
    let mut input = CreatePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePolicyVersionRequest, String> {
    let mut input = CreatePolicyVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePolicyVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePolicyVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("setAsDefault") {
        input.set_as_default = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_provisioning_claim_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProvisioningClaimRequest, String> {
    let mut input = CreateProvisioningClaimRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_provisioning_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProvisioningTemplateRequest, String> {
    let mut input = CreateProvisioningTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateProvisioningTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateProvisioningTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_provisioning_template_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProvisioningTemplateVersionRequest, String> {
    let mut input = CreateProvisioningTemplateVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateProvisioningTemplateVersionRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize CreateProvisioningTemplateVersion request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("setAsDefault") {
        input.set_as_default = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_role_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRoleAliasRequest, String> {
    let mut input = CreateRoleAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRoleAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRoleAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "roleAlias" => {
                input.role_alias = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_scheduled_audit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateScheduledAuditRequest, String> {
    let mut input = CreateScheduledAuditRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateScheduledAuditRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateScheduledAudit request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "scheduledAuditName" => {
                input.scheduled_audit_name = value.to_string();
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
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStreamRequest, String> {
    let mut input = CreateStreamRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateStreamRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateStream request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "streamId" => {
                input.stream_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThingRequest, String> {
    let mut input = CreateThingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThing request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThingGroupRequest, String> {
    let mut input = CreateThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThingGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThingGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_thing_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThingTypeRequest, String> {
    let mut input = CreateThingTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThingTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThingType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingTypeName" => {
                input.thing_type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicRuleRequest, String> {
    let mut input = CreateTopicRuleRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<TopicRulePayload>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTopicRule payload: {err}"))?;
        input.topic_rule_payload = payload;
    }
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-tagging")
        .and_then(|value| value.to_str().ok())
    {
        input.tags = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_topic_rule_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicRuleDestinationRequest, String> {
    let mut input = CreateTopicRuleDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTopicRuleDestinationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTopicRuleDestination request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_account_audit_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccountAuditConfigurationRequest, String> {
    let mut input = DeleteAccountAuditConfigurationRequest::default();
    if let Some(value) = query.get("deleteScheduledAudits") {
        input.delete_scheduled_audits = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_audit_suppression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAuditSuppressionRequest, String> {
    let mut input = DeleteAuditSuppressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAuditSuppressionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteAuditSuppression request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAuthorizerRequest, String> {
    let mut input = DeleteAuthorizerRequest::default();
    for (name, value) in labels {
        match *name {
            "authorizerName" => {
                input.authorizer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBillingGroupRequest, String> {
    let mut input = DeleteBillingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "billingGroupName" => {
                input.billing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_c_a_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCACertificateRequest, String> {
    let mut input = DeleteCACertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCertificateRequest, String> {
    let mut input = DeleteCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("forceDelete") {
        input.force_delete = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_certificate_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCertificateProviderRequest, String> {
    let mut input = DeleteCertificateProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateProviderName" => {
                input.certificate_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_command_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCommandRequest, String> {
    let mut input = DeleteCommandRequest::default();
    for (name, value) in labels {
        match *name {
            "commandId" => {
                input.command_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_command_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCommandExecutionRequest, String> {
    let mut input = DeleteCommandExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "executionId" => {
                input.execution_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("targetArn") {
        input.target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomMetricRequest, String> {
    let mut input = DeleteCustomMetricRequest::default();
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_dimension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDimensionRequest, String> {
    let mut input = DeleteDimensionRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainConfigurationRequest, String> {
    let mut input = DeleteDomainConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainConfigurationName" => {
                input.domain_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_dynamic_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDynamicThingGroupRequest, String> {
    let mut input = DeleteDynamicThingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_fleet_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFleetMetricRequest, String> {
    let mut input = DeleteFleetMetricRequest::default();
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobRequest, String> {
    let mut input = DeleteJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobExecutionRequest, String> {
    let mut input = DeleteJobExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "executionNumber" => {
                input.execution_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            "jobId" => {
                input.job_id = value.to_string();
            }
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobTemplateRequest, String> {
    let mut input = DeleteJobTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "jobTemplateId" => {
                input.job_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_mitigation_action_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMitigationActionRequest, String> {
    let mut input = DeleteMitigationActionRequest::default();
    for (name, value) in labels {
        match *name {
            "actionName" => {
                input.action_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_o_t_a_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteOTAUpdateRequest, String> {
    let mut input = DeleteOTAUpdateRequest::default();
    for (name, value) in labels {
        match *name {
            "otaUpdateId" => {
                input.ota_update_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deleteStream") {
        input.delete_stream = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("forceDeleteAWSJob") {
        input.force_delete_a_w_s_job = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePackageRequest, String> {
    let mut input = DeletePackageRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
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
pub fn deserialize_delete_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePackageVersionRequest, String> {
    let mut input = DeletePackageVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
pub fn deserialize_delete_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePolicyRequest, String> {
    let mut input = DeletePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePolicyVersionRequest, String> {
    let mut input = DeletePolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            "policyVersionId" => {
                input.policy_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_provisioning_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProvisioningTemplateRequest, String> {
    let mut input = DeleteProvisioningTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_provisioning_template_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProvisioningTemplateVersionRequest, String> {
    let mut input = DeleteProvisioningTemplateVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            "versionId" => {
                input.version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_registration_code_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRegistrationCodeRequest, String> {
    let input = DeleteRegistrationCodeRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_role_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoleAliasRequest, String> {
    let mut input = DeleteRoleAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "roleAlias" => {
                input.role_alias = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_scheduled_audit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduledAuditRequest, String> {
    let mut input = DeleteScheduledAuditRequest::default();
    for (name, value) in labels {
        match *name {
            "scheduledAuditName" => {
                input.scheduled_audit_name = value.to_string();
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
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStreamRequest, String> {
    let mut input = DeleteStreamRequest::default();
    for (name, value) in labels {
        match *name {
            "streamId" => {
                input.stream_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThingRequest, String> {
    let mut input = DeleteThingRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThingGroupRequest, String> {
    let mut input = DeleteThingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_thing_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThingTypeRequest, String> {
    let mut input = DeleteThingTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "thingTypeName" => {
                input.thing_type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicRuleRequest, String> {
    let mut input = DeleteTopicRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_topic_rule_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicRuleDestinationRequest, String> {
    let mut input = DeleteTopicRuleDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_v2_logging_level_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteV2LoggingLevelRequest, String> {
    let mut input = DeleteV2LoggingLevelRequest::default();
    if let Some(value) = query.get("targetName") {
        input.target_name = value.to_string();
    }
    if let Some(value) = query.get("targetType") {
        input.target_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deprecate_thing_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeprecateThingTypeRequest, String> {
    let mut input = DeprecateThingTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeprecateThingTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeprecateThingType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingTypeName" => {
                input.thing_type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_audit_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountAuditConfigurationRequest, String> {
    let input = DescribeAccountAuditConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_audit_finding_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuditFindingRequest, String> {
    let mut input = DescribeAuditFindingRequest::default();
    for (name, value) in labels {
        match *name {
            "findingId" => {
                input.finding_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_audit_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuditMitigationActionsTaskRequest, String> {
    let mut input = DescribeAuditMitigationActionsTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_audit_suppression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuditSuppressionRequest, String> {
    let mut input = DescribeAuditSuppressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAuditSuppressionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeAuditSuppression request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_audit_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuditTaskRequest, String> {
    let mut input = DescribeAuditTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuthorizerRequest, String> {
    let mut input = DescribeAuthorizerRequest::default();
    for (name, value) in labels {
        match *name {
            "authorizerName" => {
                input.authorizer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBillingGroupRequest, String> {
    let mut input = DescribeBillingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "billingGroupName" => {
                input.billing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_c_a_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCACertificateRequest, String> {
    let mut input = DescribeCACertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCertificateRequest, String> {
    let mut input = DescribeCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_certificate_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCertificateProviderRequest, String> {
    let mut input = DescribeCertificateProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateProviderName" => {
                input.certificate_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_custom_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCustomMetricRequest, String> {
    let mut input = DescribeCustomMetricRequest::default();
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_default_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDefaultAuthorizerRequest, String> {
    let input = DescribeDefaultAuthorizerRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_detect_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDetectMitigationActionsTaskRequest, String> {
    let mut input = DescribeDetectMitigationActionsTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dimension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDimensionRequest, String> {
    let mut input = DescribeDimensionRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainConfigurationRequest, String> {
    let mut input = DescribeDomainConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainConfigurationName" => {
                input.domain_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_encryption_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEncryptionConfigurationRequest, String> {
    let input = DescribeEncryptionConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEndpointRequest, String> {
    let mut input = DescribeEndpointRequest::default();
    if let Some(value) = query.get("endpointType") {
        input.endpoint_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_event_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventConfigurationsRequest, String> {
    let input = DescribeEventConfigurationsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_fleet_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFleetMetricRequest, String> {
    let mut input = DescribeFleetMetricRequest::default();
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIndexRequest, String> {
    let mut input = DescribeIndexRequest::default();
    for (name, value) in labels {
        match *name {
            "indexName" => {
                input.index_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobRequest, String> {
    let mut input = DescribeJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("beforeSubstitution") {
        input.before_substitution = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobExecutionRequest, String> {
    let mut input = DescribeJobExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("executionNumber") {
        input.execution_number = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobTemplateRequest, String> {
    let mut input = DescribeJobTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "jobTemplateId" => {
                input.job_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_managed_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeManagedJobTemplateRequest, String> {
    let mut input = DescribeManagedJobTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("templateVersion") {
        input.template_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_mitigation_action_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMitigationActionRequest, String> {
    let mut input = DescribeMitigationActionRequest::default();
    for (name, value) in labels {
        match *name {
            "actionName" => {
                input.action_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_provisioning_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeProvisioningTemplateRequest, String> {
    let mut input = DescribeProvisioningTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_provisioning_template_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeProvisioningTemplateVersionRequest, String> {
    let mut input = DescribeProvisioningTemplateVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            "versionId" => {
                input.version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_role_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRoleAliasRequest, String> {
    let mut input = DescribeRoleAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "roleAlias" => {
                input.role_alias = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_scheduled_audit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeScheduledAuditRequest, String> {
    let mut input = DescribeScheduledAuditRequest::default();
    for (name, value) in labels {
        match *name {
            "scheduledAuditName" => {
                input.scheduled_audit_name = value.to_string();
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
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeStreamRequest, String> {
    let mut input = DescribeStreamRequest::default();
    for (name, value) in labels {
        match *name {
            "streamId" => {
                input.stream_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThingRequest, String> {
    let mut input = DescribeThingRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThingGroupRequest, String> {
    let mut input = DescribeThingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_thing_registration_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThingRegistrationTaskRequest, String> {
    let mut input = DescribeThingRegistrationTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_thing_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThingTypeRequest, String> {
    let mut input = DescribeThingTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "thingTypeName" => {
                input.thing_type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachPolicyRequest, String> {
    let mut input = DetachPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DetachPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DetachPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_principal_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachPrincipalPolicyRequest, String> {
    let mut input = DetachPrincipalPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-iot-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachSecurityProfileRequest, String> {
    let mut input = DetachSecurityProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("securityProfileTargetArn") {
        input.security_profile_target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_thing_principal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachThingPrincipalRequest, String> {
    let mut input = DetachThingPrincipalRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableTopicRuleRequest, String> {
    let mut input = DisableTopicRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_sbom_from_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateSbomFromPackageVersionRequest, String> {
    let mut input = DisassociateSbomFromPackageVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
pub fn deserialize_enable_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableTopicRuleRequest, String> {
    let mut input = EnableTopicRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_behavior_model_training_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBehaviorModelTrainingSummariesRequest, String> {
    let mut input = GetBehaviorModelTrainingSummariesRequest::default();
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
    if let Some(value) = query.get("securityProfileName") {
        input.security_profile_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_buckets_aggregation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketsAggregationRequest, String> {
    let mut input = GetBucketsAggregationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBucketsAggregationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBucketsAggregation request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cardinality_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCardinalityRequest, String> {
    let mut input = GetCardinalityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCardinalityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCardinality request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_command_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCommandRequest, String> {
    let mut input = GetCommandRequest::default();
    for (name, value) in labels {
        match *name {
            "commandId" => {
                input.command_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_command_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCommandExecutionRequest, String> {
    let mut input = GetCommandExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "executionId" => {
                input.execution_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeResult") {
        input.include_result = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("targetArn") {
        input.target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_effective_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEffectivePoliciesRequest, String> {
    let mut input = GetEffectivePoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetEffectivePoliciesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetEffectivePolicies request: {err}"))?;
    }
    if let Some(value) = query.get("thingName") {
        input.thing_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_indexing_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIndexingConfigurationRequest, String> {
    let input = GetIndexingConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_job_document_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetJobDocumentRequest, String> {
    let mut input = GetJobDocumentRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("beforeSubstitution") {
        input.before_substitution = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_logging_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLoggingOptionsRequest, String> {
    let input = GetLoggingOptionsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_o_t_a_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOTAUpdateRequest, String> {
    let mut input = GetOTAUpdateRequest::default();
    for (name, value) in labels {
        match *name {
            "otaUpdateId" => {
                input.ota_update_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPackageRequest, String> {
    let mut input = GetPackageRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_package_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPackageConfigurationRequest, String> {
    let input = GetPackageConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPackageVersionRequest, String> {
    let mut input = GetPackageVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_percentiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPercentilesRequest, String> {
    let mut input = GetPercentilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPercentilesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetPercentiles request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPolicyRequest, String> {
    let mut input = GetPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPolicyVersionRequest, String> {
    let mut input = GetPolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            "policyVersionId" => {
                input.policy_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_registration_code_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRegistrationCodeRequest, String> {
    let input = GetRegistrationCodeRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStatisticsRequest, String> {
    let mut input = GetStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetStatistics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_thing_connectivity_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetThingConnectivityDataRequest, String> {
    let mut input = GetThingConnectivityDataRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTopicRuleRequest, String> {
    let mut input = GetTopicRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_topic_rule_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTopicRuleDestinationRequest, String> {
    let mut input = GetTopicRuleDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_v2_logging_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetV2LoggingOptionsRequest, String> {
    let mut input = GetV2LoggingOptionsRequest::default();
    if let Some(value) = query.get("verbose") {
        input.verbose = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_active_violations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListActiveViolationsRequest, String> {
    let mut input = ListActiveViolationsRequest::default();
    if let Some(value) = query.get("behaviorCriteriaType") {
        input.behavior_criteria_type = Some(value.to_string());
    }
    if let Some(value) = query.get("listSuppressedAlerts") {
        input.list_suppressed_alerts = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
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
    if let Some(value) = query.get("securityProfileName") {
        input.security_profile_name = Some(value.to_string());
    }
    if let Some(value) = query.get("thingName") {
        input.thing_name = Some(value.to_string());
    }
    if let Some(value) = query.get("verificationState") {
        input.verification_state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attached_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedPoliciesRequest, String> {
    let mut input = ListAttachedPoliciesRequest::default();
    for (name, value) in labels {
        match *name {
            "target" => {
                input.target = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("recursive") {
        input.recursive = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditFindingsRequest, String> {
    let mut input = ListAuditFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAuditFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAuditFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_mitigation_actions_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditMitigationActionsExecutionsRequest, String> {
    let mut input = ListAuditMitigationActionsExecutionsRequest::default();
    if let Some(value) = query.get("actionStatus") {
        input.action_status = Some(value.to_string());
    }
    if let Some(value) = query.get("findingId") {
        input.finding_id = value.to_string();
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
    if let Some(value) = query.get("taskId") {
        input.task_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_mitigation_actions_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditMitigationActionsTasksRequest, String> {
    let mut input = ListAuditMitigationActionsTasksRequest::default();
    if let Some(value) = query.get("auditTaskId") {
        input.audit_task_id = Some(value.to_string());
    }
    if let Some(value) = query.get("endTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("findingId") {
        input.finding_id = Some(value.to_string());
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
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("taskStatus") {
        input.task_status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_suppressions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditSuppressionsRequest, String> {
    let mut input = ListAuditSuppressionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAuditSuppressionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAuditSuppressions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditTasksRequest, String> {
    let mut input = ListAuditTasksRequest::default();
    if let Some(value) = query.get("endTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
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
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("taskStatus") {
        input.task_status = Some(value.to_string());
    }
    if let Some(value) = query.get("taskType") {
        input.task_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_authorizers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuthorizersRequest, String> {
    let mut input = ListAuthorizersRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_billing_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBillingGroupsRequest, String> {
    let mut input = ListBillingGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namePrefixFilter") {
        input.name_prefix_filter = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_c_a_certificates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCACertificatesRequest, String> {
    let mut input = ListCACertificatesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("templateName") {
        input.template_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_certificate_providers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCertificateProvidersRequest, String> {
    let mut input = ListCertificateProvidersRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_certificates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCertificatesRequest, String> {
    let mut input = ListCertificatesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_certificates_by_c_a_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCertificatesByCARequest, String> {
    let mut input = ListCertificatesByCARequest::default();
    for (name, value) in labels {
        match *name {
            "caCertificateId" => {
                input.ca_certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_command_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCommandExecutionsRequest, String> {
    let mut input = ListCommandExecutionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCommandExecutionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListCommandExecutions request: {err}"))?;
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
pub fn deserialize_list_commands_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCommandsRequest, String> {
    let mut input = ListCommandsRequest::default();
    if let Some(value) = query.get("commandParameterName") {
        input.command_parameter_name = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomMetricsRequest, String> {
    let mut input = ListCustomMetricsRequest::default();
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
pub fn deserialize_list_detect_mitigation_actions_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDetectMitigationActionsExecutionsRequest, String> {
    let mut input = ListDetectMitigationActionsExecutionsRequest::default();
    if let Some(value) = query.get("endTime") {
        input.end_time = Some(
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
    if let Some(value) = query.get("taskId") {
        input.task_id = Some(value.to_string());
    }
    if let Some(value) = query.get("thingName") {
        input.thing_name = Some(value.to_string());
    }
    if let Some(value) = query.get("violationId") {
        input.violation_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_detect_mitigation_actions_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDetectMitigationActionsTasksRequest, String> {
    let mut input = ListDetectMitigationActionsTasksRequest::default();
    if let Some(value) = query.get("endTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
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
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_dimensions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDimensionsRequest, String> {
    let mut input = ListDimensionsRequest::default();
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
pub fn deserialize_list_domain_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainConfigurationsRequest, String> {
    let mut input = ListDomainConfigurationsRequest::default();
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("serviceType") {
        input.service_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_fleet_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFleetMetricsRequest, String> {
    let mut input = ListFleetMetricsRequest::default();
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
pub fn deserialize_list_indices_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIndicesRequest, String> {
    let mut input = ListIndicesRequest::default();
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
pub fn deserialize_list_job_executions_for_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobExecutionsForJobRequest, String> {
    let mut input = ListJobExecutionsForJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
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
pub fn deserialize_list_job_executions_for_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobExecutionsForThingRequest, String> {
    let mut input = ListJobExecutionsForThingRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("jobId") {
        input.job_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
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
pub fn deserialize_list_job_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobTemplatesRequest, String> {
    let mut input = ListJobTemplatesRequest::default();
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
pub fn deserialize_list_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobsRequest, String> {
    let mut input = ListJobsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    if let Some(value) = query.get("targetSelection") {
        input.target_selection = Some(value.to_string());
    }
    if let Some(value) = query.get("thingGroupId") {
        input.thing_group_id = Some(value.to_string());
    }
    if let Some(value) = query.get("thingGroupName") {
        input.thing_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_managed_job_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListManagedJobTemplatesRequest, String> {
    let mut input = ListManagedJobTemplatesRequest::default();
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
    if let Some(value) = query.get("templateName") {
        input.template_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_metric_values_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMetricValuesRequest, String> {
    let mut input = ListMetricValuesRequest::default();
    if let Some(value) = query.get("dimensionName") {
        input.dimension_name = Some(value.to_string());
    }
    if let Some(value) = query.get("dimensionValueOperator") {
        input.dimension_value_operator = Some(value.to_string());
    }
    if let Some(value) = query.get("endTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("metricName") {
        input.metric_name = value.to_string();
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("startTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("thingName") {
        input.thing_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_mitigation_actions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMitigationActionsRequest, String> {
    let mut input = ListMitigationActionsRequest::default();
    if let Some(value) = query.get("actionType") {
        input.action_type = Some(value.to_string());
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
pub fn deserialize_list_o_t_a_updates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOTAUpdatesRequest, String> {
    let mut input = ListOTAUpdatesRequest::default();
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
    if let Some(value) = query.get("otaUpdateStatus") {
        input.ota_update_status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_outgoing_certificates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOutgoingCertificatesRequest, String> {
    let mut input = ListOutgoingCertificatesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_package_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPackageVersionsRequest, String> {
    let mut input = ListPackageVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
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
pub fn deserialize_list_packages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPackagesRequest, String> {
    let mut input = ListPackagesRequest::default();
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
pub fn deserialize_list_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPoliciesRequest, String> {
    let mut input = ListPoliciesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_policy_principals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyPrincipalsRequest, String> {
    let mut input = ListPolicyPrincipalsRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-iot-policy")
        .and_then(|value| value.to_str().ok())
    {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_policy_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyVersionsRequest, String> {
    let mut input = ListPolicyVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_principal_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPrincipalPoliciesRequest, String> {
    let mut input = ListPrincipalPoliciesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-iot-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_principal_things_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPrincipalThingsRequest, String> {
    let mut input = ListPrincipalThingsRequest::default();
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
    if let Some(value) = request
        .headers
        .get("x-amzn-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_principal_things_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPrincipalThingsV2Request, String> {
    let mut input = ListPrincipalThingsV2Request::default();
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
    if let Some(value) = query.get("thingPrincipalType") {
        input.thing_principal_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-principal")
        .and_then(|value| value.to_str().ok())
    {
        input.principal = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_provisioning_template_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProvisioningTemplateVersionsRequest, String> {
    let mut input = ListProvisioningTemplateVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
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
pub fn deserialize_list_provisioning_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProvisioningTemplatesRequest, String> {
    let mut input = ListProvisioningTemplatesRequest::default();
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
pub fn deserialize_list_related_resources_for_audit_finding_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRelatedResourcesForAuditFindingRequest, String> {
    let mut input = ListRelatedResourcesForAuditFindingRequest::default();
    if let Some(value) = query.get("findingId") {
        input.finding_id = value.to_string();
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
pub fn deserialize_list_role_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoleAliasesRequest, String> {
    let mut input = ListRoleAliasesRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_sbom_validation_results_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSbomValidationResultsRequest, String> {
    let mut input = ListSbomValidationResultsRequest::default();
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
    if let Some(value) = query.get("validationResult") {
        input.validation_result = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scheduled_audits_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScheduledAuditsRequest, String> {
    let mut input = ListScheduledAuditsRequest::default();
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
    if let Some(value) = query.get("dimensionName") {
        input.dimension_name = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("metricName") {
        input.metric_name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_profiles_for_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityProfilesForTargetRequest, String> {
    let mut input = ListSecurityProfilesForTargetRequest::default();
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
    if let Some(value) = query.get("recursive") {
        input.recursive = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("securityProfileTargetArn") {
        input.security_profile_target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_streams_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamsRequest, String> {
    let mut input = ListStreamsRequest::default();
    if let Some(value) = query.get("isAscendingOrder") {
        input.ascending_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
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
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_targets_for_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTargetsForPolicyRequest, String> {
    let mut input = ListTargetsForPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("pageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_targets_for_security_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTargetsForSecurityProfileRequest, String> {
    let mut input = ListTargetsForSecurityProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
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
pub fn deserialize_list_thing_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingGroupsRequest, String> {
    let mut input = ListThingGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namePrefixFilter") {
        input.name_prefix_filter = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("parentGroup") {
        input.parent_group = Some(value.to_string());
    }
    if let Some(value) = query.get("recursive") {
        input.recursive = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_thing_groups_for_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingGroupsForThingRequest, String> {
    let mut input = ListThingGroupsForThingRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
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
pub fn deserialize_list_thing_principals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingPrincipalsRequest, String> {
    let mut input = ListThingPrincipalsRequest::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
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
pub fn deserialize_list_thing_principals_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingPrincipalsV2Request, String> {
    let mut input = ListThingPrincipalsV2Request::default();
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
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
    if let Some(value) = query.get("thingPrincipalType") {
        input.thing_principal_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_thing_registration_task_reports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingRegistrationTaskReportsRequest, String> {
    let mut input = ListThingRegistrationTaskReportsRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
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
    if let Some(value) = query.get("reportType") {
        input.report_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_thing_registration_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingRegistrationTasksRequest, String> {
    let mut input = ListThingRegistrationTasksRequest::default();
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
pub fn deserialize_list_thing_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingTypesRequest, String> {
    let mut input = ListThingTypesRequest::default();
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
    if let Some(value) = query.get("thingTypeName") {
        input.thing_type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_things_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingsRequest, String> {
    let mut input = ListThingsRequest::default();
    if let Some(value) = query.get("attributeName") {
        input.attribute_name = Some(value.to_string());
    }
    if let Some(value) = query.get("attributeValue") {
        input.attribute_value = Some(value.to_string());
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
    if let Some(value) = query.get("thingTypeName") {
        input.thing_type_name = Some(value.to_string());
    }
    if let Some(value) = query.get("usePrefixAttributeValue") {
        input.use_prefix_attribute_value = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_things_in_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingsInBillingGroupRequest, String> {
    let mut input = ListThingsInBillingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "billingGroupName" => {
                input.billing_group_name = value.to_string();
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
pub fn deserialize_list_things_in_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThingsInThingGroupRequest, String> {
    let mut input = ListThingsInThingGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
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
    if let Some(value) = query.get("recursive") {
        input.recursive = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_topic_rule_destinations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicRuleDestinationsRequest, String> {
    let mut input = ListTopicRuleDestinationsRequest::default();
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
pub fn deserialize_list_topic_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTopicRulesRequest, String> {
    let mut input = ListTopicRulesRequest::default();
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
    if let Some(value) = query.get("ruleDisabled") {
        input.rule_disabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("topic") {
        input.topic = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_v2_logging_levels_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListV2LoggingLevelsRequest, String> {
    let mut input = ListV2LoggingLevelsRequest::default();
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
    if let Some(value) = query.get("targetType") {
        input.target_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_violation_events_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListViolationEventsRequest, String> {
    let mut input = ListViolationEventsRequest::default();
    if let Some(value) = query.get("behaviorCriteriaType") {
        input.behavior_criteria_type = Some(value.to_string());
    }
    if let Some(value) = query.get("endTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("listSuppressedAlerts") {
        input.list_suppressed_alerts = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
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
    if let Some(value) = query.get("securityProfileName") {
        input.security_profile_name = Some(value.to_string());
    }
    if let Some(value) = query.get("startTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("thingName") {
        input.thing_name = Some(value.to_string());
    }
    if let Some(value) = query.get("verificationState") {
        input.verification_state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_verification_state_on_violation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutVerificationStateOnViolationRequest, String> {
    let mut input = PutVerificationStateOnViolationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutVerificationStateOnViolationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutVerificationStateOnViolation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "violationId" => {
                input.violation_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_c_a_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterCACertificateRequest, String> {
    let mut input = RegisterCACertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterCACertificateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterCACertificate request: {err}"))?;
    }
    if let Some(value) = query.get("allowAutoRegistration") {
        input.allow_auto_registration = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("setAsActive") {
        input.set_as_active = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterCertificateRequest, String> {
    let mut input = RegisterCertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterCertificateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterCertificate request: {err}"))?;
    }
    if let Some(value) = query.get("setAsActive") {
        input.set_as_active = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_certificate_without_c_a_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterCertificateWithoutCARequest, String> {
    let mut input = RegisterCertificateWithoutCARequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterCertificateWithoutCARequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RegisterCertificateWithoutCA request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterThingRequest, String> {
    let mut input = RegisterThingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterThingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterThing request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_certificate_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectCertificateTransferRequest, String> {
    let mut input = RejectCertificateTransferRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RejectCertificateTransferRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RejectCertificateTransfer request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_thing_from_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveThingFromBillingGroupRequest, String> {
    let mut input = RemoveThingFromBillingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveThingFromBillingGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RemoveThingFromBillingGroup request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_thing_from_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveThingFromThingGroupRequest, String> {
    let mut input = RemoveThingFromThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveThingFromThingGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RemoveThingFromThingGroup request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_replace_topic_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReplaceTopicRuleRequest, String> {
    let mut input = ReplaceTopicRuleRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<TopicRulePayload>(&request.body)
            .map_err(|err| format!("failed to deserialize ReplaceTopicRule payload: {err}"))?;
        input.topic_rule_payload = payload;
    }
    for (name, value) in labels {
        match *name {
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchIndexRequest, String> {
    let mut input = SearchIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchIndex request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_default_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetDefaultAuthorizerRequest, String> {
    let mut input = SetDefaultAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SetDefaultAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SetDefaultAuthorizer request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_default_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetDefaultPolicyVersionRequest, String> {
    let mut input = SetDefaultPolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "policyName" => {
                input.policy_name = value.to_string();
            }
            "policyVersionId" => {
                input.policy_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_logging_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetLoggingOptionsRequest, String> {
    let mut input = SetLoggingOptionsRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<LoggingOptionsPayload>(&request.body)
            .map_err(|err| format!("failed to deserialize SetLoggingOptions payload: {err}"))?;
        input.logging_options_payload = payload;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_v2_logging_level_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetV2LoggingLevelRequest, String> {
    let mut input = SetV2LoggingLevelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SetV2LoggingLevelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SetV2LoggingLevel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_set_v2_logging_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SetV2LoggingOptionsRequest, String> {
    let mut input = SetV2LoggingOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SetV2LoggingOptionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SetV2LoggingOptions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_audit_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAuditMitigationActionsTaskRequest, String> {
    let mut input = StartAuditMitigationActionsTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAuditMitigationActionsTaskRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartAuditMitigationActionsTask request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_detect_mitigation_actions_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDetectMitigationActionsTaskRequest, String> {
    let mut input = StartDetectMitigationActionsTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDetectMitigationActionsTaskRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartDetectMitigationActionsTask request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_on_demand_audit_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartOnDemandAuditTaskRequest, String> {
    let mut input = StartOnDemandAuditTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartOnDemandAuditTaskRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartOnDemandAuditTask request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_thing_registration_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartThingRegistrationTaskRequest, String> {
    let mut input = StartThingRegistrationTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartThingRegistrationTaskRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartThingRegistrationTask request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_thing_registration_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopThingRegistrationTaskRequest, String> {
    let mut input = StopThingRegistrationTaskRequest::default();
    for (name, value) in labels {
        match *name {
            "taskId" => {
                input.task_id = value.to_string();
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_test_authorization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestAuthorizationRequest, String> {
    let mut input = TestAuthorizationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestAuthorizationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TestAuthorization request: {err}"))?;
    }
    if let Some(value) = query.get("clientId") {
        input.client_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_test_invoke_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestInvokeAuthorizerRequest, String> {
    let mut input = TestInvokeAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestInvokeAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TestInvokeAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "authorizerName" => {
                input.authorizer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_transfer_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TransferCertificateRequest, String> {
    let mut input = TransferCertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TransferCertificateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TransferCertificate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("targetAwsAccount") {
        input.target_aws_account = value.to_string();
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
pub fn deserialize_update_account_audit_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountAuditConfigurationRequest, String> {
    let mut input = UpdateAccountAuditConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountAuditConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAccountAuditConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_audit_suppression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAuditSuppressionRequest, String> {
    let mut input = UpdateAuditSuppressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAuditSuppressionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAuditSuppression request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAuthorizerRequest, String> {
    let mut input = UpdateAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "authorizerName" => {
                input.authorizer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_billing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBillingGroupRequest, String> {
    let mut input = UpdateBillingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBillingGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBillingGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "billingGroupName" => {
                input.billing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_c_a_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCACertificateRequest, String> {
    let mut input = UpdateCACertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCACertificateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCACertificate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("newAutoRegistrationStatus") {
        input.new_auto_registration_status = Some(value.to_string());
    }
    if let Some(value) = query.get("newStatus") {
        input.new_status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCertificateRequest, String> {
    let mut input = UpdateCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "certificateId" => {
                input.certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("newStatus") {
        input.new_status = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_certificate_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCertificateProviderRequest, String> {
    let mut input = UpdateCertificateProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCertificateProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateCertificateProvider request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "certificateProviderName" => {
                input.certificate_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_command_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCommandRequest, String> {
    let mut input = UpdateCommandRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCommandRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCommand request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "commandId" => {
                input.command_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_custom_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCustomMetricRequest, String> {
    let mut input = UpdateCustomMetricRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCustomMetricRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCustomMetric request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dimension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDimensionRequest, String> {
    let mut input = UpdateDimensionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDimensionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDimension request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_domain_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainConfigurationRequest, String> {
    let mut input = UpdateDomainConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDomainConfigurationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDomainConfiguration request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "domainConfigurationName" => {
                input.domain_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_dynamic_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDynamicThingGroupRequest, String> {
    let mut input = UpdateDynamicThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDynamicThingGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDynamicThingGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_encryption_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEncryptionConfigurationRequest, String> {
    let mut input = UpdateEncryptionConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEncryptionConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateEncryptionConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_event_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEventConfigurationsRequest, String> {
    let mut input = UpdateEventConfigurationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEventConfigurationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateEventConfigurations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_fleet_metric_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFleetMetricRequest, String> {
    let mut input = UpdateFleetMetricRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFleetMetricRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFleetMetric request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "metricName" => {
                input.metric_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_indexing_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIndexingConfigurationRequest, String> {
    let mut input = UpdateIndexingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIndexingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateIndexingConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateJobRequest, String> {
    let mut input = UpdateJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("namespaceId") {
        input.namespace_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_mitigation_action_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMitigationActionRequest, String> {
    let mut input = UpdateMitigationActionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMitigationActionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateMitigationAction request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "actionName" => {
                input.action_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePackageRequest, String> {
    let mut input = UpdatePackageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePackageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePackage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
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
pub fn deserialize_update_package_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePackageConfigurationRequest, String> {
    let mut input = UpdatePackageConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePackageConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePackageConfiguration request: {err}")
            })?;
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_package_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePackageVersionRequest, String> {
    let mut input = UpdatePackageVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePackageVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePackageVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "packageName" => {
                input.package_name = value.to_string();
            }
            "versionName" => {
                input.version_name = value.to_string();
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
pub fn deserialize_update_provisioning_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateProvisioningTemplateRequest, String> {
    let mut input = UpdateProvisioningTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateProvisioningTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateProvisioningTemplate request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "templateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_role_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoleAliasRequest, String> {
    let mut input = UpdateRoleAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRoleAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRoleAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "roleAlias" => {
                input.role_alias = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_scheduled_audit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateScheduledAuditRequest, String> {
    let mut input = UpdateScheduledAuditRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateScheduledAuditRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateScheduledAudit request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "scheduledAuditName" => {
                input.scheduled_audit_name = value.to_string();
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
            "securityProfileName" => {
                input.security_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedVersion") {
        input.expected_version = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStreamRequest, String> {
    let mut input = UpdateStreamRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStreamRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateStream request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "streamId" => {
                input.stream_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThingRequest, String> {
    let mut input = UpdateThingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThing request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_thing_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThingGroupRequest, String> {
    let mut input = UpdateThingGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThingGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThingGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingGroupName" => {
                input.thing_group_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_thing_groups_for_thing_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThingGroupsForThingRequest, String> {
    let mut input = UpdateThingGroupsForThingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThingGroupsForThingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateThingGroupsForThing request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_thing_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThingTypeRequest, String> {
    let mut input = UpdateThingTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThingTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThingType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "thingTypeName" => {
                input.thing_type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_topic_rule_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTopicRuleDestinationRequest, String> {
    let mut input = UpdateTopicRuleDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTopicRuleDestinationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateTopicRuleDestination request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_validate_security_profile_behaviors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ValidateSecurityProfileBehaviorsRequest, String> {
    let mut input = ValidateSecurityProfileBehaviorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ValidateSecurityProfileBehaviorsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ValidateSecurityProfileBehaviors request: {err}")
            })?;
    }
    Ok(input)
}
