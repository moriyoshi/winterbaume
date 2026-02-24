//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elbv2

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
pub fn serialize_add_listener_certificates_response(
    result: &AddListenerCertificatesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AddListenerCertificatesResult>{inner_xml}</AddListenerCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddListenerCertificatesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddListenerCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_add_tags_response(result: &AddTagsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AddTagsResult>{inner_xml}</AddTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_add_trust_store_revocations_response(
    result: &AddTrustStoreRevocationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AddTrustStoreRevocationsResult>{inner_xml}</AddTrustStoreRevocationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddTrustStoreRevocationsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddTrustStoreRevocationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_listener_response(result: &CreateListenerOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateListenerResult>{inner_xml}</CreateListenerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateListenerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateListenerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_load_balancer_response(result: &CreateLoadBalancerOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateLoadBalancerResult>{inner_xml}</CreateLoadBalancerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_rule_response(result: &CreateRuleOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateRuleResult>{inner_xml}</CreateRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateRuleResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_target_group_response(result: &CreateTargetGroupOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateTargetGroupResult>{inner_xml}</CreateTargetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTargetGroupResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTargetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_trust_store_response(result: &CreateTrustStoreOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateTrustStoreResult>{inner_xml}</CreateTrustStoreResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTrustStoreResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTrustStoreResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_listener_response(result: &DeleteListenerOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteListenerResult>{inner_xml}</DeleteListenerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteListenerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteListenerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_load_balancer_response(result: &DeleteLoadBalancerOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteLoadBalancerResult>{inner_xml}</DeleteLoadBalancerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_rule_response(result: &DeleteRuleOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteRuleResult>{inner_xml}</DeleteRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteRuleResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_shared_trust_store_association_response(
    result: &DeleteSharedTrustStoreAssociationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteSharedTrustStoreAssociationResult>{inner_xml}</DeleteSharedTrustStoreAssociationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSharedTrustStoreAssociationResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSharedTrustStoreAssociationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_target_group_response(result: &DeleteTargetGroupOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteTargetGroupResult>{inner_xml}</DeleteTargetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTargetGroupResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTargetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_trust_store_response(result: &DeleteTrustStoreOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteTrustStoreResult>{inner_xml}</DeleteTrustStoreResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTrustStoreResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTrustStoreResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deregister_targets_response(result: &DeregisterTargetsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeregisterTargetsResult>{inner_xml}</DeregisterTargetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeregisterTargetsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeregisterTargetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_limits_response(
    result: &DescribeAccountLimitsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountLimitsResult>{inner_xml}</DescribeAccountLimitsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountLimitsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountLimitsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_capacity_reservation_response(
    result: &DescribeCapacityReservationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeCapacityReservationResult>{inner_xml}</DescribeCapacityReservationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeCapacityReservationResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeCapacityReservationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_listener_attributes_response(
    result: &DescribeListenerAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeListenerAttributesResult>{inner_xml}</DescribeListenerAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeListenerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeListenerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_listener_certificates_response(
    result: &DescribeListenerCertificatesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeListenerCertificatesResult>{inner_xml}</DescribeListenerCertificatesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeListenerCertificatesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeListenerCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_listeners_response(result: &DescribeListenersOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeListenersResult>{inner_xml}</DescribeListenersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeListenersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeListenersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancer_attributes_response(
    result: &DescribeLoadBalancerAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeLoadBalancerAttributesResult>{inner_xml}</DescribeLoadBalancerAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancers_response(
    result: &DescribeLoadBalancersOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLoadBalancersResult>{inner_xml}</DescribeLoadBalancersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_rules_response(result: &DescribeRulesOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeRulesResult>{inner_xml}</DescribeRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeRulesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_s_s_l_policies_response(
    result: &DescribeSSLPoliciesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeSSLPoliciesResult>{inner_xml}</DescribeSSLPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeSSLPoliciesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeSSLPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_tags_response(result: &DescribeTagsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeTagsResult>{inner_xml}</DescribeTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_target_group_attributes_response(
    result: &DescribeTargetGroupAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeTargetGroupAttributesResult>{inner_xml}</DescribeTargetGroupAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTargetGroupAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTargetGroupAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_target_groups_response(
    result: &DescribeTargetGroupsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTargetGroupsResult>{inner_xml}</DescribeTargetGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTargetGroupsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTargetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_target_health_response(
    result: &DescribeTargetHealthOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTargetHealthResult>{inner_xml}</DescribeTargetHealthResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTargetHealthResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTargetHealthResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_trust_store_associations_response(
    result: &DescribeTrustStoreAssociationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeTrustStoreAssociationsResult>{inner_xml}</DescribeTrustStoreAssociationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTrustStoreAssociationsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTrustStoreAssociationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_trust_store_revocations_response(
    result: &DescribeTrustStoreRevocationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeTrustStoreRevocationsResult>{inner_xml}</DescribeTrustStoreRevocationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTrustStoreRevocationsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTrustStoreRevocationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_trust_stores_response(
    result: &DescribeTrustStoresOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeTrustStoresResult>{inner_xml}</DescribeTrustStoresResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTrustStoresResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTrustStoresResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetResourcePolicyResult>{inner_xml}</GetResourcePolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetResourcePolicyResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetResourcePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_trust_store_ca_certificates_bundle_response(
    result: &GetTrustStoreCaCertificatesBundleOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetTrustStoreCaCertificatesBundleResult>{inner_xml}</GetTrustStoreCaCertificatesBundleResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTrustStoreCaCertificatesBundleResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTrustStoreCaCertificatesBundleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_trust_store_revocation_content_response(
    result: &GetTrustStoreRevocationContentOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetTrustStoreRevocationContentResult>{inner_xml}</GetTrustStoreRevocationContentResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTrustStoreRevocationContentResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTrustStoreRevocationContentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_capacity_reservation_response(
    result: &ModifyCapacityReservationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyCapacityReservationResult>{inner_xml}</ModifyCapacityReservationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyCapacityReservationResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyCapacityReservationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_ip_pools_response(result: &ModifyIpPoolsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyIpPoolsResult>{inner_xml}</ModifyIpPoolsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyIpPoolsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyIpPoolsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_listener_response(result: &ModifyListenerOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyListenerResult>{inner_xml}</ModifyListenerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyListenerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyListenerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_listener_attributes_response(
    result: &ModifyListenerAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ModifyListenerAttributesResult>{inner_xml}</ModifyListenerAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyListenerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyListenerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_load_balancer_attributes_response(
    result: &ModifyLoadBalancerAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyLoadBalancerAttributesResult>{inner_xml}</ModifyLoadBalancerAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyLoadBalancerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyLoadBalancerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_rule_response(result: &ModifyRuleOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyRuleResult>{inner_xml}</ModifyRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyRuleResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_target_group_response(result: &ModifyTargetGroupOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyTargetGroupResult>{inner_xml}</ModifyTargetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyTargetGroupResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyTargetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_target_group_attributes_response(
    result: &ModifyTargetGroupAttributesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ModifyTargetGroupAttributesResult>{inner_xml}</ModifyTargetGroupAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyTargetGroupAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyTargetGroupAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_modify_trust_store_response(result: &ModifyTrustStoreOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ModifyTrustStoreResult>{inner_xml}</ModifyTrustStoreResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ModifyTrustStoreResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyTrustStoreResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_targets_response(result: &RegisterTargetsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RegisterTargetsResult>{inner_xml}</RegisterTargetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterTargetsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterTargetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_listener_certificates_response(
    result: &RemoveListenerCertificatesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RemoveListenerCertificatesResult>{inner_xml}</RemoveListenerCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveListenerCertificatesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveListenerCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_tags_response(result: &RemoveTagsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RemoveTagsResult>{inner_xml}</RemoveTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_remove_trust_store_revocations_response(
    result: &RemoveTrustStoreRevocationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RemoveTrustStoreRevocationsResult>{inner_xml}</RemoveTrustStoreRevocationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveTrustStoreRevocationsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveTrustStoreRevocationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_ip_address_type_response(result: &SetIpAddressTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SetIpAddressTypeResult>{inner_xml}</SetIpAddressTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIpAddressTypeResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIpAddressTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_rule_priorities_response(result: &SetRulePrioritiesOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SetRulePrioritiesResult>{inner_xml}</SetRulePrioritiesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetRulePrioritiesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetRulePrioritiesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_security_groups_response(result: &SetSecurityGroupsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SetSecurityGroupsResult>{inner_xml}</SetSecurityGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetSecurityGroupsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetSecurityGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_subnets_response(result: &SetSubnetsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SetSubnetsResult>{inner_xml}</SetSubnetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetSubnetsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetSubnetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_jwt_validation_action_additional_claim_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<JwtValidationActionAdditionalClaim>, String> {
    let mut item = JwtValidationActionAdditionalClaim::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Format")) {
        item.format = value.to_string();
        found = true;
    }
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
            item.values = JwtValidationActionAdditionalClaimValues { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rewrite_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RewriteConfig>, String> {
    let mut item = RewriteConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Regex")) {
        item.regex = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Replace")) {
        item.replace = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_query_string_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<QueryStringConditionConfig>, String> {
    let mut item = QueryStringConditionConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_query_string_key_value_pair_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(QueryStringKeyValuePairList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_matcher_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Matcher>, String> {
    let mut item = Matcher::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.GrpcCode")) {
        item.grpc_code = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.HttpCode")) {
        item.http_code = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_mutual_authentication_attributes_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MutualAuthenticationAttributes>, String> {
    let mut item = MutualAuthenticationAttributes::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AdvertiseTrustStoreCaNames")) {
        item.advertise_trust_store_ca_names = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IgnoreClientCertificateExpiry")) {
        item.ignore_client_certificate_expiry = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IgnoreClientCertificateExpiry: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Mode")) {
        item.mode = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TrustStoreArn")) {
        item.trust_store_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TrustStoreAssociationStatus")) {
        item.trust_store_association_status = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_ipam_pools_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<IpamPools>, String> {
    let mut item = IpamPools::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Ipv4IpamPoolId")) {
        item.ipv4_ipam_pool_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_source_ip_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SourceIpConditionConfig>, String> {
    let mut item = SourceIpConditionConfig::default();
    let mut found = false;
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_forward_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ForwardActionConfig>, String> {
    let mut item = ForwardActionConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.TargetGroups");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_target_group_tuple_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.target_groups = Some(TargetGroupList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_group_stickiness_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetGroupStickinessConfig>, String> {
    let mut item = TargetGroupStickinessConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DurationSeconds")) {
        item.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
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
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_group_attribute_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetGroupAttribute>, String> {
    let mut item = TargetGroupAttribute::default();
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

fn deserialize_http_header_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<HttpHeaderConditionConfig>, String> {
    let mut item = HttpHeaderConditionConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.HttpHeaderName")) {
        item.http_header_name = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RegexValues");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.regex_values = Some(ListOfString { items: sub_items });
            found = true;
        }
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_query_string_key_value_pair_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<QueryStringKeyValuePair>, String> {
    let mut item = QueryStringKeyValuePair::default();
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

fn deserialize_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Action>, String> {
    let mut item = Action::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Order")) {
        item.order = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Order: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TargetGroupArn")) {
        item.target_group_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = value.to_string();
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
        item.key = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_host_header_rewrite_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<HostHeaderRewriteConfig>, String> {
    let mut item = HostHeaderRewriteConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Rewrites");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_rewrite_config_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.rewrites = Some(RewriteConfigList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_load_balancer_attribute_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LoadBalancerAttribute>, String> {
    let mut item = LoadBalancerAttribute::default();
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

fn deserialize_listener_attribute_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ListenerAttribute>, String> {
    let mut item = ListenerAttribute::default();
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

fn deserialize_host_header_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<HostHeaderConditionConfig>, String> {
    let mut item = HostHeaderConditionConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RegexValues");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.regex_values = Some(ListOfString { items: sub_items });
            found = true;
        }
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_revocation_content_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RevocationContent>, String> {
    let mut item = RevocationContent::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.RevocationType")) {
        item.revocation_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3Bucket")) {
        item.s3_bucket = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3Key")) {
        item.s3_key = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3ObjectVersion")) {
        item.s3_object_version = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_fixed_response_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<FixedResponseActionConfig>, String> {
    let mut item = FixedResponseActionConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ContentType")) {
        item.content_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MessageBody")) {
        item.message_body = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StatusCode")) {
        item.status_code = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_http_request_method_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<HttpRequestMethodConditionConfig>, String> {
    let mut item = HttpRequestMethodConditionConfig::default();
    let mut found = false;
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_jwt_validation_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<JwtValidationActionConfig>, String> {
    let mut item = JwtValidationActionConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AdditionalClaims");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_jwt_validation_action_additional_claim_from_query(params, &item_key)?
            {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.additional_claims = Some(JwtValidationActionAdditionalClaims { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Issuer")) {
        item.issuer = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.JwksEndpoint")) {
        item.jwks_endpoint = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_minimum_load_balancer_capacity_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MinimumLoadBalancerCapacity>, String> {
    let mut item = MinimumLoadBalancerCapacity::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.CapacityUnits")) {
        item.capacity_units = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse CapacityUnits: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_authenticate_cognito_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AuthenticateCognitoActionConfig>, String> {
    let mut item = AuthenticateCognitoActionConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.OnUnauthenticatedRequest")) {
        item.on_unauthenticated_request = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Scope")) {
        item.scope = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SessionCookieName")) {
        item.session_cookie_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SessionTimeout")) {
        item.session_timeout = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse SessionTimeout: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UserPoolArn")) {
        item.user_pool_arn = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UserPoolClientId")) {
        item.user_pool_client_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UserPoolDomain")) {
        item.user_pool_domain = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_path_pattern_condition_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PathPatternConditionConfig>, String> {
    let mut item = PathPatternConditionConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RegexValues");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.regex_values = Some(ListOfString { items: sub_items });
            found = true;
        }
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_subnet_mapping_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SubnetMapping>, String> {
    let mut item = SubnetMapping::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AllocationId")) {
        item.allocation_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IPv6Address")) {
        item.i_pv6_address = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.PrivateIPv4Address")) {
        item.private_i_pv4_address = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SourceNatIpv6Prefix")) {
        item.source_nat_ipv6_prefix = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SubnetId")) {
        item.subnet_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_url_rewrite_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<UrlRewriteConfig>, String> {
    let mut item = UrlRewriteConfig::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Rewrites");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_rewrite_config_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.rewrites = Some(RewriteConfigList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_certificate_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Certificate>, String> {
    let mut item = Certificate::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.CertificateArn")) {
        item.certificate_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IsDefault")) {
        item.is_default = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IsDefault: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_authenticate_oidc_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AuthenticateOidcActionConfig>, String> {
    let mut item = AuthenticateOidcActionConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AuthorizationEndpoint")) {
        item.authorization_endpoint = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ClientId")) {
        item.client_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ClientSecret")) {
        item.client_secret = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Issuer")) {
        item.issuer = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.OnUnauthenticatedRequest")) {
        item.on_unauthenticated_request = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Scope")) {
        item.scope = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SessionCookieName")) {
        item.session_cookie_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SessionTimeout")) {
        item.session_timeout = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse SessionTimeout: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TokenEndpoint")) {
        item.token_endpoint = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UseExistingClientSecret")) {
        item.use_existing_client_secret = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UseExistingClientSecret: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UserInfoEndpoint")) {
        item.user_info_endpoint = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_description_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetDescription>, String> {
    let mut item = TargetDescription::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AvailabilityZone")) {
        item.availability_zone = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Id")) {
        item.id = value.to_string();
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
    if let Some(value) = params.get(&format!("{prefix}.QuicServerId")) {
        item.quic_server_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rule_transform_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RuleTransform>, String> {
    let mut item = RuleTransform::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_group_tuple_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetGroupTuple>, String> {
    let mut item = TargetGroupTuple::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.TargetGroupArn")) {
        item.target_group_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Weight")) {
        item.weight = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Weight: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rule_priority_pair_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RulePriorityPair>, String> {
    let mut item = RulePriorityPair::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Priority")) {
        item.priority = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Priority: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RuleArn")) {
        item.rule_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_redirect_action_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RedirectActionConfig>, String> {
    let mut item = RedirectActionConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Host")) {
        item.host = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Path")) {
        item.path = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Port")) {
        item.port = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Protocol")) {
        item.protocol = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Query")) {
        item.query = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StatusCode")) {
        item.status_code = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rule_condition_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RuleCondition>, String> {
    let mut item = RuleCondition::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Field")) {
        item.field = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RegexValues");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.regex_values = Some(ListOfString { items: sub_items });
            found = true;
        }
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
            item.values = Some(ListOfString { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AddListenerCertificates.
pub fn deserialize_add_listener_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddListenerCertificatesInput, String> {
    let mut input = AddListenerCertificatesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Certificates".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_certificate_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.certificates = CertificateList { items };
        }
    }
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddTags.
pub fn deserialize_add_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddTagsInput, String> {
    let mut input = AddTagsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_arns = ResourceArns { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
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

/// Deserialize awsQuery request for AddTrustStoreRevocations.
pub fn deserialize_add_trust_store_revocations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddTrustStoreRevocationsInput, String> {
    let mut input = AddTrustStoreRevocationsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RevocationContents".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_revocation_content_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.revocation_contents = Some(RevocationContents { items });
        }
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateListener.
pub fn deserialize_create_listener_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateListenerInput, String> {
    let mut input = CreateListenerInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AlpnPolicy".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alpn_policy = Some(AlpnPolicyName { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Certificates".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_certificate_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.certificates = Some(CertificateList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DefaultActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_action_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.default_actions = Actions { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    if let Some(val) =
        deserialize_mutual_authentication_attributes_from_query(params, "MutualAuthentication")?
    {
        input.mutual_authentication = Some(val);
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("Protocol") {
        input.protocol = Some(value.to_string());
    }
    if let Some(value) = params.get("SslPolicy") {
        input.ssl_policy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
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

/// Deserialize awsQuery request for CreateLoadBalancer.
pub fn deserialize_create_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLoadBalancerInput, String> {
    let mut input = CreateLoadBalancerInput::default();
    if let Some(value) = params.get("CustomerOwnedIpv4Pool") {
        input.customer_owned_ipv4_pool = Some(value.to_string());
    }
    if let Some(value) = params.get("EnablePrefixForIpv6SourceNat") {
        input.enable_prefix_for_ipv6_source_nat = Some(value.to_string());
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(val) = deserialize_ipam_pools_from_query(params, "IpamPools")? {
        input.ipam_pools = Some(val);
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("Scheme") {
        input.scheme = Some(value.to_string());
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
            input.security_groups = Some(SecurityGroups { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetMappings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_subnet_mapping_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_mappings = Some(SubnetMappings { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Subnets".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnets = Some(Subnets { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateRule.
pub fn deserialize_create_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateRuleInput, String> {
    let mut input = CreateRuleInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Actions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_action_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.actions = Actions { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Conditions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_rule_condition_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.conditions = RuleConditionList { items };
        }
    }
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    if let Some(value) = params.get("Priority") {
        input.priority = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Priority: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
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
        let list_prefix = "Transforms".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_rule_transform_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.transforms = Some(RuleTransformList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateTargetGroup.
pub fn deserialize_create_target_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTargetGroupInput, String> {
    let mut input = CreateTargetGroupInput::default();
    if let Some(value) = params.get("HealthCheckEnabled") {
        input.health_check_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse HealthCheckEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckIntervalSeconds") {
        input.health_check_interval_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckIntervalSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckPath") {
        input.health_check_path = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckPort") {
        input.health_check_port = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckProtocol") {
        input.health_check_protocol = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckTimeoutSeconds") {
        input.health_check_timeout_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckTimeoutSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthyThresholdCount") {
        input.healthy_threshold_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthyThresholdCount: {e}"))?,
        );
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(val) = deserialize_matcher_from_query(params, "Matcher")? {
        input.matcher = Some(val);
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("Protocol") {
        input.protocol = Some(value.to_string());
    }
    if let Some(value) = params.get("ProtocolVersion") {
        input.protocol_version = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(TagList { items });
        }
    }
    if let Some(value) = params.get("TargetControlPort") {
        input.target_control_port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TargetControlPort: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetType") {
        input.target_type = Some(value.to_string());
    }
    if let Some(value) = params.get("UnhealthyThresholdCount") {
        input.unhealthy_threshold_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse UnhealthyThresholdCount: {e}"))?,
        );
    }
    if let Some(value) = params.get("VpcId") {
        input.vpc_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateTrustStore.
pub fn deserialize_create_trust_store_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTrustStoreInput, String> {
    let mut input = CreateTrustStoreInput::default();
    if let Some(value) = params.get("CaCertificatesBundleS3Bucket") {
        input.ca_certificates_bundle_s3_bucket = value.to_string();
    }
    if let Some(value) = params.get("CaCertificatesBundleS3Key") {
        input.ca_certificates_bundle_s3_key = value.to_string();
    }
    if let Some(value) = params.get("CaCertificatesBundleS3ObjectVersion") {
        input.ca_certificates_bundle_s3_object_version = Some(value.to_string());
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
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

/// Deserialize awsQuery request for DeleteListener.
pub fn deserialize_delete_listener_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteListenerInput, String> {
    let mut input = DeleteListenerInput::default();
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLoadBalancer.
pub fn deserialize_delete_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoadBalancerInput, String> {
    let mut input = DeleteLoadBalancerInput::default();
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteRule.
pub fn deserialize_delete_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteRuleInput, String> {
    let mut input = DeleteRuleInput::default();
    if let Some(value) = params.get("RuleArn") {
        input.rule_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSharedTrustStoreAssociation.
pub fn deserialize_delete_shared_trust_store_association_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSharedTrustStoreAssociationInput, String> {
    let mut input = DeleteSharedTrustStoreAssociationInput::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTargetGroup.
pub fn deserialize_delete_target_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTargetGroupInput, String> {
    let mut input = DeleteTargetGroupInput::default();
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTrustStore.
pub fn deserialize_delete_trust_store_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrustStoreInput, String> {
    let mut input = DeleteTrustStoreInput::default();
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeregisterTargets.
pub fn deserialize_deregister_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeregisterTargetsInput, String> {
    let mut input = DeregisterTargetsInput::default();
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Targets".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_target_description_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.targets = TargetDescriptions { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAccountLimits.
pub fn deserialize_describe_account_limits_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountLimitsInput, String> {
    let mut input = DescribeAccountLimitsInput::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeCapacityReservation.
pub fn deserialize_describe_capacity_reservation_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeCapacityReservationInput, String> {
    let mut input = DescribeCapacityReservationInput::default();
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeListenerAttributes.
pub fn deserialize_describe_listener_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeListenerAttributesInput, String> {
    let mut input = DescribeListenerAttributesInput::default();
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeListenerCertificates.
pub fn deserialize_describe_listener_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeListenerCertificatesInput, String> {
    let mut input = DescribeListenerCertificatesInput::default();
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeListeners.
pub fn deserialize_describe_listeners_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeListenersInput, String> {
    let mut input = DescribeListenersInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ListenerArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.listener_arns = Some(ListenerArns { items });
        }
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancerAttributes.
pub fn deserialize_describe_load_balancer_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancerAttributesInput, String> {
    let mut input = DescribeLoadBalancerAttributesInput::default();
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancers.
pub fn deserialize_describe_load_balancers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancersInput, String> {
    let mut input = DescribeLoadBalancersInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_arns = Some(LoadBalancerArns { items });
        }
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = Some(LoadBalancerNames { items });
        }
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeRules.
pub fn deserialize_describe_rules_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeRulesInput, String> {
    let mut input = DescribeRulesInput::default();
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RuleArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_arns = Some(RuleArns { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeSSLPolicies.
pub fn deserialize_describe_s_s_l_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeSSLPoliciesInput, String> {
    let mut input = DescribeSSLPoliciesInput::default();
    if let Some(value) = params.get("LoadBalancerType") {
        input.load_balancer_type = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = Some(SslPolicyNames { items });
        }
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTags.
pub fn deserialize_describe_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTagsInput, String> {
    let mut input = DescribeTagsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_arns = ResourceArns { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTargetGroupAttributes.
pub fn deserialize_describe_target_group_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTargetGroupAttributesInput, String> {
    let mut input = DescribeTargetGroupAttributesInput::default();
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTargetGroups.
pub fn deserialize_describe_target_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTargetGroupsInput, String> {
    let mut input = DescribeTargetGroupsInput::default();
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = Some(TargetGroupNames { items });
        }
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TargetGroupArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.target_group_arns = Some(TargetGroupArns { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTargetHealth.
pub fn deserialize_describe_target_health_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTargetHealthInput, String> {
    let mut input = DescribeTargetHealthInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Include".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.include = Some(ListOfDescribeTargetHealthIncludeOptions { items });
        }
    }
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Targets".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_target_description_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.targets = Some(TargetDescriptions { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTrustStoreAssociations.
pub fn deserialize_describe_trust_store_associations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTrustStoreAssociationsInput, String> {
    let mut input = DescribeTrustStoreAssociationsInput::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTrustStoreRevocations.
pub fn deserialize_describe_trust_store_revocations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTrustStoreRevocationsInput, String> {
    let mut input = DescribeTrustStoreRevocationsInput::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RevocationIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(
                    value
                        .parse::<i64>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !items.is_empty() {
            input.revocation_ids = Some(RevocationIds { items });
        }
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTrustStores.
pub fn deserialize_describe_trust_stores_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTrustStoresInput, String> {
    let mut input = DescribeTrustStoresInput::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = Some(TrustStoreNames { items });
        }
    }
    if let Some(value) = params.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PageSize: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TrustStoreArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.trust_store_arns = Some(TrustStoreArns { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetResourcePolicy.
pub fn deserialize_get_resource_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyInput, String> {
    let mut input = GetResourcePolicyInput::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTrustStoreCaCertificatesBundle.
pub fn deserialize_get_trust_store_ca_certificates_bundle_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTrustStoreCaCertificatesBundleInput, String> {
    let mut input = GetTrustStoreCaCertificatesBundleInput::default();
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTrustStoreRevocationContent.
pub fn deserialize_get_trust_store_revocation_content_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTrustStoreRevocationContentInput, String> {
    let mut input = GetTrustStoreRevocationContentInput::default();
    if let Some(value) = params.get("RevocationId") {
        input.revocation_id = value
            .parse::<i64>()
            .map_err(|e| format!("failed to parse RevocationId: {e}"))?;
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyCapacityReservation.
pub fn deserialize_modify_capacity_reservation_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyCapacityReservationInput, String> {
    let mut input = ModifyCapacityReservationInput::default();
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    if let Some(val) = deserialize_minimum_load_balancer_capacity_from_query(
        params,
        "MinimumLoadBalancerCapacity",
    )? {
        input.minimum_load_balancer_capacity = Some(val);
    }
    if let Some(value) = params.get("ResetCapacityReservation") {
        input.reset_capacity_reservation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ResetCapacityReservation: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyIpPools.
pub fn deserialize_modify_ip_pools_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyIpPoolsInput, String> {
    let mut input = ModifyIpPoolsInput::default();
    if let Some(val) = deserialize_ipam_pools_from_query(params, "IpamPools")? {
        input.ipam_pools = Some(val);
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RemoveIpamPools".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.remove_ipam_pools = Some(RemoveIpamPools { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyListener.
pub fn deserialize_modify_listener_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyListenerInput, String> {
    let mut input = ModifyListenerInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AlpnPolicy".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alpn_policy = Some(AlpnPolicyName { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Certificates".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_certificate_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.certificates = Some(CertificateList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DefaultActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_action_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.default_actions = Some(Actions { items });
        }
    }
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    if let Some(val) =
        deserialize_mutual_authentication_attributes_from_query(params, "MutualAuthentication")?
    {
        input.mutual_authentication = Some(val);
    }
    if let Some(value) = params.get("Port") {
        input.port = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Port: {e}"))?,
        );
    }
    if let Some(value) = params.get("Protocol") {
        input.protocol = Some(value.to_string());
    }
    if let Some(value) = params.get("SslPolicy") {
        input.ssl_policy = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyListenerAttributes.
pub fn deserialize_modify_listener_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyListenerAttributesInput, String> {
    let mut input = ModifyListenerAttributesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Attributes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_listener_attribute_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attributes = ListenerAttributes { items };
        }
    }
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyLoadBalancerAttributes.
pub fn deserialize_modify_load_balancer_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyLoadBalancerAttributesInput, String> {
    let mut input = ModifyLoadBalancerAttributesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Attributes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_load_balancer_attribute_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attributes = LoadBalancerAttributes { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyRule.
pub fn deserialize_modify_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyRuleInput, String> {
    let mut input = ModifyRuleInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Actions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_action_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.actions = Some(Actions { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Conditions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_rule_condition_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.conditions = Some(RuleConditionList { items });
        }
    }
    if let Some(value) = params.get("ResetTransforms") {
        input.reset_transforms = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ResetTransforms: {e}"))?,
        );
    }
    if let Some(value) = params.get("RuleArn") {
        input.rule_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Transforms".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_rule_transform_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.transforms = Some(RuleTransformList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyTargetGroup.
pub fn deserialize_modify_target_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyTargetGroupInput, String> {
    let mut input = ModifyTargetGroupInput::default();
    if let Some(value) = params.get("HealthCheckEnabled") {
        input.health_check_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse HealthCheckEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckIntervalSeconds") {
        input.health_check_interval_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckIntervalSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckPath") {
        input.health_check_path = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckPort") {
        input.health_check_port = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckProtocol") {
        input.health_check_protocol = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckTimeoutSeconds") {
        input.health_check_timeout_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckTimeoutSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthyThresholdCount") {
        input.healthy_threshold_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthyThresholdCount: {e}"))?,
        );
    }
    if let Some(val) = deserialize_matcher_from_query(params, "Matcher")? {
        input.matcher = Some(val);
    }
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    if let Some(value) = params.get("UnhealthyThresholdCount") {
        input.unhealthy_threshold_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse UnhealthyThresholdCount: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyTargetGroupAttributes.
pub fn deserialize_modify_target_group_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyTargetGroupAttributesInput, String> {
    let mut input = ModifyTargetGroupAttributesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Attributes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_target_group_attribute_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attributes = TargetGroupAttributes { items };
        }
    }
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyTrustStore.
pub fn deserialize_modify_trust_store_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyTrustStoreInput, String> {
    let mut input = ModifyTrustStoreInput::default();
    if let Some(value) = params.get("CaCertificatesBundleS3Bucket") {
        input.ca_certificates_bundle_s3_bucket = value.to_string();
    }
    if let Some(value) = params.get("CaCertificatesBundleS3Key") {
        input.ca_certificates_bundle_s3_key = value.to_string();
    }
    if let Some(value) = params.get("CaCertificatesBundleS3ObjectVersion") {
        input.ca_certificates_bundle_s3_object_version = Some(value.to_string());
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterTargets.
pub fn deserialize_register_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterTargetsInput, String> {
    let mut input = RegisterTargetsInput::default();
    if let Some(value) = params.get("TargetGroupArn") {
        input.target_group_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Targets".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_target_description_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.targets = TargetDescriptions { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveListenerCertificates.
pub fn deserialize_remove_listener_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveListenerCertificatesInput, String> {
    let mut input = RemoveListenerCertificatesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Certificates".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_certificate_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.certificates = CertificateList { items };
        }
    }
    if let Some(value) = params.get("ListenerArn") {
        input.listener_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveTags.
pub fn deserialize_remove_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveTagsInput, String> {
    let mut input = RemoveTagsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_arns = ResourceArns { items };
        }
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
            input.tag_keys = TagKeys { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveTrustStoreRevocations.
pub fn deserialize_remove_trust_store_revocations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveTrustStoreRevocationsInput, String> {
    let mut input = RemoveTrustStoreRevocationsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RevocationIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(
                    value
                        .parse::<i64>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !items.is_empty() {
            input.revocation_ids = RevocationIds { items };
        }
    }
    if let Some(value) = params.get("TrustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIpAddressType.
pub fn deserialize_set_ip_address_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIpAddressTypeInput, String> {
    let mut input = SetIpAddressTypeInput::default();
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = value.to_string();
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetRulePriorities.
pub fn deserialize_set_rule_priorities_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetRulePrioritiesInput, String> {
    let mut input = SetRulePrioritiesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RulePriorities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_rule_priority_pair_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_priorities = RulePriorityList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetSecurityGroups.
pub fn deserialize_set_security_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetSecurityGroupsInput, String> {
    let mut input = SetSecurityGroupsInput::default();
    if let Some(value) = params.get("EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic") {
        input.enforce_security_group_inbound_rules_on_private_link_traffic =
            Some(value.to_string());
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
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
            input.security_groups = SecurityGroups { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetSubnets.
pub fn deserialize_set_subnets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetSubnetsInput, String> {
    let mut input = SetSubnetsInput::default();
    if let Some(value) = params.get("EnablePrefixForIpv6SourceNat") {
        input.enable_prefix_for_ipv6_source_nat = Some(value.to_string());
    }
    if let Some(value) = params.get("IpAddressType") {
        input.ip_address_type = Some(value.to_string());
    }
    if let Some(value) = params.get("LoadBalancerArn") {
        input.load_balancer_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetMappings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_subnet_mapping_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_mappings = Some(SubnetMappings { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Subnets".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnets = Some(Subnets { items });
        }
    }
    Ok(input)
}
