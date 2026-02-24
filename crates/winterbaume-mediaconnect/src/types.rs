use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A MediaConnect flow.
#[derive(Debug, Clone)]
pub struct Flow {
    /// The ARN of the flow.
    pub flow_arn: String,
    /// The name of the flow.
    pub name: String,
    /// The description of the flow.
    pub description: String,
    /// The Availability Zone in which the flow was created.
    pub availability_zone: String,
    /// The current status of the flow.
    pub status: String,
    /// When the flow was created.
    pub created_at: DateTime<Utc>,
    /// The outputs of the flow.
    pub outputs: Vec<FlowOutput>,
    /// The sources of the flow.
    pub sources: Vec<FlowSource>,
    /// The VPC interfaces of the flow.
    pub vpc_interfaces: Vec<FlowVpcInterface>,
    /// Tags associated with this flow.
    pub tags: HashMap<String, String>,
    /// Entitlements granted for this flow.
    pub entitlements: Vec<FlowEntitlement>,
}

/// An entitlement on a flow.
#[derive(Debug, Clone)]
pub struct FlowEntitlement {
    pub entitlement_arn: String,
    pub name: String,
    pub description: String,
    pub subscribers: Vec<String>,
}

/// An output of a flow.
#[derive(Debug, Clone)]
pub struct FlowOutput {
    pub output_arn: String,
    pub name: String,
    pub description: String,
    pub port: i32,
    pub protocol: String,
    pub destination: String,
}

/// A source of a flow.
#[derive(Debug, Clone)]
pub struct FlowSource {
    pub source_arn: String,
    pub name: String,
    pub description: String,
    pub ingest_port: i32,
    pub protocol: String,
    pub whitelist_cidr: String,
}

/// A VPC interface of a flow.
#[derive(Debug, Clone)]
pub struct FlowVpcInterface {
    pub name: String,
    pub role_arn: String,
    pub security_group_ids: Vec<String>,
    pub subnet_id: String,
    pub network_interface_type: String,
    pub network_interface_ids: Vec<String>,
}
