//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-networkfirewall

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptNetworkFirewallTransitGatewayAttachmentRequest {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    pub transit_gateway_attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptNetworkFirewallTransitGatewayAttachmentResponse {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAvailabilityZonesRequest {
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    pub availability_zone_mappings: Vec<AvailabilityZoneMapping>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneMapping {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    pub availability_zone: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAvailabilityZonesResponse {
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_mappings: Option<Vec<AvailabilityZoneMapping>>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFirewallPolicyRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    pub firewall_policy_arn: String,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFirewallPolicyResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSubnetsRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    pub subnet_mappings: Vec<SubnetMapping>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetMapping {
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSubnetsResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachRuleGroupsToProxyConfigurationRequest {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "RuleGroups")]
    #[serde(default)]
    pub rule_groups: Vec<ProxyRuleGroupAttachment>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleGroupAttachment {
    #[serde(rename = "InsertPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_position: Option<i32>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachRuleGroupsToProxyConfigurationResponse {
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfiguration {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DefaultRulePhaseActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rule_phase_actions: Option<ProxyConfigDefaultRulePhaseActionsRequest>,
    #[serde(rename = "DeleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "RuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<ProxyConfigRuleGroup>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfigDefaultRulePhaseActionsRequest {
    #[serde(rename = "PostRESPONSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_r_e_s_p_o_n_s_e: Option<String>,
    #[serde(rename = "PreDNS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_d_n_s: Option<String>,
    #[serde(rename = "PreREQUEST")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_r_e_q_u_e_s_t: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfigRuleGroup {
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
pub struct CreateFirewallPolicyRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallPolicy")]
    #[serde(default)]
    pub firewall_policy: FirewallPolicy,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    pub firewall_policy_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicy {
    #[serde(rename = "EnableTLSSessionHolding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_t_l_s_session_holding: Option<bool>,
    #[serde(rename = "PolicyVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_variables: Option<PolicyVariables>,
    #[serde(rename = "StatefulDefaultActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_default_actions: Option<Vec<String>>,
    #[serde(rename = "StatefulEngineOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_engine_options: Option<StatefulEngineOptions>,
    #[serde(rename = "StatefulRuleGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_group_references: Option<Vec<StatefulRuleGroupReference>>,
    #[serde(rename = "StatelessCustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_custom_actions: Option<Vec<CustomAction>>,
    #[serde(rename = "StatelessDefaultActions")]
    #[serde(default)]
    pub stateless_default_actions: Vec<String>,
    #[serde(rename = "StatelessFragmentDefaultActions")]
    #[serde(default)]
    pub stateless_fragment_default_actions: Vec<String>,
    #[serde(rename = "StatelessRuleGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rule_group_references: Option<Vec<StatelessRuleGroupReference>>,
    #[serde(rename = "TLSInspectionConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyVariables {
    #[serde(rename = "RuleVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_variables: Option<std::collections::HashMap<String, IPSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSet {
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatefulEngineOptions {
    #[serde(rename = "FlowTimeouts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_timeouts: Option<FlowTimeouts>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<String>,
    #[serde(rename = "StreamExceptionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_exception_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowTimeouts {
    #[serde(rename = "TcpIdleTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_idle_timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatefulRuleGroupReference {
    #[serde(rename = "DeepThreatInspection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_threat_inspection: Option<bool>,
    #[serde(rename = "Override")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<StatefulRuleGroupOverride>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatefulRuleGroupOverride {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAction {
    #[serde(rename = "ActionDefinition")]
    #[serde(default)]
    pub action_definition: ActionDefinition,
    #[serde(rename = "ActionName")]
    #[serde(default)]
    pub action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionDefinition {
    #[serde(rename = "PublishMetricAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_metric_action: Option<PublishMetricAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishMetricAction {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessRuleGroupReference {
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_response: Option<FirewallPolicyResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyResponse {
    #[serde(rename = "ConsumedStatefulDomainCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_stateful_domain_capacity: Option<i32>,
    #[serde(rename = "ConsumedStatefulRuleCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_stateful_rule_capacity: Option<i32>,
    #[serde(rename = "ConsumedStatelessRuleCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_stateless_rule_capacity: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_id: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
    #[serde(rename = "FirewallPolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_status: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NumberOfAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associations: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallRequest {
    #[serde(rename = "AvailabilityZoneChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_change_protection: Option<bool>,
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_mappings: Option<Vec<AvailabilityZoneMapping>>,
    #[serde(rename = "DeleteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnabledAnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_analysis_types: Option<Vec<String>>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    pub firewall_name: String,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    pub firewall_policy_arn: String,
    #[serde(rename = "FirewallPolicyChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_change_protection: Option<bool>,
    #[serde(rename = "SubnetChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_change_protection: Option<bool>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TransitGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFirewallResponse {
    #[serde(rename = "Firewall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Firewall>,
    #[serde(rename = "FirewallStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_status: Option<FirewallStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Firewall {
    #[serde(rename = "AvailabilityZoneChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_change_protection: Option<bool>,
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_mappings: Option<Vec<AvailabilityZoneMapping>>,
    #[serde(rename = "DeleteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnabledAnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_analysis_types: Option<Vec<String>>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_id: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_change_protection: Option<bool>,
    #[serde(rename = "NumberOfAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associations: Option<i32>,
    #[serde(rename = "SubnetChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_change_protection: Option<bool>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TransitGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<String>,
    #[serde(rename = "TransitGatewayOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_owner_account_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallStatus {
    #[serde(rename = "CapacityUsageSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_usage_summary: Option<CapacityUsageSummary>,
    #[serde(rename = "ConfigurationSyncStateSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sync_state_summary: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SyncStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_states: Option<std::collections::HashMap<String, SyncState>>,
    #[serde(rename = "TransitGatewayAttachmentSyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_sync_state: Option<TransitGatewayAttachmentSyncState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityUsageSummary {
    #[serde(rename = "CIDRs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_rs: Option<CIDRSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CIDRSummary {
    #[serde(rename = "AvailableCIDRCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_c_i_d_r_count: Option<i32>,
    #[serde(rename = "IPSetReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_references: Option<std::collections::HashMap<String, IPSetMetadata>>,
    #[serde(rename = "UtilizedCIDRCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilized_c_i_d_r_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSetMetadata {
    #[serde(rename = "ResolvedCIDRCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_c_i_d_r_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SyncState {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, PerObjectStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
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
pub struct PerObjectStatus {
    #[serde(rename = "SyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayAttachmentSyncState {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyConfigurationRequest {
    #[serde(rename = "DefaultRulePhaseActions")]
    #[serde(default)]
    pub default_rule_phase_actions: ProxyConfigDefaultRulePhaseActionsRequest,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    pub proxy_configuration_name: String,
    #[serde(rename = "RuleGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arns: Option<Vec<String>>,
    #[serde(rename = "RuleGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_names: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyConfigurationResponse {
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRequest {
    #[serde(rename = "ListenerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_properties: Option<Vec<ListenerPropertyRequest>>,
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    pub nat_gateway_id: String,
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    pub proxy_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TlsInterceptProperties")]
    #[serde(default)]
    pub tls_intercept_properties: TlsInterceptPropertiesRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerPropertyRequest {
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsInterceptPropertiesRequest {
    #[serde(rename = "PcaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pca_arn: Option<String>,
    #[serde(rename = "TlsInterceptMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_intercept_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyResponse {
    #[serde(rename = "Proxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Proxy>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Proxy {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DeleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<f64>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "ListenerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_properties: Option<Vec<ListenerProperty>>,
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "ProxyModifyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_modify_state: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
    #[serde(rename = "ProxyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TlsInterceptProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_intercept_properties: Option<TlsInterceptProperties>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerProperty {
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsInterceptProperties {
    #[serde(rename = "PcaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pca_arn: Option<String>,
    #[serde(rename = "TlsInterceptMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_intercept_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRuleGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    pub proxy_rule_group_name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<ProxyRulesByRequestPhase>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRulesByRequestPhase {
    #[serde(rename = "PostRESPONSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_r_e_s_p_o_n_s_e: Option<Vec<ProxyRule>>,
    #[serde(rename = "PreDNS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_d_n_s: Option<Vec<ProxyRule>>,
    #[serde(rename = "PreREQUEST")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_r_e_q_u_e_s_t: Option<Vec<ProxyRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ProxyRuleCondition>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleCondition {
    #[serde(rename = "ConditionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_key: Option<String>,
    #[serde(rename = "ConditionOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_operator: Option<String>,
    #[serde(rename = "ConditionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRuleGroupResponse {
    #[serde(rename = "ProxyRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group: Option<ProxyRuleGroup>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleGroup {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DeleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<ProxyRulesByRequestPhase>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRulesRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: CreateProxyRulesByRequestPhase,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRulesByRequestPhase {
    #[serde(rename = "PostRESPONSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_r_e_s_p_o_n_s_e: Option<Vec<CreateProxyRule>>,
    #[serde(rename = "PreDNS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_d_n_s: Option<Vec<CreateProxyRule>>,
    #[serde(rename = "PreREQUEST")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_r_e_q_u_e_s_t: Option<Vec<CreateProxyRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ProxyRuleCondition>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InsertPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_position: Option<i32>,
    #[serde(rename = "ProxyRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProxyRulesResponse {
    #[serde(rename = "ProxyRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group: Option<ProxyRuleGroup>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupRequest {
    #[serde(rename = "AnalyzeRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_rule_group: Option<bool>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    pub capacity: i32,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    pub rule_group_name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(rename = "SourceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<SourceMetadata>,
    #[serde(rename = "SummaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_configuration: Option<SummaryConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroup {
    #[serde(rename = "ReferenceSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_sets: Option<ReferenceSets>,
    #[serde(rename = "RuleVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_variables: Option<RuleVariables>,
    #[serde(rename = "RulesSource")]
    #[serde(default)]
    pub rules_source: RulesSource,
    #[serde(rename = "StatefulRuleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_options: Option<StatefulRuleOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceSets {
    #[serde(rename = "IPSetReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_references: Option<std::collections::HashMap<String, IPSetReference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPSetReference {
    #[serde(rename = "ReferenceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleVariables {
    #[serde(rename = "IPSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_sets: Option<std::collections::HashMap<String, IPSet>>,
    #[serde(rename = "PortSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_sets: Option<std::collections::HashMap<String, PortSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortSet {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RulesSource {
    #[serde(rename = "RulesSourceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_source_list: Option<RulesSourceList>,
    #[serde(rename = "RulesString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_string: Option<String>,
    #[serde(rename = "StatefulRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rules: Option<Vec<StatefulRule>>,
    #[serde(rename = "StatelessRulesAndCustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rules_and_custom_actions: Option<StatelessRulesAndCustomActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RulesSourceList {
    #[serde(rename = "GeneratedRulesType")]
    #[serde(default)]
    pub generated_rules_type: String,
    #[serde(rename = "TargetTypes")]
    #[serde(default)]
    pub target_types: Vec<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatefulRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: Header,
    #[serde(rename = "RuleOptions")]
    #[serde(default)]
    pub rule_options: Vec<RuleOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Header {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: String,
    #[serde(rename = "DestinationPort")]
    #[serde(default)]
    pub destination_port: String,
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "SourcePort")]
    #[serde(default)]
    pub source_port: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleOption {
    #[serde(rename = "Keyword")]
    #[serde(default)]
    pub keyword: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessRulesAndCustomActions {
    #[serde(rename = "CustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_actions: Option<Vec<CustomAction>>,
    #[serde(rename = "StatelessRules")]
    #[serde(default)]
    pub stateless_rules: Vec<StatelessRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessRule {
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "RuleDefinition")]
    #[serde(default)]
    pub rule_definition: RuleDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleDefinition {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "MatchAttributes")]
    #[serde(default)]
    pub match_attributes: MatchAttributes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchAttributes {
    #[serde(rename = "DestinationPorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ports: Option<Vec<PortRange>>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Address>>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,
    #[serde(rename = "SourcePorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ports: Option<Vec<PortRange>>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Address>>,
    #[serde(rename = "TCPFlags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_c_p_flags: Option<Vec<TCPFlagField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRange {
    #[serde(rename = "FromPort")]
    #[serde(default)]
    pub from_port: i32,
    #[serde(rename = "ToPort")]
    #[serde(default)]
    pub to_port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "AddressDefinition")]
    #[serde(default)]
    pub address_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TCPFlagField {
    #[serde(rename = "Flags")]
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(rename = "Masks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatefulRuleOptions {
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceMetadata {
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "SourceUpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SummaryConfiguration {
    #[serde(rename = "RuleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupResponse {
    #[serde(rename = "RuleGroupResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_response: Option<RuleGroupResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupResponse {
    #[serde(rename = "AnalysisResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_results: Option<Vec<AnalysisResult>>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NumberOfAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associations: Option<i32>,
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "RuleGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_status: Option<String>,
    #[serde(rename = "SnsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
    #[serde(rename = "SourceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<SourceMetadata>,
    #[serde(rename = "SummaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_configuration: Option<SummaryConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisResult {
    #[serde(rename = "AnalysisDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_detail: Option<String>,
    #[serde(rename = "IdentifiedRuleIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_rule_ids: Option<Vec<String>>,
    #[serde(rename = "IdentifiedType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTLSInspectionConfigurationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "TLSInspectionConfiguration")]
    #[serde(default)]
    pub t_l_s_inspection_configuration: TLSInspectionConfiguration,
    #[serde(rename = "TLSInspectionConfigurationName")]
    #[serde(default)]
    pub t_l_s_inspection_configuration_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TLSInspectionConfiguration {
    #[serde(rename = "ServerCertificateConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_configurations: Option<Vec<ServerCertificateConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerCertificateConfiguration {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CheckCertificateRevocationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_certificate_revocation_status: Option<CheckCertificateRevocationStatusActions>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ServerCertificateScope>>,
    #[serde(rename = "ServerCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificates: Option<Vec<ServerCertificate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckCertificateRevocationStatusActions {
    #[serde(rename = "RevokedStatusAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_status_action: Option<String>,
    #[serde(rename = "UnknownStatusAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_status_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerCertificateScope {
    #[serde(rename = "DestinationPorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ports: Option<Vec<PortRange>>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Address>>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,
    #[serde(rename = "SourcePorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ports: Option<Vec<PortRange>>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Address>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerCertificate {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTLSInspectionConfigurationResponse {
    #[serde(rename = "TLSInspectionConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_response: Option<TLSInspectionConfigurationResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TLSInspectionConfigurationResponse {
    #[serde(rename = "CertificateAuthority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<TlsCertificateData>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<TlsCertificateData>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NumberOfAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associations: Option<i32>,
    #[serde(rename = "TLSInspectionConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_arn: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_id: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_name: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsCertificateData {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateSerial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_serial: Option<String>,
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
pub struct CreateVpcEndpointAssociationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "SubnetMapping")]
    #[serde(default)]
    pub subnet_mapping: SubnetMapping,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointAssociationResponse {
    #[serde(rename = "VpcEndpointAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association: Option<VpcEndpointAssociation>,
    #[serde(rename = "VpcEndpointAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_status: Option<VpcEndpointAssociationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointAssociation {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "SubnetMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mapping: Option<SubnetMapping>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointAssociationStatus {
    #[serde(rename = "AssociationSyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_sync_state: Option<std::collections::HashMap<String, AZSyncState>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AZSyncState {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallPolicyRequest {
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_response: Option<FirewallPolicyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFirewallResponse {
    #[serde(rename = "Firewall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Firewall>,
    #[serde(rename = "FirewallStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_status: Option<FirewallStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkFirewallTransitGatewayAttachmentRequest {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    pub transit_gateway_attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkFirewallTransitGatewayAttachmentResponse {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyConfigurationRequest {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyConfigurationResponse {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyRequest {
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    pub nat_gateway_id: String,
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyResponse {
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyRuleGroupRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyRuleGroupResponse {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyRulesRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProxyRulesResponse {
    #[serde(rename = "ProxyRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group: Option<ProxyRuleGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleGroupRequest {
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleGroupResponse {
    #[serde(rename = "RuleGroupResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_response: Option<RuleGroupResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTLSInspectionConfigurationRequest {
    #[serde(rename = "TLSInspectionConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_arn: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTLSInspectionConfigurationResponse {
    #[serde(rename = "TLSInspectionConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_response: Option<TLSInspectionConfigurationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointAssociationRequest {
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    pub vpc_endpoint_association_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointAssociationResponse {
    #[serde(rename = "VpcEndpointAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association: Option<VpcEndpointAssociation>,
    #[serde(rename = "VpcEndpointAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_status: Option<VpcEndpointAssociationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallMetadataRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallMetadataResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SupportedAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_availability_zones:
        Option<std::collections::HashMap<String, AvailabilityZoneMetadata>>,
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneMetadata {
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallPolicyRequest {
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy: Option<FirewallPolicy>,
    #[serde(rename = "FirewallPolicyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_response: Option<FirewallPolicyResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFirewallResponse {
    #[serde(rename = "Firewall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Firewall>,
    #[serde(rename = "FirewallStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_status: Option<FirewallStatus>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowOperationRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    pub flow_operation_id: String,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowOperationResponse {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FlowOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation: Option<FlowOperation>,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_id: Option<String>,
    #[serde(rename = "FlowOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_status: Option<String>,
    #[serde(rename = "FlowOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_type: Option<String>,
    #[serde(rename = "FlowRequestTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_request_timestamp: Option<f64>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowOperation {
    #[serde(rename = "FlowFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_filters: Option<Vec<FlowFilter>>,
    #[serde(rename = "MinimumFlowAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_flow_age_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowFilter {
    #[serde(rename = "DestinationAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<Address>,
    #[serde(rename = "DestinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "SourceAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<Address>,
    #[serde(rename = "SourcePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLoggingConfigurationRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLoggingConfigurationResponse {
    #[serde(rename = "EnableMonitoringDashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_monitoring_dashboard: Option<bool>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogDestinationConfigs")]
    #[serde(default)]
    pub log_destination_configs: Vec<LogDestinationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDestinationConfig {
    #[serde(rename = "LogDestination")]
    #[serde(default)]
    pub log_destination: std::collections::HashMap<String, String>,
    #[serde(rename = "LogDestinationType")]
    #[serde(default)]
    pub log_destination_type: String,
    #[serde(rename = "LogType")]
    #[serde(default)]
    pub log_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyConfigurationRequest {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyConfigurationResponse {
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyRequest {
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyResponse {
    #[serde(rename = "Proxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<DescribeProxyResource>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyResource {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DeleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<f64>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "ListenerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_properties: Option<Vec<ListenerProperty>>,
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,
    #[serde(rename = "PrivateDNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_d_n_s_name: Option<String>,
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "ProxyModifyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_modify_state: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
    #[serde(rename = "ProxyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TlsInterceptProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_intercept_properties: Option<TlsInterceptProperties>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "VpcEndpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyRuleGroupRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyRuleGroupResponse {
    #[serde(rename = "ProxyRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group: Option<ProxyRuleGroup>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyRuleRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "ProxyRuleName")]
    #[serde(default)]
    pub proxy_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProxyRuleResponse {
    #[serde(rename = "ProxyRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule: Option<ProxyRule>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupMetadataRequest {
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupMetadataResponse {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "ListingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "StatefulRuleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_options: Option<StatefulRuleOptions>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupRequest {
    #[serde(rename = "AnalyzeRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_rule_group: Option<bool>,
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupResponse {
    #[serde(rename = "RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
    #[serde(rename = "RuleGroupResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_response: Option<RuleGroupResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupSummaryRequest {
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupSummaryResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Summary {
    #[serde(rename = "RuleSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_summaries: Option<Vec<RuleSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleSummary {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "Msg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename = "SID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTLSInspectionConfigurationRequest {
    #[serde(rename = "TLSInspectionConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_arn: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTLSInspectionConfigurationResponse {
    #[serde(rename = "TLSInspectionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration: Option<TLSInspectionConfiguration>,
    #[serde(rename = "TLSInspectionConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_response: Option<TLSInspectionConfigurationResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcEndpointAssociationRequest {
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    pub vpc_endpoint_association_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcEndpointAssociationResponse {
    #[serde(rename = "VpcEndpointAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association: Option<VpcEndpointAssociation>,
    #[serde(rename = "VpcEndpointAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_status: Option<VpcEndpointAssociationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachRuleGroupsFromProxyConfigurationRequest {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "RuleGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arns: Option<Vec<String>>,
    #[serde(rename = "RuleGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_names: Option<Vec<String>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachRuleGroupsFromProxyConfigurationResponse {
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAvailabilityZonesRequest {
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    pub availability_zone_mappings: Vec<AvailabilityZoneMapping>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAvailabilityZonesResponse {
    #[serde(rename = "AvailabilityZoneMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_mappings: Option<Vec<AvailabilityZoneMapping>>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSubnetsRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSubnetsResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnalysisReportResultsRequest {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    pub analysis_report_id: String,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
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
pub struct GetAnalysisReportResultsResponse {
    #[serde(rename = "AnalysisReportResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_results: Option<Vec<AnalysisTypeReportResult>>,
    #[serde(rename = "AnalysisType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_type: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisTypeReportResult {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "FirstAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_accessed: Option<f64>,
    #[serde(rename = "Hits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Hits>,
    #[serde(rename = "LastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<f64>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "UniqueSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_sources: Option<UniqueSources>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hits {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UniqueSources {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalysisReportsRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
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
pub struct ListAnalysisReportsResponse {
    #[serde(rename = "AnalysisReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_reports: Option<Vec<AnalysisReport>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisReport {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_id: Option<String>,
    #[serde(rename = "AnalysisType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_type: Option<String>,
    #[serde(rename = "ReportTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallPoliciesRequest {
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
pub struct ListFirewallPoliciesResponse {
    #[serde(rename = "FirewallPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policies: Option<Vec<FirewallPolicyMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFirewallsResponse {
    #[serde(rename = "Firewalls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<FirewallMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallMetadata {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowOperationResultsRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    pub flow_operation_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowOperationResultsResponse {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_id: Option<String>,
    #[serde(rename = "FlowOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_status: Option<String>,
    #[serde(rename = "FlowRequestTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_request_timestamp: Option<f64>,
    #[serde(rename = "Flows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<Flow>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Flow {
    #[serde(rename = "Age")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(rename = "ByteCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    #[serde(rename = "DestinationAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<Address>,
    #[serde(rename = "DestinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    #[serde(rename = "PacketCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_count: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SourceAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<Address>,
    #[serde(rename = "SourcePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowOperationsRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "FlowOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowOperationsResponse {
    #[serde(rename = "FlowOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operations: Option<Vec<FlowOperationMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowOperationMetadata {
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_id: Option<String>,
    #[serde(rename = "FlowOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_status: Option<String>,
    #[serde(rename = "FlowOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_type: Option<String>,
    #[serde(rename = "FlowRequestTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_request_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProxiesRequest {
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
pub struct ListProxiesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Proxies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxies: Option<Vec<ProxyMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProxyConfigurationsRequest {
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
pub struct ListProxyConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProxyConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configurations: Option<Vec<ProxyConfigurationMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyConfigurationMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProxyRuleGroupsRequest {
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
pub struct ListProxyRuleGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProxyRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_groups: Option<Vec<ProxyRuleGroupMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleGroupMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsRequest {
    #[serde(rename = "ManagedType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "SubscriptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<RuleGroupMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTLSInspectionConfigurationsRequest {
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
pub struct ListTLSInspectionConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TLSInspectionConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configurations: Option<Vec<TLSInspectionConfigurationMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TLSInspectionConfigurationMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct ListVpcEndpointAssociationsRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
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
pub struct ListVpcEndpointAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcEndpointAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_associations: Option<Vec<VpcEndpointAssociationMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointAssociationMetadata {
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectNetworkFirewallTransitGatewayAttachmentRequest {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    pub transit_gateway_attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectNetworkFirewallTransitGatewayAttachmentResponse {
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAnalysisReportRequest {
    #[serde(rename = "AnalysisType")]
    #[serde(default)]
    pub analysis_type: String,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAnalysisReportResponse {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowCaptureRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "FlowFilters")]
    #[serde(default)]
    pub flow_filters: Vec<FlowFilter>,
    #[serde(rename = "MinimumFlowAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_flow_age_in_seconds: Option<i32>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowCaptureResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_id: Option<String>,
    #[serde(rename = "FlowOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowFlushRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    pub firewall_arn: String,
    #[serde(rename = "FlowFilters")]
    #[serde(default)]
    pub flow_filters: Vec<FlowFilter>,
    #[serde(rename = "MinimumFlowAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_flow_age_in_seconds: Option<i32>,
    #[serde(rename = "VpcEndpointAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_association_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowFlushResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FlowOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_id: Option<String>,
    #[serde(rename = "FlowOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_operation_status: Option<String>,
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
pub struct UpdateAvailabilityZoneChangeProtectionRequest {
    #[serde(rename = "AvailabilityZoneChangeProtection")]
    #[serde(default)]
    pub availability_zone_change_protection: bool,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAvailabilityZoneChangeProtectionResponse {
    #[serde(rename = "AvailabilityZoneChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_change_protection: Option<bool>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallAnalysisSettingsRequest {
    #[serde(rename = "EnabledAnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_analysis_types: Option<Vec<String>>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallAnalysisSettingsResponse {
    #[serde(rename = "EnabledAnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_analysis_types: Option<Vec<String>>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDeleteProtectionRequest {
    #[serde(rename = "DeleteProtection")]
    #[serde(default)]
    pub delete_protection: bool,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDeleteProtectionResponse {
    #[serde(rename = "DeleteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<bool>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDescriptionRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallDescriptionResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallEncryptionConfigurationRequest {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallEncryptionConfigurationResponse {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallPolicyChangeProtectionRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyChangeProtection")]
    #[serde(default)]
    pub firewall_policy_change_protection: bool,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallPolicyChangeProtectionResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_change_protection: Option<bool>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallPolicyRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "FirewallPolicy")]
    #[serde(default)]
    pub firewall_policy: FirewallPolicy,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_response: Option<FirewallPolicyResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggingConfigurationRequest {
    #[serde(rename = "EnableMonitoringDashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_monitoring_dashboard: Option<bool>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggingConfigurationResponse {
    #[serde(rename = "EnableMonitoringDashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_monitoring_dashboard: Option<bool>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyConfigurationRequest {
    #[serde(rename = "DefaultRulePhaseActions")]
    #[serde(default)]
    pub default_rule_phase_actions: ProxyConfigDefaultRulePhaseActionsRequest,
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyConfigurationResponse {
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRequest {
    #[serde(rename = "ListenerPropertiesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_properties_to_add: Option<Vec<ListenerPropertyRequest>>,
    #[serde(rename = "ListenerPropertiesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_properties_to_remove: Option<Vec<ListenerPropertyRequest>>,
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    pub nat_gateway_id: String,
    #[serde(rename = "ProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_arn: Option<String>,
    #[serde(rename = "ProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_name: Option<String>,
    #[serde(rename = "TlsInterceptProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_intercept_properties: Option<TlsInterceptPropertiesRequest>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyResponse {
    #[serde(rename = "Proxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Proxy>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRuleGroupPrioritiesRequest {
    #[serde(rename = "ProxyConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_arn: Option<String>,
    #[serde(rename = "ProxyConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_name: Option<String>,
    #[serde(rename = "RuleGroups")]
    #[serde(default)]
    pub rule_groups: Vec<ProxyRuleGroupPriority>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleGroupPriority {
    #[serde(rename = "NewPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_position: Option<i32>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRuleGroupPrioritiesResponse {
    #[serde(rename = "ProxyRuleGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_groups: Option<Vec<ProxyRuleGroupPriorityResult>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRuleGroupPriorityResult {
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRulePrioritiesRequest {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "RuleGroupRequestPhase")]
    #[serde(default)]
    pub rule_group_request_phase: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<ProxyRulePriority>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProxyRulePriority {
    #[serde(rename = "NewPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_position: Option<i32>,
    #[serde(rename = "ProxyRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRulePrioritiesResponse {
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "RuleGroupRequestPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_request_phase: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ProxyRulePriority>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRuleRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AddConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_conditions: Option<Vec<ProxyRuleCondition>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProxyRuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_arn: Option<String>,
    #[serde(rename = "ProxyRuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule_group_name: Option<String>,
    #[serde(rename = "ProxyRuleName")]
    #[serde(default)]
    pub proxy_rule_name: String,
    #[serde(rename = "RemoveConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_conditions: Option<Vec<ProxyRuleCondition>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProxyRuleResponse {
    #[serde(rename = "ProxyRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_rule: Option<ProxyRule>,
    #[serde(rename = "RemovedConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_conditions: Option<Vec<ProxyRuleCondition>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleGroupRequest {
    #[serde(rename = "AnalyzeRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_rule_group: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(rename = "SourceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<SourceMetadata>,
    #[serde(rename = "SummaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_configuration: Option<SummaryConfiguration>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleGroupResponse {
    #[serde(rename = "RuleGroupResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_response: Option<RuleGroupResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubnetChangeProtectionRequest {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetChangeProtection")]
    #[serde(default)]
    pub subnet_change_protection: bool,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubnetChangeProtectionResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "SubnetChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_change_protection: Option<bool>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTLSInspectionConfigurationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "TLSInspectionConfiguration")]
    #[serde(default)]
    pub t_l_s_inspection_configuration: TLSInspectionConfiguration,
    #[serde(rename = "TLSInspectionConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_arn: Option<String>,
    #[serde(rename = "TLSInspectionConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_name: Option<String>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTLSInspectionConfigurationResponse {
    #[serde(rename = "TLSInspectionConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_inspection_configuration_response: Option<TLSInspectionConfigurationResponse>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_token: Option<String>,
}
