//! Serde-compatible view types for DAX state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DaxService;
use crate::state::DaxState;
use crate::types::{DaxCluster, DaxParameterGroup, DaxSubnetGroup, DaxTag};

/// Serializable view of the entire DAX state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DaxStateView {
    /// Clusters keyed by cluster name.
    #[serde(default)]
    pub clusters: HashMap<String, DaxClusterView>,
    /// Parameter groups keyed by parameter group name.
    #[serde(default)]
    pub parameter_groups: HashMap<String, DaxParameterGroupView>,
    /// Subnet groups keyed by subnet group name.
    #[serde(default)]
    pub subnet_groups: HashMap<String, DaxSubnetGroupView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaxClusterView {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub node_type: String,
    pub status: String,
    pub description: String,
    pub iam_role_arn: String,
    pub replication_factor: i32,
    pub sse_enabled: bool,
    pub cluster_endpoint_encryption_type: String,
    #[serde(default)]
    pub tags: Vec<DaxTagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaxTagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaxParameterGroupView {
    pub parameter_group_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaxSubnetGroupView {
    pub subnet_group_name: String,
    pub description: String,
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub vpc_id: Option<String>,
}

// --- From internal types to view types ---

impl From<&DaxState> for DaxStateView {
    fn from(state: &DaxState) -> Self {
        DaxStateView {
            clusters: state
                .clusters
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DaxClusterView {
                            cluster_name: v.cluster_name.clone(),
                            cluster_arn: v.cluster_arn.clone(),
                            node_type: v.node_type.clone(),
                            status: v.status.clone(),
                            description: v.description.clone(),
                            iam_role_arn: v.iam_role_arn.clone(),
                            replication_factor: v.replication_factor,
                            sse_enabled: v.sse_enabled,
                            cluster_endpoint_encryption_type: v
                                .cluster_endpoint_encryption_type
                                .clone(),
                            tags: v
                                .tags
                                .iter()
                                .map(|t| DaxTagView {
                                    key: t.key.clone(),
                                    value: t.value.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            parameter_groups: state
                .parameter_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DaxParameterGroupView {
                            parameter_group_name: v.parameter_group_name.clone(),
                            description: v.description.clone(),
                        },
                    )
                })
                .collect(),
            subnet_groups: state
                .subnet_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DaxSubnetGroupView {
                            subnet_group_name: v.subnet_group_name.clone(),
                            description: v.description.clone(),
                            subnet_ids: v.subnet_ids.clone(),
                            vpc_id: v.vpc_id.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<DaxStateView> for DaxState {
    fn from(view: DaxStateView) -> Self {
        DaxState {
            clusters: view
                .clusters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DaxCluster {
                            cluster_name: v.cluster_name,
                            cluster_arn: v.cluster_arn,
                            node_type: v.node_type,
                            status: v.status,
                            description: v.description,
                            iam_role_arn: v.iam_role_arn,
                            replication_factor: v.replication_factor,
                            sse_enabled: v.sse_enabled,
                            cluster_endpoint_encryption_type: v.cluster_endpoint_encryption_type,
                            tags: v
                                .tags
                                .into_iter()
                                .map(|t| DaxTag {
                                    key: t.key,
                                    value: t.value,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            parameter_groups: view
                .parameter_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DaxParameterGroup {
                            parameter_group_name: v.parameter_group_name,
                            description: v.description,
                        },
                    )
                })
                .collect(),
            subnet_groups: view
                .subnet_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DaxSubnetGroup {
                            subnet_group_name: v.subnet_group_name,
                            description: v.description,
                            subnet_ids: v.subnet_ids,
                            vpc_id: v.vpc_id,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DaxService {
    type StateView = DaxStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DaxStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = DaxState::from(view);
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
            let incoming = DaxState::from(view);
            for (k, v) in incoming.clusters {
                guard.clusters.insert(k, v);
            }
            for (k, v) in incoming.parameter_groups {
                guard.parameter_groups.insert(k, v);
            }
            for (k, v) in incoming.subnet_groups {
                guard.subnet_groups.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
