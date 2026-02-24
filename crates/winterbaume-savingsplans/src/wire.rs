//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-savingsplans

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
pub fn serialize_create_savings_plan_response(result: &CreateSavingsPlanResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_queued_savings_plan_response(
    result: &DeleteQueuedSavingsPlanResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_savings_plan_rates_response(
    result: &DescribeSavingsPlanRatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_savings_plans_response(
    result: &DescribeSavingsPlansResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_savings_plans_offering_rates_response(
    result: &DescribeSavingsPlansOfferingRatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_savings_plans_offerings_response(
    result: &DescribeSavingsPlansOfferingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_return_savings_plan_response(result: &ReturnSavingsPlanResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_savings_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSavingsPlanRequest, String> {
    let mut input = CreateSavingsPlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSavingsPlanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSavingsPlan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_queued_savings_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQueuedSavingsPlanRequest, String> {
    let mut input = DeleteQueuedSavingsPlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteQueuedSavingsPlanRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteQueuedSavingsPlan request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_savings_plan_rates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSavingsPlanRatesRequest, String> {
    let mut input = DescribeSavingsPlanRatesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSavingsPlanRatesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeSavingsPlanRates request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_savings_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSavingsPlansRequest, String> {
    let mut input = DescribeSavingsPlansRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSavingsPlansRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeSavingsPlans request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_savings_plans_offering_rates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSavingsPlansOfferingRatesRequest, String> {
    let mut input = DescribeSavingsPlansOfferingRatesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSavingsPlansOfferingRatesRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize DescribeSavingsPlansOfferingRates request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_savings_plans_offerings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSavingsPlansOfferingsRequest, String> {
    let mut input = DescribeSavingsPlansOfferingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSavingsPlansOfferingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeSavingsPlansOfferings request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagsForResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagsForResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_return_savings_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReturnSavingsPlanRequest, String> {
    let mut input = ReturnSavingsPlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ReturnSavingsPlanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ReturnSavingsPlan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}
