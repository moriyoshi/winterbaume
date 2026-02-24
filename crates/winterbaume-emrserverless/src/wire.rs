//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emrserverless

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
pub fn serialize_cancel_job_run_response(result: &CancelJobRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_response(result: &GetApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_dashboard_for_job_run_response(
    result: &GetDashboardForJobRunResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_job_run_response(result: &GetJobRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_dashboard_response(
    result: &GetResourceDashboardResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_session_response(result: &GetSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_session_endpoint_response(
    result: &GetSessionEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_job_run_attempts_response(
    result: &ListJobRunAttemptsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_job_runs_response(result: &ListJobRunsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sessions_response(result: &ListSessionsResponse) -> MockResponse {
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
pub fn serialize_start_application_response(result: &StartApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_job_run_response(result: &StartJobRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_session_response(result: &StartSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_application_response(result: &StopApplicationResponse) -> MockResponse {
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
pub fn serialize_terminate_session_response(result: &TerminateSessionResponse) -> MockResponse {
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
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelJobRunRequest, String> {
    let mut input = CancelJobRunRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "jobRunId" => {
                input.job_run_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("shutdownGracePeriodInSeconds") {
        input.shutdown_grace_period_in_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationRequest, String> {
    let mut input = CreateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApplication request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationRequest, String> {
    let mut input = DeleteApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApplicationRequest, String> {
    let mut input = GetApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_dashboard_for_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDashboardForJobRunRequest, String> {
    let mut input = GetDashboardForJobRunRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "jobRunId" => {
                input.job_run_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("accessSystemProfileLogs") {
        input.access_system_profile_logs = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("attempt") {
        input.attempt = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetJobRunRequest, String> {
    let mut input = GetJobRunRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "jobRunId" => {
                input.job_run_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("attempt") {
        input.attempt = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_dashboard_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceDashboardRequest, String> {
    let mut input = GetResourceDashboardRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("resourceId") {
        input.resource_id = value.to_string();
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSessionRequest, String> {
    let mut input = GetSessionRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "sessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_session_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSessionEndpointRequest, String> {
    let mut input = GetSessionEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "sessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_applications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApplicationsRequest, String> {
    let mut input = ListApplicationsRequest::default();
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
    if let Some(value) = query.get("states") {
        input.states = Some(
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
pub fn deserialize_list_job_run_attempts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobRunAttemptsRequest, String> {
    let mut input = ListJobRunAttemptsRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "jobRunId" => {
                input.job_run_id = value.to_string();
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
pub fn deserialize_list_job_runs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobRunsRequest, String> {
    let mut input = ListJobRunsRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("createdAtAfter") {
        input.created_at_after = Some(
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
    if let Some(value) = query.get("createdAtBefore") {
        input.created_at_before = Some(
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
    if let Some(value) = query.get("mode") {
        input.mode = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("states") {
        input.states = Some(
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
pub fn deserialize_list_sessions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSessionsRequest, String> {
    let mut input = ListSessionsRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("createdAtAfter") {
        input.created_at_after = Some(
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
    if let Some(value) = query.get("createdAtBefore") {
        input.created_at_before = Some(
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
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("states") {
        input.states = Some(
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
pub fn deserialize_start_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartApplicationRequest, String> {
    let mut input = StartApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartJobRunRequest, String> {
    let mut input = StartJobRunRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartJobRunRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartJobRun request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSessionRequest, String> {
    let mut input = StartSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartSession request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopApplicationRequest, String> {
    let mut input = StopApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_terminate_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TerminateSessionRequest, String> {
    let mut input = TerminateSessionRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "sessionId" => {
                input.session_id = value.to_string();
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
pub fn deserialize_update_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationRequest, String> {
    let mut input = UpdateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApplication request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
