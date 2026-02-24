use std::collections::HashMap;

/// A Route 53 hosted zone.
#[derive(Debug, Clone)]
pub struct HostedZone {
    pub id: String,
    pub name: String,
    pub caller_reference: String,
    pub resource_record_count: u64,
    pub records: Vec<ResourceRecordSet>,
    /// Associated VPCs for private hosted zones.
    pub vpcs: Vec<Vpc>,
    pub comment: Option<String>,
    pub delegation_set: Route53DelegationSet,
    pub tags: HashMap<String, String>,
}

/// A VPC associated with a hosted zone.
#[derive(Debug, Clone)]
pub struct Vpc {
    pub vpc_id: String,
    pub vpc_region: String,
}

/// A DNS resource record set.
#[derive(Debug, Clone)]
pub struct ResourceRecordSet {
    pub name: String,
    pub record_type: String,
    pub ttl: Option<u64>,
    pub resource_records: Vec<String>,
}

/// A reusable delegation set or hosted-zone-owned delegation set.
#[derive(Debug, Clone)]
pub struct Route53DelegationSet {
    pub id: String,
    pub caller_reference: Option<String>,
    pub name_servers: Vec<String>,
}

/// Stored Route 53 health check.
#[derive(Debug, Clone)]
pub struct Route53HealthCheck {
    pub id: String,
    pub caller_reference: String,
    pub config: Route53HealthCheckConfig,
    pub version: i64,
    pub tags: HashMap<String, String>,
}

/// Simplified health check configuration used by moto-covered operations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Route53HealthCheckConfig {
    pub type_: String,
    pub ip_address: Option<String>,
    pub port: Option<i32>,
    pub resource_path: Option<String>,
    pub fully_qualified_domain_name: Option<String>,
    pub failure_threshold: Option<i32>,
    pub request_interval: Option<i32>,
    pub inverted: Option<bool>,
    pub disabled: Option<bool>,
    pub measure_latency: Option<bool>,
    pub search_string: Option<String>,
    pub enable_sni: Option<bool>,
    pub health_threshold: Option<i32>,
    pub insufficient_data_health_status: Option<String>,
    pub regions: Vec<String>,
    pub child_health_checks: Vec<String>,
}

/// Stored Route 53 query logging configuration.
#[derive(Debug, Clone)]
pub struct Route53QueryLoggingConfig {
    pub id: String,
    pub hosted_zone_id: String,
    pub cloud_watch_logs_log_group_arn: String,
}

/// Tag listing entry for ListTagsForResources.
#[derive(Debug, Clone)]
pub struct TagResource {
    pub resource_id: String,
    pub resource_type: String,
    pub tags: HashMap<String, String>,
}

/// A DNSSEC key signing key.
#[derive(Debug, Clone)]
pub struct KeySigningKeyEntry {
    pub name: String,
    pub hosted_zone_id: String,
    pub kms_arn: String,
    pub status: String,
    pub created_date: String,
    pub last_modified_date: String,
}

/// A CIDR collection.
#[derive(Debug, Clone)]
pub struct CidrCollectionEntry {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub version: i64,
    /// CIDR locations keyed by location name, each containing a list of CIDR blocks.
    pub locations: HashMap<String, Vec<String>>,
}

/// A traffic policy.
#[derive(Debug, Clone)]
pub struct TrafficPolicyEntry {
    pub id: String,
    pub name: String,
    pub version: i32,
    pub document: String,
    pub comment: Option<String>,
    pub type_: String,
}

/// A traffic policy instance.
#[derive(Debug, Clone)]
pub struct TrafficPolicyInstanceEntry {
    pub id: String,
    pub hosted_zone_id: String,
    pub name: String,
    pub ttl: i64,
    pub state: String,
    pub message: String,
    pub traffic_policy_id: String,
    pub traffic_policy_version: i32,
    pub traffic_policy_type: String,
}

/// A VPC association authorization.
#[derive(Debug, Clone)]
pub struct VpcAssociationAuthorization {
    pub hosted_zone_id: String,
    pub vpc_id: String,
    pub vpc_region: String,
}
