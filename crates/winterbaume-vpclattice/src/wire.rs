//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-vpclattice

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
pub fn serialize_batch_update_rule_response(result: &BatchUpdateRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_access_log_subscription_response(
    result: &CreateAccessLogSubscriptionResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_listener_response(result: &CreateListenerResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_configuration_response(
    result: &CreateResourceConfigurationResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_gateway_response(
    result: &CreateResourceGatewayResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_rule_response(result: &CreateRuleResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_response(result: &CreateServiceResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_network_response(
    result: &CreateServiceNetworkResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_network_resource_association_response(
    result: &CreateServiceNetworkResourceAssociationResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_network_service_association_response(
    result: &CreateServiceNetworkServiceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_network_vpc_association_response(
    result: &CreateServiceNetworkVpcAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_target_group_response(result: &CreateTargetGroupResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_access_log_subscription_response(
    result: &DeleteAccessLogSubscriptionResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_auth_policy_response(result: &DeleteAuthPolicyResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_domain_verification_response(
    result: &DeleteDomainVerificationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_listener_response(result: &DeleteListenerResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_configuration_response(
    result: &DeleteResourceConfigurationResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_endpoint_association_response(
    result: &DeleteResourceEndpointAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_gateway_response(
    result: &DeleteResourceGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_rule_response(result: &DeleteRuleResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_response(result: &DeleteServiceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_network_response(
    result: &DeleteServiceNetworkResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_network_resource_association_response(
    result: &DeleteServiceNetworkResourceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_network_service_association_response(
    result: &DeleteServiceNetworkServiceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_network_vpc_association_response(
    result: &DeleteServiceNetworkVpcAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_target_group_response(result: &DeleteTargetGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_targets_response(result: &DeregisterTargetsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_access_log_subscription_response(
    result: &GetAccessLogSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_auth_policy_response(result: &GetAuthPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_verification_response(
    result: &GetDomainVerificationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_listener_response(result: &GetListenerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_configuration_response(
    result: &GetResourceConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_gateway_response(
    result: &GetResourceGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_rule_response(result: &GetRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_response(result: &GetServiceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_network_response(result: &GetServiceNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_network_resource_association_response(
    result: &GetServiceNetworkResourceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_network_service_association_response(
    result: &GetServiceNetworkServiceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_network_vpc_association_response(
    result: &GetServiceNetworkVpcAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_target_group_response(result: &GetTargetGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_access_log_subscriptions_response(
    result: &ListAccessLogSubscriptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_verifications_response(
    result: &ListDomainVerificationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_listeners_response(result: &ListListenersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_configurations_response(
    result: &ListResourceConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_endpoint_associations_response(
    result: &ListResourceEndpointAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_gateways_response(
    result: &ListResourceGatewaysResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_rules_response(result: &ListRulesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_network_resource_associations_response(
    result: &ListServiceNetworkResourceAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_network_service_associations_response(
    result: &ListServiceNetworkServiceAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_network_vpc_associations_response(
    result: &ListServiceNetworkVpcAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_network_vpc_endpoint_associations_response(
    result: &ListServiceNetworkVpcEndpointAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_networks_response(
    result: &ListServiceNetworksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_services_response(result: &ListServicesResponse) -> MockResponse {
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
pub fn serialize_list_target_groups_response(result: &ListTargetGroupsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_targets_response(result: &ListTargetsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_auth_policy_response(result: &PutAuthPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_targets_response(result: &RegisterTargetsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_domain_verification_response(
    result: &StartDomainVerificationResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_access_log_subscription_response(
    result: &UpdateAccessLogSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_listener_response(result: &UpdateListenerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_configuration_response(
    result: &UpdateResourceConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_gateway_response(
    result: &UpdateResourceGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_rule_response(result: &UpdateRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_service_response(result: &UpdateServiceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_service_network_response(
    result: &UpdateServiceNetworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_service_network_vpc_association_response(
    result: &UpdateServiceNetworkVpcAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_target_group_response(result: &UpdateTargetGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateRuleRequest, String> {
    let mut input = BatchUpdateRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchUpdateRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_access_log_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessLogSubscriptionRequest, String> {
    let mut input = CreateAccessLogSubscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccessLogSubscriptionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateAccessLogSubscription request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_listener_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateListenerRequest, String> {
    let mut input = CreateListenerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateListenerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateListener request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceConfigurationRequest, String> {
    let mut input = CreateResourceConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateResourceConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceGatewayRequest, String> {
    let mut input = CreateResourceGatewayRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceGatewayRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateResourceGateway request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRuleRequest, String> {
    let mut input = CreateRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceRequest, String> {
    let mut input = CreateServiceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateService request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceNetworkRequest, String> {
    let mut input = CreateServiceNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateServiceNetwork request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_network_resource_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceNetworkResourceAssociationRequest, String> {
    let mut input = CreateServiceNetworkResourceAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceNetworkResourceAssociationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateServiceNetworkResourceAssociation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_network_service_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceNetworkServiceAssociationRequest, String> {
    let mut input = CreateServiceNetworkServiceAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceNetworkServiceAssociationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateServiceNetworkServiceAssociation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_network_vpc_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceNetworkVpcAssociationRequest, String> {
    let mut input = CreateServiceNetworkVpcAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceNetworkVpcAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateServiceNetworkVpcAssociation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_target_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTargetGroupRequest, String> {
    let mut input = CreateTargetGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTargetGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTargetGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_access_log_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessLogSubscriptionRequest, String> {
    let mut input = DeleteAccessLogSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "accessLogSubscriptionIdentifier" => {
                input.access_log_subscription_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_auth_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAuthPolicyRequest, String> {
    let mut input = DeleteAuthPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_verification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainVerificationRequest, String> {
    let mut input = DeleteDomainVerificationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainVerificationIdentifier" => {
                input.domain_verification_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_listener_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteListenerRequest, String> {
    let mut input = DeleteListenerRequest::default();
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceConfigurationRequest, String> {
    let mut input = DeleteResourceConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceConfigurationIdentifier" => {
                input.resource_configuration_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_endpoint_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceEndpointAssociationRequest, String> {
    let mut input = DeleteResourceEndpointAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceEndpointAssociationIdentifier" => {
                input.resource_endpoint_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceGatewayRequest, String> {
    let mut input = DeleteResourceGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceGatewayIdentifier" => {
                input.resource_gateway_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRuleRequest, String> {
    let mut input = DeleteRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "ruleIdentifier" => {
                input.rule_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceRequest, String> {
    let mut input = DeleteServiceRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceNetworkRequest, String> {
    let mut input = DeleteServiceNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkIdentifier" => {
                input.service_network_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_network_resource_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceNetworkResourceAssociationRequest, String> {
    let mut input = DeleteServiceNetworkResourceAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkResourceAssociationIdentifier" => {
                input.service_network_resource_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_network_service_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceNetworkServiceAssociationRequest, String> {
    let mut input = DeleteServiceNetworkServiceAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkServiceAssociationIdentifier" => {
                input.service_network_service_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_network_vpc_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceNetworkVpcAssociationRequest, String> {
    let mut input = DeleteServiceNetworkVpcAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkVpcAssociationIdentifier" => {
                input.service_network_vpc_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_target_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTargetGroupRequest, String> {
    let mut input = DeleteTargetGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterTargetsRequest, String> {
    let mut input = DeregisterTargetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeregisterTargetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeregisterTargets request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_access_log_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccessLogSubscriptionRequest, String> {
    let mut input = GetAccessLogSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "accessLogSubscriptionIdentifier" => {
                input.access_log_subscription_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_auth_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAuthPolicyRequest, String> {
    let mut input = GetAuthPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_verification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainVerificationRequest, String> {
    let mut input = GetDomainVerificationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainVerificationIdentifier" => {
                input.domain_verification_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_listener_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetListenerRequest, String> {
    let mut input = GetListenerRequest::default();
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceConfigurationRequest, String> {
    let mut input = GetResourceConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceConfigurationIdentifier" => {
                input.resource_configuration_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceGatewayRequest, String> {
    let mut input = GetResourceGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceGatewayIdentifier" => {
                input.resource_gateway_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyRequest, String> {
    let mut input = GetResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRuleRequest, String> {
    let mut input = GetRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "ruleIdentifier" => {
                input.rule_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceRequest, String> {
    let mut input = GetServiceRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceNetworkRequest, String> {
    let mut input = GetServiceNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkIdentifier" => {
                input.service_network_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_network_resource_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceNetworkResourceAssociationRequest, String> {
    let mut input = GetServiceNetworkResourceAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkResourceAssociationIdentifier" => {
                input.service_network_resource_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_network_service_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceNetworkServiceAssociationRequest, String> {
    let mut input = GetServiceNetworkServiceAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkServiceAssociationIdentifier" => {
                input.service_network_service_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_network_vpc_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceNetworkVpcAssociationRequest, String> {
    let mut input = GetServiceNetworkVpcAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceNetworkVpcAssociationIdentifier" => {
                input.service_network_vpc_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_target_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTargetGroupRequest, String> {
    let mut input = GetTargetGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_access_log_subscriptions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessLogSubscriptionsRequest, String> {
    let mut input = ListAccessLogSubscriptionsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceIdentifier") {
        input.resource_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domain_verifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainVerificationsRequest, String> {
    let mut input = ListDomainVerificationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_listeners_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListListenersRequest, String> {
    let mut input = ListListenersRequest::default();
    for (name, value) in labels {
        match *name {
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceConfigurationsRequest, String> {
    let mut input = ListResourceConfigurationsRequest::default();
    if let Some(value) = query.get("domainVerificationIdentifier") {
        input.domain_verification_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceConfigurationGroupIdentifier") {
        input.resource_configuration_group_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceGatewayIdentifier") {
        input.resource_gateway_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_endpoint_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceEndpointAssociationsRequest, String> {
    let mut input = ListResourceEndpointAssociationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceConfigurationIdentifier") {
        input.resource_configuration_identifier = value.to_string();
    }
    if let Some(value) = query.get("resourceEndpointAssociationIdentifier") {
        input.resource_endpoint_association_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("vpcEndpointId") {
        input.vpc_endpoint_id = Some(value.to_string());
    }
    if let Some(value) = query.get("vpcEndpointOwner") {
        input.vpc_endpoint_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_gateways_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceGatewaysRequest, String> {
    let mut input = ListResourceGatewaysRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRulesRequest, String> {
    let mut input = ListRulesRequest::default();
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_network_resource_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceNetworkResourceAssociationsRequest, String> {
    let mut input = ListServiceNetworkResourceAssociationsRequest::default();
    if let Some(value) = query.get("includeChildren") {
        input.include_children = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceConfigurationIdentifier") {
        input.resource_configuration_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceNetworkIdentifier") {
        input.service_network_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_network_service_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceNetworkServiceAssociationsRequest, String> {
    let mut input = ListServiceNetworkServiceAssociationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceIdentifier") {
        input.service_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceNetworkIdentifier") {
        input.service_network_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_network_vpc_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceNetworkVpcAssociationsRequest, String> {
    let mut input = ListServiceNetworkVpcAssociationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceNetworkIdentifier") {
        input.service_network_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("vpcIdentifier") {
        input.vpc_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_network_vpc_endpoint_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceNetworkVpcEndpointAssociationsRequest, String> {
    let mut input = ListServiceNetworkVpcEndpointAssociationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceNetworkIdentifier") {
        input.service_network_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_networks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceNetworksRequest, String> {
    let mut input = ListServiceNetworksRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_services_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServicesRequest, String> {
    let mut input = ListServicesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_target_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTargetGroupsRequest, String> {
    let mut input = ListTargetGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("targetGroupType") {
        input.target_group_type = Some(value.to_string());
    }
    if let Some(value) = query.get("vpcIdentifier") {
        input.vpc_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTargetsRequest, String> {
    let mut input = ListTargetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTargetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTargets request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_auth_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAuthPolicyRequest, String> {
    let mut input = PutAuthPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAuthPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAuthPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterTargetsRequest, String> {
    let mut input = RegisterTargetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterTargetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterTargets request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_domain_verification_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDomainVerificationRequest, String> {
    let mut input = StartDomainVerificationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDomainVerificationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartDomainVerification request: {err}"),
        )?;
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
            "resourceArn" => {
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
            "resourceArn" => {
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
pub fn deserialize_update_access_log_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccessLogSubscriptionRequest, String> {
    let mut input = UpdateAccessLogSubscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccessLogSubscriptionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAccessLogSubscription request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "accessLogSubscriptionIdentifier" => {
                input.access_log_subscription_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_listener_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateListenerRequest, String> {
    let mut input = UpdateListenerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateListenerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateListener request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceConfigurationRequest, String> {
    let mut input = UpdateResourceConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateResourceConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "resourceConfigurationIdentifier" => {
                input.resource_configuration_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceGatewayRequest, String> {
    let mut input = UpdateResourceGatewayRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceGatewayRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResourceGateway request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceGatewayIdentifier" => {
                input.resource_gateway_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRuleRequest, String> {
    let mut input = UpdateRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "listenerIdentifier" => {
                input.listener_identifier = value.to_string();
            }
            "ruleIdentifier" => {
                input.rule_identifier = value.to_string();
            }
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceRequest, String> {
    let mut input = UpdateServiceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateService request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "serviceIdentifier" => {
                input.service_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_service_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceNetworkRequest, String> {
    let mut input = UpdateServiceNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateServiceNetwork request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "serviceNetworkIdentifier" => {
                input.service_network_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_service_network_vpc_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceNetworkVpcAssociationRequest, String> {
    let mut input = UpdateServiceNetworkVpcAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceNetworkVpcAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateServiceNetworkVpcAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "serviceNetworkVpcAssociationIdentifier" => {
                input.service_network_vpc_association_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_target_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTargetGroupRequest, String> {
    let mut input = UpdateTargetGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTargetGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTargetGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "targetGroupIdentifier" => {
                input.target_group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
