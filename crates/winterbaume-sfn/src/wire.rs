//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sfn

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_activity_response(result: &CreateActivityOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_state_machine_response(result: &CreateStateMachineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_state_machine_alias_response(
    result: &CreateStateMachineAliasOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_activity_response(result: &DeleteActivityOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_state_machine_response(result: &DeleteStateMachineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_state_machine_alias_response(
    result: &DeleteStateMachineAliasOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_state_machine_version_response(
    result: &DeleteStateMachineVersionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_activity_response(result: &DescribeActivityOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_execution_response(result: &DescribeExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_map_run_response(result: &DescribeMapRunOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_state_machine_response(
    result: &DescribeStateMachineOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_state_machine_alias_response(
    result: &DescribeStateMachineAliasOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_state_machine_for_execution_response(
    result: &DescribeStateMachineForExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_activity_task_response(result: &GetActivityTaskOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_execution_history_response(
    result: &GetExecutionHistoryOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_activities_response(result: &ListActivitiesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_executions_response(result: &ListExecutionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_map_runs_response(result: &ListMapRunsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_state_machine_aliases_response(
    result: &ListStateMachineAliasesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_state_machine_versions_response(
    result: &ListStateMachineVersionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_state_machines_response(result: &ListStateMachinesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_publish_state_machine_version_response(
    result: &PublishStateMachineVersionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_redrive_execution_response(result: &RedriveExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_task_failure_response(result: &SendTaskFailureOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_task_heartbeat_response(result: &SendTaskHeartbeatOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_task_success_response(result: &SendTaskSuccessOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_execution_response(result: &StartExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_sync_execution_response(result: &StartSyncExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_execution_response(result: &StopExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_state_response(result: &TestStateOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_map_run_response(result: &UpdateMapRunOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_state_machine_response(result: &UpdateStateMachineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_state_machine_alias_response(
    result: &UpdateStateMachineAliasOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_validate_state_machine_definition_response(
    result: &ValidateStateMachineDefinitionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_activity_request(body: &[u8]) -> Result<CreateActivityInput, String> {
    if body.is_empty() {
        return Ok(CreateActivityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateActivity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_state_machine_request(
    body: &[u8],
) -> Result<CreateStateMachineInput, String> {
    if body.is_empty() {
        return Ok(CreateStateMachineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStateMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_state_machine_alias_request(
    body: &[u8],
) -> Result<CreateStateMachineAliasInput, String> {
    if body.is_empty() {
        return Ok(CreateStateMachineAliasInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStateMachineAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_activity_request(body: &[u8]) -> Result<DeleteActivityInput, String> {
    if body.is_empty() {
        return Ok(DeleteActivityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteActivity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_state_machine_request(
    body: &[u8],
) -> Result<DeleteStateMachineInput, String> {
    if body.is_empty() {
        return Ok(DeleteStateMachineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStateMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_state_machine_alias_request(
    body: &[u8],
) -> Result<DeleteStateMachineAliasInput, String> {
    if body.is_empty() {
        return Ok(DeleteStateMachineAliasInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStateMachineAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_state_machine_version_request(
    body: &[u8],
) -> Result<DeleteStateMachineVersionInput, String> {
    if body.is_empty() {
        return Ok(DeleteStateMachineVersionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStateMachineVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_activity_request(body: &[u8]) -> Result<DescribeActivityInput, String> {
    if body.is_empty() {
        return Ok(DescribeActivityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeActivity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_execution_request(
    body: &[u8],
) -> Result<DescribeExecutionInput, String> {
    if body.is_empty() {
        return Ok(DescribeExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_map_run_request(body: &[u8]) -> Result<DescribeMapRunInput, String> {
    if body.is_empty() {
        return Ok(DescribeMapRunInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMapRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_state_machine_request(
    body: &[u8],
) -> Result<DescribeStateMachineInput, String> {
    if body.is_empty() {
        return Ok(DescribeStateMachineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStateMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_state_machine_alias_request(
    body: &[u8],
) -> Result<DescribeStateMachineAliasInput, String> {
    if body.is_empty() {
        return Ok(DescribeStateMachineAliasInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStateMachineAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_state_machine_for_execution_request(
    body: &[u8],
) -> Result<DescribeStateMachineForExecutionInput, String> {
    if body.is_empty() {
        return Ok(DescribeStateMachineForExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStateMachineForExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_activity_task_request(body: &[u8]) -> Result<GetActivityTaskInput, String> {
    if body.is_empty() {
        return Ok(GetActivityTaskInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetActivityTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_execution_history_request(
    body: &[u8],
) -> Result<GetExecutionHistoryInput, String> {
    if body.is_empty() {
        return Ok(GetExecutionHistoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetExecutionHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_activities_request(body: &[u8]) -> Result<ListActivitiesInput, String> {
    if body.is_empty() {
        return Ok(ListActivitiesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListActivities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_executions_request(body: &[u8]) -> Result<ListExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_map_runs_request(body: &[u8]) -> Result<ListMapRunsInput, String> {
    if body.is_empty() {
        return Ok(ListMapRunsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMapRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_state_machine_aliases_request(
    body: &[u8],
) -> Result<ListStateMachineAliasesInput, String> {
    if body.is_empty() {
        return Ok(ListStateMachineAliasesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStateMachineAliases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_state_machine_versions_request(
    body: &[u8],
) -> Result<ListStateMachineVersionsInput, String> {
    if body.is_empty() {
        return Ok(ListStateMachineVersionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStateMachineVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_state_machines_request(
    body: &[u8],
) -> Result<ListStateMachinesInput, String> {
    if body.is_empty() {
        return Ok(ListStateMachinesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStateMachines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_publish_state_machine_version_request(
    body: &[u8],
) -> Result<PublishStateMachineVersionInput, String> {
    if body.is_empty() {
        return Ok(PublishStateMachineVersionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PublishStateMachineVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_redrive_execution_request(body: &[u8]) -> Result<RedriveExecutionInput, String> {
    if body.is_empty() {
        return Ok(RedriveExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RedriveExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_task_failure_request(body: &[u8]) -> Result<SendTaskFailureInput, String> {
    if body.is_empty() {
        return Ok(SendTaskFailureInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendTaskFailure request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_task_heartbeat_request(
    body: &[u8],
) -> Result<SendTaskHeartbeatInput, String> {
    if body.is_empty() {
        return Ok(SendTaskHeartbeatInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendTaskHeartbeat request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_task_success_request(body: &[u8]) -> Result<SendTaskSuccessInput, String> {
    if body.is_empty() {
        return Ok(SendTaskSuccessInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendTaskSuccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_execution_request(body: &[u8]) -> Result<StartExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_sync_execution_request(
    body: &[u8],
) -> Result<StartSyncExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartSyncExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSyncExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_execution_request(body: &[u8]) -> Result<StopExecutionInput, String> {
    if body.is_empty() {
        return Ok(StopExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_state_request(body: &[u8]) -> Result<TestStateInput, String> {
    if body.is_empty() {
        return Ok(TestStateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_map_run_request(body: &[u8]) -> Result<UpdateMapRunInput, String> {
    if body.is_empty() {
        return Ok(UpdateMapRunInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMapRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_state_machine_request(
    body: &[u8],
) -> Result<UpdateStateMachineInput, String> {
    if body.is_empty() {
        return Ok(UpdateStateMachineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStateMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_state_machine_alias_request(
    body: &[u8],
) -> Result<UpdateStateMachineAliasInput, String> {
    if body.is_empty() {
        return Ok(UpdateStateMachineAliasInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStateMachineAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_validate_state_machine_definition_request(
    body: &[u8],
) -> Result<ValidateStateMachineDefinitionInput, String> {
    if body.is_empty() {
        return Ok(ValidateStateMachineDefinitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ValidateStateMachineDefinition request: {e}"))
}
