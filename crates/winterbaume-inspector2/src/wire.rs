//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-inspector2

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
pub fn serialize_associate_member_response(result: &AssociateMemberResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_associate_code_security_scan_configuration_response(
    result: &BatchAssociateCodeSecurityScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_disassociate_code_security_scan_configuration_response(
    result: &BatchDisassociateCodeSecurityScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_account_status_response(
    result: &BatchGetAccountStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_code_snippet_response(
    result: &BatchGetCodeSnippetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_finding_details_response(
    result: &BatchGetFindingDetailsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_free_trial_info_response(
    result: &BatchGetFreeTrialInfoResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_member_ec2_deep_inspection_status_response(
    result: &BatchGetMemberEc2DeepInspectionStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_member_ec2_deep_inspection_status_response(
    result: &BatchUpdateMemberEc2DeepInspectionStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_findings_report_response(
    result: &CancelFindingsReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_sbom_export_response(result: &CancelSbomExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cis_scan_configuration_response(
    result: &CreateCisScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_code_security_integration_response(
    result: &CreateCodeSecurityIntegrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_code_security_scan_configuration_response(
    result: &CreateCodeSecurityScanConfigurationResponse,
) -> MockResponse {
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
pub fn serialize_create_findings_report_response(
    result: &CreateFindingsReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_sbom_export_response(result: &CreateSbomExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_cis_scan_configuration_response(
    result: &DeleteCisScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_code_security_integration_response(
    result: &DeleteCodeSecurityIntegrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_code_security_scan_configuration_response(
    result: &DeleteCodeSecurityScanConfigurationResponse,
) -> MockResponse {
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
pub fn serialize_describe_organization_configuration_response(
    result: &DescribeOrganizationConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_response(result: &DisableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_delegated_admin_account_response(
    result: &DisableDelegatedAdminAccountResponse,
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
pub fn serialize_enable_response(result: &EnableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_delegated_admin_account_response(
    result: &EnableDelegatedAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cis_scan_report_response(result: &GetCisScanReportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cis_scan_result_details_response(
    result: &GetCisScanResultDetailsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_clusters_for_image_response(
    result: &GetClustersForImageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_code_security_integration_response(
    result: &GetCodeSecurityIntegrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_code_security_scan_response(
    result: &GetCodeSecurityScanResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_code_security_scan_configuration_response(
    result: &GetCodeSecurityScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_response(result: &GetConfigurationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_delegated_admin_account_response(
    result: &GetDelegatedAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_ec2_deep_inspection_configuration_response(
    result: &GetEc2DeepInspectionConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_encryption_key_response(result: &GetEncryptionKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_report_status_response(
    result: &GetFindingsReportStatusResponse,
) -> MockResponse {
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
pub fn serialize_get_sbom_export_response(result: &GetSbomExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_account_permissions_response(
    result: &ListAccountPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cis_scan_configurations_response(
    result: &ListCisScanConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cis_scan_results_aggregated_by_checks_response(
    result: &ListCisScanResultsAggregatedByChecksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cis_scan_results_aggregated_by_target_resource_response(
    result: &ListCisScanResultsAggregatedByTargetResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cis_scans_response(result: &ListCisScansResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_code_security_integrations_response(
    result: &ListCodeSecurityIntegrationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_code_security_scan_configuration_associations_response(
    result: &ListCodeSecurityScanConfigurationAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_code_security_scan_configurations_response(
    result: &ListCodeSecurityScanConfigurationsResponse,
) -> MockResponse {
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
pub fn serialize_list_coverage_statistics_response(
    result: &ListCoverageStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_delegated_admin_accounts_response(
    result: &ListDelegatedAdminAccountsResponse,
) -> MockResponse {
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
pub fn serialize_list_finding_aggregations_response(
    result: &ListFindingAggregationsResponse,
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
pub fn serialize_list_members_response(result: &ListMembersResponse) -> MockResponse {
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
pub fn serialize_list_usage_totals_response(result: &ListUsageTotalsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reset_encryption_key_response(
    result: &ResetEncryptionKeyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_vulnerabilities_response(
    result: &SearchVulnerabilitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_cis_session_health_response(
    result: &SendCisSessionHealthResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_cis_session_telemetry_response(
    result: &SendCisSessionTelemetryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_cis_session_response(result: &StartCisSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_code_security_scan_response(
    result: &StartCodeSecurityScanResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_cis_session_response(result: &StopCisSessionResponse) -> MockResponse {
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
pub fn serialize_update_cis_scan_configuration_response(
    result: &UpdateCisScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_code_security_integration_response(
    result: &UpdateCodeSecurityIntegrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_code_security_scan_configuration_response(
    result: &UpdateCodeSecurityScanConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_response(
    result: &UpdateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_ec2_deep_inspection_configuration_response(
    result: &UpdateEc2DeepInspectionConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_encryption_key_response(
    result: &UpdateEncryptionKeyResponse,
) -> MockResponse {
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
pub fn serialize_update_org_ec2_deep_inspection_configuration_response(
    result: &UpdateOrgEc2DeepInspectionConfigurationResponse,
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

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateMemberRequest, String> {
    let mut input = AssociateMemberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateMemberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateMember request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_associate_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchAssociateCodeSecurityScanConfigurationRequest, String> {
    let mut input = BatchAssociateCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchAssociateCodeSecurityScanConfigurationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize BatchAssociateCodeSecurityScanConfiguration request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_disassociate_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDisassociateCodeSecurityScanConfigurationRequest, String> {
    let mut input = BatchDisassociateCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDisassociateCodeSecurityScanConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchDisassociateCodeSecurityScanConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_account_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetAccountStatusRequest, String> {
    let mut input = BatchGetAccountStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetAccountStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetAccountStatus request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_code_snippet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetCodeSnippetRequest, String> {
    let mut input = BatchGetCodeSnippetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetCodeSnippetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetCodeSnippet request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_finding_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetFindingDetailsRequest, String> {
    let mut input = BatchGetFindingDetailsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetFindingDetailsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchGetFindingDetails request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_free_trial_info_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetFreeTrialInfoRequest, String> {
    let mut input = BatchGetFreeTrialInfoRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetFreeTrialInfoRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetFreeTrialInfo request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_member_ec2_deep_inspection_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetMemberEc2DeepInspectionStatusRequest, String> {
    let mut input = BatchGetMemberEc2DeepInspectionStatusRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<BatchGetMemberEc2DeepInspectionStatusRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize BatchGetMemberEc2DeepInspectionStatus request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_member_ec2_deep_inspection_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateMemberEc2DeepInspectionStatusRequest, String> {
    let mut input = BatchUpdateMemberEc2DeepInspectionStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateMemberEc2DeepInspectionStatusRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize BatchUpdateMemberEc2DeepInspectionStatus request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_findings_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelFindingsReportRequest, String> {
    let mut input = CancelFindingsReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelFindingsReportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelFindingsReport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_sbom_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelSbomExportRequest, String> {
    let mut input = CancelSbomExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelSbomExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelSbomExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cis_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCisScanConfigurationRequest, String> {
    let mut input = CreateCisScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCisScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCisScanConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_code_security_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCodeSecurityIntegrationRequest, String> {
    let mut input = CreateCodeSecurityIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCodeSecurityIntegrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCodeSecurityIntegration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCodeSecurityScanConfigurationRequest, String> {
    let mut input = CreateCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCodeSecurityScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCodeSecurityScanConfiguration request: {err}")
            })?;
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_findings_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFindingsReportRequest, String> {
    let mut input = CreateFindingsReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFindingsReportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFindingsReport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_sbom_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSbomExportRequest, String> {
    let mut input = CreateSbomExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSbomExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSbomExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cis_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCisScanConfigurationRequest, String> {
    let mut input = DeleteCisScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteCisScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteCisScanConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_code_security_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCodeSecurityIntegrationRequest, String> {
    let mut input = DeleteCodeSecurityIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteCodeSecurityIntegrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteCodeSecurityIntegration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCodeSecurityScanConfigurationRequest, String> {
    let mut input = DeleteCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteCodeSecurityScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteCodeSecurityScanConfiguration request: {err}")
            })?;
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
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteFilter request: {err}"))?;
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
pub fn deserialize_disable_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableRequest, String> {
    let mut input = DisableRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisableRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize Disable request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_delegated_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableDelegatedAdminAccountRequest, String> {
    let mut input = DisableDelegatedAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisableDelegatedAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisableDelegatedAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateMemberRequest, String> {
    let mut input = DisassociateMemberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateMemberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisassociateMember request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableRequest, String> {
    let mut input = EnableRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize Enable request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_delegated_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableDelegatedAdminAccountRequest, String> {
    let mut input = EnableDelegatedAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableDelegatedAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize EnableDelegatedAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cis_scan_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCisScanReportRequest, String> {
    let mut input = GetCisScanReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCisScanReportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCisScanReport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cis_scan_result_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCisScanResultDetailsRequest, String> {
    let mut input = GetCisScanResultDetailsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCisScanResultDetailsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetCisScanResultDetails request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_clusters_for_image_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClustersForImageRequest, String> {
    let mut input = GetClustersForImageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetClustersForImageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetClustersForImage request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_code_security_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCodeSecurityIntegrationRequest, String> {
    let mut input = GetCodeSecurityIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCodeSecurityIntegrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetCodeSecurityIntegration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_code_security_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCodeSecurityScanRequest, String> {
    let mut input = GetCodeSecurityScanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCodeSecurityScanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCodeSecurityScan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCodeSecurityScanConfigurationRequest, String> {
    let mut input = GetCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCodeSecurityScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetCodeSecurityScanConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationRequest, String> {
    let input = GetConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_delegated_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDelegatedAdminAccountRequest, String> {
    let input = GetDelegatedAdminAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_ec2_deep_inspection_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEc2DeepInspectionConfigurationRequest, String> {
    let input = GetEc2DeepInspectionConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_encryption_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEncryptionKeyRequest, String> {
    let mut input = GetEncryptionKeyRequest::default();
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    if let Some(value) = query.get("scanType") {
        input.scan_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_findings_report_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsReportStatusRequest, String> {
    let mut input = GetFindingsReportStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsReportStatusRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetFindingsReportStatus request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_member_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMemberRequest, String> {
    let mut input = GetMemberRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMemberRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetMember request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sbom_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSbomExportRequest, String> {
    let mut input = GetSbomExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSbomExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetSbomExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_account_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccountPermissionsRequest, String> {
    let mut input = ListAccountPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAccountPermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAccountPermissions request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cis_scan_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCisScanConfigurationsRequest, String> {
    let mut input = ListCisScanConfigurationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCisScanConfigurationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListCisScanConfigurations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cis_scan_results_aggregated_by_checks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCisScanResultsAggregatedByChecksRequest, String> {
    let mut input = ListCisScanResultsAggregatedByChecksRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<ListCisScanResultsAggregatedByChecksRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize ListCisScanResultsAggregatedByChecks request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cis_scan_results_aggregated_by_target_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCisScanResultsAggregatedByTargetResourceRequest, String> {
    let mut input = ListCisScanResultsAggregatedByTargetResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCisScanResultsAggregatedByTargetResourceRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize ListCisScanResultsAggregatedByTargetResource request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cis_scans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCisScansRequest, String> {
    let mut input = ListCisScansRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCisScansRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListCisScans request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_code_security_integrations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCodeSecurityIntegrationsRequest, String> {
    let mut input = ListCodeSecurityIntegrationsRequest::default();
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
pub fn deserialize_list_code_security_scan_configuration_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCodeSecurityScanConfigurationAssociationsRequest, String> {
    let mut input = ListCodeSecurityScanConfigurationAssociationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCodeSecurityScanConfigurationAssociationsRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize ListCodeSecurityScanConfigurationAssociations request: {err}"
            )
        })?;
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
pub fn deserialize_list_code_security_scan_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCodeSecurityScanConfigurationsRequest, String> {
    let mut input = ListCodeSecurityScanConfigurationsRequest::default();
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_coverage_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoverageStatisticsRequest, String> {
    let mut input = ListCoverageStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCoverageStatisticsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListCoverageStatistics request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_delegated_admin_accounts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDelegatedAdminAccountsRequest, String> {
    let mut input = ListDelegatedAdminAccountsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDelegatedAdminAccountsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListDelegatedAdminAccounts request: {err}")
            })?;
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
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFiltersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFilters request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_finding_aggregations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingAggregationsRequest, String> {
    let mut input = ListFindingAggregationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFindingAggregationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListFindingAggregations request: {err}"),
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
pub fn deserialize_list_members_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMembersRequest, String> {
    let mut input = ListMembersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListMembersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListMembers request: {err}"))?;
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
pub fn deserialize_list_usage_totals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUsageTotalsRequest, String> {
    let mut input = ListUsageTotalsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListUsageTotalsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListUsageTotals request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reset_encryption_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResetEncryptionKeyRequest, String> {
    let mut input = ResetEncryptionKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResetEncryptionKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ResetEncryptionKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_vulnerabilities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchVulnerabilitiesRequest, String> {
    let mut input = SearchVulnerabilitiesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchVulnerabilitiesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchVulnerabilities request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_cis_session_health_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendCisSessionHealthRequest, String> {
    let mut input = SendCisSessionHealthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendCisSessionHealthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SendCisSessionHealth request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_cis_session_telemetry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendCisSessionTelemetryRequest, String> {
    let mut input = SendCisSessionTelemetryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendCisSessionTelemetryRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SendCisSessionTelemetry request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_cis_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCisSessionRequest, String> {
    let mut input = StartCisSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartCisSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartCisSession request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_code_security_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCodeSecurityScanRequest, String> {
    let mut input = StartCodeSecurityScanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartCodeSecurityScanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartCodeSecurityScan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_cis_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopCisSessionRequest, String> {
    let mut input = StopCisSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopCisSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopCisSession request: {err}"))?;
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
pub fn deserialize_update_cis_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCisScanConfigurationRequest, String> {
    let mut input = UpdateCisScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCisScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCisScanConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_code_security_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCodeSecurityIntegrationRequest, String> {
    let mut input = UpdateCodeSecurityIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCodeSecurityIntegrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCodeSecurityIntegration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_code_security_scan_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCodeSecurityScanConfigurationRequest, String> {
    let mut input = UpdateCodeSecurityScanConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCodeSecurityScanConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCodeSecurityScanConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationRequest, String> {
    let mut input = UpdateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_ec2_deep_inspection_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEc2DeepInspectionConfigurationRequest, String> {
    let mut input = UpdateEc2DeepInspectionConfigurationRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<UpdateEc2DeepInspectionConfigurationRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize UpdateEc2DeepInspectionConfiguration request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_encryption_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEncryptionKeyRequest, String> {
    let mut input = UpdateEncryptionKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEncryptionKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateEncryptionKey request: {err}"))?;
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_org_ec2_deep_inspection_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateOrgEc2DeepInspectionConfigurationRequest, String> {
    let mut input = UpdateOrgEc2DeepInspectionConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateOrgEc2DeepInspectionConfigurationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize UpdateOrgEc2DeepInspectionConfiguration request: {err}")
        })?;
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
