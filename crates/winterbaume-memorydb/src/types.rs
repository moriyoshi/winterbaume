use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A MemoryDB cluster.
#[derive(Debug, Clone)]
pub struct Cluster {
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
    pub creation_time: DateTime<Utc>,
}

/// A MemoryDB snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotData {
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

/// A MemoryDB subnet group.
#[derive(Debug, Clone)]
pub struct SubnetGroupData {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
}

/// A tag key-value pair.
#[derive(Debug, Clone)]
pub struct TagData {
    pub key: String,
    pub value: String,
}

/// A MemoryDB ACL (Access Control List).
#[derive(Debug, Clone)]
pub struct AclData {
    pub acl_name: String,
    pub status: String,
    pub user_names: Vec<String>,
    pub minimum_engine_version: String,
    pub arn: String,
    pub tags: Vec<TagData>,
}

/// Tags storage keyed by resource ARN.
pub type TagsMap = HashMap<String, Vec<TagData>>;
