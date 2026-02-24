//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appconfigdata

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
pub fn serialize_get_latest_configuration_response(
    result: &GetLatestConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-type": set by caller from content_type field
    // Header "next-poll-configuration-token": set by caller from next_poll_configuration_token field
    // Header "next-poll-interval-in-seconds": set by caller from next_poll_interval_in_seconds field
    // Header "version-label": set by caller from version_label field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_start_configuration_session_response(
    result: &StartConfigurationSessionResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}
