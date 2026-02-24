//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emr

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_instance_fleet_response(result: &AddInstanceFleetOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_instance_groups_response(result: &AddInstanceGroupsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_job_flow_steps_response(result: &AddJobFlowStepsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_response(result: &AddTagsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_steps_response(result: &CancelStepsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_persistent_app_u_i_response(
    result: &CreatePersistentAppUIOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_security_configuration_response(
    result: &CreateSecurityConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_studio_response(result: &CreateStudioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_studio_session_mapping_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_security_configuration_response(
    result: &DeleteSecurityConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_studio_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_studio_session_mapping_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_cluster_response(result: &DescribeClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_job_flows_response(result: &DescribeJobFlowsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_notebook_execution_response(
    result: &DescribeNotebookExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_persistent_app_u_i_response(
    result: &DescribePersistentAppUIOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_release_label_response(
    result: &DescribeReleaseLabelOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_security_configuration_response(
    result: &DescribeSecurityConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_step_response(result: &DescribeStepOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_studio_response(result: &DescribeStudioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_auto_termination_policy_response(
    result: &GetAutoTerminationPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_block_public_access_configuration_response(
    result: &GetBlockPublicAccessConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cluster_session_credentials_response(
    result: &GetClusterSessionCredentialsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_managed_scaling_policy_response(
    result: &GetManagedScalingPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_on_cluster_app_u_i_presigned_u_r_l_response(
    result: &GetOnClusterAppUIPresignedURLOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_persistent_app_u_i_presigned_u_r_l_response(
    result: &GetPersistentAppUIPresignedURLOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_studio_session_mapping_response(
    result: &GetStudioSessionMappingOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_bootstrap_actions_response(
    result: &ListBootstrapActionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_clusters_response(result: &ListClustersOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_instance_fleets_response(result: &ListInstanceFleetsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_instance_groups_response(result: &ListInstanceGroupsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_instances_response(result: &ListInstancesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_notebook_executions_response(
    result: &ListNotebookExecutionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_release_labels_response(result: &ListReleaseLabelsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_security_configurations_response(
    result: &ListSecurityConfigurationsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_steps_response(result: &ListStepsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_studio_session_mappings_response(
    result: &ListStudioSessionMappingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_studios_response(result: &ListStudiosOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_supported_instance_types_response(
    result: &ListSupportedInstanceTypesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_cluster_response(result: &ModifyClusterOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_modify_instance_fleet_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_modify_instance_groups_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_auto_scaling_policy_response(
    result: &PutAutoScalingPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_auto_termination_policy_response(
    result: &PutAutoTerminationPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_block_public_access_configuration_response(
    result: &PutBlockPublicAccessConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_managed_scaling_policy_response(
    result: &PutManagedScalingPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_auto_scaling_policy_response(
    result: &RemoveAutoScalingPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_auto_termination_policy_response(
    result: &RemoveAutoTerminationPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_managed_scaling_policy_response(
    result: &RemoveManagedScalingPolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_response(result: &RemoveTagsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_run_job_flow_response(result: &RunJobFlowOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_keep_job_flow_alive_when_no_steps_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_termination_protection_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_unhealthy_node_replacement_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_visible_to_all_users_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_notebook_execution_response(
    result: &StartNotebookExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_notebook_execution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_terminate_job_flows_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_studio_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_studio_session_mapping_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_instance_fleet_request(
    body: &[u8],
) -> Result<AddInstanceFleetInput, String> {
    if body.is_empty() {
        return Ok(AddInstanceFleetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddInstanceFleet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_instance_groups_request(
    body: &[u8],
) -> Result<AddInstanceGroupsInput, String> {
    if body.is_empty() {
        return Ok(AddInstanceGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddInstanceGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_job_flow_steps_request(body: &[u8]) -> Result<AddJobFlowStepsInput, String> {
    if body.is_empty() {
        return Ok(AddJobFlowStepsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddJobFlowSteps request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_request(body: &[u8]) -> Result<AddTagsInput, String> {
    if body.is_empty() {
        return Ok(AddTagsInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize AddTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_steps_request(body: &[u8]) -> Result<CancelStepsInput, String> {
    if body.is_empty() {
        return Ok(CancelStepsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelSteps request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_persistent_app_u_i_request(
    body: &[u8],
) -> Result<CreatePersistentAppUIInput, String> {
    if body.is_empty() {
        return Ok(CreatePersistentAppUIInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePersistentAppUI request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_security_configuration_request(
    body: &[u8],
) -> Result<CreateSecurityConfigurationInput, String> {
    if body.is_empty() {
        return Ok(CreateSecurityConfigurationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_studio_request(body: &[u8]) -> Result<CreateStudioInput, String> {
    if body.is_empty() {
        return Ok(CreateStudioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStudio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_studio_session_mapping_request(
    body: &[u8],
) -> Result<CreateStudioSessionMappingInput, String> {
    if body.is_empty() {
        return Ok(CreateStudioSessionMappingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStudioSessionMapping request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_security_configuration_request(
    body: &[u8],
) -> Result<DeleteSecurityConfigurationInput, String> {
    if body.is_empty() {
        return Ok(DeleteSecurityConfigurationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_studio_request(body: &[u8]) -> Result<DeleteStudioInput, String> {
    if body.is_empty() {
        return Ok(DeleteStudioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStudio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_studio_session_mapping_request(
    body: &[u8],
) -> Result<DeleteStudioSessionMappingInput, String> {
    if body.is_empty() {
        return Ok(DeleteStudioSessionMappingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStudioSessionMapping request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_cluster_request(body: &[u8]) -> Result<DescribeClusterInput, String> {
    if body.is_empty() {
        return Ok(DescribeClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_job_flows_request(
    body: &[u8],
) -> Result<DescribeJobFlowsInput, String> {
    if body.is_empty() {
        return Ok(DescribeJobFlowsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeJobFlows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_notebook_execution_request(
    body: &[u8],
) -> Result<DescribeNotebookExecutionInput, String> {
    if body.is_empty() {
        return Ok(DescribeNotebookExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeNotebookExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_persistent_app_u_i_request(
    body: &[u8],
) -> Result<DescribePersistentAppUIInput, String> {
    if body.is_empty() {
        return Ok(DescribePersistentAppUIInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePersistentAppUI request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_release_label_request(
    body: &[u8],
) -> Result<DescribeReleaseLabelInput, String> {
    if body.is_empty() {
        return Ok(DescribeReleaseLabelInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReleaseLabel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_security_configuration_request(
    body: &[u8],
) -> Result<DescribeSecurityConfigurationInput, String> {
    if body.is_empty() {
        return Ok(DescribeSecurityConfigurationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_step_request(body: &[u8]) -> Result<DescribeStepInput, String> {
    if body.is_empty() {
        return Ok(DescribeStepInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStep request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_studio_request(body: &[u8]) -> Result<DescribeStudioInput, String> {
    if body.is_empty() {
        return Ok(DescribeStudioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStudio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_auto_termination_policy_request(
    body: &[u8],
) -> Result<GetAutoTerminationPolicyInput, String> {
    if body.is_empty() {
        return Ok(GetAutoTerminationPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAutoTerminationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_block_public_access_configuration_request(
    body: &[u8],
) -> Result<GetBlockPublicAccessConfigurationInput, String> {
    if body.is_empty() {
        return Ok(GetBlockPublicAccessConfigurationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetBlockPublicAccessConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cluster_session_credentials_request(
    body: &[u8],
) -> Result<GetClusterSessionCredentialsInput, String> {
    if body.is_empty() {
        return Ok(GetClusterSessionCredentialsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetClusterSessionCredentials request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_managed_scaling_policy_request(
    body: &[u8],
) -> Result<GetManagedScalingPolicyInput, String> {
    if body.is_empty() {
        return Ok(GetManagedScalingPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetManagedScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_on_cluster_app_u_i_presigned_u_r_l_request(
    body: &[u8],
) -> Result<GetOnClusterAppUIPresignedURLInput, String> {
    if body.is_empty() {
        return Ok(GetOnClusterAppUIPresignedURLInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOnClusterAppUIPresignedURL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_persistent_app_u_i_presigned_u_r_l_request(
    body: &[u8],
) -> Result<GetPersistentAppUIPresignedURLInput, String> {
    if body.is_empty() {
        return Ok(GetPersistentAppUIPresignedURLInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPersistentAppUIPresignedURL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_studio_session_mapping_request(
    body: &[u8],
) -> Result<GetStudioSessionMappingInput, String> {
    if body.is_empty() {
        return Ok(GetStudioSessionMappingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetStudioSessionMapping request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_bootstrap_actions_request(
    body: &[u8],
) -> Result<ListBootstrapActionsInput, String> {
    if body.is_empty() {
        return Ok(ListBootstrapActionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBootstrapActions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_clusters_request(body: &[u8]) -> Result<ListClustersInput, String> {
    if body.is_empty() {
        return Ok(ListClustersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListClusters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_instance_fleets_request(
    body: &[u8],
) -> Result<ListInstanceFleetsInput, String> {
    if body.is_empty() {
        return Ok(ListInstanceFleetsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInstanceFleets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_instance_groups_request(
    body: &[u8],
) -> Result<ListInstanceGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListInstanceGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInstanceGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_instances_request(body: &[u8]) -> Result<ListInstancesInput, String> {
    if body.is_empty() {
        return Ok(ListInstancesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_notebook_executions_request(
    body: &[u8],
) -> Result<ListNotebookExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListNotebookExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNotebookExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_release_labels_request(
    body: &[u8],
) -> Result<ListReleaseLabelsInput, String> {
    if body.is_empty() {
        return Ok(ListReleaseLabelsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListReleaseLabels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_security_configurations_request(
    body: &[u8],
) -> Result<ListSecurityConfigurationsInput, String> {
    if body.is_empty() {
        return Ok(ListSecurityConfigurationsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecurityConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_steps_request(body: &[u8]) -> Result<ListStepsInput, String> {
    if body.is_empty() {
        return Ok(ListStepsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSteps request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_studio_session_mappings_request(
    body: &[u8],
) -> Result<ListStudioSessionMappingsInput, String> {
    if body.is_empty() {
        return Ok(ListStudioSessionMappingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStudioSessionMappings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_studios_request(body: &[u8]) -> Result<ListStudiosInput, String> {
    if body.is_empty() {
        return Ok(ListStudiosInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStudios request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_supported_instance_types_request(
    body: &[u8],
) -> Result<ListSupportedInstanceTypesInput, String> {
    if body.is_empty() {
        return Ok(ListSupportedInstanceTypesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSupportedInstanceTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_cluster_request(body: &[u8]) -> Result<ModifyClusterInput, String> {
    if body.is_empty() {
        return Ok(ModifyClusterInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_instance_fleet_request(
    body: &[u8],
) -> Result<ModifyInstanceFleetInput, String> {
    if body.is_empty() {
        return Ok(ModifyInstanceFleetInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyInstanceFleet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_instance_groups_request(
    body: &[u8],
) -> Result<ModifyInstanceGroupsInput, String> {
    if body.is_empty() {
        return Ok(ModifyInstanceGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyInstanceGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_auto_scaling_policy_request(
    body: &[u8],
) -> Result<PutAutoScalingPolicyInput, String> {
    if body.is_empty() {
        return Ok(PutAutoScalingPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAutoScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_auto_termination_policy_request(
    body: &[u8],
) -> Result<PutAutoTerminationPolicyInput, String> {
    if body.is_empty() {
        return Ok(PutAutoTerminationPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAutoTerminationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_block_public_access_configuration_request(
    body: &[u8],
) -> Result<PutBlockPublicAccessConfigurationInput, String> {
    if body.is_empty() {
        return Ok(PutBlockPublicAccessConfigurationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutBlockPublicAccessConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_managed_scaling_policy_request(
    body: &[u8],
) -> Result<PutManagedScalingPolicyInput, String> {
    if body.is_empty() {
        return Ok(PutManagedScalingPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutManagedScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_auto_scaling_policy_request(
    body: &[u8],
) -> Result<RemoveAutoScalingPolicyInput, String> {
    if body.is_empty() {
        return Ok(RemoveAutoScalingPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveAutoScalingPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_auto_termination_policy_request(
    body: &[u8],
) -> Result<RemoveAutoTerminationPolicyInput, String> {
    if body.is_empty() {
        return Ok(RemoveAutoTerminationPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveAutoTerminationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_managed_scaling_policy_request(
    body: &[u8],
) -> Result<RemoveManagedScalingPolicyInput, String> {
    if body.is_empty() {
        return Ok(RemoveManagedScalingPolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveManagedScalingPolicy request: {e}"))
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
pub fn deserialize_run_job_flow_request(body: &[u8]) -> Result<RunJobFlowInput, String> {
    if body.is_empty() {
        return Ok(RunJobFlowInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RunJobFlow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_keep_job_flow_alive_when_no_steps_request(
    body: &[u8],
) -> Result<SetKeepJobFlowAliveWhenNoStepsInput, String> {
    if body.is_empty() {
        return Ok(SetKeepJobFlowAliveWhenNoStepsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetKeepJobFlowAliveWhenNoSteps request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_termination_protection_request(
    body: &[u8],
) -> Result<SetTerminationProtectionInput, String> {
    if body.is_empty() {
        return Ok(SetTerminationProtectionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetTerminationProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_unhealthy_node_replacement_request(
    body: &[u8],
) -> Result<SetUnhealthyNodeReplacementInput, String> {
    if body.is_empty() {
        return Ok(SetUnhealthyNodeReplacementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetUnhealthyNodeReplacement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_visible_to_all_users_request(
    body: &[u8],
) -> Result<SetVisibleToAllUsersInput, String> {
    if body.is_empty() {
        return Ok(SetVisibleToAllUsersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetVisibleToAllUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_notebook_execution_request(
    body: &[u8],
) -> Result<StartNotebookExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartNotebookExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartNotebookExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_notebook_execution_request(
    body: &[u8],
) -> Result<StopNotebookExecutionInput, String> {
    if body.is_empty() {
        return Ok(StopNotebookExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopNotebookExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_job_flows_request(
    body: &[u8],
) -> Result<TerminateJobFlowsInput, String> {
    if body.is_empty() {
        return Ok(TerminateJobFlowsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateJobFlows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_studio_request(body: &[u8]) -> Result<UpdateStudioInput, String> {
    if body.is_empty() {
        return Ok(UpdateStudioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStudio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_studio_session_mapping_request(
    body: &[u8],
) -> Result<UpdateStudioSessionMappingInput, String> {
    if body.is_empty() {
        return Ok(UpdateStudioSessionMappingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStudioSessionMapping request: {e}"))
}
