//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-guardduty

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
pub fn serialize_archive_findings_response(result: &ArchiveFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_detector_response(result: &CreateDetectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_filter_response(result: &CreateFilterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_i_p_set_response(result: &CreateIPSetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_malware_protection_plan_response(
    result: &CreateMalwareProtectionPlanResponse,
) -> MockResponse {
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
pub fn serialize_create_publishing_destination_response(
    result: &CreatePublishingDestinationResponse,
) -> MockResponse {
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
pub fn serialize_create_threat_entity_set_response(
    result: &CreateThreatEntitySetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_threat_intel_set_response(
    result: &CreateThreatIntelSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_trusted_entity_set_response(
    result: &CreateTrustedEntitySetResponse,
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
pub fn serialize_delete_detector_response(result: &DeleteDetectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_filter_response(result: &DeleteFilterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_i_p_set_response(result: &DeleteIPSetResponse) -> MockResponse {
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

/// Serialize void response for restJson protocol.
pub fn serialize_delete_malware_protection_plan_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_members_response(result: &DeleteMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_publishing_destination_response(
    result: &DeletePublishingDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_threat_entity_set_response(
    result: &DeleteThreatEntitySetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_threat_intel_set_response(
    result: &DeleteThreatIntelSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_trusted_entity_set_response(
    result: &DeleteTrustedEntitySetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_malware_scans_response(
    result: &DescribeMalwareScansResponse,
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
pub fn serialize_describe_publishing_destination_response(
    result: &DescribePublishingDestinationResponse,
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
pub fn serialize_get_coverage_statistics_response(
    result: &GetCoverageStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_detector_response(result: &GetDetectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_filter_response(result: &GetFilterResponse) -> MockResponse {
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
pub fn serialize_get_findings_statistics_response(
    result: &GetFindingsStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_i_p_set_response(result: &GetIPSetResponse) -> MockResponse {
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
pub fn serialize_get_malware_protection_plan_response(
    result: &GetMalwareProtectionPlanResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_malware_scan_response(result: &GetMalwareScanResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_malware_scan_settings_response(
    result: &GetMalwareScanSettingsResponse,
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
pub fn serialize_get_member_detectors_response(
    result: &GetMemberDetectorsResponse,
) -> MockResponse {
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
pub fn serialize_get_organization_statistics_response(
    result: &GetOrganizationStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_remaining_free_trial_days_response(
    result: &GetRemainingFreeTrialDaysResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_threat_entity_set_response(
    result: &GetThreatEntitySetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_threat_intel_set_response(result: &GetThreatIntelSetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trusted_entity_set_response(
    result: &GetTrustedEntitySetResponse,
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
pub fn serialize_invite_members_response(result: &InviteMembersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_coverage_response(result: &ListCoverageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_detectors_response(result: &ListDetectorsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_filters_response(result: &ListFiltersResponse) -> MockResponse {
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
pub fn serialize_list_i_p_sets_response(result: &ListIPSetsResponse) -> MockResponse {
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
pub fn serialize_list_malware_protection_plans_response(
    result: &ListMalwareProtectionPlansResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_malware_scans_response(result: &ListMalwareScansResponse) -> MockResponse {
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
pub fn serialize_list_publishing_destinations_response(
    result: &ListPublishingDestinationsResponse,
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
pub fn serialize_list_threat_entity_sets_response(
    result: &ListThreatEntitySetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_threat_intel_sets_response(
    result: &ListThreatIntelSetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_trusted_entity_sets_response(
    result: &ListTrustedEntitySetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_object_malware_scan_response(
    result: &SendObjectMalwareScanResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_malware_scan_response(result: &StartMalwareScanResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_monitoring_members_response(
    result: &StartMonitoringMembersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_monitoring_members_response(
    result: &StopMonitoringMembersResponse,
) -> MockResponse {
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
pub fn serialize_unarchive_findings_response(result: &UnarchiveFindingsResponse) -> MockResponse {
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
pub fn serialize_update_detector_response(result: &UpdateDetectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_filter_response(result: &UpdateFilterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_findings_feedback_response(
    result: &UpdateFindingsFeedbackResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_i_p_set_response(result: &UpdateIPSetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_malware_protection_plan_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_malware_scan_settings_response(
    result: &UpdateMalwareScanSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_member_detectors_response(
    result: &UpdateMemberDetectorsResponse,
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
pub fn serialize_update_publishing_destination_response(
    result: &UpdatePublishingDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_threat_entity_set_response(
    result: &UpdateThreatEntitySetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_threat_intel_set_response(
    result: &UpdateThreatIntelSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_trusted_entity_set_response(
    result: &UpdateTrustedEntitySetResponse,
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_archive_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ArchiveFindingsRequest, String> {
    let mut input = ArchiveFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ArchiveFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ArchiveFindings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDetectorRequest, String> {
    let mut input = CreateDetectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDetectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDetector request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFilterRequest, String> {
    let mut input = CreateFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFilter request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_i_p_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIPSetRequest, String> {
    let mut input = CreateIPSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIPSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIPSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_malware_protection_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMalwareProtectionPlanRequest, String> {
    let mut input = CreateMalwareProtectionPlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMalwareProtectionPlanRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateMalwareProtectionPlan request: {err}")
            })?;
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_publishing_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePublishingDestinationRequest, String> {
    let mut input = CreatePublishingDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePublishingDestinationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePublishingDestination request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_threat_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThreatEntitySetRequest, String> {
    let mut input = CreateThreatEntitySetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThreatEntitySetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThreatEntitySet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_threat_intel_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateThreatIntelSetRequest, String> {
    let mut input = CreateThreatIntelSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateThreatIntelSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateThreatIntelSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_trusted_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrustedEntitySetRequest, String> {
    let mut input = CreateTrustedEntitySetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTrustedEntitySetRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateTrustedEntitySet request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_delete_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDetectorRequest, String> {
    let mut input = DeleteDetectorRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFilterRequest, String> {
    let mut input = DeleteFilterRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "FilterName" => {
                input.filter_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_i_p_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIPSetRequest, String> {
    let mut input = DeleteIPSetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "IpSetId" => {
                input.ip_set_id = value.to_string();
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
pub fn deserialize_delete_malware_protection_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMalwareProtectionPlanRequest, String> {
    let mut input = DeleteMalwareProtectionPlanRequest::default();
    for (name, value) in labels {
        match *name {
            "MalwareProtectionPlanId" => {
                input.malware_protection_plan_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_publishing_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePublishingDestinationRequest, String> {
    let mut input = DeletePublishingDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "DestinationId" => {
                input.destination_id = value.to_string();
            }
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_threat_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThreatEntitySetRequest, String> {
    let mut input = DeleteThreatEntitySetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatEntitySetId" => {
                input.threat_entity_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_threat_intel_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteThreatIntelSetRequest, String> {
    let mut input = DeleteThreatIntelSetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatIntelSetId" => {
                input.threat_intel_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_trusted_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrustedEntitySetRequest, String> {
    let mut input = DeleteTrustedEntitySetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "TrustedEntitySetId" => {
                input.trusted_entity_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_malware_scans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMalwareScansRequest, String> {
    let mut input = DescribeMalwareScansRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeMalwareScansRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeMalwareScans request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
    let mut input = DescribeOrganizationConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_describe_publishing_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePublishingDestinationRequest, String> {
    let mut input = DescribePublishingDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "DestinationId" => {
                input.destination_id = value.to_string();
            }
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_disassociate_from_administrator_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateFromAdministratorAccountRequest, String> {
    let mut input = DisassociateFromAdministratorAccountRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_from_master_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateFromMasterAccountRequest, String> {
    let mut input = DisassociateFromMasterAccountRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
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
    let mut input = GetAdministratorAccountRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_coverage_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoverageStatisticsRequest, String> {
    let mut input = GetCoverageStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCoverageStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCoverageStatistics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDetectorRequest, String> {
    let mut input = GetDetectorRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFilterRequest, String> {
    let mut input = GetFilterRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "FilterName" => {
                input.filter_name = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsStatisticsRequest, String> {
    let mut input = GetFindingsStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingsStatistics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_i_p_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIPSetRequest, String> {
    let mut input = GetIPSetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "IpSetId" => {
                input.ip_set_id = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_get_malware_protection_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMalwareProtectionPlanRequest, String> {
    let mut input = GetMalwareProtectionPlanRequest::default();
    for (name, value) in labels {
        match *name {
            "MalwareProtectionPlanId" => {
                input.malware_protection_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_malware_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMalwareScanRequest, String> {
    let mut input = GetMalwareScanRequest::default();
    for (name, value) in labels {
        match *name {
            "ScanId" => {
                input.scan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_malware_scan_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMalwareScanSettingsRequest, String> {
    let mut input = GetMalwareScanSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_master_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMasterAccountRequest, String> {
    let mut input = GetMasterAccountRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_member_detectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMemberDetectorsRequest, String> {
    let mut input = GetMemberDetectorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMemberDetectorsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetMemberDetectors request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_remaining_free_trial_days_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRemainingFreeTrialDaysRequest, String> {
    let mut input = GetRemainingFreeTrialDaysRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetRemainingFreeTrialDaysRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetRemainingFreeTrialDays request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_threat_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetThreatEntitySetRequest, String> {
    let mut input = GetThreatEntitySetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatEntitySetId" => {
                input.threat_entity_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_threat_intel_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetThreatIntelSetRequest, String> {
    let mut input = GetThreatIntelSetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatIntelSetId" => {
                input.threat_intel_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trusted_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrustedEntitySetRequest, String> {
    let mut input = GetTrustedEntitySetRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "TrustedEntitySetId" => {
                input.trusted_entity_set_id = value.to_string();
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_coverage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoverageRequest, String> {
    let mut input = ListCoverageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCoverageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListCoverage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_detectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDetectorsRequest, String> {
    let mut input = ListDetectorsRequest::default();
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
pub fn deserialize_list_filters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFiltersRequest, String> {
    let mut input = ListFiltersRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_i_p_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIPSetsRequest, String> {
    let mut input = ListIPSetsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_list_malware_protection_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMalwareProtectionPlansRequest, String> {
    let mut input = ListMalwareProtectionPlansRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_malware_scans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMalwareScansRequest, String> {
    let mut input = ListMalwareScansRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListMalwareScansRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListMalwareScans request: {err}"))?;
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
pub fn deserialize_list_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMembersRequest, String> {
    let mut input = ListMembersRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_list_publishing_destinations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPublishingDestinationsRequest, String> {
    let mut input = ListPublishingDestinationsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_threat_entity_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThreatEntitySetsRequest, String> {
    let mut input = ListThreatEntitySetsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_list_threat_intel_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListThreatIntelSetsRequest, String> {
    let mut input = ListThreatIntelSetsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_list_trusted_entity_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrustedEntitySetsRequest, String> {
    let mut input = ListTrustedEntitySetsRequest::default();
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_send_object_malware_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendObjectMalwareScanRequest, String> {
    let mut input = SendObjectMalwareScanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendObjectMalwareScanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SendObjectMalwareScan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_malware_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMalwareScanRequest, String> {
    let mut input = StartMalwareScanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartMalwareScanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartMalwareScan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_monitoring_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMonitoringMembersRequest, String> {
    let mut input = StartMonitoringMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartMonitoringMembersRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartMonitoringMembers request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_monitoring_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopMonitoringMembersRequest, String> {
    let mut input = StopMonitoringMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopMonitoringMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopMonitoringMembers request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_unarchive_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UnarchiveFindingsRequest, String> {
    let mut input = UnarchiveFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UnarchiveFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UnarchiveFindings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
pub fn deserialize_update_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDetectorRequest, String> {
    let mut input = UpdateDetectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDetectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDetector request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFilterRequest, String> {
    let mut input = UpdateFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFilter request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "FilterName" => {
                input.filter_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_findings_feedback_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFindingsFeedbackRequest, String> {
    let mut input = UpdateFindingsFeedbackRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFindingsFeedbackRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateFindingsFeedback request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_i_p_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIPSetRequest, String> {
    let mut input = UpdateIPSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIPSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIPSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "IpSetId" => {
                input.ip_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_malware_protection_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMalwareProtectionPlanRequest, String> {
    let mut input = UpdateMalwareProtectionPlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMalwareProtectionPlanRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateMalwareProtectionPlan request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "MalwareProtectionPlanId" => {
                input.malware_protection_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_malware_scan_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMalwareScanSettingsRequest, String> {
    let mut input = UpdateMalwareScanSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMalwareScanSettingsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateMalwareScanSettings request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_member_detectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMemberDetectorsRequest, String> {
    let mut input = UpdateMemberDetectorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMemberDetectorsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMemberDetectors request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
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
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_publishing_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePublishingDestinationRequest, String> {
    let mut input = UpdatePublishingDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePublishingDestinationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePublishingDestination request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DestinationId" => {
                input.destination_id = value.to_string();
            }
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_threat_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThreatEntitySetRequest, String> {
    let mut input = UpdateThreatEntitySetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThreatEntitySetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThreatEntitySet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatEntitySetId" => {
                input.threat_entity_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_threat_intel_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThreatIntelSetRequest, String> {
    let mut input = UpdateThreatIntelSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThreatIntelSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateThreatIntelSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "ThreatIntelSetId" => {
                input.threat_intel_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_trusted_entity_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrustedEntitySetRequest, String> {
    let mut input = UpdateTrustedEntitySetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTrustedEntitySetRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateTrustedEntitySet request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DetectorId" => {
                input.detector_id = value.to_string();
            }
            "TrustedEntitySetId" => {
                input.trusted_entity_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
