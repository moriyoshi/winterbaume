//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53resolver

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFirewallRuleGroupRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
    #[serde(rename = "MutationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_protection: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFirewallRuleGroupResponse {
    #[serde(rename = "FirewallRuleGroupAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_association: Option<FirewallRuleGroupAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallRuleGroupAssociation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagedOwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_owner_name: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "MutationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_protection: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverEndpointIpAddressRequest {
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    pub ip_address: IpAddressUpdate,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAddressUpdate {
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "IpId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_id: Option<String>,
    #[serde(rename = "Ipv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverEndpointIpAddressResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverEndpoint {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Direction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "HostVPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_v_p_c_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IpAddressCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_count: Option<i32>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "PreferredInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_instance_type: Option<String>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "ResolverEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_type: Option<String>,
    #[serde(rename = "RniEnhancedMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rni_enhanced_metrics_enabled: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TargetNameServerMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name_server_metrics_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverQueryLogConfigRequest {
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(default)]
    pub resolver_query_log_config_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverQueryLogConfigResponse {
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverQueryLogConfigAssociation {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverRuleRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    pub resolver_rule_id: String,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    pub v_p_c_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResolverRuleResponse {
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverRuleAssociation {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallDomainListRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallDomainListResponse {
    #[serde(rename = "FirewallDomainList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list: Option<FirewallDomainList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallDomainList {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DomainCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_count: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagedOwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_owner_name: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallRuleGroupRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallRuleGroupResponse {
    #[serde(rename = "FirewallRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group: Option<FirewallRuleGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallRuleGroup {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "RuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_count: Option<i32>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallRuleRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "BlockOverrideDnsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_dns_type: Option<String>,
    #[serde(rename = "BlockOverrideDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_domain: Option<String>,
    #[serde(rename = "BlockOverrideTtl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_ttl: Option<i32>,
    #[serde(rename = "BlockResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_response: Option<String>,
    #[serde(rename = "ConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "DnsThreatProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_threat_protection: Option<String>,
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list_id: Option<String>,
    #[serde(rename = "FirewallDomainRedirectionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_redirection_action: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "Qtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtype: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallRuleResponse {
    #[serde(rename = "FirewallRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule: Option<FirewallRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "BlockOverrideDnsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_dns_type: Option<String>,
    #[serde(rename = "BlockOverrideDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_domain: Option<String>,
    #[serde(rename = "BlockOverrideTtl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_ttl: Option<i32>,
    #[serde(rename = "BlockResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_response: Option<String>,
    #[serde(rename = "ConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DnsThreatProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_threat_protection: Option<String>,
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list_id: Option<String>,
    #[serde(rename = "FirewallDomainRedirectionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_redirection_action: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_id: Option<String>,
    #[serde(rename = "FirewallThreatProtectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_threat_protection_id: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Qtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtype: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutpostResolverRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    pub outpost_arn: String,
    #[serde(rename = "PreferredInstanceType")]
    #[serde(default)]
    pub preferred_instance_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutpostResolverResponse {
    #[serde(rename = "OutpostResolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_resolver: Option<OutpostResolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutpostResolver {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "PreferredInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_instance_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverEndpointRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "IpAddresses")]
    #[serde(default)]
    pub ip_addresses: Vec<IpAddressRequest>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "PreferredInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_instance_type: Option<String>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "ResolverEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_type: Option<String>,
    #[serde(rename = "RniEnhancedMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rni_enhanced_metrics_enabled: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetNameServerMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name_server_metrics_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAddressRequest {
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Ipv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverEndpointResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverQueryLogConfigRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "DestinationArn")]
    #[serde(default)]
    pub destination_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverQueryLogConfigResponse {
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverQueryLogConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssociationCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_count: Option<i32>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverRuleRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    pub creator_request_id: String,
    #[serde(rename = "DelegationRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_record: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    #[serde(rename = "RuleType")]
    #[serde(default)]
    pub rule_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetAddress {
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Ipv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ServerNameIndication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name_indication: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverRuleResponse {
    #[serde(rename = "ResolverRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverRule {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DelegationRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_record: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    #[serde(rename = "RuleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TargetIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallDomainListRequest {
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    pub firewall_domain_list_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallDomainListResponse {
    #[serde(rename = "FirewallDomainList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list: Option<FirewallDomainList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallRuleGroupRequest {
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallRuleGroupResponse {
    #[serde(rename = "FirewallRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group: Option<FirewallRuleGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallRuleRequest {
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list_id: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
    #[serde(rename = "FirewallThreatProtectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_threat_protection_id: Option<String>,
    #[serde(rename = "Qtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtype: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallRuleResponse {
    #[serde(rename = "FirewallRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule: Option<FirewallRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutpostResolverRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutpostResolverResponse {
    #[serde(rename = "OutpostResolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_resolver: Option<OutpostResolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverEndpointRequest {
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverEndpointResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverQueryLogConfigRequest {
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(default)]
    pub resolver_query_log_config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverQueryLogConfigResponse {
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverRuleRequest {
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    pub resolver_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverRuleResponse {
    #[serde(rename = "ResolverRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFirewallRuleGroupRequest {
    #[serde(rename = "FirewallRuleGroupAssociationId")]
    #[serde(default)]
    pub firewall_rule_group_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFirewallRuleGroupResponse {
    #[serde(rename = "FirewallRuleGroupAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_association: Option<FirewallRuleGroupAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverEndpointIpAddressRequest {
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    pub ip_address: IpAddressUpdate,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverEndpointIpAddressResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverQueryLogConfigRequest {
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(default)]
    pub resolver_query_log_config_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverQueryLogConfigResponse {
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverRuleRequest {
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    pub resolver_rule_id: String,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    pub v_p_c_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResolverRuleResponse {
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallConfigRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallConfigResponse {
    #[serde(rename = "FirewallConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_config: Option<FirewallConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallConfig {
    #[serde(rename = "FirewallFailOpen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_fail_open: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallDomainListRequest {
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    pub firewall_domain_list_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallDomainListResponse {
    #[serde(rename = "FirewallDomainList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list: Option<FirewallDomainList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupAssociationRequest {
    #[serde(rename = "FirewallRuleGroupAssociationId")]
    #[serde(default)]
    pub firewall_rule_group_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupAssociationResponse {
    #[serde(rename = "FirewallRuleGroupAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_association: Option<FirewallRuleGroupAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupPolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupPolicyResponse {
    #[serde(rename = "FirewallRuleGroupPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupRequest {
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFirewallRuleGroupResponse {
    #[serde(rename = "FirewallRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group: Option<FirewallRuleGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostResolverRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostResolverResponse {
    #[serde(rename = "OutpostResolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_resolver: Option<OutpostResolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverConfigRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverConfigResponse {
    #[serde(rename = "ResolverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_config: Option<ResolverConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverConfig {
    #[serde(rename = "AutodefinedReverse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autodefined_reverse: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverDnssecConfigRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverDnssecConfigResponse {
    #[serde(rename = "ResolverDNSSECConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_d_n_s_s_e_c_config: Option<ResolverDnssecConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverDnssecConfig {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverEndpointRequest {
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverEndpointResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigAssociationRequest {
    #[serde(rename = "ResolverQueryLogConfigAssociationId")]
    #[serde(default)]
    pub resolver_query_log_config_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigAssociationResponse {
    #[serde(rename = "ResolverQueryLogConfigAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_association: Option<ResolverQueryLogConfigAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigPolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigPolicyResponse {
    #[serde(rename = "ResolverQueryLogConfigPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigRequest {
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(default)]
    pub resolver_query_log_config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverQueryLogConfigResponse {
    #[serde(rename = "ResolverQueryLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config: Option<ResolverQueryLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRuleAssociationRequest {
    #[serde(rename = "ResolverRuleAssociationId")]
    #[serde(default)]
    pub resolver_rule_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRuleAssociationResponse {
    #[serde(rename = "ResolverRuleAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_association: Option<ResolverRuleAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRulePolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRulePolicyResponse {
    #[serde(rename = "ResolverRulePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRuleRequest {
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    pub resolver_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRuleResponse {
    #[serde(rename = "ResolverRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFirewallDomainsRequest {
    #[serde(rename = "DomainFileUrl")]
    #[serde(default)]
    pub domain_file_url: String,
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    pub firewall_domain_list_id: String,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFirewallDomainsResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallConfigsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallConfigsResponse {
    #[serde(rename = "FirewallConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_configs: Option<Vec<FirewallConfig>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallDomainListsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallDomainListsResponse {
    #[serde(rename = "FirewallDomainLists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_lists: Option<Vec<FirewallDomainListMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallDomainListMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagedOwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_owner_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallDomainsRequest {
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    pub firewall_domain_list_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallDomainsResponse {
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRuleGroupAssociationsRequest {
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRuleGroupAssociationsResponse {
    #[serde(rename = "FirewallRuleGroupAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_associations: Option<Vec<FirewallRuleGroupAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRuleGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRuleGroupsResponse {
    #[serde(rename = "FirewallRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_groups: Option<Vec<FirewallRuleGroupMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallRuleGroupMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRulesRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallRulesResponse {
    #[serde(rename = "FirewallRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rules: Option<Vec<FirewallRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutpostResolversRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutpostResolversResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_resolvers: Option<Vec<OutpostResolver>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverConfigsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_configs: Option<Vec<ResolverConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverDnssecConfigsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverDnssecConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverDnssecConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_dnssec_configs: Option<Vec<ResolverDnssecConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverEndpointIpAddressesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverEndpointIpAddressesResponse {
    #[serde(rename = "IpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<IpAddressResponse>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAddressResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "IpId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_id: Option<String>,
    #[serde(rename = "Ipv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(rename = "ModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverEndpointsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverEndpointsResponse {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoints: Option<Vec<ResolverEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverQueryLogConfigAssociationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverQueryLogConfigAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverQueryLogConfigAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_config_associations: Option<Vec<ResolverQueryLogConfigAssociation>>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "TotalFilteredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverQueryLogConfigsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverQueryLogConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverQueryLogConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_query_log_configs: Option<Vec<ResolverQueryLogConfig>>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "TotalFilteredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverRuleAssociationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverRuleAssociationsResponse {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverRuleAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule_associations: Option<Vec<ResolverRuleAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverRulesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolverRulesResponse {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResolverRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rules: Option<Vec<ResolverRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFirewallRuleGroupPolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "FirewallRuleGroupPolicy")]
    #[serde(default)]
    pub firewall_rule_group_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFirewallRuleGroupPolicyResponse {
    #[serde(rename = "ReturnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResolverQueryLogConfigPolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "ResolverQueryLogConfigPolicy")]
    #[serde(default)]
    pub resolver_query_log_config_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResolverQueryLogConfigPolicyResponse {
    #[serde(rename = "ReturnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResolverRulePolicyRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "ResolverRulePolicy")]
    #[serde(default)]
    pub resolver_rule_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResolverRulePolicyResponse {
    #[serde(rename = "ReturnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallConfigRequest {
    #[serde(rename = "FirewallFailOpen")]
    #[serde(default)]
    pub firewall_fail_open: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallConfigResponse {
    #[serde(rename = "FirewallConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_config: Option<FirewallConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDomainsRequest {
    #[serde(rename = "Domains")]
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    pub firewall_domain_list_id: String,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDomainsResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallRuleGroupAssociationRequest {
    #[serde(rename = "FirewallRuleGroupAssociationId")]
    #[serde(default)]
    pub firewall_rule_group_association_id: String,
    #[serde(rename = "MutationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_protection: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallRuleGroupAssociationResponse {
    #[serde(rename = "FirewallRuleGroupAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule_group_association: Option<FirewallRuleGroupAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallRuleRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "BlockOverrideDnsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_dns_type: Option<String>,
    #[serde(rename = "BlockOverrideDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_domain: Option<String>,
    #[serde(rename = "BlockOverrideTtl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_override_ttl: Option<i32>,
    #[serde(rename = "BlockResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_response: Option<String>,
    #[serde(rename = "ConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<String>,
    #[serde(rename = "DnsThreatProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_threat_protection: Option<String>,
    #[serde(rename = "FirewallDomainListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_list_id: Option<String>,
    #[serde(rename = "FirewallDomainRedirectionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_domain_redirection_action: Option<String>,
    #[serde(rename = "FirewallRuleGroupId")]
    #[serde(default)]
    pub firewall_rule_group_id: String,
    #[serde(rename = "FirewallThreatProtectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_threat_protection_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Qtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtype: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallRuleResponse {
    #[serde(rename = "FirewallRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_rule: Option<FirewallRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOutpostResolverRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PreferredInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_instance_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOutpostResolverResponse {
    #[serde(rename = "OutpostResolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_resolver: Option<OutpostResolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverConfigRequest {
    #[serde(rename = "AutodefinedReverseFlag")]
    #[serde(default)]
    pub autodefined_reverse_flag: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverConfigResponse {
    #[serde(rename = "ResolverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_config: Option<ResolverConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverDnssecConfigRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Validation")]
    #[serde(default)]
    pub validation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverDnssecConfigResponse {
    #[serde(rename = "ResolverDNSSECConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_d_n_s_s_e_c_config: Option<ResolverDnssecConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverEndpointRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    pub resolver_endpoint_id: String,
    #[serde(rename = "ResolverEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_type: Option<String>,
    #[serde(rename = "RniEnhancedMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rni_enhanced_metrics_enabled: Option<bool>,
    #[serde(rename = "TargetNameServerMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name_server_metrics_enabled: Option<bool>,
    #[serde(rename = "UpdateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_ip_addresses: Option<Vec<UpdateIpAddress>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIpAddress {
    #[serde(rename = "IpId")]
    #[serde(default)]
    pub ip_id: String,
    #[serde(rename = "Ipv6")]
    #[serde(default)]
    pub ipv6: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverEndpointResponse {
    #[serde(rename = "ResolverEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint: Option<ResolverEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverRuleRequest {
    #[serde(rename = "Config")]
    #[serde(default)]
    pub config: ResolverRuleConfig,
    #[serde(rename = "ResolverRuleId")]
    #[serde(default)]
    pub resolver_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolverRuleConfig {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResolverEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<String>,
    #[serde(rename = "TargetIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverRuleResponse {
    #[serde(rename = "ResolverRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_rule: Option<ResolverRule>,
}
