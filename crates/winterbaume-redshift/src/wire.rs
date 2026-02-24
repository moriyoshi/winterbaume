//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-redshift

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
pub fn serialize_accept_reserved_node_exchange_response(
    result: &AcceptReservedNodeExchangeOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AcceptReservedNodeExchangeResult>{inner_xml}</AcceptReservedNodeExchangeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AcceptReservedNodeExchangeResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AcceptReservedNodeExchangeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_add_partner_response(result: &PartnerIntegrationOutputMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AddPartnerResult>{inner_xml}</AddPartnerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddPartnerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddPartnerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_associate_data_share_consumer_response(result: &DataShare) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AssociateDataShareConsumerResult>{inner_xml}</AssociateDataShareConsumerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssociateDataShareConsumerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssociateDataShareConsumerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_authorize_cluster_security_group_ingress_response(
    result: &AuthorizeClusterSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AuthorizeClusterSecurityGroupIngressResult>{inner_xml}</AuthorizeClusterSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeClusterSecurityGroupIngressResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeClusterSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_authorize_data_share_response(result: &DataShare) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AuthorizeDataShareResult>{inner_xml}</AuthorizeDataShareResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeDataShareResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeDataShareResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_authorize_endpoint_access_response(
    result: &EndpointAuthorization,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AuthorizeEndpointAccessResult>{inner_xml}</AuthorizeEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeEndpointAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_authorize_snapshot_access_response(
    result: &AuthorizeSnapshotAccessResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AuthorizeSnapshotAccessResult>{inner_xml}</AuthorizeSnapshotAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AuthorizeSnapshotAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AuthorizeSnapshotAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_delete_cluster_snapshots_response(
    result: &BatchDeleteClusterSnapshotsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<BatchDeleteClusterSnapshotsResult>{inner_xml}</BatchDeleteClusterSnapshotsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchDeleteClusterSnapshotsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchDeleteClusterSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_modify_cluster_snapshots_response(
    result: &BatchModifyClusterSnapshotsOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<BatchModifyClusterSnapshotsResult>{inner_xml}</BatchModifyClusterSnapshotsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchModifyClusterSnapshotsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchModifyClusterSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_cancel_resize_response(result: &ResizeProgressMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CancelResizeResult>{inner_xml}</CancelResizeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CancelResizeResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CancelResizeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_copy_cluster_snapshot_response(
    result: &CopyClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CopyClusterSnapshotResult>{inner_xml}</CopyClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CopyClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CopyClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_authentication_profile_response(
    result: &CreateAuthenticationProfileResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateAuthenticationProfileResult>{inner_xml}</CreateAuthenticationProfileResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateAuthenticationProfileResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAuthenticationProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateClusterResult>{inner_xml}</CreateClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cluster_parameter_group_response(
    result: &CreateClusterParameterGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateClusterParameterGroupResult>{inner_xml}</CreateClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateClusterParameterGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cluster_security_group_response(
    result: &CreateClusterSecurityGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateClusterSecurityGroupResult>{inner_xml}</CreateClusterSecurityGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateClusterSecurityGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateClusterSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cluster_snapshot_response(
    result: &CreateClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateClusterSnapshotResult>{inner_xml}</CreateClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_cluster_subnet_group_response(
    result: &CreateClusterSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateClusterSubnetGroupResult>{inner_xml}</CreateClusterSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateClusterSubnetGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateClusterSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_custom_domain_association_response(
    result: &CreateCustomDomainAssociationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateCustomDomainAssociationResult>{inner_xml}</CreateCustomDomainAssociationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCustomDomainAssociationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCustomDomainAssociationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_endpoint_access_response(result: &EndpointAccess) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateEndpointAccessResult>{inner_xml}</CreateEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateEndpointAccessResponse>"#
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
        r#"<CreateEventSubscriptionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateEventSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_hsm_client_certificate_response(
    result: &CreateHsmClientCertificateResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateHsmClientCertificateResult>{inner_xml}</CreateHsmClientCertificateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateHsmClientCertificateResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateHsmClientCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_hsm_configuration_response(
    result: &CreateHsmConfigurationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateHsmConfigurationResult>{inner_xml}</CreateHsmConfigurationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateHsmConfigurationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateHsmConfigurationResponse>"#
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
        r#"<CreateIntegrationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_redshift_idc_application_response(
    result: &CreateRedshiftIdcApplicationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateRedshiftIdcApplicationResult>{inner_xml}</CreateRedshiftIdcApplicationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateRedshiftIdcApplicationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateRedshiftIdcApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_scheduled_action_response(result: &ScheduledAction) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateScheduledActionResult>{inner_xml}</CreateScheduledActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateScheduledActionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateScheduledActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_snapshot_copy_grant_response(
    result: &CreateSnapshotCopyGrantResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateSnapshotCopyGrantResult>{inner_xml}</CreateSnapshotCopyGrantResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateSnapshotCopyGrantResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateSnapshotCopyGrantResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_snapshot_schedule_response(result: &SnapshotSchedule) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateSnapshotScheduleResult>{inner_xml}</CreateSnapshotScheduleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateSnapshotScheduleResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateSnapshotScheduleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_tags_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTagsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_usage_limit_response(result: &UsageLimit) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateUsageLimitResult>{inner_xml}</CreateUsageLimitResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateUsageLimitResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateUsageLimitResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deauthorize_data_share_response(result: &DataShare) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeauthorizeDataShareResult>{inner_xml}</DeauthorizeDataShareResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeauthorizeDataShareResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeauthorizeDataShareResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_authentication_profile_response(
    result: &DeleteAuthenticationProfileResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteAuthenticationProfileResult>{inner_xml}</DeleteAuthenticationProfileResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAuthenticationProfileResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAuthenticationProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteClusterResult>{inner_xml}</DeleteClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cluster_parameter_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteClusterParameterGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cluster_security_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteClusterSecurityGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteClusterSecurityGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_cluster_snapshot_response(
    result: &DeleteClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteClusterSnapshotResult>{inner_xml}</DeleteClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_cluster_subnet_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteClusterSubnetGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteClusterSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_custom_domain_association_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCustomDomainAssociationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCustomDomainAssociationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_endpoint_access_response(result: &EndpointAccess) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteEndpointAccessResult>{inner_xml}</DeleteEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteEndpointAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_event_subscription_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteEventSubscriptionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteEventSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_hsm_client_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteHsmClientCertificateResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteHsmClientCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_hsm_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteHsmConfigurationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteHsmConfigurationResponse>"#
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
        r#"<DeleteIntegrationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_partner_response(result: &PartnerIntegrationOutputMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeletePartnerResult>{inner_xml}</DeletePartnerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePartnerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePartnerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_redshift_idc_application_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteRedshiftIdcApplicationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRedshiftIdcApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteResourcePolicyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteResourcePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_scheduled_action_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteScheduledActionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteScheduledActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_snapshot_copy_grant_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSnapshotCopyGrantResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSnapshotCopyGrantResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_snapshot_schedule_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSnapshotScheduleResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSnapshotScheduleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_tags_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTagsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_usage_limit_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUsageLimitResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUsageLimitResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deregister_namespace_response(
    result: &DeregisterNamespaceOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeregisterNamespaceResult>{inner_xml}</DeregisterNamespaceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeregisterNamespaceResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeregisterNamespaceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_attributes_response(
    result: &AccountAttributeList,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountAttributesResult>{inner_xml}</DescribeAccountAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountAttributesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_authentication_profiles_response(
    result: &DescribeAuthenticationProfilesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeAuthenticationProfilesResult>{inner_xml}</DescribeAuthenticationProfilesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAuthenticationProfilesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAuthenticationProfilesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_db_revisions_response(
    result: &ClusterDbRevisionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeClusterDbRevisionsResult>{inner_xml}</DescribeClusterDbRevisionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterDbRevisionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterDbRevisionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_parameter_groups_response(
    result: &ClusterParameterGroupsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeClusterParameterGroupsResult>{inner_xml}</DescribeClusterParameterGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterParameterGroupsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterParameterGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_parameters_response(
    result: &ClusterParameterGroupDetails,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeClusterParametersResult>{inner_xml}</DescribeClusterParametersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterParametersResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_security_groups_response(
    result: &ClusterSecurityGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeClusterSecurityGroupsResult>{inner_xml}</DescribeClusterSecurityGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterSecurityGroupsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterSecurityGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_snapshots_response(result: &SnapshotMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeClusterSnapshotsResult>{inner_xml}</DescribeClusterSnapshotsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterSnapshotsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterSnapshotsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_subnet_groups_response(
    result: &ClusterSubnetGroupMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeClusterSubnetGroupsResult>{inner_xml}</DescribeClusterSubnetGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterSubnetGroupsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterSubnetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_tracks_response(result: &TrackListMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeClusterTracksResult>{inner_xml}</DescribeClusterTracksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterTracksResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterTracksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_cluster_versions_response(
    result: &ClusterVersionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeClusterVersionsResult>{inner_xml}</DescribeClusterVersionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClusterVersionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClusterVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_clusters_response(result: &ClustersMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeClustersResult>{inner_xml}</DescribeClustersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeClustersResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeClustersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_custom_domain_associations_response(
    result: &CustomDomainAssociationsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeCustomDomainAssociationsResult>{inner_xml}</DescribeCustomDomainAssociationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCustomDomainAssociationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCustomDomainAssociationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_data_shares_response(result: &DescribeDataSharesResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeDataSharesResult>{inner_xml}</DescribeDataSharesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDataSharesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDataSharesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_data_shares_for_consumer_response(
    result: &DescribeDataSharesForConsumerResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDataSharesForConsumerResult>{inner_xml}</DescribeDataSharesForConsumerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDataSharesForConsumerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDataSharesForConsumerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_data_shares_for_producer_response(
    result: &DescribeDataSharesForProducerResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDataSharesForProducerResult>{inner_xml}</DescribeDataSharesForProducerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDataSharesForProducerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDataSharesForProducerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_default_cluster_parameters_response(
    result: &DescribeDefaultClusterParametersResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeDefaultClusterParametersResult>{inner_xml}</DescribeDefaultClusterParametersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeDefaultClusterParametersResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeDefaultClusterParametersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_endpoint_access_response(result: &EndpointAccessList) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeEndpointAccessResult>{inner_xml}</DescribeEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEndpointAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_endpoint_authorization_response(
    result: &EndpointAuthorizationList,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEndpointAuthorizationResult>{inner_xml}</DescribeEndpointAuthorizationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEndpointAuthorizationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEndpointAuthorizationResponse>"#
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
        r#"<DescribeEventCategoriesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
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
        r#"<DescribeEventSubscriptionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
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
        r#"<DescribeEventsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_hsm_client_certificates_response(
    result: &HsmClientCertificateMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeHsmClientCertificatesResult>{inner_xml}</DescribeHsmClientCertificatesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeHsmClientCertificatesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeHsmClientCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_hsm_configurations_response(
    result: &HsmConfigurationMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeHsmConfigurationsResult>{inner_xml}</DescribeHsmConfigurationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeHsmConfigurationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeHsmConfigurationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_inbound_integrations_response(
    result: &InboundIntegrationsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeInboundIntegrationsResult>{inner_xml}</DescribeInboundIntegrationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeInboundIntegrationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeInboundIntegrationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_integrations_response(result: &IntegrationsMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeIntegrationsResult>{inner_xml}</DescribeIntegrationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeIntegrationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeIntegrationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_logging_status_response(result: &LoggingStatus) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLoggingStatusResult>{inner_xml}</DescribeLoggingStatusResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoggingStatusResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoggingStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_node_configuration_options_response(
    result: &NodeConfigurationOptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeNodeConfigurationOptionsResult>{inner_xml}</DescribeNodeConfigurationOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeNodeConfigurationOptionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeNodeConfigurationOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_orderable_cluster_options_response(
    result: &OrderableClusterOptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeOrderableClusterOptionsResult>{inner_xml}</DescribeOrderableClusterOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeOrderableClusterOptionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeOrderableClusterOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_partners_response(
    result: &DescribePartnersOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribePartnersResult>{inner_xml}</DescribePartnersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribePartnersResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribePartnersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_redshift_idc_applications_response(
    result: &DescribeRedshiftIdcApplicationsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeRedshiftIdcApplicationsResult>{inner_xml}</DescribeRedshiftIdcApplicationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeRedshiftIdcApplicationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeRedshiftIdcApplicationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_node_exchange_status_response(
    result: &DescribeReservedNodeExchangeStatusOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeReservedNodeExchangeStatusResult>{inner_xml}</DescribeReservedNodeExchangeStatusResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedNodeExchangeStatusResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedNodeExchangeStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_node_offerings_response(
    result: &ReservedNodeOfferingsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeReservedNodeOfferingsResult>{inner_xml}</DescribeReservedNodeOfferingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedNodeOfferingsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedNodeOfferingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_reserved_nodes_response(result: &ReservedNodesMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeReservedNodesResult>{inner_xml}</DescribeReservedNodesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReservedNodesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReservedNodesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_resize_response(result: &ResizeProgressMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeResizeResult>{inner_xml}</DescribeResizeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeResizeResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeResizeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_scheduled_actions_response(
    result: &ScheduledActionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeScheduledActionsResult>{inner_xml}</DescribeScheduledActionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeScheduledActionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeScheduledActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_snapshot_copy_grants_response(
    result: &SnapshotCopyGrantMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeSnapshotCopyGrantsResult>{inner_xml}</DescribeSnapshotCopyGrantsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeSnapshotCopyGrantsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeSnapshotCopyGrantsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_snapshot_schedules_response(
    result: &DescribeSnapshotSchedulesOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeSnapshotSchedulesResult>{inner_xml}</DescribeSnapshotSchedulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeSnapshotSchedulesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeSnapshotSchedulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_storage_response(result: &CustomerStorageMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeStorageResult>{inner_xml}</DescribeStorageResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStorageResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStorageResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_table_restore_status_response(
    result: &TableRestoreStatusMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTableRestoreStatusResult>{inner_xml}</DescribeTableRestoreStatusResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTableRestoreStatusResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTableRestoreStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_tags_response(result: &TaggedResourceListMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeTagsResult>{inner_xml}</DescribeTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTagsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_usage_limits_response(result: &UsageLimitList) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeUsageLimitsResult>{inner_xml}</DescribeUsageLimitsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeUsageLimitsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeUsageLimitsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_logging_response(result: &LoggingStatus) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DisableLoggingResult>{inner_xml}</DisableLoggingResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableLoggingResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableLoggingResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_snapshot_copy_response(
    result: &DisableSnapshotCopyResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DisableSnapshotCopyResult>{inner_xml}</DisableSnapshotCopyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableSnapshotCopyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableSnapshotCopyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disassociate_data_share_consumer_response(result: &DataShare) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DisassociateDataShareConsumerResult>{inner_xml}</DisassociateDataShareConsumerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisassociateDataShareConsumerResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisassociateDataShareConsumerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_logging_response(result: &LoggingStatus) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<EnableLoggingResult>{inner_xml}</EnableLoggingResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableLoggingResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableLoggingResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_snapshot_copy_response(result: &EnableSnapshotCopyResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<EnableSnapshotCopyResult>{inner_xml}</EnableSnapshotCopyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableSnapshotCopyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableSnapshotCopyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_failover_primary_compute_response(
    result: &FailoverPrimaryComputeResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<FailoverPrimaryComputeResult>{inner_xml}</FailoverPrimaryComputeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<FailoverPrimaryComputeResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</FailoverPrimaryComputeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_cluster_credentials_response(result: &ClusterCredentials) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetClusterCredentialsResult>{inner_xml}</GetClusterCredentialsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetClusterCredentialsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetClusterCredentialsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_cluster_credentials_with_i_a_m_response(
    result: &ClusterExtendedCredentials,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetClusterCredentialsWithIAMResult>{inner_xml}</GetClusterCredentialsWithIAMResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetClusterCredentialsWithIAMResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetClusterCredentialsWithIAMResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_center_auth_token_response(
    result: &GetIdentityCenterAuthTokenResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetIdentityCenterAuthTokenResult>{inner_xml}</GetIdentityCenterAuthTokenResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityCenterAuthTokenResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityCenterAuthTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_reserved_node_exchange_configuration_options_response(
    result: &GetReservedNodeExchangeConfigurationOptionsOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetReservedNodeExchangeConfigurationOptionsResult>{inner_xml}</GetReservedNodeExchangeConfigurationOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetReservedNodeExchangeConfigurationOptionsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetReservedNodeExchangeConfigurationOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_reserved_node_exchange_offerings_response(
    result: &GetReservedNodeExchangeOfferingsOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetReservedNodeExchangeOfferingsResult>{inner_xml}</GetReservedNodeExchangeOfferingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetReservedNodeExchangeOfferingsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetReservedNodeExchangeOfferingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetResourcePolicyResult>{inner_xml}</GetResourcePolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetResourcePolicyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetResourcePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_recommendations_response(result: &ListRecommendationsResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListRecommendationsResult>{inner_xml}</ListRecommendationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListRecommendationsResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRecommendationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_aqua_configuration_response(
    result: &ModifyAquaOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyAquaConfigurationResult>{inner_xml}</ModifyAquaConfigurationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyAquaConfigurationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyAquaConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_authentication_profile_response(
    result: &ModifyAuthenticationProfileResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyAuthenticationProfileResult>{inner_xml}</ModifyAuthenticationProfileResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyAuthenticationProfileResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyAuthenticationProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_response(result: &ModifyClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyClusterResult>{inner_xml}</ModifyClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_db_revision_response(
    result: &ModifyClusterDbRevisionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyClusterDbRevisionResult>{inner_xml}</ModifyClusterDbRevisionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterDbRevisionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterDbRevisionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_iam_roles_response(
    result: &ModifyClusterIamRolesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyClusterIamRolesResult>{inner_xml}</ModifyClusterIamRolesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterIamRolesResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterIamRolesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_maintenance_response(
    result: &ModifyClusterMaintenanceResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyClusterMaintenanceResult>{inner_xml}</ModifyClusterMaintenanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterMaintenanceResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterMaintenanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_parameter_group_response(
    result: &ClusterParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyClusterParameterGroupResult>{inner_xml}</ModifyClusterParameterGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterParameterGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_snapshot_response(
    result: &ModifyClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyClusterSnapshotResult>{inner_xml}</ModifyClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_modify_cluster_snapshot_schedule_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterSnapshotScheduleResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterSnapshotScheduleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_cluster_subnet_group_response(
    result: &ModifyClusterSubnetGroupResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyClusterSubnetGroupResult>{inner_xml}</ModifyClusterSubnetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyClusterSubnetGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyClusterSubnetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_custom_domain_association_response(
    result: &ModifyCustomDomainAssociationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyCustomDomainAssociationResult>{inner_xml}</ModifyCustomDomainAssociationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCustomDomainAssociationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCustomDomainAssociationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_endpoint_access_response(result: &EndpointAccess) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyEndpointAccessResult>{inner_xml}</ModifyEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyEndpointAccessResponse>"#
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
        r#"<ModifyEventSubscriptionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyEventSubscriptionResponse>"#
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
        r#"<ModifyIntegrationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyIntegrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_lakehouse_configuration_response(
    result: &LakehouseConfiguration,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyLakehouseConfigurationResult>{inner_xml}</ModifyLakehouseConfigurationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyLakehouseConfigurationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyLakehouseConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_redshift_idc_application_response(
    result: &ModifyRedshiftIdcApplicationResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyRedshiftIdcApplicationResult>{inner_xml}</ModifyRedshiftIdcApplicationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyRedshiftIdcApplicationResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyRedshiftIdcApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_scheduled_action_response(result: &ScheduledAction) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyScheduledActionResult>{inner_xml}</ModifyScheduledActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyScheduledActionResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyScheduledActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_snapshot_copy_retention_period_response(
    result: &ModifySnapshotCopyRetentionPeriodResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifySnapshotCopyRetentionPeriodResult>{inner_xml}</ModifySnapshotCopyRetentionPeriodResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifySnapshotCopyRetentionPeriodResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifySnapshotCopyRetentionPeriodResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_snapshot_schedule_response(result: &SnapshotSchedule) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifySnapshotScheduleResult>{inner_xml}</ModifySnapshotScheduleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifySnapshotScheduleResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifySnapshotScheduleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_usage_limit_response(result: &UsageLimit) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyUsageLimitResult>{inner_xml}</ModifyUsageLimitResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyUsageLimitResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyUsageLimitResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_pause_cluster_response(result: &PauseClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PauseClusterResult>{inner_xml}</PauseClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PauseClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PauseClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_purchase_reserved_node_offering_response(
    result: &PurchaseReservedNodeOfferingResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<PurchaseReservedNodeOfferingResult>{inner_xml}</PurchaseReservedNodeOfferingResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PurchaseReservedNodeOfferingResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PurchaseReservedNodeOfferingResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutResourcePolicyResult>{inner_xml}</PutResourcePolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutResourcePolicyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutResourcePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reboot_cluster_response(result: &RebootClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RebootClusterResult>{inner_xml}</RebootClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebootClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebootClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_namespace_response(
    result: &RegisterNamespaceOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RegisterNamespaceResult>{inner_xml}</RegisterNamespaceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterNamespaceResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterNamespaceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reject_data_share_response(result: &DataShare) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RejectDataShareResult>{inner_xml}</RejectDataShareResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RejectDataShareResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RejectDataShareResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reset_cluster_parameter_group_response(
    result: &ClusterParameterGroupNameMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ResetClusterParameterGroupResult>{inner_xml}</ResetClusterParameterGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResetClusterParameterGroupResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResetClusterParameterGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_resize_cluster_response(result: &ResizeClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ResizeClusterResult>{inner_xml}</ResizeClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResizeClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResizeClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_from_cluster_snapshot_response(
    result: &RestoreFromClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RestoreFromClusterSnapshotResult>{inner_xml}</RestoreFromClusterSnapshotResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreFromClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreFromClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_restore_table_from_cluster_snapshot_response(
    result: &RestoreTableFromClusterSnapshotResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RestoreTableFromClusterSnapshotResult>{inner_xml}</RestoreTableFromClusterSnapshotResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestoreTableFromClusterSnapshotResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestoreTableFromClusterSnapshotResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_resume_cluster_response(result: &ResumeClusterResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ResumeClusterResult>{inner_xml}</ResumeClusterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResumeClusterResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResumeClusterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_revoke_cluster_security_group_ingress_response(
    result: &RevokeClusterSecurityGroupIngressResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RevokeClusterSecurityGroupIngressResult>{inner_xml}</RevokeClusterSecurityGroupIngressResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RevokeClusterSecurityGroupIngressResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RevokeClusterSecurityGroupIngressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_revoke_endpoint_access_response(result: &EndpointAuthorization) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RevokeEndpointAccessResult>{inner_xml}</RevokeEndpointAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RevokeEndpointAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RevokeEndpointAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_revoke_snapshot_access_response(
    result: &RevokeSnapshotAccessResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RevokeSnapshotAccessResult>{inner_xml}</RevokeSnapshotAccessResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RevokeSnapshotAccessResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RevokeSnapshotAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_rotate_encryption_key_response(
    result: &RotateEncryptionKeyResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RotateEncryptionKeyResult>{inner_xml}</RotateEncryptionKeyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RotateEncryptionKeyResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RotateEncryptionKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_partner_status_response(
    result: &PartnerIntegrationOutputMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdatePartnerStatusResult>{inner_xml}</UpdatePartnerStatusResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdatePartnerStatusResponse xmlns="http://redshift.amazonaws.com/doc/2012-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdatePartnerStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_delete_cluster_snapshot_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DeleteClusterSnapshotMessage>, String> {
    let mut item = DeleteClusterSnapshotMessage::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.SnapshotClusterIdentifier")) {
        item.snapshot_cluster_identifier = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SnapshotIdentifier")) {
        item.snapshot_identifier = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scheduled_action_type_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScheduledActionType>, String> {
    let mut item = ScheduledActionType::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scheduled_action_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScheduledActionFilter>, String> {
    let mut item = ScheduledActionFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.item.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = ValueStringList { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_authorized_token_issuer_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AuthorizedTokenIssuer>, String> {
    let mut item = AuthorizedTokenIssuer::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AuthorizedAudiencesList");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.authorized_audiences_list = Some(AuthorizedAudienceList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.TrustedTokenIssuerArn")) {
        item.trusted_token_issuer_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_redshift_scope_union_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RedshiftScopeUnion>, String> {
    let mut item = RedshiftScopeUnion::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_s3_access_grants_scope_union_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<S3AccessGrantsScopeUnion>, String> {
    let mut item = S3AccessGrantsScopeUnion::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_provisioned_identifier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ProvisionedIdentifier>, String> {
    let mut item = ProvisionedIdentifier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ClusterIdentifier")) {
        item.cluster_identifier = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_namespace_identifier_union_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<NamespaceIdentifierUnion>, String> {
    let mut item = NamespaceIdentifierUnion::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_describe_integrations_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DescribeIntegrationsFilter>, String> {
    let mut item = DescribeIntegrationsFilter::default();
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
            item.values = DescribeIntegrationsFilterValueList { items: sub_items };
            found = true;
        }
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

fn deserialize_service_integrations_union_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ServiceIntegrationsUnion>, String> {
    let mut item = ServiceIntegrationsUnion::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.LakeFormation");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_lake_formation_scope_union_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.lake_formation = Some(LakeFormationServiceIntegrations { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Redshift");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_redshift_scope_union_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.redshift = Some(RedshiftServiceIntegrations { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.S3AccessGrants");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_s3_access_grants_scope_union_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.s3_access_grants = Some(S3AccessGrantsServiceIntegrations { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_lake_formation_scope_union_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LakeFormationScopeUnion>, String> {
    let mut item = LakeFormationScopeUnion::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_read_write_access_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReadWriteAccess>, String> {
    let mut item = ReadWriteAccess::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Authorization")) {
        item.authorization = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_pause_cluster_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PauseClusterMessage>, String> {
    let mut item = PauseClusterMessage::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ClusterIdentifier")) {
        item.cluster_identifier = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_connect_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Connect>, String> {
    let mut item = Connect::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Authorization")) {
        item.authorization = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resume_cluster_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResumeClusterMessage>, String> {
    let mut item = ResumeClusterMessage::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ClusterIdentifier")) {
        item.cluster_identifier = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_lake_formation_query_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LakeFormationQuery>, String> {
    let mut item = LakeFormationQuery::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Authorization")) {
        item.authorization = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resize_cluster_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResizeClusterMessage>, String> {
    let mut item = ResizeClusterMessage::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Classic")) {
        item.classic = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Classic: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ClusterIdentifier")) {
        item.cluster_identifier = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ClusterType")) {
        item.cluster_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NodeType")) {
        item.node_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NumberOfNodes")) {
        item.number_of_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfNodes: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReservedNodeId")) {
        item.reserved_node_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TargetReservedNodeOfferingId")) {
        item.target_reserved_node_offering_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_serverless_identifier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ServerlessIdentifier>, String> {
    let mut item = ServerlessIdentifier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.NamespaceIdentifier")) {
        item.namespace_identifier = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.WorkgroupIdentifier")) {
        item.workgroup_identifier = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_snapshot_sorting_entity_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SnapshotSortingEntity>, String> {
    let mut item = SnapshotSortingEntity::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Attribute")) {
        item.attribute = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SortOrder")) {
        item.sort_order = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_node_configuration_options_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<NodeConfigurationOptionsFilter>, String> {
    let mut item = NodeConfigurationOptionsFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Operator")) {
        item.operator = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.item.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(ValueStringList { items: sub_items });
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

/// Deserialize awsQuery request for AcceptReservedNodeExchange.
pub fn deserialize_accept_reserved_node_exchange_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AcceptReservedNodeExchangeInputMessage, String> {
    let mut input = AcceptReservedNodeExchangeInputMessage::default();
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = value.to_string();
    }
    if let Some(value) = params.get("TargetReservedNodeOfferingId") {
        input.target_reserved_node_offering_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddPartner.
pub fn deserialize_add_partner_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PartnerIntegrationInputMessage, String> {
    let mut input = PartnerIntegrationInputMessage::default();
    if let Some(value) = params.get("AccountId") {
        input.account_id = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = value.to_string();
    }
    if let Some(value) = params.get("PartnerName") {
        input.partner_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssociateDataShareConsumer.
pub fn deserialize_associate_data_share_consumer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssociateDataShareConsumerMessage, String> {
    let mut input = AssociateDataShareConsumerMessage::default();
    if let Some(value) = params.get("AllowWrites") {
        input.allow_writes = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowWrites: {e}"))?,
        );
    }
    if let Some(value) = params.get("AssociateEntireAccount") {
        input.associate_entire_account = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AssociateEntireAccount: {e}"))?,
        );
    }
    if let Some(value) = params.get("ConsumerArn") {
        input.consumer_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("ConsumerRegion") {
        input.consumer_region = Some(value.to_string());
    }
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AuthorizeClusterSecurityGroupIngress.
pub fn deserialize_authorize_cluster_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeClusterSecurityGroupIngressMessage, String> {
    let mut input = AuthorizeClusterSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CIDRIP") {
        input.c_i_d_r_i_p = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterSecurityGroupName") {
        input.cluster_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for AuthorizeDataShare.
pub fn deserialize_authorize_data_share_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeDataShareMessage, String> {
    let mut input = AuthorizeDataShareMessage::default();
    if let Some(value) = params.get("AllowWrites") {
        input.allow_writes = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowWrites: {e}"))?,
        );
    }
    if let Some(value) = params.get("ConsumerIdentifier") {
        input.consumer_identifier = value.to_string();
    }
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AuthorizeEndpointAccess.
pub fn deserialize_authorize_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeEndpointAccessMessage, String> {
    let mut input = AuthorizeEndpointAccessMessage::default();
    if let Some(value) = params.get("Account") {
        input.account = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcIdentifier.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_ids = Some(VpcIdentifierList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AuthorizeSnapshotAccess.
pub fn deserialize_authorize_snapshot_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeSnapshotAccessMessage, String> {
    let mut input = AuthorizeSnapshotAccessMessage::default();
    if let Some(value) = params.get("AccountWithRestoreAccess") {
        input.account_with_restore_access = value.to_string();
    }
    if let Some(value) = params.get("SnapshotArn") {
        input.snapshot_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotClusterIdentifier") {
        input.snapshot_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchDeleteClusterSnapshots.
pub fn deserialize_batch_delete_cluster_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteClusterSnapshotsRequest, String> {
    let mut input = BatchDeleteClusterSnapshotsRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Identifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.DeleteClusterSnapshotMessage.{i}");
            match deserialize_delete_cluster_snapshot_message_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.identifiers = DeleteClusterSnapshotMessageList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchModifyClusterSnapshots.
pub fn deserialize_batch_modify_cluster_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchModifyClusterSnapshotsMessage, String> {
    let mut input = BatchModifyClusterSnapshotsMessage::default();
    if let Some(value) = params.get("Force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Force: {e}"))?,
        );
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SnapshotIdentifierList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.String.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.snapshot_identifier_list = SnapshotIdentifierList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CancelResize.
pub fn deserialize_cancel_resize_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CancelResizeMessage, String> {
    let mut input = CancelResizeMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CopyClusterSnapshot.
pub fn deserialize_copy_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CopyClusterSnapshotMessage, String> {
    let mut input = CopyClusterSnapshotMessage::default();
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("SourceSnapshotClusterIdentifier") {
        input.source_snapshot_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceSnapshotIdentifier") {
        input.source_snapshot_identifier = value.to_string();
    }
    if let Some(value) = params.get("TargetSnapshotIdentifier") {
        input.target_snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateAuthenticationProfile.
pub fn deserialize_create_authentication_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAuthenticationProfileMessage, String> {
    let mut input = CreateAuthenticationProfileMessage::default();
    if let Some(value) = params.get("AuthenticationProfileContent") {
        input.authentication_profile_content = value.to_string();
    }
    if let Some(value) = params.get("AuthenticationProfileName") {
        input.authentication_profile_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCluster.
pub fn deserialize_create_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterMessage, String> {
    let mut input = CreateClusterMessage::default();
    if let Some(value) = params.get("AdditionalInfo") {
        input.additional_info = Some(value.to_string());
    }
    if let Some(value) = params.get("AllowVersionUpgrade") {
        input.allow_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AquaConfigurationStatus") {
        input.aqua_configuration_status = Some(value.to_string());
    }
    if let Some(value) = params.get("AutomatedSnapshotRetentionPeriod") {
        input.automated_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AutomatedSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("AvailabilityZone") {
        input.availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("AvailabilityZoneRelocation") {
        input.availability_zone_relocation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AvailabilityZoneRelocation: {e}"))?,
        );
    }
    if let Some(value) = params.get("CatalogName") {
        input.catalog_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("ClusterParameterGroupName") {
        input.cluster_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ClusterSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ClusterSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cluster_security_groups = Some(ClusterSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterType") {
        input.cluster_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterVersion") {
        input.cluster_version = Some(value.to_string());
    }
    if let Some(value) = params.get("DBName") {
        input.d_b_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DefaultIamRoleArn") {
        input.default_iam_role_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("ElasticIp") {
        input.elastic_ip = Some(value.to_string());
    }
    if let Some(value) = params.get("Encrypted") {
        input.encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Encrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnhancedVpcRouting") {
        input.enhanced_vpc_routing = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnhancedVpcRouting: {e}"))?,
        );
    }
    if let Some(value) = params.get("ExtraComputeForAutomaticOptimization") {
        input.extra_compute_for_automatic_optimization =
            Some(value.parse::<bool>().map_err(|e| {
                format!("failed to parse ExtraComputeForAutomaticOptimization: {e}")
            })?);
    }
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "IamRoles".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.IamRoleArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.iam_roles = Some(IamRoleArnList { items });
        }
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("LoadSampleData") {
        input.load_sample_data = Some(value.to_string());
    }
    if let Some(value) = params.get("MaintenanceTrackName") {
        input.maintenance_track_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ManageMasterPassword") {
        input.manage_master_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterPasswordSecretKmsKeyId") {
        input.master_password_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUsername") {
        input.master_username = value.to_string();
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NodeType") {
        input.node_type = value.to_string();
    }
    if let Some(value) = params.get("NumberOfNodes") {
        input.number_of_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
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
    if let Some(value) = params.get("RedshiftIdcApplicationArn") {
        input.redshift_idc_application_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotScheduleIdentifier") {
        input.snapshot_schedule_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for CreateClusterParameterGroup.
pub fn deserialize_create_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterParameterGroupMessage, String> {
    let mut input = CreateClusterParameterGroupMessage::default();
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    if let Some(value) = params.get("ParameterGroupFamily") {
        input.parameter_group_family = value.to_string();
    }
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateClusterSecurityGroup.
pub fn deserialize_create_cluster_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterSecurityGroupMessage, String> {
    let mut input = CreateClusterSecurityGroupMessage::default();
    if let Some(value) = params.get("ClusterSecurityGroupName") {
        input.cluster_security_group_name = value.to_string();
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

/// Deserialize awsQuery request for CreateClusterSnapshot.
pub fn deserialize_create_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterSnapshotMessage, String> {
    let mut input = CreateClusterSnapshotMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = value.to_string();
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

/// Deserialize awsQuery request for CreateClusterSubnetGroup.
pub fn deserialize_create_cluster_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterSubnetGroupMessage, String> {
    let mut input = CreateClusterSubnetGroupMessage::default();
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
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

/// Deserialize awsQuery request for CreateCustomDomainAssociation.
pub fn deserialize_create_custom_domain_association_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomDomainAssociationMessage, String> {
    let mut input = CreateCustomDomainAssociationMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("CustomDomainCertificateArn") {
        input.custom_domain_certificate_arn = value.to_string();
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateEndpointAccess.
pub fn deserialize_create_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateEndpointAccessMessage, String> {
    let mut input = CreateEndpointAccessMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("EndpointName") {
        input.endpoint_name = value.to_string();
    }
    if let Some(value) = params.get("ResourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    if let Some(value) = params.get("SubnetGroupName") {
        input.subnet_group_name = value.to_string();
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
    if let Some(value) = params.get("Severity") {
        input.severity = Some(value.to_string());
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

/// Deserialize awsQuery request for CreateHsmClientCertificate.
pub fn deserialize_create_hsm_client_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateHsmClientCertificateMessage, String> {
    let mut input = CreateHsmClientCertificateMessage::default();
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = value.to_string();
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

/// Deserialize awsQuery request for CreateHsmConfiguration.
pub fn deserialize_create_hsm_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateHsmConfigurationMessage, String> {
    let mut input = CreateHsmConfigurationMessage::default();
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = value.to_string();
    }
    if let Some(value) = params.get("HsmIpAddress") {
        input.hsm_ip_address = value.to_string();
    }
    if let Some(value) = params.get("HsmPartitionName") {
        input.hsm_partition_name = value.to_string();
    }
    if let Some(value) = params.get("HsmPartitionPassword") {
        input.hsm_partition_password = value.to_string();
    }
    if let Some(value) = params.get("HsmServerPublicCertificate") {
        input.hsm_server_public_certificate = value.to_string();
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
        let list_prefix = "TagList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.Tag.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_list = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TargetArn") {
        input.target_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateRedshiftIdcApplication.
pub fn deserialize_create_redshift_idc_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateRedshiftIdcApplicationMessage, String> {
    let mut input = CreateRedshiftIdcApplicationMessage::default();
    if let Some(value) = params.get("ApplicationType") {
        input.application_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AuthorizedTokenIssuerList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_authorized_token_issuer_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.authorized_token_issuer_list = Some(AuthorizedTokenIssuerList { items });
        }
    }
    if let Some(value) = params.get("IamRoleArn") {
        input.iam_role_arn = value.to_string();
    }
    if let Some(value) = params.get("IdcDisplayName") {
        input.idc_display_name = value.to_string();
    }
    if let Some(value) = params.get("IdcInstanceArn") {
        input.idc_instance_arn = value.to_string();
    }
    if let Some(value) = params.get("IdentityNamespace") {
        input.identity_namespace = Some(value.to_string());
    }
    if let Some(value) = params.get("RedshiftIdcApplicationName") {
        input.redshift_idc_application_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ServiceIntegrations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_service_integrations_union_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.service_integrations = Some(ServiceIntegrationList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SsoTagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.sso_tag_keys = Some(TagKeyList { items });
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

/// Deserialize awsQuery request for CreateScheduledAction.
pub fn deserialize_create_scheduled_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateScheduledActionMessage, String> {
    let mut input = CreateScheduledActionMessage::default();
    if let Some(value) = params.get("Enable") {
        input.enable = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enable: {e}"))?,
        );
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("IamRole") {
        input.iam_role = value.to_string();
    }
    if let Some(value) = params.get("Schedule") {
        input.schedule = value.to_string();
    }
    if let Some(value) = params.get("ScheduledActionDescription") {
        input.scheduled_action_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = value.to_string();
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    if let Some(val) = deserialize_scheduled_action_type_from_query(params, "TargetAction")? {
        input.target_action = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateSnapshotCopyGrant.
pub fn deserialize_create_snapshot_copy_grant_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateSnapshotCopyGrantMessage, String> {
    let mut input = CreateSnapshotCopyGrantMessage::default();
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotCopyGrantName") {
        input.snapshot_copy_grant_name = value.to_string();
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

/// Deserialize awsQuery request for CreateSnapshotSchedule.
pub fn deserialize_create_snapshot_schedule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateSnapshotScheduleMessage, String> {
    let mut input = CreateSnapshotScheduleMessage::default();
    if let Some(value) = params.get("DryRun") {
        input.dry_run = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DryRun: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextInvocations") {
        input.next_invocations = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NextInvocations: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScheduleDefinitions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ScheduleDefinition.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.schedule_definitions = Some(ScheduleDefinitionList { items });
        }
    }
    if let Some(value) = params.get("ScheduleDescription") {
        input.schedule_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ScheduleIdentifier") {
        input.schedule_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for CreateTags.
pub fn deserialize_create_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTagsMessage, String> {
    let mut input = CreateTagsMessage::default();
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

/// Deserialize awsQuery request for CreateUsageLimit.
pub fn deserialize_create_usage_limit_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateUsageLimitMessage, String> {
    let mut input = CreateUsageLimitMessage::default();
    if let Some(value) = params.get("Amount") {
        input.amount = value
            .parse::<i64>()
            .map_err(|e| format!("failed to parse Amount: {e}"))?;
    }
    if let Some(value) = params.get("BreachAction") {
        input.breach_action = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("FeatureType") {
        input.feature_type = value.to_string();
    }
    if let Some(value) = params.get("LimitType") {
        input.limit_type = value.to_string();
    }
    if let Some(value) = params.get("Period") {
        input.period = Some(value.to_string());
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

/// Deserialize awsQuery request for DeauthorizeDataShare.
pub fn deserialize_deauthorize_data_share_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeauthorizeDataShareMessage, String> {
    let mut input = DeauthorizeDataShareMessage::default();
    if let Some(value) = params.get("ConsumerIdentifier") {
        input.consumer_identifier = value.to_string();
    }
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAuthenticationProfile.
pub fn deserialize_delete_authentication_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAuthenticationProfileMessage, String> {
    let mut input = DeleteAuthenticationProfileMessage::default();
    if let Some(value) = params.get("AuthenticationProfileName") {
        input.authentication_profile_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCluster.
pub fn deserialize_delete_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterMessage, String> {
    let mut input = DeleteClusterMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("FinalClusterSnapshotIdentifier") {
        input.final_cluster_snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("FinalClusterSnapshotRetentionPeriod") {
        input.final_cluster_snapshot_retention_period =
            Some(value.parse::<i32>().map_err(|e| {
                format!("failed to parse FinalClusterSnapshotRetentionPeriod: {e}")
            })?);
    }
    if let Some(value) = params.get("SkipFinalClusterSnapshot") {
        input.skip_final_cluster_snapshot = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipFinalClusterSnapshot: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteClusterParameterGroup.
pub fn deserialize_delete_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterParameterGroupMessage, String> {
    let mut input = DeleteClusterParameterGroupMessage::default();
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteClusterSecurityGroup.
pub fn deserialize_delete_cluster_security_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterSecurityGroupMessage, String> {
    let mut input = DeleteClusterSecurityGroupMessage::default();
    if let Some(value) = params.get("ClusterSecurityGroupName") {
        input.cluster_security_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteClusterSnapshot.
pub fn deserialize_delete_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterSnapshotMessage, String> {
    let mut input = DeleteClusterSnapshotMessage::default();
    if let Some(value) = params.get("SnapshotClusterIdentifier") {
        input.snapshot_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteClusterSubnetGroup.
pub fn deserialize_delete_cluster_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterSubnetGroupMessage, String> {
    let mut input = DeleteClusterSubnetGroupMessage::default();
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCustomDomainAssociation.
pub fn deserialize_delete_custom_domain_association_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomDomainAssociationMessage, String> {
    let mut input = DeleteCustomDomainAssociationMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteEndpointAccess.
pub fn deserialize_delete_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteEndpointAccessMessage, String> {
    let mut input = DeleteEndpointAccessMessage::default();
    if let Some(value) = params.get("EndpointName") {
        input.endpoint_name = value.to_string();
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

/// Deserialize awsQuery request for DeleteHsmClientCertificate.
pub fn deserialize_delete_hsm_client_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteHsmClientCertificateMessage, String> {
    let mut input = DeleteHsmClientCertificateMessage::default();
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteHsmConfiguration.
pub fn deserialize_delete_hsm_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteHsmConfigurationMessage, String> {
    let mut input = DeleteHsmConfigurationMessage::default();
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteIntegration.
pub fn deserialize_delete_integration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntegrationMessage, String> {
    let mut input = DeleteIntegrationMessage::default();
    if let Some(value) = params.get("IntegrationArn") {
        input.integration_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePartner.
pub fn deserialize_delete_partner_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PartnerIntegrationInputMessage, String> {
    let mut input = PartnerIntegrationInputMessage::default();
    if let Some(value) = params.get("AccountId") {
        input.account_id = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = value.to_string();
    }
    if let Some(value) = params.get("PartnerName") {
        input.partner_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteRedshiftIdcApplication.
pub fn deserialize_delete_redshift_idc_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteRedshiftIdcApplicationMessage, String> {
    let mut input = DeleteRedshiftIdcApplicationMessage::default();
    if let Some(value) = params.get("RedshiftIdcApplicationArn") {
        input.redshift_idc_application_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteResourcePolicy.
pub fn deserialize_delete_resource_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyMessage, String> {
    let mut input = DeleteResourcePolicyMessage::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteScheduledAction.
pub fn deserialize_delete_scheduled_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduledActionMessage, String> {
    let mut input = DeleteScheduledActionMessage::default();
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSnapshotCopyGrant.
pub fn deserialize_delete_snapshot_copy_grant_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSnapshotCopyGrantMessage, String> {
    let mut input = DeleteSnapshotCopyGrantMessage::default();
    if let Some(value) = params.get("SnapshotCopyGrantName") {
        input.snapshot_copy_grant_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSnapshotSchedule.
pub fn deserialize_delete_snapshot_schedule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSnapshotScheduleMessage, String> {
    let mut input = DeleteSnapshotScheduleMessage::default();
    if let Some(value) = params.get("ScheduleIdentifier") {
        input.schedule_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTags.
pub fn deserialize_delete_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTagsMessage, String> {
    let mut input = DeleteTagsMessage::default();
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = TagKeyList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUsageLimit.
pub fn deserialize_delete_usage_limit_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUsageLimitMessage, String> {
    let mut input = DeleteUsageLimitMessage::default();
    if let Some(value) = params.get("UsageLimitId") {
        input.usage_limit_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeregisterNamespace.
pub fn deserialize_deregister_namespace_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeregisterNamespaceInputMessage, String> {
    let mut input = DeregisterNamespaceInputMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ConsumerIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.consumer_identifiers = ConsumerIdentifierList { items };
        }
    }
    if let Some(val) =
        deserialize_namespace_identifier_union_from_query(params, "NamespaceIdentifier")?
    {
        input.namespace_identifier = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAccountAttributes.
pub fn deserialize_describe_account_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountAttributesMessage, String> {
    let mut input = DescribeAccountAttributesMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AttributeNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.AttributeName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attribute_names = Some(AttributeNameList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAuthenticationProfiles.
pub fn deserialize_describe_authentication_profiles_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAuthenticationProfilesMessage, String> {
    let mut input = DescribeAuthenticationProfilesMessage::default();
    if let Some(value) = params.get("AuthenticationProfileName") {
        input.authentication_profile_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterDbRevisions.
pub fn deserialize_describe_cluster_db_revisions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterDbRevisionsMessage, String> {
    let mut input = DescribeClusterDbRevisionsMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeClusterParameterGroups.
pub fn deserialize_describe_cluster_parameter_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterParameterGroupsMessage, String> {
    let mut input = DescribeClusterParameterGroupsMessage::default();
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
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterParameters.
pub fn deserialize_describe_cluster_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterParametersMessage, String> {
    let mut input = DescribeClusterParametersMessage::default();
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
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = value.to_string();
    }
    if let Some(value) = params.get("Source") {
        input.source = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterSecurityGroups.
pub fn deserialize_describe_cluster_security_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterSecurityGroupsMessage, String> {
    let mut input = DescribeClusterSecurityGroupsMessage::default();
    if let Some(value) = params.get("ClusterSecurityGroupName") {
        input.cluster_security_group_name = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterSnapshots.
pub fn deserialize_describe_cluster_snapshots_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterSnapshotsMessage, String> {
    let mut input = DescribeClusterSnapshotsMessage::default();
    if let Some(value) = params.get("ClusterExists") {
        input.cluster_exists = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ClusterExists: {e}"))?,
        );
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("OwnerAccount") {
        input.owner_account = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotArn") {
        input.snapshot_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotType") {
        input.snapshot_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SortingEntities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.SnapshotSortingEntity.{i}");
            match deserialize_snapshot_sorting_entity_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.sorting_entities = Some(SnapshotSortingEntityList { items });
        }
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterSubnetGroups.
pub fn deserialize_describe_cluster_subnet_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterSubnetGroupsMessage, String> {
    let mut input = DescribeClusterSubnetGroupsMessage::default();
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeClusterTracks.
pub fn deserialize_describe_cluster_tracks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterTracksMessage, String> {
    let mut input = DescribeClusterTracksMessage::default();
    if let Some(value) = params.get("MaintenanceTrackName") {
        input.maintenance_track_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeClusterVersions.
pub fn deserialize_describe_cluster_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterVersionsMessage, String> {
    let mut input = DescribeClusterVersionsMessage::default();
    if let Some(value) = params.get("ClusterParameterGroupFamily") {
        input.cluster_parameter_group_family = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterVersion") {
        input.cluster_version = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeClusters.
pub fn deserialize_describe_clusters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeClustersMessage, String> {
    let mut input = DescribeClustersMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCustomDomainAssociations.
pub fn deserialize_describe_custom_domain_associations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCustomDomainAssociationsMessage, String> {
    let mut input = DescribeCustomDomainAssociationsMessage::default();
    if let Some(value) = params.get("CustomDomainCertificateArn") {
        input.custom_domain_certificate_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDataShares.
pub fn deserialize_describe_data_shares_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSharesMessage, String> {
    let mut input = DescribeDataSharesMessage::default();
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeDataSharesForConsumer.
pub fn deserialize_describe_data_shares_for_consumer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSharesForConsumerMessage, String> {
    let mut input = DescribeDataSharesForConsumerMessage::default();
    if let Some(value) = params.get("ConsumerArn") {
        input.consumer_arn = Some(value.to_string());
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
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDataSharesForProducer.
pub fn deserialize_describe_data_shares_for_producer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDataSharesForProducerMessage, String> {
    let mut input = DescribeDataSharesForProducerMessage::default();
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
    if let Some(value) = params.get("ProducerArn") {
        input.producer_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeDefaultClusterParameters.
pub fn deserialize_describe_default_cluster_parameters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeDefaultClusterParametersMessage, String> {
    let mut input = DescribeDefaultClusterParametersMessage::default();
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
    if let Some(value) = params.get("ParameterGroupFamily") {
        input.parameter_group_family = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEndpointAccess.
pub fn deserialize_describe_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEndpointAccessMessage, String> {
    let mut input = DescribeEndpointAccessMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("EndpointName") {
        input.endpoint_name = Some(value.to_string());
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
    if let Some(value) = params.get("ResourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    if let Some(value) = params.get("VpcId") {
        input.vpc_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEndpointAuthorization.
pub fn deserialize_describe_endpoint_authorization_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEndpointAuthorizationMessage, String> {
    let mut input = DescribeEndpointAuthorizationMessage::default();
    if let Some(value) = params.get("Account") {
        input.account = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("Grantee") {
        input.grantee = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Grantee: {e}"))?,
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

/// Deserialize awsQuery request for DescribeEventCategories.
pub fn deserialize_describe_event_categories_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventCategoriesMessage, String> {
    let mut input = DescribeEventCategoriesMessage::default();
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
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
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

/// Deserialize awsQuery request for DescribeHsmClientCertificates.
pub fn deserialize_describe_hsm_client_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeHsmClientCertificatesMessage, String> {
    let mut input = DescribeHsmClientCertificatesMessage::default();
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeHsmConfigurations.
pub fn deserialize_describe_hsm_configurations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeHsmConfigurationsMessage, String> {
    let mut input = DescribeHsmConfigurationsMessage::default();
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeInboundIntegrations.
pub fn deserialize_describe_inbound_integrations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeInboundIntegrationsMessage, String> {
    let mut input = DescribeInboundIntegrationsMessage::default();
    if let Some(value) = params.get("IntegrationArn") {
        input.integration_arn = Some(value.to_string());
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
    if let Some(value) = params.get("TargetArn") {
        input.target_arn = Some(value.to_string());
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
            let item_key = format!("{list_prefix}.DescribeIntegrationsFilter.{i}");
            match deserialize_describe_integrations_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(DescribeIntegrationsFilterList { items });
        }
    }
    if let Some(value) = params.get("IntegrationArn") {
        input.integration_arn = Some(value.to_string());
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

/// Deserialize awsQuery request for DescribeLoggingStatus.
pub fn deserialize_describe_logging_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoggingStatusMessage, String> {
    let mut input = DescribeLoggingStatusMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeNodeConfigurationOptions.
pub fn deserialize_describe_node_configuration_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeNodeConfigurationOptionsMessage, String> {
    let mut input = DescribeNodeConfigurationOptionsMessage::default();
    if let Some(value) = params.get("ActionType") {
        input.action_type = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.NodeConfigurationOptionsFilter.{i}");
            match deserialize_node_configuration_options_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(NodeConfigurationOptionsFilterList { items });
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
    if let Some(value) = params.get("OwnerAccount") {
        input.owner_account = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotArn") {
        input.snapshot_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeOrderableClusterOptions.
pub fn deserialize_describe_orderable_cluster_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrderableClusterOptionsMessage, String> {
    let mut input = DescribeOrderableClusterOptionsMessage::default();
    if let Some(value) = params.get("ClusterVersion") {
        input.cluster_version = Some(value.to_string());
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
    if let Some(value) = params.get("NodeType") {
        input.node_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribePartners.
pub fn deserialize_describe_partners_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribePartnersInputMessage, String> {
    let mut input = DescribePartnersInputMessage::default();
    if let Some(value) = params.get("AccountId") {
        input.account_id = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PartnerName") {
        input.partner_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeRedshiftIdcApplications.
pub fn deserialize_describe_redshift_idc_applications_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeRedshiftIdcApplicationsMessage, String> {
    let mut input = DescribeRedshiftIdcApplicationsMessage::default();
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
    if let Some(value) = params.get("RedshiftIdcApplicationArn") {
        input.redshift_idc_application_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedNodeExchangeStatus.
pub fn deserialize_describe_reserved_node_exchange_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedNodeExchangeStatusInputMessage, String> {
    let mut input = DescribeReservedNodeExchangeStatusInputMessage::default();
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
    if let Some(value) = params.get("ReservedNodeExchangeRequestId") {
        input.reserved_node_exchange_request_id = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedNodeOfferings.
pub fn deserialize_describe_reserved_node_offerings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedNodeOfferingsMessage, String> {
    let mut input = DescribeReservedNodeOfferingsMessage::default();
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
    if let Some(value) = params.get("ReservedNodeOfferingId") {
        input.reserved_node_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReservedNodes.
pub fn deserialize_describe_reserved_nodes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedNodesMessage, String> {
    let mut input = DescribeReservedNodesMessage::default();
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
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeResize.
pub fn deserialize_describe_resize_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeResizeMessage, String> {
    let mut input = DescribeResizeMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeScheduledActions.
pub fn deserialize_describe_scheduled_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeScheduledActionsMessage, String> {
    let mut input = DescribeScheduledActionsMessage::default();
    if let Some(value) = params.get("Active") {
        input.active = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Active: {e}"))?,
        );
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ScheduledActionFilter.{i}");
            match deserialize_scheduled_action_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(ScheduledActionFilterList { items });
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
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = Some(value.to_string());
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetActionType") {
        input.target_action_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeSnapshotCopyGrants.
pub fn deserialize_describe_snapshot_copy_grants_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeSnapshotCopyGrantsMessage, String> {
    let mut input = DescribeSnapshotCopyGrantsMessage::default();
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
    if let Some(value) = params.get("SnapshotCopyGrantName") {
        input.snapshot_copy_grant_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeSnapshotSchedules.
pub fn deserialize_describe_snapshot_schedules_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeSnapshotSchedulesMessage, String> {
    let mut input = DescribeSnapshotSchedulesMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("ScheduleIdentifier") {
        input.schedule_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTableRestoreStatus.
pub fn deserialize_describe_table_restore_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTableRestoreStatusMessage, String> {
    let mut input = DescribeTableRestoreStatusMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("TableRestoreRequestId") {
        input.table_restore_request_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTags.
pub fn deserialize_describe_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTagsMessage, String> {
    let mut input = DescribeTagsMessage::default();
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
    if let Some(value) = params.get("ResourceName") {
        input.resource_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeUsageLimits.
pub fn deserialize_describe_usage_limits_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeUsageLimitsMessage, String> {
    let mut input = DescribeUsageLimitsMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("FeatureType") {
        input.feature_type = Some(value.to_string());
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
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagKey.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = Some(TagKeyList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagValues".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.TagValue.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_values = Some(TagValueList { items });
        }
    }
    if let Some(value) = params.get("UsageLimitId") {
        input.usage_limit_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableLogging.
pub fn deserialize_disable_logging_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableLoggingMessage, String> {
    let mut input = DisableLoggingMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableSnapshotCopy.
pub fn deserialize_disable_snapshot_copy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableSnapshotCopyMessage, String> {
    let mut input = DisableSnapshotCopyMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisassociateDataShareConsumer.
pub fn deserialize_disassociate_data_share_consumer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisassociateDataShareConsumerMessage, String> {
    let mut input = DisassociateDataShareConsumerMessage::default();
    if let Some(value) = params.get("ConsumerArn") {
        input.consumer_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("ConsumerRegion") {
        input.consumer_region = Some(value.to_string());
    }
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = value.to_string();
    }
    if let Some(value) = params.get("DisassociateEntireAccount") {
        input.disassociate_entire_account = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisassociateEntireAccount: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableLogging.
pub fn deserialize_enable_logging_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableLoggingMessage, String> {
    let mut input = EnableLoggingMessage::default();
    if let Some(value) = params.get("BucketName") {
        input.bucket_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("LogDestinationType") {
        input.log_destination_type = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LogExports".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.log_exports = Some(LogTypeList { items });
        }
    }
    if let Some(value) = params.get("S3KeyPrefix") {
        input.s3_key_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableSnapshotCopy.
pub fn deserialize_enable_snapshot_copy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableSnapshotCopyMessage, String> {
    let mut input = EnableSnapshotCopyMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DestinationRegion") {
        input.destination_region = value.to_string();
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("RetentionPeriod") {
        input.retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse RetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotCopyGrantName") {
        input.snapshot_copy_grant_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for FailoverPrimaryCompute.
pub fn deserialize_failover_primary_compute_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<FailoverPrimaryComputeInputMessage, String> {
    let mut input = FailoverPrimaryComputeInputMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetClusterCredentials.
pub fn deserialize_get_cluster_credentials_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetClusterCredentialsMessage, String> {
    let mut input = GetClusterCredentialsMessage::default();
    if let Some(value) = params.get("AutoCreate") {
        input.auto_create = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoCreate: {e}"))?,
        );
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DbGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.DbGroup.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.db_groups = Some(DbGroupList { items });
        }
    }
    if let Some(value) = params.get("DbName") {
        input.db_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DbUser") {
        input.db_user = value.to_string();
    }
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetClusterCredentialsWithIAM.
pub fn deserialize_get_cluster_credentials_with_i_a_m_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetClusterCredentialsWithIAMMessage, String> {
    let mut input = GetClusterCredentialsWithIAMMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DbName") {
        input.db_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityCenterAuthToken.
pub fn deserialize_get_identity_center_auth_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityCenterAuthTokenRequest, String> {
    let mut input = GetIdentityCenterAuthTokenRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ClusterIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ClusterIdentifier.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cluster_ids = ClusterIdentifierList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetReservedNodeExchangeConfigurationOptions.
pub fn deserialize_get_reserved_node_exchange_configuration_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetReservedNodeExchangeConfigurationOptionsInputMessage, String> {
    let mut input = GetReservedNodeExchangeConfigurationOptionsInputMessage::default();
    if let Some(value) = params.get("ActionType") {
        input.action_type = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetReservedNodeExchangeOfferings.
pub fn deserialize_get_reserved_node_exchange_offerings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetReservedNodeExchangeOfferingsInputMessage, String> {
    let mut input = GetReservedNodeExchangeOfferingsInputMessage::default();
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
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetResourcePolicy.
pub fn deserialize_get_resource_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyMessage, String> {
    let mut input = GetResourcePolicyMessage::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListRecommendations.
pub fn deserialize_list_recommendations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListRecommendationsMessage, String> {
    let mut input = ListRecommendationsMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
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
    if let Some(value) = params.get("NamespaceArn") {
        input.namespace_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyAquaConfiguration.
pub fn deserialize_modify_aqua_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyAquaInputMessage, String> {
    let mut input = ModifyAquaInputMessage::default();
    if let Some(value) = params.get("AquaConfigurationStatus") {
        input.aqua_configuration_status = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyAuthenticationProfile.
pub fn deserialize_modify_authentication_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyAuthenticationProfileMessage, String> {
    let mut input = ModifyAuthenticationProfileMessage::default();
    if let Some(value) = params.get("AuthenticationProfileContent") {
        input.authentication_profile_content = value.to_string();
    }
    if let Some(value) = params.get("AuthenticationProfileName") {
        input.authentication_profile_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCluster.
pub fn deserialize_modify_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterMessage, String> {
    let mut input = ModifyClusterMessage::default();
    if let Some(value) = params.get("AllowVersionUpgrade") {
        input.allow_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AutomatedSnapshotRetentionPeriod") {
        input.automated_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AutomatedSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("AvailabilityZone") {
        input.availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("AvailabilityZoneRelocation") {
        input.availability_zone_relocation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AvailabilityZoneRelocation: {e}"))?,
        );
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("ClusterParameterGroupName") {
        input.cluster_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ClusterSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ClusterSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cluster_security_groups = Some(ClusterSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("ClusterType") {
        input.cluster_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterVersion") {
        input.cluster_version = Some(value.to_string());
    }
    if let Some(value) = params.get("ElasticIp") {
        input.elastic_ip = Some(value.to_string());
    }
    if let Some(value) = params.get("Encrypted") {
        input.encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Encrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnhancedVpcRouting") {
        input.enhanced_vpc_routing = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnhancedVpcRouting: {e}"))?,
        );
    }
    if let Some(value) = params.get("ExtraComputeForAutomaticOptimization") {
        input.extra_compute_for_automatic_optimization =
            Some(value.parse::<bool>().map_err(|e| {
                format!("failed to parse ExtraComputeForAutomaticOptimization: {e}")
            })?);
    }
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MaintenanceTrackName") {
        input.maintenance_track_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ManageMasterPassword") {
        input.manage_master_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterPasswordSecretKmsKeyId") {
        input.master_password_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MasterUserPassword") {
        input.master_user_password = Some(value.to_string());
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NewClusterIdentifier") {
        input.new_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("NodeType") {
        input.node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NumberOfNodes") {
        input.number_of_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
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

/// Deserialize awsQuery request for ModifyClusterDbRevision.
pub fn deserialize_modify_cluster_db_revision_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterDbRevisionMessage, String> {
    let mut input = ModifyClusterDbRevisionMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("RevisionTarget") {
        input.revision_target = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyClusterIamRoles.
pub fn deserialize_modify_cluster_iam_roles_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterIamRolesMessage, String> {
    let mut input = ModifyClusterIamRolesMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AddIamRoles".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.IamRoleArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.add_iam_roles = Some(IamRoleArnList { items });
        }
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DefaultIamRoleArn") {
        input.default_iam_role_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RemoveIamRoles".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.IamRoleArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.remove_iam_roles = Some(IamRoleArnList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyClusterMaintenance.
pub fn deserialize_modify_cluster_maintenance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterMaintenanceMessage, String> {
    let mut input = ModifyClusterMaintenanceMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DeferMaintenance") {
        input.defer_maintenance = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeferMaintenance: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeferMaintenanceDuration") {
        input.defer_maintenance_duration = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DeferMaintenanceDuration: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeferMaintenanceEndTime") {
        input.defer_maintenance_end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("DeferMaintenanceIdentifier") {
        input.defer_maintenance_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("DeferMaintenanceStartTime") {
        input.defer_maintenance_start_time = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyClusterParameterGroup.
pub fn deserialize_modify_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterParameterGroupMessage, String> {
    let mut input = ModifyClusterParameterGroupMessage::default();
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = value.to_string();
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

/// Deserialize awsQuery request for ModifyClusterSnapshot.
pub fn deserialize_modify_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterSnapshotMessage, String> {
    let mut input = ModifyClusterSnapshotMessage::default();
    if let Some(value) = params.get("Force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Force: {e}"))?,
        );
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyClusterSnapshotSchedule.
pub fn deserialize_modify_cluster_snapshot_schedule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterSnapshotScheduleMessage, String> {
    let mut input = ModifyClusterSnapshotScheduleMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DisassociateSchedule") {
        input.disassociate_schedule = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisassociateSchedule: {e}"))?,
        );
    }
    if let Some(value) = params.get("ScheduleIdentifier") {
        input.schedule_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyClusterSubnetGroup.
pub fn deserialize_modify_cluster_subnet_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyClusterSubnetGroupMessage, String> {
    let mut input = ModifyClusterSubnetGroupMessage::default();
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
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

/// Deserialize awsQuery request for ModifyCustomDomainAssociation.
pub fn deserialize_modify_custom_domain_association_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCustomDomainAssociationMessage, String> {
    let mut input = ModifyCustomDomainAssociationMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("CustomDomainCertificateArn") {
        input.custom_domain_certificate_arn = value.to_string();
    }
    if let Some(value) = params.get("CustomDomainName") {
        input.custom_domain_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyEndpointAccess.
pub fn deserialize_modify_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyEndpointAccessMessage, String> {
    let mut input = ModifyEndpointAccessMessage::default();
    if let Some(value) = params.get("EndpointName") {
        input.endpoint_name = value.to_string();
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
    if let Some(value) = params.get("Severity") {
        input.severity = Some(value.to_string());
    }
    if let Some(value) = params.get("SnsTopicArn") {
        input.sns_topic_arn = Some(value.to_string());
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
    Ok(input)
}

/// Deserialize awsQuery request for ModifyIntegration.
pub fn deserialize_modify_integration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyIntegrationMessage, String> {
    let mut input = ModifyIntegrationMessage::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("IntegrationArn") {
        input.integration_arn = value.to_string();
    }
    if let Some(value) = params.get("IntegrationName") {
        input.integration_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyLakehouseConfiguration.
pub fn deserialize_modify_lakehouse_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyLakehouseConfigurationMessage, String> {
    let mut input = ModifyLakehouseConfigurationMessage::default();
    if let Some(value) = params.get("CatalogName") {
        input.catalog_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DryRun") {
        input.dry_run = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DryRun: {e}"))?,
        );
    }
    if let Some(value) = params.get("LakehouseIdcApplicationArn") {
        input.lakehouse_idc_application_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("LakehouseIdcRegistration") {
        input.lakehouse_idc_registration = Some(value.to_string());
    }
    if let Some(value) = params.get("LakehouseRegistration") {
        input.lakehouse_registration = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyRedshiftIdcApplication.
pub fn deserialize_modify_redshift_idc_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyRedshiftIdcApplicationMessage, String> {
    let mut input = ModifyRedshiftIdcApplicationMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AuthorizedTokenIssuerList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_authorized_token_issuer_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.authorized_token_issuer_list = Some(AuthorizedTokenIssuerList { items });
        }
    }
    if let Some(value) = params.get("IamRoleArn") {
        input.iam_role_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("IdcDisplayName") {
        input.idc_display_name = Some(value.to_string());
    }
    if let Some(value) = params.get("IdentityNamespace") {
        input.identity_namespace = Some(value.to_string());
    }
    if let Some(value) = params.get("RedshiftIdcApplicationArn") {
        input.redshift_idc_application_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ServiceIntegrations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_service_integrations_union_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.service_integrations = Some(ServiceIntegrationList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyScheduledAction.
pub fn deserialize_modify_scheduled_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyScheduledActionMessage, String> {
    let mut input = ModifyScheduledActionMessage::default();
    if let Some(value) = params.get("Enable") {
        input.enable = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enable: {e}"))?,
        );
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("IamRole") {
        input.iam_role = Some(value.to_string());
    }
    if let Some(value) = params.get("Schedule") {
        input.schedule = Some(value.to_string());
    }
    if let Some(value) = params.get("ScheduledActionDescription") {
        input.scheduled_action_description = Some(value.to_string());
    }
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = value.to_string();
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    if let Some(val) = deserialize_scheduled_action_type_from_query(params, "TargetAction")? {
        input.target_action = Some(val);
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifySnapshotCopyRetentionPeriod.
pub fn deserialize_modify_snapshot_copy_retention_period_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifySnapshotCopyRetentionPeriodMessage, String> {
    let mut input = ModifySnapshotCopyRetentionPeriodMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("Manual") {
        input.manual = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Manual: {e}"))?,
        );
    }
    if let Some(value) = params.get("RetentionPeriod") {
        input.retention_period = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse RetentionPeriod: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifySnapshotSchedule.
pub fn deserialize_modify_snapshot_schedule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifySnapshotScheduleMessage, String> {
    let mut input = ModifySnapshotScheduleMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ScheduleDefinitions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ScheduleDefinition.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.schedule_definitions = ScheduleDefinitionList { items };
        }
    }
    if let Some(value) = params.get("ScheduleIdentifier") {
        input.schedule_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyUsageLimit.
pub fn deserialize_modify_usage_limit_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyUsageLimitMessage, String> {
    let mut input = ModifyUsageLimitMessage::default();
    if let Some(value) = params.get("Amount") {
        input.amount = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse Amount: {e}"))?,
        );
    }
    if let Some(value) = params.get("BreachAction") {
        input.breach_action = Some(value.to_string());
    }
    if let Some(value) = params.get("UsageLimitId") {
        input.usage_limit_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PauseCluster.
pub fn deserialize_pause_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PauseClusterMessage, String> {
    let mut input = PauseClusterMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PurchaseReservedNodeOffering.
pub fn deserialize_purchase_reserved_node_offering_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PurchaseReservedNodeOfferingMessage, String> {
    let mut input = PurchaseReservedNodeOfferingMessage::default();
    if let Some(value) = params.get("NodeCount") {
        input.node_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NodeCount: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReservedNodeOfferingId") {
        input.reserved_node_offering_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutResourcePolicy.
pub fn deserialize_put_resource_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyMessage, String> {
    let mut input = PutResourcePolicyMessage::default();
    if let Some(value) = params.get("Policy") {
        input.policy = value.to_string();
    }
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RebootCluster.
pub fn deserialize_reboot_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebootClusterMessage, String> {
    let mut input = RebootClusterMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterNamespace.
pub fn deserialize_register_namespace_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterNamespaceInputMessage, String> {
    let mut input = RegisterNamespaceInputMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ConsumerIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.consumer_identifiers = ConsumerIdentifierList { items };
        }
    }
    if let Some(val) =
        deserialize_namespace_identifier_union_from_query(params, "NamespaceIdentifier")?
    {
        input.namespace_identifier = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for RejectDataShare.
pub fn deserialize_reject_data_share_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RejectDataShareMessage, String> {
    let mut input = RejectDataShareMessage::default();
    if let Some(value) = params.get("DataShareArn") {
        input.data_share_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResetClusterParameterGroup.
pub fn deserialize_reset_cluster_parameter_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResetClusterParameterGroupMessage, String> {
    let mut input = ResetClusterParameterGroupMessage::default();
    if let Some(value) = params.get("ParameterGroupName") {
        input.parameter_group_name = value.to_string();
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

/// Deserialize awsQuery request for ResizeCluster.
pub fn deserialize_resize_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResizeClusterMessage, String> {
    let mut input = ResizeClusterMessage::default();
    if let Some(value) = params.get("Classic") {
        input.classic = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Classic: {e}"))?,
        );
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("ClusterType") {
        input.cluster_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NodeType") {
        input.node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NumberOfNodes") {
        input.number_of_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetReservedNodeOfferingId") {
        input.target_reserved_node_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RestoreFromClusterSnapshot.
pub fn deserialize_restore_from_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreFromClusterSnapshotMessage, String> {
    let mut input = RestoreFromClusterSnapshotMessage::default();
    if let Some(value) = params.get("AdditionalInfo") {
        input.additional_info = Some(value.to_string());
    }
    if let Some(value) = params.get("AllowVersionUpgrade") {
        input.allow_version_upgrade = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowVersionUpgrade: {e}"))?,
        );
    }
    if let Some(value) = params.get("AquaConfigurationStatus") {
        input.aqua_configuration_status = Some(value.to_string());
    }
    if let Some(value) = params.get("AutomatedSnapshotRetentionPeriod") {
        input.automated_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse AutomatedSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("AvailabilityZone") {
        input.availability_zone = Some(value.to_string());
    }
    if let Some(value) = params.get("AvailabilityZoneRelocation") {
        input.availability_zone_relocation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AvailabilityZoneRelocation: {e}"))?,
        );
    }
    if let Some(value) = params.get("CatalogName") {
        input.catalog_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("ClusterParameterGroupName") {
        input.cluster_parameter_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ClusterSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.ClusterSecurityGroupName.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.cluster_security_groups = Some(ClusterSecurityGroupNameList { items });
        }
    }
    if let Some(value) = params.get("ClusterSubnetGroupName") {
        input.cluster_subnet_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("DefaultIamRoleArn") {
        input.default_iam_role_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("ElasticIp") {
        input.elastic_ip = Some(value.to_string());
    }
    if let Some(value) = params.get("Encrypted") {
        input.encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Encrypted: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnhancedVpcRouting") {
        input.enhanced_vpc_routing = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnhancedVpcRouting: {e}"))?,
        );
    }
    if let Some(value) = params.get("HsmClientCertificateIdentifier") {
        input.hsm_client_certificate_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("HsmConfigurationIdentifier") {
        input.hsm_configuration_identifier = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "IamRoles".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.IamRoleArn.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.iam_roles = Some(IamRoleArnList { items });
        }
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(value) = params.get("KmsKeyId") {
        input.kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MaintenanceTrackName") {
        input.maintenance_track_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ManageMasterPassword") {
        input.manage_master_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ManageMasterPassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("ManualSnapshotRetentionPeriod") {
        input.manual_snapshot_retention_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ManualSnapshotRetentionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("MasterPasswordSecretKmsKeyId") {
        input.master_password_secret_kms_key_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MultiAZ") {
        input.multi_a_z = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse MultiAZ: {e}"))?,
        );
    }
    if let Some(value) = params.get("NodeType") {
        input.node_type = Some(value.to_string());
    }
    if let Some(value) = params.get("NumberOfNodes") {
        input.number_of_nodes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse NumberOfNodes: {e}"))?,
        );
    }
    if let Some(value) = params.get("OwnerAccount") {
        input.owner_account = Some(value.to_string());
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
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
    if let Some(value) = params.get("RedshiftIdcApplicationArn") {
        input.redshift_idc_application_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("ReservedNodeId") {
        input.reserved_node_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotArn") {
        input.snapshot_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotClusterIdentifier") {
        input.snapshot_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotScheduleIdentifier") {
        input.snapshot_schedule_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetReservedNodeOfferingId") {
        input.target_reserved_node_offering_id = Some(value.to_string());
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

/// Deserialize awsQuery request for RestoreTableFromClusterSnapshot.
pub fn deserialize_restore_table_from_cluster_snapshot_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestoreTableFromClusterSnapshotMessage, String> {
    let mut input = RestoreTableFromClusterSnapshotMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("EnableCaseSensitiveIdentifier") {
        input.enable_case_sensitive_identifier = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableCaseSensitiveIdentifier: {e}"))?,
        );
    }
    if let Some(value) = params.get("NewTableName") {
        input.new_table_name = value.to_string();
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = value.to_string();
    }
    if let Some(value) = params.get("SourceDatabaseName") {
        input.source_database_name = value.to_string();
    }
    if let Some(value) = params.get("SourceSchemaName") {
        input.source_schema_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceTableName") {
        input.source_table_name = value.to_string();
    }
    if let Some(value) = params.get("TargetDatabaseName") {
        input.target_database_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetSchemaName") {
        input.target_schema_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResumeCluster.
pub fn deserialize_resume_cluster_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResumeClusterMessage, String> {
    let mut input = ResumeClusterMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RevokeClusterSecurityGroupIngress.
pub fn deserialize_revoke_cluster_security_group_ingress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RevokeClusterSecurityGroupIngressMessage, String> {
    let mut input = RevokeClusterSecurityGroupIngressMessage::default();
    if let Some(value) = params.get("CIDRIP") {
        input.c_i_d_r_i_p = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterSecurityGroupName") {
        input.cluster_security_group_name = value.to_string();
    }
    if let Some(value) = params.get("EC2SecurityGroupName") {
        input.e_c2_security_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EC2SecurityGroupOwnerId") {
        input.e_c2_security_group_owner_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RevokeEndpointAccess.
pub fn deserialize_revoke_endpoint_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RevokeEndpointAccessMessage, String> {
    let mut input = RevokeEndpointAccessMessage::default();
    if let Some(value) = params.get("Account") {
        input.account = Some(value.to_string());
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("Force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Force: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VpcIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.VpcIdentifier.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.vpc_ids = Some(VpcIdentifierList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RevokeSnapshotAccess.
pub fn deserialize_revoke_snapshot_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RevokeSnapshotAccessMessage, String> {
    let mut input = RevokeSnapshotAccessMessage::default();
    if let Some(value) = params.get("AccountWithRestoreAccess") {
        input.account_with_restore_access = value.to_string();
    }
    if let Some(value) = params.get("SnapshotArn") {
        input.snapshot_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotClusterIdentifier") {
        input.snapshot_cluster_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("SnapshotIdentifier") {
        input.snapshot_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RotateEncryptionKey.
pub fn deserialize_rotate_encryption_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RotateEncryptionKeyMessage, String> {
    let mut input = RotateEncryptionKeyMessage::default();
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdatePartnerStatus.
pub fn deserialize_update_partner_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdatePartnerStatusInputMessage, String> {
    let mut input = UpdatePartnerStatusInputMessage::default();
    if let Some(value) = params.get("AccountId") {
        input.account_id = value.to_string();
    }
    if let Some(value) = params.get("ClusterIdentifier") {
        input.cluster_identifier = value.to_string();
    }
    if let Some(value) = params.get("DatabaseName") {
        input.database_name = value.to_string();
    }
    if let Some(value) = params.get("PartnerName") {
        input.partner_name = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("StatusMessage") {
        input.status_message = Some(value.to_string());
    }
    Ok(input)
}
