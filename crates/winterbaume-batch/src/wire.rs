//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-batch

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
pub fn serialize_cancel_job_response(result: &CancelJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_compute_environment_response(
    result: &CreateComputeEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_consumable_resource_response(
    result: &CreateConsumableResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_job_queue_response(result: &CreateJobQueueResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_quota_share_response(result: &CreateQuotaShareResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_scheduling_policy_response(
    result: &CreateSchedulingPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_environment_response(
    result: &CreateServiceEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_compute_environment_response(
    result: &DeleteComputeEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_consumable_resource_response(
    result: &DeleteConsumableResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_job_queue_response(result: &DeleteJobQueueResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_quota_share_response(result: &DeleteQuotaShareResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_scheduling_policy_response(
    result: &DeleteSchedulingPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_environment_response(
    result: &DeleteServiceEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_job_definition_response(
    result: &DeregisterJobDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_compute_environments_response(
    result: &DescribeComputeEnvironmentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_consumable_resource_response(
    result: &DescribeConsumableResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_definitions_response(
    result: &DescribeJobDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_queues_response(result: &DescribeJobQueuesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_jobs_response(result: &DescribeJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_quota_share_response(
    result: &DescribeQuotaShareResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_scheduling_policies_response(
    result: &DescribeSchedulingPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_service_environments_response(
    result: &DescribeServiceEnvironmentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_service_job_response(
    result: &DescribeServiceJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_job_queue_snapshot_response(
    result: &GetJobQueueSnapshotResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_consumable_resources_response(
    result: &ListConsumableResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_jobs_response(result: &ListJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_jobs_by_consumable_resource_response(
    result: &ListJobsByConsumableResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_quota_shares_response(result: &ListQuotaSharesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scheduling_policies_response(
    result: &ListSchedulingPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_jobs_response(result: &ListServiceJobsResponse) -> MockResponse {
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
pub fn serialize_register_job_definition_response(
    result: &RegisterJobDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_submit_job_response(result: &SubmitJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_submit_service_job_response(result: &SubmitServiceJobResponse) -> MockResponse {
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
pub fn serialize_terminate_job_response(result: &TerminateJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_terminate_service_job_response(
    result: &TerminateServiceJobResponse,
) -> MockResponse {
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
pub fn serialize_update_compute_environment_response(
    result: &UpdateComputeEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_consumable_resource_response(
    result: &UpdateConsumableResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_job_queue_response(result: &UpdateJobQueueResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_quota_share_response(result: &UpdateQuotaShareResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_scheduling_policy_response(
    result: &UpdateSchedulingPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_service_environment_response(
    result: &UpdateServiceEnvironmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_service_job_response(result: &UpdateServiceJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelJobRequest, String> {
    let mut input = CancelJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_compute_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateComputeEnvironmentRequest, String> {
    let mut input = CreateComputeEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateComputeEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateComputeEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_consumable_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConsumableResourceRequest, String> {
    let mut input = CreateConsumableResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConsumableResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConsumableResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_job_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateJobQueueRequest, String> {
    let mut input = CreateJobQueueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateJobQueueRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateJobQueue request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_quota_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateQuotaShareRequest, String> {
    let mut input = CreateQuotaShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateQuotaShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateQuotaShare request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_scheduling_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSchedulingPolicyRequest, String> {
    let mut input = CreateSchedulingPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSchedulingPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateSchedulingPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceEnvironmentRequest, String> {
    let mut input = CreateServiceEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateServiceEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_compute_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteComputeEnvironmentRequest, String> {
    let mut input = DeleteComputeEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteComputeEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteComputeEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_consumable_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConsumableResourceRequest, String> {
    let mut input = DeleteConsumableResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteConsumableResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteConsumableResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_job_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteJobQueueRequest, String> {
    let mut input = DeleteJobQueueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteJobQueueRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteJobQueue request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_quota_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQuotaShareRequest, String> {
    let mut input = DeleteQuotaShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteQuotaShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteQuotaShare request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_scheduling_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSchedulingPolicyRequest, String> {
    let mut input = DeleteSchedulingPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteSchedulingPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteSchedulingPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceEnvironmentRequest, String> {
    let mut input = DeleteServiceEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteServiceEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteServiceEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_job_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterJobDefinitionRequest, String> {
    let mut input = DeregisterJobDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeregisterJobDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeregisterJobDefinition request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_compute_environments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeComputeEnvironmentsRequest, String> {
    let mut input = DescribeComputeEnvironmentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeComputeEnvironmentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeComputeEnvironments request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_consumable_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConsumableResourceRequest, String> {
    let mut input = DescribeConsumableResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeConsumableResourceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeConsumableResource request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobDefinitionsRequest, String> {
    let mut input = DescribeJobDefinitionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeJobDefinitionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeJobDefinitions request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_job_queues_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobQueuesRequest, String> {
    let mut input = DescribeJobQueuesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeJobQueuesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeJobQueues request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeJobsRequest, String> {
    let mut input = DescribeJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeJobs request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_quota_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQuotaShareRequest, String> {
    let mut input = DescribeQuotaShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeQuotaShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeQuotaShare request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_scheduling_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSchedulingPoliciesRequest, String> {
    let mut input = DescribeSchedulingPoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSchedulingPoliciesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeSchedulingPolicies request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_service_environments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeServiceEnvironmentsRequest, String> {
    let mut input = DescribeServiceEnvironmentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeServiceEnvironmentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeServiceEnvironments request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_service_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeServiceJobRequest, String> {
    let mut input = DescribeServiceJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeServiceJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeServiceJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_job_queue_snapshot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetJobQueueSnapshotRequest, String> {
    let mut input = GetJobQueueSnapshotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetJobQueueSnapshotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetJobQueueSnapshot request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_consumable_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConsumableResourcesRequest, String> {
    let mut input = ListConsumableResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListConsumableResourcesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListConsumableResources request: {err}"),
        )?;
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
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListJobs request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_jobs_by_consumable_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListJobsByConsumableResourceRequest, String> {
    let mut input = ListJobsByConsumableResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListJobsByConsumableResourceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListJobsByConsumableResource request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_quota_shares_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQuotaSharesRequest, String> {
    let mut input = ListQuotaSharesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListQuotaSharesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListQuotaShares request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scheduling_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSchedulingPoliciesRequest, String> {
    let mut input = ListSchedulingPoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSchedulingPoliciesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListSchedulingPolicies request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceJobsRequest, String> {
    let mut input = ListServiceJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListServiceJobs request: {err}"))?;
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
pub fn deserialize_register_job_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterJobDefinitionRequest, String> {
    let mut input = RegisterJobDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterJobDefinitionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterJobDefinition request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_submit_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SubmitJobRequest, String> {
    let mut input = SubmitJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SubmitJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SubmitJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_submit_service_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SubmitServiceJobRequest, String> {
    let mut input = SubmitServiceJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SubmitServiceJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SubmitServiceJob request: {err}"))?;
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
pub fn deserialize_terminate_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TerminateJobRequest, String> {
    let mut input = TerminateJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TerminateJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TerminateJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_terminate_service_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TerminateServiceJobRequest, String> {
    let mut input = TerminateServiceJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TerminateServiceJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TerminateServiceJob request: {err}"))?;
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
pub fn deserialize_update_compute_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateComputeEnvironmentRequest, String> {
    let mut input = UpdateComputeEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateComputeEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateComputeEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_consumable_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConsumableResourceRequest, String> {
    let mut input = UpdateConsumableResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConsumableResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateConsumableResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_job_queue_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateJobQueueRequest, String> {
    let mut input = UpdateJobQueueRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateJobQueueRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateJobQueue request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_quota_share_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQuotaShareRequest, String> {
    let mut input = UpdateQuotaShareRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQuotaShareRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateQuotaShare request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_scheduling_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSchedulingPolicyRequest, String> {
    let mut input = UpdateSchedulingPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSchedulingPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateSchedulingPolicy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_service_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceEnvironmentRequest, String> {
    let mut input = UpdateServiceEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceEnvironmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateServiceEnvironment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_service_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceJobRequest, String> {
    let mut input = UpdateServiceJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateServiceJob request: {err}"))?;
    }
    Ok(input)
}
