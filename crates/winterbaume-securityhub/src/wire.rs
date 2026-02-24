//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-securityhub

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
pub fn serialize_accept_administrator_invitation_response(
    result: &AcceptAdministratorInvitationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_accept_invitation_response(result: &AcceptInvitationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_automation_rules_response(
    result: &BatchDeleteAutomationRulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_disable_standards_response(
    result: &BatchDisableStandardsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_enable_standards_response(
    result: &BatchEnableStandardsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_automation_rules_response(
    result: &BatchGetAutomationRulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_configuration_policy_associations_response(
    result: &BatchGetConfigurationPolicyAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_security_controls_response(
    result: &BatchGetSecurityControlsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_standards_control_associations_response(
    result: &BatchGetStandardsControlAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_import_findings_response(
    result: &BatchImportFindingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_automation_rules_response(
    result: &BatchUpdateAutomationRulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_findings_response(
    result: &BatchUpdateFindingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_findings_v2_response(
    result: &BatchUpdateFindingsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_standards_control_associations_response(
    result: &BatchUpdateStandardsControlAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_action_target_response(
    result: &CreateActionTargetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_aggregator_v2_response(
    result: &CreateAggregatorV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_automation_rule_response(
    result: &CreateAutomationRuleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_automation_rule_v2_response(
    result: &CreateAutomationRuleV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_policy_response(
    result: &CreateConfigurationPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connector_v2_response(result: &CreateConnectorV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_finding_aggregator_response(
    result: &CreateFindingAggregatorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_insight_response(result: &CreateInsightResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_members_response(result: &CreateMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_ticket_v2_response(result: &CreateTicketV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_decline_invitations_response(result: &DeclineInvitationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_action_target_response(
    result: &DeleteActionTargetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_aggregator_v2_response(
    result: &DeleteAggregatorV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_automation_rule_v2_response(
    result: &DeleteAutomationRuleV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_configuration_policy_response(
    result: &DeleteConfigurationPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_connector_v2_response(result: &DeleteConnectorV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_finding_aggregator_response(
    result: &DeleteFindingAggregatorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_insight_response(result: &DeleteInsightResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_invitations_response(result: &DeleteInvitationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_members_response(result: &DeleteMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_action_targets_response(
    result: &DescribeActionTargetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_hub_response(result: &DescribeHubResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_organization_configuration_response(
    result: &DescribeOrganizationConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_products_response(result: &DescribeProductsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_products_v2_response(
    result: &DescribeProductsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_security_hub_v2_response(
    result: &DescribeSecurityHubV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_standards_response(result: &DescribeStandardsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_standards_controls_response(
    result: &DescribeStandardsControlsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_import_findings_for_product_response(
    result: &DisableImportFindingsForProductResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_organization_admin_account_response(
    result: &DisableOrganizationAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_security_hub_response(
    result: &DisableSecurityHubResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_security_hub_v2_response(
    result: &DisableSecurityHubV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_from_administrator_account_response(
    result: &DisassociateFromAdministratorAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_from_master_account_response(
    result: &DisassociateFromMasterAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_members_response(
    result: &DisassociateMembersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_import_findings_for_product_response(
    result: &EnableImportFindingsForProductResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_organization_admin_account_response(
    result: &EnableOrganizationAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_security_hub_response(result: &EnableSecurityHubResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_security_hub_v2_response(
    result: &EnableSecurityHubV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_administrator_account_response(
    result: &GetAdministratorAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_aggregator_v2_response(result: &GetAggregatorV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automation_rule_v2_response(
    result: &GetAutomationRuleV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_policy_response(
    result: &GetConfigurationPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_policy_association_response(
    result: &GetConfigurationPolicyAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connector_v2_response(result: &GetConnectorV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_enabled_standards_response(
    result: &GetEnabledStandardsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_aggregator_response(
    result: &GetFindingAggregatorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_history_response(result: &GetFindingHistoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_statistics_v2_response(
    result: &GetFindingStatisticsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_response(result: &GetFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_trends_v2_response(
    result: &GetFindingsTrendsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_v2_response(result: &GetFindingsV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insight_results_response(result: &GetInsightResultsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insights_response(result: &GetInsightsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_invitations_count_response(
    result: &GetInvitationsCountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_master_account_response(result: &GetMasterAccountResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_members_response(result: &GetMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resources_statistics_v2_response(
    result: &GetResourcesStatisticsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resources_trends_v2_response(
    result: &GetResourcesTrendsV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resources_v2_response(result: &GetResourcesV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_security_control_definition_response(
    result: &GetSecurityControlDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_invite_members_response(result: &InviteMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_aggregators_v2_response(result: &ListAggregatorsV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automation_rules_response(
    result: &ListAutomationRulesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automation_rules_v2_response(
    result: &ListAutomationRulesV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_policies_response(
    result: &ListConfigurationPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_policy_associations_response(
    result: &ListConfigurationPolicyAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connectors_v2_response(result: &ListConnectorsV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_enabled_products_for_import_response(
    result: &ListEnabledProductsForImportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_finding_aggregators_response(
    result: &ListFindingAggregatorsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_invitations_response(result: &ListInvitationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_members_response(result: &ListMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_organization_admin_accounts_response(
    result: &ListOrganizationAdminAccountsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_control_definitions_response(
    result: &ListSecurityControlDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_standards_control_associations_response(
    result: &ListStandardsControlAssociationsResponse,
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
pub fn serialize_register_connector_v2_response(
    result: &RegisterConnectorV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_configuration_policy_association_response(
    result: &StartConfigurationPolicyAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_configuration_policy_disassociation_response(
    result: &StartConfigurationPolicyDisassociationResponse,
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
pub fn serialize_update_action_target_response(
    result: &UpdateActionTargetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_aggregator_v2_response(
    result: &UpdateAggregatorV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_automation_rule_v2_response(
    result: &UpdateAutomationRuleV2Response,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_policy_response(
    result: &UpdateConfigurationPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_connector_v2_response(result: &UpdateConnectorV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_finding_aggregator_response(
    result: &UpdateFindingAggregatorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_findings_response(result: &UpdateFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_insight_response(result: &UpdateInsightResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_organization_configuration_response(
    result: &UpdateOrganizationConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_security_control_response(
    result: &UpdateSecurityControlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_security_hub_configuration_response(
    result: &UpdateSecurityHubConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_standards_control_response(
    result: &UpdateStandardsControlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_administrator_invitation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptAdministratorInvitationRequest, String> {
    let mut input = AcceptAdministratorInvitationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AcceptAdministratorInvitationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AcceptAdministratorInvitation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_invitation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptInvitationRequest, String> {
    let mut input = AcceptInvitationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AcceptInvitationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AcceptInvitation request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_automation_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteAutomationRulesRequest, String> {
    let mut input = BatchDeleteAutomationRulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteAutomationRulesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDeleteAutomationRules request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_disable_standards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDisableStandardsRequest, String> {
    let mut input = BatchDisableStandardsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDisableStandardsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchDisableStandards request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_enable_standards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchEnableStandardsRequest, String> {
    let mut input = BatchEnableStandardsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchEnableStandardsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchEnableStandards request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_automation_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetAutomationRulesRequest, String> {
    let mut input = BatchGetAutomationRulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetAutomationRulesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchGetAutomationRules request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_configuration_policy_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetConfigurationPolicyAssociationsRequest, String> {
    let mut input = BatchGetConfigurationPolicyAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetConfigurationPolicyAssociationsRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize BatchGetConfigurationPolicyAssociations request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_security_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetSecurityControlsRequest, String> {
    let mut input = BatchGetSecurityControlsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetSecurityControlsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchGetSecurityControls request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_standards_control_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetStandardsControlAssociationsRequest, String> {
    let mut input = BatchGetStandardsControlAssociationsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<BatchGetStandardsControlAssociationsRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize BatchGetStandardsControlAssociations request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_import_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchImportFindingsRequest, String> {
    let mut input = BatchImportFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchImportFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchImportFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_automation_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateAutomationRulesRequest, String> {
    let mut input = BatchUpdateAutomationRulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateAutomationRulesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchUpdateAutomationRules request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateFindingsRequest, String> {
    let mut input = BatchUpdateFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchUpdateFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_findings_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateFindingsV2Request, String> {
    let mut input = BatchUpdateFindingsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateFindingsV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchUpdateFindingsV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_standards_control_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateStandardsControlAssociationsRequest, String> {
    let mut input = BatchUpdateStandardsControlAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateStandardsControlAssociationsRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize BatchUpdateStandardsControlAssociations request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_action_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateActionTargetRequest, String> {
    let mut input = CreateActionTargetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateActionTargetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateActionTarget request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_aggregator_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAggregatorV2Request, String> {
    let mut input = CreateAggregatorV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAggregatorV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAggregatorV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_automation_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAutomationRuleRequest, String> {
    let mut input = CreateAutomationRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAutomationRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAutomationRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_automation_rule_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAutomationRuleV2Request, String> {
    let mut input = CreateAutomationRuleV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAutomationRuleV2Request>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAutomationRuleV2 request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationPolicyRequest, String> {
    let mut input = CreateConfigurationPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConfigurationPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connector_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectorV2Request, String> {
    let mut input = CreateConnectorV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectorV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateConnectorV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_finding_aggregator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFindingAggregatorRequest, String> {
    let mut input = CreateFindingAggregatorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFindingAggregatorRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateFindingAggregator request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_insight_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInsightRequest, String> {
    let mut input = CreateInsightRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInsightRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateInsight request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMembersRequest, String> {
    let mut input = CreateMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMembers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_ticket_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTicketV2Request, String> {
    let mut input = CreateTicketV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTicketV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTicketV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_decline_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeclineInvitationsRequest, String> {
    let mut input = DeclineInvitationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeclineInvitationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeclineInvitations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_action_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteActionTargetRequest, String> {
    let mut input = DeleteActionTargetRequest::default();
    for (name, value) in labels {
        match *name {
            "ActionTargetArn" => {
                input.action_target_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_aggregator_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAggregatorV2Request, String> {
    let mut input = DeleteAggregatorV2Request::default();
    for (name, value) in labels {
        match *name {
            "AggregatorV2Arn" => {
                input.aggregator_v2_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_automation_rule_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAutomationRuleV2Request, String> {
    let mut input = DeleteAutomationRuleV2Request::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationPolicyRequest, String> {
    let mut input = DeleteConfigurationPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connector_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectorV2Request, String> {
    let mut input = DeleteConnectorV2Request::default();
    for (name, value) in labels {
        match *name {
            "ConnectorId" => {
                input.connector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_finding_aggregator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFindingAggregatorRequest, String> {
    let mut input = DeleteFindingAggregatorRequest::default();
    for (name, value) in labels {
        match *name {
            "FindingAggregatorArn" => {
                input.finding_aggregator_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_insight_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInsightRequest, String> {
    let mut input = DeleteInsightRequest::default();
    for (name, value) in labels {
        match *name {
            "InsightArn" => {
                input.insight_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInvitationsRequest, String> {
    let mut input = DeleteInvitationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteInvitationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteInvitations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMembersRequest, String> {
    let mut input = DeleteMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteMembers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_action_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeActionTargetsRequest, String> {
    let mut input = DescribeActionTargetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeActionTargetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeActionTargets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_hub_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeHubRequest, String> {
    let mut input = DescribeHubRequest::default();
    if let Some(value) = query.get("HubArn") {
        input.hub_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_organization_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrganizationConfigurationRequest, String> {
    let input = DescribeOrganizationConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_products_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeProductsRequest, String> {
    let mut input = DescribeProductsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ProductArn") {
        input.product_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_products_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeProductsV2Request, String> {
    let mut input = DescribeProductsV2Request::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_security_hub_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSecurityHubV2Request, String> {
    let input = DescribeSecurityHubV2Request {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_standards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeStandardsRequest, String> {
    let mut input = DescribeStandardsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_standards_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeStandardsControlsRequest, String> {
    let mut input = DescribeStandardsControlsRequest::default();
    for (name, value) in labels {
        match *name {
            "StandardsSubscriptionArn" => {
                input.standards_subscription_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_import_findings_for_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableImportFindingsForProductRequest, String> {
    let mut input = DisableImportFindingsForProductRequest::default();
    for (name, value) in labels {
        match *name {
            "ProductSubscriptionArn" => {
                input.product_subscription_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableOrganizationAdminAccountRequest, String> {
    let mut input = DisableOrganizationAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisableOrganizationAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisableOrganizationAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_security_hub_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableSecurityHubRequest, String> {
    let input = DisableSecurityHubRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_security_hub_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableSecurityHubV2Request, String> {
    let input = DisableSecurityHubV2Request {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_from_administrator_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateFromAdministratorAccountRequest, String> {
    let input = DisassociateFromAdministratorAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_from_master_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateFromMasterAccountRequest, String> {
    let input = DisassociateFromMasterAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateMembersRequest, String> {
    let mut input = DisassociateMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisassociateMembers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_import_findings_for_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableImportFindingsForProductRequest, String> {
    let mut input = EnableImportFindingsForProductRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableImportFindingsForProductRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize EnableImportFindingsForProduct request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableOrganizationAdminAccountRequest, String> {
    let mut input = EnableOrganizationAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableOrganizationAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize EnableOrganizationAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_security_hub_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableSecurityHubRequest, String> {
    let mut input = EnableSecurityHubRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableSecurityHubRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize EnableSecurityHub request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_security_hub_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableSecurityHubV2Request, String> {
    let mut input = EnableSecurityHubV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableSecurityHubV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize EnableSecurityHubV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_administrator_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAdministratorAccountRequest, String> {
    let input = GetAdministratorAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_aggregator_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAggregatorV2Request, String> {
    let mut input = GetAggregatorV2Request::default();
    for (name, value) in labels {
        match *name {
            "AggregatorV2Arn" => {
                input.aggregator_v2_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automation_rule_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomationRuleV2Request, String> {
    let mut input = GetAutomationRuleV2Request::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationPolicyRequest, String> {
    let mut input = GetConfigurationPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_policy_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationPolicyAssociationRequest, String> {
    let mut input = GetConfigurationPolicyAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetConfigurationPolicyAssociationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize GetConfigurationPolicyAssociation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connector_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectorV2Request, String> {
    let mut input = GetConnectorV2Request::default();
    for (name, value) in labels {
        match *name {
            "ConnectorId" => {
                input.connector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_enabled_standards_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEnabledStandardsRequest, String> {
    let mut input = GetEnabledStandardsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetEnabledStandardsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetEnabledStandards request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_aggregator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingAggregatorRequest, String> {
    let mut input = GetFindingAggregatorRequest::default();
    for (name, value) in labels {
        match *name {
            "FindingAggregatorArn" => {
                input.finding_aggregator_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_history_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingHistoryRequest, String> {
    let mut input = GetFindingHistoryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingHistoryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingHistory request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_statistics_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingStatisticsV2Request, String> {
    let mut input = GetFindingStatisticsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingStatisticsV2Request>(&request.body).map_err(
            |err| format!("failed to deserialize GetFindingStatisticsV2 request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsRequest, String> {
    let mut input = GetFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_trends_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsTrendsV2Request, String> {
    let mut input = GetFindingsTrendsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsTrendsV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingsTrendsV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsV2Request, String> {
    let mut input = GetFindingsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingsV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insight_results_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightResultsRequest, String> {
    let mut input = GetInsightResultsRequest::default();
    for (name, value) in labels {
        match *name {
            "InsightArn" => {
                input.insight_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightsRequest, String> {
    let mut input = GetInsightsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetInsightsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetInsights request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_invitations_count_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInvitationsCountRequest, String> {
    let input = GetInvitationsCountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_master_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMasterAccountRequest, String> {
    let input = GetMasterAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMembersRequest, String> {
    let mut input = GetMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetMembers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resources_statistics_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcesStatisticsV2Request, String> {
    let mut input = GetResourcesStatisticsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourcesStatisticsV2Request>(&request.body).map_err(
            |err| format!("failed to deserialize GetResourcesStatisticsV2 request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resources_trends_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcesTrendsV2Request, String> {
    let mut input = GetResourcesTrendsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourcesTrendsV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize GetResourcesTrendsV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resources_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcesV2Request, String> {
    let mut input = GetResourcesV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourcesV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize GetResourcesV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_security_control_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSecurityControlDefinitionRequest, String> {
    let mut input = GetSecurityControlDefinitionRequest::default();
    if let Some(value) = query.get("SecurityControlId") {
        input.security_control_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_invite_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<InviteMembersRequest, String> {
    let mut input = InviteMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<InviteMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize InviteMembers request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_aggregators_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAggregatorsV2Request, String> {
    let mut input = ListAggregatorsV2Request::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automation_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomationRulesRequest, String> {
    let mut input = ListAutomationRulesRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automation_rules_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomationRulesV2Request, String> {
    let mut input = ListAutomationRulesV2Request::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_configuration_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationPoliciesRequest, String> {
    let mut input = ListConfigurationPoliciesRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_configuration_policy_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationPolicyAssociationsRequest, String> {
    let mut input = ListConfigurationPolicyAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListConfigurationPolicyAssociationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListConfigurationPolicyAssociations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connectors_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectorsV2Request, String> {
    let mut input = ListConnectorsV2Request::default();
    if let Some(value) = query.get("ConnectorStatus") {
        input.connector_status = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ProviderName") {
        input.provider_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_enabled_products_for_import_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEnabledProductsForImportRequest, String> {
    let mut input = ListEnabledProductsForImportRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_finding_aggregators_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingAggregatorsRequest, String> {
    let mut input = ListFindingAggregatorsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInvitationsRequest, String> {
    let mut input = ListInvitationsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMembersRequest, String> {
    let mut input = ListMembersRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("OnlyAssociated") {
        input.only_associated = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_organization_admin_accounts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOrganizationAdminAccountsRequest, String> {
    let mut input = ListOrganizationAdminAccountsRequest::default();
    if let Some(value) = query.get("Feature") {
        input.feature = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_security_control_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityControlDefinitionsRequest, String> {
    let mut input = ListSecurityControlDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("StandardsArn") {
        input.standards_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_standards_control_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStandardsControlAssociationsRequest, String> {
    let mut input = ListStandardsControlAssociationsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("SecurityControlId") {
        input.security_control_id = value.to_string();
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
pub fn deserialize_register_connector_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterConnectorV2Request, String> {
    let mut input = RegisterConnectorV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterConnectorV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterConnectorV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_configuration_policy_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartConfigurationPolicyAssociationRequest, String> {
    let mut input = StartConfigurationPolicyAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartConfigurationPolicyAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartConfigurationPolicyAssociation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_configuration_policy_disassociation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartConfigurationPolicyDisassociationRequest, String> {
    let mut input = StartConfigurationPolicyDisassociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartConfigurationPolicyDisassociationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize StartConfigurationPolicyDisassociation request: {err}")
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
pub fn deserialize_update_action_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateActionTargetRequest, String> {
    let mut input = UpdateActionTargetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateActionTargetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateActionTarget request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ActionTargetArn" => {
                input.action_target_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_aggregator_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAggregatorV2Request, String> {
    let mut input = UpdateAggregatorV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAggregatorV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAggregatorV2 request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AggregatorV2Arn" => {
                input.aggregator_v2_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_automation_rule_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutomationRuleV2Request, String> {
    let mut input = UpdateAutomationRuleV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAutomationRuleV2Request>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAutomationRuleV2 request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationPolicyRequest, String> {
    let mut input = UpdateConfigurationPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateConfigurationPolicy request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_connector_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectorV2Request, String> {
    let mut input = UpdateConnectorV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectorV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConnectorV2 request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ConnectorId" => {
                input.connector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_finding_aggregator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFindingAggregatorRequest, String> {
    let mut input = UpdateFindingAggregatorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFindingAggregatorRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateFindingAggregator request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFindingsRequest, String> {
    let mut input = UpdateFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_insight_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInsightRequest, String> {
    let mut input = UpdateInsightRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInsightRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateInsight request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InsightArn" => {
                input.insight_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_organization_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateOrganizationConfigurationRequest, String> {
    let mut input = UpdateOrganizationConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateOrganizationConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateOrganizationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_security_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSecurityControlRequest, String> {
    let mut input = UpdateSecurityControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSecurityControlRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSecurityControl request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_security_hub_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSecurityHubConfigurationRequest, String> {
    let mut input = UpdateSecurityHubConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSecurityHubConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSecurityHubConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_standards_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStandardsControlRequest, String> {
    let mut input = UpdateStandardsControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStandardsControlRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateStandardsControl request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "StandardsControlArn" => {
                input.standards_control_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
