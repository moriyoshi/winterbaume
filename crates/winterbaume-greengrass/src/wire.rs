//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-greengrass

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
pub fn serialize_associate_role_to_group_response(
    result: &AssociateRoleToGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_service_role_to_account_response(
    result: &AssociateServiceRoleToAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connector_definition_response(
    result: &CreateConnectorDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connector_definition_version_response(
    result: &CreateConnectorDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_core_definition_response(
    result: &CreateCoreDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_core_definition_version_response(
    result: &CreateCoreDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deployment_response(result: &CreateDeploymentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_device_definition_response(
    result: &CreateDeviceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_device_definition_version_response(
    result: &CreateDeviceDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_function_definition_response(
    result: &CreateFunctionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_function_definition_version_response(
    result: &CreateFunctionDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_certificate_authority_response(
    result: &CreateGroupCertificateAuthorityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_version_response(
    result: &CreateGroupVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_logger_definition_response(
    result: &CreateLoggerDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_logger_definition_version_response(
    result: &CreateLoggerDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_definition_response(
    result: &CreateResourceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_definition_version_response(
    result: &CreateResourceDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_software_update_job_response(
    result: &CreateSoftwareUpdateJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_subscription_definition_response(
    result: &CreateSubscriptionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_subscription_definition_version_response(
    result: &CreateSubscriptionDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_connector_definition_response(
    result: &DeleteConnectorDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_core_definition_response(
    result: &DeleteCoreDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_device_definition_response(
    result: &DeleteDeviceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_function_definition_response(
    result: &DeleteFunctionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_logger_definition_response(
    result: &DeleteLoggerDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_definition_response(
    result: &DeleteResourceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_subscription_definition_response(
    result: &DeleteSubscriptionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_role_from_group_response(
    result: &DisassociateRoleFromGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_service_role_from_account_response(
    result: &DisassociateServiceRoleFromAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_associated_role_response(result: &GetAssociatedRoleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_bulk_deployment_status_response(
    result: &GetBulkDeploymentStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connectivity_info_response(
    result: &GetConnectivityInfoResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connector_definition_response(
    result: &GetConnectorDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connector_definition_version_response(
    result: &GetConnectorDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_definition_response(result: &GetCoreDefinitionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_definition_version_response(
    result: &GetCoreDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployment_status_response(
    result: &GetDeploymentStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_device_definition_response(
    result: &GetDeviceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_device_definition_version_response(
    result: &GetDeviceDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_definition_response(
    result: &GetFunctionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_definition_version_response(
    result: &GetFunctionDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_response(result: &GetGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_certificate_authority_response(
    result: &GetGroupCertificateAuthorityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_certificate_configuration_response(
    result: &GetGroupCertificateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_version_response(result: &GetGroupVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_logger_definition_response(
    result: &GetLoggerDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_logger_definition_version_response(
    result: &GetLoggerDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_definition_response(
    result: &GetResourceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_definition_version_response(
    result: &GetResourceDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_role_for_account_response(
    result: &GetServiceRoleForAccountResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_subscription_definition_response(
    result: &GetSubscriptionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_subscription_definition_version_response(
    result: &GetSubscriptionDefinitionVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_thing_runtime_configuration_response(
    result: &GetThingRuntimeConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bulk_deployment_detailed_reports_response(
    result: &ListBulkDeploymentDetailedReportsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bulk_deployments_response(
    result: &ListBulkDeploymentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connector_definition_versions_response(
    result: &ListConnectorDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connector_definitions_response(
    result: &ListConnectorDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_definition_versions_response(
    result: &ListCoreDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_definitions_response(
    result: &ListCoreDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_deployments_response(result: &ListDeploymentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_device_definition_versions_response(
    result: &ListDeviceDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_device_definitions_response(
    result: &ListDeviceDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_function_definition_versions_response(
    result: &ListFunctionDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_function_definitions_response(
    result: &ListFunctionDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_group_certificate_authorities_response(
    result: &ListGroupCertificateAuthoritiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_group_versions_response(result: &ListGroupVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_logger_definition_versions_response(
    result: &ListLoggerDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_logger_definitions_response(
    result: &ListLoggerDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_definition_versions_response(
    result: &ListResourceDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_definitions_response(
    result: &ListResourceDefinitionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_subscription_definition_versions_response(
    result: &ListSubscriptionDefinitionVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_subscription_definitions_response(
    result: &ListSubscriptionDefinitionsResponse,
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
pub fn serialize_reset_deployments_response(result: &ResetDeploymentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_bulk_deployment_response(
    result: &StartBulkDeploymentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_bulk_deployment_response(
    result: &StopBulkDeploymentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_connectivity_info_response(
    result: &UpdateConnectivityInfoResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_connector_definition_response(
    result: &UpdateConnectorDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_core_definition_response(
    result: &UpdateCoreDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_device_definition_response(
    result: &UpdateDeviceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_definition_response(
    result: &UpdateFunctionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_group_certificate_configuration_response(
    result: &UpdateGroupCertificateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_logger_definition_response(
    result: &UpdateLoggerDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_definition_response(
    result: &UpdateResourceDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_subscription_definition_response(
    result: &UpdateSubscriptionDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_thing_runtime_configuration_response(
    result: &UpdateThingRuntimeConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_role_to_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateRoleToGroupRequest, String> {
    let mut input = AssociateRoleToGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateRoleToGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateRoleToGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_service_role_to_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateServiceRoleToAccountRequest, String> {
    let mut input = AssociateServiceRoleToAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateServiceRoleToAccountRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateServiceRoleToAccount request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connector_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectorDefinitionRequest, String> {
    let mut input = CreateConnectorDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectorDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConnectorDefinition request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connector_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectorDefinitionVersionRequest, String> {
    let mut input = CreateConnectorDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectorDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateConnectorDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_core_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCoreDefinitionRequest, String> {
    let mut input = CreateCoreDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCoreDefinitionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCoreDefinition request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_core_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCoreDefinitionVersionRequest, String> {
    let mut input = CreateCoreDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCoreDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCoreDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeploymentRequest, String> {
    let mut input = CreateDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_device_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeviceDefinitionRequest, String> {
    let mut input = CreateDeviceDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeviceDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDeviceDefinition request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_device_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeviceDefinitionVersionRequest, String> {
    let mut input = CreateDeviceDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeviceDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateDeviceDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_function_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionDefinitionRequest, String> {
    let mut input = CreateFunctionDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFunctionDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateFunctionDefinition request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_function_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionDefinitionVersionRequest, String> {
    let mut input = CreateFunctionDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFunctionDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateFunctionDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupRequest, String> {
    let mut input = CreateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroup request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_certificate_authority_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupCertificateAuthorityRequest, String> {
    let mut input = CreateGroupCertificateAuthorityRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupVersionRequest, String> {
    let mut input = CreateGroupVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroupVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_logger_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLoggerDefinitionRequest, String> {
    let mut input = CreateLoggerDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLoggerDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateLoggerDefinition request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_logger_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLoggerDefinitionVersionRequest, String> {
    let mut input = CreateLoggerDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLoggerDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateLoggerDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceDefinitionRequest, String> {
    let mut input = CreateResourceDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateResourceDefinition request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceDefinitionVersionRequest, String> {
    let mut input = CreateResourceDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateResourceDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_software_update_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSoftwareUpdateJobRequest, String> {
    let mut input = CreateSoftwareUpdateJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSoftwareUpdateJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateSoftwareUpdateJob request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_subscription_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSubscriptionDefinitionRequest, String> {
    let mut input = CreateSubscriptionDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSubscriptionDefinitionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateSubscriptionDefinition request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_subscription_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSubscriptionDefinitionVersionRequest, String> {
    let mut input = CreateSubscriptionDefinitionVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSubscriptionDefinitionVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateSubscriptionDefinitionVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connector_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectorDefinitionRequest, String> {
    let mut input = DeleteConnectorDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_core_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCoreDefinitionRequest, String> {
    let mut input = DeleteCoreDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_device_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDeviceDefinitionRequest, String> {
    let mut input = DeleteDeviceDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionDefinitionRequest, String> {
    let mut input = DeleteFunctionDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupRequest, String> {
    let mut input = DeleteGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_logger_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoggerDefinitionRequest, String> {
    let mut input = DeleteLoggerDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceDefinitionRequest, String> {
    let mut input = DeleteResourceDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_subscription_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSubscriptionDefinitionRequest, String> {
    let mut input = DeleteSubscriptionDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_role_from_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateRoleFromGroupRequest, String> {
    let mut input = DisassociateRoleFromGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_service_role_from_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateServiceRoleFromAccountRequest, String> {
    let input = DisassociateServiceRoleFromAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_associated_role_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAssociatedRoleRequest, String> {
    let mut input = GetAssociatedRoleRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_bulk_deployment_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBulkDeploymentStatusRequest, String> {
    let mut input = GetBulkDeploymentStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "BulkDeploymentId" => {
                input.bulk_deployment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connectivity_info_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectivityInfoRequest, String> {
    let mut input = GetConnectivityInfoRequest::default();
    for (name, value) in labels {
        match *name {
            "ThingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connector_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectorDefinitionRequest, String> {
    let mut input = GetConnectorDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connector_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectorDefinitionVersionRequest, String> {
    let mut input = GetConnectorDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            "ConnectorDefinitionVersionId" => {
                input.connector_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_core_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreDefinitionRequest, String> {
    let mut input = GetCoreDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_core_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreDefinitionVersionRequest, String> {
    let mut input = GetCoreDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            "CoreDefinitionVersionId" => {
                input.core_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deployment_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeploymentStatusRequest, String> {
    let mut input = GetDeploymentStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "DeploymentId" => {
                input.deployment_id = value.to_string();
            }
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_device_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeviceDefinitionRequest, String> {
    let mut input = GetDeviceDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_device_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeviceDefinitionVersionRequest, String> {
    let mut input = GetDeviceDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            "DeviceDefinitionVersionId" => {
                input.device_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionDefinitionRequest, String> {
    let mut input = GetFunctionDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionDefinitionVersionRequest, String> {
    let mut input = GetFunctionDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            "FunctionDefinitionVersionId" => {
                input.function_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupRequest, String> {
    let mut input = GetGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_certificate_authority_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupCertificateAuthorityRequest, String> {
    let mut input = GetGroupCertificateAuthorityRequest::default();
    for (name, value) in labels {
        match *name {
            "CertificateAuthorityId" => {
                input.certificate_authority_id = value.to_string();
            }
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_certificate_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupCertificateConfigurationRequest, String> {
    let mut input = GetGroupCertificateConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupVersionRequest, String> {
    let mut input = GetGroupVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            "GroupVersionId" => {
                input.group_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_logger_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLoggerDefinitionRequest, String> {
    let mut input = GetLoggerDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_logger_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLoggerDefinitionVersionRequest, String> {
    let mut input = GetLoggerDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            "LoggerDefinitionVersionId" => {
                input.logger_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceDefinitionRequest, String> {
    let mut input = GetResourceDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceDefinitionVersionRequest, String> {
    let mut input = GetResourceDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            "ResourceDefinitionVersionId" => {
                input.resource_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_role_for_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceRoleForAccountRequest, String> {
    let input = GetServiceRoleForAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_subscription_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSubscriptionDefinitionRequest, String> {
    let mut input = GetSubscriptionDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_subscription_definition_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSubscriptionDefinitionVersionRequest, String> {
    let mut input = GetSubscriptionDefinitionVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            "SubscriptionDefinitionVersionId" => {
                input.subscription_definition_version_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_thing_runtime_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetThingRuntimeConfigurationRequest, String> {
    let mut input = GetThingRuntimeConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ThingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bulk_deployment_detailed_reports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBulkDeploymentDetailedReportsRequest, String> {
    let mut input = ListBulkDeploymentDetailedReportsRequest::default();
    for (name, value) in labels {
        match *name {
            "BulkDeploymentId" => {
                input.bulk_deployment_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bulk_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBulkDeploymentsRequest, String> {
    let mut input = ListBulkDeploymentsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connector_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectorDefinitionVersionsRequest, String> {
    let mut input = ListConnectorDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connector_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectorDefinitionsRequest, String> {
    let mut input = ListConnectorDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_core_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreDefinitionVersionsRequest, String> {
    let mut input = ListCoreDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_core_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreDefinitionsRequest, String> {
    let mut input = ListCoreDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeploymentsRequest, String> {
    let mut input = ListDeploymentsRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_device_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeviceDefinitionVersionsRequest, String> {
    let mut input = ListDeviceDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_device_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeviceDefinitionsRequest, String> {
    let mut input = ListDeviceDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_function_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionDefinitionVersionsRequest, String> {
    let mut input = ListFunctionDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_function_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionDefinitionsRequest, String> {
    let mut input = ListFunctionDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_group_certificate_authorities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupCertificateAuthoritiesRequest, String> {
    let mut input = ListGroupCertificateAuthoritiesRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_group_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupVersionsRequest, String> {
    let mut input = ListGroupVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsRequest, String> {
    let mut input = ListGroupsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_logger_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLoggerDefinitionVersionsRequest, String> {
    let mut input = ListLoggerDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_logger_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLoggerDefinitionsRequest, String> {
    let mut input = ListLoggerDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceDefinitionVersionsRequest, String> {
    let mut input = ListResourceDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceDefinitionsRequest, String> {
    let mut input = ListResourceDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_subscription_definition_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSubscriptionDefinitionVersionsRequest, String> {
    let mut input = ListSubscriptionDefinitionVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_subscription_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSubscriptionDefinitionsRequest, String> {
    let mut input = ListSubscriptionDefinitionsRequest::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reset_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResetDeploymentsRequest, String> {
    let mut input = ResetDeploymentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResetDeploymentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ResetDeployments request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_bulk_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartBulkDeploymentRequest, String> {
    let mut input = StartBulkDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartBulkDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartBulkDeployment request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("X-Amzn-Client-Token")
        .and_then(|value| value.to_str().ok())
    {
        input.amzn_client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_bulk_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopBulkDeploymentRequest, String> {
    let mut input = StopBulkDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "BulkDeploymentId" => {
                input.bulk_deployment_id = value.to_string();
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
            "ResourceArn" => {
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
            "ResourceArn" => {
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
pub fn deserialize_update_connectivity_info_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectivityInfoRequest, String> {
    let mut input = UpdateConnectivityInfoRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectivityInfoRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateConnectivityInfo request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ThingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_connector_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectorDefinitionRequest, String> {
    let mut input = UpdateConnectorDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectorDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateConnectorDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ConnectorDefinitionId" => {
                input.connector_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_core_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCoreDefinitionRequest, String> {
    let mut input = UpdateCoreDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCoreDefinitionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCoreDefinition request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "CoreDefinitionId" => {
                input.core_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_device_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDeviceDefinitionRequest, String> {
    let mut input = UpdateDeviceDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDeviceDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDeviceDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DeviceDefinitionId" => {
                input.device_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionDefinitionRequest, String> {
    let mut input = UpdateFunctionDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateFunctionDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionDefinitionId" => {
                input.function_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupRequest, String> {
    let mut input = UpdateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_certificate_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupCertificateConfigurationRequest, String> {
    let mut input = UpdateGroupCertificateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupCertificateConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateGroupCertificateConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "GroupId" => {
                input.group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_logger_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLoggerDefinitionRequest, String> {
    let mut input = UpdateLoggerDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLoggerDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateLoggerDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "LoggerDefinitionId" => {
                input.logger_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceDefinitionRequest, String> {
    let mut input = UpdateResourceDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateResourceDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceDefinitionId" => {
                input.resource_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_subscription_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSubscriptionDefinitionRequest, String> {
    let mut input = UpdateSubscriptionDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSubscriptionDefinitionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSubscriptionDefinition request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "SubscriptionDefinitionId" => {
                input.subscription_definition_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_thing_runtime_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateThingRuntimeConfigurationRequest, String> {
    let mut input = UpdateThingRuntimeConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateThingRuntimeConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateThingRuntimeConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ThingName" => {
                input.thing_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
