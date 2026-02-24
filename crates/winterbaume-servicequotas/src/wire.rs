//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicequotas

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_service_quota_template_response(
    result: &AssociateServiceQuotaTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_support_case_response(result: &CreateSupportCaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_quota_increase_request_from_template_response(
    result: &DeleteServiceQuotaIncreaseRequestFromTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_service_quota_template_response(
    result: &DisassociateServiceQuotaTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_a_w_s_default_service_quota_response(
    result: &GetAWSDefaultServiceQuotaResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_association_for_service_quota_template_response(
    result: &GetAssociationForServiceQuotaTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_auto_management_configuration_response(
    result: &GetAutoManagementConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_quota_utilization_report_response(
    result: &GetQuotaUtilizationReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_requested_service_quota_change_response(
    result: &GetRequestedServiceQuotaChangeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_service_quota_response(result: &GetServiceQuotaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_service_quota_increase_request_from_template_response(
    result: &GetServiceQuotaIncreaseRequestFromTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_a_w_s_default_service_quotas_response(
    result: &ListAWSDefaultServiceQuotasResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_requested_service_quota_change_history_response(
    result: &ListRequestedServiceQuotaChangeHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_requested_service_quota_change_history_by_quota_response(
    result: &ListRequestedServiceQuotaChangeHistoryByQuotaResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_service_quota_increase_requests_in_template_response(
    result: &ListServiceQuotaIncreaseRequestsInTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_service_quotas_response(result: &ListServiceQuotasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_services_response(result: &ListServicesResponse) -> MockResponse {
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
pub fn serialize_put_service_quota_increase_request_into_template_response(
    result: &PutServiceQuotaIncreaseRequestIntoTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_request_service_quota_increase_response(
    result: &RequestServiceQuotaIncreaseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_auto_management_response(
    result: &StartAutoManagementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_quota_utilization_report_response(
    result: &StartQuotaUtilizationReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_auto_management_response(
    result: &StopAutoManagementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_auto_management_response(
    result: &UpdateAutoManagementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_service_quota_template_request(
    body: &[u8],
) -> Result<AssociateServiceQuotaTemplateRequest, String> {
    if body.is_empty() {
        return Ok(AssociateServiceQuotaTemplateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateServiceQuotaTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_support_case_request(
    body: &[u8],
) -> Result<CreateSupportCaseRequest, String> {
    if body.is_empty() {
        return Ok(CreateSupportCaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSupportCase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_service_quota_increase_request_from_template_request(
    body: &[u8],
) -> Result<DeleteServiceQuotaIncreaseRequestFromTemplateRequest, String> {
    if body.is_empty() {
        return Ok(DeleteServiceQuotaIncreaseRequestFromTemplateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteServiceQuotaIncreaseRequestFromTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_service_quota_template_request(
    body: &[u8],
) -> Result<DisassociateServiceQuotaTemplateRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateServiceQuotaTemplateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateServiceQuotaTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_a_w_s_default_service_quota_request(
    body: &[u8],
) -> Result<GetAWSDefaultServiceQuotaRequest, String> {
    if body.is_empty() {
        return Ok(GetAWSDefaultServiceQuotaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAWSDefaultServiceQuota request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_association_for_service_quota_template_request(
    body: &[u8],
) -> Result<GetAssociationForServiceQuotaTemplateRequest, String> {
    if body.is_empty() {
        return Ok(GetAssociationForServiceQuotaTemplateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetAssociationForServiceQuotaTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_auto_management_configuration_request(
    body: &[u8],
) -> Result<GetAutoManagementConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetAutoManagementConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAutoManagementConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_quota_utilization_report_request(
    body: &[u8],
) -> Result<GetQuotaUtilizationReportRequest, String> {
    if body.is_empty() {
        return Ok(GetQuotaUtilizationReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQuotaUtilizationReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_requested_service_quota_change_request(
    body: &[u8],
) -> Result<GetRequestedServiceQuotaChangeRequest, String> {
    if body.is_empty() {
        return Ok(GetRequestedServiceQuotaChangeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRequestedServiceQuotaChange request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_service_quota_request(
    body: &[u8],
) -> Result<GetServiceQuotaRequest, String> {
    if body.is_empty() {
        return Ok(GetServiceQuotaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetServiceQuota request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_service_quota_increase_request_from_template_request(
    body: &[u8],
) -> Result<GetServiceQuotaIncreaseRequestFromTemplateRequest, String> {
    if body.is_empty() {
        return Ok(GetServiceQuotaIncreaseRequestFromTemplateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetServiceQuotaIncreaseRequestFromTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_a_w_s_default_service_quotas_request(
    body: &[u8],
) -> Result<ListAWSDefaultServiceQuotasRequest, String> {
    if body.is_empty() {
        return Ok(ListAWSDefaultServiceQuotasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAWSDefaultServiceQuotas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_requested_service_quota_change_history_request(
    body: &[u8],
) -> Result<ListRequestedServiceQuotaChangeHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListRequestedServiceQuotaChangeHistoryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListRequestedServiceQuotaChangeHistory request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_requested_service_quota_change_history_by_quota_request(
    body: &[u8],
) -> Result<ListRequestedServiceQuotaChangeHistoryByQuotaRequest, String> {
    if body.is_empty() {
        return Ok(ListRequestedServiceQuotaChangeHistoryByQuotaRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListRequestedServiceQuotaChangeHistoryByQuota request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_service_quota_increase_requests_in_template_request(
    body: &[u8],
) -> Result<ListServiceQuotaIncreaseRequestsInTemplateRequest, String> {
    if body.is_empty() {
        return Ok(ListServiceQuotaIncreaseRequestsInTemplateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListServiceQuotaIncreaseRequestsInTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_service_quotas_request(
    body: &[u8],
) -> Result<ListServiceQuotasRequest, String> {
    if body.is_empty() {
        return Ok(ListServiceQuotasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServiceQuotas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_services_request(body: &[u8]) -> Result<ListServicesRequest, String> {
    if body.is_empty() {
        return Ok(ListServicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServices request: {e}"))
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
pub fn deserialize_put_service_quota_increase_request_into_template_request(
    body: &[u8],
) -> Result<PutServiceQuotaIncreaseRequestIntoTemplateRequest, String> {
    if body.is_empty() {
        return Ok(PutServiceQuotaIncreaseRequestIntoTemplateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutServiceQuotaIncreaseRequestIntoTemplate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_request_service_quota_increase_request(
    body: &[u8],
) -> Result<RequestServiceQuotaIncreaseRequest, String> {
    if body.is_empty() {
        return Ok(RequestServiceQuotaIncreaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RequestServiceQuotaIncrease request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_auto_management_request(
    body: &[u8],
) -> Result<StartAutoManagementRequest, String> {
    if body.is_empty() {
        return Ok(StartAutoManagementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartAutoManagement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_quota_utilization_report_request(
    body: &[u8],
) -> Result<StartQuotaUtilizationReportRequest, String> {
    if body.is_empty() {
        return Ok(StartQuotaUtilizationReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartQuotaUtilizationReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_auto_management_request(
    body: &[u8],
) -> Result<StopAutoManagementRequest, String> {
    if body.is_empty() {
        return Ok(StopAutoManagementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopAutoManagement request: {e}"))
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
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_auto_management_request(
    body: &[u8],
) -> Result<UpdateAutoManagementRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAutoManagementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAutoManagement request: {e}"))
}
