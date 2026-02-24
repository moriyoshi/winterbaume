//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-accessanalyzer

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

/// Serialize void response for restJson protocol.
pub fn serialize_apply_archive_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_policy_generation_response(
    result: &CancelPolicyGenerationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_check_access_not_granted_response(
    result: &CheckAccessNotGrantedResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_check_no_new_access_response(result: &CheckNoNewAccessResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_check_no_public_access_response(
    result: &CheckNoPublicAccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_access_preview_response(
    result: &CreateAccessPreviewResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_analyzer_response(result: &CreateAnalyzerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_archive_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_analyzer_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_archive_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_generate_finding_recommendation_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_access_preview_response(result: &GetAccessPreviewResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_analyzed_resource_response(
    result: &GetAnalyzedResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_analyzer_response(result: &GetAnalyzerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_archive_rule_response(result: &GetArchiveRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_response(result: &GetFindingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_recommendation_response(
    result: &GetFindingRecommendationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_finding_v2_response(result: &GetFindingV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_findings_statistics_response(
    result: &GetFindingsStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_generated_policy_response(
    result: &GetGeneratedPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_access_preview_findings_response(
    result: &ListAccessPreviewFindingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_access_previews_response(
    result: &ListAccessPreviewsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_analyzed_resources_response(
    result: &ListAnalyzedResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_analyzers_response(result: &ListAnalyzersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_archive_rules_response(result: &ListArchiveRulesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_findings_response(result: &ListFindingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_findings_v2_response(result: &ListFindingsV2Response) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_policy_generations_response(
    result: &ListPolicyGenerationsResponse,
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
pub fn serialize_start_policy_generation_response(
    result: &StartPolicyGenerationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_start_resource_scan_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
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
pub fn serialize_update_analyzer_response(result: &UpdateAnalyzerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_archive_rule_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_findings_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_validate_policy_response(result: &ValidatePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_apply_archive_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ApplyArchiveRuleRequest, String> {
    let mut input = ApplyArchiveRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ApplyArchiveRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ApplyArchiveRule request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_policy_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelPolicyGenerationRequest, String> {
    let mut input = CancelPolicyGenerationRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_check_access_not_granted_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CheckAccessNotGrantedRequest, String> {
    let mut input = CheckAccessNotGrantedRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CheckAccessNotGrantedRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CheckAccessNotGranted request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_check_no_new_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CheckNoNewAccessRequest, String> {
    let mut input = CheckNoNewAccessRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CheckNoNewAccessRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CheckNoNewAccess request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_check_no_public_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CheckNoPublicAccessRequest, String> {
    let mut input = CheckNoPublicAccessRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CheckNoPublicAccessRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CheckNoPublicAccess request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_access_preview_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessPreviewRequest, String> {
    let mut input = CreateAccessPreviewRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccessPreviewRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAccessPreview request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAnalyzerRequest, String> {
    let mut input = CreateAnalyzerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAnalyzerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAnalyzer request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_archive_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateArchiveRuleRequest, String> {
    let mut input = CreateArchiveRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateArchiveRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateArchiveRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAnalyzerRequest, String> {
    let mut input = DeleteAnalyzerRequest::default();
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_archive_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteArchiveRuleRequest, String> {
    let mut input = DeleteArchiveRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_finding_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateFindingRecommendationRequest, String> {
    let mut input = GenerateFindingRecommendationRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_access_preview_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessPreviewRequest, String> {
    let mut input = GetAccessPreviewRequest::default();
    for (name, value) in labels {
        match *name {
            "accessPreviewId" => {
                input.access_preview_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_analyzed_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAnalyzedResourceRequest, String> {
    let mut input = GetAnalyzedResourceRequest::default();
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAnalyzerRequest, String> {
    let mut input = GetAnalyzerRequest::default();
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_archive_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetArchiveRuleRequest, String> {
    let mut input = GetArchiveRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingRequest, String> {
    let mut input = GetFindingRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_finding_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingRecommendationRequest, String> {
    let mut input = GetFindingRecommendationRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
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
pub fn deserialize_get_finding_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingV2Request, String> {
    let mut input = GetFindingV2Request::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
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
pub fn deserialize_get_findings_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFindingsStatisticsRequest, String> {
    let mut input = GetFindingsStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFindingsStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFindingsStatistics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_generated_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGeneratedPolicyRequest, String> {
    let mut input = GetGeneratedPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeResourcePlaceholders") {
        input.include_resource_placeholders = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("includeServiceLevelTemplate") {
        input.include_service_level_template = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_access_preview_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPreviewFindingsRequest, String> {
    let mut input = ListAccessPreviewFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAccessPreviewFindingsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAccessPreviewFindings request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "accessPreviewId" => {
                input.access_preview_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_access_previews_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPreviewsRequest, String> {
    let mut input = ListAccessPreviewsRequest::default();
    if let Some(value) = query.get("analyzerArn") {
        input.analyzer_arn = value.to_string();
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
pub fn deserialize_list_analyzed_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnalyzedResourcesRequest, String> {
    let mut input = ListAnalyzedResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAnalyzedResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAnalyzedResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_analyzers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnalyzersRequest, String> {
    let mut input = ListAnalyzersRequest::default();
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
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_archive_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListArchiveRulesRequest, String> {
    let mut input = ListArchiveRulesRequest::default();
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_list_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingsRequest, String> {
    let mut input = ListFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_findings_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFindingsV2Request, String> {
    let mut input = ListFindingsV2Request::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFindingsV2Request>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFindingsV2 request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_policy_generations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyGenerationsRequest, String> {
    let mut input = ListPolicyGenerationsRequest::default();
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
    if let Some(value) = query.get("principalArn") {
        input.principal_arn = Some(value.to_string());
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
pub fn deserialize_start_policy_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartPolicyGenerationRequest, String> {
    let mut input = StartPolicyGenerationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartPolicyGenerationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartPolicyGeneration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_resource_scan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartResourceScanRequest, String> {
    let mut input = StartResourceScanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartResourceScanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartResourceScan request: {err}"))?;
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
pub fn deserialize_update_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAnalyzerRequest, String> {
    let mut input = UpdateAnalyzerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAnalyzerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAnalyzer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_archive_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateArchiveRuleRequest, String> {
    let mut input = UpdateArchiveRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateArchiveRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateArchiveRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "analyzerName" => {
                input.analyzer_name = value.to_string();
            }
            "ruleName" => {
                input.rule_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFindingsRequest, String> {
    let mut input = UpdateFindingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFindingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFindings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_validate_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ValidatePolicyRequest, String> {
    let mut input = ValidatePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ValidatePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ValidatePolicy request: {err}"))?;
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
