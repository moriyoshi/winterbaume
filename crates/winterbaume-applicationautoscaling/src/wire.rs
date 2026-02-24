//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationautoscaling

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_delete_scaling_policy_response(
    result: &DeleteScalingPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_scheduled_action_response(
    result: &DeleteScheduledActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_scalable_target_response(
    result: &DeregisterScalableTargetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scalable_targets_response(
    result: &DescribeScalableTargetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scaling_activities_response(
    result: &DescribeScalingActivitiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scaling_policies_response(
    result: &DescribeScalingPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_scheduled_actions_response(
    result: &DescribeScheduledActionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_predictive_scaling_forecast_response(
    result: &GetPredictiveScalingForecastResponse,
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
pub fn serialize_put_scaling_policy_response(result: &PutScalingPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_scheduled_action_response(
    result: &PutScheduledActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_scalable_target_response(
    result: &RegisterScalableTargetResponse,
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
pub fn deserialize_delete_scaling_policy_request(
    body: &[u8],
) -> Result<DeleteScalingPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScalingPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_scheduled_action_request(
    body: &[u8],
) -> Result<DeleteScheduledActionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScheduledActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScheduledAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_scalable_target_request(
    body: &[u8],
) -> Result<DeregisterScalableTargetRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterScalableTargetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterScalableTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scalable_targets_request(
    body: &[u8],
) -> Result<DescribeScalableTargetsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScalableTargetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScalableTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scaling_activities_request(
    body: &[u8],
) -> Result<DescribeScalingActivitiesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScalingActivitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScalingActivities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scaling_policies_request(
    body: &[u8],
) -> Result<DescribeScalingPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScalingPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScalingPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_scheduled_actions_request(
    body: &[u8],
) -> Result<DescribeScheduledActionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeScheduledActionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeScheduledActions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_predictive_scaling_forecast_request(
    body: &[u8],
) -> Result<GetPredictiveScalingForecastRequest, String> {
    if body.is_empty() {
        return Ok(GetPredictiveScalingForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPredictiveScalingForecast request: {e}"))
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
pub fn deserialize_put_scaling_policy_request(
    body: &[u8],
) -> Result<PutScalingPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutScalingPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_scheduled_action_request(
    body: &[u8],
) -> Result<PutScheduledActionRequest, String> {
    if body.is_empty() {
        return Ok(PutScheduledActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutScheduledAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_scalable_target_request(
    body: &[u8],
) -> Result<RegisterScalableTargetRequest, String> {
    if body.is_empty() {
        return Ok(RegisterScalableTargetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterScalableTarget request: {e}"))
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
