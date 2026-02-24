//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationcostprofiler

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
pub fn serialize_delete_report_definition_response(
    result: &DeleteReportDefinitionResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_report_definition_response(
    result: &GetReportDefinitionResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_application_usage_response(
    result: &ImportApplicationUsageResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_report_definitions_response(
    result: &ListReportDefinitionsResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_report_definition_response(
    result: &PutReportDefinitionResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_report_definition_response(
    result: &UpdateReportDefinitionResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_report_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReportDefinitionRequest, String> {
    let mut input = DeleteReportDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "reportId" => {
                input.report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_report_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetReportDefinitionRequest, String> {
    let mut input = GetReportDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "reportId" => {
                input.report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_application_usage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportApplicationUsageRequest, String> {
    let mut input = ImportApplicationUsageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportApplicationUsageRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ImportApplicationUsage request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_report_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReportDefinitionsRequest, String> {
    let mut input = ListReportDefinitionsRequest::default();
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
pub fn deserialize_put_report_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutReportDefinitionRequest, String> {
    let mut input = PutReportDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutReportDefinitionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutReportDefinition request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_report_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReportDefinitionRequest, String> {
    let mut input = UpdateReportDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReportDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateReportDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "reportId" => {
                input.report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
