//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pi

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_performance_analysis_report_response(
    result: &CreatePerformanceAnalysisReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_performance_analysis_report_response(
    result: &DeletePerformanceAnalysisReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dimension_keys_response(
    result: &DescribeDimensionKeysResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dimension_key_details_response(
    result: &GetDimensionKeyDetailsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_performance_analysis_report_response(
    result: &GetPerformanceAnalysisReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_metadata_response(
    result: &GetResourceMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_metrics_response(
    result: &GetResourceMetricsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_available_resource_dimensions_response(
    result: &ListAvailableResourceDimensionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_available_resource_metrics_response(
    result: &ListAvailableResourceMetricsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_performance_analysis_reports_response(
    result: &ListPerformanceAnalysisReportsResponse,
) -> MockResponse {
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
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_performance_analysis_report_request(
    body: &[u8],
) -> Result<CreatePerformanceAnalysisReportRequest, String> {
    if body.is_empty() {
        return Ok(CreatePerformanceAnalysisReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePerformanceAnalysisReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_performance_analysis_report_request(
    body: &[u8],
) -> Result<DeletePerformanceAnalysisReportRequest, String> {
    if body.is_empty() {
        return Ok(DeletePerformanceAnalysisReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePerformanceAnalysisReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dimension_keys_request(
    body: &[u8],
) -> Result<DescribeDimensionKeysRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDimensionKeysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDimensionKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dimension_key_details_request(
    body: &[u8],
) -> Result<GetDimensionKeyDetailsRequest, String> {
    if body.is_empty() {
        return Ok(GetDimensionKeyDetailsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDimensionKeyDetails request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_performance_analysis_report_request(
    body: &[u8],
) -> Result<GetPerformanceAnalysisReportRequest, String> {
    if body.is_empty() {
        return Ok(GetPerformanceAnalysisReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPerformanceAnalysisReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_metadata_request(
    body: &[u8],
) -> Result<GetResourceMetadataRequest, String> {
    if body.is_empty() {
        return Ok(GetResourceMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_metrics_request(
    body: &[u8],
) -> Result<GetResourceMetricsRequest, String> {
    if body.is_empty() {
        return Ok(GetResourceMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_available_resource_dimensions_request(
    body: &[u8],
) -> Result<ListAvailableResourceDimensionsRequest, String> {
    if body.is_empty() {
        return Ok(ListAvailableResourceDimensionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAvailableResourceDimensions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_available_resource_metrics_request(
    body: &[u8],
) -> Result<ListAvailableResourceMetricsRequest, String> {
    if body.is_empty() {
        return Ok(ListAvailableResourceMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAvailableResourceMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_performance_analysis_reports_request(
    body: &[u8],
) -> Result<ListPerformanceAnalysisReportsRequest, String> {
    if body.is_empty() {
        return Ok(ListPerformanceAnalysisReportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPerformanceAnalysisReports request: {e}"))
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
