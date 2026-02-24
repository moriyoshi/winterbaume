use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An Elastic Load Balancer (v2).
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    pub arn: String,
    pub dns_name: String,
    pub name: String,
    pub scheme: String,
    pub state: String,
    pub lb_type: String,
    pub vpc_id: String,
    pub availability_zones: Vec<AvailabilityZone>,
    pub created_time: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
    pub ip_address_type: String,
    pub security_groups: Vec<String>,
    pub subnets: Vec<String>,
}

/// An availability zone for a load balancer.
#[derive(Debug, Clone)]
pub struct AvailabilityZone {
    pub zone_name: String,
    pub subnet_id: String,
}

/// A target group.
#[derive(Debug, Clone)]
pub struct TargetGroup {
    pub arn: String,
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub vpc_id: String,
    pub health_check_path: String,
    pub target_type: String,
    pub attributes: HashMap<String, String>,
    pub targets: Vec<TargetDescription>,
}

/// A listener.
#[derive(Debug, Clone)]
pub struct Listener {
    pub arn: String,
    pub load_balancer_arn: String,
    pub port: i32,
    pub protocol: String,
    pub default_actions: Vec<ListenerAction>,
    pub certificates: Vec<Certificate>,
    pub attributes: HashMap<String, String>,
}

/// A listener default action.
#[derive(Debug, Clone)]
pub struct ListenerAction {
    pub action_type: String,
    pub target_group_arn: String,
}

/// A rule for a listener.
#[derive(Debug, Clone)]
pub struct Rule {
    pub rule_arn: String,
    pub priority: String,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<RuleAction>,
    pub is_default: bool,
    pub listener_arn: String,
}

/// A condition for a listener rule.
#[derive(Debug, Clone)]
pub struct RuleCondition {
    pub field: String,
    pub values: Vec<String>,
}

/// An action for a listener rule.
#[derive(Debug, Clone)]
pub struct RuleAction {
    pub action_type: String,
    pub target_group_arn: String,
}

/// A target registered with a target group.
#[derive(Debug, Clone)]
pub struct TargetDescription {
    pub id: String,
    pub port: Option<i32>,
    pub availability_zone: Option<String>,
}

/// A certificate for a listener.
#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_arn: String,
    pub is_default: Option<bool>,
}

/// An ELBv2 Trust Store.
#[derive(Debug, Clone)]
pub struct TrustStore {
    pub arn: String,
    pub name: String,
    pub status: String,
    pub number_of_ca_certificates: i32,
    pub total_revoked_entries: i64,
    /// Revocation entries keyed by revocation_id.
    pub revocations: HashMap<i64, TrustStoreRevocationEntry>,
    /// Next revocation ID counter.
    pub next_revocation_id: i64,
}

/// A revocation entry stored inside a TrustStore.
#[derive(Debug, Clone)]
pub struct TrustStoreRevocationEntry {
    pub revocation_id: i64,
    pub revocation_type: String,
    pub number_of_revoked_entries: i64,
}

/// A trust store association (a listener that references a trust store).
#[derive(Debug, Clone)]
pub struct TrustStoreAssociation {
    pub resource_arn: String,
}

/// Capacity reservation state for a load balancer.
#[derive(Debug, Clone)]
pub struct CapacityReservation {
    pub load_balancer_arn: String,
    pub minimum_capacity_units: i32,
    pub availability_zone_states: Vec<ZonalCapacityState>,
    pub decrease_requests_remaining: i32,
}

/// Per-AZ capacity state.
#[derive(Debug, Clone)]
pub struct ZonalCapacityState {
    pub availability_zone: String,
    pub effective_capacity_units: f64,
    pub status_code: String,
}

/// A resource-level policy for ELBv2 resources.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub resource_arn: String,
    pub policy: String,
}

/// An IPAM pool assignment for a load balancer.
#[derive(Debug, Clone)]
pub struct IpamPool {
    pub ipv4_ipam_pool_id: Option<String>,
    pub ipv6_ipam_pool_id: Option<String>,
}
