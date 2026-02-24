//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-swf

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_count_closed_workflow_executions_response(
    result: &WorkflowExecutionCount,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_count_open_workflow_executions_response(
    result: &WorkflowExecutionCount,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_count_pending_activity_tasks_response(result: &PendingTaskCount) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_count_pending_decision_tasks_response(result: &PendingTaskCount) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_activity_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_workflow_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deprecate_activity_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deprecate_domain_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deprecate_workflow_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_activity_type_response(result: &ActivityTypeDetail) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_domain_response(result: &DomainDetail) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workflow_execution_response(
    result: &WorkflowExecutionDetail,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workflow_type_response(result: &WorkflowTypeDetail) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_workflow_execution_history_response(result: &History) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_activity_types_response(result: &ActivityTypeInfos) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_closed_workflow_executions_response(
    result: &WorkflowExecutionInfos,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_domains_response(result: &DomainInfos) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_open_workflow_executions_response(
    result: &WorkflowExecutionInfos,
) -> MockResponse {
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
pub fn serialize_list_workflow_types_response(result: &WorkflowTypeInfos) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_poll_for_activity_task_response(result: &ActivityTask) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_poll_for_decision_task_response(result: &DecisionTask) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_record_activity_task_heartbeat_response(
    result: &ActivityTaskStatus,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_activity_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_domain_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_workflow_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_request_cancel_workflow_execution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_respond_activity_task_canceled_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_respond_activity_task_completed_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_respond_activity_task_failed_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_respond_decision_task_completed_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_signal_workflow_execution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_workflow_execution_response(result: &Run) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_terminate_workflow_execution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_undeprecate_activity_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_undeprecate_domain_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_undeprecate_workflow_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_count_closed_workflow_executions_request(
    body: &[u8],
) -> Result<CountClosedWorkflowExecutionsInput, String> {
    if body.is_empty() {
        return Ok(CountClosedWorkflowExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CountClosedWorkflowExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_count_open_workflow_executions_request(
    body: &[u8],
) -> Result<CountOpenWorkflowExecutionsInput, String> {
    if body.is_empty() {
        return Ok(CountOpenWorkflowExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CountOpenWorkflowExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_count_pending_activity_tasks_request(
    body: &[u8],
) -> Result<CountPendingActivityTasksInput, String> {
    if body.is_empty() {
        return Ok(CountPendingActivityTasksInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CountPendingActivityTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_count_pending_decision_tasks_request(
    body: &[u8],
) -> Result<CountPendingDecisionTasksInput, String> {
    if body.is_empty() {
        return Ok(CountPendingDecisionTasksInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CountPendingDecisionTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_activity_type_request(
    body: &[u8],
) -> Result<DeleteActivityTypeInput, String> {
    if body.is_empty() {
        return Ok(DeleteActivityTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteActivityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_workflow_type_request(
    body: &[u8],
) -> Result<DeleteWorkflowTypeInput, String> {
    if body.is_empty() {
        return Ok(DeleteWorkflowTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkflowType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deprecate_activity_type_request(
    body: &[u8],
) -> Result<DeprecateActivityTypeInput, String> {
    if body.is_empty() {
        return Ok(DeprecateActivityTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeprecateActivityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deprecate_domain_request(body: &[u8]) -> Result<DeprecateDomainInput, String> {
    if body.is_empty() {
        return Ok(DeprecateDomainInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeprecateDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deprecate_workflow_type_request(
    body: &[u8],
) -> Result<DeprecateWorkflowTypeInput, String> {
    if body.is_empty() {
        return Ok(DeprecateWorkflowTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeprecateWorkflowType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_activity_type_request(
    body: &[u8],
) -> Result<DescribeActivityTypeInput, String> {
    if body.is_empty() {
        return Ok(DescribeActivityTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeActivityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_domain_request(body: &[u8]) -> Result<DescribeDomainInput, String> {
    if body.is_empty() {
        return Ok(DescribeDomainInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workflow_execution_request(
    body: &[u8],
) -> Result<DescribeWorkflowExecutionInput, String> {
    if body.is_empty() {
        return Ok(DescribeWorkflowExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkflowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workflow_type_request(
    body: &[u8],
) -> Result<DescribeWorkflowTypeInput, String> {
    if body.is_empty() {
        return Ok(DescribeWorkflowTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkflowType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_workflow_execution_history_request(
    body: &[u8],
) -> Result<GetWorkflowExecutionHistoryInput, String> {
    if body.is_empty() {
        return Ok(GetWorkflowExecutionHistoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkflowExecutionHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_activity_types_request(
    body: &[u8],
) -> Result<ListActivityTypesInput, String> {
    if body.is_empty() {
        return Ok(ListActivityTypesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListActivityTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_closed_workflow_executions_request(
    body: &[u8],
) -> Result<ListClosedWorkflowExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListClosedWorkflowExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListClosedWorkflowExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_domains_request(body: &[u8]) -> Result<ListDomainsInput, String> {
    if body.is_empty() {
        return Ok(ListDomainsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDomains request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_open_workflow_executions_request(
    body: &[u8],
) -> Result<ListOpenWorkflowExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListOpenWorkflowExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOpenWorkflowExecutions request: {e}"))
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
pub fn deserialize_list_workflow_types_request(
    body: &[u8],
) -> Result<ListWorkflowTypesInput, String> {
    if body.is_empty() {
        return Ok(ListWorkflowTypesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWorkflowTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_poll_for_activity_task_request(
    body: &[u8],
) -> Result<PollForActivityTaskInput, String> {
    if body.is_empty() {
        return Ok(PollForActivityTaskInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PollForActivityTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_poll_for_decision_task_request(
    body: &[u8],
) -> Result<PollForDecisionTaskInput, String> {
    if body.is_empty() {
        return Ok(PollForDecisionTaskInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PollForDecisionTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_record_activity_task_heartbeat_request(
    body: &[u8],
) -> Result<RecordActivityTaskHeartbeatInput, String> {
    if body.is_empty() {
        return Ok(RecordActivityTaskHeartbeatInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RecordActivityTaskHeartbeat request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_activity_type_request(
    body: &[u8],
) -> Result<RegisterActivityTypeInput, String> {
    if body.is_empty() {
        return Ok(RegisterActivityTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterActivityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_domain_request(body: &[u8]) -> Result<RegisterDomainInput, String> {
    if body.is_empty() {
        return Ok(RegisterDomainInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_workflow_type_request(
    body: &[u8],
) -> Result<RegisterWorkflowTypeInput, String> {
    if body.is_empty() {
        return Ok(RegisterWorkflowTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterWorkflowType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_request_cancel_workflow_execution_request(
    body: &[u8],
) -> Result<RequestCancelWorkflowExecutionInput, String> {
    if body.is_empty() {
        return Ok(RequestCancelWorkflowExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RequestCancelWorkflowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_respond_activity_task_canceled_request(
    body: &[u8],
) -> Result<RespondActivityTaskCanceledInput, String> {
    if body.is_empty() {
        return Ok(RespondActivityTaskCanceledInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RespondActivityTaskCanceled request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_respond_activity_task_completed_request(
    body: &[u8],
) -> Result<RespondActivityTaskCompletedInput, String> {
    if body.is_empty() {
        return Ok(RespondActivityTaskCompletedInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RespondActivityTaskCompleted request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_respond_activity_task_failed_request(
    body: &[u8],
) -> Result<RespondActivityTaskFailedInput, String> {
    if body.is_empty() {
        return Ok(RespondActivityTaskFailedInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RespondActivityTaskFailed request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_respond_decision_task_completed_request(
    body: &[u8],
) -> Result<RespondDecisionTaskCompletedInput, String> {
    if body.is_empty() {
        return Ok(RespondDecisionTaskCompletedInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RespondDecisionTaskCompleted request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_signal_workflow_execution_request(
    body: &[u8],
) -> Result<SignalWorkflowExecutionInput, String> {
    if body.is_empty() {
        return Ok(SignalWorkflowExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SignalWorkflowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_workflow_execution_request(
    body: &[u8],
) -> Result<StartWorkflowExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartWorkflowExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartWorkflowExecution request: {e}"))
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
pub fn deserialize_terminate_workflow_execution_request(
    body: &[u8],
) -> Result<TerminateWorkflowExecutionInput, String> {
    if body.is_empty() {
        return Ok(TerminateWorkflowExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateWorkflowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_undeprecate_activity_type_request(
    body: &[u8],
) -> Result<UndeprecateActivityTypeInput, String> {
    if body.is_empty() {
        return Ok(UndeprecateActivityTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UndeprecateActivityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_undeprecate_domain_request(
    body: &[u8],
) -> Result<UndeprecateDomainInput, String> {
    if body.is_empty() {
        return Ok(UndeprecateDomainInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UndeprecateDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_undeprecate_workflow_type_request(
    body: &[u8],
) -> Result<UndeprecateWorkflowTypeInput, String> {
    if body.is_empty() {
        return Ok(UndeprecateWorkflowTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UndeprecateWorkflowType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}
