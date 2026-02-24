//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicediscovery

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_http_namespace_response(
    result: &CreateHttpNamespaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_private_dns_namespace_response(
    result: &CreatePrivateDnsNamespaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_public_dns_namespace_response(
    result: &CreatePublicDnsNamespaceResponse,
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
pub fn serialize_delete_namespace_response(result: &DeleteNamespaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_response(result: &DeleteServiceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_attributes_response(
    result: &DeleteServiceAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_instance_response(result: &DeregisterInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_discover_instances_response(result: &DiscoverInstancesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_discover_instances_revision_response(
    result: &DiscoverInstancesRevisionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_instance_response(result: &GetInstanceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_instances_health_status_response(
    result: &GetInstancesHealthStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_namespace_response(result: &GetNamespaceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_operation_response(result: &GetOperationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_service_response(result: &GetServiceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_service_attributes_response(
    result: &GetServiceAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_instances_response(result: &ListInstancesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_namespaces_response(result: &ListNamespacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_operations_response(result: &ListOperationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_services_response(result: &ListServicesResponse) -> MockResponse {
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
pub fn serialize_register_instance_response(result: &RegisterInstanceResponse) -> MockResponse {
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
pub fn serialize_update_http_namespace_response(
    result: &UpdateHttpNamespaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_instance_custom_health_status_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_private_dns_namespace_response(
    result: &UpdatePrivateDnsNamespaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_public_dns_namespace_response(
    result: &UpdatePublicDnsNamespaceResponse,
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
pub fn serialize_update_service_attributes_response(
    result: &UpdateServiceAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_http_namespace_request(
    body: &[u8],
) -> Result<CreateHttpNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(CreateHttpNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateHttpNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_private_dns_namespace_request(
    body: &[u8],
) -> Result<CreatePrivateDnsNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(CreatePrivateDnsNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePrivateDnsNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_public_dns_namespace_request(
    body: &[u8],
) -> Result<CreatePublicDnsNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(CreatePublicDnsNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePublicDnsNamespace request: {e}"))
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
pub fn deserialize_delete_namespace_request(body: &[u8]) -> Result<DeleteNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteNamespace request: {e}"))
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
pub fn deserialize_delete_service_attributes_request(
    body: &[u8],
) -> Result<DeleteServiceAttributesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteServiceAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteServiceAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_instance_request(
    body: &[u8],
) -> Result<DeregisterInstanceRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_discover_instances_request(
    body: &[u8],
) -> Result<DiscoverInstancesRequest, String> {
    if body.is_empty() {
        return Ok(DiscoverInstancesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DiscoverInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_discover_instances_revision_request(
    body: &[u8],
) -> Result<DiscoverInstancesRevisionRequest, String> {
    if body.is_empty() {
        return Ok(DiscoverInstancesRevisionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DiscoverInstancesRevision request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_instance_request(body: &[u8]) -> Result<GetInstanceRequest, String> {
    if body.is_empty() {
        return Ok(GetInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_instances_health_status_request(
    body: &[u8],
) -> Result<GetInstancesHealthStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetInstancesHealthStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInstancesHealthStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_namespace_request(body: &[u8]) -> Result<GetNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(GetNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_operation_request(body: &[u8]) -> Result<GetOperationRequest, String> {
    if body.is_empty() {
        return Ok(GetOperationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOperation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_service_request(body: &[u8]) -> Result<GetServiceRequest, String> {
    if body.is_empty() {
        return Ok(GetServiceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetService request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_service_attributes_request(
    body: &[u8],
) -> Result<GetServiceAttributesRequest, String> {
    if body.is_empty() {
        return Ok(GetServiceAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetServiceAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_instances_request(body: &[u8]) -> Result<ListInstancesRequest, String> {
    if body.is_empty() {
        return Ok(ListInstancesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_namespaces_request(body: &[u8]) -> Result<ListNamespacesRequest, String> {
    if body.is_empty() {
        return Ok(ListNamespacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNamespaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_operations_request(body: &[u8]) -> Result<ListOperationsRequest, String> {
    if body.is_empty() {
        return Ok(ListOperationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOperations request: {e}"))
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
pub fn deserialize_register_instance_request(
    body: &[u8],
) -> Result<RegisterInstanceRequest, String> {
    if body.is_empty() {
        return Ok(RegisterInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterInstance request: {e}"))
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
pub fn deserialize_update_http_namespace_request(
    body: &[u8],
) -> Result<UpdateHttpNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(UpdateHttpNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateHttpNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_instance_custom_health_status_request(
    body: &[u8],
) -> Result<UpdateInstanceCustomHealthStatusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateInstanceCustomHealthStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateInstanceCustomHealthStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_private_dns_namespace_request(
    body: &[u8],
) -> Result<UpdatePrivateDnsNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePrivateDnsNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePrivateDnsNamespace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_public_dns_namespace_request(
    body: &[u8],
) -> Result<UpdatePublicDnsNamespaceRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePublicDnsNamespaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePublicDnsNamespace request: {e}"))
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
pub fn deserialize_update_service_attributes_request(
    body: &[u8],
) -> Result<UpdateServiceAttributesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateServiceAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateServiceAttributes request: {e}"))
}
