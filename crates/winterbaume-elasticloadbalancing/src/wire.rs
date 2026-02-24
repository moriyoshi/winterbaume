//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elastic-load-balancing

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
pub fn serialize_add_tags_response(result: &AddTagsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AddTagsResult>{inner_xml}</AddTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_apply_security_groups_to_load_balancer_response(
    result: &ApplySecurityGroupsToLoadBalancerOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ApplySecurityGroupsToLoadBalancerResult>{inner_xml}</ApplySecurityGroupsToLoadBalancerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ApplySecurityGroupsToLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ApplySecurityGroupsToLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_attach_load_balancer_to_subnets_response(
    result: &AttachLoadBalancerToSubnetsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AttachLoadBalancerToSubnetsResult>{inner_xml}</AttachLoadBalancerToSubnetsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachLoadBalancerToSubnetsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachLoadBalancerToSubnetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_configure_health_check_response(
    result: &ConfigureHealthCheckOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ConfigureHealthCheckResult>{inner_xml}</ConfigureHealthCheckResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ConfigureHealthCheckResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ConfigureHealthCheckResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_app_cookie_stickiness_policy_response(
    result: &CreateAppCookieStickinessPolicyOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateAppCookieStickinessPolicyResult>{inner_xml}</CreateAppCookieStickinessPolicyResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateAppCookieStickinessPolicyResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAppCookieStickinessPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_l_b_cookie_stickiness_policy_response(
    result: &CreateLBCookieStickinessPolicyOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateLBCookieStickinessPolicyResult>{inner_xml}</CreateLBCookieStickinessPolicyResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLBCookieStickinessPolicyResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLBCookieStickinessPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_load_balancer_response(result: &CreateAccessPointOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateLoadBalancerResult>{inner_xml}</CreateLoadBalancerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_load_balancer_listeners_response(
    result: &CreateLoadBalancerListenerOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateLoadBalancerListenersResult>{inner_xml}</CreateLoadBalancerListenersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLoadBalancerListenersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLoadBalancerListenersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_load_balancer_policy_response(
    result: &CreateLoadBalancerPolicyOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateLoadBalancerPolicyResult>{inner_xml}</CreateLoadBalancerPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLoadBalancerPolicyResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLoadBalancerPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_load_balancer_response(result: &DeleteAccessPointOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteLoadBalancerResult>{inner_xml}</DeleteLoadBalancerResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_load_balancer_listeners_response(
    result: &DeleteLoadBalancerListenerOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteLoadBalancerListenersResult>{inner_xml}</DeleteLoadBalancerListenersResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLoadBalancerListenersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLoadBalancerListenersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_load_balancer_policy_response(
    result: &DeleteLoadBalancerPolicyOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteLoadBalancerPolicyResult>{inner_xml}</DeleteLoadBalancerPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLoadBalancerPolicyResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLoadBalancerPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deregister_instances_from_load_balancer_response(
    result: &DeregisterEndPointsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeregisterInstancesFromLoadBalancerResult>{inner_xml}</DeregisterInstancesFromLoadBalancerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeregisterInstancesFromLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeregisterInstancesFromLoadBalancerResponse>"#
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
        r#"<DescribeAccountLimitsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountLimitsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_instance_health_response(
    result: &DescribeEndPointStateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeInstanceHealthResult>{inner_xml}</DescribeInstanceHealthResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeInstanceHealthResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeInstanceHealthResponse>"#
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
        r#"<DescribeLoadBalancerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancer_policies_response(
    result: &DescribeLoadBalancerPoliciesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeLoadBalancerPoliciesResult>{inner_xml}</DescribeLoadBalancerPoliciesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancerPoliciesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancerPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancer_policy_types_response(
    result: &DescribeLoadBalancerPolicyTypesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeLoadBalancerPolicyTypesResult>{inner_xml}</DescribeLoadBalancerPolicyTypesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancerPolicyTypesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancerPolicyTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancers_response(
    result: &DescribeAccessPointsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLoadBalancersResult>{inner_xml}</DescribeLoadBalancersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancersResponse>"#
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
        r#"<DescribeTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detach_load_balancer_from_subnets_response(
    result: &DetachLoadBalancerFromSubnetsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DetachLoadBalancerFromSubnetsResult>{inner_xml}</DetachLoadBalancerFromSubnetsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachLoadBalancerFromSubnetsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachLoadBalancerFromSubnetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_availability_zones_for_load_balancer_response(
    result: &RemoveAvailabilityZonesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DisableAvailabilityZonesForLoadBalancerResult>{inner_xml}</DisableAvailabilityZonesForLoadBalancerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableAvailabilityZonesForLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableAvailabilityZonesForLoadBalancerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_availability_zones_for_load_balancer_response(
    result: &AddAvailabilityZonesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<EnableAvailabilityZonesForLoadBalancerResult>{inner_xml}</EnableAvailabilityZonesForLoadBalancerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableAvailabilityZonesForLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableAvailabilityZonesForLoadBalancerResponse>"#
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
        r#"<ModifyLoadBalancerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ModifyLoadBalancerAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_instances_with_load_balancer_response(
    result: &RegisterEndPointsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RegisterInstancesWithLoadBalancerResult>{inner_xml}</RegisterInstancesWithLoadBalancerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterInstancesWithLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterInstancesWithLoadBalancerResponse>"#
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
        r#"<RemoveTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_load_balancer_listener_s_s_l_certificate_response(
    result: &SetLoadBalancerListenerSSLCertificateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetLoadBalancerListenerSSLCertificateResult>{inner_xml}</SetLoadBalancerListenerSSLCertificateResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetLoadBalancerListenerSSLCertificateResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetLoadBalancerListenerSSLCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_load_balancer_policies_for_backend_server_response(
    result: &SetLoadBalancerPoliciesForBackendServerOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetLoadBalancerPoliciesForBackendServerResult>{inner_xml}</SetLoadBalancerPoliciesForBackendServerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetLoadBalancerPoliciesForBackendServerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetLoadBalancerPoliciesForBackendServerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_load_balancer_policies_of_listener_response(
    result: &SetLoadBalancerPoliciesOfListenerOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetLoadBalancerPoliciesOfListenerResult>{inner_xml}</SetLoadBalancerPoliciesOfListenerResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetLoadBalancerPoliciesOfListenerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetLoadBalancerPoliciesOfListenerResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_health_check_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<HealthCheck>, String> {
    let mut item = HealthCheck::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.HealthyThreshold")) {
        item.healthy_threshold = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse HealthyThreshold: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Interval")) {
        item.interval = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Interval: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Target")) {
        item.target = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Timeout")) {
        item.timeout = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Timeout: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UnhealthyThreshold")) {
        item.unhealthy_threshold = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse UnhealthyThreshold: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_listener_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Listener>, String> {
    let mut item = Listener::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.InstancePort")) {
        item.instance_port = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse InstancePort: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.InstanceProtocol")) {
        item.instance_protocol = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LoadBalancerPort")) {
        item.load_balancer_port = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse LoadBalancerPort: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Protocol")) {
        item.protocol = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SSLCertificateId")) {
        item.s_s_l_certificate_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_load_balancer_attributes_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LoadBalancerAttributes>, String> {
    let mut item = LoadBalancerAttributes::default();
    let mut found = false;
    {
        let sub_prefix = format!("{prefix}.AccessLog");
        if let Some(val) = deserialize_access_log_from_query(params, &sub_prefix)? {
            item.access_log = Some(val);
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AdditionalAttributes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_additional_attribute_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.additional_attributes = Some(AdditionalAttributes { items: sub_items });
            found = true;
        }
    }
    {
        let sub_prefix = format!("{prefix}.ConnectionDraining");
        if let Some(val) = deserialize_connection_draining_from_query(params, &sub_prefix)? {
            item.connection_draining = Some(val);
            found = true;
        }
    }
    {
        let sub_prefix = format!("{prefix}.ConnectionSettings");
        if let Some(val) = deserialize_connection_settings_from_query(params, &sub_prefix)? {
            item.connection_settings = Some(val);
            found = true;
        }
    }
    {
        let sub_prefix = format!("{prefix}.CrossZoneLoadBalancing");
        if let Some(val) = deserialize_cross_zone_load_balancing_from_query(params, &sub_prefix)? {
            item.cross_zone_load_balancing = Some(val);
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Instance>, String> {
    let mut item = Instance::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.InstanceId")) {
        item.instance_id = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_additional_attribute_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AdditionalAttribute>, String> {
    let mut item = AdditionalAttribute::default();
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

fn deserialize_policy_attribute_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PolicyAttribute>, String> {
    let mut item = PolicyAttribute::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AttributeName")) {
        item.attribute_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.AttributeValue")) {
        item.attribute_value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_connection_settings_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConnectionSettings>, String> {
    let mut item = ConnectionSettings::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.IdleTimeout")) {
        item.idle_timeout = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse IdleTimeout: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_access_log_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AccessLog>, String> {
    let mut item = AccessLog::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.EmitInterval")) {
        item.emit_interval = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse EmitInterval: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3BucketName")) {
        item.s3_bucket_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3BucketPrefix")) {
        item.s3_bucket_prefix = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cross_zone_load_balancing_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CrossZoneLoadBalancing>, String> {
    let mut item = CrossZoneLoadBalancing::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_key_only_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TagKeyOnly>, String> {
    let mut item = TagKeyOnly::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Key")) {
        item.key = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_connection_draining_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConnectionDraining>, String> {
    let mut item = ConnectionDraining::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Timeout")) {
        item.timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Timeout: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AddTags.
pub fn deserialize_add_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddTagsInput, String> {
    let mut input = AddTagsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = LoadBalancerNames { items };
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

/// Deserialize awsQuery request for ApplySecurityGroupsToLoadBalancer.
pub fn deserialize_apply_security_groups_to_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ApplySecurityGroupsToLoadBalancerInput, String> {
    let mut input = ApplySecurityGroupsToLoadBalancerInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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

/// Deserialize awsQuery request for AttachLoadBalancerToSubnets.
pub fn deserialize_attach_load_balancer_to_subnets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachLoadBalancerToSubnetsInput, String> {
    let mut input = AttachLoadBalancerToSubnetsInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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
            input.subnets = Subnets { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ConfigureHealthCheck.
pub fn deserialize_configure_health_check_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ConfigureHealthCheckInput, String> {
    let mut input = ConfigureHealthCheckInput::default();
    if let Some(val) = deserialize_health_check_from_query(params, "HealthCheck")? {
        input.health_check = val;
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateAppCookieStickinessPolicy.
pub fn deserialize_create_app_cookie_stickiness_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAppCookieStickinessPolicyInput, String> {
    let mut input = CreateAppCookieStickinessPolicyInput::default();
    if let Some(value) = params.get("CookieName") {
        input.cookie_name = value.to_string();
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateLBCookieStickinessPolicy.
pub fn deserialize_create_l_b_cookie_stickiness_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLBCookieStickinessPolicyInput, String> {
    let mut input = CreateLBCookieStickinessPolicyInput::default();
    if let Some(value) = params.get("CookieExpirationPeriod") {
        input.cookie_expiration_period = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse CookieExpirationPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateLoadBalancer.
pub fn deserialize_create_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessPointInput, String> {
    let mut input = CreateAccessPointInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZones { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Listeners".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_listener_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.listeners = Listeners { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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
    Ok(input)
}

/// Deserialize awsQuery request for CreateLoadBalancerListeners.
pub fn deserialize_create_load_balancer_listeners_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLoadBalancerListenerInput, String> {
    let mut input = CreateLoadBalancerListenerInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Listeners".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_listener_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.listeners = Listeners { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateLoadBalancerPolicy.
pub fn deserialize_create_load_balancer_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLoadBalancerPolicyInput, String> {
    let mut input = CreateLoadBalancerPolicyInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyAttributes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_policy_attribute_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_attributes = Some(PolicyAttributes { items });
        }
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyTypeName") {
        input.policy_type_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLoadBalancer.
pub fn deserialize_delete_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointInput, String> {
    let mut input = DeleteAccessPointInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLoadBalancerListeners.
pub fn deserialize_delete_load_balancer_listeners_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoadBalancerListenerInput, String> {
    let mut input = DeleteLoadBalancerListenerInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerPorts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(
                    value
                        .parse::<i32>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_ports = Ports { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLoadBalancerPolicy.
pub fn deserialize_delete_load_balancer_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoadBalancerPolicyInput, String> {
    let mut input = DeleteLoadBalancerPolicyInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeregisterInstancesFromLoadBalancer.
pub fn deserialize_deregister_instances_from_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeregisterEndPointsInput, String> {
    let mut input = DeregisterEndPointsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Instances".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_instance_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instances = Instances { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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

/// Deserialize awsQuery request for DescribeInstanceHealth.
pub fn deserialize_describe_instance_health_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEndPointStateInput, String> {
    let mut input = DescribeEndPointStateInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Instances".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_instance_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instances = Some(Instances { items });
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancerAttributes.
pub fn deserialize_describe_load_balancer_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancerAttributesInput, String> {
    let mut input = DescribeLoadBalancerAttributesInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancerPolicies.
pub fn deserialize_describe_load_balancer_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancerPoliciesInput, String> {
    let mut input = DescribeLoadBalancerPoliciesInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_names = Some(PolicyNames { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancerPolicyTypes.
pub fn deserialize_describe_load_balancer_policy_types_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancerPolicyTypesInput, String> {
    let mut input = DescribeLoadBalancerPolicyTypesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyTypeNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_type_names = Some(PolicyTypeNames { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancers.
pub fn deserialize_describe_load_balancers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccessPointsInput, String> {
    let mut input = DescribeAccessPointsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = Some(LoadBalancerNames { items });
        }
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

/// Deserialize awsQuery request for DescribeTags.
pub fn deserialize_describe_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTagsInput, String> {
    let mut input = DescribeTagsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = LoadBalancerNamesMax20 { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachLoadBalancerFromSubnets.
pub fn deserialize_detach_load_balancer_from_subnets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachLoadBalancerFromSubnetsInput, String> {
    let mut input = DetachLoadBalancerFromSubnetsInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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
            input.subnets = Subnets { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableAvailabilityZonesForLoadBalancer.
pub fn deserialize_disable_availability_zones_for_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveAvailabilityZonesInput, String> {
    let mut input = RemoveAvailabilityZonesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = AvailabilityZones { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableAvailabilityZonesForLoadBalancer.
pub fn deserialize_enable_availability_zones_for_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddAvailabilityZonesInput, String> {
    let mut input = AddAvailabilityZonesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = AvailabilityZones { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ModifyLoadBalancerAttributes.
pub fn deserialize_modify_load_balancer_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ModifyLoadBalancerAttributesInput, String> {
    let mut input = ModifyLoadBalancerAttributesInput::default();
    if let Some(val) =
        deserialize_load_balancer_attributes_from_query(params, "LoadBalancerAttributes")?
    {
        input.load_balancer_attributes = val;
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterInstancesWithLoadBalancer.
pub fn deserialize_register_instances_with_load_balancer_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterEndPointsInput, String> {
    let mut input = RegisterEndPointsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Instances".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_instance_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instances = Instances { items };
        }
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
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
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = LoadBalancerNames { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_key_only_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = TagKeyList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetLoadBalancerListenerSSLCertificate.
pub fn deserialize_set_load_balancer_listener_s_s_l_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetLoadBalancerListenerSSLCertificateInput, String> {
    let mut input = SetLoadBalancerListenerSSLCertificateInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    if let Some(value) = params.get("LoadBalancerPort") {
        input.load_balancer_port = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse LoadBalancerPort: {e}"))?;
    }
    if let Some(value) = params.get("SSLCertificateId") {
        input.s_s_l_certificate_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetLoadBalancerPoliciesForBackendServer.
pub fn deserialize_set_load_balancer_policies_for_backend_server_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetLoadBalancerPoliciesForBackendServerInput, String> {
    let mut input = SetLoadBalancerPoliciesForBackendServerInput::default();
    if let Some(value) = params.get("InstancePort") {
        input.instance_port = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse InstancePort: {e}"))?;
    }
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_names = PolicyNames { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetLoadBalancerPoliciesOfListener.
pub fn deserialize_set_load_balancer_policies_of_listener_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetLoadBalancerPoliciesOfListenerInput, String> {
    let mut input = SetLoadBalancerPoliciesOfListenerInput::default();
    if let Some(value) = params.get("LoadBalancerName") {
        input.load_balancer_name = value.to_string();
    }
    if let Some(value) = params.get("LoadBalancerPort") {
        input.load_balancer_port = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse LoadBalancerPort: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_names = PolicyNames { items };
        }
    }
    Ok(input)
}
