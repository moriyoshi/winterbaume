//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-resiliencehub

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
pub fn serialize_accept_resource_grouping_recommendations_response(
    result: &AcceptResourceGroupingRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_draft_app_version_resource_mappings_response(
    result: &AddDraftAppVersionResourceMappingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_recommendation_status_response(
    result: &BatchUpdateRecommendationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_app_response(result: &CreateAppResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_app_version_app_component_response(
    result: &CreateAppVersionAppComponentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_app_version_resource_response(
    result: &CreateAppVersionResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_recommendation_template_response(
    result: &CreateRecommendationTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resiliency_policy_response(
    result: &CreateResiliencyPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_response(result: &DeleteAppResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_assessment_response(
    result: &DeleteAppAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_input_source_response(
    result: &DeleteAppInputSourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_version_app_component_response(
    result: &DeleteAppVersionAppComponentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_version_resource_response(
    result: &DeleteAppVersionResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_recommendation_template_response(
    result: &DeleteRecommendationTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resiliency_policy_response(
    result: &DeleteResiliencyPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_response(result: &DescribeAppResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_assessment_response(
    result: &DescribeAppAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_version_response(
    result: &DescribeAppVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_version_app_component_response(
    result: &DescribeAppVersionAppComponentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_version_resource_response(
    result: &DescribeAppVersionResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_version_resources_resolution_status_response(
    result: &DescribeAppVersionResourcesResolutionStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_app_version_template_response(
    result: &DescribeAppVersionTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_draft_app_version_resources_import_status_response(
    result: &DescribeDraftAppVersionResourcesImportStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_metrics_export_response(
    result: &DescribeMetricsExportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_resiliency_policy_response(
    result: &DescribeResiliencyPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_resource_grouping_recommendation_task_response(
    result: &DescribeResourceGroupingRecommendationTaskResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_resources_to_draft_app_version_response(
    result: &ImportResourcesToDraftAppVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_alarm_recommendations_response(
    result: &ListAlarmRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_assessment_compliance_drifts_response(
    result: &ListAppAssessmentComplianceDriftsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_assessment_resource_drifts_response(
    result: &ListAppAssessmentResourceDriftsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_assessments_response(
    result: &ListAppAssessmentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_component_compliances_response(
    result: &ListAppComponentCompliancesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_component_recommendations_response(
    result: &ListAppComponentRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_input_sources_response(
    result: &ListAppInputSourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_version_app_components_response(
    result: &ListAppVersionAppComponentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_version_resource_mappings_response(
    result: &ListAppVersionResourceMappingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_version_resources_response(
    result: &ListAppVersionResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_app_versions_response(result: &ListAppVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_apps_response(result: &ListAppsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_metrics_response(result: &ListMetricsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recommendation_templates_response(
    result: &ListRecommendationTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resiliency_policies_response(
    result: &ListResiliencyPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_grouping_recommendations_response(
    result: &ListResourceGroupingRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sop_recommendations_response(
    result: &ListSopRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_suggested_resiliency_policies_response(
    result: &ListSuggestedResiliencyPoliciesResponse,
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
pub fn serialize_list_test_recommendations_response(
    result: &ListTestRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_unsupported_app_version_resources_response(
    result: &ListUnsupportedAppVersionResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_publish_app_version_response(result: &PublishAppVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_draft_app_version_template_response(
    result: &PutDraftAppVersionTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_resource_grouping_recommendations_response(
    result: &RejectResourceGroupingRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_draft_app_version_resource_mappings_response(
    result: &RemoveDraftAppVersionResourceMappingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_resolve_app_version_resources_response(
    result: &ResolveAppVersionResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_app_assessment_response(
    result: &StartAppAssessmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_metrics_export_response(
    result: &StartMetricsExportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_resource_grouping_recommendation_task_response(
    result: &StartResourceGroupingRecommendationTaskResponse,
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
pub fn serialize_update_app_response(result: &UpdateAppResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_app_version_response(result: &UpdateAppVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_app_version_app_component_response(
    result: &UpdateAppVersionAppComponentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_app_version_resource_response(
    result: &UpdateAppVersionResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resiliency_policy_response(
    result: &UpdateResiliencyPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_resource_grouping_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptResourceGroupingRecommendationsRequest, String> {
    let mut input = AcceptResourceGroupingRecommendationsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<AcceptResourceGroupingRecommendationsRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize AcceptResourceGroupingRecommendations request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_draft_app_version_resource_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddDraftAppVersionResourceMappingsRequest, String> {
    let mut input = AddDraftAppVersionResourceMappingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddDraftAppVersionResourceMappingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AddDraftAppVersionResourceMappings request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_recommendation_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateRecommendationStatusRequest, String> {
    let mut input = BatchUpdateRecommendationStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateRecommendationStatusRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchUpdateRecommendationStatus request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_app_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAppRequest, String> {
    let mut input = CreateAppRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAppRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApp request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_app_version_app_component_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAppVersionAppComponentRequest, String> {
    let mut input = CreateAppVersionAppComponentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAppVersionAppComponentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateAppVersionAppComponent request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_app_version_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAppVersionResourceRequest, String> {
    let mut input = CreateAppVersionResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAppVersionResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAppVersionResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_recommendation_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRecommendationTemplateRequest, String> {
    let mut input = CreateRecommendationTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRecommendationTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateRecommendationTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resiliency_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResiliencyPolicyRequest, String> {
    let mut input = CreateResiliencyPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResiliencyPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateResiliencyPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_app_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAppRequest, String> {
    let mut input = DeleteAppRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAppRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteApp request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_app_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAppAssessmentRequest, String> {
    let mut input = DeleteAppAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAppAssessmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteAppAssessment request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_app_input_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAppInputSourceRequest, String> {
    let mut input = DeleteAppInputSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAppInputSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteAppInputSource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_app_version_app_component_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAppVersionAppComponentRequest, String> {
    let mut input = DeleteAppVersionAppComponentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAppVersionAppComponentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteAppVersionAppComponent request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_app_version_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAppVersionResourceRequest, String> {
    let mut input = DeleteAppVersionResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAppVersionResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteAppVersionResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_recommendation_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRecommendationTemplateRequest, String> {
    let mut input = DeleteRecommendationTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteRecommendationTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteRecommendationTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resiliency_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResiliencyPolicyRequest, String> {
    let mut input = DeleteResiliencyPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteResiliencyPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteResiliencyPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppRequest, String> {
    let mut input = DescribeAppRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeApp request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppAssessmentRequest, String> {
    let mut input = DescribeAppAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppAssessmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeAppAssessment request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppVersionRequest, String> {
    let mut input = DescribeAppVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeAppVersion request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_version_app_component_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppVersionAppComponentRequest, String> {
    let mut input = DescribeAppVersionAppComponentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppVersionAppComponentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeAppVersionAppComponent request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_version_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppVersionResourceRequest, String> {
    let mut input = DescribeAppVersionResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppVersionResourceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeAppVersionResource request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_version_resources_resolution_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppVersionResourcesResolutionStatusRequest, String> {
    let mut input = DescribeAppVersionResourcesResolutionStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppVersionResourcesResolutionStatusRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize DescribeAppVersionResourcesResolutionStatus request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_app_version_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAppVersionTemplateRequest, String> {
    let mut input = DescribeAppVersionTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAppVersionTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeAppVersionTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_draft_app_version_resources_import_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDraftAppVersionResourcesImportStatusRequest, String> {
    let mut input = DescribeDraftAppVersionResourcesImportStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeDraftAppVersionResourcesImportStatusRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize DescribeDraftAppVersionResourcesImportStatus request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_metrics_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMetricsExportRequest, String> {
    let mut input = DescribeMetricsExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeMetricsExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeMetricsExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_resiliency_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeResiliencyPolicyRequest, String> {
    let mut input = DescribeResiliencyPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeResiliencyPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeResiliencyPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_resource_grouping_recommendation_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeResourceGroupingRecommendationTaskRequest, String> {
    let mut input = DescribeResourceGroupingRecommendationTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeResourceGroupingRecommendationTaskRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize DescribeResourceGroupingRecommendationTask request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_resources_to_draft_app_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportResourcesToDraftAppVersionRequest, String> {
    let mut input = ImportResourcesToDraftAppVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportResourcesToDraftAppVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ImportResourcesToDraftAppVersion request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_alarm_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAlarmRecommendationsRequest, String> {
    let mut input = ListAlarmRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAlarmRecommendationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAlarmRecommendations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_assessment_compliance_drifts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppAssessmentComplianceDriftsRequest, String> {
    let mut input = ListAppAssessmentComplianceDriftsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppAssessmentComplianceDriftsRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize ListAppAssessmentComplianceDrifts request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_assessment_resource_drifts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppAssessmentResourceDriftsRequest, String> {
    let mut input = ListAppAssessmentResourceDriftsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppAssessmentResourceDriftsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListAppAssessmentResourceDrifts request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_assessments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppAssessmentsRequest, String> {
    let mut input = ListAppAssessmentsRequest::default();
    if let Some(value) = query.get("appArn") {
        input.app_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("assessmentName") {
        input.assessment_name = Some(value.to_string());
    }
    if let Some(value) = query.get("assessmentStatus") {
        input.assessment_status = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("complianceStatus") {
        input.compliance_status = Some(value.to_string());
    }
    if let Some(value) = query.get("invoker") {
        input.invoker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("reverseOrder") {
        input.reverse_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_component_compliances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppComponentCompliancesRequest, String> {
    let mut input = ListAppComponentCompliancesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppComponentCompliancesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListAppComponentCompliances request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_component_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppComponentRecommendationsRequest, String> {
    let mut input = ListAppComponentRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppComponentRecommendationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListAppComponentRecommendations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_input_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppInputSourcesRequest, String> {
    let mut input = ListAppInputSourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppInputSourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAppInputSources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_version_app_components_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppVersionAppComponentsRequest, String> {
    let mut input = ListAppVersionAppComponentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppVersionAppComponentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListAppVersionAppComponents request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_version_resource_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppVersionResourceMappingsRequest, String> {
    let mut input = ListAppVersionResourceMappingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppVersionResourceMappingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListAppVersionResourceMappings request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_version_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppVersionResourcesRequest, String> {
    let mut input = ListAppVersionResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppVersionResourcesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAppVersionResources request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_app_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppVersionsRequest, String> {
    let mut input = ListAppVersionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppVersionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAppVersions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_apps_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppsRequest, String> {
    let mut input = ListAppsRequest::default();
    if let Some(value) = query.get("appArn") {
        input.app_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("awsApplicationArn") {
        input.aws_application_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("fromLastAssessmentTime") {
        input.from_last_assessment_time = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("reverseOrder") {
        input.reverse_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("toLastAssessmentTime") {
        input.to_last_assessment_time = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMetricsRequest, String> {
    let mut input = ListMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListMetricsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListMetrics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recommendation_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecommendationTemplatesRequest, String> {
    let mut input = ListRecommendationTemplatesRequest::default();
    if let Some(value) = query.get("assessmentArn") {
        input.assessment_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("recommendationTemplateArn") {
        input.recommendation_template_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("reverseOrder") {
        input.reverse_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("status") {
        input.status = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resiliency_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResiliencyPoliciesRequest, String> {
    let mut input = ListResiliencyPoliciesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("policyName") {
        input.policy_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_grouping_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceGroupingRecommendationsRequest, String> {
    let mut input = ListResourceGroupingRecommendationsRequest::default();
    if let Some(value) = query.get("appArn") {
        input.app_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_sop_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSopRecommendationsRequest, String> {
    let mut input = ListSopRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSopRecommendationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListSopRecommendations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_suggested_resiliency_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSuggestedResiliencyPoliciesRequest, String> {
    let mut input = ListSuggestedResiliencyPoliciesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestRecommendationsRequest, String> {
    let mut input = ListTestRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTestRecommendationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListTestRecommendations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_unsupported_app_version_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUnsupportedAppVersionResourcesRequest, String> {
    let mut input = ListUnsupportedAppVersionResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListUnsupportedAppVersionResourcesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListUnsupportedAppVersionResources request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_publish_app_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishAppVersionRequest, String> {
    let mut input = PublishAppVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PublishAppVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PublishAppVersion request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_draft_app_version_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDraftAppVersionTemplateRequest, String> {
    let mut input = PutDraftAppVersionTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDraftAppVersionTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDraftAppVersionTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_resource_grouping_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectResourceGroupingRecommendationsRequest, String> {
    let mut input = RejectResourceGroupingRecommendationsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<RejectResourceGroupingRecommendationsRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize RejectResourceGroupingRecommendations request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_draft_app_version_resource_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveDraftAppVersionResourceMappingsRequest, String> {
    let mut input = RemoveDraftAppVersionResourceMappingsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<RemoveDraftAppVersionResourceMappingsRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize RemoveDraftAppVersionResourceMappings request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_resolve_app_version_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResolveAppVersionResourcesRequest, String> {
    let mut input = ResolveAppVersionResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResolveAppVersionResourcesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ResolveAppVersionResources request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_app_assessment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAppAssessmentRequest, String> {
    let mut input = StartAppAssessmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAppAssessmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartAppAssessment request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_metrics_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMetricsExportRequest, String> {
    let mut input = StartMetricsExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartMetricsExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartMetricsExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_resource_grouping_recommendation_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartResourceGroupingRecommendationTaskRequest, String> {
    let mut input = StartResourceGroupingRecommendationTaskRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartResourceGroupingRecommendationTaskRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize StartResourceGroupingRecommendationTask request: {err}")
        })?;
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_app_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAppRequest, String> {
    let mut input = UpdateAppRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAppRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApp request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_app_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAppVersionRequest, String> {
    let mut input = UpdateAppVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAppVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAppVersion request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_app_version_app_component_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAppVersionAppComponentRequest, String> {
    let mut input = UpdateAppVersionAppComponentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAppVersionAppComponentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAppVersionAppComponent request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_app_version_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAppVersionResourceRequest, String> {
    let mut input = UpdateAppVersionResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAppVersionResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAppVersionResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resiliency_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResiliencyPolicyRequest, String> {
    let mut input = UpdateResiliencyPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResiliencyPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateResiliencyPolicy request: {err}"),
        )?;
    }
    Ok(input)
}
