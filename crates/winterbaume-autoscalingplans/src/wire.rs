//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-autoscalingplans

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_scaling_plan_response(result: &CreateScalingPlanResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_scaling_plan_response(result: &DeleteScalingPlanResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scaling_plan_resources_response(
    result: &DescribeScalingPlanResourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scaling_plans_response(
    result: &DescribeScalingPlansResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_scaling_plan_resource_forecast_data_response(
    result: &GetScalingPlanResourceForecastDataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_scaling_plan_response(result: &UpdateScalingPlanResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_scaling_plan_request(
    body: &[u8],
) -> Result<CreateScalingPlanRequest, String> {
    if body.is_empty() {
        return Ok(CreateScalingPlanRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateScalingPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_scaling_plan_request(
    body: &[u8],
) -> Result<DeleteScalingPlanRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScalingPlanRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScalingPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scaling_plan_resources_request(
    body: &[u8],
) -> Result<DescribeScalingPlanResourcesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScalingPlanResourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScalingPlanResources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scaling_plans_request(
    body: &[u8],
) -> Result<DescribeScalingPlansRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScalingPlansRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScalingPlans request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_scaling_plan_resource_forecast_data_request(
    body: &[u8],
) -> Result<GetScalingPlanResourceForecastDataRequest, String> {
    if body.is_empty() {
        return Ok(GetScalingPlanResourceForecastDataRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetScalingPlanResourceForecastData request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_scaling_plan_request(
    body: &[u8],
) -> Result<UpdateScalingPlanRequest, String> {
    if body.is_empty() {
        return Ok(UpdateScalingPlanRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateScalingPlan request: {e}"))
}
