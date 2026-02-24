//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ecs

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_capacity_provider_response(
    result: &CreateCapacityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_daemon_response(result: &CreateDaemonResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_express_gateway_service_response(
    result: &CreateExpressGatewayServiceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_service_response(result: &CreateServiceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_task_set_response(result: &CreateTaskSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_account_setting_response(
    result: &DeleteAccountSettingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_attributes_response(result: &DeleteAttributesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_capacity_provider_response(
    result: &DeleteCapacityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_daemon_response(result: &DeleteDaemonResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_daemon_task_definition_response(
    result: &DeleteDaemonTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_express_gateway_service_response(
    result: &DeleteExpressGatewayServiceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_response(result: &DeleteServiceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_task_definitions_response(
    result: &DeleteTaskDefinitionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_task_set_response(result: &DeleteTaskSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_container_instance_response(
    result: &DeregisterContainerInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_task_definition_response(
    result: &DeregisterTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_capacity_providers_response(
    result: &DescribeCapacityProvidersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_clusters_response(result: &DescribeClustersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_container_instances_response(
    result: &DescribeContainerInstancesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_daemon_response(result: &DescribeDaemonResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_daemon_deployments_response(
    result: &DescribeDaemonDeploymentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_daemon_revisions_response(
    result: &DescribeDaemonRevisionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_daemon_task_definition_response(
    result: &DescribeDaemonTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_express_gateway_service_response(
    result: &DescribeExpressGatewayServiceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_service_deployments_response(
    result: &DescribeServiceDeploymentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_service_revisions_response(
    result: &DescribeServiceRevisionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_services_response(result: &DescribeServicesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_task_definition_response(
    result: &DescribeTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_task_sets_response(result: &DescribeTaskSetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_tasks_response(result: &DescribeTasksResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_discover_poll_endpoint_response(
    result: &DiscoverPollEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_command_response(result: &ExecuteCommandResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_task_protection_response(result: &GetTaskProtectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_settings_response(
    result: &ListAccountSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_attributes_response(result: &ListAttributesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_clusters_response(result: &ListClustersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_container_instances_response(
    result: &ListContainerInstancesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_daemon_deployments_response(
    result: &ListDaemonDeploymentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_daemon_task_definitions_response(
    result: &ListDaemonTaskDefinitionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_daemons_response(result: &ListDaemonsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_service_deployments_response(
    result: &ListServiceDeploymentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_services_response(result: &ListServicesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_services_by_namespace_response(
    result: &ListServicesByNamespaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_task_definition_families_response(
    result: &ListTaskDefinitionFamiliesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_task_definitions_response(
    result: &ListTaskDefinitionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tasks_response(result: &ListTasksResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_account_setting_response(result: &PutAccountSettingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_account_setting_default_response(
    result: &PutAccountSettingDefaultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_attributes_response(result: &PutAttributesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_cluster_capacity_providers_response(
    result: &PutClusterCapacityProvidersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_container_instance_response(
    result: &RegisterContainerInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_daemon_task_definition_response(
    result: &RegisterDaemonTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_task_definition_response(
    result: &RegisterTaskDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_run_task_response(result: &RunTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_task_response(result: &StartTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_service_deployment_response(
    result: &StopServiceDeploymentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_task_response(result: &StopTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_submit_attachment_state_changes_response(
    result: &SubmitAttachmentStateChangesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_submit_container_state_change_response(
    result: &SubmitContainerStateChangeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_submit_task_state_change_response(
    result: &SubmitTaskStateChangeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_capacity_provider_response(
    result: &UpdateCapacityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_cluster_response(result: &UpdateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_cluster_settings_response(
    result: &UpdateClusterSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_container_agent_response(
    result: &UpdateContainerAgentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_container_instances_state_response(
    result: &UpdateContainerInstancesStateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_daemon_response(result: &UpdateDaemonResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_express_gateway_service_response(
    result: &UpdateExpressGatewayServiceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_service_response(result: &UpdateServiceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_service_primary_task_set_response(
    result: &UpdateServicePrimaryTaskSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_task_protection_response(
    result: &UpdateTaskProtectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_task_set_response(result: &UpdateTaskSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_capacity_provider_request(
    body: &[u8],
) -> Result<CreateCapacityProviderRequest, String> {
    if body.is_empty() {
        return Ok(CreateCapacityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCapacityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_cluster_request(body: &[u8]) -> Result<CreateClusterRequest, String> {
    if body.is_empty() {
        return Ok(CreateClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_daemon_request(body: &[u8]) -> Result<CreateDaemonRequest, String> {
    if body.is_empty() {
        return Ok(CreateDaemonRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDaemon request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_express_gateway_service_request(
    body: &[u8],
) -> Result<CreateExpressGatewayServiceRequest, String> {
    if body.is_empty() {
        return Ok(CreateExpressGatewayServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateExpressGatewayService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_service_request(body: &[u8]) -> Result<CreateServiceRequest, String> {
    if body.is_empty() {
        return Ok(CreateServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_task_set_request(body: &[u8]) -> Result<CreateTaskSetRequest, String> {
    if body.is_empty() {
        return Ok(CreateTaskSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTaskSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_account_setting_request(
    body: &[u8],
) -> Result<DeleteAccountSettingRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccountSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccountSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_attributes_request(
    body: &[u8],
) -> Result<DeleteAttributesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_capacity_provider_request(
    body: &[u8],
) -> Result<DeleteCapacityProviderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCapacityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCapacityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_cluster_request(body: &[u8]) -> Result<DeleteClusterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_daemon_request(body: &[u8]) -> Result<DeleteDaemonRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDaemonRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDaemon request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_daemon_task_definition_request(
    body: &[u8],
) -> Result<DeleteDaemonTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDaemonTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDaemonTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_express_gateway_service_request(
    body: &[u8],
) -> Result<DeleteExpressGatewayServiceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteExpressGatewayServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteExpressGatewayService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_service_request(body: &[u8]) -> Result<DeleteServiceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_task_definitions_request(
    body: &[u8],
) -> Result<DeleteTaskDefinitionsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTaskDefinitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTaskDefinitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_task_set_request(body: &[u8]) -> Result<DeleteTaskSetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTaskSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTaskSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_container_instance_request(
    body: &[u8],
) -> Result<DeregisterContainerInstanceRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterContainerInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterContainerInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_task_definition_request(
    body: &[u8],
) -> Result<DeregisterTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_capacity_providers_request(
    body: &[u8],
) -> Result<DescribeCapacityProvidersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCapacityProvidersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCapacityProviders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_clusters_request(
    body: &[u8],
) -> Result<DescribeClustersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeClustersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeClusters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_container_instances_request(
    body: &[u8],
) -> Result<DescribeContainerInstancesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeContainerInstancesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeContainerInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_daemon_request(body: &[u8]) -> Result<DescribeDaemonRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDaemonRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDaemon request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_daemon_deployments_request(
    body: &[u8],
) -> Result<DescribeDaemonDeploymentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDaemonDeploymentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDaemonDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_daemon_revisions_request(
    body: &[u8],
) -> Result<DescribeDaemonRevisionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDaemonRevisionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDaemonRevisions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_daemon_task_definition_request(
    body: &[u8],
) -> Result<DescribeDaemonTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDaemonTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDaemonTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_express_gateway_service_request(
    body: &[u8],
) -> Result<DescribeExpressGatewayServiceRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExpressGatewayServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExpressGatewayService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_service_deployments_request(
    body: &[u8],
) -> Result<DescribeServiceDeploymentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServiceDeploymentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServiceDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_service_revisions_request(
    body: &[u8],
) -> Result<DescribeServiceRevisionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServiceRevisionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServiceRevisions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_services_request(
    body: &[u8],
) -> Result<DescribeServicesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_task_definition_request(
    body: &[u8],
) -> Result<DescribeTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_task_sets_request(
    body: &[u8],
) -> Result<DescribeTaskSetsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTaskSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTaskSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_tasks_request(body: &[u8]) -> Result<DescribeTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_discover_poll_endpoint_request(
    body: &[u8],
) -> Result<DiscoverPollEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DiscoverPollEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DiscoverPollEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_command_request(body: &[u8]) -> Result<ExecuteCommandRequest, String> {
    if body.is_empty() {
        return Ok(ExecuteCommandRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteCommand request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_task_protection_request(
    body: &[u8],
) -> Result<GetTaskProtectionRequest, String> {
    if body.is_empty() {
        return Ok(GetTaskProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTaskProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_settings_request(
    body: &[u8],
) -> Result<ListAccountSettingsRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_attributes_request(body: &[u8]) -> Result<ListAttributesRequest, String> {
    if body.is_empty() {
        return Ok(ListAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_clusters_request(body: &[u8]) -> Result<ListClustersRequest, String> {
    if body.is_empty() {
        return Ok(ListClustersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListClusters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_container_instances_request(
    body: &[u8],
) -> Result<ListContainerInstancesRequest, String> {
    if body.is_empty() {
        return Ok(ListContainerInstancesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListContainerInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_daemon_deployments_request(
    body: &[u8],
) -> Result<ListDaemonDeploymentsRequest, String> {
    if body.is_empty() {
        return Ok(ListDaemonDeploymentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDaemonDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_daemon_task_definitions_request(
    body: &[u8],
) -> Result<ListDaemonTaskDefinitionsRequest, String> {
    if body.is_empty() {
        return Ok(ListDaemonTaskDefinitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDaemonTaskDefinitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_daemons_request(body: &[u8]) -> Result<ListDaemonsRequest, String> {
    if body.is_empty() {
        return Ok(ListDaemonsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDaemons request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_service_deployments_request(
    body: &[u8],
) -> Result<ListServiceDeploymentsRequest, String> {
    if body.is_empty() {
        return Ok(ListServiceDeploymentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServiceDeployments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_services_request(body: &[u8]) -> Result<ListServicesRequest, String> {
    if body.is_empty() {
        return Ok(ListServicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_services_by_namespace_request(
    body: &[u8],
) -> Result<ListServicesByNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(ListServicesByNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServicesByNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_task_definition_families_request(
    body: &[u8],
) -> Result<ListTaskDefinitionFamiliesRequest, String> {
    if body.is_empty() {
        return Ok(ListTaskDefinitionFamiliesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTaskDefinitionFamilies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_task_definitions_request(
    body: &[u8],
) -> Result<ListTaskDefinitionsRequest, String> {
    if body.is_empty() {
        return Ok(ListTaskDefinitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTaskDefinitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tasks_request(body: &[u8]) -> Result<ListTasksRequest, String> {
    if body.is_empty() {
        return Ok(ListTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_account_setting_request(
    body: &[u8],
) -> Result<PutAccountSettingRequest, String> {
    if body.is_empty() {
        return Ok(PutAccountSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAccountSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_account_setting_default_request(
    body: &[u8],
) -> Result<PutAccountSettingDefaultRequest, String> {
    if body.is_empty() {
        return Ok(PutAccountSettingDefaultRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAccountSettingDefault request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_attributes_request(body: &[u8]) -> Result<PutAttributesRequest, String> {
    if body.is_empty() {
        return Ok(PutAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_cluster_capacity_providers_request(
    body: &[u8],
) -> Result<PutClusterCapacityProvidersRequest, String> {
    if body.is_empty() {
        return Ok(PutClusterCapacityProvidersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutClusterCapacityProviders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_container_instance_request(
    body: &[u8],
) -> Result<RegisterContainerInstanceRequest, String> {
    if body.is_empty() {
        return Ok(RegisterContainerInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterContainerInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_daemon_task_definition_request(
    body: &[u8],
) -> Result<RegisterDaemonTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(RegisterDaemonTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterDaemonTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_task_definition_request(
    body: &[u8],
) -> Result<RegisterTaskDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(RegisterTaskDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterTaskDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_run_task_request(body: &[u8]) -> Result<RunTaskRequest, String> {
    if body.is_empty() {
        return Ok(RunTaskRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize RunTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_task_request(body: &[u8]) -> Result<StartTaskRequest, String> {
    if body.is_empty() {
        return Ok(StartTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_service_deployment_request(
    body: &[u8],
) -> Result<StopServiceDeploymentRequest, String> {
    if body.is_empty() {
        return Ok(StopServiceDeploymentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopServiceDeployment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_task_request(body: &[u8]) -> Result<StopTaskRequest, String> {
    if body.is_empty() {
        return Ok(StopTaskRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize StopTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_submit_attachment_state_changes_request(
    body: &[u8],
) -> Result<SubmitAttachmentStateChangesRequest, String> {
    if body.is_empty() {
        return Ok(SubmitAttachmentStateChangesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SubmitAttachmentStateChanges request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_submit_container_state_change_request(
    body: &[u8],
) -> Result<SubmitContainerStateChangeRequest, String> {
    if body.is_empty() {
        return Ok(SubmitContainerStateChangeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SubmitContainerStateChange request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_submit_task_state_change_request(
    body: &[u8],
) -> Result<SubmitTaskStateChangeRequest, String> {
    if body.is_empty() {
        return Ok(SubmitTaskStateChangeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SubmitTaskStateChange request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_capacity_provider_request(
    body: &[u8],
) -> Result<UpdateCapacityProviderRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCapacityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCapacityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_cluster_request(body: &[u8]) -> Result<UpdateClusterRequest, String> {
    if body.is_empty() {
        return Ok(UpdateClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_cluster_settings_request(
    body: &[u8],
) -> Result<UpdateClusterSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateClusterSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateClusterSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_container_agent_request(
    body: &[u8],
) -> Result<UpdateContainerAgentRequest, String> {
    if body.is_empty() {
        return Ok(UpdateContainerAgentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateContainerAgent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_container_instances_state_request(
    body: &[u8],
) -> Result<UpdateContainerInstancesStateRequest, String> {
    if body.is_empty() {
        return Ok(UpdateContainerInstancesStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateContainerInstancesState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_daemon_request(body: &[u8]) -> Result<UpdateDaemonRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDaemonRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDaemon request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_express_gateway_service_request(
    body: &[u8],
) -> Result<UpdateExpressGatewayServiceRequest, String> {
    if body.is_empty() {
        return Ok(UpdateExpressGatewayServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateExpressGatewayService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_service_request(body: &[u8]) -> Result<UpdateServiceRequest, String> {
    if body.is_empty() {
        return Ok(UpdateServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_service_primary_task_set_request(
    body: &[u8],
) -> Result<UpdateServicePrimaryTaskSetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateServicePrimaryTaskSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateServicePrimaryTaskSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_task_protection_request(
    body: &[u8],
) -> Result<UpdateTaskProtectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTaskProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTaskProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_task_set_request(body: &[u8]) -> Result<UpdateTaskSetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTaskSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTaskSet request: {e}"))
}
