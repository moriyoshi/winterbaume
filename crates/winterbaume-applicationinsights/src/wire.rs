//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationinsights

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_workload_response(result: &AddWorkloadResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_component_response(result: &CreateComponentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_log_pattern_response(result: &CreateLogPatternResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_component_response(result: &DeleteComponentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_log_pattern_response(result: &DeleteLogPatternResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_response(
    result: &DescribeApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_component_response(result: &DescribeComponentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_component_configuration_response(
    result: &DescribeComponentConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_component_configuration_recommendation_response(
    result: &DescribeComponentConfigurationRecommendationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_log_pattern_response(
    result: &DescribeLogPatternResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_observation_response(
    result: &DescribeObservationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_problem_response(result: &DescribeProblemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_problem_observations_response(
    result: &DescribeProblemObservationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workload_response(result: &DescribeWorkloadResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_components_response(result: &ListComponentsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_configuration_history_response(
    result: &ListConfigurationHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_pattern_sets_response(
    result: &ListLogPatternSetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_patterns_response(result: &ListLogPatternsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_problems_response(result: &ListProblemsResponse) -> MockResponse {
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
pub fn serialize_list_workloads_response(result: &ListWorkloadsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_workload_response(result: &RemoveWorkloadResponse) -> MockResponse {
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
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_component_response(result: &UpdateComponentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_component_configuration_response(
    result: &UpdateComponentConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_log_pattern_response(result: &UpdateLogPatternResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_problem_response(result: &UpdateProblemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_workload_response(result: &UpdateWorkloadResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_workload_request(body: &[u8]) -> Result<AddWorkloadRequest, String> {
    if body.is_empty() {
        return Ok(AddWorkloadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddWorkload request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_request(
    body: &[u8],
) -> Result<CreateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_component_request(body: &[u8]) -> Result<CreateComponentRequest, String> {
    if body.is_empty() {
        return Ok(CreateComponentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateComponent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_log_pattern_request(
    body: &[u8],
) -> Result<CreateLogPatternRequest, String> {
    if body.is_empty() {
        return Ok(CreateLogPatternRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLogPattern request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_request(
    body: &[u8],
) -> Result<DeleteApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_component_request(body: &[u8]) -> Result<DeleteComponentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteComponentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteComponent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_log_pattern_request(
    body: &[u8],
) -> Result<DeleteLogPatternRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLogPatternRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLogPattern request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_request(
    body: &[u8],
) -> Result<DescribeApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_component_request(
    body: &[u8],
) -> Result<DescribeComponentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeComponentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeComponent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_component_configuration_request(
    body: &[u8],
) -> Result<DescribeComponentConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeComponentConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeComponentConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_component_configuration_recommendation_request(
    body: &[u8],
) -> Result<DescribeComponentConfigurationRecommendationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeComponentConfigurationRecommendationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeComponentConfigurationRecommendation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_log_pattern_request(
    body: &[u8],
) -> Result<DescribeLogPatternRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLogPatternRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLogPattern request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_observation_request(
    body: &[u8],
) -> Result<DescribeObservationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeObservationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeObservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_problem_request(body: &[u8]) -> Result<DescribeProblemRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProblemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProblem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_problem_observations_request(
    body: &[u8],
) -> Result<DescribeProblemObservationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProblemObservationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProblemObservations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workload_request(
    body: &[u8],
) -> Result<DescribeWorkloadRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkloadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkload request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_applications_request(
    body: &[u8],
) -> Result<ListApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_components_request(body: &[u8]) -> Result<ListComponentsRequest, String> {
    if body.is_empty() {
        return Ok(ListComponentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListComponents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_configuration_history_request(
    body: &[u8],
) -> Result<ListConfigurationHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListConfigurationHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConfigurationHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_pattern_sets_request(
    body: &[u8],
) -> Result<ListLogPatternSetsRequest, String> {
    if body.is_empty() {
        return Ok(ListLogPatternSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogPatternSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_patterns_request(
    body: &[u8],
) -> Result<ListLogPatternsRequest, String> {
    if body.is_empty() {
        return Ok(ListLogPatternsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogPatterns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_problems_request(body: &[u8]) -> Result<ListProblemsRequest, String> {
    if body.is_empty() {
        return Ok(ListProblemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProblems request: {e}"))
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
pub fn deserialize_list_workloads_request(body: &[u8]) -> Result<ListWorkloadsRequest, String> {
    if body.is_empty() {
        return Ok(ListWorkloadsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWorkloads request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_workload_request(body: &[u8]) -> Result<RemoveWorkloadRequest, String> {
    if body.is_empty() {
        return Ok(RemoveWorkloadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveWorkload request: {e}"))
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
pub fn deserialize_update_application_request(
    body: &[u8],
) -> Result<UpdateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_component_request(body: &[u8]) -> Result<UpdateComponentRequest, String> {
    if body.is_empty() {
        return Ok(UpdateComponentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateComponent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_component_configuration_request(
    body: &[u8],
) -> Result<UpdateComponentConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateComponentConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateComponentConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_log_pattern_request(
    body: &[u8],
) -> Result<UpdateLogPatternRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLogPatternRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLogPattern request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_problem_request(body: &[u8]) -> Result<UpdateProblemRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProblemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProblem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_workload_request(body: &[u8]) -> Result<UpdateWorkloadRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWorkloadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkload request: {e}"))
}
