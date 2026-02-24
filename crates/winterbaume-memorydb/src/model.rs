//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-memorydb

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateClusterRequest {
    #[serde(rename = "ClusterNames")]
    #[serde(default)]
    pub cluster_names: Vec<String>,
    #[serde(rename = "ServiceUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update: Option<ServiceUpdateRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceUpdateRequest {
    #[serde(rename = "ServiceUpdateNameToApply")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name_to_apply: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateClusterResponse {
    #[serde(rename = "ProcessedClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_clusters: Option<Vec<Cluster>>,
    #[serde(rename = "UnprocessedClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_clusters: Option<Vec<UnprocessedCluster>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l_name: Option<String>,
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<String>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<Endpoint>,
    #[serde(rename = "DataTiering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EnginePatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_patch_version: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<i32>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "ParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
    #[serde(rename = "PendingUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_updates: Option<ClusterPendingUpdates>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroupMembership>>,
    #[serde(rename = "Shards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SnsTopicStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(rename = "TLSEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterPendingUpdates {
    #[serde(rename = "ACLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_ls: Option<ACLsUpdateStatus>,
    #[serde(rename = "Resharding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resharding: Option<ReshardingStatus>,
    #[serde(rename = "ServiceUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_updates: Option<Vec<PendingModifiedServiceUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ACLsUpdateStatus {
    #[serde(rename = "ACLToApply")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l_to_apply: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReshardingStatus {
    #[serde(rename = "SlotMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_migration: Option<SlotMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotMigration {
    #[serde(rename = "ProgressPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingModifiedServiceUpdate {
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroupMembership {
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Shard {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedCluster {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopySnapshotRequest {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceSnapshotName")]
    #[serde(default)]
    pub source_snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_bucket: Option<String>,
    #[serde(rename = "TargetSnapshotName")]
    #[serde(default)]
    pub target_snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopySnapshotResponse {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Snapshot {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<ClusterConfiguration>,
    #[serde(rename = "DataTiering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterConfiguration {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_group_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_shards: Option<i32>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Shards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<ShardDetail>>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShardDetail {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ShardConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "SnapshotCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_creation_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShardConfiguration {
    #[serde(rename = "ReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateACLRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub a_c_l_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateACLResponse {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<ACL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ACL {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PendingChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_changes: Option<ACLPendingChanges>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ACLPendingChanges {
    #[serde(rename = "UserNamesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names_to_add: Option<Vec<String>>,
    #[serde(rename = "UserNamesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names_to_remove: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub a_c_l_name: String,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "DataTiering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    pub node_type: String,
    #[serde(rename = "NumReplicasPerShard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_replicas_per_shard: Option<i32>,
    #[serde(rename = "NumShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_shards: Option<i32>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SnapshotArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<Vec<String>>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(rename = "TLSEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_enabled: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiRegionClusterRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MultiRegionClusterNameSuffix")]
    #[serde(default)]
    pub multi_region_cluster_name_suffix: String,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_group_name: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    pub node_type: String,
    #[serde(rename = "NumShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_shards: Option<i32>,
    #[serde(rename = "TLSEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_enabled: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiRegionClusterResponse {
    #[serde(rename = "MultiRegionCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster: Option<MultiRegionCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionCluster {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<RegionalCluster>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_group_name: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TLSEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionalCluster {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParameterGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Family")]
    #[serde(default)]
    pub family: String,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParameterGroupResponse {
    #[serde(rename = "ParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotResponse {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubnetGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    pub subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubnetGroupResponse {
    #[serde(rename = "SubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subnet {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<AvailabilityZone>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "AccessString")]
    #[serde(default)]
    pub access_string: String,
    #[serde(rename = "AuthenticationMode")]
    #[serde(default)]
    pub authentication_mode: AuthenticationMode,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationMode {
    #[serde(rename = "Passwords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "ACLNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l_names: Option<Vec<String>>,
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AccessString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<String>,
    #[serde(rename = "Authentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authentication {
    #[serde(rename = "PasswordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteACLRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub a_c_l_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteACLResponse {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<ACL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "FinalSnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_name: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiRegionClusterRequest {
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    pub multi_region_cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiRegionClusterResponse {
    #[serde(rename = "MultiRegionCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster: Option<MultiRegionCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParameterGroupRequest {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParameterGroupResponse {
    #[serde(rename = "ParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotRequest {
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotResponse {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubnetGroupRequest {
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    pub subnet_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubnetGroupResponse {
    #[serde(rename = "SubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeACLsRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeACLsResponse {
    #[serde(rename = "ACLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_ls: Option<Vec<ACL>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClustersRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ShowShardDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_shard_details: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClustersResponse {
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEngineVersionsRequest {
    #[serde(rename = "DefaultOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_only: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEngineVersionsResponse {
    #[serde(rename = "EngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<Vec<EngineVersionInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineVersionInfo {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EnginePatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_patch_version: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventsRequest {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventsResponse {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionClustersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ShowClusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cluster_details: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionClustersResponse {
    #[serde(rename = "MultiRegionClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_clusters: Option<Vec<MultiRegionCluster>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionParameterGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_group_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionParameterGroupsResponse {
    #[serde(rename = "MultiRegionParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_groups: Option<Vec<MultiRegionParameterGroup>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionParameterGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionParametersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    pub multi_region_parameter_group_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiRegionParametersResponse {
    #[serde(rename = "MultiRegionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameters: Option<Vec<MultiRegionParameter>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionParameter {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParameterGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParameterGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<Vec<ParameterGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParametersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParametersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameter {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedNodesOfferingsRequest {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ReservedNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedNodesOfferingsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedNodesOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes_offerings: Option<Vec<ReservedNodesOffering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNodesOffering {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    #[serde(rename = "ReservedNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurringCharge {
    #[serde(rename = "RecurringChargeAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_amount: Option<f64>,
    #[serde(rename = "RecurringChargeFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedNodesRequest {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "ReservedNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedNodesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes: Option<Vec<ReservedNode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNode {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "NodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "ReservedNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes_offering_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceUpdatesRequest {
    #[serde(rename = "ClusterNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_names: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceUpdatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_updates: Option<Vec<ServiceUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceUpdate {
    #[serde(rename = "AutoUpdateStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_start_date: Option<f64>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "NodesUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes_updated: Option<String>,
    #[serde(rename = "ReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSnapshotsRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ShowDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail: Option<bool>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSnapshotsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Snapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubnetGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubnetGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_groups: Option<Vec<SubnetGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUsersRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverShardRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "ShardName")]
    #[serde(default)]
    pub shard_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverShardResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedMultiRegionClusterUpdatesRequest {
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    pub multi_region_cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedMultiRegionClusterUpdatesResponse {
    #[serde(rename = "ScaleDownNodeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_node_types: Option<Vec<String>>,
    #[serde(rename = "ScaleUpNodeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_up_node_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedNodeTypeUpdatesRequest {
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedNodeTypeUpdatesResponse {
    #[serde(rename = "ScaleDownNodeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_node_types: Option<Vec<String>>,
    #[serde(rename = "ScaleUpNodeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_up_node_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseReservedNodesOfferingRequest {
    #[serde(rename = "NodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "ReservedNodesOfferingId")]
    #[serde(default)]
    pub reserved_nodes_offering_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseReservedNodesOfferingResponse {
    #[serde(rename = "ReservedNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node: Option<ReservedNode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetParameterGroupRequest {
    #[serde(rename = "AllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_parameters: Option<bool>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "ParameterNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetParameterGroupResponse {
    #[serde(rename = "ParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateACLRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub a_c_l_name: String,
    #[serde(rename = "UserNamesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names_to_add: Option<Vec<String>>,
    #[serde(rename = "UserNamesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names_to_remove: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateACLResponse {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<ACL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l_name: Option<String>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    pub cluster_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "ReplicaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_configuration: Option<ReplicaConfigurationRequest>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "ShardConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_configuration: Option<ShardConfigurationRequest>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SnsTopicStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaConfigurationRequest {
    #[serde(rename = "ReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShardConfigurationRequest {
    #[serde(rename = "ShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterResponse {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiRegionClusterRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MultiRegionClusterName")]
    #[serde(default)]
    pub multi_region_cluster_name: String,
    #[serde(rename = "MultiRegionParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_parameter_group_name: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "ShardConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_configuration: Option<ShardConfigurationRequest>,
    #[serde(rename = "UpdateStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiRegionClusterResponse {
    #[serde(rename = "MultiRegionCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_cluster: Option<MultiRegionCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParameterGroupRequest {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "ParameterNameValues")]
    #[serde(default)]
    pub parameter_name_values: Vec<ParameterNameValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterNameValue {
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParameterGroupResponse {
    #[serde(rename = "ParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubnetGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    pub subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubnetGroupResponse {
    #[serde(rename = "SubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "AccessString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<String>,
    #[serde(rename = "AuthenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
