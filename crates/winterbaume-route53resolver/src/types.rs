use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ResolverEndpoint {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub security_group_ids: Vec<String>,
    pub direction: String,
    pub ip_address_count: i32,
    pub host_vpc_id: String,
    pub status: String,
    pub status_message: String,
    pub creation_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
    pub creator_request_id: String,
    pub protocols: Vec<String>,
    pub resolver_endpoint_type: String,
    pub ip_addresses: Vec<IpAddressEntry>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct IpAddressEntry {
    pub ip_id: String,
    pub subnet_id: String,
    pub ip: Option<String>,
    pub status: String,
    pub status_message: String,
    pub creation_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ResolverRule {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub domain_name: String,
    pub rule_type: String,
    pub resolver_endpoint_id: Option<String>,
    pub target_ips: Vec<TargetAddress>,
    pub status: String,
    pub status_message: String,
    pub owner_id: String,
    pub share_status: String,
    pub creator_request_id: String,
    pub creation_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TargetAddress {
    pub ip: Option<String>,
    pub ipv6: Option<String>,
    pub port: Option<i32>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolverRuleAssociation {
    pub id: String,
    pub resolver_rule_id: String,
    pub name: String,
    pub vpc_id: String,
    pub status: String,
    pub status_message: String,
}

#[derive(Debug, Clone)]
pub struct ResolverQueryLogConfig {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub destination_arn: String,
    pub owner_id: String,
    pub status: String,
    pub share_status: String,
    pub association_count: i32,
    pub creator_request_id: String,
    pub creation_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ResolverQueryLogConfigAssociation {
    pub id: String,
    pub resolver_query_log_config_id: String,
    pub resource_id: String,
    pub status: String,
    pub error: String,
    pub error_message: String,
    pub creation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ResolverDnssecConfig {
    pub id: String,
    pub owner_id: String,
    pub resource_id: String,
    pub validation_status: String,
}
