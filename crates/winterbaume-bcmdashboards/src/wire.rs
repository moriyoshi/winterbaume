//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bcmdashboards

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_dashboard_response(result: &CreateDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_scheduled_report_response(
    result: &CreateScheduledReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_dashboard_response(result: &DeleteDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_scheduled_report_response(
    result: &DeleteScheduledReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_scheduled_report_response(
    result: &ExecuteScheduledReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dashboard_response(result: &GetDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_scheduled_report_response(
    result: &GetScheduledReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dashboards_response(result: &ListDashboardsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_scheduled_reports_response(
    result: &ListScheduledReportsResponse,
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

/// Serialize response for awsJson protocol.
pub fn serialize_update_dashboard_response(result: &UpdateDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_scheduled_report_response(
    result: &UpdateScheduledReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dashboard_request(body: &[u8]) -> Result<CreateDashboardRequest, String> {
    if body.is_empty() {
        return Ok(CreateDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_scheduled_report_request(
    body: &[u8],
) -> Result<CreateScheduledReportRequest, String> {
    if body.is_empty() {
        return Ok(CreateScheduledReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateScheduledReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dashboard_request(body: &[u8]) -> Result<DeleteDashboardRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_scheduled_report_request(
    body: &[u8],
) -> Result<DeleteScheduledReportRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScheduledReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScheduledReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_scheduled_report_request(
    body: &[u8],
) -> Result<ExecuteScheduledReportRequest, String> {
    if body.is_empty() {
        return Ok(ExecuteScheduledReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteScheduledReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dashboard_request(body: &[u8]) -> Result<GetDashboardRequest, String> {
    if body.is_empty() {
        return Ok(GetDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_scheduled_report_request(
    body: &[u8],
) -> Result<GetScheduledReportRequest, String> {
    if body.is_empty() {
        return Ok(GetScheduledReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetScheduledReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dashboards_request(body: &[u8]) -> Result<ListDashboardsRequest, String> {
    if body.is_empty() {
        return Ok(ListDashboardsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDashboards request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_scheduled_reports_request(
    body: &[u8],
) -> Result<ListScheduledReportsRequest, String> {
    if body.is_empty() {
        return Ok(ListScheduledReportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListScheduledReports request: {e}"))
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

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_dashboard_request(body: &[u8]) -> Result<UpdateDashboardRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_scheduled_report_request(
    body: &[u8],
) -> Result<UpdateScheduledReportRequest, String> {
    if body.is_empty() {
        return Ok(UpdateScheduledReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateScheduledReport request: {e}"))
}
