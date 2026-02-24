use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct Ec2InstanceConnectState {
    pub endpoints: HashMap<String, Ec2InstanceConnectEndpoint>,
}

/// An EC2 Instance Connect Endpoint.
#[derive(Debug, Clone)]
pub struct Ec2InstanceConnectEndpoint {
    pub instance_connect_endpoint_id: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub security_group_ids: Vec<String>,
    pub state: String,
    pub dns_name: Option<String>,
    pub fips_dns_name: Option<String>,
    pub network_interface_ids: Vec<String>,
    pub owner_id: String,
    pub availability_zone: String,
    pub created_at: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum Ec2InstanceConnectError {
    #[error("Missing X-Amz-Target header")]
    MissingAction,

    #[error("Invalid JSON body")]
    SerializationException,

    #[error("Could not find operation {action} for EC2 Instance Connect")]
    InvalidAction { action: String },

    #[error("InstanceId is required")]
    InstanceIdRequired,

    #[error("InstanceOSUser is required")]
    InstanceOSUserRequired,

    #[error("SSHPublicKey is required")]
    SSHPublicKeyRequired,
}
