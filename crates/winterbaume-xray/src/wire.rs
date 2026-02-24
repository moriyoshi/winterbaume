//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-xray

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
pub fn serialize_batch_get_traces_response(result: &BatchGetTracesResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_trace_retrieval_response(
    result: &CancelTraceRetrievalResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_sampling_rule_response(result: &CreateSamplingRuleResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_sampling_rule_response(result: &DeleteSamplingRuleResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_encryption_config_response(
    result: &GetEncryptionConfigResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_response(result: &GetGroupResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_groups_response(result: &GetGroupsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_indexing_rules_response(result: &GetIndexingRulesResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insight_response(result: &GetInsightResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insight_events_response(result: &GetInsightEventsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insight_impact_graph_response(
    result: &GetInsightImpactGraphResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_insight_summaries_response(
    result: &GetInsightSummariesResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_retrieved_traces_graph_response(
    result: &GetRetrievedTracesGraphResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sampling_rules_response(result: &GetSamplingRulesResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sampling_statistic_summaries_response(
    result: &GetSamplingStatisticSummariesResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sampling_targets_response(result: &GetSamplingTargetsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_graph_response(result: &GetServiceGraphResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_time_series_service_statistics_response(
    result: &GetTimeSeriesServiceStatisticsResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trace_graph_response(result: &GetTraceGraphResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trace_segment_destination_response(
    result: &GetTraceSegmentDestinationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trace_summaries_response(result: &GetTraceSummariesResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_policies_response(
    result: &ListResourcePoliciesResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_retrieved_traces_response(
    result: &ListRetrievedTracesResult,
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
pub fn serialize_put_encryption_config_response(
    result: &PutEncryptionConfigResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_telemetry_records_response(
    result: &PutTelemetryRecordsResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_trace_segments_response(result: &PutTraceSegmentsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_trace_retrieval_response(
    result: &StartTraceRetrievalResult,
) -> MockResponse {
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

/// Serialize response for restJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_indexing_rule_response(result: &UpdateIndexingRuleResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_sampling_rule_response(result: &UpdateSamplingRuleResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_trace_segment_destination_response(
    result: &UpdateTraceSegmentDestinationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_traces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetTracesRequest, String> {
    let mut input = BatchGetTracesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetTracesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetTraces request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_trace_retrieval_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelTraceRetrievalRequest, String> {
    let mut input = CancelTraceRetrievalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelTraceRetrievalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelTraceRetrieval request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupRequest, String> {
    let mut input = CreateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_sampling_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSamplingRuleRequest, String> {
    let mut input = CreateSamplingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSamplingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSamplingRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupRequest, String> {
    let mut input = DeleteGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_sampling_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSamplingRuleRequest, String> {
    let mut input = DeleteSamplingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteSamplingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteSamplingRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEncryptionConfigRequest, String> {
    let input = GetEncryptionConfigRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupRequest, String> {
    let mut input = GetGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupsRequest, String> {
    let mut input = GetGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGroupsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetGroups request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_indexing_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIndexingRulesRequest, String> {
    let mut input = GetIndexingRulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetIndexingRulesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetIndexingRules request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insight_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightRequest, String> {
    let mut input = GetInsightRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetInsightRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetInsight request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insight_events_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightEventsRequest, String> {
    let mut input = GetInsightEventsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetInsightEventsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetInsightEvents request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insight_impact_graph_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightImpactGraphRequest, String> {
    let mut input = GetInsightImpactGraphRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetInsightImpactGraphRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetInsightImpactGraph request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_insight_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInsightSummariesRequest, String> {
    let mut input = GetInsightSummariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetInsightSummariesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetInsightSummaries request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_retrieved_traces_graph_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRetrievedTracesGraphRequest, String> {
    let mut input = GetRetrievedTracesGraphRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetRetrievedTracesGraphRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetRetrievedTracesGraph request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sampling_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSamplingRulesRequest, String> {
    let mut input = GetSamplingRulesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSamplingRulesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetSamplingRules request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sampling_statistic_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSamplingStatisticSummariesRequest, String> {
    let mut input = GetSamplingStatisticSummariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSamplingStatisticSummariesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetSamplingStatisticSummaries request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sampling_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSamplingTargetsRequest, String> {
    let mut input = GetSamplingTargetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSamplingTargetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetSamplingTargets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_graph_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceGraphRequest, String> {
    let mut input = GetServiceGraphRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetServiceGraphRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetServiceGraph request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_time_series_service_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTimeSeriesServiceStatisticsRequest, String> {
    let mut input = GetTimeSeriesServiceStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTimeSeriesServiceStatisticsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetTimeSeriesServiceStatistics request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trace_graph_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTraceGraphRequest, String> {
    let mut input = GetTraceGraphRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTraceGraphRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTraceGraph request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trace_segment_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTraceSegmentDestinationRequest, String> {
    let input = GetTraceSegmentDestinationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trace_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTraceSummariesRequest, String> {
    let mut input = GetTraceSummariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTraceSummariesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTraceSummaries request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourcePoliciesRequest, String> {
    let mut input = ListResourcePoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourcePoliciesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListResourcePolicies request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_retrieved_traces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRetrievedTracesRequest, String> {
    let mut input = ListRetrievedTracesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRetrievedTracesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListRetrievedTraces request: {err}"))?;
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
pub fn deserialize_put_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEncryptionConfigRequest, String> {
    let mut input = PutEncryptionConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEncryptionConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutEncryptionConfig request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_telemetry_records_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTelemetryRecordsRequest, String> {
    let mut input = PutTelemetryRecordsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTelemetryRecordsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTelemetryRecords request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_trace_segments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTraceSegmentsRequest, String> {
    let mut input = PutTraceSegmentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTraceSegmentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTraceSegments request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_trace_retrieval_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTraceRetrievalRequest, String> {
    let mut input = StartTraceRetrievalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTraceRetrievalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartTraceRetrieval request: {err}"))?;
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

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupRequest, String> {
    let mut input = UpdateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_indexing_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIndexingRuleRequest, String> {
    let mut input = UpdateIndexingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIndexingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIndexingRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_sampling_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSamplingRuleRequest, String> {
    let mut input = UpdateSamplingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSamplingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSamplingRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_trace_segment_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTraceSegmentDestinationRequest, String> {
    let mut input = UpdateTraceSegmentDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTraceSegmentDestinationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateTraceSegmentDestination request: {err}")
            })?;
    }
    Ok(input)
}
