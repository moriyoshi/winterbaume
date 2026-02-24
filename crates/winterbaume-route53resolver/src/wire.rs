//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53resolver

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_firewall_rule_group_response(
    result: &AssociateFirewallRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_resolver_endpoint_ip_address_response(
    result: &AssociateResolverEndpointIpAddressResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_resolver_query_log_config_response(
    result: &AssociateResolverQueryLogConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_resolver_rule_response(
    result: &AssociateResolverRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_firewall_domain_list_response(
    result: &CreateFirewallDomainListResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_firewall_rule_response(
    result: &CreateFirewallRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_firewall_rule_group_response(
    result: &CreateFirewallRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_outpost_resolver_response(
    result: &CreateOutpostResolverResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resolver_endpoint_response(
    result: &CreateResolverEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resolver_query_log_config_response(
    result: &CreateResolverQueryLogConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resolver_rule_response(
    result: &CreateResolverRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_domain_list_response(
    result: &DeleteFirewallDomainListResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_rule_response(
    result: &DeleteFirewallRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_rule_group_response(
    result: &DeleteFirewallRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_outpost_resolver_response(
    result: &DeleteOutpostResolverResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resolver_endpoint_response(
    result: &DeleteResolverEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resolver_query_log_config_response(
    result: &DeleteResolverQueryLogConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resolver_rule_response(
    result: &DeleteResolverRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_firewall_rule_group_response(
    result: &DisassociateFirewallRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_resolver_endpoint_ip_address_response(
    result: &DisassociateResolverEndpointIpAddressResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_resolver_query_log_config_response(
    result: &DisassociateResolverQueryLogConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_resolver_rule_response(
    result: &DisassociateResolverRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_firewall_config_response(result: &GetFirewallConfigResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_firewall_domain_list_response(
    result: &GetFirewallDomainListResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_firewall_rule_group_response(
    result: &GetFirewallRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_firewall_rule_group_association_response(
    result: &GetFirewallRuleGroupAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_firewall_rule_group_policy_response(
    result: &GetFirewallRuleGroupPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_outpost_resolver_response(
    result: &GetOutpostResolverResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_config_response(result: &GetResolverConfigResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_dnssec_config_response(
    result: &GetResolverDnssecConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_endpoint_response(
    result: &GetResolverEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_query_log_config_response(
    result: &GetResolverQueryLogConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_query_log_config_association_response(
    result: &GetResolverQueryLogConfigAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_query_log_config_policy_response(
    result: &GetResolverQueryLogConfigPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_rule_response(result: &GetResolverRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_rule_association_response(
    result: &GetResolverRuleAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resolver_rule_policy_response(
    result: &GetResolverRulePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_firewall_domains_response(
    result: &ImportFirewallDomainsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_configs_response(
    result: &ListFirewallConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_domain_lists_response(
    result: &ListFirewallDomainListsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_domains_response(
    result: &ListFirewallDomainsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_rule_group_associations_response(
    result: &ListFirewallRuleGroupAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_rule_groups_response(
    result: &ListFirewallRuleGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_rules_response(result: &ListFirewallRulesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_outpost_resolvers_response(
    result: &ListOutpostResolversResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_configs_response(
    result: &ListResolverConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_dnssec_configs_response(
    result: &ListResolverDnssecConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_endpoint_ip_addresses_response(
    result: &ListResolverEndpointIpAddressesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_endpoints_response(
    result: &ListResolverEndpointsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_query_log_config_associations_response(
    result: &ListResolverQueryLogConfigAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_query_log_configs_response(
    result: &ListResolverQueryLogConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_rule_associations_response(
    result: &ListResolverRuleAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resolver_rules_response(result: &ListResolverRulesResponse) -> MockResponse {
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
pub fn serialize_put_firewall_rule_group_policy_response(
    result: &PutFirewallRuleGroupPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resolver_query_log_config_policy_response(
    result: &PutResolverQueryLogConfigPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resolver_rule_policy_response(
    result: &PutResolverRulePolicyResponse,
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
pub fn serialize_update_firewall_config_response(
    result: &UpdateFirewallConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_domains_response(
    result: &UpdateFirewallDomainsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_rule_response(
    result: &UpdateFirewallRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_rule_group_association_response(
    result: &UpdateFirewallRuleGroupAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_outpost_resolver_response(
    result: &UpdateOutpostResolverResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resolver_config_response(
    result: &UpdateResolverConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resolver_dnssec_config_response(
    result: &UpdateResolverDnssecConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resolver_endpoint_response(
    result: &UpdateResolverEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resolver_rule_response(
    result: &UpdateResolverRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_firewall_rule_group_request(
    body: &[u8],
) -> Result<AssociateFirewallRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(AssociateFirewallRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateFirewallRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_resolver_endpoint_ip_address_request(
    body: &[u8],
) -> Result<AssociateResolverEndpointIpAddressRequest, String> {
    if body.is_empty() {
        return Ok(AssociateResolverEndpointIpAddressRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateResolverEndpointIpAddress request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_resolver_query_log_config_request(
    body: &[u8],
) -> Result<AssociateResolverQueryLogConfigRequest, String> {
    if body.is_empty() {
        return Ok(AssociateResolverQueryLogConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateResolverQueryLogConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_resolver_rule_request(
    body: &[u8],
) -> Result<AssociateResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(AssociateResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateResolverRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_firewall_domain_list_request(
    body: &[u8],
) -> Result<CreateFirewallDomainListRequest, String> {
    if body.is_empty() {
        return Ok(CreateFirewallDomainListRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFirewallDomainList request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_firewall_rule_request(
    body: &[u8],
) -> Result<CreateFirewallRuleRequest, String> {
    if body.is_empty() {
        return Ok(CreateFirewallRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFirewallRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_firewall_rule_group_request(
    body: &[u8],
) -> Result<CreateFirewallRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateFirewallRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFirewallRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_outpost_resolver_request(
    body: &[u8],
) -> Result<CreateOutpostResolverRequest, String> {
    if body.is_empty() {
        return Ok(CreateOutpostResolverRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateOutpostResolver request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resolver_endpoint_request(
    body: &[u8],
) -> Result<CreateResolverEndpointRequest, String> {
    if body.is_empty() {
        return Ok(CreateResolverEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResolverEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resolver_query_log_config_request(
    body: &[u8],
) -> Result<CreateResolverQueryLogConfigRequest, String> {
    if body.is_empty() {
        return Ok(CreateResolverQueryLogConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResolverQueryLogConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resolver_rule_request(
    body: &[u8],
) -> Result<CreateResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(CreateResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResolverRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_domain_list_request(
    body: &[u8],
) -> Result<DeleteFirewallDomainListRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallDomainListRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewallDomainList request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_rule_request(
    body: &[u8],
) -> Result<DeleteFirewallRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewallRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_rule_group_request(
    body: &[u8],
) -> Result<DeleteFirewallRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewallRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_outpost_resolver_request(
    body: &[u8],
) -> Result<DeleteOutpostResolverRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOutpostResolverRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteOutpostResolver request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resolver_endpoint_request(
    body: &[u8],
) -> Result<DeleteResolverEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResolverEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResolverEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resolver_query_log_config_request(
    body: &[u8],
) -> Result<DeleteResolverQueryLogConfigRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResolverQueryLogConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResolverQueryLogConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resolver_rule_request(
    body: &[u8],
) -> Result<DeleteResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResolverRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_firewall_rule_group_request(
    body: &[u8],
) -> Result<DisassociateFirewallRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateFirewallRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateFirewallRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_resolver_endpoint_ip_address_request(
    body: &[u8],
) -> Result<DisassociateResolverEndpointIpAddressRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateResolverEndpointIpAddressRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateResolverEndpointIpAddress request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_resolver_query_log_config_request(
    body: &[u8],
) -> Result<DisassociateResolverQueryLogConfigRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateResolverQueryLogConfigRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateResolverQueryLogConfig request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_resolver_rule_request(
    body: &[u8],
) -> Result<DisassociateResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateResolverRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_firewall_config_request(
    body: &[u8],
) -> Result<GetFirewallConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetFirewallConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFirewallConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_firewall_domain_list_request(
    body: &[u8],
) -> Result<GetFirewallDomainListRequest, String> {
    if body.is_empty() {
        return Ok(GetFirewallDomainListRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFirewallDomainList request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_firewall_rule_group_request(
    body: &[u8],
) -> Result<GetFirewallRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(GetFirewallRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFirewallRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_firewall_rule_group_association_request(
    body: &[u8],
) -> Result<GetFirewallRuleGroupAssociationRequest, String> {
    if body.is_empty() {
        return Ok(GetFirewallRuleGroupAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFirewallRuleGroupAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_firewall_rule_group_policy_request(
    body: &[u8],
) -> Result<GetFirewallRuleGroupPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetFirewallRuleGroupPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFirewallRuleGroupPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_outpost_resolver_request(
    body: &[u8],
) -> Result<GetOutpostResolverRequest, String> {
    if body.is_empty() {
        return Ok(GetOutpostResolverRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOutpostResolver request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_config_request(
    body: &[u8],
) -> Result<GetResolverConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_dnssec_config_request(
    body: &[u8],
) -> Result<GetResolverDnssecConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverDnssecConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverDnssecConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_endpoint_request(
    body: &[u8],
) -> Result<GetResolverEndpointRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_query_log_config_request(
    body: &[u8],
) -> Result<GetResolverQueryLogConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverQueryLogConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverQueryLogConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_query_log_config_association_request(
    body: &[u8],
) -> Result<GetResolverQueryLogConfigAssociationRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverQueryLogConfigAssociationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetResolverQueryLogConfigAssociation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_query_log_config_policy_request(
    body: &[u8],
) -> Result<GetResolverQueryLogConfigPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverQueryLogConfigPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverQueryLogConfigPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_rule_request(
    body: &[u8],
) -> Result<GetResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_rule_association_request(
    body: &[u8],
) -> Result<GetResolverRuleAssociationRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverRuleAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverRuleAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resolver_rule_policy_request(
    body: &[u8],
) -> Result<GetResolverRulePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResolverRulePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResolverRulePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_firewall_domains_request(
    body: &[u8],
) -> Result<ImportFirewallDomainsRequest, String> {
    if body.is_empty() {
        return Ok(ImportFirewallDomainsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportFirewallDomains request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_configs_request(
    body: &[u8],
) -> Result<ListFirewallConfigsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallConfigsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_domain_lists_request(
    body: &[u8],
) -> Result<ListFirewallDomainListsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallDomainListsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallDomainLists request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_domains_request(
    body: &[u8],
) -> Result<ListFirewallDomainsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallDomainsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallDomains request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_rule_group_associations_request(
    body: &[u8],
) -> Result<ListFirewallRuleGroupAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallRuleGroupAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListFirewallRuleGroupAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_rule_groups_request(
    body: &[u8],
) -> Result<ListFirewallRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallRuleGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_rules_request(
    body: &[u8],
) -> Result<ListFirewallRulesRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_outpost_resolvers_request(
    body: &[u8],
) -> Result<ListOutpostResolversRequest, String> {
    if body.is_empty() {
        return Ok(ListOutpostResolversRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOutpostResolvers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_configs_request(
    body: &[u8],
) -> Result<ListResolverConfigsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverConfigsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_dnssec_configs_request(
    body: &[u8],
) -> Result<ListResolverDnssecConfigsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverDnssecConfigsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverDnssecConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_endpoint_ip_addresses_request(
    body: &[u8],
) -> Result<ListResolverEndpointIpAddressesRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverEndpointIpAddressesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverEndpointIpAddresses request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_endpoints_request(
    body: &[u8],
) -> Result<ListResolverEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_query_log_config_associations_request(
    body: &[u8],
) -> Result<ListResolverQueryLogConfigAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverQueryLogConfigAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListResolverQueryLogConfigAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_query_log_configs_request(
    body: &[u8],
) -> Result<ListResolverQueryLogConfigsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverQueryLogConfigsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverQueryLogConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_rule_associations_request(
    body: &[u8],
) -> Result<ListResolverRuleAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverRuleAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverRuleAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resolver_rules_request(
    body: &[u8],
) -> Result<ListResolverRulesRequest, String> {
    if body.is_empty() {
        return Ok(ListResolverRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResolverRules request: {e}"))
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
pub fn deserialize_put_firewall_rule_group_policy_request(
    body: &[u8],
) -> Result<PutFirewallRuleGroupPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutFirewallRuleGroupPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutFirewallRuleGroupPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resolver_query_log_config_policy_request(
    body: &[u8],
) -> Result<PutResolverQueryLogConfigPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResolverQueryLogConfigPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResolverQueryLogConfigPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resolver_rule_policy_request(
    body: &[u8],
) -> Result<PutResolverRulePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResolverRulePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResolverRulePolicy request: {e}"))
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
pub fn deserialize_update_firewall_config_request(
    body: &[u8],
) -> Result<UpdateFirewallConfigRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_domains_request(
    body: &[u8],
) -> Result<UpdateFirewallDomainsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallDomainsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallDomains request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_rule_request(
    body: &[u8],
) -> Result<UpdateFirewallRuleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_rule_group_association_request(
    body: &[u8],
) -> Result<UpdateFirewallRuleGroupAssociationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallRuleGroupAssociationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateFirewallRuleGroupAssociation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_outpost_resolver_request(
    body: &[u8],
) -> Result<UpdateOutpostResolverRequest, String> {
    if body.is_empty() {
        return Ok(UpdateOutpostResolverRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateOutpostResolver request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resolver_config_request(
    body: &[u8],
) -> Result<UpdateResolverConfigRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResolverConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResolverConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resolver_dnssec_config_request(
    body: &[u8],
) -> Result<UpdateResolverDnssecConfigRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResolverDnssecConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResolverDnssecConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resolver_endpoint_request(
    body: &[u8],
) -> Result<UpdateResolverEndpointRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResolverEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResolverEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resolver_rule_request(
    body: &[u8],
) -> Result<UpdateResolverRuleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResolverRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResolverRule request: {e}"))
}
