//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplify

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
pub fn serialize_create_app_response(result: &CreateAppResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_environment_response(
    result: &CreateBackendEnvironmentResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_branch_response(result: &CreateBranchResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deployment_response(result: &CreateDeploymentResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_association_response(
    result: &CreateDomainAssociationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_webhook_response(result: &CreateWebhookResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_response(result: &DeleteAppResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backend_environment_response(
    result: &DeleteBackendEnvironmentResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_branch_response(result: &DeleteBranchResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_domain_association_response(
    result: &DeleteDomainAssociationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_job_response(result: &DeleteJobResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_webhook_response(result: &DeleteWebhookResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_access_logs_response(result: &GenerateAccessLogsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_app_response(result: &GetAppResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_artifact_url_response(result: &GetArtifactUrlResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_environment_response(
    result: &GetBackendEnvironmentResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_branch_response(result: &GetBranchResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_association_response(
    result: &GetDomainAssociationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_job_response(result: &GetJobResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_webhook_response(result: &GetWebhookResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_apps_response(result: &ListAppsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_artifacts_response(result: &ListArtifactsResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backend_environments_response(
    result: &ListBackendEnvironmentsResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_branches_response(result: &ListBranchesResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_associations_response(
    result: &ListDomainAssociationsResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_jobs_response(result: &ListJobsResult) -> MockResponse {
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
pub fn serialize_list_webhooks_response(result: &ListWebhooksResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_deployment_response(result: &StartDeploymentResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_job_response(result: &StartJobResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_job_response(result: &StopJobResult) -> MockResponse {
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
pub fn serialize_update_app_response(result: &UpdateAppResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_branch_response(result: &UpdateBranchResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_association_response(
    result: &UpdateDomainAssociationResult,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_webhook_response(result: &UpdateWebhookResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
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
pub fn deserialize_create_backend_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendEnvironmentRequest, String> {
    let mut input = CreateBackendEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateBackendEnvironment request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_branch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBranchRequest, String> {
    let mut input = CreateBranchRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBranchRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBranch request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeploymentRequest, String> {
    let mut input = CreateDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainAssociationRequest, String> {
    let mut input = CreateDomainAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainAssociationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDomainAssociation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_webhook_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateWebhookRequest, String> {
    let mut input = CreateWebhookRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateWebhookRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateWebhook request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backend_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackendEnvironmentRequest, String> {
    let mut input = DeleteBackendEnvironmentRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "environmentName" => {
                input.environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_branch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBranchRequest, String> {
    let mut input = DeleteBranchRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainAssociationRequest, String> {
    let mut input = DeleteDomainAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobRequest, String> {
    let mut input = DeleteJobRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_webhook_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteWebhookRequest, String> {
    let mut input = DeleteWebhookRequest::default();
    for (name, value) in labels {
        match *name {
            "webhookId" => {
                input.webhook_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_access_logs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateAccessLogsRequest, String> {
    let mut input = GenerateAccessLogsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateAccessLogsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GenerateAccessLogs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_app_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAppRequest, String> {
    let mut input = GetAppRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_artifact_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetArtifactUrlRequest, String> {
    let mut input = GetArtifactUrlRequest::default();
    for (name, value) in labels {
        match *name {
            "artifactId" => {
                input.artifact_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendEnvironmentRequest, String> {
    let mut input = GetBackendEnvironmentRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "environmentName" => {
                input.environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_branch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBranchRequest, String> {
    let mut input = GetBranchRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainAssociationRequest, String> {
    let mut input = GetDomainAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetJobRequest, String> {
    let mut input = GetJobRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_webhook_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetWebhookRequest, String> {
    let mut input = GetWebhookRequest::default();
    for (name, value) in labels {
        match *name {
            "webhookId" => {
                input.webhook_id = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_list_artifacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListArtifactsRequest, String> {
    let mut input = ListArtifactsRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            "jobId" => {
                input.job_id = value.to_string();
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
pub fn deserialize_list_backend_environments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackendEnvironmentsRequest, String> {
    let mut input = ListBackendEnvironmentsRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("environmentName") {
        input.environment_name = Some(value.to_string());
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
pub fn deserialize_list_branches_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBranchesRequest, String> {
    let mut input = ListBranchesRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
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
pub fn deserialize_list_domain_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainAssociationsRequest, String> {
    let mut input = ListDomainAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
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
pub fn deserialize_list_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobsRequest, String> {
    let mut input = ListJobsRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
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
pub fn deserialize_list_webhooks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListWebhooksRequest, String> {
    let mut input = ListWebhooksRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
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
pub fn deserialize_start_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDeploymentRequest, String> {
    let mut input = StartDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartJobRequest, String> {
    let mut input = StartJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopJobRequest, String> {
    let mut input = StopJobRequest::default();
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            "jobId" => {
                input.job_id = value.to_string();
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
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_branch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBranchRequest, String> {
    let mut input = UpdateBranchRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBranchRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBranch request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "branchName" => {
                input.branch_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_domain_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainAssociationRequest, String> {
    let mut input = UpdateDomainAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDomainAssociationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDomainAssociation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "appId" => {
                input.app_id = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_webhook_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWebhookRequest, String> {
    let mut input = UpdateWebhookRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWebhookRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateWebhook request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "webhookId" => {
                input.webhook_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
