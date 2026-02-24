//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-simpledbv2

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
pub fn serialize_get_export_response(result: &GetExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_exports_response(result: &ListExportsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_domain_export_response(result: &StartDomainExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetExportRequest, String> {
    let mut input = GetExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_exports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListExportsRequest, String> {
    let mut input = ListExportsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListExportsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListExports request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_domain_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDomainExportRequest, String> {
    let mut input = StartDomainExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDomainExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartDomainExport request: {err}"))?;
    }
    Ok(input)
}
