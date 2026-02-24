use std::collections::HashMap;

/// A Classic ELB listener.
#[derive(Debug, Clone)]
pub struct ElbListener {
    pub load_balancer_port: i32,
    pub instance_port: i32,
    pub protocol: String,
    pub instance_protocol: String,
    pub ssl_certificate_id: Option<String>,
    /// Policy names applied to this listener.
    pub policy_names: Vec<String>,
}

/// Health-check configuration for a Classic ELB.
#[derive(Debug, Clone)]
pub struct ElbHealthCheck {
    pub target: String,
    pub interval: i32,
    pub timeout: i32,
    pub unhealthy_threshold: i32,
    pub healthy_threshold: i32,
}

impl Default for ElbHealthCheck {
    fn default() -> Self {
        Self {
            target: "TCP:80".to_string(),
            interval: 30,
            timeout: 5,
            unhealthy_threshold: 2,
            healthy_threshold: 10,
        }
    }
}

/// A policy attached to a Classic ELB.
#[derive(Debug, Clone)]
pub struct ElbPolicy {
    pub policy_name: String,
    pub policy_type_name: String,
    pub attributes: Vec<(String, String)>,
}

/// Attributes for a Classic ELB.
#[derive(Debug, Clone)]
pub struct ElbAttributes {
    pub cross_zone_load_balancing_enabled: bool,
    pub access_log_enabled: bool,
    pub access_log_emit_interval: Option<i32>,
    pub access_log_s3_bucket_name: Option<String>,
    pub access_log_s3_bucket_prefix: Option<String>,
    pub connection_draining_enabled: bool,
    pub connection_draining_timeout: Option<i32>,
    pub connection_idle_timeout: i32,
}

impl Default for ElbAttributes {
    fn default() -> Self {
        Self {
            cross_zone_load_balancing_enabled: false,
            access_log_enabled: false,
            access_log_emit_interval: None,
            access_log_s3_bucket_name: None,
            access_log_s3_bucket_prefix: None,
            connection_draining_enabled: false,
            connection_draining_timeout: None,
            connection_idle_timeout: 60,
        }
    }
}

/// A Classic ELB load balancer.
#[derive(Debug, Clone)]
pub struct ElbLoadBalancer {
    pub name: String,
    pub dns_name: String,
    pub scheme: String,
    pub availability_zones: Vec<String>,
    pub subnets: Vec<String>,
    pub security_groups: Vec<String>,
    pub instances: Vec<String>,
    pub listeners: Vec<ElbListener>,
    pub health_check: ElbHealthCheck,
    pub tags: HashMap<String, String>,
    pub policies: Vec<ElbPolicy>,
    pub attributes: ElbAttributes,
    pub vpc_id: Option<String>,
}
