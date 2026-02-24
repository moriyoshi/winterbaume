use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace_type: String,
    pub description: Option<String>,
    pub creator_request_id: Option<String>,
    pub vpc: Option<String>,
    pub hosted_zone_id: Option<String>,
    pub soa_ttl: Option<i64>,
    pub service_count: i32,
    pub create_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub operation_type: String,
    pub status: String,
    pub targets: HashMap<String, String>,
    pub create_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ServiceEntry {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace_id: String,
    pub description: Option<String>,
    pub creator_request_id: Option<String>,
    pub dns_config: Option<DnsConfigEntry>,
    pub health_check_config: Option<HealthCheckConfigEntry>,
    pub health_check_custom_config: Option<HealthCheckCustomConfigEntry>,
    pub instance_count: i32,
    pub create_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub service_type: Option<String>,
    pub include_namespace_id_in_response: bool,
}

#[derive(Debug, Clone)]
pub struct DnsConfigEntry {
    pub namespace_id: Option<String>,
    pub routing_policy: Option<String>,
    pub dns_records: Vec<DnsRecordEntry>,
}

#[derive(Debug, Clone)]
pub struct DnsRecordEntry {
    pub record_type: String,
    pub ttl: i64,
}

#[derive(Debug, Clone)]
pub struct HealthCheckConfigEntry {
    pub check_type: String,
    pub resource_path: Option<String>,
    pub failure_threshold: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct HealthCheckCustomConfigEntry {
    pub failure_threshold: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct InstanceEntry {
    pub id: String,
    pub service_id: String,
    pub creator_request_id: Option<String>,
    pub attributes: HashMap<String, String>,
    pub health_status: String,
}
