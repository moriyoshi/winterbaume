use serde::{Deserialize, Serialize};

/// Represents an MSK cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub state: ClusterState,
    pub cluster_type: ClusterType,
    pub creation_time: String,
    /// Provisioned cluster info, present when cluster_type is PROVISIONED.
    pub provisioned: Option<ProvisionedClusterInfo>,
    /// Serverless cluster info, present when cluster_type is SERVERLESS.
    pub serverless: Option<ServerlessClusterInfo>,
    pub tags: std::collections::HashMap<String, String>,
    /// Opaque storage for nested Terraform blocks (client_authentication, configuration_info, etc.)
    #[serde(default)]
    pub extra_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClusterState {
    Active,
    Creating,
    Deleting,
    Failed,
    Maintenance,
    RebootingBroker,
    Updating,
}

impl ClusterState {
    pub fn as_str(&self) -> &str {
        match self {
            ClusterState::Active => "ACTIVE",
            ClusterState::Creating => "CREATING",
            ClusterState::Deleting => "DELETING",
            ClusterState::Failed => "FAILED",
            ClusterState::Maintenance => "MAINTENANCE",
            ClusterState::RebootingBroker => "REBOOTING_BROKER",
            ClusterState::Updating => "UPDATING",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClusterType {
    Provisioned,
    Serverless,
}

impl ClusterType {
    pub fn as_str(&self) -> &str {
        match self {
            ClusterType::Provisioned => "PROVISIONED",
            ClusterType::Serverless => "SERVERLESS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedClusterInfo {
    pub kafka_version: String,
    pub number_of_broker_nodes: i32,
    pub instance_type: String,
    pub client_subnets: Vec<String>,
    pub security_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerlessClusterInfo {
    pub vpc_configs: Vec<VpcConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfig {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}
