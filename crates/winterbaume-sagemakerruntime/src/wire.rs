//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sagemaker-runtime

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
pub fn serialize_invoke_endpoint_response(result: &InvokeEndpointOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amzn-sagemaker-closed-session-id": set by caller from closed_session_id field
    // Header "content-type": set by caller from content_type field
    // Header "x-amzn-sagemaker-custom-attributes": set by caller from custom_attributes field
    // Header "x-amzn-invoked-production-variant": set by caller from invoked_production_variant field
    // Header "x-amzn-sagemaker-new-session-id": set by caller from new_session_id field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_invoke_endpoint_async_response(
    result: &InvokeEndpointAsyncOutput,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amzn-sagemaker-failurelocation": set by caller from failure_location field
    // Header "x-amzn-sagemaker-outputlocation": set by caller from output_location field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_invoke_endpoint_with_response_stream_response(
    result: &InvokeEndpointWithResponseStreamOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.body).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amzn-sagemaker-content-type": set by caller from content_type field
    // Header "x-amzn-sagemaker-custom-attributes": set by caller from custom_attributes field
    // Header "x-amzn-invoked-production-variant": set by caller from invoked_production_variant field
    resp
}
