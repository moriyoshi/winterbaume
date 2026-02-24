//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emrcontainers

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
pub fn serialize_create_job_template_response(result: &CreateJobTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_managed_endpoint_response(
    result: &CreateManagedEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_security_configuration_response(
    result: &CreateSecurityConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_virtual_cluster_response(
    result: &CreateVirtualClusterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_job_template_response(result: &DeleteJobTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_managed_endpoint_response(
    result: &DeleteManagedEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_virtual_cluster_response(
    result: &DeleteVirtualClusterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_run_response(result: &DescribeJobRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_template_response(
    result: &DescribeJobTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_managed_endpoint_response(
    result: &DescribeManagedEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_security_configuration_response(
    result: &DescribeSecurityConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_virtual_cluster_response(
    result: &DescribeVirtualClusterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_managed_endpoint_session_credentials_response(
    result: &GetManagedEndpointSessionCredentialsResponse,
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
pub fn serialize_list_job_templates_response(result: &ListJobTemplatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_managed_endpoints_response(
    result: &ListManagedEndpointsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_security_configurations_response(
    result: &ListSecurityConfigurationsResponse,
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
pub fn serialize_list_virtual_clusters_response(
    result: &ListVirtualClustersResponse,
) -> MockResponse {
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
pub fn deserialize_cancel_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelJobRunRequest, String> {
    let mut input = CancelJobRunRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateJobTemplateRequest, String> {
    let mut input = CreateJobTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateJobTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateJobTemplate request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_managed_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateManagedEndpointRequest, String> {
    let mut input = CreateManagedEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateManagedEndpointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateManagedEndpoint request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_security_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSecurityConfigurationRequest, String> {
    let mut input = CreateSecurityConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSecurityConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateSecurityConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_virtual_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualClusterRequest, String> {
    let mut input = CreateVirtualClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVirtualClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVirtualCluster request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobTemplateRequest, String> {
    let mut input = DeleteJobTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_managed_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteManagedEndpointRequest, String> {
    let mut input = DeleteManagedEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_virtual_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualClusterRequest, String> {
    let mut input = DeleteVirtualClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobRunRequest, String> {
    let mut input = DescribeJobRunRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobTemplateRequest, String> {
    let mut input = DescribeJobTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_managed_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeManagedEndpointRequest, String> {
    let mut input = DescribeManagedEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_security_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSecurityConfigurationRequest, String> {
    let mut input = DescribeSecurityConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_virtual_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVirtualClusterRequest, String> {
    let mut input = DescribeVirtualClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_managed_endpoint_session_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetManagedEndpointSessionCredentialsRequest, String> {
    let mut input = GetManagedEndpointSessionCredentialsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<GetManagedEndpointSessionCredentialsRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize GetManagedEndpointSessionCredentials request: {err}"
                    )
                })?;
    }
    for (name, value) in labels {
        match *name {
            "endpointIdentifier" => {
                input.endpoint_identifier = value.to_string();
            }
            "virtualClusterIdentifier" => {
                input.virtual_cluster_identifier = value.to_string();
            }
            _ => {}
        }
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
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
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
pub fn deserialize_list_job_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobTemplatesRequest, String> {
    let mut input = ListJobTemplatesRequest::default();
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
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
pub fn deserialize_list_managed_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListManagedEndpointsRequest, String> {
    let mut input = ListManagedEndpointsRequest::default();
    for (name, value) in labels {
        match *name {
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
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
    if let Some(value) = query.get("types") {
        input.types = Some(
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
pub fn deserialize_list_security_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSecurityConfigurationsRequest, String> {
    let mut input = ListSecurityConfigurationsRequest::default();
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
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
pub fn deserialize_list_virtual_clusters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualClustersRequest, String> {
    let mut input = ListVirtualClustersRequest::default();
    if let Some(value) = query.get("containerProviderId") {
        input.container_provider_id = Some(value.to_string());
    }
    if let Some(value) = query.get("containerProviderType") {
        input.container_provider_type = Some(value.to_string());
    }
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
    }
    if let Some(value) = query.get("eksAccessEntryIntegrated") {
        input.eks_access_entry_integrated = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
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
            "virtualClusterId" => {
                input.virtual_cluster_id = value.to_string();
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
