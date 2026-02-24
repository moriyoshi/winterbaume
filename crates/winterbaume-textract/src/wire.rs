//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-textract

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_analyze_document_response(result: &AnalyzeDocumentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_analyze_expense_response(result: &AnalyzeExpenseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_analyze_i_d_response(result: &AnalyzeIDResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_adapter_response(result: &CreateAdapterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_adapter_version_response(
    result: &CreateAdapterVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_adapter_response(result: &DeleteAdapterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_adapter_version_response(
    result: &DeleteAdapterVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_document_text_response(
    result: &DetectDocumentTextResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_adapter_response(result: &GetAdapterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_adapter_version_response(result: &GetAdapterVersionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_document_analysis_response(
    result: &GetDocumentAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_document_text_detection_response(
    result: &GetDocumentTextDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_expense_analysis_response(
    result: &GetExpenseAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_lending_analysis_response(
    result: &GetLendingAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_lending_analysis_summary_response(
    result: &GetLendingAnalysisSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_adapter_versions_response(
    result: &ListAdapterVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_adapters_response(result: &ListAdaptersResponse) -> MockResponse {
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
pub fn serialize_start_document_analysis_response(
    result: &StartDocumentAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_document_text_detection_response(
    result: &StartDocumentTextDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_expense_analysis_response(
    result: &StartExpenseAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_lending_analysis_response(
    result: &StartLendingAnalysisResponse,
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
pub fn serialize_update_adapter_response(result: &UpdateAdapterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_analyze_document_request(body: &[u8]) -> Result<AnalyzeDocumentRequest, String> {
    if body.is_empty() {
        return Ok(AnalyzeDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AnalyzeDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_analyze_expense_request(body: &[u8]) -> Result<AnalyzeExpenseRequest, String> {
    if body.is_empty() {
        return Ok(AnalyzeExpenseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AnalyzeExpense request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_analyze_i_d_request(body: &[u8]) -> Result<AnalyzeIDRequest, String> {
    if body.is_empty() {
        return Ok(AnalyzeIDRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AnalyzeID request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_adapter_request(body: &[u8]) -> Result<CreateAdapterRequest, String> {
    if body.is_empty() {
        return Ok(CreateAdapterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAdapter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_adapter_version_request(
    body: &[u8],
) -> Result<CreateAdapterVersionRequest, String> {
    if body.is_empty() {
        return Ok(CreateAdapterVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAdapterVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_adapter_request(body: &[u8]) -> Result<DeleteAdapterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAdapterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAdapter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_adapter_version_request(
    body: &[u8],
) -> Result<DeleteAdapterVersionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAdapterVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAdapterVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_document_text_request(
    body: &[u8],
) -> Result<DetectDocumentTextRequest, String> {
    if body.is_empty() {
        return Ok(DetectDocumentTextRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectDocumentText request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_adapter_request(body: &[u8]) -> Result<GetAdapterRequest, String> {
    if body.is_empty() {
        return Ok(GetAdapterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAdapter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_adapter_version_request(
    body: &[u8],
) -> Result<GetAdapterVersionRequest, String> {
    if body.is_empty() {
        return Ok(GetAdapterVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAdapterVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_document_analysis_request(
    body: &[u8],
) -> Result<GetDocumentAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(GetDocumentAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDocumentAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_document_text_detection_request(
    body: &[u8],
) -> Result<GetDocumentTextDetectionRequest, String> {
    if body.is_empty() {
        return Ok(GetDocumentTextDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDocumentTextDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_expense_analysis_request(
    body: &[u8],
) -> Result<GetExpenseAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(GetExpenseAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetExpenseAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_lending_analysis_request(
    body: &[u8],
) -> Result<GetLendingAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(GetLendingAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLendingAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_lending_analysis_summary_request(
    body: &[u8],
) -> Result<GetLendingAnalysisSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetLendingAnalysisSummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLendingAnalysisSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_adapter_versions_request(
    body: &[u8],
) -> Result<ListAdapterVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListAdapterVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAdapterVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_adapters_request(body: &[u8]) -> Result<ListAdaptersRequest, String> {
    if body.is_empty() {
        return Ok(ListAdaptersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAdapters request: {e}"))
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
pub fn deserialize_start_document_analysis_request(
    body: &[u8],
) -> Result<StartDocumentAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(StartDocumentAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDocumentAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_document_text_detection_request(
    body: &[u8],
) -> Result<StartDocumentTextDetectionRequest, String> {
    if body.is_empty() {
        return Ok(StartDocumentTextDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDocumentTextDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_expense_analysis_request(
    body: &[u8],
) -> Result<StartExpenseAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(StartExpenseAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExpenseAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_lending_analysis_request(
    body: &[u8],
) -> Result<StartLendingAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(StartLendingAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartLendingAnalysis request: {e}"))
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
pub fn deserialize_update_adapter_request(body: &[u8]) -> Result<UpdateAdapterRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAdapterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAdapter request: {e}"))
}
