//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codedeploy

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_add_tags_to_on_premises_instances_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_application_revisions_response(
    result: &BatchGetApplicationRevisionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_applications_response(
    result: &BatchGetApplicationsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_deployment_groups_response(
    result: &BatchGetDeploymentGroupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_deployment_instances_response(
    result: &BatchGetDeploymentInstancesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_deployment_targets_response(
    result: &BatchGetDeploymentTargetsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_deployments_response(
    result: &BatchGetDeploymentsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_on_premises_instances_response(
    result: &BatchGetOnPremisesInstancesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_continue_deployment_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_deployment_response(result: &CreateDeploymentOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_deployment_config_response(
    result: &CreateDeploymentConfigOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_deployment_group_response(
    result: &CreateDeploymentGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_application_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_deployment_config_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_deployment_group_response(
    result: &DeleteDeploymentGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_git_hub_account_token_response(
    result: &DeleteGitHubAccountTokenOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resources_by_external_id_response(
    result: &DeleteResourcesByExternalIdOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deregister_on_premises_instance_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_response(result: &GetApplicationOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_application_revision_response(
    result: &GetApplicationRevisionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployment_response(result: &GetDeploymentOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployment_config_response(
    result: &GetDeploymentConfigOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployment_group_response(result: &GetDeploymentGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployment_instance_response(
    result: &GetDeploymentInstanceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployment_target_response(
    result: &GetDeploymentTargetOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_on_premises_instance_response(
    result: &GetOnPremisesInstanceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_revisions_response(
    result: &ListApplicationRevisionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deployment_configs_response(
    result: &ListDeploymentConfigsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deployment_groups_response(
    result: &ListDeploymentGroupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deployment_instances_response(
    result: &ListDeploymentInstancesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deployment_targets_response(
    result: &ListDeploymentTargetsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_deployments_response(result: &ListDeploymentsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_git_hub_account_token_names_response(
    result: &ListGitHubAccountTokenNamesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_on_premises_instances_response(
    result: &ListOnPremisesInstancesOutput,
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
pub fn serialize_put_lifecycle_event_hook_execution_status_response(
    result: &PutLifecycleEventHookExecutionStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_application_revision_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_register_on_premises_instance_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_tags_from_on_premises_instances_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_skip_wait_time_for_instance_termination_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_deployment_response(result: &StopDeploymentOutput) -> MockResponse {
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
pub fn serialize_update_application_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_deployment_group_response(
    result: &UpdateDeploymentGroupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_to_on_premises_instances_request(
    body: &[u8],
) -> Result<AddTagsToOnPremisesInstancesInput, String> {
    if body.is_empty() {
        return Ok(AddTagsToOnPremisesInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToOnPremisesInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_application_revisions_request(
    body: &[u8],
) -> Result<BatchGetApplicationRevisionsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetApplicationRevisionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetApplicationRevisions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_applications_request(
    body: &[u8],
) -> Result<BatchGetApplicationsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetApplicationsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_deployment_groups_request(
    body: &[u8],
) -> Result<BatchGetDeploymentGroupsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetDeploymentGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDeploymentGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_deployment_instances_request(
    body: &[u8],
) -> Result<BatchGetDeploymentInstancesInput, String> {
    if body.is_empty() {
        return Ok(BatchGetDeploymentInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDeploymentInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_deployment_targets_request(
    body: &[u8],
) -> Result<BatchGetDeploymentTargetsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetDeploymentTargetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDeploymentTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_deployments_request(
    body: &[u8],
) -> Result<BatchGetDeploymentsInput, String> {
    if body.is_empty() {
        return Ok(BatchGetDeploymentsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_on_premises_instances_request(
    body: &[u8],
) -> Result<BatchGetOnPremisesInstancesInput, String> {
    if body.is_empty() {
        return Ok(BatchGetOnPremisesInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetOnPremisesInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_continue_deployment_request(
    body: &[u8],
) -> Result<ContinueDeploymentInput, String> {
    if body.is_empty() {
        return Ok(ContinueDeploymentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ContinueDeployment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_request(
    body: &[u8],
) -> Result<CreateApplicationInput, String> {
    if body.is_empty() {
        return Ok(CreateApplicationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_deployment_request(body: &[u8]) -> Result<CreateDeploymentInput, String> {
    if body.is_empty() {
        return Ok(CreateDeploymentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDeployment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_deployment_config_request(
    body: &[u8],
) -> Result<CreateDeploymentConfigInput, String> {
    if body.is_empty() {
        return Ok(CreateDeploymentConfigInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDeploymentConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_deployment_group_request(
    body: &[u8],
) -> Result<CreateDeploymentGroupInput, String> {
    if body.is_empty() {
        return Ok(CreateDeploymentGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDeploymentGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_request(
    body: &[u8],
) -> Result<DeleteApplicationInput, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_deployment_config_request(
    body: &[u8],
) -> Result<DeleteDeploymentConfigInput, String> {
    if body.is_empty() {
        return Ok(DeleteDeploymentConfigInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeploymentConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_deployment_group_request(
    body: &[u8],
) -> Result<DeleteDeploymentGroupInput, String> {
    if body.is_empty() {
        return Ok(DeleteDeploymentGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeploymentGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_git_hub_account_token_request(
    body: &[u8],
) -> Result<DeleteGitHubAccountTokenInput, String> {
    if body.is_empty() {
        return Ok(DeleteGitHubAccountTokenInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteGitHubAccountToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resources_by_external_id_request(
    body: &[u8],
) -> Result<DeleteResourcesByExternalIdInput, String> {
    if body.is_empty() {
        return Ok(DeleteResourcesByExternalIdInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcesByExternalId request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_on_premises_instance_request(
    body: &[u8],
) -> Result<DeregisterOnPremisesInstanceInput, String> {
    if body.is_empty() {
        return Ok(DeregisterOnPremisesInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterOnPremisesInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_request(body: &[u8]) -> Result<GetApplicationInput, String> {
    if body.is_empty() {
        return Ok(GetApplicationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_application_revision_request(
    body: &[u8],
) -> Result<GetApplicationRevisionInput, String> {
    if body.is_empty() {
        return Ok(GetApplicationRevisionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApplicationRevision request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployment_request(body: &[u8]) -> Result<GetDeploymentInput, String> {
    if body.is_empty() {
        return Ok(GetDeploymentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeployment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployment_config_request(
    body: &[u8],
) -> Result<GetDeploymentConfigInput, String> {
    if body.is_empty() {
        return Ok(GetDeploymentConfigInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeploymentConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployment_group_request(
    body: &[u8],
) -> Result<GetDeploymentGroupInput, String> {
    if body.is_empty() {
        return Ok(GetDeploymentGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeploymentGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployment_instance_request(
    body: &[u8],
) -> Result<GetDeploymentInstanceInput, String> {
    if body.is_empty() {
        return Ok(GetDeploymentInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeploymentInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployment_target_request(
    body: &[u8],
) -> Result<GetDeploymentTargetInput, String> {
    if body.is_empty() {
        return Ok(GetDeploymentTargetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeploymentTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_on_premises_instance_request(
    body: &[u8],
) -> Result<GetOnPremisesInstanceInput, String> {
    if body.is_empty() {
        return Ok(GetOnPremisesInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOnPremisesInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_revisions_request(
    body: &[u8],
) -> Result<ListApplicationRevisionsInput, String> {
    if body.is_empty() {
        return Ok(ListApplicationRevisionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationRevisions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_applications_request(body: &[u8]) -> Result<ListApplicationsInput, String> {
    if body.is_empty() {
        return Ok(ListApplicationsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deployment_configs_request(
    body: &[u8],
) -> Result<ListDeploymentConfigsInput, String> {
    if body.is_empty() {
        return Ok(ListDeploymentConfigsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeploymentConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deployment_groups_request(
    body: &[u8],
) -> Result<ListDeploymentGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListDeploymentGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeploymentGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deployment_instances_request(
    body: &[u8],
) -> Result<ListDeploymentInstancesInput, String> {
    if body.is_empty() {
        return Ok(ListDeploymentInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeploymentInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deployment_targets_request(
    body: &[u8],
) -> Result<ListDeploymentTargetsInput, String> {
    if body.is_empty() {
        return Ok(ListDeploymentTargetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeploymentTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_deployments_request(body: &[u8]) -> Result<ListDeploymentsInput, String> {
    if body.is_empty() {
        return Ok(ListDeploymentsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_git_hub_account_token_names_request(
    body: &[u8],
) -> Result<ListGitHubAccountTokenNamesInput, String> {
    if body.is_empty() {
        return Ok(ListGitHubAccountTokenNamesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGitHubAccountTokenNames request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_on_premises_instances_request(
    body: &[u8],
) -> Result<ListOnPremisesInstancesInput, String> {
    if body.is_empty() {
        return Ok(ListOnPremisesInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOnPremisesInstances request: {e}"))
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
pub fn deserialize_put_lifecycle_event_hook_execution_status_request(
    body: &[u8],
) -> Result<PutLifecycleEventHookExecutionStatusInput, String> {
    if body.is_empty() {
        return Ok(PutLifecycleEventHookExecutionStatusInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutLifecycleEventHookExecutionStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_application_revision_request(
    body: &[u8],
) -> Result<RegisterApplicationRevisionInput, String> {
    if body.is_empty() {
        return Ok(RegisterApplicationRevisionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterApplicationRevision request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_on_premises_instance_request(
    body: &[u8],
) -> Result<RegisterOnPremisesInstanceInput, String> {
    if body.is_empty() {
        return Ok(RegisterOnPremisesInstanceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterOnPremisesInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_from_on_premises_instances_request(
    body: &[u8],
) -> Result<RemoveTagsFromOnPremisesInstancesInput, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromOnPremisesInstancesInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RemoveTagsFromOnPremisesInstances request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_skip_wait_time_for_instance_termination_request(
    body: &[u8],
) -> Result<SkipWaitTimeForInstanceTerminationInput, String> {
    if body.is_empty() {
        return Ok(SkipWaitTimeForInstanceTerminationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize SkipWaitTimeForInstanceTermination request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_deployment_request(body: &[u8]) -> Result<StopDeploymentInput, String> {
    if body.is_empty() {
        return Ok(StopDeploymentInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopDeployment request: {e}"))
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
pub fn deserialize_update_application_request(
    body: &[u8],
) -> Result<UpdateApplicationInput, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_deployment_group_request(
    body: &[u8],
) -> Result<UpdateDeploymentGroupInput, String> {
    if body.is_empty() {
        return Ok(UpdateDeploymentGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDeploymentGroup request: {e}"))
}
