//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-support

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_attachments_to_set_response(
    result: &AddAttachmentsToSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_communication_to_case_response(
    result: &AddCommunicationToCaseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_case_response(result: &CreateCaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_attachment_response(result: &DescribeAttachmentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_cases_response(result: &DescribeCasesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_communications_response(
    result: &DescribeCommunicationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_create_case_options_response(
    result: &DescribeCreateCaseOptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_services_response(result: &DescribeServicesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_severity_levels_response(
    result: &DescribeSeverityLevelsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_supported_languages_response(
    result: &DescribeSupportedLanguagesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusted_advisor_check_refresh_statuses_response(
    result: &DescribeTrustedAdvisorCheckRefreshStatusesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusted_advisor_check_result_response(
    result: &DescribeTrustedAdvisorCheckResultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusted_advisor_check_summaries_response(
    result: &DescribeTrustedAdvisorCheckSummariesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trusted_advisor_checks_response(
    result: &DescribeTrustedAdvisorChecksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_refresh_trusted_advisor_check_response(
    result: &RefreshTrustedAdvisorCheckResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resolve_case_response(result: &ResolveCaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_attachments_to_set_request(
    body: &[u8],
) -> Result<AddAttachmentsToSetRequest, String> {
    if body.is_empty() {
        return Ok(AddAttachmentsToSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddAttachmentsToSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_communication_to_case_request(
    body: &[u8],
) -> Result<AddCommunicationToCaseRequest, String> {
    if body.is_empty() {
        return Ok(AddCommunicationToCaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddCommunicationToCase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_case_request(body: &[u8]) -> Result<CreateCaseRequest, String> {
    if body.is_empty() {
        return Ok(CreateCaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_attachment_request(
    body: &[u8],
) -> Result<DescribeAttachmentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAttachmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAttachment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_cases_request(body: &[u8]) -> Result<DescribeCasesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_communications_request(
    body: &[u8],
) -> Result<DescribeCommunicationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCommunicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCommunications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_create_case_options_request(
    body: &[u8],
) -> Result<DescribeCreateCaseOptionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCreateCaseOptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCreateCaseOptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_services_request(
    body: &[u8],
) -> Result<DescribeServicesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_severity_levels_request(
    body: &[u8],
) -> Result<DescribeSeverityLevelsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSeverityLevelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSeverityLevels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_supported_languages_request(
    body: &[u8],
) -> Result<DescribeSupportedLanguagesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSupportedLanguagesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSupportedLanguages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusted_advisor_check_refresh_statuses_request(
    body: &[u8],
) -> Result<DescribeTrustedAdvisorCheckRefreshStatusesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustedAdvisorCheckRefreshStatusesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeTrustedAdvisorCheckRefreshStatuses request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusted_advisor_check_result_request(
    body: &[u8],
) -> Result<DescribeTrustedAdvisorCheckResultRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustedAdvisorCheckResultRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeTrustedAdvisorCheckResult request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusted_advisor_check_summaries_request(
    body: &[u8],
) -> Result<DescribeTrustedAdvisorCheckSummariesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustedAdvisorCheckSummariesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeTrustedAdvisorCheckSummaries request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trusted_advisor_checks_request(
    body: &[u8],
) -> Result<DescribeTrustedAdvisorChecksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrustedAdvisorChecksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTrustedAdvisorChecks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_refresh_trusted_advisor_check_request(
    body: &[u8],
) -> Result<RefreshTrustedAdvisorCheckRequest, String> {
    if body.is_empty() {
        return Ok(RefreshTrustedAdvisorCheckRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RefreshTrustedAdvisorCheck request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resolve_case_request(body: &[u8]) -> Result<ResolveCaseRequest, String> {
    if body.is_empty() {
        return Ok(ResolveCaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResolveCase request: {e}"))
}
