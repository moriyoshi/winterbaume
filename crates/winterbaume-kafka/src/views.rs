//! Serde-compatible view types for MSK (Kafka) state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KafkaService;
use crate::state::KafkaState;
use crate::types::{
    Cluster, ClusterState, ClusterType, ProvisionedClusterInfo, ServerlessClusterInfo, VpcConfig,
};

/// Serializable view of the entire MSK state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KafkaStateView {
    /// Clusters keyed by cluster ARN.
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// Tags keyed by resource ARN -> (key -> value).
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub state: String,
    pub cluster_type: String,
    pub creation_time: String,
    pub provisioned: Option<ProvisionedClusterInfoView>,
    pub serverless: Option<ServerlessClusterInfoView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub client_authentication: Option<serde_json::Value>,
    #[serde(default)]
    pub configuration_info: Option<serde_json::Value>,
    #[serde(default)]
    pub encryption_info: Option<serde_json::Value>,
    #[serde(default)]
    pub logging_info: Option<serde_json::Value>,
    #[serde(default)]
    pub open_monitoring: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedClusterInfoView {
    pub kafka_version: String,
    pub number_of_broker_nodes: i32,
    pub instance_type: String,
    pub client_subnets: Vec<String>,
    pub security_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerlessClusterInfoView {
    pub vpc_configs: Vec<VpcConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfigView {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&ProvisionedClusterInfo> for ProvisionedClusterInfoView {
    fn from(p: &ProvisionedClusterInfo) -> Self {
        ProvisionedClusterInfoView {
            kafka_version: p.kafka_version.clone(),
            number_of_broker_nodes: p.number_of_broker_nodes,
            instance_type: p.instance_type.clone(),
            client_subnets: p.client_subnets.clone(),
            security_groups: p.security_groups.clone(),
        }
    }
}

impl From<ProvisionedClusterInfoView> for ProvisionedClusterInfo {
    fn from(v: ProvisionedClusterInfoView) -> Self {
        ProvisionedClusterInfo {
            kafka_version: v.kafka_version,
            number_of_broker_nodes: v.number_of_broker_nodes,
            instance_type: v.instance_type,
            client_subnets: v.client_subnets,
            security_groups: v.security_groups,
        }
    }
}

impl From<&VpcConfig> for VpcConfigView {
    fn from(v: &VpcConfig) -> Self {
        VpcConfigView {
            subnet_ids: v.subnet_ids.clone(),
            security_group_ids: v.security_group_ids.clone(),
        }
    }
}

impl From<VpcConfigView> for VpcConfig {
    fn from(v: VpcConfigView) -> Self {
        VpcConfig {
            subnet_ids: v.subnet_ids,
            security_group_ids: v.security_group_ids,
        }
    }
}

impl From<&ServerlessClusterInfo> for ServerlessClusterInfoView {
    fn from(s: &ServerlessClusterInfo) -> Self {
        ServerlessClusterInfoView {
            vpc_configs: s.vpc_configs.iter().map(VpcConfigView::from).collect(),
        }
    }
}

impl From<ServerlessClusterInfoView> for ServerlessClusterInfo {
    fn from(v: ServerlessClusterInfoView) -> Self {
        ServerlessClusterInfo {
            vpc_configs: v.vpc_configs.into_iter().map(VpcConfig::from).collect(),
        }
    }
}

impl From<&Cluster> for ClusterView {
    fn from(c: &Cluster) -> Self {
        let extra = c.extra_config.as_ref();
        ClusterView {
            cluster_name: c.cluster_name.clone(),
            cluster_arn: c.cluster_arn.clone(),
            state: c.state.as_str().to_string(),
            cluster_type: c.cluster_type.as_str().to_string(),
            creation_time: c.creation_time.clone(),
            provisioned: c.provisioned.as_ref().map(ProvisionedClusterInfoView::from),
            serverless: c.serverless.as_ref().map(ServerlessClusterInfoView::from),
            tags: c.tags.clone(),
            client_authentication: extra.and_then(|e| e.get("client_authentication")).cloned(),
            configuration_info: extra.and_then(|e| e.get("configuration_info")).cloned(),
            encryption_info: extra.and_then(|e| e.get("encryption_info")).cloned(),
            logging_info: extra.and_then(|e| e.get("logging_info")).cloned(),
            open_monitoring: extra.and_then(|e| e.get("open_monitoring")).cloned(),
        }
    }
}

impl From<ClusterView> for Cluster {
    fn from(v: ClusterView) -> Self {
        let state = match v.state.as_str() {
            "ACTIVE" => ClusterState::Active,
            "CREATING" => ClusterState::Creating,
            "DELETING" => ClusterState::Deleting,
            "FAILED" => ClusterState::Failed,
            "MAINTENANCE" => ClusterState::Maintenance,
            "REBOOTING_BROKER" => ClusterState::RebootingBroker,
            _ => ClusterState::Updating,
        };
        let cluster_type = match v.cluster_type.as_str() {
            "SERVERLESS" => ClusterType::Serverless,
            _ => ClusterType::Provisioned,
        };
        let mut extra = serde_json::Map::new();
        if let Some(val) = v.client_authentication {
            extra.insert("client_authentication".to_string(), val);
        }
        if let Some(val) = v.configuration_info {
            extra.insert("configuration_info".to_string(), val);
        }
        if let Some(val) = v.encryption_info {
            extra.insert("encryption_info".to_string(), val);
        }
        if let Some(val) = v.logging_info {
            extra.insert("logging_info".to_string(), val);
        }
        if let Some(val) = v.open_monitoring {
            extra.insert("open_monitoring".to_string(), val);
        }
        let extra_config = if extra.is_empty() {
            None
        } else {
            Some(serde_json::Value::Object(extra))
        };
        Cluster {
            cluster_name: v.cluster_name,
            cluster_arn: v.cluster_arn,
            state,
            cluster_type,
            creation_time: v.creation_time,
            provisioned: v.provisioned.map(ProvisionedClusterInfo::from),
            serverless: v.serverless.map(ServerlessClusterInfo::from),
            tags: v.tags,
            extra_config,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for KafkaService {
    type StateView = KafkaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let clusters = guard
            .clusters
            .iter()
            .map(|(k, v)| (k.clone(), ClusterView::from(v)))
            .collect();

        let tags = guard.tags.clone();

        KafkaStateView { clusters, tags }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = KafkaState::default();
        for (arn, cv) in view.clusters {
            new_state.clusters.insert(arn, Cluster::from(cv));
        }
        new_state.tags = view.tags;
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (arn, cv) in view.clusters {
                guard.clusters.insert(arn, Cluster::from(cv));
            }
            for (arn, tag_map) in view.tags {
                guard.tags.insert(arn, tag_map);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
