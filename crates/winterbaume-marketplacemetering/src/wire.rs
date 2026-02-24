//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-marketplacemetering

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_meter_usage_response(result: &BatchMeterUsageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_meter_usage_response(result: &MeterUsageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_usage_response(result: &RegisterUsageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resolve_customer_response(result: &ResolveCustomerResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_meter_usage_request(
    body: &[u8],
) -> Result<BatchMeterUsageRequest, String> {
    if body.is_empty() {
        return Ok(BatchMeterUsageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchMeterUsage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_meter_usage_request(body: &[u8]) -> Result<MeterUsageRequest, String> {
    if body.is_empty() {
        return Ok(MeterUsageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MeterUsage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_usage_request(body: &[u8]) -> Result<RegisterUsageRequest, String> {
    if body.is_empty() {
        return Ok(RegisterUsageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterUsage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resolve_customer_request(body: &[u8]) -> Result<ResolveCustomerRequest, String> {
    if body.is_empty() {
        return Ok(ResolveCustomerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResolveCustomer request: {e}"))
}
