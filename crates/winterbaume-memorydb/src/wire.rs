//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-memorydb

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_update_cluster_response(
    result: &BatchUpdateClusterResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_snapshot_response(result: &CopySnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_a_c_l_response(result: &CreateACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_multi_region_cluster_response(
    result: &CreateMultiRegionClusterResponse,
) -> MockResponse {
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
pub fn serialize_create_snapshot_response(result: &CreateSnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_subnet_group_response(result: &CreateSubnetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_a_c_l_response(result: &DeleteACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_multi_region_cluster_response(
    result: &DeleteMultiRegionClusterResponse,
) -> MockResponse {
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
pub fn serialize_delete_snapshot_response(result: &DeleteSnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_subnet_group_response(result: &DeleteSubnetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_response(result: &DeleteUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_a_c_ls_response(result: &DescribeACLsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_clusters_response(result: &DescribeClustersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_engine_versions_response(
    result: &DescribeEngineVersionsResponse,
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
pub fn serialize_describe_multi_region_clusters_response(
    result: &DescribeMultiRegionClustersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_multi_region_parameter_groups_response(
    result: &DescribeMultiRegionParameterGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_multi_region_parameters_response(
    result: &DescribeMultiRegionParametersResponse,
) -> MockResponse {
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
pub fn serialize_describe_reserved_nodes_response(
    result: &DescribeReservedNodesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_reserved_nodes_offerings_response(
    result: &DescribeReservedNodesOfferingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_service_updates_response(
    result: &DescribeServiceUpdatesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_snapshots_response(result: &DescribeSnapshotsResponse) -> MockResponse {
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
pub fn serialize_describe_users_response(result: &DescribeUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_failover_shard_response(result: &FailoverShardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_allowed_multi_region_cluster_updates_response(
    result: &ListAllowedMultiRegionClusterUpdatesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_allowed_node_type_updates_response(
    result: &ListAllowedNodeTypeUpdatesResponse,
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
pub fn serialize_purchase_reserved_nodes_offering_response(
    result: &PurchaseReservedNodesOfferingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reset_parameter_group_response(
    result: &ResetParameterGroupResponse,
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
pub fn serialize_update_a_c_l_response(result: &UpdateACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_cluster_response(result: &UpdateClusterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_multi_region_cluster_response(
    result: &UpdateMultiRegionClusterResponse,
) -> MockResponse {
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

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_response(result: &UpdateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_update_cluster_request(
    body: &[u8],
) -> Result<BatchUpdateClusterRequest, String> {
    if body.is_empty() {
        return Ok(BatchUpdateClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchUpdateCluster request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_snapshot_request(body: &[u8]) -> Result<CopySnapshotRequest, String> {
    if body.is_empty() {
        return Ok(CopySnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopySnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_a_c_l_request(body: &[u8]) -> Result<CreateACLRequest, String> {
    if body.is_empty() {
        return Ok(CreateACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateACL request: {e}"))
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
pub fn deserialize_create_multi_region_cluster_request(
    body: &[u8],
) -> Result<CreateMultiRegionClusterRequest, String> {
    if body.is_empty() {
        return Ok(CreateMultiRegionClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMultiRegionCluster request: {e}"))
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
pub fn deserialize_create_snapshot_request(body: &[u8]) -> Result<CreateSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(CreateSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSnapshot request: {e}"))
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
pub fn deserialize_create_user_request(body: &[u8]) -> Result<CreateUserRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_a_c_l_request(body: &[u8]) -> Result<DeleteACLRequest, String> {
    if body.is_empty() {
        return Ok(DeleteACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteACL request: {e}"))
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
pub fn deserialize_delete_multi_region_cluster_request(
    body: &[u8],
) -> Result<DeleteMultiRegionClusterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMultiRegionClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMultiRegionCluster request: {e}"))
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
pub fn deserialize_delete_snapshot_request(body: &[u8]) -> Result<DeleteSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSnapshot request: {e}"))
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
pub fn deserialize_delete_user_request(body: &[u8]) -> Result<DeleteUserRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_a_c_ls_request(body: &[u8]) -> Result<DescribeACLsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeACLsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeACLs request: {e}"))
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
pub fn deserialize_describe_engine_versions_request(
    body: &[u8],
) -> Result<DescribeEngineVersionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEngineVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEngineVersions request: {e}"))
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
pub fn deserialize_describe_multi_region_clusters_request(
    body: &[u8],
) -> Result<DescribeMultiRegionClustersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMultiRegionClustersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMultiRegionClusters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_multi_region_parameter_groups_request(
    body: &[u8],
) -> Result<DescribeMultiRegionParameterGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMultiRegionParameterGroupsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMultiRegionParameterGroups request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_multi_region_parameters_request(
    body: &[u8],
) -> Result<DescribeMultiRegionParametersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMultiRegionParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMultiRegionParameters request: {e}"))
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
pub fn deserialize_describe_reserved_nodes_request(
    body: &[u8],
) -> Result<DescribeReservedNodesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeReservedNodesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReservedNodes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_reserved_nodes_offerings_request(
    body: &[u8],
) -> Result<DescribeReservedNodesOfferingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeReservedNodesOfferingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReservedNodesOfferings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_service_updates_request(
    body: &[u8],
) -> Result<DescribeServiceUpdatesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeServiceUpdatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServiceUpdates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_snapshots_request(
    body: &[u8],
) -> Result<DescribeSnapshotsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSnapshotsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSnapshots request: {e}"))
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
pub fn deserialize_describe_users_request(body: &[u8]) -> Result<DescribeUsersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_failover_shard_request(body: &[u8]) -> Result<FailoverShardRequest, String> {
    if body.is_empty() {
        return Ok(FailoverShardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize FailoverShard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_allowed_multi_region_cluster_updates_request(
    body: &[u8],
) -> Result<ListAllowedMultiRegionClusterUpdatesRequest, String> {
    if body.is_empty() {
        return Ok(ListAllowedMultiRegionClusterUpdatesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAllowedMultiRegionClusterUpdates request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_allowed_node_type_updates_request(
    body: &[u8],
) -> Result<ListAllowedNodeTypeUpdatesRequest, String> {
    if body.is_empty() {
        return Ok(ListAllowedNodeTypeUpdatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAllowedNodeTypeUpdates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_request(body: &[u8]) -> Result<ListTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_purchase_reserved_nodes_offering_request(
    body: &[u8],
) -> Result<PurchaseReservedNodesOfferingRequest, String> {
    if body.is_empty() {
        return Ok(PurchaseReservedNodesOfferingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PurchaseReservedNodesOffering request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reset_parameter_group_request(
    body: &[u8],
) -> Result<ResetParameterGroupRequest, String> {
    if body.is_empty() {
        return Ok(ResetParameterGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResetParameterGroup request: {e}"))
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
pub fn deserialize_update_a_c_l_request(body: &[u8]) -> Result<UpdateACLRequest, String> {
    if body.is_empty() {
        return Ok(UpdateACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateACL request: {e}"))
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
pub fn deserialize_update_multi_region_cluster_request(
    body: &[u8],
) -> Result<UpdateMultiRegionClusterRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMultiRegionClusterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMultiRegionCluster request: {e}"))
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

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_request(body: &[u8]) -> Result<UpdateUserRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUser request: {e}"))
}
