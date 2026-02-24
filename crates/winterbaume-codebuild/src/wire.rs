//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codebuild

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_builds_response(result: &BatchDeleteBuildsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_build_batches_response(
    result: &BatchGetBuildBatchesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_builds_response(result: &BatchGetBuildsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_command_executions_response(
    result: &BatchGetCommandExecutionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_fleets_response(result: &BatchGetFleetsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_projects_response(result: &BatchGetProjectsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_report_groups_response(
    result: &BatchGetReportGroupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_reports_response(result: &BatchGetReportsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_sandboxes_response(result: &BatchGetSandboxesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_fleet_response(result: &CreateFleetOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_project_response(result: &CreateProjectOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_report_group_response(result: &CreateReportGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_webhook_response(result: &CreateWebhookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_build_batch_response(result: &DeleteBuildBatchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_fleet_response(result: &DeleteFleetOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_project_response(result: &DeleteProjectOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_report_response(result: &DeleteReportOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_report_group_response(result: &DeleteReportGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_source_credentials_response(
    result: &DeleteSourceCredentialsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_webhook_response(result: &DeleteWebhookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_code_coverages_response(
    result: &DescribeCodeCoveragesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_test_cases_response(result: &DescribeTestCasesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_report_group_trend_response(
    result: &GetReportGroupTrendOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_source_credentials_response(
    result: &ImportSourceCredentialsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_invalidate_project_cache_response(
    result: &InvalidateProjectCacheOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_build_batches_response(result: &ListBuildBatchesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_build_batches_for_project_response(
    result: &ListBuildBatchesForProjectOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_builds_response(result: &ListBuildsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_builds_for_project_response(
    result: &ListBuildsForProjectOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_command_executions_for_sandbox_response(
    result: &ListCommandExecutionsForSandboxOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_curated_environment_images_response(
    result: &ListCuratedEnvironmentImagesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_fleets_response(result: &ListFleetsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_projects_response(result: &ListProjectsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_report_groups_response(result: &ListReportGroupsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_reports_response(result: &ListReportsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_reports_for_report_group_response(
    result: &ListReportsForReportGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sandboxes_response(result: &ListSandboxesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sandboxes_for_project_response(
    result: &ListSandboxesForProjectOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_shared_projects_response(result: &ListSharedProjectsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_shared_report_groups_response(
    result: &ListSharedReportGroupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_source_credentials_response(
    result: &ListSourceCredentialsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_retry_build_response(result: &RetryBuildOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_retry_build_batch_response(result: &RetryBuildBatchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_build_response(result: &StartBuildOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_build_batch_response(result: &StartBuildBatchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_command_execution_response(
    result: &StartCommandExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_sandbox_response(result: &StartSandboxOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_sandbox_connection_response(
    result: &StartSandboxConnectionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_build_response(result: &StopBuildOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_build_batch_response(result: &StopBuildBatchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_sandbox_response(result: &StopSandboxOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_fleet_response(result: &UpdateFleetOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_project_response(result: &UpdateProjectOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_project_visibility_response(
    result: &UpdateProjectVisibilityOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_report_group_response(result: &UpdateReportGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_webhook_response(result: &UpdateWebhookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_builds_request(
    body: &[u8],
) -> Result<BatchDeleteBuildsInput, String> {
    if body.is_empty() {
        return Ok(BatchDeleteBuildsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteBuilds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_build_batches_request(
    body: &[u8],
) -> Result<BatchGetBuildBatchesInput, String> {
    if body.is_empty() {
        return Ok(BatchGetBuildBatchesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetBuildBatches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_builds_request(body: &[u8]) -> Result<BatchGetBuildsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetBuildsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetBuilds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_command_executions_request(
    body: &[u8],
) -> Result<BatchGetCommandExecutionsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetCommandExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCommandExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_fleets_request(body: &[u8]) -> Result<BatchGetFleetsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetFleetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetFleets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_projects_request(
    body: &[u8],
) -> Result<BatchGetProjectsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetProjectsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetProjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_report_groups_request(
    body: &[u8],
) -> Result<BatchGetReportGroupsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetReportGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetReportGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_reports_request(body: &[u8]) -> Result<BatchGetReportsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetReportsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetReports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_sandboxes_request(
    body: &[u8],
) -> Result<BatchGetSandboxesInput, String> {
    if body.is_empty() {
        return Ok(BatchGetSandboxesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetSandboxes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_fleet_request(body: &[u8]) -> Result<CreateFleetInput, String> {
    if body.is_empty() {
        return Ok(CreateFleetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFleet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_project_request(body: &[u8]) -> Result<CreateProjectInput, String> {
    if body.is_empty() {
        return Ok(CreateProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_report_group_request(
    body: &[u8],
) -> Result<CreateReportGroupInput, String> {
    if body.is_empty() {
        return Ok(CreateReportGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateReportGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_webhook_request(body: &[u8]) -> Result<CreateWebhookInput, String> {
    if body.is_empty() {
        return Ok(CreateWebhookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWebhook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_build_batch_request(
    body: &[u8],
) -> Result<DeleteBuildBatchInput, String> {
    if body.is_empty() {
        return Ok(DeleteBuildBatchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBuildBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_fleet_request(body: &[u8]) -> Result<DeleteFleetInput, String> {
    if body.is_empty() {
        return Ok(DeleteFleetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFleet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_project_request(body: &[u8]) -> Result<DeleteProjectInput, String> {
    if body.is_empty() {
        return Ok(DeleteProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_report_request(body: &[u8]) -> Result<DeleteReportInput, String> {
    if body.is_empty() {
        return Ok(DeleteReportInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_report_group_request(
    body: &[u8],
) -> Result<DeleteReportGroupInput, String> {
    if body.is_empty() {
        return Ok(DeleteReportGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReportGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_source_credentials_request(
    body: &[u8],
) -> Result<DeleteSourceCredentialsInput, String> {
    if body.is_empty() {
        return Ok(DeleteSourceCredentialsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSourceCredentials request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_webhook_request(body: &[u8]) -> Result<DeleteWebhookInput, String> {
    if body.is_empty() {
        return Ok(DeleteWebhookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebhook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_code_coverages_request(
    body: &[u8],
) -> Result<DescribeCodeCoveragesInput, String> {
    if body.is_empty() {
        return Ok(DescribeCodeCoveragesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCodeCoverages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_test_cases_request(
    body: &[u8],
) -> Result<DescribeTestCasesInput, String> {
    if body.is_empty() {
        return Ok(DescribeTestCasesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTestCases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_report_group_trend_request(
    body: &[u8],
) -> Result<GetReportGroupTrendInput, String> {
    if body.is_empty() {
        return Ok(GetReportGroupTrendInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetReportGroupTrend request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_source_credentials_request(
    body: &[u8],
) -> Result<ImportSourceCredentialsInput, String> {
    if body.is_empty() {
        return Ok(ImportSourceCredentialsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportSourceCredentials request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_invalidate_project_cache_request(
    body: &[u8],
) -> Result<InvalidateProjectCacheInput, String> {
    if body.is_empty() {
        return Ok(InvalidateProjectCacheInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize InvalidateProjectCache request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_build_batches_request(
    body: &[u8],
) -> Result<ListBuildBatchesInput, String> {
    if body.is_empty() {
        return Ok(ListBuildBatchesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBuildBatches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_build_batches_for_project_request(
    body: &[u8],
) -> Result<ListBuildBatchesForProjectInput, String> {
    if body.is_empty() {
        return Ok(ListBuildBatchesForProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBuildBatchesForProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_builds_request(body: &[u8]) -> Result<ListBuildsInput, String> {
    if body.is_empty() {
        return Ok(ListBuildsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBuilds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_builds_for_project_request(
    body: &[u8],
) -> Result<ListBuildsForProjectInput, String> {
    if body.is_empty() {
        return Ok(ListBuildsForProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBuildsForProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_command_executions_for_sandbox_request(
    body: &[u8],
) -> Result<ListCommandExecutionsForSandboxInput, String> {
    if body.is_empty() {
        return Ok(ListCommandExecutionsForSandboxInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCommandExecutionsForSandbox request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_curated_environment_images_request(
    body: &[u8],
) -> Result<ListCuratedEnvironmentImagesInput, String> {
    if body.is_empty() {
        return Ok(ListCuratedEnvironmentImagesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCuratedEnvironmentImages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_fleets_request(body: &[u8]) -> Result<ListFleetsInput, String> {
    if body.is_empty() {
        return Ok(ListFleetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFleets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_projects_request(body: &[u8]) -> Result<ListProjectsInput, String> {
    if body.is_empty() {
        return Ok(ListProjectsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_report_groups_request(
    body: &[u8],
) -> Result<ListReportGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListReportGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListReportGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_reports_request(body: &[u8]) -> Result<ListReportsInput, String> {
    if body.is_empty() {
        return Ok(ListReportsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListReports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_reports_for_report_group_request(
    body: &[u8],
) -> Result<ListReportsForReportGroupInput, String> {
    if body.is_empty() {
        return Ok(ListReportsForReportGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListReportsForReportGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_sandboxes_request(body: &[u8]) -> Result<ListSandboxesInput, String> {
    if body.is_empty() {
        return Ok(ListSandboxesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSandboxes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_sandboxes_for_project_request(
    body: &[u8],
) -> Result<ListSandboxesForProjectInput, String> {
    if body.is_empty() {
        return Ok(ListSandboxesForProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSandboxesForProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_shared_projects_request(
    body: &[u8],
) -> Result<ListSharedProjectsInput, String> {
    if body.is_empty() {
        return Ok(ListSharedProjectsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSharedProjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_shared_report_groups_request(
    body: &[u8],
) -> Result<ListSharedReportGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListSharedReportGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSharedReportGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_source_credentials_request(
    body: &[u8],
) -> Result<ListSourceCredentialsInput, String> {
    if body.is_empty() {
        return Ok(ListSourceCredentialsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSourceCredentials request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_retry_build_request(body: &[u8]) -> Result<RetryBuildInput, String> {
    if body.is_empty() {
        return Ok(RetryBuildInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RetryBuild request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_retry_build_batch_request(body: &[u8]) -> Result<RetryBuildBatchInput, String> {
    if body.is_empty() {
        return Ok(RetryBuildBatchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RetryBuildBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_build_request(body: &[u8]) -> Result<StartBuildInput, String> {
    if body.is_empty() {
        return Ok(StartBuildInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartBuild request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_build_batch_request(body: &[u8]) -> Result<StartBuildBatchInput, String> {
    if body.is_empty() {
        return Ok(StartBuildBatchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartBuildBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_command_execution_request(
    body: &[u8],
) -> Result<StartCommandExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartCommandExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCommandExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_sandbox_request(body: &[u8]) -> Result<StartSandboxInput, String> {
    if body.is_empty() {
        return Ok(StartSandboxInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSandbox request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_sandbox_connection_request(
    body: &[u8],
) -> Result<StartSandboxConnectionInput, String> {
    if body.is_empty() {
        return Ok(StartSandboxConnectionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSandboxConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_build_request(body: &[u8]) -> Result<StopBuildInput, String> {
    if body.is_empty() {
        return Ok(StopBuildInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopBuild request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_build_batch_request(body: &[u8]) -> Result<StopBuildBatchInput, String> {
    if body.is_empty() {
        return Ok(StopBuildBatchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopBuildBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_sandbox_request(body: &[u8]) -> Result<StopSandboxInput, String> {
    if body.is_empty() {
        return Ok(StopSandboxInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopSandbox request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_fleet_request(body: &[u8]) -> Result<UpdateFleetInput, String> {
    if body.is_empty() {
        return Ok(UpdateFleetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFleet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_project_request(body: &[u8]) -> Result<UpdateProjectInput, String> {
    if body.is_empty() {
        return Ok(UpdateProjectInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_project_visibility_request(
    body: &[u8],
) -> Result<UpdateProjectVisibilityInput, String> {
    if body.is_empty() {
        return Ok(UpdateProjectVisibilityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProjectVisibility request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_report_group_request(
    body: &[u8],
) -> Result<UpdateReportGroupInput, String> {
    if body.is_empty() {
        return Ok(UpdateReportGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateReportGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_webhook_request(body: &[u8]) -> Result<UpdateWebhookInput, String> {
    if body.is_empty() {
        return Ok(UpdateWebhookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWebhook request: {e}"))
}
