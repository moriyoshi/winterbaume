//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dax

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_parameter_group_response(
    result: &CreateParameterGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_subnet_group_response(result: &CreateSubnetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_decrease_replication_factor_response(
    result: &DecreaseReplicationFactorResponse,
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
pub fn serialize_delete_parameter_group_response(
    result: &DeleteParameterGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_subnet_group_response(result: &DeleteSubnetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_clusters_response(result: &DescribeClustersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_default_parameters_response(
    result: &DescribeDefaultParametersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_events_response(result: &DescribeEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_parameter_groups_response(
    result: &DescribeParameterGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_parameters_response(result: &DescribeParametersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_subnet_groups_response(
    result: &DescribeSubnetGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_increase_replication_factor_response(
    result: &IncreaseReplicationFactorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reboot_node_response(result: &RebootNodeResponse) -> MockResponse {
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
pub fn serialize_update_cluster_response(result: &UpdateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_parameter_group_response(
    result: &UpdateParameterGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_subnet_group_response(result: &UpdateSubnetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
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
pub fn deserialize_create_parameter_group_request(
    body: &[u8],
) -> Result<CreateParameterGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateParameterGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateParameterGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_subnet_group_request(
    body: &[u8],
) -> Result<CreateSubnetGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateSubnetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSubnetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_decrease_replication_factor_request(
    body: &[u8],
) -> Result<DecreaseReplicationFactorRequest, String> {
    if body.is_empty() {
        return Ok(DecreaseReplicationFactorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DecreaseReplicationFactor request: {e}"))
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
pub fn deserialize_delete_parameter_group_request(
    body: &[u8],
) -> Result<DeleteParameterGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteParameterGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteParameterGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_subnet_group_request(
    body: &[u8],
) -> Result<DeleteSubnetGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSubnetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSubnetGroup request: {e}"))
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
pub fn deserialize_describe_default_parameters_request(
    body: &[u8],
) -> Result<DescribeDefaultParametersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDefaultParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDefaultParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_events_request(body: &[u8]) -> Result<DescribeEventsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_parameter_groups_request(
    body: &[u8],
) -> Result<DescribeParameterGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeParameterGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeParameterGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_parameters_request(
    body: &[u8],
) -> Result<DescribeParametersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_subnet_groups_request(
    body: &[u8],
) -> Result<DescribeSubnetGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSubnetGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSubnetGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_increase_replication_factor_request(
    body: &[u8],
) -> Result<IncreaseReplicationFactorRequest, String> {
    if body.is_empty() {
        return Ok(IncreaseReplicationFactorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize IncreaseReplicationFactor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_request(body: &[u8]) -> Result<ListTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reboot_node_request(body: &[u8]) -> Result<RebootNodeRequest, String> {
    if body.is_empty() {
        return Ok(RebootNodeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebootNode request: {e}"))
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
pub fn deserialize_update_cluster_request(body: &[u8]) -> Result<UpdateClusterRequest, String> {
    if body.is_empty() {
        return Ok(UpdateClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_parameter_group_request(
    body: &[u8],
) -> Result<UpdateParameterGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateParameterGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateParameterGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_subnet_group_request(
    body: &[u8],
) -> Result<UpdateSubnetGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSubnetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSubnetGroup request: {e}"))
}
