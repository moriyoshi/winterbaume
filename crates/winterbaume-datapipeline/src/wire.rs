//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-data-pipeline

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_activate_pipeline_response(result: &ActivatePipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_response(result: &AddTagsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_pipeline_response(result: &CreatePipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deactivate_pipeline_response(result: &DeactivatePipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_pipeline_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_objects_response(result: &DescribeObjectsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pipelines_response(result: &DescribePipelinesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_evaluate_expression_response(result: &EvaluateExpressionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pipeline_definition_response(
    result: &GetPipelineDefinitionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_pipelines_response(result: &ListPipelinesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_poll_for_task_response(result: &PollForTaskOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_pipeline_definition_response(
    result: &PutPipelineDefinitionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_query_objects_response(result: &QueryObjectsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_response(result: &RemoveTagsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_report_task_progress_response(result: &ReportTaskProgressOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_report_task_runner_heartbeat_response(
    result: &ReportTaskRunnerHeartbeatOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_status_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_task_status_response(result: &SetTaskStatusOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_validate_pipeline_definition_response(
    result: &ValidatePipelineDefinitionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_activate_pipeline_request(body: &[u8]) -> Result<ActivatePipelineInput, String> {
    if body.is_empty() {
        return Ok(ActivatePipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ActivatePipeline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_request(body: &[u8]) -> Result<AddTagsInput, String> {
    if body.is_empty() {
        return Ok(AddTagsInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize AddTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_pipeline_request(body: &[u8]) -> Result<CreatePipelineInput, String> {
    if body.is_empty() {
        return Ok(CreatePipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePipeline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deactivate_pipeline_request(
    body: &[u8],
) -> Result<DeactivatePipelineInput, String> {
    if body.is_empty() {
        return Ok(DeactivatePipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeactivatePipeline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_pipeline_request(body: &[u8]) -> Result<DeletePipelineInput, String> {
    if body.is_empty() {
        return Ok(DeletePipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePipeline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_objects_request(body: &[u8]) -> Result<DescribeObjectsInput, String> {
    if body.is_empty() {
        return Ok(DescribeObjectsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeObjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pipelines_request(
    body: &[u8],
) -> Result<DescribePipelinesInput, String> {
    if body.is_empty() {
        return Ok(DescribePipelinesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePipelines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_evaluate_expression_request(
    body: &[u8],
) -> Result<EvaluateExpressionInput, String> {
    if body.is_empty() {
        return Ok(EvaluateExpressionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EvaluateExpression request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pipeline_definition_request(
    body: &[u8],
) -> Result<GetPipelineDefinitionInput, String> {
    if body.is_empty() {
        return Ok(GetPipelineDefinitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPipelineDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_pipelines_request(body: &[u8]) -> Result<ListPipelinesInput, String> {
    if body.is_empty() {
        return Ok(ListPipelinesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPipelines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_poll_for_task_request(body: &[u8]) -> Result<PollForTaskInput, String> {
    if body.is_empty() {
        return Ok(PollForTaskInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PollForTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_pipeline_definition_request(
    body: &[u8],
) -> Result<PutPipelineDefinitionInput, String> {
    if body.is_empty() {
        return Ok(PutPipelineDefinitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutPipelineDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_query_objects_request(body: &[u8]) -> Result<QueryObjectsInput, String> {
    if body.is_empty() {
        return Ok(QueryObjectsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize QueryObjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_request(body: &[u8]) -> Result<RemoveTagsInput, String> {
    if body.is_empty() {
        return Ok(RemoveTagsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_report_task_progress_request(
    body: &[u8],
) -> Result<ReportTaskProgressInput, String> {
    if body.is_empty() {
        return Ok(ReportTaskProgressInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReportTaskProgress request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_report_task_runner_heartbeat_request(
    body: &[u8],
) -> Result<ReportTaskRunnerHeartbeatInput, String> {
    if body.is_empty() {
        return Ok(ReportTaskRunnerHeartbeatInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReportTaskRunnerHeartbeat request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_status_request(body: &[u8]) -> Result<SetStatusInput, String> {
    if body.is_empty() {
        return Ok(SetStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_task_status_request(body: &[u8]) -> Result<SetTaskStatusInput, String> {
    if body.is_empty() {
        return Ok(SetTaskStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetTaskStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_validate_pipeline_definition_request(
    body: &[u8],
) -> Result<ValidatePipelineDefinitionInput, String> {
    if body.is_empty() {
        return Ok(ValidatePipelineDefinitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ValidatePipelineDefinition request: {e}"))
}
