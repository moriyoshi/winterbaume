//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codepipeline

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_acknowledge_job_response(result: &AcknowledgeJobOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_acknowledge_third_party_job_response(
    result: &AcknowledgeThirdPartyJobOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_custom_action_type_response(
    result: &CreateCustomActionTypeOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_pipeline_response(result: &CreatePipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_custom_action_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_pipeline_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_webhook_response(result: &DeleteWebhookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_webhook_with_third_party_response(
    result: &DeregisterWebhookWithThirdPartyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_stage_transition_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_stage_transition_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_action_type_response(result: &GetActionTypeOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_job_details_response(result: &GetJobDetailsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pipeline_response(result: &GetPipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pipeline_execution_response(
    result: &GetPipelineExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_pipeline_state_response(result: &GetPipelineStateOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_third_party_job_details_response(
    result: &GetThirdPartyJobDetailsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_action_executions_response(
    result: &ListActionExecutionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_action_types_response(result: &ListActionTypesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deploy_action_execution_targets_response(
    result: &ListDeployActionExecutionTargetsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_pipeline_executions_response(
    result: &ListPipelineExecutionsOutput,
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
pub fn serialize_list_rule_executions_response(result: &ListRuleExecutionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_rule_types_response(result: &ListRuleTypesOutput) -> MockResponse {
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
pub fn serialize_list_webhooks_response(result: &ListWebhooksOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_override_stage_condition_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_poll_for_jobs_response(result: &PollForJobsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_poll_for_third_party_jobs_response(
    result: &PollForThirdPartyJobsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_action_revision_response(result: &PutActionRevisionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_approval_result_response(result: &PutApprovalResultOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_job_failure_result_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_job_success_result_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_third_party_job_failure_result_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_third_party_job_success_result_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_webhook_response(result: &PutWebhookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_webhook_with_third_party_response(
    result: &RegisterWebhookWithThirdPartyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_retry_stage_execution_response(
    result: &RetryStageExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_rollback_stage_response(result: &RollbackStageOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_pipeline_execution_response(
    result: &StartPipelineExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_pipeline_execution_response(
    result: &StopPipelineExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_action_type_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pipeline_response(result: &UpdatePipelineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_acknowledge_job_request(body: &[u8]) -> Result<AcknowledgeJobInput, String> {
    if body.is_empty() {
        return Ok(AcknowledgeJobInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcknowledgeJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_acknowledge_third_party_job_request(
    body: &[u8],
) -> Result<AcknowledgeThirdPartyJobInput, String> {
    if body.is_empty() {
        return Ok(AcknowledgeThirdPartyJobInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcknowledgeThirdPartyJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_custom_action_type_request(
    body: &[u8],
) -> Result<CreateCustomActionTypeInput, String> {
    if body.is_empty() {
        return Ok(CreateCustomActionTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCustomActionType request: {e}"))
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
pub fn deserialize_delete_custom_action_type_request(
    body: &[u8],
) -> Result<DeleteCustomActionTypeInput, String> {
    if body.is_empty() {
        return Ok(DeleteCustomActionTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCustomActionType request: {e}"))
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
pub fn deserialize_delete_webhook_request(body: &[u8]) -> Result<DeleteWebhookInput, String> {
    if body.is_empty() {
        return Ok(DeleteWebhookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebhook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_webhook_with_third_party_request(
    body: &[u8],
) -> Result<DeregisterWebhookWithThirdPartyInput, String> {
    if body.is_empty() {
        return Ok(DeregisterWebhookWithThirdPartyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterWebhookWithThirdParty request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_stage_transition_request(
    body: &[u8],
) -> Result<DisableStageTransitionInput, String> {
    if body.is_empty() {
        return Ok(DisableStageTransitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableStageTransition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_stage_transition_request(
    body: &[u8],
) -> Result<EnableStageTransitionInput, String> {
    if body.is_empty() {
        return Ok(EnableStageTransitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableStageTransition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_action_type_request(body: &[u8]) -> Result<GetActionTypeInput, String> {
    if body.is_empty() {
        return Ok(GetActionTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetActionType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_job_details_request(body: &[u8]) -> Result<GetJobDetailsInput, String> {
    if body.is_empty() {
        return Ok(GetJobDetailsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetJobDetails request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pipeline_request(body: &[u8]) -> Result<GetPipelineInput, String> {
    if body.is_empty() {
        return Ok(GetPipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPipeline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pipeline_execution_request(
    body: &[u8],
) -> Result<GetPipelineExecutionInput, String> {
    if body.is_empty() {
        return Ok(GetPipelineExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPipelineExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_pipeline_state_request(
    body: &[u8],
) -> Result<GetPipelineStateInput, String> {
    if body.is_empty() {
        return Ok(GetPipelineStateInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPipelineState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_third_party_job_details_request(
    body: &[u8],
) -> Result<GetThirdPartyJobDetailsInput, String> {
    if body.is_empty() {
        return Ok(GetThirdPartyJobDetailsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetThirdPartyJobDetails request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_action_executions_request(
    body: &[u8],
) -> Result<ListActionExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListActionExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListActionExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_action_types_request(body: &[u8]) -> Result<ListActionTypesInput, String> {
    if body.is_empty() {
        return Ok(ListActionTypesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListActionTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deploy_action_execution_targets_request(
    body: &[u8],
) -> Result<ListDeployActionExecutionTargetsInput, String> {
    if body.is_empty() {
        return Ok(ListDeployActionExecutionTargetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeployActionExecutionTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_pipeline_executions_request(
    body: &[u8],
) -> Result<ListPipelineExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListPipelineExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPipelineExecutions request: {e}"))
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
pub fn deserialize_list_rule_executions_request(
    body: &[u8],
) -> Result<ListRuleExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListRuleExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRuleExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_rule_types_request(body: &[u8]) -> Result<ListRuleTypesInput, String> {
    if body.is_empty() {
        return Ok(ListRuleTypesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRuleTypes request: {e}"))
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
pub fn deserialize_list_webhooks_request(body: &[u8]) -> Result<ListWebhooksInput, String> {
    if body.is_empty() {
        return Ok(ListWebhooksInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWebhooks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_override_stage_condition_request(
    body: &[u8],
) -> Result<OverrideStageConditionInput, String> {
    if body.is_empty() {
        return Ok(OverrideStageConditionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize OverrideStageCondition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_poll_for_jobs_request(body: &[u8]) -> Result<PollForJobsInput, String> {
    if body.is_empty() {
        return Ok(PollForJobsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PollForJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_poll_for_third_party_jobs_request(
    body: &[u8],
) -> Result<PollForThirdPartyJobsInput, String> {
    if body.is_empty() {
        return Ok(PollForThirdPartyJobsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PollForThirdPartyJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_action_revision_request(
    body: &[u8],
) -> Result<PutActionRevisionInput, String> {
    if body.is_empty() {
        return Ok(PutActionRevisionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutActionRevision request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_approval_result_request(
    body: &[u8],
) -> Result<PutApprovalResultInput, String> {
    if body.is_empty() {
        return Ok(PutApprovalResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutApprovalResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_job_failure_result_request(
    body: &[u8],
) -> Result<PutJobFailureResultInput, String> {
    if body.is_empty() {
        return Ok(PutJobFailureResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutJobFailureResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_job_success_result_request(
    body: &[u8],
) -> Result<PutJobSuccessResultInput, String> {
    if body.is_empty() {
        return Ok(PutJobSuccessResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutJobSuccessResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_third_party_job_failure_result_request(
    body: &[u8],
) -> Result<PutThirdPartyJobFailureResultInput, String> {
    if body.is_empty() {
        return Ok(PutThirdPartyJobFailureResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutThirdPartyJobFailureResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_third_party_job_success_result_request(
    body: &[u8],
) -> Result<PutThirdPartyJobSuccessResultInput, String> {
    if body.is_empty() {
        return Ok(PutThirdPartyJobSuccessResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutThirdPartyJobSuccessResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_webhook_request(body: &[u8]) -> Result<PutWebhookInput, String> {
    if body.is_empty() {
        return Ok(PutWebhookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutWebhook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_webhook_with_third_party_request(
    body: &[u8],
) -> Result<RegisterWebhookWithThirdPartyInput, String> {
    if body.is_empty() {
        return Ok(RegisterWebhookWithThirdPartyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterWebhookWithThirdParty request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_retry_stage_execution_request(
    body: &[u8],
) -> Result<RetryStageExecutionInput, String> {
    if body.is_empty() {
        return Ok(RetryStageExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RetryStageExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_rollback_stage_request(body: &[u8]) -> Result<RollbackStageInput, String> {
    if body.is_empty() {
        return Ok(RollbackStageInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RollbackStage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_pipeline_execution_request(
    body: &[u8],
) -> Result<StartPipelineExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartPipelineExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartPipelineExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_pipeline_execution_request(
    body: &[u8],
) -> Result<StopPipelineExecutionInput, String> {
    if body.is_empty() {
        return Ok(StopPipelineExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopPipelineExecution request: {e}"))
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
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_action_type_request(
    body: &[u8],
) -> Result<UpdateActionTypeInput, String> {
    if body.is_empty() {
        return Ok(UpdateActionTypeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateActionType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pipeline_request(body: &[u8]) -> Result<UpdatePipelineInput, String> {
    if body.is_empty() {
        return Ok(UpdatePipelineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePipeline request: {e}"))
}
