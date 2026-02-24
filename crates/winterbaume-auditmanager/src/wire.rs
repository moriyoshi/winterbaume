//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-auditmanager

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
pub fn serialize_associate_assessment_report_evidence_folder_response(
    result: &AssociateAssessmentReportEvidenceFolderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_associate_assessment_report_evidence_response(
    result: &BatchAssociateAssessmentReportEvidenceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_create_delegation_by_assessment_response(
    result: &BatchCreateDelegationByAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_delegation_by_assessment_response(
    result: &BatchDeleteDelegationByAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_disassociate_assessment_report_evidence_response(
    result: &BatchDisassociateAssessmentReportEvidenceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_import_evidence_to_assessment_control_response(
    result: &BatchImportEvidenceToAssessmentControlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_assessment_response(result: &CreateAssessmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_assessment_framework_response(
    result: &CreateAssessmentFrameworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_assessment_report_response(
    result: &CreateAssessmentReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_control_response(result: &CreateControlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_assessment_response(result: &DeleteAssessmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_assessment_framework_response(
    result: &DeleteAssessmentFrameworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_assessment_framework_share_response(
    result: &DeleteAssessmentFrameworkShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_assessment_report_response(
    result: &DeleteAssessmentReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_control_response(result: &DeleteControlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_account_response(result: &DeregisterAccountResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_organization_admin_account_response(
    result: &DeregisterOrganizationAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_assessment_report_evidence_folder_response(
    result: &DisassociateAssessmentReportEvidenceFolderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_status_response(result: &GetAccountStatusResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_assessment_response(result: &GetAssessmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_assessment_framework_response(
    result: &GetAssessmentFrameworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_assessment_report_url_response(
    result: &GetAssessmentReportUrlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_change_logs_response(result: &GetChangeLogsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_control_response(result: &GetControlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_delegations_response(result: &GetDelegationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_response(result: &GetEvidenceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_by_evidence_folder_response(
    result: &GetEvidenceByEvidenceFolderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_file_upload_url_response(
    result: &GetEvidenceFileUploadUrlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_folder_response(result: &GetEvidenceFolderResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_folders_by_assessment_response(
    result: &GetEvidenceFoldersByAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evidence_folders_by_assessment_control_response(
    result: &GetEvidenceFoldersByAssessmentControlResponse,
) -> MockResponse {
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
pub fn serialize_get_insights_by_assessment_response(
    result: &GetInsightsByAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_organization_admin_account_response(
    result: &GetOrganizationAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_services_in_scope_response(
    result: &GetServicesInScopeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_settings_response(result: &GetSettingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_assessment_control_insights_by_control_domain_response(
    result: &ListAssessmentControlInsightsByControlDomainResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_assessment_framework_share_requests_response(
    result: &ListAssessmentFrameworkShareRequestsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_assessment_frameworks_response(
    result: &ListAssessmentFrameworksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_assessment_reports_response(
    result: &ListAssessmentReportsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_assessments_response(result: &ListAssessmentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_control_domain_insights_response(
    result: &ListControlDomainInsightsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_control_domain_insights_by_assessment_response(
    result: &ListControlDomainInsightsByAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_control_insights_by_control_domain_response(
    result: &ListControlInsightsByControlDomainResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_controls_response(result: &ListControlsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_keywords_for_data_source_response(
    result: &ListKeywordsForDataSourceResponse,
) -> MockResponse {
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
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_account_response(result: &RegisterAccountResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_organization_admin_account_response(
    result: &RegisterOrganizationAdminAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_assessment_framework_share_response(
    result: &StartAssessmentFrameworkShareResponse,
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
pub fn serialize_update_assessment_response(result: &UpdateAssessmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_assessment_control_response(
    result: &UpdateAssessmentControlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_assessment_control_set_status_response(
    result: &UpdateAssessmentControlSetStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_assessment_framework_response(
    result: &UpdateAssessmentFrameworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_assessment_framework_share_response(
    result: &UpdateAssessmentFrameworkShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_assessment_status_response(
    result: &UpdateAssessmentStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_control_response(result: &UpdateControlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_settings_response(result: &UpdateSettingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_validate_assessment_report_integrity_response(
    result: &ValidateAssessmentReportIntegrityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_assessment_report_evidence_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAssessmentReportEvidenceFolderRequest, String> {
    let mut input = AssociateAssessmentReportEvidenceFolderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateAssessmentReportEvidenceFolderRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize AssociateAssessmentReportEvidenceFolder request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_associate_assessment_report_evidence_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchAssociateAssessmentReportEvidenceRequest, String> {
    let mut input = BatchAssociateAssessmentReportEvidenceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchAssociateAssessmentReportEvidenceRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize BatchAssociateAssessmentReportEvidence request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_create_delegation_by_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchCreateDelegationByAssessmentRequest, String> {
    let mut input = BatchCreateDelegationByAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchCreateDelegationByAssessmentRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize BatchCreateDelegationByAssessment request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_delegation_by_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteDelegationByAssessmentRequest, String> {
    let mut input = BatchDeleteDelegationByAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteDelegationByAssessmentRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize BatchDeleteDelegationByAssessment request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_disassociate_assessment_report_evidence_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDisassociateAssessmentReportEvidenceRequest, String> {
    let mut input = BatchDisassociateAssessmentReportEvidenceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDisassociateAssessmentReportEvidenceRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize BatchDisassociateAssessmentReportEvidence request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_import_evidence_to_assessment_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchImportEvidenceToAssessmentControlRequest, String> {
    let mut input = BatchImportEvidenceToAssessmentControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchImportEvidenceToAssessmentControlRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize BatchImportEvidenceToAssessmentControl request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlId" => {
                input.control_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAssessmentRequest, String> {
    let mut input = CreateAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAssessmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAssessment request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_assessment_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAssessmentFrameworkRequest, String> {
    let mut input = CreateAssessmentFrameworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAssessmentFrameworkRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAssessmentFramework request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_assessment_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAssessmentReportRequest, String> {
    let mut input = CreateAssessmentReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAssessmentReportRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAssessmentReport request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateControlRequest, String> {
    let mut input = CreateControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateControlRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateControl request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAssessmentRequest, String> {
    let mut input = DeleteAssessmentRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_assessment_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAssessmentFrameworkRequest, String> {
    let mut input = DeleteAssessmentFrameworkRequest::default();
    for (name, value) in labels {
        match *name {
            "frameworkId" => {
                input.framework_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_assessment_framework_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAssessmentFrameworkShareRequest, String> {
    let mut input = DeleteAssessmentFrameworkShareRequest::default();
    for (name, value) in labels {
        match *name {
            "requestId" => {
                input.request_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("requestType") {
        input.request_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_assessment_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAssessmentReportRequest, String> {
    let mut input = DeleteAssessmentReportRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "assessmentReportId" => {
                input.assessment_report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteControlRequest, String> {
    let mut input = DeleteControlRequest::default();
    for (name, value) in labels {
        match *name {
            "controlId" => {
                input.control_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterAccountRequest, String> {
    let input = DeregisterAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterOrganizationAdminAccountRequest, String> {
    let mut input = DeregisterOrganizationAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeregisterOrganizationAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeregisterOrganizationAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_assessment_report_evidence_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAssessmentReportEvidenceFolderRequest, String> {
    let mut input = DisassociateAssessmentReportEvidenceFolderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateAssessmentReportEvidenceFolderRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize DisassociateAssessmentReportEvidenceFolder request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_account_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountStatusRequest, String> {
    let input = GetAccountStatusRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAssessmentRequest, String> {
    let mut input = GetAssessmentRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_assessment_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAssessmentFrameworkRequest, String> {
    let mut input = GetAssessmentFrameworkRequest::default();
    for (name, value) in labels {
        match *name {
            "frameworkId" => {
                input.framework_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_assessment_report_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAssessmentReportUrlRequest, String> {
    let mut input = GetAssessmentReportUrlRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "assessmentReportId" => {
                input.assessment_report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_change_logs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetChangeLogsRequest, String> {
    let mut input = GetChangeLogsRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("controlId") {
        input.control_id = Some(value.to_string());
    }
    if let Some(value) = query.get("controlSetId") {
        input.control_set_id = Some(value.to_string());
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
pub fn deserialize_get_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetControlRequest, String> {
    let mut input = GetControlRequest::default();
    for (name, value) in labels {
        match *name {
            "controlId" => {
                input.control_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_delegations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDelegationsRequest, String> {
    let mut input = GetDelegationsRequest::default();
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
pub fn deserialize_get_evidence_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceRequest, String> {
    let mut input = GetEvidenceRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            "evidenceFolderId" => {
                input.evidence_folder_id = value.to_string();
            }
            "evidenceId" => {
                input.evidence_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_evidence_by_evidence_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceByEvidenceFolderRequest, String> {
    let mut input = GetEvidenceByEvidenceFolderRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            "evidenceFolderId" => {
                input.evidence_folder_id = value.to_string();
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
pub fn deserialize_get_evidence_file_upload_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceFileUploadUrlRequest, String> {
    let mut input = GetEvidenceFileUploadUrlRequest::default();
    if let Some(value) = query.get("fileName") {
        input.file_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_evidence_folder_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceFolderRequest, String> {
    let mut input = GetEvidenceFolderRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            "evidenceFolderId" => {
                input.evidence_folder_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_evidence_folders_by_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceFoldersByAssessmentRequest, String> {
    let mut input = GetEvidenceFoldersByAssessmentRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
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
pub fn deserialize_get_evidence_folders_by_assessment_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvidenceFoldersByAssessmentControlRequest, String> {
    let mut input = GetEvidenceFoldersByAssessmentControlRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlId" => {
                input.control_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
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
pub fn deserialize_get_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightsRequest, String> {
    let input = GetInsightsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insights_by_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightsByAssessmentRequest, String> {
    let mut input = GetInsightsByAssessmentRequest::default();
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOrganizationAdminAccountRequest, String> {
    let input = GetOrganizationAdminAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_services_in_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServicesInScopeRequest, String> {
    let input = GetServicesInScopeRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSettingsRequest, String> {
    let mut input = GetSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "attribute" => {
                input.attribute = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_assessment_control_insights_by_control_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssessmentControlInsightsByControlDomainRequest, String> {
    let mut input = ListAssessmentControlInsightsByControlDomainRequest::default();
    if let Some(value) = query.get("assessmentId") {
        input.assessment_id = value.to_string();
    }
    if let Some(value) = query.get("controlDomainId") {
        input.control_domain_id = value.to_string();
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
pub fn deserialize_list_assessment_framework_share_requests_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssessmentFrameworkShareRequestsRequest, String> {
    let mut input = ListAssessmentFrameworkShareRequestsRequest::default();
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
    if let Some(value) = query.get("requestType") {
        input.request_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_assessment_frameworks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssessmentFrameworksRequest, String> {
    let mut input = ListAssessmentFrameworksRequest::default();
    if let Some(value) = query.get("frameworkType") {
        input.framework_type = value.to_string();
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
pub fn deserialize_list_assessment_reports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssessmentReportsRequest, String> {
    let mut input = ListAssessmentReportsRequest::default();
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
pub fn deserialize_list_assessments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssessmentsRequest, String> {
    let mut input = ListAssessmentsRequest::default();
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
pub fn deserialize_list_control_domain_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListControlDomainInsightsRequest, String> {
    let mut input = ListControlDomainInsightsRequest::default();
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
pub fn deserialize_list_control_domain_insights_by_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListControlDomainInsightsByAssessmentRequest, String> {
    let mut input = ListControlDomainInsightsByAssessmentRequest::default();
    if let Some(value) = query.get("assessmentId") {
        input.assessment_id = value.to_string();
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
pub fn deserialize_list_control_insights_by_control_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListControlInsightsByControlDomainRequest, String> {
    let mut input = ListControlInsightsByControlDomainRequest::default();
    if let Some(value) = query.get("controlDomainId") {
        input.control_domain_id = value.to_string();
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
pub fn deserialize_list_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListControlsRequest, String> {
    let mut input = ListControlsRequest::default();
    if let Some(value) = query.get("controlCatalogId") {
        input.control_catalog_id = Some(value.to_string());
    }
    if let Some(value) = query.get("controlType") {
        input.control_type = value.to_string();
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
pub fn deserialize_list_keywords_for_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKeywordsForDataSourceRequest, String> {
    let mut input = ListKeywordsForDataSourceRequest::default();
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
    if let Some(value) = query.get("source") {
        input.source = value.to_string();
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
pub fn deserialize_register_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterAccountRequest, String> {
    let mut input = RegisterAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterAccountRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterAccount request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_organization_admin_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterOrganizationAdminAccountRequest, String> {
    let mut input = RegisterOrganizationAdminAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterOrganizationAdminAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RegisterOrganizationAdminAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_assessment_framework_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAssessmentFrameworkShareRequest, String> {
    let mut input = StartAssessmentFrameworkShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAssessmentFrameworkShareRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartAssessmentFrameworkShare request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "frameworkId" => {
                input.framework_id = value.to_string();
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
pub fn deserialize_update_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentRequest, String> {
    let mut input = UpdateAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAssessment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_assessment_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentControlRequest, String> {
    let mut input = UpdateAssessmentControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentControlRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAssessmentControl request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlId" => {
                input.control_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_assessment_control_set_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentControlSetStatusRequest, String> {
    let mut input = UpdateAssessmentControlSetStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentControlSetStatusRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAssessmentControlSetStatus request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            "controlSetId" => {
                input.control_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_assessment_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentFrameworkRequest, String> {
    let mut input = UpdateAssessmentFrameworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentFrameworkRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAssessmentFramework request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "frameworkId" => {
                input.framework_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_assessment_framework_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentFrameworkShareRequest, String> {
    let mut input = UpdateAssessmentFrameworkShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentFrameworkShareRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAssessmentFrameworkShare request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "requestId" => {
                input.request_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_assessment_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssessmentStatusRequest, String> {
    let mut input = UpdateAssessmentStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAssessmentStatusRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAssessmentStatus request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "assessmentId" => {
                input.assessment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateControlRequest, String> {
    let mut input = UpdateControlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateControlRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateControl request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "controlId" => {
                input.control_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSettingsRequest, String> {
    let mut input = UpdateSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_validate_assessment_report_integrity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ValidateAssessmentReportIntegrityRequest, String> {
    let mut input = ValidateAssessmentReportIntegrityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ValidateAssessmentReportIntegrityRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize ValidateAssessmentReportIntegrity request: {err}")
        })?;
    }
    Ok(input)
}
