use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub auth_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Listener {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub service_id: String,
    pub service_arn: String,
    pub port: Option<i32>,
    pub protocol: String,
    pub default_action: ListenerDefaultAction,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ListenerDefaultAction {
    pub fixed_response_status_code: Option<i32>,
    pub forward_target_groups: Vec<(String, Option<i32>)>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub listener_id: String,
    pub service_id: String,
    pub priority: i32,
    pub is_default: bool,
    pub action: RuleAction,
    pub rule_match: Option<RuleMatchData>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub fixed_response_status_code: Option<i32>,
    pub forward_target_groups: Vec<(String, Option<i32>)>,
}

#[derive(Debug, Clone)]
pub struct RuleMatchData {
    pub method: Option<String>,
    pub path_exact: Option<String>,
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthPolicy {
    pub policy: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy: String,
}

#[derive(Debug, Clone)]
pub struct ServiceNetwork {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub auth_type: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub number_of_associated_services: i64,
    pub number_of_associated_v_p_cs: i64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AccessLogSubscription {
    pub id: String,
    pub arn: String,
    pub resource_arn: String,
    pub resource_id: String,
    pub destination_arn: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ServiceNetworkServiceAssociation {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub service_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub service_id: String,
    pub service_arn: String,
    pub service_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ServiceNetworkVpcAssociation {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub vpc_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub vpc_id: String,
    pub status: String,
    pub security_group_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TargetGroup {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub target_group_type: String,
    pub config: Option<TargetGroupConfig>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub targets: Vec<TargetEntry>,
}

#[derive(Debug, Clone)]
pub struct TargetGroupConfig {
    pub port: Option<i32>,
    pub protocol: Option<String>,
    pub vpc_identifier: Option<String>,
    pub ip_address_type: Option<String>,
    pub protocol_version: Option<String>,
    pub lambda_event_structure_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TargetEntry {
    pub id: String,
    pub port: Option<i32>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ResourceConfiguration {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub resource_configuration_type: String,
    pub status: String,
    pub resource_gateway_id: Option<String>,
    pub port_ranges: Vec<String>,
    pub protocol: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ResourceGateway {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub ip_address_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ServiceNetworkResourceAssociation {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub resource_configuration_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub resource_configuration_id: String,
    pub resource_configuration_arn: String,
    pub resource_configuration_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DomainVerification {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}
