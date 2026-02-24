//! Serde-compatible view types for CloudHSM V2 state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudHsmV2Service;
use crate::state::CloudHsmV2State;
use crate::types::ClusterState;

/// Serializable view of the entire CloudHSM V2 state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudHsmV2StateView {
    /// Clusters keyed by cluster ID.
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// Backups keyed by backup ID.
    #[serde(default)]
    pub backups: HashMap<String, BackupView>,
    /// Resource policies keyed by resource ARN.
    #[serde(default)]
    pub resource_policies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub cluster_id: String,
    pub hsm_type: String,
    #[serde(default)]
    pub subnet_mapping: HashMap<String, String>,
    pub vpc_id: String,
    /// Cluster state as a string (e.g. "ACTIVE", "UNINITIALIZED").
    pub state: String,
    pub security_group: String,
    pub source_backup_id: Option<String>,
    pub backup_policy: String,
    pub backup_retention_policy: Option<BackupRetentionPolicyView>,
    pub create_timestamp: f64,
    #[serde(default)]
    pub tag_list: Vec<TagView>,
    pub region: String,
    pub account_id: String,
    #[serde(default)]
    pub hsms: Vec<HsmView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmView {
    pub hsm_id: String,
    pub cluster_id: String,
    pub availability_zone: String,
    pub subnet_id: Option<String>,
    pub eni_id: Option<String>,
    pub eni_ip: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupView {
    pub backup_id: String,
    pub backup_arn: String,
    pub backup_state: String,
    pub cluster_id: Option<String>,
    pub create_timestamp: f64,
    pub copy_timestamp: Option<f64>,
    pub delete_timestamp: Option<f64>,
    pub hsm_type: Option<String>,
    pub never_expires: bool,
    pub source_backup: Option<String>,
    pub source_cluster: Option<String>,
    pub source_region: Option<String>,
    #[serde(default)]
    pub tag_list: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRetentionPolicyView {
    pub r#type: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

fn cluster_state_from_str(s: &str) -> ClusterState {
    match s {
        "CREATE_IN_PROGRESS" => ClusterState::CreateInProgress,
        "UNINITIALIZED" => ClusterState::Uninitialized,
        "INITIALIZE_IN_PROGRESS" => ClusterState::InitializeInProgress,
        "INITIALIZED" => ClusterState::Initialized,
        "ACTIVE" => ClusterState::Active,
        "DELETE_IN_PROGRESS" => ClusterState::DeleteInProgress,
        "DELETED" => ClusterState::Deleted,
        _ => ClusterState::Uninitialized,
    }
}

// --- From internal types to view types ---

impl From<&CloudHsmV2State> for CloudHsmV2StateView {
    fn from(state: &CloudHsmV2State) -> Self {
        CloudHsmV2StateView {
            clusters: state
                .clusters
                .iter()
                .map(|(k, c)| {
                    (
                        k.clone(),
                        ClusterView {
                            cluster_id: c.cluster_id.clone(),
                            hsm_type: c.hsm_type.clone(),
                            subnet_mapping: c.subnet_mapping.clone(),
                            vpc_id: c.vpc_id.clone(),
                            state: c.state.as_str().to_string(),
                            security_group: c.security_group.clone(),
                            source_backup_id: c.source_backup_id.clone(),
                            backup_policy: c.backup_policy.clone(),
                            backup_retention_policy: c.backup_retention_policy.as_ref().map(
                                |brp| BackupRetentionPolicyView {
                                    r#type: brp.r#type.clone(),
                                    value: brp.value.clone(),
                                },
                            ),
                            create_timestamp: c.create_timestamp,
                            tag_list: c
                                .tag_list
                                .iter()
                                .map(|t| TagView {
                                    key: t.key.clone(),
                                    value: t.value.clone(),
                                })
                                .collect(),
                            region: c.region.clone(),
                            account_id: c.account_id.clone(),
                            hsms: c
                                .hsms
                                .iter()
                                .map(|h| HsmView {
                                    hsm_id: h.hsm_id.clone(),
                                    cluster_id: h.cluster_id.clone(),
                                    availability_zone: h.availability_zone.clone(),
                                    subnet_id: h.subnet_id.clone(),
                                    eni_id: h.eni_id.clone(),
                                    eni_ip: h.eni_ip.clone(),
                                    state: h.state.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            backups: state
                .backups
                .iter()
                .map(|(k, b)| {
                    (
                        k.clone(),
                        BackupView {
                            backup_id: b.backup_id.clone(),
                            backup_arn: b.backup_arn.clone(),
                            backup_state: b.backup_state.clone(),
                            cluster_id: b.cluster_id.clone(),
                            create_timestamp: b.create_timestamp,
                            copy_timestamp: b.copy_timestamp,
                            delete_timestamp: b.delete_timestamp,
                            hsm_type: b.hsm_type.clone(),
                            never_expires: b.never_expires,
                            source_backup: b.source_backup.clone(),
                            source_cluster: b.source_cluster.clone(),
                            source_region: b.source_region.clone(),
                            tag_list: b
                                .tag_list
                                .iter()
                                .map(|t| TagView {
                                    key: t.key.clone(),
                                    value: t.value.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            resource_policies: state.resource_policies.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<CloudHsmV2StateView> for CloudHsmV2State {
    fn from(view: CloudHsmV2StateView) -> Self {
        CloudHsmV2State {
            clusters: view
                .clusters
                .into_iter()
                .map(|(k, c)| {
                    (
                        k,
                        crate::types::Cluster {
                            cluster_id: c.cluster_id,
                            hsm_type: c.hsm_type,
                            subnet_mapping: c.subnet_mapping,
                            vpc_id: c.vpc_id,
                            state: cluster_state_from_str(&c.state),
                            security_group: c.security_group,
                            source_backup_id: c.source_backup_id,
                            backup_policy: c.backup_policy,
                            backup_retention_policy: c.backup_retention_policy.map(|brp| {
                                crate::types::BackupRetentionPolicy {
                                    r#type: brp.r#type,
                                    value: brp.value,
                                }
                            }),
                            create_timestamp: c.create_timestamp,
                            tag_list: c
                                .tag_list
                                .into_iter()
                                .map(|t| crate::types::Tag {
                                    key: t.key,
                                    value: t.value,
                                })
                                .collect(),
                            region: c.region,
                            account_id: c.account_id,
                            hsms: c
                                .hsms
                                .into_iter()
                                .map(|h| crate::types::Hsm {
                                    hsm_id: h.hsm_id,
                                    cluster_id: h.cluster_id,
                                    availability_zone: h.availability_zone,
                                    subnet_id: h.subnet_id,
                                    eni_id: h.eni_id,
                                    eni_ip: h.eni_ip,
                                    state: h.state,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            backups: view
                .backups
                .into_iter()
                .map(|(k, b)| {
                    (
                        k,
                        crate::types::Backup {
                            backup_id: b.backup_id,
                            backup_arn: b.backup_arn,
                            backup_state: b.backup_state,
                            cluster_id: b.cluster_id,
                            create_timestamp: b.create_timestamp,
                            copy_timestamp: b.copy_timestamp,
                            delete_timestamp: b.delete_timestamp,
                            hsm_type: b.hsm_type,
                            never_expires: b.never_expires,
                            source_backup: b.source_backup,
                            source_cluster: b.source_cluster,
                            source_region: b.source_region,
                            tag_list: b
                                .tag_list
                                .into_iter()
                                .map(|t| crate::types::Tag {
                                    key: t.key,
                                    value: t.value,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            resource_policies: view.resource_policies,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CloudHsmV2Service {
    type StateView = CloudHsmV2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudHsmV2StateView::from(&*guard)
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
            *guard = CloudHsmV2State::from(view);
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
            let merged = CloudHsmV2State::from(view);
            for (k, v) in merged.clusters {
                guard.clusters.insert(k, v);
            }
            for (k, v) in merged.backups {
                guard.backups.insert(k, v);
            }
            for (k, v) in merged.resource_policies {
                guard.resource_policies.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
