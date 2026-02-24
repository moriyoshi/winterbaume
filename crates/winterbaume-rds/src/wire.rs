//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rds

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

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_role_to_d_b_instance_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddRoleToDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddRoleToDBInstanceResponse>"#
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
pub fn serialize_authorize_d_b_security_group_ingress_response(
    result: &AuthorizeDBSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AuthorizeDBSecurityGroupIngressResult>{inner_xml}</AuthorizeDBSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeDBSecurityGroupIngressResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeDBSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_backtrack_d_b_cluster_response(result: &DBClusterBacktrack) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<BacktrackDBClusterResult>{inner_xml}</BacktrackDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BacktrackDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BacktrackDBClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_cancel_export_task_response(result: &ExportTask) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CancelExportTaskResult>{inner_xml}</CancelExportTaskResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CancelExportTaskResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CancelExportTaskResponse>"#
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
pub fn serialize_copy_d_b_snapshot_response(result: &CopyDBSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CopyDBSnapshotResult>{inner_xml}</CopyDBSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyDBSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyDBSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_option_group_response(result: &CopyOptionGroupResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CopyOptionGroupResult>{inner_xml}</CopyOptionGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyOptionGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyOptionGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_blue_green_deployment_response(
    result: &CreateBlueGreenDeploymentResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateBlueGreenDeploymentResult>{inner_xml}</CreateBlueGreenDeploymentResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateBlueGreenDeploymentResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateBlueGreenDeploymentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_custom_d_b_engine_version_response(
    result: &DBEngineVersion,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateCustomDBEngineVersionResult>{inner_xml}</CreateCustomDBEngineVersionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCustomDBEngineVersionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCustomDBEngineVersionResponse>"#
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
pub fn serialize_create_d_b_cluster_endpoint_response(result: &DBClusterEndpoint) -> MockResponse {
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
pub fn serialize_create_d_b_instance_read_replica_response(
    result: &CreateDBInstanceReadReplicaResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateDBInstanceReadReplicaResult>{inner_xml}</CreateDBInstanceReadReplicaResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBInstanceReadReplicaResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBInstanceReadReplicaResponse>"#
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
pub fn serialize_create_d_b_proxy_response(result: &CreateDBProxyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBProxyResult>{inner_xml}</CreateDBProxyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBProxyResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBProxyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_proxy_endpoint_response(
    result: &CreateDBProxyEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDBProxyEndpointResult>{inner_xml}</CreateDBProxyEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBProxyEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBProxyEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_security_group_response(
    result: &CreateDBSecurityGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDBSecurityGroupResult>{inner_xml}</CreateDBSecurityGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBSecurityGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_shard_group_response(result: &DBShardGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBShardGroupResult>{inner_xml}</CreateDBShardGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBShardGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBShardGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_d_b_snapshot_response(result: &CreateDBSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateDBSnapshotResult>{inner_xml}</CreateDBSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDBSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDBSnapshotResponse>"#
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
pub fn serialize_create_integration_response(result: &Integration) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateIntegrationResult>{inner_xml}</CreateIntegrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateIntegrationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_option_group_response(result: &CreateOptionGroupResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateOptionGroupResult>{inner_xml}</CreateOptionGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateOptionGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateOptionGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_tenant_database_response(
    result: &CreateTenantDatabaseResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateTenantDatabaseResult>{inner_xml}</CreateTenantDatabaseResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTenantDatabaseResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTenantDatabaseResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_blue_green_deployment_response(
    result: &DeleteBlueGreenDeploymentResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteBlueGreenDeploymentResult>{inner_xml}</DeleteBlueGreenDeploymentResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteBlueGreenDeploymentResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteBlueGreenDeploymentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_custom_d_b_engine_version_response(
    result: &DBEngineVersion,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteCustomDBEngineVersionResult>{inner_xml}</DeleteCustomDBEngineVersionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCustomDBEngineVersionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCustomDBEngineVersionResponse>"#
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
pub fn serialize_delete_d_b_cluster_automated_backup_response(
    result: &DeleteDBClusterAutomatedBackupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteDBClusterAutomatedBackupResult>{inner_xml}</DeleteDBClusterAutomatedBackupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBClusterAutomatedBackupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBClusterAutomatedBackupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_cluster_endpoint_response(result: &DBClusterEndpoint) -> MockResponse {
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

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_instance_automated_backup_response(
    result: &DeleteDBInstanceAutomatedBackupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteDBInstanceAutomatedBackupResult>{inner_xml}</DeleteDBInstanceAutomatedBackupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBInstanceAutomatedBackupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBInstanceAutomatedBackupResponse>"#
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

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_proxy_response(result: &DeleteDBProxyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDBProxyResult>{inner_xml}</DeleteDBProxyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBProxyResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBProxyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_proxy_endpoint_response(
    result: &DeleteDBProxyEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteDBProxyEndpointResult>{inner_xml}</DeleteDBProxyEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBProxyEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBProxyEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_d_b_security_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBSecurityGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_shard_group_response(result: &DBShardGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDBShardGroupResult>{inner_xml}</DeleteDBShardGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBShardGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBShardGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_d_b_snapshot_response(result: &DeleteDBSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDBSnapshotResult>{inner_xml}</DeleteDBSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDBSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDBSnapshotResponse>"#
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
pub fn serialize_delete_integration_response(result: &Integration) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteIntegrationResult>{inner_xml}</DeleteIntegrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteIntegrationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_option_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteOptionGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteOptionGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_tenant_database_response(
    result: &DeleteTenantDatabaseResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteTenantDatabaseResult>{inner_xml}</DeleteTenantDatabaseResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTenantDatabaseResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTenantDatabaseResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deregister_d_b_proxy_targets_response(
    result: &DeregisterDBProxyTargetsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeregisterDBProxyTargetsResult>{inner_xml}</DeregisterDBProxyTargetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeregisterDBProxyTargetsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeregisterDBProxyTargetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_attributes_response(
    result: &AccountAttributesMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountAttributesResult>{inner_xml}</DescribeAccountAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountAttributesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_blue_green_deployments_response(
    result: &DescribeBlueGreenDeploymentsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeBlueGreenDeploymentsResult>{inner_xml}</DescribeBlueGreenDeploymentsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeBlueGreenDeploymentsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeBlueGreenDeploymentsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_certificates_response(result: &CertificateMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeCertificatesResult>{inner_xml}</DescribeCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCertificatesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_automated_backups_response(
    result: &DBClusterAutomatedBackupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBClusterAutomatedBackupsResult>{inner_xml}</DescribeDBClusterAutomatedBackupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterAutomatedBackupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterAutomatedBackupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_cluster_backtracks_response(
    result: &DBClusterBacktrackMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBClusterBacktracksResult>{inner_xml}</DescribeDBClusterBacktracksResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBClusterBacktracksResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBClusterBacktracksResponse>"#
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
pub fn serialize_describe_d_b_instance_automated_backups_response(
    result: &DBInstanceAutomatedBackupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBInstanceAutomatedBackupsResult>{inner_xml}</DescribeDBInstanceAutomatedBackupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBInstanceAutomatedBackupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBInstanceAutomatedBackupsResponse>"#
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
pub fn serialize_describe_d_b_log_files_response(
    result: &DescribeDBLogFilesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDBLogFilesResult>{inner_xml}</DescribeDBLogFilesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBLogFilesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBLogFilesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_major_engine_versions_response(
    result: &DescribeDBMajorEngineVersionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBMajorEngineVersionsResult>{inner_xml}</DescribeDBMajorEngineVersionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBMajorEngineVersionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBMajorEngineVersionsResponse>"#
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
pub fn serialize_describe_d_b_proxies_response(result: &DescribeDBProxiesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDBProxiesResult>{inner_xml}</DescribeDBProxiesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBProxiesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBProxiesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_proxy_endpoints_response(
    result: &DescribeDBProxyEndpointsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBProxyEndpointsResult>{inner_xml}</DescribeDBProxyEndpointsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBProxyEndpointsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBProxyEndpointsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_proxy_target_groups_response(
    result: &DescribeDBProxyTargetGroupsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBProxyTargetGroupsResult>{inner_xml}</DescribeDBProxyTargetGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBProxyTargetGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBProxyTargetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_proxy_targets_response(
    result: &DescribeDBProxyTargetsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBProxyTargetsResult>{inner_xml}</DescribeDBProxyTargetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBProxyTargetsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBProxyTargetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_recommendations_response(
    result: &DBRecommendationsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBRecommendationsResult>{inner_xml}</DescribeDBRecommendationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBRecommendationsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBRecommendationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_security_groups_response(
    result: &DBSecurityGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBSecurityGroupsResult>{inner_xml}</DescribeDBSecurityGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBSecurityGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBSecurityGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_shard_groups_response(
    result: &DescribeDBShardGroupsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeDBShardGroupsResult>{inner_xml}</DescribeDBShardGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBShardGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBShardGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_snapshot_attributes_response(
    result: &DescribeDBSnapshotAttributesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBSnapshotAttributesResult>{inner_xml}</DescribeDBSnapshotAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBSnapshotAttributesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBSnapshotAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_snapshot_tenant_databases_response(
    result: &DBSnapshotTenantDatabasesMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDBSnapshotTenantDatabasesResult>{inner_xml}</DescribeDBSnapshotTenantDatabasesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBSnapshotTenantDatabasesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBSnapshotTenantDatabasesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_d_b_snapshots_response(result: &DBSnapshotMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDBSnapshotsResult>{inner_xml}</DescribeDBSnapshotsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDBSnapshotsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDBSnapshotsResponse>"#
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
pub fn serialize_describe_export_tasks_response(result: &ExportTasksMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeExportTasksResult>{inner_xml}</DescribeExportTasksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeExportTasksResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeExportTasksResponse>"#
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
pub fn serialize_describe_integrations_response(
    result: &DescribeIntegrationsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeIntegrationsResult>{inner_xml}</DescribeIntegrationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeIntegrationsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeIntegrationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_option_group_options_response(
    result: &OptionGroupOptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeOptionGroupOptionsResult>{inner_xml}</DescribeOptionGroupOptionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeOptionGroupOptionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeOptionGroupOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_option_groups_response(result: &OptionGroups) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeOptionGroupsResult>{inner_xml}</DescribeOptionGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeOptionGroupsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeOptionGroupsResponse>"#
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
pub fn serialize_describe_reserved_d_b_instances_response(
    result: &ReservedDBInstanceMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeReservedDBInstancesResult>{inner_xml}</DescribeReservedDBInstancesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedDBInstancesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedDBInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_d_b_instances_offerings_response(
    result: &ReservedDBInstancesOfferingMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeReservedDBInstancesOfferingsResult>{inner_xml}</DescribeReservedDBInstancesOfferingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedDBInstancesOfferingsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedDBInstancesOfferingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_source_regions_response(result: &SourceRegionMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeSourceRegionsResult>{inner_xml}</DescribeSourceRegionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeSourceRegionsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeSourceRegionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_tenant_databases_response(
    result: &TenantDatabasesMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTenantDatabasesResult>{inner_xml}</DescribeTenantDatabasesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTenantDatabasesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTenantDatabasesResponse>"#
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
pub fn serialize_disable_http_endpoint_response(
    result: &DisableHttpEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DisableHttpEndpointResult>{inner_xml}</DisableHttpEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableHttpEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableHttpEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_download_d_b_log_file_portion_response(
    result: &DownloadDBLogFilePortionDetails,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DownloadDBLogFilePortionResult>{inner_xml}</DownloadDBLogFilePortionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DownloadDBLogFilePortionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DownloadDBLogFilePortionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_http_endpoint_response(
    result: &EnableHttpEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<EnableHttpEndpointResult>{inner_xml}</EnableHttpEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableHttpEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableHttpEndpointResponse>"#
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
pub fn serialize_modify_activity_stream_response(
    result: &ModifyActivityStreamResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyActivityStreamResult>{inner_xml}</ModifyActivityStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyActivityStreamResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyActivityStreamResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_certificates_response(result: &ModifyCertificatesResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyCertificatesResult>{inner_xml}</ModifyCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCertificatesResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_current_d_b_cluster_capacity_response(
    result: &DBClusterCapacityInfo,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyCurrentDBClusterCapacityResult>{inner_xml}</ModifyCurrentDBClusterCapacityResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCurrentDBClusterCapacityResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCurrentDBClusterCapacityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_custom_d_b_engine_version_response(
    result: &DBEngineVersion,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyCustomDBEngineVersionResult>{inner_xml}</ModifyCustomDBEngineVersionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCustomDBEngineVersionResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCustomDBEngineVersionResponse>"#
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
pub fn serialize_modify_d_b_cluster_endpoint_response(result: &DBClusterEndpoint) -> MockResponse {
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
pub fn serialize_modify_d_b_proxy_response(result: &ModifyDBProxyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBProxyResult>{inner_xml}</ModifyDBProxyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBProxyResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBProxyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_proxy_endpoint_response(
    result: &ModifyDBProxyEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBProxyEndpointResult>{inner_xml}</ModifyDBProxyEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBProxyEndpointResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBProxyEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_proxy_target_group_response(
    result: &ModifyDBProxyTargetGroupResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBProxyTargetGroupResult>{inner_xml}</ModifyDBProxyTargetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBProxyTargetGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBProxyTargetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_recommendation_response(
    result: &DBRecommendationMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBRecommendationResult>{inner_xml}</ModifyDBRecommendationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBRecommendationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBRecommendationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_shard_group_response(result: &DBShardGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBShardGroupResult>{inner_xml}</ModifyDBShardGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBShardGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBShardGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_snapshot_response(result: &ModifyDBSnapshotResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyDBSnapshotResult>{inner_xml}</ModifyDBSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_d_b_snapshot_attribute_response(
    result: &ModifyDBSnapshotAttributeResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyDBSnapshotAttributeResult>{inner_xml}</ModifyDBSnapshotAttributeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyDBSnapshotAttributeResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyDBSnapshotAttributeResponse>"#
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
pub fn serialize_modify_integration_response(result: &Integration) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyIntegrationResult>{inner_xml}</ModifyIntegrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyIntegrationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_option_group_response(result: &ModifyOptionGroupResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyOptionGroupResult>{inner_xml}</ModifyOptionGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyOptionGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyOptionGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_tenant_database_response(
    result: &ModifyTenantDatabaseResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyTenantDatabaseResult>{inner_xml}</ModifyTenantDatabaseResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyTenantDatabaseResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyTenantDatabaseResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_promote_read_replica_response(result: &PromoteReadReplicaResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PromoteReadReplicaResult>{inner_xml}</PromoteReadReplicaResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PromoteReadReplicaResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PromoteReadReplicaResponse>"#
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
pub fn serialize_purchase_reserved_d_b_instances_offering_response(
    result: &PurchaseReservedDBInstancesOfferingResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<PurchaseReservedDBInstancesOfferingResult>{inner_xml}</PurchaseReservedDBInstancesOfferingResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PurchaseReservedDBInstancesOfferingResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PurchaseReservedDBInstancesOfferingResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reboot_d_b_cluster_response(result: &RebootDBClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RebootDBClusterResult>{inner_xml}</RebootDBClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebootDBClusterResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebootDBClusterResponse>"#
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
pub fn serialize_reboot_d_b_shard_group_response(result: &DBShardGroup) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RebootDBShardGroupResult>{inner_xml}</RebootDBShardGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebootDBShardGroupResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebootDBShardGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_d_b_proxy_targets_response(
    result: &RegisterDBProxyTargetsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RegisterDBProxyTargetsResult>{inner_xml}</RegisterDBProxyTargetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterDBProxyTargetsResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterDBProxyTargetsResponse>"#
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

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_role_from_d_b_instance_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveRoleFromDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveRoleFromDBInstanceResponse>"#
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
pub fn serialize_restore_d_b_cluster_from_s3_response(
    result: &RestoreDBClusterFromS3Result,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RestoreDBClusterFromS3Result>{inner_xml}</RestoreDBClusterFromS3Result>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBClusterFromS3Response xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBClusterFromS3Response>"#
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
pub fn serialize_restore_d_b_instance_from_d_b_snapshot_response(
    result: &RestoreDBInstanceFromDBSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RestoreDBInstanceFromDBSnapshotResult>{inner_xml}</RestoreDBInstanceFromDBSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBInstanceFromDBSnapshotResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBInstanceFromDBSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_d_b_instance_from_s3_response(
    result: &RestoreDBInstanceFromS3Result,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RestoreDBInstanceFromS3Result>{inner_xml}</RestoreDBInstanceFromS3Result>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBInstanceFromS3Response xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBInstanceFromS3Response>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_d_b_instance_to_point_in_time_response(
    result: &RestoreDBInstanceToPointInTimeResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RestoreDBInstanceToPointInTimeResult>{inner_xml}</RestoreDBInstanceToPointInTimeResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreDBInstanceToPointInTimeResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreDBInstanceToPointInTimeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_revoke_d_b_security_group_ingress_response(
    result: &RevokeDBSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RevokeDBSecurityGroupIngressResult>{inner_xml}</RevokeDBSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RevokeDBSecurityGroupIngressResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RevokeDBSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_activity_stream_response(
    result: &StartActivityStreamResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartActivityStreamResult>{inner_xml}</StartActivityStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartActivityStreamResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartActivityStreamResponse>"#
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
pub fn serialize_start_d_b_instance_response(result: &StartDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartDBInstanceResult>{inner_xml}</StartDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_d_b_instance_automated_backups_replication_response(
    result: &StartDBInstanceAutomatedBackupsReplicationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<StartDBInstanceAutomatedBackupsReplicationResult>{inner_xml}</StartDBInstanceAutomatedBackupsReplicationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartDBInstanceAutomatedBackupsReplicationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartDBInstanceAutomatedBackupsReplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_export_task_response(result: &ExportTask) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartExportTaskResult>{inner_xml}</StartExportTaskResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartExportTaskResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartExportTaskResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_stop_activity_stream_response(
    result: &StopActivityStreamResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StopActivityStreamResult>{inner_xml}</StopActivityStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopActivityStreamResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopActivityStreamResponse>"#
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
pub fn serialize_stop_d_b_instance_response(result: &StopDBInstanceResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StopDBInstanceResult>{inner_xml}</StopDBInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopDBInstanceResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopDBInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_stop_d_b_instance_automated_backups_replication_response(
    result: &StopDBInstanceAutomatedBackupsReplicationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<StopDBInstanceAutomatedBackupsReplicationResult>{inner_xml}</StopDBInstanceAutomatedBackupsReplicationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopDBInstanceAutomatedBackupsReplicationResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopDBInstanceAutomatedBackupsReplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_switchover_blue_green_deployment_response(
    result: &SwitchoverBlueGreenDeploymentResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SwitchoverBlueGreenDeploymentResult>{inner_xml}</SwitchoverBlueGreenDeploymentResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SwitchoverBlueGreenDeploymentResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SwitchoverBlueGreenDeploymentResponse>"#
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

/// Serialize response for awsQuery protocol.
pub fn serialize_switchover_read_replica_response(
    result: &SwitchoverReadReplicaResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SwitchoverReadReplicaResult>{inner_xml}</SwitchoverReadReplicaResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SwitchoverReadReplicaResponse xmlns="http://rds.amazonaws.com/doc/2014-10-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SwitchoverReadReplicaResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_modify_additional_storage_volume_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ModifyAdditionalStorageVolume>, String> {
    let mut item = ModifyAdditionalStorageVolume::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AllocatedStorage")) {
        item.allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AllocatedStorage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IOPS")) {
        item.i_o_p_s = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse IOPS: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxAllocatedStorage")) {
        item.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SetForDelete")) {
        item.set_for_delete = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SetForDelete: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StorageThroughput")) {
        item.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StorageType")) {
        item.storage_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.VolumeName")) {
        item.volume_name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TagSpecification>, String> {
    let mut item = TagSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ResourceType")) {
        item.resource_type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Tags");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.tags = Some(TagList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_option_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<OptionConfiguration>, String> {
    let mut item = OptionConfiguration::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.DBSecurityGroupMemberships");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.DBSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.d_b_security_group_memberships =
                Some(DBSecurityGroupNameList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.OptionName")) {
        item.option_name = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.OptionSettings");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.OptionSetting.{i}");
            match deserialize_option_setting_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.option_settings = Some(OptionSettingsList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.OptionVersion")) {
        item.option_version = Some(value.to_string());
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
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.VpcSecurityGroupMemberships");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.VpcSecurityGroupId.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.vpc_security_group_memberships = Some(VpcSecurityGroupIdList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_user_auth_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<UserAuthConfig>, String> {
    let mut item = UserAuthConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AuthScheme")) {
        item.auth_scheme = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ClientPasswordAuthType")) {
        item.client_password_auth_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Description")) {
        item.description = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IAMAuth")) {
        item.i_a_m_auth = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SecretArn")) {
        item.secret_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UserName")) {
        item.user_name = Some(value.to_string());
        found = true;
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
    if let Some(value) = params.get(&format!("{prefix}.SecondsUntilAutoPause")) {
        item.seconds_until_auto_pause = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SecondsUntilAutoPause: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
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

fn deserialize_scaling_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScalingConfiguration>, String> {
    let mut item = ScalingConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AutoPause")) {
        item.auto_pause = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoPause: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxCapacity")) {
        item.max_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxCapacity: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinCapacity")) {
        item.min_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinCapacity: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SecondsBeforeTimeout")) {
        item.seconds_before_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SecondsBeforeTimeout: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SecondsUntilAutoPause")) {
        item.seconds_until_auto_pause = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SecondsUntilAutoPause: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TimeoutAction")) {
        item.timeout_action = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rds_custom_cluster_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RdsCustomClusterConfiguration>, String> {
    let mut item = RdsCustomClusterConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.InterconnectSubnetId")) {
        item.interconnect_subnet_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReplicaMode")) {
        item.replica_mode = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TransitGatewayMulticastDomainId")) {
        item.transit_gateway_multicast_domain_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_connection_pool_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConnectionPoolConfiguration>, String> {
    let mut item = ConnectionPoolConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ConnectionBorrowTimeout")) {
        item.connection_borrow_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ConnectionBorrowTimeout: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.InitQuery")) {
        item.init_query = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxConnectionsPercent")) {
        item.max_connections_percent = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxConnectionsPercent: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxIdleConnectionsPercent")) {
        item.max_idle_connections_percent = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxIdleConnectionsPercent: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.SessionPinningFilters");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.session_pinning_filters = Some(StringList { items: sub_items });
            found = true;
        }
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

fn deserialize_processor_feature_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ProcessorFeature>, String> {
    let mut item = ProcessorFeature::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_additional_storage_volume_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AdditionalStorageVolume>, String> {
    let mut item = AdditionalStorageVolume::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AllocatedStorage")) {
        item.allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AllocatedStorage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IOPS")) {
        item.i_o_p_s = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse IOPS: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxAllocatedStorage")) {
        item.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StorageThroughput")) {
        item.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StorageType")) {
        item.storage_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.VolumeName")) {
        item.volume_name = value.to_string();
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
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.SupportedEngineModes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.supported_engine_modes = Some(EngineModeList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_recommended_action_update_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RecommendedActionUpdate>, String> {
    let mut item = RecommendedActionUpdate::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ActionId")) {
        item.action_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Status")) {
        item.status = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_option_setting_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<OptionSetting>, String> {
    let mut item = OptionSetting::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AllowedValues")) {
        item.allowed_values = Some(value.to_string());
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
    if let Some(value) = params.get(&format!("{prefix}.DefaultValue")) {
        item.default_value = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Description")) {
        item.description = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IsCollection")) {
        item.is_collection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IsCollection: {e}"))?,
        );
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
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
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

/// Deserialize awsQuery request for AddRoleToDBInstance.
pub fn deserialize_add_role_to_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddRoleToDBInstanceMessage, String> {
    let mut input = AddRoleToDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("FeatureName") {
        input.feature_name = value.to_string();
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

/// Deserialize awsQuery request for AuthorizeDBSecurityGroupIngress.
pub fn deserialize_authorize_d_b_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeDBSecurityGroupIngressMessage, String> {
    let mut input = AuthorizeDBSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CIDRIP") {
        input.c_i_d_r_i_p = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSecurityGroupName") {
        input.d_b_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupId") {
        input.e_c2_security_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for BacktrackDBCluster.
pub fn deserialize_backtrack_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BacktrackDBClusterMessage, String> {
    let mut input = BacktrackDBClusterMessage::default();
    if let Some(value) = params.get("BacktrackTo") {
        input.backtrack_to = value.to_string();
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("Force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Force: {e}"))?,
        );
    }
    if let Some(value) = params.get("UseEarliestTimeOnPointInTimeUnavailable") {
        input.use_earliest_time_on_point_in_time_unavailable =
            Some(value.parse::<bool>().map_err(|e| {
                format!("failed to parse UseEarliestTimeOnPointInTimeUnavailable: {e}")
            })?);
    }
    Ok(input)
}

/// Deserialize awsQuery request for CancelExportTask.
pub fn deserialize_cancel_export_task_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CancelExportTaskMessage, String> {
    let mut input = CancelExportTaskMessage::default();
    if let Some(value) = params.get("ExportTaskIdentifier") {
        input.export_task_identifier = value.to_string();
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

/// Deserialize awsQuery request for CopyDBSnapshot.
pub fn deserialize_copy_d_b_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyDBSnapshotMessage, String> {
    let mut input = CopyDBSnapshotMessage::default();
    if let Some(value) = params.get("CopyOptionGroup") {
        input.copy_option_group = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyOptionGroup: {e}"))?,
        );
    }
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
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PreSignedUrl") {
        input.pre_signed_url = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotAvailabilityZone") {
        input.snapshot_availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotTarget") {
        input.snapshot_target = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBSnapshotIdentifier") {
        input.source_d_b_snapshot_identifier = value.to_string();
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
    if let Some(value) = params.get("TargetCustomAvailabilityZone") {
        input.target_custom_availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetDBSnapshotIdentifier") {
        input.target_d_b_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyOptionGroup.
pub fn deserialize_copy_option_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyOptionGroupMessage, String> {
    let mut input = CopyOptionGroupMessage::default();
    if let Some(value) = params.get("SourceOptionGroupIdentifier") {
        input.source_option_group_identifier = value.to_string();
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
    if let Some(value) = params.get("TargetOptionGroupDescription") {
        input.target_option_group_description = value.to_string();
    }
    if let Some(value) = params.get("TargetOptionGroupIdentifier") {
        input.target_option_group_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateBlueGreenDeployment.
pub fn deserialize_create_blue_green_deployment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateBlueGreenDeploymentRequest, String> {
    let mut input = CreateBlueGreenDeploymentRequest::default();
    if let Some(value) = params.get("BlueGreenDeploymentName") {
        input.blue_green_deployment_name = value.to_string();
    }
    if let Some(value) = params.get("Source") {
        input.source = value.to_string();
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
    if let Some(value) = params.get("TargetAllocatedStorage") {
        input.target_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TargetAllocatedStorage: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetDBClusterParameterGroupName") {
        input.target_d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetDBInstanceClass") {
        input.target_d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetDBParameterGroupName") {
        input.target_d_b_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetEngineVersion") {
        input.target_engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetIops") {
        input.target_iops = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TargetIops: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetStorageThroughput") {
        input.target_storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TargetStorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetStorageType") {
        input.target_storage_type = Some(value.to_string());
    }
    if let Some(value) = params.get("UpgradeTargetStorageConfig") {
        input.upgrade_target_storage_config = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UpgradeTargetStorageConfig: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCustomDBEngineVersion.
pub fn deserialize_create_custom_d_b_engine_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomDBEngineVersionMessage, String> {
    let mut input = CreateCustomDBEngineVersionMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "DatabaseInstallationFiles".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.database_installation_files = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DatabaseInstallationFilesS3BucketName") {
        input.database_installation_files_s3_bucket_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseInstallationFilesS3Prefix") {
        input.database_installation_files_s3_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = value.to_string();
    }
    if let Some(value) = params.get("ImageId") {
        input.image_id = Some(value.to_string());
    }
    if let Some(value) = params.get("KMSKeyId") {
        input.k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Manifest") {
        input.manifest = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceCustomDbEngineVersionIdentifier") {
        input.source_custom_db_engine_version_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("UseAwsProvidedLatestImage") {
        input.use_aws_provided_latest_image = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseAwsProvidedLatestImage: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBCluster.
pub fn deserialize_create_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBClusterMessage, String> {
    let mut input = CreateDBClusterMessage::default();
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
    if let Some(value) = params.get("BacktrackWindow") {
        input.backtrack_window = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse BacktrackWindow: {e}"))?,
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
    if let Some(value) = params.get("CharacterSetName") {
        input.character_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterScalabilityType") {
        input.cluster_scalability_type = Some(value.to_string());
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
    if let Some(value) = params.get("DBClusterInstanceClass") {
        input.d_b_cluster_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSystemId") {
        input.d_b_system_id = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
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
    if let Some(value) = params.get("EnableGlobalWriteForwarding") {
        input.enable_global_write_forwarding = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableGlobalWriteForwarding: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableHttpEndpoint") {
        input.enable_http_endpoint = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableHttpEndpoint: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableLimitlessDatabase") {
        input.enable_limitless_database = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableLimitlessDatabase: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableLocalWriteForwarding") {
        input.enable_local_write_forwarding = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableLocalWriteForwarding: {e}"))?,
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineMode") {
        input.engine_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("GlobalClusterIdentifier") {
        input.global_cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserAuthenticationType") {
        input.master_user_authentication_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(val) = deserialize_rds_custom_cluster_configuration_from_query(
        params,
        "RdsCustomClusterConfiguration",
    )? {
        input.rds_custom_cluster_configuration = Some(val);
    }
    if let Some(value) = params.get("ReplicationSourceIdentifier") {
        input.replication_source_identifier = Some(value.to_string());
    }
    if let Some(val) = deserialize_scaling_configuration_from_query(params, "ScalingConfiguration")?
    {
        input.scaling_configuration = Some(val);
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
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(AdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("BackupTarget") {
        input.backup_target = Some(value.to_string());
    }
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("CustomIamInstanceProfile") {
        input.custom_iam_instance_profile = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("DBSystemId") {
        input.d_b_system_id = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
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
    if let Some(value) = params.get("DomainAuthSecretArn") {
        input.domain_auth_secret_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DomainDnsIps".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.domain_dns_ips = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DomainFqdn") {
        input.domain_fqdn = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainOu") {
        input.domain_ou = Some(value.to_string());
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
    if let Some(value) = params.get("EnableCustomerOwnedIp") {
        input.enable_customer_owned_ip = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCustomerOwnedIp: {e}"))?,
        );
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserAuthenticationType") {
        input.master_user_authentication_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxAllocatedStorage") {
        input.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
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
    if let Some(value) = params.get("MultiTenant") {
        input.multi_tenant = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiTenant: {e}"))?,
        );
    }
    if let Some(value) = params.get("NcharCharacterSetName") {
        input.nchar_character_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
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
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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

/// Deserialize awsQuery request for CreateDBInstanceReadReplica.
pub fn deserialize_create_d_b_instance_read_replica_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBInstanceReadReplicaMessage, String> {
    let mut input = CreateDBInstanceReadReplicaMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(AdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("BackupTarget") {
        input.backup_target = Some(value.to_string());
    }
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("CustomIamInstanceProfile") {
        input.custom_iam_instance_profile = Some(value.to_string());
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
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
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
    if let Some(value) = params.get("DomainAuthSecretArn") {
        input.domain_auth_secret_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DomainDnsIps".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.domain_dns_ips = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DomainFqdn") {
        input.domain_fqdn = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainOu") {
        input.domain_ou = Some(value.to_string());
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
    if let Some(value) = params.get("EnableCustomerOwnedIp") {
        input.enable_customer_owned_ip = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCustomerOwnedIp: {e}"))?,
        );
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
    if let Some(value) = params.get("MaxAllocatedStorage") {
        input.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReplicaMode") {
        input.replica_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBClusterIdentifier") {
        input.source_d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBInstanceIdentifier") {
        input.source_d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("UpgradeStorageConfig") {
        input.upgrade_storage_config = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UpgradeStorageConfig: {e}"))?,
        );
    }
    if let Some(value) = params.get("UseDefaultProcessorFeatures") {
        input.use_default_processor_features = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseDefaultProcessorFeatures: {e}"))?,
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

/// Deserialize awsQuery request for CreateDBProxy.
pub fn deserialize_create_d_b_proxy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBProxyRequest, String> {
    let mut input = CreateDBProxyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Auth".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_user_auth_config_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.auth = Some(UserAuthConfigList { items });
        }
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("DebugLogging") {
        input.debug_logging = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DebugLogging: {e}"))?,
        );
    }
    if let Some(value) = params.get("DefaultAuthScheme") {
        input.default_auth_scheme = Some(value.to_string());
    }
    if let Some(value) = params.get("EndpointNetworkType") {
        input.endpoint_network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineFamily") {
        input.engine_family = value.to_string();
    }
    if let Some(value) = params.get("IdleClientTimeout") {
        input.idle_client_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse IdleClientTimeout: {e}"))?,
        );
    }
    if let Some(value) = params.get("RequireTLS") {
        input.require_t_l_s = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireTLS: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
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
    if let Some(value) = params.get("TargetConnectionNetworkType") {
        input.target_connection_network_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_subnet_ids = StringList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBProxyEndpoint.
pub fn deserialize_create_d_b_proxy_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBProxyEndpointRequest, String> {
    let mut input = CreateDBProxyEndpointRequest::default();
    if let Some(value) = params.get("DBProxyEndpointName") {
        input.d_b_proxy_endpoint_name = value.to_string();
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("EndpointNetworkType") {
        input.endpoint_network_type = Some(value.to_string());
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
    if let Some(value) = params.get("TargetRole") {
        input.target_role = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_subnet_ids = StringList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDBSecurityGroup.
pub fn deserialize_create_d_b_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBSecurityGroupMessage, String> {
    let mut input = CreateDBSecurityGroupMessage::default();
    if let Some(value) = params.get("DBSecurityGroupDescription") {
        input.d_b_security_group_description = value.to_string();
    }
    if let Some(value) = params.get("DBSecurityGroupName") {
        input.d_b_security_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateDBShardGroup.
pub fn deserialize_create_d_b_shard_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBShardGroupMessage, String> {
    let mut input = CreateDBShardGroupMessage::default();
    if let Some(value) = params.get("ComputeRedundancy") {
        input.compute_redundancy = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ComputeRedundancy: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBShardGroupIdentifier") {
        input.d_b_shard_group_identifier = value.to_string();
    }
    if let Some(value) = params.get("MaxACU") {
        input.max_a_c_u = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse MaxACU: {e}"))?;
    }
    if let Some(value) = params.get("MinACU") {
        input.min_a_c_u = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MinACU: {e}"))?,
        );
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
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

/// Deserialize awsQuery request for CreateDBSnapshot.
pub fn deserialize_create_d_b_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDBSnapshotMessage, String> {
    let mut input = CreateDBSnapshotMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = value.to_string();
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
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

/// Deserialize awsQuery request for CreateIntegration.
pub fn deserialize_create_integration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateIntegrationMessage, String> {
    let mut input = CreateIntegrationMessage::default();
    if let Some(value) = params.get("DataFilter") {
        input.data_filter = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("IntegrationName") {
        input.integration_name = value.to_string();
    }
    if let Some(value) = params.get("KMSKeyId") {
        input.k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = value.to_string();
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
    if let Some(value) = params.get("TargetArn") {
        input.target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateOptionGroup.
pub fn deserialize_create_option_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateOptionGroupMessage, String> {
    let mut input = CreateOptionGroupMessage::default();
    if let Some(value) = params.get("EngineName") {
        input.engine_name = value.to_string();
    }
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = value.to_string();
    }
    if let Some(value) = params.get("OptionGroupDescription") {
        input.option_group_description = value.to_string();
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateTenantDatabase.
pub fn deserialize_create_tenant_database_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTenantDatabaseMessage, String> {
    let mut input = CreateTenantDatabaseMessage::default();
    if let Some(value) = params.get("CharacterSetName") {
        input.character_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = value.to_string();
    }
    if let Some(value) = params.get("NcharCharacterSetName") {
        input.nchar_character_set_name = Some(value.to_string());
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
    if let Some(value) = params.get("TenantDBName") {
        input.tenant_d_b_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteBlueGreenDeployment.
pub fn deserialize_delete_blue_green_deployment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteBlueGreenDeploymentRequest, String> {
    let mut input = DeleteBlueGreenDeploymentRequest::default();
    if let Some(value) = params.get("BlueGreenDeploymentIdentifier") {
        input.blue_green_deployment_identifier = value.to_string();
    }
    if let Some(value) = params.get("DeleteTarget") {
        input.delete_target = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteTarget: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCustomDBEngineVersion.
pub fn deserialize_delete_custom_d_b_engine_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomDBEngineVersionMessage, String> {
    let mut input = DeleteCustomDBEngineVersionMessage::default();
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = value.to_string();
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
    if let Some(value) = params.get("DeleteAutomatedBackups") {
        input.delete_automated_backups = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteAutomatedBackups: {e}"))?,
        );
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

/// Deserialize awsQuery request for DeleteDBClusterAutomatedBackup.
pub fn deserialize_delete_d_b_cluster_automated_backup_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBClusterAutomatedBackupMessage, String> {
    let mut input = DeleteDBClusterAutomatedBackupMessage::default();
    if let Some(value) = params.get("DbClusterResourceId") {
        input.db_cluster_resource_id = value.to_string();
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
    if let Some(value) = params.get("DeleteAutomatedBackups") {
        input.delete_automated_backups = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteAutomatedBackups: {e}"))?,
        );
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

/// Deserialize awsQuery request for DeleteDBInstanceAutomatedBackup.
pub fn deserialize_delete_d_b_instance_automated_backup_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBInstanceAutomatedBackupMessage, String> {
    let mut input = DeleteDBInstanceAutomatedBackupMessage::default();
    if let Some(value) = params.get("DBInstanceAutomatedBackupsArn") {
        input.d_b_instance_automated_backups_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("DbiResourceId") {
        input.dbi_resource_id = Some(value.to_string());
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

/// Deserialize awsQuery request for DeleteDBProxy.
pub fn deserialize_delete_d_b_proxy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBProxyRequest, String> {
    let mut input = DeleteDBProxyRequest::default();
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBProxyEndpoint.
pub fn deserialize_delete_d_b_proxy_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBProxyEndpointRequest, String> {
    let mut input = DeleteDBProxyEndpointRequest::default();
    if let Some(value) = params.get("DBProxyEndpointName") {
        input.d_b_proxy_endpoint_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBSecurityGroup.
pub fn deserialize_delete_d_b_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBSecurityGroupMessage, String> {
    let mut input = DeleteDBSecurityGroupMessage::default();
    if let Some(value) = params.get("DBSecurityGroupName") {
        input.d_b_security_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBShardGroup.
pub fn deserialize_delete_d_b_shard_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBShardGroupMessage, String> {
    let mut input = DeleteDBShardGroupMessage::default();
    if let Some(value) = params.get("DBShardGroupIdentifier") {
        input.d_b_shard_group_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDBSnapshot.
pub fn deserialize_delete_d_b_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDBSnapshotMessage, String> {
    let mut input = DeleteDBSnapshotMessage::default();
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = value.to_string();
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

/// Deserialize awsQuery request for DeleteIntegration.
pub fn deserialize_delete_integration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntegrationMessage, String> {
    let mut input = DeleteIntegrationMessage::default();
    if let Some(value) = params.get("IntegrationIdentifier") {
        input.integration_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteOptionGroup.
pub fn deserialize_delete_option_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteOptionGroupMessage, String> {
    let mut input = DeleteOptionGroupMessage::default();
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTenantDatabase.
pub fn deserialize_delete_tenant_database_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTenantDatabaseMessage, String> {
    let mut input = DeleteTenantDatabaseMessage::default();
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
    if let Some(value) = params.get("TenantDBName") {
        input.tenant_d_b_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeregisterDBProxyTargets.
pub fn deserialize_deregister_d_b_proxy_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeregisterDBProxyTargetsRequest, String> {
    let mut input = DeregisterDBProxyTargetsRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "DBClusterIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_cluster_identifiers = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DBInstanceIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_instance_identifiers = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("TargetGroupName") {
        input.target_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAccountAttributes.
pub fn deserialize_describe_account_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountAttributesMessage, String> {
    let input = DescribeAccountAttributesMessage {};
    Ok(input)
}

/// Deserialize awsQuery request for DescribeBlueGreenDeployments.
pub fn deserialize_describe_blue_green_deployments_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeBlueGreenDeploymentsRequest, String> {
    let mut input = DescribeBlueGreenDeploymentsRequest::default();
    if let Some(value) = params.get("BlueGreenDeploymentIdentifier") {
        input.blue_green_deployment_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeCertificates.
pub fn deserialize_describe_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCertificatesMessage, String> {
    let mut input = DescribeCertificatesMessage::default();
    if let Some(value) = params.get("CertificateIdentifier") {
        input.certificate_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBClusterAutomatedBackups.
pub fn deserialize_describe_d_b_cluster_automated_backups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterAutomatedBackupsMessage, String> {
    let mut input = DescribeDBClusterAutomatedBackupsMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DbClusterResourceId") {
        input.db_cluster_resource_id = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBClusterBacktracks.
pub fn deserialize_describe_d_b_cluster_backtracks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBClusterBacktracksMessage, String> {
    let mut input = DescribeDBClusterBacktracksMessage::default();
    if let Some(value) = params.get("BacktrackIdentifier") {
        input.backtrack_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
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
    if let Some(value) = params.get("DbClusterResourceId") {
        input.db_cluster_resource_id = Some(value.to_string());
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
    if let Some(value) = params.get("IncludeAll") {
        input.include_all = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeAll: {e}"))?,
        );
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

/// Deserialize awsQuery request for DescribeDBInstanceAutomatedBackups.
pub fn deserialize_describe_d_b_instance_automated_backups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBInstanceAutomatedBackupsMessage, String> {
    let mut input = DescribeDBInstanceAutomatedBackupsMessage::default();
    if let Some(value) = params.get("DBInstanceAutomatedBackupsArn") {
        input.d_b_instance_automated_backups_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DbiResourceId") {
        input.dbi_resource_id = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBLogFiles.
pub fn deserialize_describe_d_b_log_files_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBLogFilesMessage, String> {
    let mut input = DescribeDBLogFilesMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("FileLastWritten") {
        input.file_last_written = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse FileLastWritten: {e}"))?,
        );
    }
    if let Some(value) = params.get("FileSize") {
        input.file_size = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse FileSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("FilenameContains") {
        input.filename_contains = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBMajorEngineVersions.
pub fn deserialize_describe_d_b_major_engine_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBMajorEngineVersionsRequest, String> {
    let mut input = DescribeDBMajorEngineVersionsRequest::default();
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBProxies.
pub fn deserialize_describe_d_b_proxies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBProxiesRequest, String> {
    let mut input = DescribeDBProxiesRequest::default();
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBProxyEndpoints.
pub fn deserialize_describe_d_b_proxy_endpoints_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBProxyEndpointsRequest, String> {
    let mut input = DescribeDBProxyEndpointsRequest::default();
    if let Some(value) = params.get("DBProxyEndpointName") {
        input.d_b_proxy_endpoint_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBProxyTargetGroups.
pub fn deserialize_describe_d_b_proxy_target_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBProxyTargetGroupsRequest, String> {
    let mut input = DescribeDBProxyTargetGroupsRequest::default();
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
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
    if let Some(value) = params.get("TargetGroupName") {
        input.target_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBProxyTargets.
pub fn deserialize_describe_d_b_proxy_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBProxyTargetsRequest, String> {
    let mut input = DescribeDBProxyTargetsRequest::default();
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
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
    if let Some(value) = params.get("TargetGroupName") {
        input.target_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBRecommendations.
pub fn deserialize_describe_d_b_recommendations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBRecommendationsMessage, String> {
    let mut input = DescribeDBRecommendationsMessage::default();
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
    if let Some(value) = params.get("LastUpdatedAfter") {
        input.last_updated_after = Some(value.to_string());
    }
    if let Some(value) = params.get("LastUpdatedBefore") {
        input.last_updated_before = Some(value.to_string());
    }
    if let Some(value) = params.get("Locale") {
        input.locale = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBSecurityGroups.
pub fn deserialize_describe_d_b_security_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBSecurityGroupsMessage, String> {
    let mut input = DescribeDBSecurityGroupsMessage::default();
    if let Some(value) = params.get("DBSecurityGroupName") {
        input.d_b_security_group_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBShardGroups.
pub fn deserialize_describe_d_b_shard_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBShardGroupsMessage, String> {
    let mut input = DescribeDBShardGroupsMessage::default();
    if let Some(value) = params.get("DBShardGroupIdentifier") {
        input.d_b_shard_group_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDBSnapshotAttributes.
pub fn deserialize_describe_d_b_snapshot_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBSnapshotAttributesMessage, String> {
    let mut input = DescribeDBSnapshotAttributesMessage::default();
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBSnapshotTenantDatabases.
pub fn deserialize_describe_d_b_snapshot_tenant_databases_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBSnapshotTenantDatabasesMessage, String> {
    let mut input = DescribeDBSnapshotTenantDatabasesMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DbiResourceId") {
        input.dbi_resource_id = Some(value.to_string());
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
    if let Some(value) = params.get("SnapshotType") {
        input.snapshot_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDBSnapshots.
pub fn deserialize_describe_d_b_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDBSnapshotsMessage, String> {
    let mut input = DescribeDBSnapshotsMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DbiResourceId") {
        input.dbi_resource_id = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeExportTasks.
pub fn deserialize_describe_export_tasks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeExportTasksMessage, String> {
    let mut input = DescribeExportTasksMessage::default();
    if let Some(value) = params.get("ExportTaskIdentifier") {
        input.export_task_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceType") {
        input.source_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeGlobalClusters.
pub fn deserialize_describe_global_clusters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeGlobalClustersMessage, String> {
    let mut input = DescribeGlobalClustersMessage::default();
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

/// Deserialize awsQuery request for DescribeIntegrations.
pub fn deserialize_describe_integrations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeIntegrationsMessage, String> {
    let mut input = DescribeIntegrationsMessage::default();
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
    if let Some(value) = params.get("IntegrationIdentifier") {
        input.integration_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeOptionGroupOptions.
pub fn deserialize_describe_option_group_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOptionGroupOptionsMessage, String> {
    let mut input = DescribeOptionGroupOptionsMessage::default();
    if let Some(value) = params.get("EngineName") {
        input.engine_name = value.to_string();
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
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeOptionGroups.
pub fn deserialize_describe_option_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOptionGroupsMessage, String> {
    let mut input = DescribeOptionGroupsMessage::default();
    if let Some(value) = params.get("EngineName") {
        input.engine_name = Some(value.to_string());
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
    if let Some(value) = params.get("MajorEngineVersion") {
        input.major_engine_version = Some(value.to_string());
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
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeOrderableDBInstanceOptions.
pub fn deserialize_describe_orderable_d_b_instance_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrderableDBInstanceOptionsMessage, String> {
    let mut input = DescribeOrderableDBInstanceOptionsMessage::default();
    if let Some(value) = params.get("AvailabilityZoneGroup") {
        input.availability_zone_group = Some(value.to_string());
    }
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

/// Deserialize awsQuery request for DescribeReservedDBInstances.
pub fn deserialize_describe_reserved_d_b_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedDBInstancesMessage, String> {
    let mut input = DescribeReservedDBInstancesMessage::default();
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("Duration") {
        input.duration = Some(value.to_string());
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
    if let Some(value) = params.get("LeaseId") {
        input.lease_id = Some(value.to_string());
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
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("OfferingType") {
        input.offering_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ProductDescription") {
        input.product_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedDBInstanceId") {
        input.reserved_d_b_instance_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedDBInstancesOfferingId") {
        input.reserved_d_b_instances_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedDBInstancesOfferings.
pub fn deserialize_describe_reserved_d_b_instances_offerings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedDBInstancesOfferingsMessage, String> {
    let mut input = DescribeReservedDBInstancesOfferingsMessage::default();
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("Duration") {
        input.duration = Some(value.to_string());
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
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("OfferingType") {
        input.offering_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ProductDescription") {
        input.product_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedDBInstancesOfferingId") {
        input.reserved_d_b_instances_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeSourceRegions.
pub fn deserialize_describe_source_regions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeSourceRegionsMessage, String> {
    let mut input = DescribeSourceRegionsMessage::default();
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
    if let Some(value) = params.get("RegionName") {
        input.region_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTenantDatabases.
pub fn deserialize_describe_tenant_databases_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTenantDatabasesMessage, String> {
    let mut input = DescribeTenantDatabasesMessage::default();
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
    if let Some(value) = params.get("TenantDBName") {
        input.tenant_d_b_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DisableHttpEndpoint.
pub fn deserialize_disable_http_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableHttpEndpointRequest, String> {
    let mut input = DisableHttpEndpointRequest::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DownloadDBLogFilePortion.
pub fn deserialize_download_d_b_log_file_portion_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DownloadDBLogFilePortionMessage, String> {
    let mut input = DownloadDBLogFilePortionMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("LogFileName") {
        input.log_file_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("NumberOfLines") {
        input.number_of_lines = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfLines: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableHttpEndpoint.
pub fn deserialize_enable_http_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableHttpEndpointRequest, String> {
    let mut input = EnableHttpEndpointRequest::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for FailoverDBCluster.
pub fn deserialize_failover_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<FailoverDBClusterMessage, String> {
    let mut input = FailoverDBClusterMessage::default();
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
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

/// Deserialize awsQuery request for ModifyActivityStream.
pub fn deserialize_modify_activity_stream_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyActivityStreamRequest, String> {
    let mut input = ModifyActivityStreamRequest::default();
    if let Some(value) = params.get("AuditPolicyState") {
        input.audit_policy_state = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCertificates.
pub fn deserialize_modify_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCertificatesMessage, String> {
    let mut input = ModifyCertificatesMessage::default();
    if let Some(value) = params.get("CertificateIdentifier") {
        input.certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("RemoveCustomerOverride") {
        input.remove_customer_override = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RemoveCustomerOverride: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCurrentDBClusterCapacity.
pub fn deserialize_modify_current_d_b_cluster_capacity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCurrentDBClusterCapacityMessage, String> {
    let mut input = ModifyCurrentDBClusterCapacityMessage::default();
    if let Some(value) = params.get("Capacity") {
        input.capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Capacity: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("SecondsBeforeTimeout") {
        input.seconds_before_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SecondsBeforeTimeout: {e}"))?,
        );
    }
    if let Some(value) = params.get("TimeoutAction") {
        input.timeout_action = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCustomDBEngineVersion.
pub fn deserialize_modify_custom_d_b_engine_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCustomDBEngineVersionMessage, String> {
    let mut input = ModifyCustomDBEngineVersionMessage::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBCluster.
pub fn deserialize_modify_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBClusterMessage, String> {
    let mut input = ModifyDBClusterMessage::default();
    if let Some(value) = params.get("AllocatedStorage") {
        input.allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AllocatedStorage: {e}"))?,
        );
    }
    if let Some(value) = params.get("AllowEngineModeChange") {
        input.allow_engine_mode_change = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowEngineModeChange: {e}"))?,
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
    if let Some(value) = params.get("AwsBackupRecoveryPointArn") {
        input.aws_backup_recovery_point_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("BacktrackWindow") {
        input.backtrack_window = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse BacktrackWindow: {e}"))?,
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
    if let Some(value) = params.get("DBClusterIdentifier") {
        input.d_b_cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBClusterInstanceClass") {
        input.d_b_cluster_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterParameterGroupName") {
        input.d_b_cluster_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceParameterGroupName") {
        input.d_b_instance_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
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
    if let Some(value) = params.get("EnableGlobalWriteForwarding") {
        input.enable_global_write_forwarding = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableGlobalWriteForwarding: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableHttpEndpoint") {
        input.enable_http_endpoint = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableHttpEndpoint: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableLimitlessDatabase") {
        input.enable_limitless_database = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableLimitlessDatabase: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableLocalWriteForwarding") {
        input.enable_local_write_forwarding = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableLocalWriteForwarding: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnablePerformanceInsights") {
        input.enable_performance_insights = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnablePerformanceInsights: {e}"))?,
        );
    }
    if let Some(value) = params.get("EngineMode") {
        input.engine_mode = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserAuthenticationType") {
        input.master_user_authentication_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NewDBClusterIdentifier") {
        input.new_d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("RotateMasterUserPassword") {
        input.rotate_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RotateMasterUserPassword: {e}"))?,
        );
    }
    if let Some(val) = deserialize_scaling_configuration_from_query(params, "ScalingConfiguration")?
    {
        input.scaling_configuration = Some(val);
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
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_modify_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(ModifyAdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("AutomationMode") {
        input.automation_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("AwsBackupRecoveryPointArn") {
        input.aws_backup_recovery_point_arn = Some(value.to_string());
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
    if let Some(value) = params.get("CertificateRotationRestart") {
        input.certificate_rotation_restart = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CertificateRotationRestart: {e}"))?,
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
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeletionProtection: {e}"))?,
        );
    }
    if let Some(value) = params.get("DisableDomain") {
        input.disable_domain = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisableDomain: {e}"))?,
        );
    }
    if let Some(value) = params.get("Domain") {
        input.domain = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainAuthSecretArn") {
        input.domain_auth_secret_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DomainDnsIps".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.domain_dns_ips = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DomainFqdn") {
        input.domain_fqdn = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainOu") {
        input.domain_ou = Some(value.to_string());
    }
    if let Some(value) = params.get("EnableCustomerOwnedIp") {
        input.enable_customer_owned_ip = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCustomerOwnedIp: {e}"))?,
        );
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
        input.engine = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserAuthenticationType") {
        input.master_user_authentication_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxAllocatedStorage") {
        input.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
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
    if let Some(value) = params.get("MultiTenant") {
        input.multi_tenant = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiTenant: {e}"))?,
        );
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
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
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    if let Some(value) = params.get("PreferredMaintenanceWindow") {
        input.preferred_maintenance_window = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
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
    if let Some(value) = params.get("ReplicaMode") {
        input.replica_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("ResumeFullAutomationModeMinutes") {
        input.resume_full_automation_mode_minutes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ResumeFullAutomationModeMinutes: {e}"))?,
        );
    }
    if let Some(value) = params.get("RotateMasterUserPassword") {
        input.rotate_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RotateMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
        }
    }
    if let Some(value) = params.get("TdeCredentialArn") {
        input.tde_credential_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialPassword") {
        input.tde_credential_password = Some(value.to_string());
    }
    if let Some(value) = params.get("UseDefaultProcessorFeatures") {
        input.use_default_processor_features = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseDefaultProcessorFeatures: {e}"))?,
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

/// Deserialize awsQuery request for ModifyDBProxy.
pub fn deserialize_modify_d_b_proxy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBProxyRequest, String> {
    let mut input = ModifyDBProxyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Auth".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_user_auth_config_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.auth = Some(UserAuthConfigList { items });
        }
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("DebugLogging") {
        input.debug_logging = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DebugLogging: {e}"))?,
        );
    }
    if let Some(value) = params.get("DefaultAuthScheme") {
        input.default_auth_scheme = Some(value.to_string());
    }
    if let Some(value) = params.get("IdleClientTimeout") {
        input.idle_client_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse IdleClientTimeout: {e}"))?,
        );
    }
    if let Some(value) = params.get("NewDBProxyName") {
        input.new_d_b_proxy_name = Some(value.to_string());
    }
    if let Some(value) = params.get("RequireTLS") {
        input.require_t_l_s = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireTLS: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_groups = Some(StringList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBProxyEndpoint.
pub fn deserialize_modify_d_b_proxy_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBProxyEndpointRequest, String> {
    let mut input = ModifyDBProxyEndpointRequest::default();
    if let Some(value) = params.get("DBProxyEndpointName") {
        input.d_b_proxy_endpoint_name = value.to_string();
    }
    if let Some(value) = params.get("NewDBProxyEndpointName") {
        input.new_d_b_proxy_endpoint_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcSecurityGroupIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_security_group_ids = Some(StringList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBProxyTargetGroup.
pub fn deserialize_modify_d_b_proxy_target_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBProxyTargetGroupRequest, String> {
    let mut input = ModifyDBProxyTargetGroupRequest::default();
    if let Some(val) =
        deserialize_connection_pool_configuration_from_query(params, "ConnectionPoolConfig")?
    {
        input.connection_pool_config = Some(val);
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("NewName") {
        input.new_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetGroupName") {
        input.target_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBRecommendation.
pub fn deserialize_modify_d_b_recommendation_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBRecommendationMessage, String> {
    let mut input = ModifyDBRecommendationMessage::default();
    if let Some(value) = params.get("Locale") {
        input.locale = Some(value.to_string());
    }
    if let Some(value) = params.get("RecommendationId") {
        input.recommendation_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RecommendedActionUpdates".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_recommended_action_update_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.recommended_action_updates = Some(RecommendedActionUpdateList { items });
        }
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBShardGroup.
pub fn deserialize_modify_d_b_shard_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBShardGroupMessage, String> {
    let mut input = ModifyDBShardGroupMessage::default();
    if let Some(value) = params.get("ComputeRedundancy") {
        input.compute_redundancy = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ComputeRedundancy: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBShardGroupIdentifier") {
        input.d_b_shard_group_identifier = value.to_string();
    }
    if let Some(value) = params.get("MaxACU") {
        input.max_a_c_u = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MaxACU: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinACU") {
        input.min_a_c_u = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MinACU: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBSnapshot.
pub fn deserialize_modify_d_b_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBSnapshotMessage, String> {
    let mut input = ModifyDBSnapshotMessage::default();
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = value.to_string();
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyDBSnapshotAttribute.
pub fn deserialize_modify_d_b_snapshot_attribute_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyDBSnapshotAttributeMessage, String> {
    let mut input = ModifyDBSnapshotAttributeMessage::default();
    if let Some(value) = params.get("AttributeName") {
        input.attribute_name = value.to_string();
    }
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = value.to_string();
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

/// Deserialize awsQuery request for ModifyIntegration.
pub fn deserialize_modify_integration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyIntegrationMessage, String> {
    let mut input = ModifyIntegrationMessage::default();
    if let Some(value) = params.get("DataFilter") {
        input.data_filter = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("IntegrationIdentifier") {
        input.integration_identifier = value.to_string();
    }
    if let Some(value) = params.get("IntegrationName") {
        input.integration_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyOptionGroup.
pub fn deserialize_modify_option_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyOptionGroupMessage, String> {
    let mut input = ModifyOptionGroupMessage::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionsToInclude".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.OptionConfiguration.{i}");
            match deserialize_option_configuration_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options_to_include = Some(OptionConfigurationList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options_to_remove = Some(OptionNamesList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyTenantDatabase.
pub fn deserialize_modify_tenant_database_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyTenantDatabaseMessage, String> {
    let mut input = ModifyTenantDatabaseMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("NewTenantDBName") {
        input.new_tenant_d_b_name = Some(value.to_string());
    }
    if let Some(value) = params.get("RotateMasterUserPassword") {
        input.rotate_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RotateMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("TenantDBName") {
        input.tenant_d_b_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PromoteReadReplica.
pub fn deserialize_promote_read_replica_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PromoteReadReplicaMessage, String> {
    let mut input = PromoteReadReplicaMessage::default();
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("PreferredBackupWindow") {
        input.preferred_backup_window = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
        }
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

/// Deserialize awsQuery request for PurchaseReservedDBInstancesOffering.
pub fn deserialize_purchase_reserved_d_b_instances_offering_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PurchaseReservedDBInstancesOfferingMessage, String> {
    let mut input = PurchaseReservedDBInstancesOfferingMessage::default();
    if let Some(value) = params.get("DBInstanceCount") {
        input.d_b_instance_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DBInstanceCount: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReservedDBInstanceId") {
        input.reserved_d_b_instance_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedDBInstancesOfferingId") {
        input.reserved_d_b_instances_offering_id = value.to_string();
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

/// Deserialize awsQuery request for RebootDBCluster.
pub fn deserialize_reboot_d_b_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebootDBClusterMessage, String> {
    let mut input = RebootDBClusterMessage::default();
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

/// Deserialize awsQuery request for RebootDBShardGroup.
pub fn deserialize_reboot_d_b_shard_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebootDBShardGroupMessage, String> {
    let mut input = RebootDBShardGroupMessage::default();
    if let Some(value) = params.get("DBShardGroupIdentifier") {
        input.d_b_shard_group_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterDBProxyTargets.
pub fn deserialize_register_d_b_proxy_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterDBProxyTargetsRequest, String> {
    let mut input = RegisterDBProxyTargetsRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "DBClusterIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_cluster_identifiers = Some(StringList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DBInstanceIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.d_b_instance_identifiers = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DBProxyName") {
        input.d_b_proxy_name = value.to_string();
    }
    if let Some(value) = params.get("TargetGroupName") {
        input.target_group_name = Some(value.to_string());
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

/// Deserialize awsQuery request for RemoveRoleFromDBInstance.
pub fn deserialize_remove_role_from_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveRoleFromDBInstanceMessage, String> {
    let mut input = RemoveRoleFromDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("FeatureName") {
        input.feature_name = value.to_string();
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

/// Deserialize awsQuery request for RestoreDBClusterFromS3.
pub fn deserialize_restore_d_b_cluster_from_s3_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBClusterFromS3Message, String> {
    let mut input = RestoreDBClusterFromS3Message::default();
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
    if let Some(value) = params.get("BacktrackWindow") {
        input.backtrack_window = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse BacktrackWindow: {e}"))?,
        );
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
    if let Some(value) = params.get("Engine") {
        input.engine = value.to_string();
    }
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineVersion") {
        input.engine_version = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = value.to_string();
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
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
    if let Some(value) = params.get("S3BucketName") {
        input.s3_bucket_name = value.to_string();
    }
    if let Some(value) = params.get("S3IngestionRoleArn") {
        input.s3_ingestion_role_arn = value.to_string();
    }
    if let Some(value) = params.get("S3Prefix") {
        input.s3_prefix = Some(value.to_string());
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("SourceEngine") {
        input.source_engine = value.to_string();
    }
    if let Some(value) = params.get("SourceEngineVersion") {
        input.source_engine_version = value.to_string();
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
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("BacktrackWindow") {
        input.backtrack_window = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse BacktrackWindow: {e}"))?,
        );
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("DBClusterInstanceClass") {
        input.d_b_cluster_instance_class = Some(value.to_string());
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineMode") {
        input.engine_mode = Some(value.to_string());
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(val) = deserialize_rds_custom_cluster_configuration_from_query(
        params,
        "RdsCustomClusterConfiguration",
    )? {
        input.rds_custom_cluster_configuration = Some(val);
    }
    if let Some(val) = deserialize_scaling_configuration_from_query(params, "ScalingConfiguration")?
    {
        input.scaling_configuration = Some(val);
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
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("BacktrackWindow") {
        input.backtrack_window = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse BacktrackWindow: {e}"))?,
        );
    }
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("DBClusterInstanceClass") {
        input.d_b_cluster_instance_class = Some(value.to_string());
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineMode") {
        input.engine_mode = Some(value.to_string());
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(val) = deserialize_rds_custom_cluster_configuration_from_query(
        params,
        "RdsCustomClusterConfiguration",
    )? {
        input.rds_custom_cluster_configuration = Some(val);
    }
    if let Some(value) = params.get("RestoreToTime") {
        input.restore_to_time = Some(value.to_string());
    }
    if let Some(value) = params.get("RestoreType") {
        input.restore_type = Some(value.to_string());
    }
    if let Some(val) = deserialize_scaling_configuration_from_query(params, "ScalingConfiguration")?
    {
        input.scaling_configuration = Some(val);
    }
    if let Some(val) = deserialize_serverless_v2_scaling_configuration_from_query(
        params,
        "ServerlessV2ScalingConfiguration",
    )? {
        input.serverless_v2_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("SourceDBClusterIdentifier") {
        input.source_d_b_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDbClusterResourceId") {
        input.source_db_cluster_resource_id = Some(value.to_string());
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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

/// Deserialize awsQuery request for RestoreDBInstanceFromDBSnapshot.
pub fn deserialize_restore_d_b_instance_from_d_b_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBInstanceFromDBSnapshotMessage, String> {
    let mut input = RestoreDBInstanceFromDBSnapshotMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(AdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("BackupTarget") {
        input.backup_target = Some(value.to_string());
    }
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("CustomIamInstanceProfile") {
        input.custom_iam_instance_profile = Some(value.to_string());
    }
    if let Some(value) = params.get("DBClusterSnapshotIdentifier") {
        input.d_b_cluster_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
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
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
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
    if let Some(value) = params.get("DomainAuthSecretArn") {
        input.domain_auth_secret_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DomainDnsIps".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.domain_dns_ips = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DomainFqdn") {
        input.domain_fqdn = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainOu") {
        input.domain_ou = Some(value.to_string());
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
    if let Some(value) = params.get("EnableCustomerOwnedIp") {
        input.enable_customer_owned_ip = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCustomerOwnedIp: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("TdeCredentialArn") {
        input.tde_credential_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialPassword") {
        input.tde_credential_password = Some(value.to_string());
    }
    if let Some(value) = params.get("UseDefaultProcessorFeatures") {
        input.use_default_processor_features = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseDefaultProcessorFeatures: {e}"))?,
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

/// Deserialize awsQuery request for RestoreDBInstanceFromS3.
pub fn deserialize_restore_d_b_instance_from_s3_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBInstanceFromS3Message, String> {
    let mut input = RestoreDBInstanceFromS3Message::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(AdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
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
    if let Some(value) = params.get("DatabaseInsightsMode") {
        input.database_insights_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
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
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxAllocatedStorage") {
        input.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
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
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
    }
    if let Some(value) = params.get("OptionGroupName") {
        input.option_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsKMSKeyId") {
        input.performance_insights_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PerformanceInsightsRetentionPeriod") {
        input.performance_insights_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PerformanceInsightsRetentionPeriod: {e}"))?,
        );
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
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("S3BucketName") {
        input.s3_bucket_name = value.to_string();
    }
    if let Some(value) = params.get("S3IngestionRoleArn") {
        input.s3_ingestion_role_arn = value.to_string();
    }
    if let Some(value) = params.get("S3Prefix") {
        input.s3_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceEngine") {
        input.source_engine = value.to_string();
    }
    if let Some(value) = params.get("SourceEngineVersion") {
        input.source_engine_version = value.to_string();
    }
    if let Some(value) = params.get("StorageEncrypted") {
        input.storage_encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse StorageEncrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("UseDefaultProcessorFeatures") {
        input.use_default_processor_features = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseDefaultProcessorFeatures: {e}"))?,
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

/// Deserialize awsQuery request for RestoreDBInstanceToPointInTime.
pub fn deserialize_restore_d_b_instance_to_point_in_time_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreDBInstanceToPointInTimeMessage, String> {
    let mut input = RestoreDBInstanceToPointInTimeMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AdditionalStorageVolumes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_additional_storage_volume_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.additional_storage_volumes = Some(AdditionalStorageVolumesList { items });
        }
    }
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
    if let Some(value) = params.get("BackupTarget") {
        input.backup_target = Some(value.to_string());
    }
    if let Some(value) = params.get("CACertificateIdentifier") {
        input.c_a_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CopyTagsToSnapshot") {
        input.copy_tags_to_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CopyTagsToSnapshot: {e}"))?,
        );
    }
    if let Some(value) = params.get("CustomIamInstanceProfile") {
        input.custom_iam_instance_profile = Some(value.to_string());
    }
    if let Some(value) = params.get("DBInstanceClass") {
        input.d_b_instance_class = Some(value.to_string());
    }
    if let Some(value) = params.get("DBName") {
        input.d_b_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBParameterGroupName") {
        input.d_b_parameter_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSubnetGroupName") {
        input.d_b_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DedicatedLogVolume") {
        input.dedicated_log_volume = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DedicatedLogVolume: {e}"))?,
        );
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
    if let Some(value) = params.get("DomainAuthSecretArn") {
        input.domain_auth_secret_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DomainDnsIps".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.domain_dns_ips = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("DomainFqdn") {
        input.domain_fqdn = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainIAMRoleName") {
        input.domain_i_a_m_role_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DomainOu") {
        input.domain_ou = Some(value.to_string());
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
    if let Some(value) = params.get("EnableCustomerOwnedIp") {
        input.enable_customer_owned_ip = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCustomerOwnedIp: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableIAMDatabaseAuthentication") {
        input.enable_i_a_m_database_authentication = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableIAMDatabaseAuthentication: {e}"))?,
        );
    }
    if let Some(value) = params.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = params.get("EngineLifecycleSupport") {
        input.engine_lifecycle_support = Some(value.to_string());
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
    if let Some(value) = params.get("ManageMasterUserPassword") {
        input.manage_master_user_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterUserPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterUserSecretKmsKeyId") {
        input.master_user_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxAllocatedStorage") {
        input.max_allocated_storage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAllocatedStorage: {e}"))?,
        );
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NetworkType") {
        input.network_type = Some(value.to_string());
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
    {
        let mut items = Vec::new();
        let list_prefix = "ProcessorFeatures".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ProcessorFeature.{i}");
            match deserialize_processor_feature_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.processor_features = Some(ProcessorFeatureList { items });
        }
    }
    if let Some(value) = params.get("PubliclyAccessible") {
        input.publicly_accessible = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PubliclyAccessible: {e}"))?,
        );
    }
    if let Some(value) = params.get("RestoreTime") {
        input.restore_time = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBInstanceAutomatedBackupsArn") {
        input.source_d_b_instance_automated_backups_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBInstanceIdentifier") {
        input.source_d_b_instance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDbiResourceId") {
        input.source_dbi_resource_id = Some(value.to_string());
    }
    if let Some(value) = params.get("StorageThroughput") {
        input.storage_throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageThroughput: {e}"))?,
        );
    }
    if let Some(value) = params.get("StorageType") {
        input.storage_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagSpecifications".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.item.{i}");
            match deserialize_tag_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_specifications = Some(TagSpecificationList { items });
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
    if let Some(value) = params.get("TargetDBInstanceIdentifier") {
        input.target_d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("TdeCredentialArn") {
        input.tde_credential_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TdeCredentialPassword") {
        input.tde_credential_password = Some(value.to_string());
    }
    if let Some(value) = params.get("UseDefaultProcessorFeatures") {
        input.use_default_processor_features = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseDefaultProcessorFeatures: {e}"))?,
        );
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

/// Deserialize awsQuery request for RevokeDBSecurityGroupIngress.
pub fn deserialize_revoke_d_b_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RevokeDBSecurityGroupIngressMessage, String> {
    let mut input = RevokeDBSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CIDRIP") {
        input.c_i_d_r_i_p = Some(value.to_string());
    }
    if let Some(value) = params.get("DBSecurityGroupName") {
        input.d_b_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupId") {
        input.e_c2_security_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartActivityStream.
pub fn deserialize_start_activity_stream_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartActivityStreamRequest, String> {
    let mut input = StartActivityStreamRequest::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("EngineNativeAuditFieldsIncluded") {
        input.engine_native_audit_fields_included = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EngineNativeAuditFieldsIncluded: {e}"))?,
        );
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = value.to_string();
    }
    if let Some(value) = params.get("Mode") {
        input.mode = value.to_string();
    }
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
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

/// Deserialize awsQuery request for StartDBInstance.
pub fn deserialize_start_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartDBInstanceMessage, String> {
    let mut input = StartDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartDBInstanceAutomatedBackupsReplication.
pub fn deserialize_start_d_b_instance_automated_backups_replication_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartDBInstanceAutomatedBackupsReplicationMessage, String> {
    let mut input = StartDBInstanceAutomatedBackupsReplicationMessage::default();
    if let Some(value) = params.get("BackupRetentionPeriod") {
        input.backup_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BackupRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PreSignedUrl") {
        input.pre_signed_url = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceDBInstanceArn") {
        input.source_d_b_instance_arn = value.to_string();
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

/// Deserialize awsQuery request for StartExportTask.
pub fn deserialize_start_export_task_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartExportTaskMessage, String> {
    let mut input = StartExportTaskMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ExportOnly".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.export_only = Some(StringList { items });
        }
    }
    if let Some(value) = params.get("ExportTaskIdentifier") {
        input.export_task_identifier = value.to_string();
    }
    if let Some(value) = params.get("IamRoleArn") {
        input.iam_role_arn = value.to_string();
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = value.to_string();
    }
    if let Some(value) = params.get("S3BucketName") {
        input.s3_bucket_name = value.to_string();
    }
    if let Some(value) = params.get("S3Prefix") {
        input.s3_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StopActivityStream.
pub fn deserialize_stop_activity_stream_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopActivityStreamRequest, String> {
    let mut input = StopActivityStreamRequest::default();
    if let Some(value) = params.get("ApplyImmediately") {
        input.apply_immediately = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyImmediately: {e}"))?,
        );
    }
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
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

/// Deserialize awsQuery request for StopDBInstance.
pub fn deserialize_stop_d_b_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopDBInstanceMessage, String> {
    let mut input = StopDBInstanceMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    if let Some(value) = params.get("DBSnapshotIdentifier") {
        input.d_b_snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for StopDBInstanceAutomatedBackupsReplication.
pub fn deserialize_stop_d_b_instance_automated_backups_replication_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopDBInstanceAutomatedBackupsReplicationMessage, String> {
    let mut input = StopDBInstanceAutomatedBackupsReplicationMessage::default();
    if let Some(value) = params.get("SourceDBInstanceArn") {
        input.source_d_b_instance_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SwitchoverBlueGreenDeployment.
pub fn deserialize_switchover_blue_green_deployment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SwitchoverBlueGreenDeploymentRequest, String> {
    let mut input = SwitchoverBlueGreenDeploymentRequest::default();
    if let Some(value) = params.get("BlueGreenDeploymentIdentifier") {
        input.blue_green_deployment_identifier = value.to_string();
    }
    if let Some(value) = params.get("SwitchoverTimeout") {
        input.switchover_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SwitchoverTimeout: {e}"))?,
        );
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

/// Deserialize awsQuery request for SwitchoverReadReplica.
pub fn deserialize_switchover_read_replica_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SwitchoverReadReplicaMessage, String> {
    let mut input = SwitchoverReadReplicaMessage::default();
    if let Some(value) = params.get("DBInstanceIdentifier") {
        input.d_b_instance_identifier = value.to_string();
    }
    Ok(input)
}
