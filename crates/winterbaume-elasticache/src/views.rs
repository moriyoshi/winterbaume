//! Serde-compatible view types for ElastiCache state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ElastiCacheService;
use crate::state::ElastiCacheState;
use crate::types::{
    CacheCluster, CacheParameterGroup, CacheSecurityGroup, CacheSubnetGroup, ReplicationGroup,
    Snapshot, Tag, User,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElastiCacheStateView {
    #[serde(default)]
    pub cache_clusters: HashMap<String, CacheClusterView>,
    #[serde(default)]
    pub replication_groups: HashMap<String, ReplicationGroupView>,
    #[serde(default)]
    pub cache_subnet_groups: HashMap<String, CacheSubnetGroupView>,
    #[serde(default)]
    pub cache_parameter_groups: HashMap<String, CacheParameterGroupView>,
    #[serde(default)]
    pub cache_security_groups: HashMap<String, CacheSecurityGroupView>,
    #[serde(default)]
    pub snapshots: HashMap<String, SnapshotView>,
    #[serde(default)]
    pub users: HashMap<String, UserView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheClusterView {
    pub cache_cluster_id: String,
    pub status: String,
    pub engine: String,
    pub engine_version: String,
    pub cache_node_type: String,
    pub num_cache_nodes: i32,
    pub preferred_availability_zone: String,
    pub cache_subnet_group_name: Option<String>,
    pub replication_group_id: Option<String>,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(default)]
    pub log_delivery_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationGroupView {
    pub replication_group_id: String,
    pub description: String,
    pub status: String,
    pub member_clusters: Vec<String>,
    pub cache_node_type: String,
    pub engine: String,
    pub automatic_failover: String,
    pub multi_az: String,
    pub arn: String,
    pub tags: Vec<TagView>,
    pub num_cache_clusters: i32,
    #[serde(default)]
    pub log_delivery_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSubnetGroupView {
    pub name: String,
    pub description: String,
    pub subnet_ids: Vec<String>,
    pub vpc_id: String,
    pub arn: String,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheParameterGroupView {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(default)]
    pub parameter: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSecurityGroupView {
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub arn: String,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotView {
    pub snapshot_name: String,
    pub cache_cluster_id: Option<String>,
    pub replication_group_id: Option<String>,
    pub status: String,
    pub engine: String,
    pub engine_version: String,
    pub cache_node_type: String,
    pub cache_subnet_group_name: Option<String>,
    pub arn: String,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserView {
    pub user_id: String,
    pub user_name: String,
    pub engine: String,
    pub status: String,
    pub access_string: String,
    pub arn: String,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// ---------- Tag conversions -------------------------------------------------

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Tag {
            key: v.key,
            value: v.value,
        }
    }
}

// ---------- State -> View ---------------------------------------------------

impl From<&ElastiCacheState> for ElastiCacheStateView {
    fn from(state: &ElastiCacheState) -> Self {
        ElastiCacheStateView {
            cache_clusters: state
                .cache_clusters
                .iter()
                .map(|(k, v)| (k.clone(), CacheClusterView::from(v)))
                .collect(),
            replication_groups: state
                .replication_groups
                .iter()
                .map(|(k, v)| (k.clone(), ReplicationGroupView::from(v)))
                .collect(),
            cache_subnet_groups: state
                .cache_subnet_groups
                .iter()
                .map(|(k, v)| (k.clone(), CacheSubnetGroupView::from(v)))
                .collect(),
            cache_parameter_groups: state
                .cache_parameter_groups
                .iter()
                .map(|(k, v)| (k.clone(), CacheParameterGroupView::from(v)))
                .collect(),
            cache_security_groups: state
                .cache_security_groups
                .iter()
                .map(|(k, v)| (k.clone(), CacheSecurityGroupView::from(v)))
                .collect(),
            snapshots: state
                .snapshots
                .iter()
                .map(|(k, v)| (k.clone(), SnapshotView::from(v)))
                .collect(),
            users: state
                .users
                .iter()
                .map(|(k, v)| (k.clone(), UserView::from(v)))
                .collect(),
        }
    }
}

impl From<&CacheCluster> for CacheClusterView {
    fn from(c: &CacheCluster) -> Self {
        CacheClusterView {
            cache_cluster_id: c.cache_cluster_id.clone(),
            status: c.status.clone(),
            engine: c.engine.clone(),
            engine_version: c.engine_version.clone(),
            cache_node_type: c.cache_node_type.clone(),
            num_cache_nodes: c.num_cache_nodes,
            preferred_availability_zone: c.preferred_availability_zone.clone(),
            cache_subnet_group_name: c.cache_subnet_group_name.clone(),
            replication_group_id: c.replication_group_id.clone(),
            arn: c.arn.clone(),
            tags: c.tags.iter().map(TagView::from).collect(),
            log_delivery_configuration: None,
        }
    }
}

impl From<&ReplicationGroup> for ReplicationGroupView {
    fn from(r: &ReplicationGroup) -> Self {
        ReplicationGroupView {
            replication_group_id: r.replication_group_id.clone(),
            description: r.description.clone(),
            status: r.status.clone(),
            member_clusters: r.member_clusters.clone(),
            cache_node_type: r.cache_node_type.clone(),
            engine: r.engine.clone(),
            automatic_failover: r.automatic_failover.clone(),
            multi_az: r.multi_az.clone(),
            arn: r.arn.clone(),
            tags: r.tags.iter().map(TagView::from).collect(),
            num_cache_clusters: r.num_cache_clusters,
            log_delivery_configuration: None,
        }
    }
}

impl From<&CacheSubnetGroup> for CacheSubnetGroupView {
    fn from(s: &CacheSubnetGroup) -> Self {
        CacheSubnetGroupView {
            name: s.name.clone(),
            description: s.description.clone(),
            subnet_ids: s.subnet_ids.clone(),
            vpc_id: s.vpc_id.clone(),
            arn: s.arn.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&CacheParameterGroup> for CacheParameterGroupView {
    fn from(p: &CacheParameterGroup) -> Self {
        CacheParameterGroupView {
            name: p.name.clone(),
            family: p.family.clone(),
            description: p.description.clone(),
            arn: p.arn.clone(),
            tags: p.tags.iter().map(TagView::from).collect(),
            parameter: None,
        }
    }
}

impl From<&CacheSecurityGroup> for CacheSecurityGroupView {
    fn from(s: &CacheSecurityGroup) -> Self {
        CacheSecurityGroupView {
            name: s.name.clone(),
            description: s.description.clone(),
            owner_id: s.owner_id.clone(),
            arn: s.arn.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Snapshot> for SnapshotView {
    fn from(s: &Snapshot) -> Self {
        SnapshotView {
            snapshot_name: s.snapshot_name.clone(),
            cache_cluster_id: s.cache_cluster_id.clone(),
            replication_group_id: s.replication_group_id.clone(),
            status: s.status.clone(),
            engine: s.engine.clone(),
            engine_version: s.engine_version.clone(),
            cache_node_type: s.cache_node_type.clone(),
            cache_subnet_group_name: s.cache_subnet_group_name.clone(),
            arn: s.arn.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        UserView {
            user_id: u.user_id.clone(),
            user_name: u.user_name.clone(),
            engine: u.engine.clone(),
            status: u.status.clone(),
            access_string: u.access_string.clone(),
            arn: u.arn.clone(),
            tags: u.tags.iter().map(TagView::from).collect(),
        }
    }
}

// ---------- View -> State ---------------------------------------------------

impl From<ElastiCacheStateView> for ElastiCacheState {
    fn from(view: ElastiCacheStateView) -> Self {
        ElastiCacheState {
            cache_clusters: view
                .cache_clusters
                .into_iter()
                .map(|(k, v)| (k, CacheCluster::from(v)))
                .collect(),
            replication_groups: view
                .replication_groups
                .into_iter()
                .map(|(k, v)| (k, ReplicationGroup::from(v)))
                .collect(),
            cache_subnet_groups: view
                .cache_subnet_groups
                .into_iter()
                .map(|(k, v)| (k, CacheSubnetGroup::from(v)))
                .collect(),
            cache_parameter_groups: view
                .cache_parameter_groups
                .into_iter()
                .map(|(k, v)| (k, CacheParameterGroup::from(v)))
                .collect(),
            cache_security_groups: view
                .cache_security_groups
                .into_iter()
                .map(|(k, v)| (k, CacheSecurityGroup::from(v)))
                .collect(),
            snapshots: view
                .snapshots
                .into_iter()
                .map(|(k, v)| (k, Snapshot::from(v)))
                .collect(),
            users: view
                .users
                .into_iter()
                .map(|(k, v)| (k, User::from(v)))
                .collect(),
        }
    }
}

impl From<CacheClusterView> for CacheCluster {
    fn from(v: CacheClusterView) -> Self {
        CacheCluster {
            cache_cluster_id: v.cache_cluster_id,
            status: v.status,
            engine: v.engine,
            engine_version: v.engine_version,
            cache_node_type: v.cache_node_type,
            num_cache_nodes: v.num_cache_nodes,
            preferred_availability_zone: v.preferred_availability_zone,
            cache_subnet_group_name: v.cache_subnet_group_name,
            replication_group_id: v.replication_group_id,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<ReplicationGroupView> for ReplicationGroup {
    fn from(v: ReplicationGroupView) -> Self {
        ReplicationGroup {
            replication_group_id: v.replication_group_id,
            description: v.description,
            status: v.status,
            member_clusters: v.member_clusters,
            cache_node_type: v.cache_node_type,
            engine: v.engine,
            automatic_failover: v.automatic_failover,
            multi_az: v.multi_az,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            num_cache_clusters: v.num_cache_clusters,
        }
    }
}

impl From<CacheSubnetGroupView> for CacheSubnetGroup {
    fn from(v: CacheSubnetGroupView) -> Self {
        CacheSubnetGroup {
            name: v.name,
            description: v.description,
            subnet_ids: v.subnet_ids,
            vpc_id: v.vpc_id,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<CacheParameterGroupView> for CacheParameterGroup {
    fn from(v: CacheParameterGroupView) -> Self {
        CacheParameterGroup {
            name: v.name,
            family: v.family,
            description: v.description,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<CacheSecurityGroupView> for CacheSecurityGroup {
    fn from(v: CacheSecurityGroupView) -> Self {
        CacheSecurityGroup {
            name: v.name,
            description: v.description,
            owner_id: v.owner_id,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<SnapshotView> for Snapshot {
    fn from(v: SnapshotView) -> Self {
        Snapshot {
            snapshot_name: v.snapshot_name,
            cache_cluster_id: v.cache_cluster_id,
            replication_group_id: v.replication_group_id,
            status: v.status,
            engine: v.engine,
            engine_version: v.engine_version,
            cache_node_type: v.cache_node_type,
            cache_subnet_group_name: v.cache_subnet_group_name,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<UserView> for User {
    fn from(v: UserView) -> Self {
        User {
            user_id: v.user_id,
            user_name: v.user_name,
            engine: v.engine,
            status: v.status,
            access_string: v.access_string,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

// ---------- StatefulService impl --------------------------------------------

impl StatefulService for ElastiCacheService {
    type StateView = ElastiCacheStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ElastiCacheStateView::from(&*guard)
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
            *guard = ElastiCacheState::from(view);
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
            for (k, v) in view.cache_clusters {
                guard.cache_clusters.insert(k, CacheCluster::from(v));
            }
            for (k, v) in view.replication_groups {
                guard
                    .replication_groups
                    .insert(k, ReplicationGroup::from(v));
            }
            for (k, v) in view.cache_subnet_groups {
                guard
                    .cache_subnet_groups
                    .insert(k, CacheSubnetGroup::from(v));
            }
            for (k, v) in view.cache_parameter_groups {
                guard
                    .cache_parameter_groups
                    .insert(k, CacheParameterGroup::from(v));
            }
            for (k, v) in view.cache_security_groups {
                guard
                    .cache_security_groups
                    .insert(k, CacheSecurityGroup::from(v));
            }
            for (k, v) in view.snapshots {
                guard.snapshots.insert(k, Snapshot::from(v));
            }
            for (k, v) in view.users {
                guard.users.insert(k, User::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
