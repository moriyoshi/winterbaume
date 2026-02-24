//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elasticache

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;
fn strip_outer_element(xml: &str) -> &str {
    // Find the end of the opening tag
    if let Some(close_pos) = xml.find('>') {
        let inner_start = close_pos + 1;
        // Find the last closing tag
        if let Some(last_open) = xml.rfind('<') {
            if last_open >= inner_start {
                return &xml[inner_start..last_open];
            }
        }
    }
    xml
}

/// Serialize response for awsQuery protocol.
pub fn serialize_add_tags_to_resource_response(result: &TagListMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AddTagsToResourceResult>{inner_xml}</AddTagsToResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddTagsToResourceResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddTagsToResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_authorize_cache_security_group_ingress_response(
    result: &AuthorizeCacheSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AuthorizeCacheSecurityGroupIngressResult>{inner_xml}</AuthorizeCacheSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeCacheSecurityGroupIngressResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeCacheSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_apply_update_action_response(
    result: &UpdateActionResultsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<BatchApplyUpdateActionResult>{inner_xml}</BatchApplyUpdateActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchApplyUpdateActionResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchApplyUpdateActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_stop_update_action_response(
    result: &UpdateActionResultsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<BatchStopUpdateActionResult>{inner_xml}</BatchStopUpdateActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchStopUpdateActionResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchStopUpdateActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_complete_migration_response(result: &CompleteMigrationResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CompleteMigrationResult>{inner_xml}</CompleteMigrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CompleteMigrationResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CompleteMigrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_serverless_cache_snapshot_response(
    result: &CopyServerlessCacheSnapshotResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CopyServerlessCacheSnapshotResult>{inner_xml}</CopyServerlessCacheSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyServerlessCacheSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyServerlessCacheSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_snapshot_response(result: &CopySnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CopySnapshotResult>{inner_xml}</CopySnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopySnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopySnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cache_cluster_response(result: &CreateCacheClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateCacheClusterResult>{inner_xml}</CreateCacheClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCacheClusterResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCacheClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cache_parameter_group_response(
    result: &CreateCacheParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateCacheParameterGroupResult>{inner_xml}</CreateCacheParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCacheParameterGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCacheParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cache_security_group_response(
    result: &CreateCacheSecurityGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateCacheSecurityGroupResult>{inner_xml}</CreateCacheSecurityGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCacheSecurityGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCacheSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cache_subnet_group_response(
    result: &CreateCacheSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateCacheSubnetGroupResult>{inner_xml}</CreateCacheSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCacheSubnetGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCacheSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_global_replication_group_response(
    result: &CreateGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateGlobalReplicationGroupResult>{inner_xml}</CreateGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_replication_group_response(
    result: &CreateReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateReplicationGroupResult>{inner_xml}</CreateReplicationGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_serverless_cache_response(
    result: &CreateServerlessCacheResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateServerlessCacheResult>{inner_xml}</CreateServerlessCacheResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateServerlessCacheResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateServerlessCacheResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_serverless_cache_snapshot_response(
    result: &CreateServerlessCacheSnapshotResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateServerlessCacheSnapshotResult>{inner_xml}</CreateServerlessCacheSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateServerlessCacheSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateServerlessCacheSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_snapshot_response(result: &CreateSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateSnapshotResult>{inner_xml}</CreateSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_user_response(result: &User) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateUserResult>{inner_xml}</CreateUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateUserResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_user_group_response(result: &UserGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateUserGroupResult>{inner_xml}</CreateUserGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateUserGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateUserGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_decrease_node_groups_in_global_replication_group_response(
    result: &DecreaseNodeGroupsInGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DecreaseNodeGroupsInGlobalReplicationGroupResult>{inner_xml}</DecreaseNodeGroupsInGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DecreaseNodeGroupsInGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DecreaseNodeGroupsInGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_decrease_replica_count_response(
    result: &DecreaseReplicaCountResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DecreaseReplicaCountResult>{inner_xml}</DecreaseReplicaCountResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DecreaseReplicaCountResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DecreaseReplicaCountResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_cache_cluster_response(result: &DeleteCacheClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteCacheClusterResult>{inner_xml}</DeleteCacheClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCacheClusterResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCacheClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cache_parameter_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCacheParameterGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCacheParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cache_security_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCacheSecurityGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCacheSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cache_subnet_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCacheSubnetGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCacheSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_global_replication_group_response(
    result: &DeleteGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteGlobalReplicationGroupResult>{inner_xml}</DeleteGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_replication_group_response(
    result: &DeleteReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteReplicationGroupResult>{inner_xml}</DeleteReplicationGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_serverless_cache_response(
    result: &DeleteServerlessCacheResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteServerlessCacheResult>{inner_xml}</DeleteServerlessCacheResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteServerlessCacheResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteServerlessCacheResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_serverless_cache_snapshot_response(
    result: &DeleteServerlessCacheSnapshotResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteServerlessCacheSnapshotResult>{inner_xml}</DeleteServerlessCacheSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteServerlessCacheSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteServerlessCacheSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_snapshot_response(result: &DeleteSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteSnapshotResult>{inner_xml}</DeleteSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_user_response(result: &User) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteUserResult>{inner_xml}</DeleteUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUserResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_user_group_response(result: &UserGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteUserGroupResult>{inner_xml}</DeleteUserGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUserGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_clusters_response(result: &CacheClusterMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeCacheClustersResult>{inner_xml}</DescribeCacheClustersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheClustersResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheClustersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_engine_versions_response(
    result: &CacheEngineVersionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeCacheEngineVersionsResult>{inner_xml}</DescribeCacheEngineVersionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheEngineVersionsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheEngineVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_parameter_groups_response(
    result: &CacheParameterGroupsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeCacheParameterGroupsResult>{inner_xml}</DescribeCacheParameterGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheParameterGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheParameterGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_parameters_response(
    result: &CacheParameterGroupDetails,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeCacheParametersResult>{inner_xml}</DescribeCacheParametersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheParametersResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_security_groups_response(
    result: &CacheSecurityGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeCacheSecurityGroupsResult>{inner_xml}</DescribeCacheSecurityGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheSecurityGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheSecurityGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cache_subnet_groups_response(
    result: &CacheSubnetGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeCacheSubnetGroupsResult>{inner_xml}</DescribeCacheSubnetGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCacheSubnetGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCacheSubnetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_engine_default_parameters_response(
    result: &DescribeEngineDefaultParametersResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEngineDefaultParametersResult>{inner_xml}</DescribeEngineDefaultParametersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEngineDefaultParametersResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEngineDefaultParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_events_response(result: &EventsMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeEventsResult>{inner_xml}</DescribeEventsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEventsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_global_replication_groups_response(
    result: &DescribeGlobalReplicationGroupsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeGlobalReplicationGroupsResult>{inner_xml}</DescribeGlobalReplicationGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeGlobalReplicationGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeGlobalReplicationGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_replication_groups_response(
    result: &ReplicationGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeReplicationGroupsResult>{inner_xml}</DescribeReplicationGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReplicationGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReplicationGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_cache_nodes_response(
    result: &ReservedCacheNodeMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeReservedCacheNodesResult>{inner_xml}</DescribeReservedCacheNodesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedCacheNodesResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedCacheNodesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_cache_nodes_offerings_response(
    result: &ReservedCacheNodesOfferingMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeReservedCacheNodesOfferingsResult>{inner_xml}</DescribeReservedCacheNodesOfferingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedCacheNodesOfferingsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedCacheNodesOfferingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_serverless_cache_snapshots_response(
    result: &DescribeServerlessCacheSnapshotsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeServerlessCacheSnapshotsResult>{inner_xml}</DescribeServerlessCacheSnapshotsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeServerlessCacheSnapshotsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeServerlessCacheSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_serverless_caches_response(
    result: &DescribeServerlessCachesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeServerlessCachesResult>{inner_xml}</DescribeServerlessCachesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeServerlessCachesResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeServerlessCachesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_service_updates_response(result: &ServiceUpdatesMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeServiceUpdatesResult>{inner_xml}</DescribeServiceUpdatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeServiceUpdatesResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeServiceUpdatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_snapshots_response(
    result: &DescribeSnapshotsListMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeSnapshotsResult>{inner_xml}</DescribeSnapshotsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeSnapshotsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_update_actions_response(result: &UpdateActionsMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeUpdateActionsResult>{inner_xml}</DescribeUpdateActionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeUpdateActionsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeUpdateActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_user_groups_response(result: &DescribeUserGroupsResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeUserGroupsResult>{inner_xml}</DescribeUserGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeUserGroupsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeUserGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_users_response(result: &DescribeUsersResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeUsersResult>{inner_xml}</DescribeUsersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeUsersResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeUsersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disassociate_global_replication_group_response(
    result: &DisassociateGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DisassociateGlobalReplicationGroupResult>{inner_xml}</DisassociateGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisassociateGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisassociateGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_export_serverless_cache_snapshot_response(
    result: &ExportServerlessCacheSnapshotResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ExportServerlessCacheSnapshotResult>{inner_xml}</ExportServerlessCacheSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ExportServerlessCacheSnapshotResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ExportServerlessCacheSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_failover_global_replication_group_response(
    result: &FailoverGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<FailoverGlobalReplicationGroupResult>{inner_xml}</FailoverGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<FailoverGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</FailoverGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_increase_node_groups_in_global_replication_group_response(
    result: &IncreaseNodeGroupsInGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<IncreaseNodeGroupsInGlobalReplicationGroupResult>{inner_xml}</IncreaseNodeGroupsInGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<IncreaseNodeGroupsInGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</IncreaseNodeGroupsInGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_increase_replica_count_response(
    result: &IncreaseReplicaCountResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<IncreaseReplicaCountResult>{inner_xml}</IncreaseReplicaCountResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<IncreaseReplicaCountResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</IncreaseReplicaCountResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_allowed_node_type_modifications_response(
    result: &AllowedNodeTypeModificationsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListAllowedNodeTypeModificationsResult>{inner_xml}</ListAllowedNodeTypeModificationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAllowedNodeTypeModificationsResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAllowedNodeTypeModificationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_tags_for_resource_response(result: &TagListMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTagsForResourceResult>{inner_xml}</ListTagsForResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTagsForResourceResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cache_cluster_response(result: &ModifyCacheClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyCacheClusterResult>{inner_xml}</ModifyCacheClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCacheClusterResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCacheClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cache_parameter_group_response(
    result: &CacheParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyCacheParameterGroupResult>{inner_xml}</ModifyCacheParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCacheParameterGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCacheParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cache_subnet_group_response(
    result: &ModifyCacheSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyCacheSubnetGroupResult>{inner_xml}</ModifyCacheSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCacheSubnetGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCacheSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_global_replication_group_response(
    result: &ModifyGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyGlobalReplicationGroupResult>{inner_xml}</ModifyGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_replication_group_response(
    result: &ModifyReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyReplicationGroupResult>{inner_xml}</ModifyReplicationGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_replication_group_shard_configuration_response(
    result: &ModifyReplicationGroupShardConfigurationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyReplicationGroupShardConfigurationResult>{inner_xml}</ModifyReplicationGroupShardConfigurationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyReplicationGroupShardConfigurationResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyReplicationGroupShardConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_serverless_cache_response(
    result: &ModifyServerlessCacheResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyServerlessCacheResult>{inner_xml}</ModifyServerlessCacheResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyServerlessCacheResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyServerlessCacheResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_user_response(result: &User) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyUserResult>{inner_xml}</ModifyUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyUserResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_user_group_response(result: &UserGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyUserGroupResult>{inner_xml}</ModifyUserGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyUserGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyUserGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_purchase_reserved_cache_nodes_offering_response(
    result: &PurchaseReservedCacheNodesOfferingResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<PurchaseReservedCacheNodesOfferingResult>{inner_xml}</PurchaseReservedCacheNodesOfferingResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PurchaseReservedCacheNodesOfferingResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PurchaseReservedCacheNodesOfferingResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_rebalance_slots_in_global_replication_group_response(
    result: &RebalanceSlotsInGlobalReplicationGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RebalanceSlotsInGlobalReplicationGroupResult>{inner_xml}</RebalanceSlotsInGlobalReplicationGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebalanceSlotsInGlobalReplicationGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebalanceSlotsInGlobalReplicationGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reboot_cache_cluster_response(result: &RebootCacheClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RebootCacheClusterResult>{inner_xml}</RebootCacheClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebootCacheClusterResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebootCacheClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_tags_from_resource_response(result: &TagListMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RemoveTagsFromResourceResult>{inner_xml}</RemoveTagsFromResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveTagsFromResourceResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveTagsFromResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reset_cache_parameter_group_response(
    result: &CacheParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ResetCacheParameterGroupResult>{inner_xml}</ResetCacheParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResetCacheParameterGroupResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResetCacheParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_revoke_cache_security_group_ingress_response(
    result: &RevokeCacheSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RevokeCacheSecurityGroupIngressResult>{inner_xml}</RevokeCacheSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RevokeCacheSecurityGroupIngressResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RevokeCacheSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_migration_response(result: &StartMigrationResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartMigrationResult>{inner_xml}</StartMigrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartMigrationResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartMigrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_test_failover_response(result: &TestFailoverResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TestFailoverResult>{inner_xml}</TestFailoverResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TestFailoverResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TestFailoverResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_test_migration_response(result: &TestMigrationResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TestMigrationResult>{inner_xml}</TestMigrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TestMigrationResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TestMigrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_node_group_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<NodeGroupConfiguration>, String> {
    let mut item = NodeGroupConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.NodeGroupId")) {
        item.node_group_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.PrimaryAvailabilityZone")) {
        item.primary_availability_zone = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.PrimaryOutpostArn")) {
        item.primary_outpost_arn = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ReplicaAvailabilityZones");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.AvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.replica_availability_zones = Some(AvailabilityZonesList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.ReplicaCount")) {
        item.replica_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ReplicaCount: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ReplicaOutpostArns");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.OutpostArn.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.replica_outpost_arns = Some(OutpostArnsList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Slots")) {
        item.slots = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_log_delivery_configuration_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LogDeliveryConfigurationRequest>, String> {
    let mut item = LogDeliveryConfigurationRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DestinationType")) {
        item.destination_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LogFormat")) {
        item.log_format = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LogType")) {
        item.log_type = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_parameter_name_value_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ParameterNameValue>, String> {
    let mut item = ParameterNameValue::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ParameterName")) {
        item.parameter_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ParameterValue")) {
        item.parameter_value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cache_usage_limits_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CacheUsageLimits>, String> {
    let mut item = CacheUsageLimits::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Filter>, String> {
    let mut item = Filter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = FilterValueList { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_data_storage_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DataStorage>, String> {
    let mut item = DataStorage::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Maximum")) {
        item.maximum = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Maximum: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Minimum")) {
        item.minimum = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Minimum: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_e_c_p_u_per_second_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ECPUPerSecond>, String> {
    let mut item = ECPUPerSecond::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Maximum")) {
        item.maximum = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Maximum: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Minimum")) {
        item.minimum = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Minimum: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Tag>, String> {
    let mut item = Tag::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Key")) {
        item.key = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_destination_details_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DestinationDetails>, String> {
    let mut item = DestinationDetails::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_regional_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RegionalConfiguration>, String> {
    let mut item = RegionalConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ReplicationGroupId")) {
        item.replication_group_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReplicationGroupRegion")) {
        item.replication_group_region = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ReshardingConfiguration");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.ReshardingConfiguration.{i}");
            match deserialize_resharding_configuration_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.resharding_configuration = ReshardingConfigurationList { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resharding_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReshardingConfiguration>, String> {
    let mut item = ReshardingConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.NodeGroupId")) {
        item.node_group_id = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.PreferredAvailabilityZones");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.AvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.preferred_availability_zones = Some(AvailabilityZonesList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_configure_shard_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConfigureShard>, String> {
    let mut item = ConfigureShard::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.NewReplicaCount")) {
        item.new_replica_count = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse NewReplicaCount: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NodeGroupId")) {
        item.node_group_id = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.PreferredAvailabilityZones");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.PreferredAvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.preferred_availability_zones =
                Some(PreferredAvailabilityZoneList { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.PreferredOutpostArns");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.PreferredOutpostArn.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.preferred_outpost_arns = Some(PreferredOutpostArnList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_time_range_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TimeRangeFilter>, String> {
    let mut item = TimeRangeFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.EndTime")) {
        item.end_time = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StartTime")) {
        item.start_time = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scale_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScaleConfig>, String> {
    let mut item = ScaleConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ScaleIntervalMinutes")) {
        item.scale_interval_minutes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ScaleIntervalMinutes: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ScalePercentage")) {
        item.scale_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ScalePercentage: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cloud_watch_logs_destination_details_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CloudWatchLogsDestinationDetails>, String> {
    let mut item = CloudWatchLogsDestinationDetails::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LogGroup")) {
        item.log_group = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_authentication_mode_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AuthenticationMode>, String> {
    let mut item = AuthenticationMode::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Passwords");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.passwords = Some(PasswordListInput { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_customer_node_endpoint_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CustomerNodeEndpoint>, String> {
    let mut item = CustomerNodeEndpoint::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Address")) {
        item.address = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Port")) {
        item.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_kinesis_firehose_destination_details_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<KinesisFirehoseDestinationDetails>, String> {
    let mut item = KinesisFirehoseDestinationDetails::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeliveryStream")) {
        item.delivery_stream = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AddTagsToResource.
pub fn deserialize_add_tags_to_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddTagsToResourceMessage, String> {
    let mut input = AddTagsToResourceMessage::default();
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = TagList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AuthorizeCacheSecurityGroupIngress.
pub fn deserialize_authorize_cache_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeCacheSecurityGroupIngressMessage, String> {
    let mut input = AuthorizeCacheSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CacheSecurityGroupName") {
        input.cache_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchApplyUpdateAction.
pub fn deserialize_batch_apply_update_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchApplyUpdateActionMessage, String> {
    let mut input = BatchApplyUpdateActionMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "CacheClusterIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_cluster_ids = Some(CacheClusterIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicationGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replication_group_ids = Some(ReplicationGroupIdList { items });
        }
    }
    if let Some(value) = params.get("ServiceUpdateName") {
        input.service_update_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchStopUpdateAction.
pub fn deserialize_batch_stop_update_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchStopUpdateActionMessage, String> {
    let mut input = BatchStopUpdateActionMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "CacheClusterIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_cluster_ids = Some(CacheClusterIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicationGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replication_group_ids = Some(ReplicationGroupIdList { items });
        }
    }
    if let Some(value) = params.get("ServiceUpdateName") {
        input.service_update_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CompleteMigration.
pub fn deserialize_complete_migration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CompleteMigrationMessage, String> {
    let mut input = CompleteMigrationMessage::default();
    if let Some(value) = params.get("Force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Force: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyServerlessCacheSnapshot.
pub fn deserialize_copy_serverless_cache_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyServerlessCacheSnapshotRequest, String> {
    let mut input = CopyServerlessCacheSnapshotRequest::default();
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceServerlessCacheSnapshotName") {
        input.source_serverless_cache_snapshot_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TargetServerlessCacheSnapshotName") {
        input.target_serverless_cache_snapshot_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopySnapshot.
pub fn deserialize_copy_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopySnapshotMessage, String> {
    let mut input = CopySnapshotMessage::default();
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceSnapshotName") {
        input.source_snapshot_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TargetBucket") {
        input.target_bucket = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetSnapshotName") {
        input.target_snapshot_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCacheCluster.
pub fn deserialize_create_cache_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCacheClusterMessage, String> {
    let mut input = CreateCacheClusterMessage::default();
    if let Some(value) = params.get("AZMode") {
        input.a_z_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("AuthToken") {
        input.auth_token = Some(value.to_string());
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = value.to_string();
    }
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheSecurityGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_security_group_names = Some(CacheSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("IpDiscovery") {
        input.ip_discovery = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LogDeliveryConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.LogDeliveryConfigurationRequest.{i}");
            match deserialize_log_delivery_configuration_request_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.log_delivery_configurations = Some(LogDeliveryConfigurationRequestList { items });
        }
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationTopicArn") {
        input.notification_topic_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("NumCacheNodes") {
        input.num_cache_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumCacheNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("OutpostMode") {
        input.outpost_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreferredAvailabilityZone") {
        input.preferred_availability_zone = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PreferredAvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.PreferredAvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.preferred_availability_zones = Some(PreferredAvailabilityZoneList { items });
        }
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredOutpostArn") {
        input.preferred_outpost_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PreferredOutpostArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.PreferredOutpostArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.preferred_outpost_arns = Some(PreferredOutpostArnList { items });
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SnapshotArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SnapshotArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.snapshot_arns = Some(SnapshotArnsList { items });
        }
    }
    if let Some(value) = params.get("SnapshotName") {
        input.snapshot_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotWindow") {
        input.snapshot_window = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TransitEncryptionEnabled") {
        input.transit_encryption_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse TransitEncryptionEnabled: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCacheParameterGroup.
pub fn deserialize_create_cache_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCacheParameterGroupMessage, String> {
    let mut input = CreateCacheParameterGroupMessage::default();
    if let Some(value) = params.get("CacheParameterGroupFamily") {
        input.cache_parameter_group_family = value.to_string();
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCacheSecurityGroup.
pub fn deserialize_create_cache_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCacheSecurityGroupMessage, String> {
    let mut input = CreateCacheSecurityGroupMessage::default();
    if let Some(value) = params.get("CacheSecurityGroupName") {
        input.cache_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCacheSubnetGroup.
pub fn deserialize_create_cache_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCacheSubnetGroupMessage, String> {
    let mut input = CreateCacheSubnetGroupMessage::default();
    if let Some(value) = params.get("CacheSubnetGroupDescription") {
        input.cache_subnet_group_description = value.to_string();
    }
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SubnetIdentifier.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_ids = SubnetIdentifierList { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateGlobalReplicationGroup.
pub fn deserialize_create_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateGlobalReplicationGroupMessage, String> {
    let mut input = CreateGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("GlobalReplicationGroupDescription") {
        input.global_replication_group_description = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalReplicationGroupIdSuffix") {
        input.global_replication_group_id_suffix = value.to_string();
    }
    if let Some(value) = params.get("PrimaryReplicationGroupId") {
        input.primary_replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateReplicationGroup.
pub fn deserialize_create_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateReplicationGroupMessage, String> {
    let mut input = CreateReplicationGroupMessage::default();
    if let Some(value) = params.get("AtRestEncryptionEnabled") {
        input.at_rest_encryption_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AtRestEncryptionEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("AuthToken") {
        input.auth_token = Some(value.to_string());
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AutomaticFailoverEnabled") {
        input.automatic_failover_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutomaticFailoverEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheSecurityGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_security_group_names = Some(CacheSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterMode") {
        input.cluster_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("DataTieringEnabled") {
        input.data_tiering_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DataTieringEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("IpDiscovery") {
        input.ip_discovery = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LogDeliveryConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.LogDeliveryConfigurationRequest.{i}");
            match deserialize_log_delivery_configuration_request_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.log_delivery_configurations = Some(LogDeliveryConfigurationRequestList { items });
        }
    }
    if let Some(value) = params.get("MultiAZEnabled") {
        input.multi_a_z_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NodeGroupConfiguration".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.NodeGroupConfiguration.{i}");
            match deserialize_node_group_configuration_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.node_group_configuration = Some(NodeGroupConfigurationList { items });
        }
    }
    if let Some(value) = params.get("NotificationTopicArn") {
        input.notification_topic_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("NumCacheClusters") {
        input.num_cache_clusters = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumCacheClusters: {e}"))?,
        );
    }
    if let Some(value) = params.get("NumNodeGroups") {
        input.num_node_groups = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumNodeGroups: {e}"))?,
        );
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PreferredCacheClusterAZs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.preferred_cache_cluster_a_zs = Some(AvailabilityZonesList { items });
        }
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PrimaryClusterId") {
        input.primary_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicasPerNodeGroup") {
        input.replicas_per_node_group = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ReplicasPerNodeGroup: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicationGroupDescription") {
        input.replication_group_description = value.to_string();
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    if let Some(value) = params.get("ServerlessCacheSnapshotName") {
        input.serverless_cache_snapshot_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SnapshotArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SnapshotArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.snapshot_arns = Some(SnapshotArnsList { items });
        }
    }
    if let Some(value) = params.get("SnapshotName") {
        input.snapshot_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotWindow") {
        input.snapshot_window = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TransitEncryptionEnabled") {
        input.transit_encryption_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse TransitEncryptionEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("TransitEncryptionMode") {
        input.transit_encryption_mode = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_group_ids = Some(UserGroupIdListInput { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateServerlessCache.
pub fn deserialize_create_serverless_cache_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateServerlessCacheRequest, String> {
    let mut input = CreateServerlessCacheRequest::default();
    if let Some(val) = deserialize_cache_usage_limits_from_query(params, "CacheUsageLimits")? {
        input.cache_usage_limits = Some(val);
    }
    if let Some(value) = params.get("DailySnapshotTime") {
        input.daily_snapshot_time = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SnapshotArnsToRestore".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SnapshotArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.snapshot_arns_to_restore = Some(SnapshotArnsList { items });
        }
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SubnetId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_ids = Some(SubnetIdsList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateServerlessCacheSnapshot.
pub fn deserialize_create_serverless_cache_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateServerlessCacheSnapshotRequest, String> {
    let mut input = CreateServerlessCacheSnapshotRequest::default();
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = value.to_string();
    }
    if let Some(value) = params.get("ServerlessCacheSnapshotName") {
        input.serverless_cache_snapshot_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateSnapshot.
pub fn deserialize_create_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateSnapshotMessage, String> {
    let mut input = CreateSnapshotMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotName") {
        input.snapshot_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateUser.
pub fn deserialize_create_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateUserMessage, String> {
    let mut input = CreateUserMessage::default();
    if let Some(value) = params.get("AccessString") {
        input.access_string = value.to_string();
    }
    if let Some(val) = deserialize_authentication_mode_from_query(params, "AuthenticationMode")? {
        input.authentication_mode = Some(val);
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("NoPasswordRequired") {
        input.no_password_required = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse NoPasswordRequired: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Passwords".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.passwords = Some(PasswordListInput { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("UserId") {
        input.user_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateUserGroup.
pub fn deserialize_create_user_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateUserGroupMessage, String> {
    let mut input = CreateUserGroupMessage::default();
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_ids = Some(UserIdListInput { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DecreaseNodeGroupsInGlobalReplicationGroup.
pub fn deserialize_decrease_node_groups_in_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DecreaseNodeGroupsInGlobalReplicationGroupMessage, String> {
    let mut input = DecreaseNodeGroupsInGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "GlobalNodeGroupsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.GlobalNodeGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.global_node_groups_to_remove = Some(GlobalNodeGroupIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "GlobalNodeGroupsToRetain".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.GlobalNodeGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.global_node_groups_to_retain = Some(GlobalNodeGroupIdList { items });
        }
    }
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("NodeGroupCount") {
        input.node_group_count = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse NodeGroupCount: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DecreaseReplicaCount.
pub fn deserialize_decrease_replica_count_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DecreaseReplicaCountMessage, String> {
    let mut input = DecreaseReplicaCountMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("NewReplicaCount") {
        input.new_replica_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NewReplicaCount: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicaConfiguration".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ConfigureShard.{i}");
            match deserialize_configure_shard_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replica_configuration = Some(ReplicaConfigurationList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicasToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replicas_to_remove = Some(RemoveReplicasList { items });
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCacheCluster.
pub fn deserialize_delete_cache_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCacheClusterMessage, String> {
    let mut input = DeleteCacheClusterMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = value.to_string();
    }
    if let Some(value) = params.get("FinalSnapshotIdentifier") {
        input.final_snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCacheParameterGroup.
pub fn deserialize_delete_cache_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCacheParameterGroupMessage, String> {
    let mut input = DeleteCacheParameterGroupMessage::default();
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCacheSecurityGroup.
pub fn deserialize_delete_cache_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCacheSecurityGroupMessage, String> {
    let mut input = DeleteCacheSecurityGroupMessage::default();
    if let Some(value) = params.get("CacheSecurityGroupName") {
        input.cache_security_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCacheSubnetGroup.
pub fn deserialize_delete_cache_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCacheSubnetGroupMessage, String> {
    let mut input = DeleteCacheSubnetGroupMessage::default();
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteGlobalReplicationGroup.
pub fn deserialize_delete_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteGlobalReplicationGroupMessage, String> {
    let mut input = DeleteGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("RetainPrimaryReplicationGroup") {
        input.retain_primary_replication_group = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse RetainPrimaryReplicationGroup: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteReplicationGroup.
pub fn deserialize_delete_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteReplicationGroupMessage, String> {
    let mut input = DeleteReplicationGroupMessage::default();
    if let Some(value) = params.get("FinalSnapshotIdentifier") {
        input.final_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("RetainPrimaryCluster") {
        input.retain_primary_cluster = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainPrimaryCluster: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteServerlessCache.
pub fn deserialize_delete_serverless_cache_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteServerlessCacheRequest, String> {
    let mut input = DeleteServerlessCacheRequest::default();
    if let Some(value) = params.get("FinalSnapshotName") {
        input.final_snapshot_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteServerlessCacheSnapshot.
pub fn deserialize_delete_serverless_cache_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteServerlessCacheSnapshotRequest, String> {
    let mut input = DeleteServerlessCacheSnapshotRequest::default();
    if let Some(value) = params.get("ServerlessCacheSnapshotName") {
        input.serverless_cache_snapshot_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSnapshot.
pub fn deserialize_delete_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSnapshotMessage, String> {
    let mut input = DeleteSnapshotMessage::default();
    if let Some(value) = params.get("SnapshotName") {
        input.snapshot_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUser.
pub fn deserialize_delete_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserMessage, String> {
    let mut input = DeleteUserMessage::default();
    if let Some(value) = params.get("UserId") {
        input.user_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUserGroup.
pub fn deserialize_delete_user_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserGroupMessage, String> {
    let mut input = DeleteUserGroupMessage::default();
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheClusters.
pub fn deserialize_describe_cache_clusters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheClustersMessage, String> {
    let mut input = DescribeCacheClustersMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("ShowCacheClustersNotInReplicationGroups") {
        input.show_cache_clusters_not_in_replication_groups =
            Some(value.parse::<bool>().map_err(|e| {
                format!("failed to parse ShowCacheClustersNotInReplicationGroups: {e}")
            })?);
    }
    if let Some(value) = params.get("ShowCacheNodeInfo") {
        input.show_cache_node_info = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ShowCacheNodeInfo: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheEngineVersions.
pub fn deserialize_describe_cache_engine_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheEngineVersionsMessage, String> {
    let mut input = DescribeCacheEngineVersionsMessage::default();
    if let Some(value) = params.get("CacheParameterGroupFamily") {
        input.cache_parameter_group_family = Some(value.to_string());
    }
    if let Some(value) = params.get("DefaultOnly") {
        input.default_only = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DefaultOnly: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheParameterGroups.
pub fn deserialize_describe_cache_parameter_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheParameterGroupsMessage, String> {
    let mut input = DescribeCacheParameterGroupsMessage::default();
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheParameters.
pub fn deserialize_describe_cache_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheParametersMessage, String> {
    let mut input = DescribeCacheParametersMessage::default();
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("Source") {
        input.source = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheSecurityGroups.
pub fn deserialize_describe_cache_security_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheSecurityGroupsMessage, String> {
    let mut input = DescribeCacheSecurityGroupsMessage::default();
    if let Some(value) = params.get("CacheSecurityGroupName") {
        input.cache_security_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCacheSubnetGroups.
pub fn deserialize_describe_cache_subnet_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCacheSubnetGroupsMessage, String> {
    let mut input = DescribeCacheSubnetGroupsMessage::default();
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEngineDefaultParameters.
pub fn deserialize_describe_engine_default_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEngineDefaultParametersMessage, String> {
    let mut input = DescribeEngineDefaultParametersMessage::default();
    if let Some(value) = params.get("CacheParameterGroupFamily") {
        input.cache_parameter_group_family = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEvents.
pub fn deserialize_describe_events_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventsMessage, String> {
    let mut input = DescribeEventsMessage::default();
    if let Some(value) = params.get("Duration") {
        input.duration = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Duration: {e}"))?,
        );
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("SourceIdentifier") {
        input.source_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceType") {
        input.source_type = Some(value.to_string());
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeGlobalReplicationGroups.
pub fn deserialize_describe_global_replication_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeGlobalReplicationGroupsMessage, String> {
    let mut input = DescribeGlobalReplicationGroupsMessage::default();
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("ShowMemberInfo") {
        input.show_member_info = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ShowMemberInfo: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReplicationGroups.
pub fn deserialize_describe_replication_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReplicationGroupsMessage, String> {
    let mut input = DescribeReplicationGroupsMessage::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedCacheNodes.
pub fn deserialize_describe_reserved_cache_nodes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedCacheNodesMessage, String> {
    let mut input = DescribeReservedCacheNodesMessage::default();
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("Duration") {
        input.duration = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("OfferingType") {
        input.offering_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ProductDescription") {
        input.product_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedCacheNodeId") {
        input.reserved_cache_node_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedCacheNodesOfferingId") {
        input.reserved_cache_nodes_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedCacheNodesOfferings.
pub fn deserialize_describe_reserved_cache_nodes_offerings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedCacheNodesOfferingsMessage, String> {
    let mut input = DescribeReservedCacheNodesOfferingsMessage::default();
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("Duration") {
        input.duration = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("OfferingType") {
        input.offering_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ProductDescription") {
        input.product_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedCacheNodesOfferingId") {
        input.reserved_cache_nodes_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeServerlessCacheSnapshots.
pub fn deserialize_describe_serverless_cache_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeServerlessCacheSnapshotsRequest, String> {
    let mut input = DescribeServerlessCacheSnapshotsRequest::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerlessCacheSnapshotName") {
        input.serverless_cache_snapshot_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotType") {
        input.snapshot_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeServerlessCaches.
pub fn deserialize_describe_serverless_caches_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeServerlessCachesRequest, String> {
    let mut input = DescribeServerlessCachesRequest::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeServiceUpdates.
pub fn deserialize_describe_service_updates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeServiceUpdatesMessage, String> {
    let mut input = DescribeServiceUpdatesMessage::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("ServiceUpdateName") {
        input.service_update_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ServiceUpdateStatus".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.service_update_status = Some(ServiceUpdateStatusList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeSnapshots.
pub fn deserialize_describe_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeSnapshotsMessage, String> {
    let mut input = DescribeSnapshotsMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ShowNodeGroupConfig") {
        input.show_node_group_config = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ShowNodeGroupConfig: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotName") {
        input.snapshot_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotSource") {
        input.snapshot_source = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeUpdateActions.
pub fn deserialize_describe_update_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeUpdateActionsMessage, String> {
    let mut input = DescribeUpdateActionsMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "CacheClusterIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_cluster_ids = Some(CacheClusterIdList { items });
        }
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicationGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replication_group_ids = Some(ReplicationGroupIdList { items });
        }
    }
    if let Some(value) = params.get("ServiceUpdateName") {
        input.service_update_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ServiceUpdateStatus".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.service_update_status = Some(ServiceUpdateStatusList { items });
        }
    }
    if let Some(val) = deserialize_time_range_filter_from_query(params, "ServiceUpdateTimeRange")? {
        input.service_update_time_range = Some(val);
    }
    if let Some(value) = params.get("ShowNodeLevelUpdateStatus") {
        input.show_node_level_update_status = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ShowNodeLevelUpdateStatus: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UpdateActionStatus".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.update_action_status = Some(UpdateActionStatusList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeUserGroups.
pub fn deserialize_describe_user_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeUserGroupsMessage, String> {
    let mut input = DescribeUserGroupsMessage::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeUsers.
pub fn deserialize_describe_users_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeUsersMessage, String> {
    let mut input = DescribeUsersMessage::default();
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserId") {
        input.user_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisassociateGlobalReplicationGroup.
pub fn deserialize_disassociate_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisassociateGlobalReplicationGroupMessage, String> {
    let mut input = DisassociateGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("ReplicationGroupRegion") {
        input.replication_group_region = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ExportServerlessCacheSnapshot.
pub fn deserialize_export_serverless_cache_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ExportServerlessCacheSnapshotRequest, String> {
    let mut input = ExportServerlessCacheSnapshotRequest::default();
    if let Some(value) = params.get("S3BucketName") {
        input.s3_bucket_name = value.to_string();
    }
    if let Some(value) = params.get("ServerlessCacheSnapshotName") {
        input.serverless_cache_snapshot_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for FailoverGlobalReplicationGroup.
pub fn deserialize_failover_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<FailoverGlobalReplicationGroupMessage, String> {
    let mut input = FailoverGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("PrimaryRegion") {
        input.primary_region = value.to_string();
    }
    if let Some(value) = params.get("PrimaryReplicationGroupId") {
        input.primary_replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for IncreaseNodeGroupsInGlobalReplicationGroup.
pub fn deserialize_increase_node_groups_in_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<IncreaseNodeGroupsInGlobalReplicationGroupMessage, String> {
    let mut input = IncreaseNodeGroupsInGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    if let Some(value) = params.get("NodeGroupCount") {
        input.node_group_count = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse NodeGroupCount: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RegionalConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.RegionalConfiguration.{i}");
            match deserialize_regional_configuration_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.regional_configurations = Some(RegionalConfigurationList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for IncreaseReplicaCount.
pub fn deserialize_increase_replica_count_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<IncreaseReplicaCountMessage, String> {
    let mut input = IncreaseReplicaCountMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("NewReplicaCount") {
        input.new_replica_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NewReplicaCount: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplicaConfiguration".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ConfigureShard.{i}");
            match deserialize_configure_shard_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.replica_configuration = Some(ReplicaConfigurationList { items });
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAllowedNodeTypeModifications.
pub fn deserialize_list_allowed_node_type_modifications_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAllowedNodeTypeModificationsMessage, String> {
    let mut input = ListAllowedNodeTypeModificationsMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTagsForResource.
pub fn deserialize_list_tags_for_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceMessage, String> {
    let mut input = ListTagsForResourceMessage::default();
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCacheCluster.
pub fn deserialize_modify_cache_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCacheClusterMessage, String> {
    let mut input = ModifyCacheClusterMessage::default();
    if let Some(value) = params.get("AZMode") {
        input.a_z_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("AuthToken") {
        input.auth_token = Some(value.to_string());
    }
    if let Some(value) = params.get("AuthTokenUpdateStrategy") {
        input.auth_token_update_strategy = Some(value.to_string());
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheNodeIdsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheNodeId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_node_ids_to_remove = Some(CacheNodeIdsList { items });
        }
    }
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheSecurityGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_security_group_names = Some(CacheSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("IpDiscovery") {
        input.ip_discovery = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LogDeliveryConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.LogDeliveryConfigurationRequest.{i}");
            match deserialize_log_delivery_configuration_request_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.log_delivery_configurations = Some(LogDeliveryConfigurationRequestList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NewAvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.PreferredAvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.new_availability_zones = Some(PreferredAvailabilityZoneList { items });
        }
    }
    if let Some(value) = params.get("NotificationTopicArn") {
        input.notification_topic_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationTopicStatus") {
        input.notification_topic_status = Some(value.to_string());
    }
    if let Some(value) = params.get("NumCacheNodes") {
        input.num_cache_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumCacheNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(val) = deserialize_scale_config_from_query(params, "ScaleConfig")? {
        input.scale_config = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotWindow") {
        input.snapshot_window = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCacheParameterGroup.
pub fn deserialize_modify_cache_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCacheParameterGroupMessage, String> {
    let mut input = ModifyCacheParameterGroupMessage::default();
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ParameterNameValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ParameterNameValue.{i}");
            match deserialize_parameter_name_value_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameter_name_values = ParameterNameValueList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCacheSubnetGroup.
pub fn deserialize_modify_cache_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCacheSubnetGroupMessage, String> {
    let mut input = ModifyCacheSubnetGroupMessage::default();
    if let Some(value) = params.get("CacheSubnetGroupDescription") {
        input.cache_subnet_group_description = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheSubnetGroupName") {
        input.cache_subnet_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SubnetIdentifier.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_ids = Some(SubnetIdentifierList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyGlobalReplicationGroup.
pub fn deserialize_modify_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyGlobalReplicationGroupMessage, String> {
    let mut input = ModifyGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("AutomaticFailoverEnabled") {
        input.automatic_failover_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutomaticFailoverEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalReplicationGroupDescription") {
        input.global_replication_group_description = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyReplicationGroup.
pub fn deserialize_modify_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyReplicationGroupMessage, String> {
    let mut input = ModifyReplicationGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("AuthToken") {
        input.auth_token = Some(value.to_string());
    }
    if let Some(value) = params.get("AuthTokenUpdateStrategy") {
        input.auth_token_update_strategy = Some(value.to_string());
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AutomaticFailoverEnabled") {
        input.automatic_failover_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutomaticFailoverEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("CacheNodeType") {
        input.cache_node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheSecurityGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_security_group_names = Some(CacheSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("ClusterMode") {
        input.cluster_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("IpDiscovery") {
        input.ip_discovery = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LogDeliveryConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.LogDeliveryConfigurationRequest.{i}");
            match deserialize_log_delivery_configuration_request_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.log_delivery_configurations = Some(LogDeliveryConfigurationRequestList { items });
        }
    }
    if let Some(value) = params.get("MultiAZEnabled") {
        input.multi_a_z_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("NodeGroupId") {
        input.node_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationTopicArn") {
        input.notification_topic_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationTopicStatus") {
        input.notification_topic_status = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PrimaryClusterId") {
        input.primary_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("RemoveUserGroups") {
        input.remove_user_groups = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RemoveUserGroups: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicationGroupDescription") {
        input.replication_group_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotWindow") {
        input.snapshot_window = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshottingClusterId") {
        input.snapshotting_cluster_id = Some(value.to_string());
    }
    if let Some(value) = params.get("TransitEncryptionEnabled") {
        input.transit_encryption_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse TransitEncryptionEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("TransitEncryptionMode") {
        input.transit_encryption_mode = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserGroupIdsToAdd".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_group_ids_to_add = Some(UserGroupIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserGroupIdsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_group_ids_to_remove = Some(UserGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyReplicationGroupShardConfiguration.
pub fn deserialize_modify_replication_group_shard_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyReplicationGroupShardConfigurationMessage, String> {
    let mut input = ModifyReplicationGroupShardConfigurationMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("NodeGroupCount") {
        input.node_group_count = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse NodeGroupCount: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NodeGroupsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.NodeGroupToRemove.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.node_groups_to_remove = Some(NodeGroupsToRemoveList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NodeGroupsToRetain".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.NodeGroupToRetain.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.node_groups_to_retain = Some(NodeGroupsToRetainList { items });
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReshardingConfiguration".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ReshardingConfiguration.{i}");
            match deserialize_resharding_configuration_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resharding_configuration = Some(ReshardingConfigurationList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyServerlessCache.
pub fn deserialize_modify_serverless_cache_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyServerlessCacheRequest, String> {
    let mut input = ModifyServerlessCacheRequest::default();
    if let Some(val) = deserialize_cache_usage_limits_from_query(params, "CacheUsageLimits")? {
        input.cache_usage_limits = Some(val);
    }
    if let Some(value) = params.get("DailySnapshotTime") {
        input.daily_snapshot_time = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("RemoveUserGroup") {
        input.remove_user_group = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RemoveUserGroup: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_group_ids = Some(SecurityGroupIdsList { items });
        }
    }
    if let Some(value) = params.get("ServerlessCacheName") {
        input.serverless_cache_name = value.to_string();
    }
    if let Some(value) = params.get("SnapshotRetentionLimit") {
        input.snapshot_retention_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SnapshotRetentionLimit: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyUser.
pub fn deserialize_modify_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyUserMessage, String> {
    let mut input = ModifyUserMessage::default();
    if let Some(value) = params.get("AccessString") {
        input.access_string = Some(value.to_string());
    }
    if let Some(value) = params.get("AppendAccessString") {
        input.append_access_string = Some(value.to_string());
    }
    if let Some(val) = deserialize_authentication_mode_from_query(params, "AuthenticationMode")? {
        input.authentication_mode = Some(val);
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("NoPasswordRequired") {
        input.no_password_required = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse NoPasswordRequired: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Passwords".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.passwords = Some(PasswordListInput { items });
        }
    }
    if let Some(value) = params.get("UserId") {
        input.user_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyUserGroup.
pub fn deserialize_modify_user_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyUserGroupMessage, String> {
    let mut input = ModifyUserGroupMessage::default();
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("UserGroupId") {
        input.user_group_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserIdsToAdd".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_ids_to_add = Some(UserIdListInput { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "UserIdsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.user_ids_to_remove = Some(UserIdListInput { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PurchaseReservedCacheNodesOffering.
pub fn deserialize_purchase_reserved_cache_nodes_offering_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PurchaseReservedCacheNodesOfferingMessage, String> {
    let mut input = PurchaseReservedCacheNodesOfferingMessage::default();
    if let Some(value) = params.get("CacheNodeCount") {
        input.cache_node_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse CacheNodeCount: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReservedCacheNodeId") {
        input.reserved_cache_node_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedCacheNodesOfferingId") {
        input.reserved_cache_nodes_offering_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RebalanceSlotsInGlobalReplicationGroup.
pub fn deserialize_rebalance_slots_in_global_replication_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebalanceSlotsInGlobalReplicationGroupMessage, String> {
    let mut input = RebalanceSlotsInGlobalReplicationGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?;
    }
    if let Some(value) = params.get("GlobalReplicationGroupId") {
        input.global_replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RebootCacheCluster.
pub fn deserialize_reboot_cache_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebootCacheClusterMessage, String> {
    let mut input = RebootCacheClusterMessage::default();
    if let Some(value) = params.get("CacheClusterId") {
        input.cache_cluster_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "CacheNodeIdsToReboot".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.CacheNodeId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cache_node_ids_to_reboot = CacheNodeIdsList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveTagsFromResource.
pub fn deserialize_remove_tags_from_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveTagsFromResourceMessage, String> {
    let mut input = RemoveTagsFromResourceMessage::default();
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = KeyList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResetCacheParameterGroup.
pub fn deserialize_reset_cache_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResetCacheParameterGroupMessage, String> {
    let mut input = ResetCacheParameterGroupMessage::default();
    if let Some(value) = params.get("CacheParameterGroupName") {
        input.cache_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ParameterNameValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ParameterNameValue.{i}");
            match deserialize_parameter_name_value_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameter_name_values = Some(ParameterNameValueList { items });
        }
    }
    if let Some(value) = params.get("ResetAllParameters") {
        input.reset_all_parameters = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ResetAllParameters: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for RevokeCacheSecurityGroupIngress.
pub fn deserialize_revoke_cache_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RevokeCacheSecurityGroupIngressMessage, String> {
    let mut input = RevokeCacheSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CacheSecurityGroupName") {
        input.cache_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartMigration.
pub fn deserialize_start_migration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartMigrationMessage, String> {
    let mut input = StartMigrationMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "CustomerNodeEndpointList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_customer_node_endpoint_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.customer_node_endpoint_list = CustomerNodeEndpointList { items };
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for TestFailover.
pub fn deserialize_test_failover_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TestFailoverMessage, String> {
    let mut input = TestFailoverMessage::default();
    if let Some(value) = params.get("NodeGroupId") {
        input.node_group_id = value.to_string();
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for TestMigration.
pub fn deserialize_test_migration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TestMigrationMessage, String> {
    let mut input = TestMigrationMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "CustomerNodeEndpointList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_customer_node_endpoint_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.customer_node_endpoint_list = CustomerNodeEndpointList { items };
        }
    }
    if let Some(value) = params.get("ReplicationGroupId") {
        input.replication_group_id = value.to_string();
    }
    Ok(input)
}
