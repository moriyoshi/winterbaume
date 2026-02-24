//! Serde-compatible view types for MemoryDB state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MemoryDbService;
use crate::state::MemoryDbState;
use crate::types::{AclData, Cluster, SnapshotData, SubnetGroupData, TagData, TagsMap};

/// Serializable view of the entire MemoryDB state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryDbStateView {
    /// Clusters keyed by cluster name.
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// Snapshots keyed by snapshot name.
    #[serde(default)]
    pub snapshots: HashMap<String, SnapshotView>,
    /// Subnet groups keyed by subnet group name.
    #[serde(default)]
    pub subnet_groups: HashMap<String, SubnetGroupView>,
    /// ACLs keyed by ACL name.
    #[serde(default)]
    pub acls: HashMap<String, AclView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
}

/// Serializable view of a MemoryDB cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub node_type: String,
    pub num_shards: i32,
    pub num_replicas_per_shard: i32,
    pub description: String,
    pub engine: String,
    pub engine_version: String,
    pub subnet_group_name: String,
    pub security_group_ids: Vec<String>,
    pub maintenance_window: String,
    pub snapshot_retention_limit: i32,
    pub snapshot_window: String,
    pub acl_name: String,
    pub parameter_group_name: String,
    pub tls_enabled: bool,
    pub auto_minor_version_upgrade: bool,
    pub creation_time: Option<String>,
}

/// Serializable view of a MemoryDB snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub source: String,
    pub cluster_name: String,
    pub cluster_description: String,
    pub cluster_engine: String,
    pub cluster_engine_version: String,
    pub cluster_node_type: String,
    pub cluster_num_shards: i32,
    pub cluster_subnet_group_name: String,
    pub cluster_snapshot_retention_limit: i32,
    pub cluster_snapshot_window: String,
    pub cluster_maintenance_window: String,
    pub cluster_parameter_group_name: String,
    pub kms_key_id: Option<String>,
}

/// Serializable view of a MemoryDB subnet group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetGroupView {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
}

/// Serializable view of a MemoryDB ACL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclView {
    pub acl_name: String,
    pub status: String,
    pub user_names: Vec<String>,
    pub minimum_engine_version: String,
    pub arn: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a tag key-value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Cluster> for ClusterView {
    fn from(c: &Cluster) -> Self {
        ClusterView {
            name: c.name.clone(),
            arn: c.arn.clone(),
            status: c.status.clone(),
            node_type: c.node_type.clone(),
            num_shards: c.num_shards,
            num_replicas_per_shard: c.num_replicas_per_shard,
            description: c.description.clone(),
            engine: c.engine.clone(),
            engine_version: c.engine_version.clone(),
            subnet_group_name: c.subnet_group_name.clone(),
            security_group_ids: c.security_group_ids.clone(),
            maintenance_window: c.maintenance_window.clone(),
            snapshot_retention_limit: c.snapshot_retention_limit,
            snapshot_window: c.snapshot_window.clone(),
            acl_name: c.acl_name.clone(),
            parameter_group_name: c.parameter_group_name.clone(),
            tls_enabled: c.tls_enabled,
            auto_minor_version_upgrade: c.auto_minor_version_upgrade,
            creation_time: Some(c.creation_time.to_rfc3339()),
        }
    }
}

impl From<ClusterView> for Cluster {
    fn from(v: ClusterView) -> Self {
        let creation_time = v
            .creation_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Cluster {
            name: v.name,
            arn: v.arn,
            status: v.status,
            node_type: v.node_type,
            num_shards: v.num_shards,
            num_replicas_per_shard: v.num_replicas_per_shard,
            description: v.description,
            engine: v.engine,
            engine_version: v.engine_version,
            subnet_group_name: v.subnet_group_name,
            security_group_ids: v.security_group_ids,
            maintenance_window: v.maintenance_window,
            snapshot_retention_limit: v.snapshot_retention_limit,
            snapshot_window: v.snapshot_window,
            acl_name: v.acl_name,
            parameter_group_name: v.parameter_group_name,
            tls_enabled: v.tls_enabled,
            auto_minor_version_upgrade: v.auto_minor_version_upgrade,
            creation_time,
        }
    }
}

impl From<&SnapshotData> for SnapshotView {
    fn from(s: &SnapshotData) -> Self {
        SnapshotView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: s.status.clone(),
            source: s.source.clone(),
            cluster_name: s.cluster_name.clone(),
            cluster_description: s.cluster_description.clone(),
            cluster_engine: s.cluster_engine.clone(),
            cluster_engine_version: s.cluster_engine_version.clone(),
            cluster_node_type: s.cluster_node_type.clone(),
            cluster_num_shards: s.cluster_num_shards,
            cluster_subnet_group_name: s.cluster_subnet_group_name.clone(),
            cluster_snapshot_retention_limit: s.cluster_snapshot_retention_limit,
            cluster_snapshot_window: s.cluster_snapshot_window.clone(),
            cluster_maintenance_window: s.cluster_maintenance_window.clone(),
            cluster_parameter_group_name: s.cluster_parameter_group_name.clone(),
            kms_key_id: s.kms_key_id.clone(),
        }
    }
}

