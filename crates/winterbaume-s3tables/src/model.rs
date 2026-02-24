//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3tables

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamespaceRequest {
    #[serde(default)]
    pub namespace: Vec<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamespaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableBucketRequest {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "storageClassConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_configuration: Option<StorageClassConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "sseAlgorithm")]
    #[serde(default)]
    pub sse_algorithm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageClassConfiguration {
    #[serde(rename = "storageClass")]
    #[serde(default)]
    pub storage_class: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableBucketResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableRequest {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TableMetadata>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "storageClassConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_configuration: Option<StorageClassConfiguration>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<IcebergMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergMetadata {
    #[serde(rename = "partitionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_spec: Option<IcebergPartitionSpec>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<IcebergSchema>,
    #[serde(rename = "schemaV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_v2: Option<IcebergSchemaV2>,
    #[serde(rename = "writeOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_order: Option<IcebergSortOrder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergPartitionSpec {
    #[serde(default)]
    pub fields: Vec<IcebergPartitionField>,
    #[serde(rename = "spec-id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergPartitionField {
    #[serde(rename = "field-id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "source-id")]
    #[serde(default)]
    pub source_id: i32,
    #[serde(default)]
    pub transform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSchema {
    #[serde(default)]
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSchemaV2 {
    #[serde(default)]
    pub fields: Vec<SchemaV2Field>,
    #[serde(rename = "identifier-field-ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_field_ids: Option<Vec<i32>>,
    #[serde(rename = "schema-id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<i32>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaV2Field {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSortOrder {
    #[serde(default)]
    pub fields: Vec<IcebergSortField>,
    #[serde(rename = "order-id")]
    #[serde(default)]
    pub order_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSortField {
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "null-order")]
    #[serde(default)]
    pub null_order: String,
    #[serde(rename = "source-id")]
    #[serde(default)]
    pub source_id: i32,
    #[serde(default)]
    pub transform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableResponse {
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamespaceRequest {
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableBucketEncryptionRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableBucketMetricsConfigurationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableBucketPolicyRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableBucketReplicationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableBucketRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTablePolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableReplicationRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    pub version_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamespaceRequest {
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamespaceResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketEncryptionRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketEncryptionResponse {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketMaintenanceConfigurationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketMaintenanceConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration:
        Option<std::collections::HashMap<String, TableBucketMaintenanceConfigurationValue>>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBucketMaintenanceConfigurationValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<TableBucketMaintenanceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBucketMaintenanceSettings {
    #[serde(rename = "icebergUnreferencedFileRemoval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_unreferenced_file_removal: Option<IcebergUnreferencedFileRemovalSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergUnreferencedFileRemovalSettings {
    #[serde(rename = "nonCurrentDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_current_days: Option<i32>,
    #[serde(rename = "unreferencedDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreferenced_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketMetricsConfigurationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketMetricsConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketPolicyRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketPolicyResponse {
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketReplicationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketReplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TableBucketReplicationConfiguration>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBucketReplicationConfiguration {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub rules: Vec<TableBucketReplicationRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBucketReplicationRule {
    #[serde(default)]
    pub destinations: Vec<ReplicationDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationDestination {
    #[serde(rename = "destinationTableBucketARN")]
    #[serde(default)]
    pub destination_table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketStorageClassRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableBucketStorageClassResponse {
    #[serde(rename = "storageClassConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_configuration: Option<StorageClassConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableEncryptionRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableEncryptionResponse {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMaintenanceConfigurationRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMaintenanceConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration:
        Option<std::collections::HashMap<String, TableMaintenanceConfigurationValue>>,
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMaintenanceConfigurationValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<TableMaintenanceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMaintenanceSettings {
    #[serde(rename = "icebergCompaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_compaction: Option<IcebergCompactionSettings>,
    #[serde(rename = "icebergSnapshotManagement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_snapshot_management: Option<IcebergSnapshotManagementSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergCompactionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(rename = "targetFileSizeMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_file_size_m_b: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSnapshotManagementSettings {
    #[serde(rename = "maxSnapshotAgeHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_snapshot_age_hours: Option<i32>,
    #[serde(rename = "minSnapshotsToKeep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_snapshots_to_keep: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMaintenanceJobStatusRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMaintenanceJobStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::collections::HashMap<String, TableMaintenanceJobStatusValue>>,
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMaintenanceJobStatusValue {
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "lastRunTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMetadataLocationRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMetadataLocationResponse {
    #[serde(rename = "metadataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
    #[serde(rename = "warehouseLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTablePolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTablePolicyResponse {
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRecordExpirationConfigurationRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRecordExpirationConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TableRecordExpirationConfigurationValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRecordExpirationConfigurationValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<TableRecordExpirationSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRecordExpirationSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRecordExpirationJobStatusRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRecordExpirationJobStatusResponse {
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "lastRunTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<TableRecordExpirationJobMetrics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRecordExpirationJobMetrics {
    #[serde(rename = "deletedDataFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_data_files: Option<i64>,
    #[serde(rename = "deletedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_records: Option<i64>,
    #[serde(rename = "removedFilesSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_files_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableReplicationRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableReplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TableReplicationConfiguration>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableReplicationConfiguration {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub rules: Vec<TableReplicationRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableReplicationRule {
    #[serde(default)]
    pub destinations: Vec<ReplicationDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableReplicationStatusRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableReplicationStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<ReplicationDestinationStatusModel>>,
    #[serde(rename = "sourceTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationDestinationStatusModel {
    #[serde(rename = "destinationTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_arn: Option<String>,
    #[serde(rename = "destinationTableBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_bucket_arn: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "lastSuccessfulReplicatedUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_replicated_update: Option<LastSuccessfulReplicatedUpdate>,
    #[serde(rename = "replicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastSuccessfulReplicatedUpdate {
    #[serde(rename = "metadataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "tableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "managedByService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_service: Option<String>,
    #[serde(rename = "managedTableInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_table_information: Option<ManagedTableInformation>,
    #[serde(rename = "metadataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
    #[serde(rename = "warehouseLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedTableInformation {
    #[serde(rename = "replicationInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_information: Option<ReplicationInformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInformation {
    #[serde(rename = "sourceTableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableStorageClassRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableStorageClassResponse {
    #[serde(rename = "storageClassConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_configuration: Option<StorageClassConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamespacesRequest {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "maxNamespaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_namespaces: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamespacesResponse {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamespaceSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableBucketsRequest {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "maxBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_buckets: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableBucketsResponse {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "tableBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_buckets: Option<Vec<TableBucketSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBucketSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesRequest {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "maxTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tables: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesResponse {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "managedByService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_service: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
    #[serde(rename = "tableBucketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketEncryptionRequest {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    pub encryption_configuration: EncryptionConfiguration,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketMaintenanceConfigurationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: TableBucketMaintenanceConfigurationValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketMetricsConfigurationRequest {
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketPolicyRequest {
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketReplicationRequest {
    #[serde(default)]
    pub configuration: TableBucketReplicationConfiguration,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketReplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableBucketStorageClassRequest {
    #[serde(rename = "storageClassConfiguration")]
    #[serde(default)]
    pub storage_class_configuration: StorageClassConfiguration,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableMaintenanceConfigurationRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: TableMaintenanceConfigurationValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTablePolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableRecordExpirationConfigurationRequest {
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
    #[serde(default)]
    pub value: TableRecordExpirationConfigurationValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableReplicationRequest {
    #[serde(default)]
    pub configuration: TableReplicationConfiguration,
    #[serde(rename = "tableArn")]
    #[serde(default)]
    pub table_arn: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTableReplicationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameTableRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "newName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "newNamespaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_namespace_name: Option<String>,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableMetadataLocationRequest {
    #[serde(rename = "metadataLocation")]
    #[serde(default)]
    pub metadata_location: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "tableBucketARN")]
    #[serde(default)]
    pub table_bucket_a_r_n: String,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    pub version_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableMetadataLocationResponse {
    #[serde(rename = "metadataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(rename = "tableARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_a_r_n: Option<String>,
    #[serde(rename = "versionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
}
