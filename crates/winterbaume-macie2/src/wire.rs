//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-macie2

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
pub fn serialize_accept_invitation_response(result: &AcceptInvitationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_custom_data_identifiers_response(
    result: &BatchGetCustomDataIdentifiersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_automated_discovery_accounts_response(
    result: &BatchUpdateAutomatedDiscoveryAccountsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_allow_list_response(result: &CreateAllowListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_classification_job_response(
    result: &CreateClassificationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_data_identifier_response(
    result: &CreateCustomDataIdentifierResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_findings_filter_response(
    result: &CreateFindingsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_invitations_response(result: &CreateInvitationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_member_response(result: &CreateMemberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_sample_findings_response(
    result: &CreateSampleFindingsResponse,
) -> MockResponse {
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
pub fn serialize_delete_allow_list_response(result: &DeleteAllowListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_data_identifier_response(
    result: &DeleteCustomDataIdentifierResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_findings_filter_response(
    result: &DeleteFindingsFilterResponse,
) -> MockResponse {
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
pub fn serialize_delete_member_response(result: &DeleteMemberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_buckets_response(result: &DescribeBucketsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_classification_job_response(
    result: &DescribeClassificationJobResponse,
) -> MockResponse {
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
pub fn serialize_disable_macie_response(result: &DisableMacieResponse) -> MockResponse {
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
pub fn serialize_disassociate_member_response(result: &DisassociateMemberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_macie_response(result: &EnableMacieResponse) -> MockResponse {
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
pub fn serialize_get_administrator_account_response(
    result: &GetAdministratorAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_allow_list_response(result: &GetAllowListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_discovery_configuration_response(
    result: &GetAutomatedDiscoveryConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_bucket_statistics_response(
    result: &GetBucketStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_classification_export_configuration_response(
    result: &GetClassificationExportConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_classification_scope_response(
    result: &GetClassificationScopeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_custom_data_identifier_response(
    result: &GetCustomDataIdentifierResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_statistics_response(
    result: &GetFindingStatisticsResponse,
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
pub fn serialize_get_findings_filter_response(result: &GetFindingsFilterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_publication_configuration_response(
    result: &GetFindingsPublicationConfigurationResponse,
) -> MockResponse {
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
pub fn serialize_get_macie_session_response(result: &GetMacieSessionResponse) -> MockResponse {
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
pub fn serialize_get_member_response(result: &GetMemberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_profile_response(
    result: &GetResourceProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_reveal_configuration_response(
    result: &GetRevealConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sensitive_data_occurrences_response(
    result: &GetSensitiveDataOccurrencesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sensitive_data_occurrences_availability_response(
    result: &GetSensitiveDataOccurrencesAvailabilityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sensitivity_inspection_template_response(
    result: &GetSensitivityInspectionTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_statistics_response(
    result: &GetUsageStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_totals_response(result: &GetUsageTotalsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_allow_lists_response(result: &ListAllowListsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automated_discovery_accounts_response(
    result: &ListAutomatedDiscoveryAccountsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_classification_jobs_response(
    result: &ListClassificationJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_classification_scopes_response(
    result: &ListClassificationScopesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_data_identifiers_response(
    result: &ListCustomDataIdentifiersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_findings_response(result: &ListFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_findings_filters_response(
    result: &ListFindingsFiltersResponse,
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
pub fn serialize_list_managed_data_identifiers_response(
    result: &ListManagedDataIdentifiersResponse,
) -> MockResponse {
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
pub fn serialize_list_resource_profile_artifacts_response(
    result: &ListResourceProfileArtifactsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_profile_detections_response(
    result: &ListResourceProfileDetectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sensitivity_inspection_templates_response(
    result: &ListSensitivityInspectionTemplatesResponse,
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
pub fn serialize_put_classification_export_configuration_response(
    result: &PutClassificationExportConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_findings_publication_configuration_response(
    result: &PutFindingsPublicationConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_resources_response(result: &SearchResourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_test_custom_data_identifier_response(
    result: &TestCustomDataIdentifierResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_allow_list_response(result: &UpdateAllowListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_automated_discovery_configuration_response(
    result: &UpdateAutomatedDiscoveryConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_classification_job_response(
    result: &UpdateClassificationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_classification_scope_response(
    result: &UpdateClassificationScopeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_findings_filter_response(
    result: &UpdateFindingsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_macie_session_response(
    result: &UpdateMacieSessionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_member_session_response(
    result: &UpdateMemberSessionResponse,
) -> MockResponse {
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
pub fn serialize_update_resource_profile_response(
    result: &UpdateResourceProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_profile_detections_response(
    result: &UpdateResourceProfileDetectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_reveal_configuration_response(
    result: &UpdateRevealConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_sensitivity_inspection_template_response(
    result: &UpdateSensitivityInspectionTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
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
pub fn deserialize_batch_get_custom_data_identifiers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetCustomDataIdentifiersRequest, String> {
    let mut input = BatchGetCustomDataIdentifiersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetCustomDataIdentifiersRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchGetCustomDataIdentifiers request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_automated_discovery_accounts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateAutomatedDiscoveryAccountsRequest, String> {
    let mut input = BatchUpdateAutomatedDiscoveryAccountsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<BatchUpdateAutomatedDiscoveryAccountsRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize BatchUpdateAutomatedDiscoveryAccounts request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_allow_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAllowListRequest, String> {
    let mut input = CreateAllowListRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAllowListRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAllowList request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_classification_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateClassificationJobRequest, String> {
    let mut input = CreateClassificationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateClassificationJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateClassificationJob request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_data_identifier_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomDataIdentifierRequest, String> {
    let mut input = CreateCustomDataIdentifierRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCustomDataIdentifierRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCustomDataIdentifier request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_findings_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFindingsFilterRequest, String> {
    let mut input = CreateFindingsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFindingsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFindingsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInvitationsRequest, String> {
    let mut input = CreateInvitationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInvitationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateInvitations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMemberRequest, String> {
    let mut input = CreateMemberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMemberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMember request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_sample_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSampleFindingsRequest, String> {
    let mut input = CreateSampleFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSampleFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSampleFindings request: {err}"))?;
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
pub fn deserialize_delete_allow_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAllowListRequest, String> {
    let mut input = DeleteAllowListRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("ignoreJobChecks") {
        input.ignore_job_checks = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_data_identifier_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomDataIdentifierRequest, String> {
    let mut input = DeleteCustomDataIdentifierRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_findings_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFindingsFilterRequest, String> {
    let mut input = DeleteFindingsFilterRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
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
pub fn deserialize_delete_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMemberRequest, String> {
    let mut input = DeleteMemberRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBucketsRequest, String> {
    let mut input = DescribeBucketsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeBucketsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeBuckets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_classification_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClassificationJobRequest, String> {
    let mut input = DescribeClassificationJobRequest::default();
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
pub fn deserialize_describe_organization_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrganizationConfigurationRequest, String> {
    let input = DescribeOrganizationConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_macie_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableMacieRequest, String> {
    let input = DisableMacieRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableOrganizationAdminAccountRequest, String> {
    let mut input = DisableOrganizationAdminAccountRequest::default();
    if let Some(value) = query.get("adminAccountId") {
        input.admin_account_id = value.to_string();
    }
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
pub fn deserialize_disassociate_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateMemberRequest, String> {
    let mut input = DisassociateMemberRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_macie_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableMacieRequest, String> {
    let mut input = EnableMacieRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableMacieRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize EnableMacie request: {err}"))?;
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
pub fn deserialize_get_administrator_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAdministratorAccountRequest, String> {
    let input = GetAdministratorAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_allow_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAllowListRequest, String> {
    let mut input = GetAllowListRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_discovery_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedDiscoveryConfigurationRequest, String> {
    let input = GetAutomatedDiscoveryConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_bucket_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketStatisticsRequest, String> {
    let mut input = GetBucketStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBucketStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBucketStatistics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_classification_export_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClassificationExportConfigurationRequest, String> {
    let input = GetClassificationExportConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_classification_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClassificationScopeRequest, String> {
    let mut input = GetClassificationScopeRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_custom_data_identifier_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCustomDataIdentifierRequest, String> {
    let mut input = GetCustomDataIdentifierRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingStatisticsRequest, String> {
    let mut input = GetFindingStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingStatistics request: {err}"))?;
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
pub fn deserialize_get_findings_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsFilterRequest, String> {
    let mut input = GetFindingsFilterRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_publication_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsPublicationConfigurationRequest, String> {
    let input = GetFindingsPublicationConfigurationRequest {};
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
pub fn deserialize_get_macie_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMacieSessionRequest, String> {
    let input = GetMacieSessionRequest {};
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
pub fn deserialize_get_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMemberRequest, String> {
    let mut input = GetMemberRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceProfileRequest, String> {
    let mut input = GetResourceProfileRequest::default();
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_reveal_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRevealConfigurationRequest, String> {
    let input = GetRevealConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sensitive_data_occurrences_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSensitiveDataOccurrencesRequest, String> {
    let mut input = GetSensitiveDataOccurrencesRequest::default();
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
pub fn deserialize_get_sensitive_data_occurrences_availability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSensitiveDataOccurrencesAvailabilityRequest, String> {
    let mut input = GetSensitiveDataOccurrencesAvailabilityRequest::default();
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
pub fn deserialize_get_sensitivity_inspection_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSensitivityInspectionTemplateRequest, String> {
    let mut input = GetSensitivityInspectionTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsageStatisticsRequest, String> {
    let mut input = GetUsageStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetUsageStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetUsageStatistics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_totals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsageTotalsRequest, String> {
    let mut input = GetUsageTotalsRequest::default();
    if let Some(value) = query.get("timeRange") {
        input.time_range = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_allow_lists_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAllowListsRequest, String> {
    let mut input = ListAllowListsRequest::default();
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
pub fn deserialize_list_automated_discovery_accounts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomatedDiscoveryAccountsRequest, String> {
    let mut input = ListAutomatedDiscoveryAccountsRequest::default();
    if let Some(value) = query.get("accountIds") {
        input.account_ids = Some(
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
pub fn deserialize_list_classification_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClassificationJobsRequest, String> {
    let mut input = ListClassificationJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListClassificationJobsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListClassificationJobs request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_classification_scopes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClassificationScopesRequest, String> {
    let mut input = ListClassificationScopesRequest::default();
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_data_identifiers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomDataIdentifiersRequest, String> {
    let mut input = ListCustomDataIdentifiersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCustomDataIdentifiersRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListCustomDataIdentifiers request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingsRequest, String> {
    let mut input = ListFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_findings_filters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingsFiltersRequest, String> {
    let mut input = ListFindingsFiltersRequest::default();
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
pub fn deserialize_list_invitations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInvitationsRequest, String> {
    let mut input = ListInvitationsRequest::default();
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
pub fn deserialize_list_managed_data_identifiers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListManagedDataIdentifiersRequest, String> {
    let mut input = ListManagedDataIdentifiersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListManagedDataIdentifiersRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListManagedDataIdentifiers request: {err}")
            })?;
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
    if let Some(value) = query.get("onlyAssociated") {
        input.only_associated = Some(value.to_string());
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
pub fn deserialize_list_resource_profile_artifacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceProfileArtifactsRequest, String> {
    let mut input = ListResourceProfileArtifactsRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_profile_detections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceProfileDetectionsRequest, String> {
    let mut input = ListResourceProfileDetectionsRequest::default();
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
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_sensitivity_inspection_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSensitivityInspectionTemplatesRequest, String> {
    let mut input = ListSensitivityInspectionTemplatesRequest::default();
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
pub fn deserialize_put_classification_export_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutClassificationExportConfigurationRequest, String> {
    let mut input = PutClassificationExportConfigurationRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutClassificationExportConfigurationRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize PutClassificationExportConfiguration request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_findings_publication_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFindingsPublicationConfigurationRequest, String> {
    let mut input = PutFindingsPublicationConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFindingsPublicationConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutFindingsPublicationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchResourcesRequest, String> {
    let mut input = SearchResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchResources request: {err}"))?;
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
pub fn deserialize_test_custom_data_identifier_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestCustomDataIdentifierRequest, String> {
    let mut input = TestCustomDataIdentifierRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestCustomDataIdentifierRequest>(&request.body).map_err(
            |err| format!("failed to deserialize TestCustomDataIdentifier request: {err}"),
        )?;
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
pub fn deserialize_update_allow_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAllowListRequest, String> {
    let mut input = UpdateAllowListRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAllowListRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAllowList request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_automated_discovery_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutomatedDiscoveryConfigurationRequest, String> {
    let mut input = UpdateAutomatedDiscoveryConfigurationRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<UpdateAutomatedDiscoveryConfigurationRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize UpdateAutomatedDiscoveryConfiguration request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_classification_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClassificationJobRequest, String> {
    let mut input = UpdateClassificationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClassificationJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateClassificationJob request: {err}"),
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_classification_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClassificationScopeRequest, String> {
    let mut input = UpdateClassificationScopeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClassificationScopeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateClassificationScope request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_findings_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFindingsFilterRequest, String> {
    let mut input = UpdateFindingsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFindingsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFindingsFilter request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_macie_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMacieSessionRequest, String> {
    let mut input = UpdateMacieSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMacieSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMacieSession request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_member_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMemberSessionRequest, String> {
    let mut input = UpdateMemberSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMemberSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMemberSession request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
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
pub fn deserialize_update_resource_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceProfileRequest, String> {
    let mut input = UpdateResourceProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResourceProfile request: {err}"))?;
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_profile_detections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceProfileDetectionsRequest, String> {
    let mut input = UpdateResourceProfileDetectionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceProfileDetectionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateResourceProfileDetections request: {err}")
            })?;
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_reveal_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRevealConfigurationRequest, String> {
    let mut input = UpdateRevealConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRevealConfigurationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateRevealConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_sensitivity_inspection_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSensitivityInspectionTemplateRequest, String> {
    let mut input = UpdateSensitivityInspectionTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSensitivityInspectionTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSensitivityInspectionTemplate request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
