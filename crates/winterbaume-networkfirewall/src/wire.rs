//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-networkfirewall

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_network_firewall_transit_gateway_attachment_response(
    result: &AcceptNetworkFirewallTransitGatewayAttachmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_availability_zones_response(
    result: &AssociateAvailabilityZonesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_firewall_policy_response(
    result: &AssociateFirewallPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_subnets_response(result: &AssociateSubnetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_attach_rule_groups_to_proxy_configuration_response(
    result: &AttachRuleGroupsToProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_firewall_response(result: &CreateFirewallResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_firewall_policy_response(
    result: &CreateFirewallPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_proxy_response(result: &CreateProxyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_proxy_configuration_response(
    result: &CreateProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_proxy_rule_group_response(
    result: &CreateProxyRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_proxy_rules_response(result: &CreateProxyRulesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_rule_group_response(result: &CreateRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_t_l_s_inspection_configuration_response(
    result: &CreateTLSInspectionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_vpc_endpoint_association_response(
    result: &CreateVpcEndpointAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_response(result: &DeleteFirewallResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_policy_response(
    result: &DeleteFirewallPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_network_firewall_transit_gateway_attachment_response(
    result: &DeleteNetworkFirewallTransitGatewayAttachmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_proxy_response(result: &DeleteProxyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_proxy_configuration_response(
    result: &DeleteProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_proxy_rule_group_response(
    result: &DeleteProxyRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_proxy_rules_response(result: &DeleteProxyRulesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_rule_group_response(result: &DeleteRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_t_l_s_inspection_configuration_response(
    result: &DeleteTLSInspectionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_vpc_endpoint_association_response(
    result: &DeleteVpcEndpointAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_firewall_response(result: &DescribeFirewallResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_firewall_metadata_response(
    result: &DescribeFirewallMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_firewall_policy_response(
    result: &DescribeFirewallPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_flow_operation_response(
    result: &DescribeFlowOperationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_logging_configuration_response(
    result: &DescribeLoggingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_proxy_response(result: &DescribeProxyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_proxy_configuration_response(
    result: &DescribeProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_proxy_rule_response(result: &DescribeProxyRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_proxy_rule_group_response(
    result: &DescribeProxyRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_resource_policy_response(
    result: &DescribeResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_rule_group_response(result: &DescribeRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_rule_group_metadata_response(
    result: &DescribeRuleGroupMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_rule_group_summary_response(
    result: &DescribeRuleGroupSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_t_l_s_inspection_configuration_response(
    result: &DescribeTLSInspectionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_vpc_endpoint_association_response(
    result: &DescribeVpcEndpointAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detach_rule_groups_from_proxy_configuration_response(
    result: &DetachRuleGroupsFromProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_availability_zones_response(
    result: &DisassociateAvailabilityZonesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_subnets_response(
    result: &DisassociateSubnetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_analysis_report_results_response(
    result: &GetAnalysisReportResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_analysis_reports_response(
    result: &ListAnalysisReportsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewall_policies_response(
    result: &ListFirewallPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_firewalls_response(result: &ListFirewallsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_flow_operation_results_response(
    result: &ListFlowOperationResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_flow_operations_response(
    result: &ListFlowOperationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_proxies_response(result: &ListProxiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_proxy_configurations_response(
    result: &ListProxyConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_proxy_rule_groups_response(
    result: &ListProxyRuleGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_rule_groups_response(result: &ListRuleGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_t_l_s_inspection_configurations_response(
    result: &ListTLSInspectionConfigurationsResponse,
) -> MockResponse {
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
pub fn serialize_list_vpc_endpoint_associations_response(
    result: &ListVpcEndpointAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reject_network_firewall_transit_gateway_attachment_response(
    result: &RejectNetworkFirewallTransitGatewayAttachmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_analysis_report_response(
    result: &StartAnalysisReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_flow_capture_response(result: &StartFlowCaptureResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_flow_flush_response(result: &StartFlowFlushResponse) -> MockResponse {
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
pub fn serialize_update_availability_zone_change_protection_response(
    result: &UpdateAvailabilityZoneChangeProtectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_analysis_settings_response(
    result: &UpdateFirewallAnalysisSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_delete_protection_response(
    result: &UpdateFirewallDeleteProtectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_description_response(
    result: &UpdateFirewallDescriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_encryption_configuration_response(
    result: &UpdateFirewallEncryptionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_policy_response(
    result: &UpdateFirewallPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_firewall_policy_change_protection_response(
    result: &UpdateFirewallPolicyChangeProtectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_logging_configuration_response(
    result: &UpdateLoggingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_proxy_response(result: &UpdateProxyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_proxy_configuration_response(
    result: &UpdateProxyConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_proxy_rule_response(result: &UpdateProxyRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_proxy_rule_group_priorities_response(
    result: &UpdateProxyRuleGroupPrioritiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_proxy_rule_priorities_response(
    result: &UpdateProxyRulePrioritiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_rule_group_response(result: &UpdateRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_subnet_change_protection_response(
    result: &UpdateSubnetChangeProtectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_t_l_s_inspection_configuration_response(
    result: &UpdateTLSInspectionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_network_firewall_transit_gateway_attachment_request(
    body: &[u8],
) -> Result<AcceptNetworkFirewallTransitGatewayAttachmentRequest, String> {
    if body.is_empty() {
        return Ok(AcceptNetworkFirewallTransitGatewayAttachmentRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AcceptNetworkFirewallTransitGatewayAttachment request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_availability_zones_request(
    body: &[u8],
) -> Result<AssociateAvailabilityZonesRequest, String> {
    if body.is_empty() {
        return Ok(AssociateAvailabilityZonesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateAvailabilityZones request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_firewall_policy_request(
    body: &[u8],
) -> Result<AssociateFirewallPolicyRequest, String> {
    if body.is_empty() {
        return Ok(AssociateFirewallPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateFirewallPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_subnets_request(
    body: &[u8],
) -> Result<AssociateSubnetsRequest, String> {
    if body.is_empty() {
        return Ok(AssociateSubnetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateSubnets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_attach_rule_groups_to_proxy_configuration_request(
    body: &[u8],
) -> Result<AttachRuleGroupsToProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(AttachRuleGroupsToProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AttachRuleGroupsToProxyConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_firewall_request(body: &[u8]) -> Result<CreateFirewallRequest, String> {
    if body.is_empty() {
        return Ok(CreateFirewallRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFirewall request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_firewall_policy_request(
    body: &[u8],
) -> Result<CreateFirewallPolicyRequest, String> {
    if body.is_empty() {
        return Ok(CreateFirewallPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFirewallPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_proxy_request(body: &[u8]) -> Result<CreateProxyRequest, String> {
    if body.is_empty() {
        return Ok(CreateProxyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProxy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_proxy_configuration_request(
    body: &[u8],
) -> Result<CreateProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(CreateProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProxyConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_proxy_rule_group_request(
    body: &[u8],
) -> Result<CreateProxyRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateProxyRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProxyRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_proxy_rules_request(
    body: &[u8],
) -> Result<CreateProxyRulesRequest, String> {
    if body.is_empty() {
        return Ok(CreateProxyRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProxyRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_rule_group_request(
    body: &[u8],
) -> Result<CreateRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_t_l_s_inspection_configuration_request(
    body: &[u8],
) -> Result<CreateTLSInspectionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(CreateTLSInspectionConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTLSInspectionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_vpc_endpoint_association_request(
    body: &[u8],
) -> Result<CreateVpcEndpointAssociationRequest, String> {
    if body.is_empty() {
        return Ok(CreateVpcEndpointAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVpcEndpointAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_request(body: &[u8]) -> Result<DeleteFirewallRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewall request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_policy_request(
    body: &[u8],
) -> Result<DeleteFirewallPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewallPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_network_firewall_transit_gateway_attachment_request(
    body: &[u8],
) -> Result<DeleteNetworkFirewallTransitGatewayAttachmentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteNetworkFirewallTransitGatewayAttachmentRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteNetworkFirewallTransitGatewayAttachment request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_proxy_request(body: &[u8]) -> Result<DeleteProxyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProxyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProxy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_proxy_configuration_request(
    body: &[u8],
) -> Result<DeleteProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProxyConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_proxy_rule_group_request(
    body: &[u8],
) -> Result<DeleteProxyRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProxyRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProxyRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_proxy_rules_request(
    body: &[u8],
) -> Result<DeleteProxyRulesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProxyRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProxyRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_rule_group_request(
    body: &[u8],
) -> Result<DeleteRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_t_l_s_inspection_configuration_request(
    body: &[u8],
) -> Result<DeleteTLSInspectionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTLSInspectionConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTLSInspectionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_vpc_endpoint_association_request(
    body: &[u8],
) -> Result<DeleteVpcEndpointAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVpcEndpointAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVpcEndpointAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_firewall_request(
    body: &[u8],
) -> Result<DescribeFirewallRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFirewallRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFirewall request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_firewall_metadata_request(
    body: &[u8],
) -> Result<DescribeFirewallMetadataRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFirewallMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFirewallMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_firewall_policy_request(
    body: &[u8],
) -> Result<DescribeFirewallPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFirewallPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFirewallPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_flow_operation_request(
    body: &[u8],
) -> Result<DescribeFlowOperationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFlowOperationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFlowOperation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_logging_configuration_request(
    body: &[u8],
) -> Result<DescribeLoggingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLoggingConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLoggingConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_proxy_request(body: &[u8]) -> Result<DescribeProxyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProxyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProxy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_proxy_configuration_request(
    body: &[u8],
) -> Result<DescribeProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProxyConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_proxy_rule_request(
    body: &[u8],
) -> Result<DescribeProxyRuleRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProxyRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProxyRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_proxy_rule_group_request(
    body: &[u8],
) -> Result<DescribeProxyRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProxyRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProxyRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_resource_policy_request(
    body: &[u8],
) -> Result<DescribeResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_rule_group_request(
    body: &[u8],
) -> Result<DescribeRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_rule_group_metadata_request(
    body: &[u8],
) -> Result<DescribeRuleGroupMetadataRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRuleGroupMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRuleGroupMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_rule_group_summary_request(
    body: &[u8],
) -> Result<DescribeRuleGroupSummaryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRuleGroupSummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRuleGroupSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_t_l_s_inspection_configuration_request(
    body: &[u8],
) -> Result<DescribeTLSInspectionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTLSInspectionConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeTLSInspectionConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_vpc_endpoint_association_request(
    body: &[u8],
) -> Result<DescribeVpcEndpointAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeVpcEndpointAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeVpcEndpointAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detach_rule_groups_from_proxy_configuration_request(
    body: &[u8],
) -> Result<DetachRuleGroupsFromProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DetachRuleGroupsFromProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DetachRuleGroupsFromProxyConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_availability_zones_request(
    body: &[u8],
) -> Result<DisassociateAvailabilityZonesRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateAvailabilityZonesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateAvailabilityZones request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_subnets_request(
    body: &[u8],
) -> Result<DisassociateSubnetsRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateSubnetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateSubnets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_analysis_report_results_request(
    body: &[u8],
) -> Result<GetAnalysisReportResultsRequest, String> {
    if body.is_empty() {
        return Ok(GetAnalysisReportResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAnalysisReportResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_analysis_reports_request(
    body: &[u8],
) -> Result<ListAnalysisReportsRequest, String> {
    if body.is_empty() {
        return Ok(ListAnalysisReportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAnalysisReports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewall_policies_request(
    body: &[u8],
) -> Result<ListFirewallPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewallPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_firewalls_request(body: &[u8]) -> Result<ListFirewallsRequest, String> {
    if body.is_empty() {
        return Ok(ListFirewallsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFirewalls request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_flow_operation_results_request(
    body: &[u8],
) -> Result<ListFlowOperationResultsRequest, String> {
    if body.is_empty() {
        return Ok(ListFlowOperationResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFlowOperationResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_flow_operations_request(
    body: &[u8],
) -> Result<ListFlowOperationsRequest, String> {
    if body.is_empty() {
        return Ok(ListFlowOperationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFlowOperations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_proxies_request(body: &[u8]) -> Result<ListProxiesRequest, String> {
    if body.is_empty() {
        return Ok(ListProxiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProxies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_proxy_configurations_request(
    body: &[u8],
) -> Result<ListProxyConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(ListProxyConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProxyConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_proxy_rule_groups_request(
    body: &[u8],
) -> Result<ListProxyRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListProxyRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProxyRuleGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_rule_groups_request(body: &[u8]) -> Result<ListRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRuleGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_t_l_s_inspection_configurations_request(
    body: &[u8],
) -> Result<ListTLSInspectionConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(ListTLSInspectionConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTLSInspectionConfigurations request: {e}"))
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
pub fn deserialize_list_vpc_endpoint_associations_request(
    body: &[u8],
) -> Result<ListVpcEndpointAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListVpcEndpointAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVpcEndpointAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reject_network_firewall_transit_gateway_attachment_request(
    body: &[u8],
) -> Result<RejectNetworkFirewallTransitGatewayAttachmentRequest, String> {
    if body.is_empty() {
        return Ok(RejectNetworkFirewallTransitGatewayAttachmentRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RejectNetworkFirewallTransitGatewayAttachment request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_analysis_report_request(
    body: &[u8],
) -> Result<StartAnalysisReportRequest, String> {
    if body.is_empty() {
        return Ok(StartAnalysisReportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartAnalysisReport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_flow_capture_request(
    body: &[u8],
) -> Result<StartFlowCaptureRequest, String> {
    if body.is_empty() {
        return Ok(StartFlowCaptureRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFlowCapture request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_flow_flush_request(body: &[u8]) -> Result<StartFlowFlushRequest, String> {
    if body.is_empty() {
        return Ok(StartFlowFlushRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFlowFlush request: {e}"))
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
pub fn deserialize_update_availability_zone_change_protection_request(
    body: &[u8],
) -> Result<UpdateAvailabilityZoneChangeProtectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAvailabilityZoneChangeProtectionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateAvailabilityZoneChangeProtection request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_analysis_settings_request(
    body: &[u8],
) -> Result<UpdateFirewallAnalysisSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallAnalysisSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallAnalysisSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_delete_protection_request(
    body: &[u8],
) -> Result<UpdateFirewallDeleteProtectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallDeleteProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallDeleteProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_description_request(
    body: &[u8],
) -> Result<UpdateFirewallDescriptionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallDescriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallDescription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_encryption_configuration_request(
    body: &[u8],
) -> Result<UpdateFirewallEncryptionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallEncryptionConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateFirewallEncryptionConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_policy_request(
    body: &[u8],
) -> Result<UpdateFirewallPolicyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFirewallPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_firewall_policy_change_protection_request(
    body: &[u8],
) -> Result<UpdateFirewallPolicyChangeProtectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFirewallPolicyChangeProtectionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateFirewallPolicyChangeProtection request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_logging_configuration_request(
    body: &[u8],
) -> Result<UpdateLoggingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLoggingConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLoggingConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_proxy_request(body: &[u8]) -> Result<UpdateProxyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProxyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProxy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_proxy_configuration_request(
    body: &[u8],
) -> Result<UpdateProxyConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProxyConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProxyConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_proxy_rule_request(
    body: &[u8],
) -> Result<UpdateProxyRuleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProxyRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProxyRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_proxy_rule_group_priorities_request(
    body: &[u8],
) -> Result<UpdateProxyRuleGroupPrioritiesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProxyRuleGroupPrioritiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProxyRuleGroupPriorities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_proxy_rule_priorities_request(
    body: &[u8],
) -> Result<UpdateProxyRulePrioritiesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProxyRulePrioritiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProxyRulePriorities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_rule_group_request(
    body: &[u8],
) -> Result<UpdateRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_subnet_change_protection_request(
    body: &[u8],
) -> Result<UpdateSubnetChangeProtectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSubnetChangeProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSubnetChangeProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_t_l_s_inspection_configuration_request(
    body: &[u8],
) -> Result<UpdateTLSInspectionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTLSInspectionConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTLSInspectionConfiguration request: {e}"))
}
