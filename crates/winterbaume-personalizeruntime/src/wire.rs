//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-personalizeruntime

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
pub fn serialize_get_action_recommendations_response(
    result: &GetActionRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_personalized_ranking_response(
    result: &GetPersonalizedRankingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recommendations_response(result: &GetRecommendationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_action_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetActionRecommendationsRequest, String> {
    let mut input = GetActionRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetActionRecommendationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetActionRecommendations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_personalized_ranking_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPersonalizedRankingRequest, String> {
    let mut input = GetPersonalizedRankingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPersonalizedRankingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetPersonalizedRanking request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRecommendationsRequest, String> {
    let mut input = GetRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetRecommendationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetRecommendations request: {err}"))?;
    }
    Ok(input)
}
