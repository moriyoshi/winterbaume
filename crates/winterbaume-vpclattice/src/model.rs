//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-vpclattice

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRuleRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(default)]
    pub rules: Vec<RuleUpdate>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleUpdate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ruleIdentifier")]
    #[serde(default)]
    pub rule_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleAction {
    #[serde(rename = "fixedResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_response: Option<FixedResponseAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<ForwardAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FixedResponseAction {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForwardAction {
    #[serde(rename = "targetGroups")]
    #[serde(default)]
    pub target_groups: Vec<WeightedTargetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeightedTargetGroup {
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleMatch {
    #[serde(rename = "httpMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_match: Option<HttpMatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpMatch {
    #[serde(rename = "headerMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_matches: Option<Vec<HeaderMatch>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "pathMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_match: Option<PathMatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderMatch {
    #[serde(rename = "caseSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: HeaderMatchType,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderMatchType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathMatch {
    #[serde(rename = "caseSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: PathMatchType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathMatchType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<RuleUpdateSuccess>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful: Option<Vec<RuleUpdateFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleUpdateSuccess {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleUpdateFailure {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "ruleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessLogSubscriptionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    pub destination_arn: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
    #[serde(rename = "serviceNetworkLogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_log_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessLogSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "serviceNetworkLogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_log_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateListenerRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    pub default_action: RuleAction,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateListenerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceConfigurationRequest {
    #[serde(rename = "allowAssociationToShareableServiceNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_association_to_shareable_service_network: Option<bool>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "domainVerificationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_identifier: Option<String>,
    #[serde(rename = "groupDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "portRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "resourceConfigurationDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_definition: Option<ResourceConfigurationDefinition>,
    #[serde(rename = "resourceConfigurationGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_identifier: Option<String>,
    #[serde(rename = "resourceGatewayIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceConfigurationDefinition {
    #[serde(rename = "arnResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn_resource: Option<ArnResource>,
    #[serde(rename = "dnsResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resource: Option<DnsResource>,
    #[serde(rename = "ipResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_resource: Option<IpResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArnResource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsResource {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpResource {
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceConfigurationResponse {
    #[serde(rename = "allowAssociationToShareableServiceNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_association_to_shareable_service_network: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "domainVerificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_arn: Option<String>,
    #[serde(rename = "domainVerificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_id: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "groupDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "portRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "resourceConfigurationDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_definition: Option<ResourceConfigurationDefinition>,
    #[serde(rename = "resourceConfigurationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_id: Option<String>,
    #[serde(rename = "resourceGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceGatewayRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ipv4AddressesPerEni")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses_per_eni: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ipv4AddressesPerEni")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses_per_eni: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleRequest {
    #[serde(default)]
    pub action: RuleAction,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: RuleMatch,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkRequest {
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sharingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_config: Option<SharingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharingConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkResourceAssociationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    pub resource_configuration_identifier: String,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkResourceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sharingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_config: Option<SharingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkServiceAssociationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkServiceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsEntry {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "hostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkVpcAssociationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dnsOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<DnsOptions>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    pub vpc_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsOptions {
    #[serde(rename = "privateDnsPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_preference: Option<String>,
    #[serde(rename = "privateDnsSpecifiedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_specified_domains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceNetworkVpcAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dnsOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<DnsOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceRequest {
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTargetGroupRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TargetGroupConfig>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupConfig {
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckConfig>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "lambdaEventStructureVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_event_structure_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "healthCheckIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i32>,
    #[serde(rename = "healthCheckTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout_seconds: Option<i32>,
    #[serde(rename = "healthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<Matcher>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(rename = "unhealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Matcher {
    #[serde(rename = "httpCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTargetGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TargetGroupConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessLogSubscriptionRequest {
    #[serde(rename = "accessLogSubscriptionIdentifier")]
    #[serde(default)]
    pub access_log_subscription_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessLogSubscriptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthPolicyRequest {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainVerificationRequest {
    #[serde(rename = "domainVerificationIdentifier")]
    #[serde(default)]
    pub domain_verification_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainVerificationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteListenerRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteListenerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceConfigurationRequest {
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    pub resource_configuration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceEndpointAssociationRequest {
    #[serde(rename = "resourceEndpointAssociationIdentifier")]
    #[serde(default)]
    pub resource_endpoint_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceEndpointAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
    #[serde(rename = "resourceConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_id: Option<String>,
    #[serde(rename = "vpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceGatewayRequest {
    #[serde(rename = "resourceGatewayIdentifier")]
    #[serde(default)]
    pub resource_gateway_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "ruleIdentifier")]
    #[serde(default)]
    pub rule_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkRequest {
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkResourceAssociationRequest {
    #[serde(rename = "serviceNetworkResourceAssociationIdentifier")]
    #[serde(default)]
    pub service_network_resource_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkResourceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkServiceAssociationRequest {
    #[serde(rename = "serviceNetworkServiceAssociationIdentifier")]
    #[serde(default)]
    pub service_network_service_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkServiceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkVpcAssociationRequest {
    #[serde(rename = "serviceNetworkVpcAssociationIdentifier")]
    #[serde(default)]
    pub service_network_vpc_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceNetworkVpcAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceRequest {
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetGroupRequest {
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTargetsRequest {
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTargetsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<Target>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful: Option<Vec<TargetFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetFailure {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessLogSubscriptionRequest {
    #[serde(rename = "accessLogSubscriptionIdentifier")]
    #[serde(default)]
    pub access_log_subscription_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessLogSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "serviceNetworkLogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_log_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthPolicyRequest {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthPolicyResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainVerificationRequest {
    #[serde(rename = "domainVerificationIdentifier")]
    #[serde(default)]
    pub domain_verification_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainVerificationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastVerifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "txtMethodConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_method_config: Option<TxtMethodConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TxtMethodConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetListenerRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetListenerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceConfigurationRequest {
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    pub resource_configuration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceConfigurationResponse {
    #[serde(rename = "allowAssociationToShareableServiceNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_association_to_shareable_service_network: Option<bool>,
    #[serde(rename = "amazonManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "domainVerificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_arn: Option<String>,
    #[serde(rename = "domainVerificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_id: Option<String>,
    #[serde(rename = "domainVerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_status: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "groupDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "portRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "resourceConfigurationDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_definition: Option<ResourceConfigurationDefinition>,
    #[serde(rename = "resourceConfigurationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_id: Option<String>,
    #[serde(rename = "resourceGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceGatewayRequest {
    #[serde(rename = "resourceGatewayIdentifier")]
    #[serde(default)]
    pub resource_gateway_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ipv4AddressesPerEni")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses_per_eni: Option<i32>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuleRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "ruleIdentifier")]
    #[serde(default)]
    pub rule_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkRequest {
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkResourceAssociationRequest {
    #[serde(rename = "serviceNetworkResourceAssociationIdentifier")]
    #[serde(default)]
    pub service_network_resource_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkResourceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(rename = "domainVerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_status: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isManagedAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed_association: Option<bool>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "privateDnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_entry: Option<DnsEntry>,
    #[serde(rename = "resourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
    #[serde(rename = "resourceConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_id: Option<String>,
    #[serde(rename = "resourceConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_name: Option<String>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfAssociatedServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_services: Option<i64>,
    #[serde(rename = "numberOfAssociatedVPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_v_p_cs: Option<i64>,
    #[serde(rename = "sharingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_config: Option<SharingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkServiceAssociationRequest {
    #[serde(rename = "serviceNetworkServiceAssociationIdentifier")]
    #[serde(default)]
    pub service_network_service_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkServiceAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkVpcAssociationRequest {
    #[serde(rename = "serviceNetworkVpcAssociationIdentifier")]
    #[serde(default)]
    pub service_network_vpc_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceNetworkVpcAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dnsOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<DnsOptions>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceRequest {
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetGroupRequest {
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TargetGroupConfig>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessLogSubscriptionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessLogSubscriptionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AccessLogSubscriptionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessLogSubscriptionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "serviceNetworkLogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_log_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainVerificationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainVerificationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainVerificationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainVerificationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastVerifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "txtMethodConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_method_config: Option<TxtMethodConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListListenersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListListenersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ListenerSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceConfigurationsRequest {
    #[serde(rename = "domainVerificationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_identifier: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceConfigurationGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_identifier: Option<String>,
    #[serde(rename = "resourceGatewayIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ResourceConfigurationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceConfigurationSummary {
    #[serde(rename = "amazonManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "domainVerificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_verification_id: Option<String>,
    #[serde(rename = "groupDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceConfigurationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_id: Option<String>,
    #[serde(rename = "resourceGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceEndpointAssociationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    pub resource_configuration_identifier: String,
    #[serde(rename = "resourceEndpointAssociationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_endpoint_association_identifier: Option<String>,
    #[serde(rename = "vpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "vpcEndpointOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceEndpointAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ResourceEndpointAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceEndpointAssociationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
    #[serde(rename = "resourceConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_id: Option<String>,
    #[serde(rename = "resourceConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_name: Option<String>,
    #[serde(rename = "vpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "vpcEndpointOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceGatewaysRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceGatewaysResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ResourceGatewaySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceGatewaySummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ipv4AddressesPerEni")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses_per_eni: Option<i32>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesRequest {
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RuleSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkResourceAssociationsRequest {
    #[serde(rename = "includeChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_identifier: Option<String>,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkResourceAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceNetworkResourceAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNetworkResourceAssociationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isManagedAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed_association: Option<bool>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "privateDnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_entry: Option<DnsEntry>,
    #[serde(rename = "resourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
    #[serde(rename = "resourceConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_id: Option<String>,
    #[serde(rename = "resourceConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_name: Option<String>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkServiceAssociationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<String>,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkServiceAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceNetworkServiceAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNetworkServiceAssociationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkVpcAssociationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_identifier: Option<String>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkVpcAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceNetworkVpcAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNetworkVpcAssociationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dnsOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<DnsOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "privateDnsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_enabled: Option<bool>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(rename = "serviceNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_id: Option<String>,
    #[serde(rename = "serviceNetworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkVpcEndpointAssociationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworkVpcEndpointAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceNetworkEndpointAssociation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNetworkEndpointAssociation {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "serviceNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "vpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "vpcEndpointOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner_id: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworksRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceNetworksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceNetworkSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNetworkSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfAssociatedResourceConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_resource_configurations: Option<i64>,
    #[serde(rename = "numberOfAssociatedServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_services: Option<i64>,
    #[serde(rename = "numberOfAssociatedVPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_v_p_cs: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "dnsEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetGroupsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetGroupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_type: Option<String>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetGroupsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TargetGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "lambdaEventStructureVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_event_structure_version: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vpcIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TargetSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "reasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAuthPolicyRequest {
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAuthPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTargetsRequest {
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTargetsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<Target>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful: Option<Vec<TargetFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainVerificationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainVerificationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "txtMethodConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_method_config: Option<TxtMethodConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessLogSubscriptionRequest {
    #[serde(rename = "accessLogSubscriptionIdentifier")]
    #[serde(default)]
    pub access_log_subscription_identifier: String,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    pub destination_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessLogSubscriptionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateListenerRequest {
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    pub default_action: RuleAction,
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateListenerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceConfigurationRequest {
    #[serde(rename = "allowAssociationToShareableServiceNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_association_to_shareable_service_network: Option<bool>,
    #[serde(rename = "portRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<String>>,
    #[serde(rename = "resourceConfigurationDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_definition: Option<ResourceConfigurationDefinition>,
    #[serde(rename = "resourceConfigurationIdentifier")]
    #[serde(default)]
    pub resource_configuration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceConfigurationResponse {
    #[serde(rename = "allowAssociationToShareableServiceNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_association_to_shareable_service_network: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "portRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "resourceConfigurationDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_definition: Option<ResourceConfigurationDefinition>,
    #[serde(rename = "resourceConfigurationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_group_id: Option<String>,
    #[serde(rename = "resourceGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_gateway_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceGatewayRequest {
    #[serde(rename = "resourceGatewayIdentifier")]
    #[serde(default)]
    pub resource_gateway_identifier: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceGatewayResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(rename = "listenerIdentifier")]
    #[serde(default)]
    pub listener_identifier: String,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ruleIdentifier")]
    #[serde(default)]
    pub rule_identifier: String,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "match")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<RuleMatch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceNetworkRequest {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "serviceNetworkIdentifier")]
    #[serde(default)]
    pub service_network_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceNetworkVpcAssociationRequest {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "serviceNetworkVpcAssociationIdentifier")]
    #[serde(default)]
    pub service_network_vpc_association_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceNetworkVpcAssociationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceRequest {
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "serviceIdentifier")]
    #[serde(default)]
    pub service_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "customDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTargetGroupRequest {
    #[serde(rename = "healthCheck")]
    #[serde(default)]
    pub health_check: HealthCheckConfig,
    #[serde(rename = "targetGroupIdentifier")]
    #[serde(default)]
    pub target_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTargetGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TargetGroupConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