impl From<SnapshotView> for SnapshotData {
    fn from(v: SnapshotView) -> Self {
        SnapshotData {
            name: v.name,
            arn: v.arn,
            status: v.status,
            source: v.source,
            cluster_name: v.cluster_name,
            cluster_description: v.cluster_description,
            cluster_engine: v.cluster_engine,
            cluster_engine_version: v.cluster_engine_version,
            cluster_node_type: v.cluster_node_type,
            cluster_num_shards: v.cluster_num_shards,
            cluster_subnet_group_name: v.cluster_subnet_group_name,
            cluster_snapshot_retention_limit: v.cluster_snapshot_retention_limit,
            cluster_snapshot_window: v.cluster_snapshot_window,
            cluster_maintenance_window: v.cluster_maintenance_window,
            cluster_parameter_group_name: v.cluster_parameter_group_name,
            kms_key_id: v.kms_key_id,
        }
    }
}

impl From<&SubnetGroupData> for SubnetGroupView {
    fn from(sg: &SubnetGroupData) -> Self {
        SubnetGroupView {
            name: sg.name.clone(),
            arn: sg.arn.clone(),
            description: sg.description.clone(),
            vpc_id: sg.vpc_id.clone(),
            subnet_ids: sg.subnet_ids.clone(),
        }
    }
}

impl From<SubnetGroupView> for SubnetGroupData {
    fn from(v: SubnetGroupView) -> Self {
        SubnetGroupData {
            name: v.name,
            arn: v.arn,
            description: v.description,
            vpc_id: v.vpc_id,
            subnet_ids: v.subnet_ids,
        }
    }
}

impl From<&TagData> for TagView {
    fn from(t: &TagData) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagView> for TagData {
    fn from(v: TagView) -> Self {
        TagData {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<&AclData> for AclView {
    fn from(a: &AclData) -> Self {
        AclView {
            acl_name: a.acl_name.clone(),
            status: a.status.clone(),
            user_names: a.user_names.clone(),
            minimum_engine_version: a.minimum_engine_version.clone(),
            arn: a.arn.clone(),
            tags: a.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<AclView> for AclData {
    fn from(v: AclView) -> Self {
        AclData {
            acl_name: v.acl_name,
            status: v.status,
            user_names: v.user_names,
            minimum_engine_version: v.minimum_engine_version,
            arn: v.arn,
            tags: v.tags.into_iter().map(TagData::from).collect(),
        }
    }
}

fn tags_map_to_view(tags: &TagsMap) -> HashMap<String, Vec<TagView>> {
    tags.iter()
        .map(|(arn, ts)| (arn.clone(), ts.iter().map(TagView::from).collect()))
        .collect()
}

fn view_to_tags_map(tags: HashMap<String, Vec<TagView>>) -> TagsMap {
    tags.into_iter()
        .map(|(arn, ts)| (arn, ts.into_iter().map(TagData::from).collect()))
        .collect()
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for MemoryDbService {
    type StateView = MemoryDbStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let clusters = guard
            .clusters
            .iter()
            .map(|(k, v)| (k.clone(), ClusterView::from(v)))
            .collect();
        let snapshots = guard
            .snapshots
            .iter()
            .map(|(k, v)| (k.clone(), SnapshotView::from(v)))
            .collect();
        let subnet_groups = guard
            .subnet_groups
            .iter()
            .map(|(k, v)| (k.clone(), SubnetGroupView::from(v)))
            .collect();
        let acls = guard
            .acls
            .iter()
            .map(|(k, v)| (k.clone(), AclView::from(v)))
            .collect();
        let tags = tags_map_to_view(&guard.tags);

        MemoryDbStateView {
            clusters,
            snapshots,
            subnet_groups,
            acls,
            tags,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = MemoryDbState {
            clusters: view
                .clusters
                .into_values()
                .map(|v| {
                    let c = Cluster::from(v);
                    (c.name.clone(), c)
                })
                .collect(),
            snapshots: view
                .snapshots
                .into_values()
                .map(|v| {
                    let s = SnapshotData::from(v);
                    (s.name.clone(), s)
                })
                .collect(),
            subnet_groups: view
                .subnet_groups
                .into_values()
                .map(|v| {
                    let sg = SubnetGroupData::from(v);
                    (sg.name.clone(), sg)
                })
                .collect(),
            acls: view
                .acls
                .into_values()
                .map(|v| {
                    let a = AclData::from(v);
                    (a.acl_name.clone(), a)
                })
                .collect(),
            tags: view_to_tags_map(view.tags),
        };

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
            for v in view.clusters.into_values() {
                let c = Cluster::from(v);
                guard.clusters.insert(c.name.clone(), c);
            }
            for v in view.snapshots.into_values() {
                let s = SnapshotData::from(v);
                guard.snapshots.insert(s.name.clone(), s);
            }
            for v in view.subnet_groups.into_values() {
                let sg = SubnetGroupData::from(v);
                guard.subnet_groups.insert(sg.name.clone(), sg);
            }
            for v in view.acls.into_values() {
                let a = AclData::from(v);
                guard.acls.insert(a.acl_name.clone(), a);
            }
            for (arn, ts) in view_to_tags_map(view.tags) {
                guard.tags.insert(arn, ts);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
