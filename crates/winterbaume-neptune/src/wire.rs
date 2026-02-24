//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-neptune

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

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_role_to_d_b_cluster_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddRoleToDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddRoleToDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_add_source_identifier_to_subscription_response(
    result: &AddSourceIdentifierToSubscriptionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AddSourceIdentifierToSubscriptionResult>{inner_xml}</AddSourceIdentifierToSubscriptionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddSourceIdentifierToSubscriptionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddSourceIdentifierToSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_tags_to_resource_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddTagsToResourceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddTagsToResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_apply_pending_maintenance_action_response(
    result: &ApplyPendingMaintenanceActionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ApplyPendingMaintenanceActionResult>{inner_xml}</ApplyPendingMaintenanceActionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ApplyPendingMaintenanceActionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ApplyPendingMaintenanceActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_d_b_cluster_parameter_group_response(
    result: &CopyDBClusterParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CopyDBClusterParameterGroupResult>{inner_xml}</CopyDBClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyDBClusterParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyDBClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_d_b_cluster_snapshot_response(
    result: &CopyDBClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CopyDBClusterSnapshotResult>{inner_xml}</CopyDBClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyDBClusterSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyDBClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_d_b_parameter_group_response(
    result: &CopyDBParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CopyDBParameterGroupResult>{inner_xml}</CopyDBParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyDBParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyDBParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_cluster_response(result: &CreateDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBClusterResult>{inner_xml}</CreateDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_cluster_endpoint_response(
    result: &CreateDBClusterEndpointOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDBClusterEndpointResult>{inner_xml}</CreateDBClusterEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBClusterEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBClusterEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_cluster_parameter_group_response(
    result: &CreateDBClusterParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateDBClusterParameterGroupResult>{inner_xml}</CreateDBClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBClusterParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_cluster_snapshot_response(
    result: &CreateDBClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDBClusterSnapshotResult>{inner_xml}</CreateDBClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBClusterSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_instance_response(result: &CreateDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBInstanceResult>{inner_xml}</CreateDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_parameter_group_response(
    result: &CreateDBParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDBParameterGroupResult>{inner_xml}</CreateDBParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_subnet_group_response(
    result: &CreateDBSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBSubnetGroupResult>{inner_xml}</CreateDBSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBSubnetGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_event_subscription_response(
    result: &CreateEventSubscriptionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateEventSubscriptionResult>{inner_xml}</CreateEventSubscriptionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateEventSubscriptionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateEventSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_global_cluster_response(
    result: &CreateGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateGlobalClusterResult>{inner_xml}</CreateGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateGlobalClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_cluster_response(result: &DeleteDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDBClusterResult>{inner_xml}</DeleteDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_cluster_endpoint_response(
    result: &DeleteDBClusterEndpointOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteDBClusterEndpointResult>{inner_xml}</DeleteDBClusterEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBClusterEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBClusterEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_d_b_cluster_parameter_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBClusterParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_cluster_snapshot_response(
    result: &DeleteDBClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteDBClusterSnapshotResult>{inner_xml}</DeleteDBClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBClusterSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_instance_response(result: &DeleteDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDBInstanceResult>{inner_xml}</DeleteDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_d_b_parameter_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_d_b_subnet_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBSubnetGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_event_subscription_response(
    result: &DeleteEventSubscriptionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteEventSubscriptionResult>{inner_xml}</DeleteEventSubscriptionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteEventSubscriptionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteEventSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_global_cluster_response(
    result: &DeleteGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteGlobalClusterResult>{inner_xml}</DeleteGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteGlobalClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_endpoints_response(
    result: &DBClusterEndpointMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBClusterEndpointsResult>{inner_xml}</DescribeDBClusterEndpointsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterEndpointsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterEndpointsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_parameter_groups_response(
    result: &DBClusterParameterGroupsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBClusterParameterGroupsResult>{inner_xml}</DescribeDBClusterParameterGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterParameterGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterParameterGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_parameters_response(
    result: &DBClusterParameterGroupDetails,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBClusterParametersResult>{inner_xml}</DescribeDBClusterParametersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterParametersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_snapshot_attributes_response(
    result: &DescribeDBClusterSnapshotAttributesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBClusterSnapshotAttributesResult>{inner_xml}</DescribeDBClusterSnapshotAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterSnapshotAttributesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterSnapshotAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_snapshots_response(
    result: &DBClusterSnapshotMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBClusterSnapshotsResult>{inner_xml}</DescribeDBClusterSnapshotsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterSnapshotsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_clusters_response(result: &DBClusterMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDBClustersResult>{inner_xml}</DescribeDBClustersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClustersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClustersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_engine_versions_response(
    result: &DBEngineVersionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBEngineVersionsResult>{inner_xml}</DescribeDBEngineVersionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBEngineVersionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBEngineVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_instances_response(result: &DBInstanceMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDBInstancesResult>{inner_xml}</DescribeDBInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBInstancesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_parameter_groups_response(
    result: &DBParameterGroupsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBParameterGroupsResult>{inner_xml}</DescribeDBParameterGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBParameterGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBParameterGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_parameters_response(
    result: &DBParameterGroupDetails,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBParametersResult>{inner_xml}</DescribeDBParametersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBParametersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_subnet_groups_response(
    result: &DBSubnetGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBSubnetGroupsResult>{inner_xml}</DescribeDBSubnetGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBSubnetGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBSubnetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_engine_default_cluster_parameters_response(
    result: &DescribeEngineDefaultClusterParametersResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEngineDefaultClusterParametersResult>{inner_xml}</DescribeEngineDefaultClusterParametersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEngineDefaultClusterParametersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEngineDefaultClusterParametersResponse>"#
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
        r#"<DescribeEngineDefaultParametersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEngineDefaultParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_event_categories_response(
    result: &EventCategoriesMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeEventCategoriesResult>{inner_xml}</DescribeEventCategoriesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEventCategoriesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventCategoriesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_event_subscriptions_response(
    result: &EventSubscriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeEventSubscriptionsResult>{inner_xml}</DescribeEventSubscriptionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEventSubscriptionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventSubscriptionsResponse>"#
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
        r#"<DescribeEventsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_global_clusters_response(result: &GlobalClustersMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeGlobalClustersResult>{inner_xml}</DescribeGlobalClustersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeGlobalClustersResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeGlobalClustersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_orderable_d_b_instance_options_response(
    result: &OrderableDBInstanceOptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeOrderableDBInstanceOptionsResult>{inner_xml}</DescribeOrderableDBInstanceOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeOrderableDBInstanceOptionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeOrderableDBInstanceOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_pending_maintenance_actions_response(
    result: &PendingMaintenanceActionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribePendingMaintenanceActionsResult>{inner_xml}</DescribePendingMaintenanceActionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribePendingMaintenanceActionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribePendingMaintenanceActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_valid_d_b_instance_modifications_response(
    result: &DescribeValidDBInstanceModificationsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeValidDBInstanceModificationsResult>{inner_xml}</DescribeValidDBInstanceModificationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeValidDBInstanceModificationsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeValidDBInstanceModificationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_failover_d_b_cluster_response(result: &FailoverDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<FailoverDBClusterResult>{inner_xml}</FailoverDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<FailoverDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</FailoverDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_failover_global_cluster_response(
    result: &FailoverGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<FailoverGlobalClusterResult>{inner_xml}</FailoverGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<FailoverGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</FailoverGlobalClusterResponse>"#
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
        r#"<ListTagsForResourceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_cluster_response(result: &ModifyDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBClusterResult>{inner_xml}</ModifyDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_cluster_endpoint_response(
    result: &ModifyDBClusterEndpointOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBClusterEndpointResult>{inner_xml}</ModifyDBClusterEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBClusterEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBClusterEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_cluster_parameter_group_response(
    result: &DBClusterParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyDBClusterParameterGroupResult>{inner_xml}</ModifyDBClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBClusterParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_cluster_snapshot_attribute_response(
    result: &ModifyDBClusterSnapshotAttributeResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyDBClusterSnapshotAttributeResult>{inner_xml}</ModifyDBClusterSnapshotAttributeResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBClusterSnapshotAttributeResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBClusterSnapshotAttributeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_instance_response(result: &ModifyDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBInstanceResult>{inner_xml}</ModifyDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_parameter_group_response(
    result: &DBParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBParameterGroupResult>{inner_xml}</ModifyDBParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_subnet_group_response(
    result: &ModifyDBSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBSubnetGroupResult>{inner_xml}</ModifyDBSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBSubnetGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_event_subscription_response(
    result: &ModifyEventSubscriptionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyEventSubscriptionResult>{inner_xml}</ModifyEventSubscriptionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyEventSubscriptionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyEventSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_global_cluster_response(
    result: &ModifyGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyGlobalClusterResult>{inner_xml}</ModifyGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyGlobalClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_promote_read_replica_d_b_cluster_response(
    result: &PromoteReadReplicaDBClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<PromoteReadReplicaDBClusterResult>{inner_xml}</PromoteReadReplicaDBClusterResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PromoteReadReplicaDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PromoteReadReplicaDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reboot_d_b_instance_response(result: &RebootDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RebootDBInstanceResult>{inner_xml}</RebootDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebootDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebootDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_from_global_cluster_response(
    result: &RemoveFromGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RemoveFromGlobalClusterResult>{inner_xml}</RemoveFromGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveFromGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveFromGlobalClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_role_from_d_b_cluster_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveRoleFromDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveRoleFromDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_source_identifier_from_subscription_response(
    result: &RemoveSourceIdentifierFromSubscriptionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RemoveSourceIdentifierFromSubscriptionResult>{inner_xml}</RemoveSourceIdentifierFromSubscriptionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveSourceIdentifierFromSubscriptionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveSourceIdentifierFromSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_tags_from_resource_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveTagsFromResourceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveTagsFromResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reset_d_b_cluster_parameter_group_response(
    result: &DBClusterParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ResetDBClusterParameterGroupResult>{inner_xml}</ResetDBClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResetDBClusterParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResetDBClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reset_d_b_parameter_group_response(
    result: &DBParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ResetDBParameterGroupResult>{inner_xml}</ResetDBParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResetDBParameterGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResetDBParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_d_b_cluster_from_snapshot_response(
    result: &RestoreDBClusterFromSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RestoreDBClusterFromSnapshotResult>{inner_xml}</RestoreDBClusterFromSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBClusterFromSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBClusterFromSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_d_b_cluster_to_point_in_time_response(
    result: &RestoreDBClusterToPointInTimeResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RestoreDBClusterToPointInTimeResult>{inner_xml}</RestoreDBClusterToPointInTimeResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBClusterToPointInTimeResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBClusterToPointInTimeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_d_b_cluster_response(result: &StartDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartDBClusterResult>{inner_xml}</StartDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_stop_d_b_cluster_response(result: &StopDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StopDBClusterResult>{inner_xml}</StopDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_switchover_global_cluster_response(
    result: &SwitchoverGlobalClusterResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SwitchoverGlobalClusterResult>{inner_xml}</SwitchoverGlobalClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SwitchoverGlobalClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SwitchoverGlobalClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_cloudwatch_logs_export_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CloudwatchLogsExportConfiguration>, String> {
    let mut item = CloudwatchLogsExportConfiguration::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.DisableLogTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.disable_log_types = Some(LogTypeList { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.EnableLogTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.enable_log_types = Some(LogTypeList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_serverless_v2_scaling_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ServerlessV2ScalingConfiguration>, String> {
    let mut item = ServerlessV2ScalingConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MaxCapacity")) {
        item.max_capacity = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MaxCapacity: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinCapacity")) {
        item.min_capacity = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MinCapacity: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_parameter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Parameter>, String> {
    let mut item = Parameter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AllowedValues")) {
        item.allowed_values = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ApplyMethod")) {
        item.apply_method = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ApplyType")) {
        item.apply_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.DataType")) {
        item.data_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Description")) {
        item.description = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IsModifiable")) {
        item.is_modifiable = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IsModifiable: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinimumEngineVersion")) {
        item.minimum_engine_version = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ParameterName")) {
        item.parameter_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ParameterValue")) {
        item.parameter_value = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Source")) {
        item.source = Some(value.to_string());
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
            let item_key = format!("{sub_list_prefix}.Value.{i}");
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

/// Deserialize awsQuery request for AddRoleToDBCluster.
pub fn deserialize_add_role_to_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddRoleToDBClusterMessage, String> {
    let mut input = AddRoleToDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("FeatureName") {
        input.feature_name = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddSourceIdentifierToSubscription.
pub fn deserialize_add_source_identifier_to_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddSourceIdentifierToSubscriptionMessage, String> {
    let mut input = AddSourceIdentifierToSubscriptionMessage::default();
    if let Some(value) = params.get("SourceIdentifier") {
        input.source_identifier = value.to_string();
    }
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = value.to_string();
    }
    Ok(input)
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

/// Deserialize awsQuery request for ApplyPendingMaintenanceAction.
pub fn deserialize_apply_pending_maintenance_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ApplyPendingMaintenanceActionMessage, String> {
    let mut input = ApplyPendingMaintenanceActionMessage::default();
    if let Some(value) = params.get("ApplyAction") {
        input.apply_action = value.to_string();
    }
    if let Some(value) = params.get("OptInType") {
        input.opt_in_type = value.to_string();
    }
    if let Some(value) = params.get("ResourceIdentifier") {
        input.resource_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyDBClusterParameterGroup.
pub fn deserialize_copy_d_b_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyDBClusterParameterGroupMessage, String> {
    let mut input = CopyDBClusterParameterGroupMessage::default();
    if let Some(value) = params.get("SourceDBClusterParameterGroupIdentifier") {
        input.source_d_b_cluster_parameter_group_identifier = value.to_string();
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
    if let Some(value) = params.get("TargetDBClusterParameterGroupDescription") {
        input.target_d_b_cluster_parameter_group_description = value.to_string();
    }
    if let Some(value) = params.get("TargetDBClusterParameterGroupIdentifier") {
        input.target_d_b_cluster_parameter_group_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyDBClusterSnapshot.
pub fn deserialize_copy_d_b_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyDBClusterSnapshotMessage, String> {
    let mut input = CopyDBClusterSnapshotMessage::default();
    if let Some(value) = params.get("CopyTags") {
        input.copy_tags = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTags: {e}"))?,
        );
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PreSignedUrl") {
        input.pre_signed_url = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBClusterSnapshotIdentifier") {
        input.source_d_b_cluster_snapshot_identifier = value.to_string();
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
    if let Some(value) = params.get("TargetDBClusterSnapshotIdentifier") {
        input.target_d_b_cluster_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyDBParameterGroup.
pub fn deserialize_copy_d_b_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyDBParameterGroupMessage, String> {
    let mut input = CopyDBParameterGroupMessage::default();
    if let Some(value) = params.get("SourceDBParameterGroupIdentifier") {
        input.source_d_b_parameter_group_identifier = value.to_string();
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
    if let Some(value) = params.get("TargetDBParameterGroupDescription") {
        input.target_d_b_parameter_group_description = value.to_string();
    }
    if let Some(value) = params.get("TargetDBParameterGroupIdentifier") {
        input.target_d_b_parameter_group_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBCluster.
pub fn deserialize_create_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBClusterMessage, String> {
    let mut input = CreateDBClusterMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZones { items });
        }
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("CharacterSetName") {
        input.character_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnableCloudwatchLogsExports".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.enable_cloudwatch_logs_exports = Some(LogTypeList { items });
        }
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreSignedUrl") {
        input.pre_signed_url = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("ReplicationSourceIdentifier") {
        input.replication_source_identifier = Some(value.to_string());
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("StorageEncrypted") {
        input.storage_encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse StorageEncrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBClusterEndpoint.
pub fn deserialize_create_d_b_cluster_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBClusterEndpointMessage, String> {
    let mut input = CreateDBClusterEndpointMessage::default();
    if let Some(value) = params.get("DBClusterEndpointIdentifier") {
        input.d_b_cluster_endpoint_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("EndpointType") {
        input.endpoint_type = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ExcludedMembers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.excluded_members = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StaticMembers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.static_members = Some(StringList { items });
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

/// Deserialize awsQuery request for CreateDBClusterParameterGroup.
pub fn deserialize_create_d_b_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBClusterParameterGroupMessage, String> {
    let mut input = CreateDBClusterParameterGroupMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = value.to_string();
    }
    if let Some(value) = params.get("DBParameterGroupFamily") {
        input.d_b_parameter_group_family = value.to_string();
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

/// Deserialize awsQuery request for CreateDBClusterSnapshot.
pub fn deserialize_create_d_b_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBClusterSnapshotMessage, String> {
    let mut input = CreateDBClusterSnapshotMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = value.to_string();
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

/// Deserialize awsQuery request for CreateDBInstance.
pub fn deserialize_create_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBInstanceMessage, String> {
    let mut input = CreateDBInstanceMessage::default();
    if let Some(value) = params.get("AllocatedStorage") {
        input.allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AllocatedStorage: {e}"))?,
        );
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AvailabilityZone") {
        input.availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("CharacterSetName") {
        input.character_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = value.to_string();
    }
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBName") {
        input.d_b_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DBSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.DBSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_security_groups = Some(DBSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("Domain") {
        input.domain = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnableCloudwatchLogsExports".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.enable_cloudwatch_logs_exports = Some(LogTypeList { items });
        }
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnablePerformanceInsights") {
        input.enable_performance_insights = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnablePerformanceInsights: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("Iops") {
        input.iops = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Iops: {e}"))?,
        );
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("LicenseModel") {
        input.license_model = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = Some(value.to_string());
    }
    if let Some(value) = params.get("MonitoringInterval") {
        input.monitoring_interval = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MonitoringInterval: {e}"))?,
        );
    }
    if let Some(value) = params.get("MonitoringRoleArn") {
        input.monitoring_role_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PromotionTier") {
        input.promotion_tier = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PromotionTier: {e}"))?,
        );
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageEncrypted") {
        input.storage_encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse StorageEncrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
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
    if let Some(value) = params.get("TdeCredentialArn") {
        input.tde_credential_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialPassword") {
        input.tde_credential_password = Some(value.to_string());
    }
    if let Some(value) = params.get("Timezone") {
        input.timezone = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBParameterGroup.
pub fn deserialize_create_d_b_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBParameterGroupMessage, String> {
    let mut input = CreateDBParameterGroupMessage::default();
    if let Some(value) = params.get("DBParameterGroupFamily") {
        input.d_b_parameter_group_family = value.to_string();
    }
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateDBSubnetGroup.
pub fn deserialize_create_d_b_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBSubnetGroupMessage, String> {
    let mut input = CreateDBSubnetGroupMessage::default();
    if let Some(value) = params.get("DBSubnetGroupDescription") {
        input.d_b_subnet_group_description = value.to_string();
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateEventSubscription.
pub fn deserialize_create_event_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateEventSubscriptionMessage, String> {
    let mut input = CreateEventSubscriptionMessage::default();
    if let Some(value) = params.get("Enabled") {
        input.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EventCategories".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.EventCategory.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.event_categories = Some(EventCategoriesList { items });
        }
    }
    if let Some(value) = params.get("SnsTopicArn") {
        input.sns_topic_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SourceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SourceId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.source_ids = Some(SourceIdsList { items });
        }
    }
    if let Some(value) = params.get("SourceType") {
        input.source_type = Some(value.to_string());
    }
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = value.to_string();
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

/// Deserialize awsQuery request for CreateGlobalCluster.
pub fn deserialize_create_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateGlobalClusterMessage, String> {
    let mut input = CreateGlobalClusterMessage::default();
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("SourceDBClusterIdentifier") {
        input.source_d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("StorageEncrypted") {
        input.storage_encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse StorageEncrypted: {e}"))?,
        );
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

/// Deserialize awsQuery request for DeleteDBCluster.
pub fn deserialize_delete_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBClusterMessage, String> {
    let mut input = DeleteDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("FinalDBSnapshotIdentifier") {
        input.final_d_b_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SkipFinalSnapshot") {
        input.skip_final_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipFinalSnapshot: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBClusterEndpoint.
pub fn deserialize_delete_d_b_cluster_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBClusterEndpointMessage, String> {
    let mut input = DeleteDBClusterEndpointMessage::default();
    if let Some(value) = params.get("DBClusterEndpointIdentifier") {
        input.d_b_cluster_endpoint_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBClusterParameterGroup.
pub fn deserialize_delete_d_b_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBClusterParameterGroupMessage, String> {
    let mut input = DeleteDBClusterParameterGroupMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBClusterSnapshot.
pub fn deserialize_delete_d_b_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBClusterSnapshotMessage, String> {
    let mut input = DeleteDBClusterSnapshotMessage::default();
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBInstance.
pub fn deserialize_delete_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBInstanceMessage, String> {
    let mut input = DeleteDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("FinalDBSnapshotIdentifier") {
        input.final_d_b_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SkipFinalSnapshot") {
        input.skip_final_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipFinalSnapshot: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBParameterGroup.
pub fn deserialize_delete_d_b_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBParameterGroupMessage, String> {
    let mut input = DeleteDBParameterGroupMessage::default();
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBSubnetGroup.
pub fn deserialize_delete_d_b_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBSubnetGroupMessage, String> {
    let mut input = DeleteDBSubnetGroupMessage::default();
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteEventSubscription.
pub fn deserialize_delete_event_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteEventSubscriptionMessage, String> {
    let mut input = DeleteEventSubscriptionMessage::default();
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteGlobalCluster.
pub fn deserialize_delete_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteGlobalClusterMessage, String> {
    let mut input = DeleteGlobalClusterMessage::default();
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusterEndpoints.
pub fn deserialize_describe_d_b_cluster_endpoints_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterEndpointsMessage, String> {
    let mut input = DescribeDBClusterEndpointsMessage::default();
    if let Some(value) = params.get("DBClusterEndpointIdentifier") {
        input.d_b_cluster_endpoint_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusterParameterGroups.
pub fn deserialize_describe_d_b_cluster_parameter_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterParameterGroupsMessage, String> {
    let mut input = DescribeDBClusterParameterGroupsMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusterParameters.
pub fn deserialize_describe_d_b_cluster_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterParametersMessage, String> {
    let mut input = DescribeDBClusterParametersMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    if let Some(value) = params.get("Source") {
        input.source = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusterSnapshotAttributes.
pub fn deserialize_describe_d_b_cluster_snapshot_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterSnapshotAttributesMessage, String> {
    let mut input = DescribeDBClusterSnapshotAttributesMessage::default();
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusterSnapshots.
pub fn deserialize_describe_d_b_cluster_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterSnapshotsMessage, String> {
    let mut input = DescribeDBClusterSnapshotsMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("IncludePublic") {
        input.include_public = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludePublic: {e}"))?,
        );
    }
    if let Some(value) = params.get("IncludeShared") {
        input.include_shared = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeShared: {e}"))?,
        );
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
    if let Some(value) = params.get("SnapshotType") {
        input.snapshot_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBClusters.
pub fn deserialize_describe_d_b_clusters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClustersMessage, String> {
    let mut input = DescribeDBClustersMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBEngineVersions.
pub fn deserialize_describe_d_b_engine_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBEngineVersionsMessage, String> {
    let mut input = DescribeDBEngineVersionsMessage::default();
    if let Some(value) = params.get("DBParameterGroupFamily") {
        input.d_b_parameter_group_family = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("ListSupportedCharacterSets") {
        input.list_supported_character_sets = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ListSupportedCharacterSets: {e}"))?,
        );
    }
    if let Some(value) = params.get("ListSupportedTimezones") {
        input.list_supported_timezones = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ListSupportedTimezones: {e}"))?,
        );
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

/// Deserialize awsQuery request for DescribeDBInstances.
pub fn deserialize_describe_d_b_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBInstancesMessage, String> {
    let mut input = DescribeDBInstancesMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBParameterGroups.
pub fn deserialize_describe_d_b_parameter_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBParameterGroupsMessage, String> {
    let mut input = DescribeDBParameterGroupsMessage::default();
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBParameters.
pub fn deserialize_describe_d_b_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBParametersMessage, String> {
    let mut input = DescribeDBParametersMessage::default();
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    if let Some(value) = params.get("Source") {
        input.source = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBSubnetGroups.
pub fn deserialize_describe_d_b_subnet_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBSubnetGroupsMessage, String> {
    let mut input = DescribeDBSubnetGroupsMessage::default();
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEngineDefaultClusterParameters.
pub fn deserialize_describe_engine_default_cluster_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEngineDefaultClusterParametersMessage, String> {
    let mut input = DescribeEngineDefaultClusterParametersMessage::default();
    if let Some(value) = params.get("DBParameterGroupFamily") {
        input.d_b_parameter_group_family = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEngineDefaultParameters.
pub fn deserialize_describe_engine_default_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEngineDefaultParametersMessage, String> {
    let mut input = DescribeEngineDefaultParametersMessage::default();
    if let Some(value) = params.get("DBParameterGroupFamily") {
        input.d_b_parameter_group_family = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEventCategories.
pub fn deserialize_describe_event_categories_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventCategoriesMessage, String> {
    let mut input = DescribeEventCategoriesMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("SourceType") {
        input.source_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEventSubscriptions.
pub fn deserialize_describe_event_subscriptions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventSubscriptionsMessage, String> {
    let mut input = DescribeEventSubscriptionsMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "EventCategories".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.EventCategory.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.event_categories = Some(EventCategoriesList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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

/// Deserialize awsQuery request for DescribeGlobalClusters.
pub fn deserialize_describe_global_clusters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeGlobalClustersMessage, String> {
    let mut input = DescribeGlobalClustersMessage::default();
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeOrderableDBInstanceOptions.
pub fn deserialize_describe_orderable_d_b_instance_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrderableDBInstanceOptionsMessage, String> {
    let mut input = DescribeOrderableDBInstanceOptionsMessage::default();
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("LicenseModel") {
        input.license_model = Some(value.to_string());
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
    if let Some(value) = params.get("Vpc") {
        input.vpc = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Vpc: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribePendingMaintenanceActions.
pub fn deserialize_describe_pending_maintenance_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribePendingMaintenanceActionsMessage, String> {
    let mut input = DescribePendingMaintenanceActionsMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
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
    if let Some(value) = params.get("ResourceIdentifier") {
        input.resource_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeValidDBInstanceModifications.
pub fn deserialize_describe_valid_d_b_instance_modifications_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeValidDBInstanceModificationsMessage, String> {
    let mut input = DescribeValidDBInstanceModificationsMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for FailoverDBCluster.
pub fn deserialize_failover_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<FailoverDBClusterMessage, String> {
    let mut input = FailoverDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetDBInstanceIdentifier") {
        input.target_d_b_instance_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for FailoverGlobalCluster.
pub fn deserialize_failover_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<FailoverGlobalClusterMessage, String> {
    let mut input = FailoverGlobalClusterMessage::default();
    if let Some(value) = params.get("AllowDataLoss") {
        input.allow_data_loss = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowDataLoss: {e}"))?,
        );
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("Switchover") {
        input.switchover = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Switchover: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetDbClusterIdentifier") {
        input.target_db_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTagsForResource.
pub fn deserialize_list_tags_for_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceMessage, String> {
    let mut input = ListTagsForResourceMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Filter.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(FilterList { items });
        }
    }
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBCluster.
pub fn deserialize_modify_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBClusterMessage, String> {
    let mut input = ModifyDBClusterMessage::default();
    if let Some(value) = params.get("AllowMajorVersionUpgrade") {
        input.allow_major_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowMajorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(val) = deserialize_cloudwatch_logs_export_configuration_from_query(
        params,
        "CloudwatchLogsExportConfiguration",
    )? {
        input.cloudwatch_logs_export_configuration = Some(val);
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceParameterGroupName") {
        input.d_b_instance_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("NewDBClusterIdentifier") {
        input.new_d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBClusterEndpoint.
pub fn deserialize_modify_d_b_cluster_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBClusterEndpointMessage, String> {
    let mut input = ModifyDBClusterEndpointMessage::default();
    if let Some(value) = params.get("DBClusterEndpointIdentifier") {
        input.d_b_cluster_endpoint_identifier = value.to_string();
    }
    if let Some(value) = params.get("EndpointType") {
        input.endpoint_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ExcludedMembers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.excluded_members = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StaticMembers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.static_members = Some(StringList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBClusterParameterGroup.
pub fn deserialize_modify_d_b_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBClusterParameterGroupMessage, String> {
    let mut input = ModifyDBClusterParameterGroupMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Parameter.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = ParametersList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBClusterSnapshotAttribute.
pub fn deserialize_modify_d_b_cluster_snapshot_attribute_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBClusterSnapshotAttributeMessage, String> {
    let mut input = ModifyDBClusterSnapshotAttributeMessage::default();
    if let Some(value) = params.get("AttributeName") {
        input.attribute_name = value.to_string();
    }
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ValuesToAdd".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AttributeValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.values_to_add = Some(AttributeValueList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ValuesToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AttributeValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.values_to_remove = Some(AttributeValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBInstance.
pub fn deserialize_modify_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBInstanceMessage, String> {
    let mut input = ModifyDBInstanceMessage::default();
    if let Some(value) = params.get("AllocatedStorage") {
        input.allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AllocatedStorage: {e}"))?,
        );
    }
    if let Some(value) = params.get("AllowMajorVersionUpgrade") {
        input.allow_major_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowMajorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("AutoMinorVersionUpgrade") {
        input.auto_minor_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoMinorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
    }
    if let Some(val) = deserialize_cloudwatch_logs_export_configuration_from_query(
        params,
        "CloudwatchLogsExportConfiguration",
    )? {
        input.cloudwatch_logs_export_configuration = Some(val);
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBPortNumber") {
        input.d_b_port_number = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DBPortNumber: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DBSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.DBSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_security_groups = Some(DBSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("Domain") {
        input.domain = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnablePerformanceInsights") {
        input.enable_performance_insights = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnablePerformanceInsights: {e}"))?,
        );
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("Iops") {
        input.iops = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Iops: {e}"))?,
        );
    }
    if let Some(value) = params.get("LicenseModel") {
        input.license_model = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MonitoringInterval") {
        input.monitoring_interval = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MonitoringInterval: {e}"))?,
        );
    }
    if let Some(value) = params.get("MonitoringRoleArn") {
        input.monitoring_role_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NewDBInstanceIdentifier") {
        input.new_d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PromotionTier") {
        input.promotion_tier = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PromotionTier: {e}"))?,
        );
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialArn") {
        input.tde_credential_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialPassword") {
        input.tde_credential_password = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBParameterGroup.
pub fn deserialize_modify_d_b_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBParameterGroupMessage, String> {
    let mut input = ModifyDBParameterGroupMessage::default();
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Parameter.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = ParametersList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBSubnetGroup.
pub fn deserialize_modify_d_b_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBSubnetGroupMessage, String> {
    let mut input = ModifyDBSubnetGroupMessage::default();
    if let Some(value) = params.get("DBSubnetGroupDescription") {
        input.d_b_subnet_group_description = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = value.to_string();
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
    Ok(input)
}

/// Deserialize awsQuery request for ModifyEventSubscription.
pub fn deserialize_modify_event_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyEventSubscriptionMessage, String> {
    let mut input = ModifyEventSubscriptionMessage::default();
    if let Some(value) = params.get("Enabled") {
        input.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EventCategories".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.EventCategory.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.event_categories = Some(EventCategoriesList { items });
        }
    }
    if let Some(value) = params.get("SnsTopicArn") {
        input.sns_topic_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceType") {
        input.source_type = Some(value.to_string());
    }
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyGlobalCluster.
pub fn deserialize_modify_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyGlobalClusterMessage, String> {
    let mut input = ModifyGlobalClusterMessage::default();
    if let Some(value) = params.get("AllowMajorVersionUpgrade") {
        input.allow_major_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowMajorVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("NewGlobalClusterIdentifier") {
        input.new_global_cluster_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PromoteReadReplicaDBCluster.
pub fn deserialize_promote_read_replica_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PromoteReadReplicaDBClusterMessage, String> {
    let mut input = PromoteReadReplicaDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RebootDBInstance.
pub fn deserialize_reboot_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebootDBInstanceMessage, String> {
    let mut input = RebootDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("ForceFailover") {
        input.force_failover = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ForceFailover: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveFromGlobalCluster.
pub fn deserialize_remove_from_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveFromGlobalClusterMessage, String> {
    let mut input = RemoveFromGlobalClusterMessage::default();
    if let Some(value) = params.get("DbClusterIdentifier") {
        input.db_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveRoleFromDBCluster.
pub fn deserialize_remove_role_from_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveRoleFromDBClusterMessage, String> {
    let mut input = RemoveRoleFromDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("FeatureName") {
        input.feature_name = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveSourceIdentifierFromSubscription.
pub fn deserialize_remove_source_identifier_from_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveSourceIdentifierFromSubscriptionMessage, String> {
    let mut input = RemoveSourceIdentifierFromSubscriptionMessage::default();
    if let Some(value) = params.get("SourceIdentifier") {
        input.source_identifier = value.to_string();
    }
    if let Some(value) = params.get("SubscriptionName") {
        input.subscription_name = value.to_string();
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

/// Deserialize awsQuery request for ResetDBClusterParameterGroup.
pub fn deserialize_reset_d_b_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResetDBClusterParameterGroupMessage, String> {
    let mut input = ResetDBClusterParameterGroupMessage::default();
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Parameter.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(ParametersList { items });
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

/// Deserialize awsQuery request for ResetDBParameterGroup.
pub fn deserialize_reset_d_b_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResetDBParameterGroupMessage, String> {
    let mut input = ResetDBParameterGroupMessage::default();
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Parameter.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(ParametersList { items });
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

/// Deserialize awsQuery request for RestoreDBClusterFromSnapshot.
pub fn deserialize_restore_d_b_cluster_from_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBClusterFromSnapshotMessage, String> {
    let mut input = RestoreDBClusterFromSnapshotMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AvailabilityZone.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZones { items });
        }
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnableCloudwatchLogsExports".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.enable_cloudwatch_logs_exports = Some(LogTypeList { items });
        }
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = value.to_string();
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RestoreDBClusterToPointInTime.
pub fn deserialize_restore_d_b_cluster_to_point_in_time_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBClusterToPointInTimeMessage, String> {
    let mut input = RestoreDBClusterToPointInTimeMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnableCloudwatchLogsExports".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.enable_cloudwatch_logs_exports = Some(LogTypeList { items });
        }
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("RestoreToTime") {
        input.restore_to_time = Some(value.to_string());
    }
    if let Some(value) = params.get("RestoreType") {
        input.restore_type = Some(value.to_string());
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("SourceDBClusterIdentifier") {
        input.source_d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
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
    if let Some(value) = params.get("UseLatestRestorableTime") {
        input.use_latest_restorable_time = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseLatestRestorableTime: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(VpcSecurityGroupIdList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartDBCluster.
pub fn deserialize_start_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartDBClusterMessage, String> {
    let mut input = StartDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StopDBCluster.
pub fn deserialize_stop_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopDBClusterMessage, String> {
    let mut input = StopDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SwitchoverGlobalCluster.
pub fn deserialize_switchover_global_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SwitchoverGlobalClusterMessage, String> {
    let mut input = SwitchoverGlobalClusterMessage::default();
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("TargetDbClusterIdentifier") {
        input.target_db_cluster_identifier = value.to_string();
    }
    Ok(input)
}
