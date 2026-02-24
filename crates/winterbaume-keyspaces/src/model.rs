//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-keyspaces

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeyspaceRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "replicationSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_specification: Option<ReplicationSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationSpecification {
    #[serde(rename = "regionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_list: Option<Vec<String>>,
    #[serde(rename = "replicationStrategy")]
    #[serde(default)]
    pub replication_strategy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeyspaceResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableRequest {
    #[serde(rename = "autoScalingSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_specification: Option<AutoScalingSpecification>,
    #[serde(rename = "capacitySpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_specification: Option<CapacitySpecification>,
    #[serde(rename = "cdcSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_specification: Option<CdcSpecification>,
    #[serde(rename = "clientSideTimestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_side_timestamps: Option<ClientSideTimestamps>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    #[serde(rename = "defaultTimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_time_to_live: Option<i32>,
    #[serde(rename = "encryptionSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_specification: Option<EncryptionSpecification>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "pointInTimeRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery: Option<PointInTimeRecovery>,
    #[serde(rename = "replicaSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_specifications: Option<Vec<ReplicaSpecification>>,
    #[serde(rename = "schemaDefinition")]
    #[serde(default)]
    pub schema_definition: SchemaDefinition,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<TimeToLive>,
    #[serde(rename = "warmThroughputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_specification: Option<WarmThroughputSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingSpecification {
    #[serde(rename = "readCapacityAutoScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_auto_scaling: Option<AutoScalingSettings>,
    #[serde(rename = "writeCapacityAutoScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_auto_scaling: Option<AutoScalingSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingSettings {
    #[serde(rename = "autoScalingDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_disabled: Option<bool>,
    #[serde(rename = "maximumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_units: Option<i64>,
    #[serde(rename = "minimumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_units: Option<i64>,
    #[serde(rename = "scalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<AutoScalingPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicy {
    #[serde(rename = "targetTrackingScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTrackingScalingPolicyConfiguration {
    #[serde(rename = "disableScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "scaleInCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i32>,
    #[serde(rename = "scaleOutCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i32>,
    #[serde(rename = "targetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacitySpecification {
    #[serde(rename = "readCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    #[serde(rename = "throughputMode")]
    #[serde(default)]
    pub throughput_mode: String,
    #[serde(rename = "writeCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CdcSpecification {
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "viewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientSideTimestamps {
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Comment {
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionSpecification {
    #[serde(rename = "kmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PointInTimeRecovery {
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaSpecification {
    #[serde(rename = "readCapacityAutoScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_auto_scaling: Option<AutoScalingSettings>,
    #[serde(rename = "readCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaDefinition {
    #[serde(rename = "allColumns")]
    #[serde(default)]
    pub all_columns: Vec<ColumnDefinition>,
    #[serde(rename = "clusteringKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering_keys: Option<Vec<ClusteringKey>>,
    #[serde(rename = "partitionKeys")]
    #[serde(default)]
    pub partition_keys: Vec<PartitionKey>,
    #[serde(rename = "staticColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_columns: Option<Vec<StaticColumn>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusteringKey {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    pub order_by: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionKey {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticColumn {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeToLive {
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarmThroughputSpecification {
    #[serde(rename = "readUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_units_per_second: Option<i64>,
    #[serde(rename = "writeUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_units_per_second: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypeRequest {
    #[serde(rename = "fieldDefinitions")]
    #[serde(default)]
    pub field_definitions: Vec<FieldDefinition>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypeResponse {
    #[serde(rename = "keyspaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_arn: Option<String>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeyspaceRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeyspaceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypeRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypeResponse {
    #[serde(rename = "keyspaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_arn: Option<String>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyspaceRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyspaceResponse {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "replicationGroupStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_statuses: Option<Vec<ReplicationGroupStatus>>,
    #[serde(rename = "replicationRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_regions: Option<Vec<String>>,
    #[serde(rename = "replicationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_strategy: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationGroupStatus {
    #[serde(rename = "keyspaceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "tablesReplicationProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_replication_progress: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableAutoScalingSettingsRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableAutoScalingSettingsResponse {
    #[serde(rename = "autoScalingSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_specification: Option<AutoScalingSpecification>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "replicaSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_specifications: Option<Vec<ReplicaAutoScalingSpecification>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaAutoScalingSpecification {
    #[serde(rename = "autoScalingSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_specification: Option<AutoScalingSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableResponse {
    #[serde(rename = "capacitySpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_specification: Option<CapacitySpecificationSummary>,
    #[serde(rename = "cdcSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_specification: Option<CdcSpecificationSummary>,
    #[serde(rename = "clientSideTimestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_side_timestamps: Option<ClientSideTimestamps>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    #[serde(rename = "creationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "defaultTimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_time_to_live: Option<i32>,
    #[serde(rename = "encryptionSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_specification: Option<EncryptionSpecification>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "latestStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<String>,
    #[serde(rename = "pointInTimeRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery: Option<PointInTimeRecoverySummary>,
    #[serde(rename = "replicaSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_specifications: Option<Vec<ReplicaSpecificationSummary>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "schemaDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_definition: Option<SchemaDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<TimeToLive>,
    #[serde(rename = "warmThroughputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_specification: Option<WarmThroughputSpecificationSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacitySpecificationSummary {
    #[serde(rename = "lastUpdateToPayPerRequestTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_timestamp: Option<f64>,
    #[serde(rename = "readCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    #[serde(rename = "throughputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
    #[serde(rename = "writeCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CdcSpecificationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "viewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PointInTimeRecoverySummary {
    #[serde(rename = "earliestRestorableTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_timestamp: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaSpecificationSummary {
    #[serde(rename = "capacitySpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_specification: Option<CapacitySpecificationSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "warmThroughputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_specification: Option<WarmThroughputSpecificationSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarmThroughputSpecificationSummary {
    #[serde(rename = "readUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_units_per_second: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "writeUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_units_per_second: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypeRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypeResponse {
    #[serde(rename = "directParentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_parent_types: Option<Vec<String>>,
    #[serde(rename = "directReferringTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_referring_tables: Option<Vec<String>>,
    #[serde(rename = "fieldDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_definitions: Option<Vec<FieldDefinition>>,
    #[serde(rename = "keyspaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_arn: Option<String>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "lastModifiedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[serde(rename = "maxNestingDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nesting_depth: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyspacesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyspacesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspaces: Option<Vec<KeyspaceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyspaceSummary {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "replicationRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_regions: Option<Vec<String>>,
    #[serde(rename = "replicationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_strategy: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableSummary {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesRequest {
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableRequest {
    #[serde(rename = "autoScalingSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_specification: Option<AutoScalingSpecification>,
    #[serde(rename = "capacitySpecificationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_specification_override: Option<CapacitySpecification>,
    #[serde(rename = "encryptionSpecificationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_specification_override: Option<EncryptionSpecification>,
    #[serde(rename = "pointInTimeRecoveryOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_override: Option<PointInTimeRecovery>,
    #[serde(rename = "replicaSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_specifications: Option<Vec<ReplicaSpecification>>,
    #[serde(rename = "restoreTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_timestamp: Option<f64>,
    #[serde(rename = "sourceKeyspaceName")]
    #[serde(default)]
    pub source_keyspace_name: String,
    #[serde(rename = "sourceTableName")]
    #[serde(default)]
    pub source_table_name: String,
    #[serde(rename = "tagsOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_override: Option<Vec<Tag>>,
    #[serde(rename = "targetKeyspaceName")]
    #[serde(default)]
    pub target_keyspace_name: String,
    #[serde(rename = "targetTableName")]
    #[serde(default)]
    pub target_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableResponse {
    #[serde(rename = "restoredTableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored_table_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeyspaceRequest {
    #[serde(rename = "clientSideTimestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_side_timestamps: Option<ClientSideTimestamps>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "replicationSpecification")]
    #[serde(default)]
    pub replication_specification: ReplicationSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeyspaceResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableRequest {
    #[serde(rename = "addColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_columns: Option<Vec<ColumnDefinition>>,
    #[serde(rename = "autoScalingSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_specification: Option<AutoScalingSpecification>,
    #[serde(rename = "capacitySpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_specification: Option<CapacitySpecification>,
    #[serde(rename = "cdcSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_specification: Option<CdcSpecification>,
    #[serde(rename = "clientSideTimestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_side_timestamps: Option<ClientSideTimestamps>,
    #[serde(rename = "defaultTimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_time_to_live: Option<i32>,
    #[serde(rename = "encryptionSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_specification: Option<EncryptionSpecification>,
    #[serde(rename = "keyspaceName")]
    #[serde(default)]
    pub keyspace_name: String,
    #[serde(rename = "pointInTimeRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery: Option<PointInTimeRecovery>,
    #[serde(rename = "replicaSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_specifications: Option<Vec<ReplicaSpecification>>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<TimeToLive>,
    #[serde(rename = "warmThroughputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_specification: Option<WarmThroughputSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}
