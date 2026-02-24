//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dynamodb

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementInput {
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "Statements")]
    #[serde(default)]
    pub statements: Vec<BatchStatementRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStatementRequest {
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<AttributeValue>>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "Statement")]
    #[serde(default)]
    pub statement: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValue {
    #[serde(rename = "B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename = "BOOL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_o_o_l: Option<bool>,
    #[serde(rename = "BS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_s: Option<Vec<String>>,
    #[serde(rename = "L")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    #[serde(rename = "M")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "N")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename = "NS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_s: Option<Vec<String>>,
    #[serde(rename = "NULL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_u_l_l: Option<bool>,
    #[serde(rename = "S")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "SS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchStatementResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumedCapacity {
    #[serde(rename = "CapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<std::collections::HashMap<String, Capacity>>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<std::collections::HashMap<String, Capacity>>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<f64>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Capacity>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capacity {
    #[serde(rename = "CapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<f64>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStatementResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchStatementError>,
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStatementError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetItemInput {
    #[serde(rename = "RequestItems")]
    #[serde(default)]
    pub request_items: std::collections::HashMap<String, KeysAndAttributes>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeysAndAttributes {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Keys")]
    #[serde(default)]
    pub keys: Vec<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetItemOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<
        std::collections::HashMap<String, Vec<std::collections::HashMap<String, AttributeValue>>>,
    >,
    #[serde(rename = "UnprocessedKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<std::collections::HashMap<String, KeysAndAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteItemInput {
    #[serde(rename = "RequestItems")]
    #[serde(default)]
    pub request_items: std::collections::HashMap<String, Vec<WriteRequest>>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteRequest {
    #[serde(rename = "DeleteRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_request: Option<DeleteRequest>,
    #[serde(rename = "PutRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_request: Option<PutRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRequest {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRequest {
    #[serde(rename = "Item")]
    #[serde(default)]
    pub item: std::collections::HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteItemOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics:
        Option<std::collections::HashMap<String, Vec<ItemCollectionMetrics>>>,
    #[serde(rename = "UnprocessedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_items: Option<std::collections::HashMap<String, Vec<WriteRequest>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemCollectionMetrics {
    #[serde(rename = "ItemCollectionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "SizeEstimateRangeGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate_range_g_b: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupInput {
    #[serde(rename = "BackupName")]
    #[serde(default)]
    pub backup_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupOutput {
    #[serde(rename = "BackupDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_details: Option<BackupDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupDetails {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<String>,
    #[serde(rename = "BackupCreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_date_time: Option<f64>,
    #[serde(rename = "BackupExpiryDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expiry_date_time: Option<f64>,
    #[serde(rename = "BackupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    #[serde(rename = "BackupSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    #[serde(rename = "BackupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    #[serde(rename = "BackupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    pub global_table_name: String,
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    pub replication_group: Vec<Replica>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Replica {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTableDescription {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "GlobalTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_arn: Option<String>,
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    #[serde(rename = "GlobalTableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_status: Option<String>,
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<Vec<ReplicaDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaDescription {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndexDescription>>,
    #[serde(rename = "GlobalTableSettingsReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_settings_replication_mode: Option<String>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_id: Option<String>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughputOverride>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "ReplicaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_arn: Option<String>,
    #[serde(rename = "ReplicaInaccessibleDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_inaccessible_date_time: Option<f64>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<String>,
    #[serde(rename = "ReplicaStatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_description: Option<String>,
    #[serde(rename = "ReplicaStatusPercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_percent_progress: Option<String>,
    #[serde(rename = "ReplicaTableClassSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_table_class_summary: Option<TableClassSummary>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<TableWarmThroughputDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughputOverride>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<GlobalSecondaryIndexWarmThroughputDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandThroughputOverride {
    #[serde(rename = "MaxReadRequestUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_read_request_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedThroughputOverride {
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndexWarmThroughputDescription {
    #[serde(rename = "ReadUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_units_per_second: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WriteUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_units_per_second: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableClassSummary {
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
    #[serde(rename = "TableClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableWarmThroughputDescription {
    #[serde(rename = "ReadUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_units_per_second: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WriteUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_units_per_second: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableInput {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "DeletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,
    #[serde(rename = "GlobalTableSettingsReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_settings_replication_mode: Option<String>,
    #[serde(rename = "GlobalTableSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_source_arn: Option<String>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
    #[serde(rename = "SSESpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_specification: Option<SSESpecification>,
    #[serde(rename = "StreamSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "TableClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    pub attribute_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeySchemaElement {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    pub key_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandThroughput {
    #[serde(rename = "MaxReadRequestUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_read_request_units: Option<i64>,
    #[serde(rename = "MaxWriteRequestUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_write_request_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Projection {
    #[serde(rename = "NonKeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_key_attributes: Option<Vec<String>>,
    #[serde(rename = "ProjectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    pub read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(default)]
    pub write_capacity_units: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarmThroughput {
    #[serde(rename = "ReadUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_units_per_second: Option<i64>,
    #[serde(rename = "WriteUnitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_units_per_second: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalSecondaryIndex {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    pub projection: Projection,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSESpecification {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_id: Option<String>,
    #[serde(rename = "SSEType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSpecification {
    #[serde(rename = "StreamEnabled")]
    #[serde(default)]
    pub stream_enabled: bool,
    #[serde(rename = "StreamViewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableOutput {
    #[serde(rename = "TableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableDescription {
    #[serde(rename = "ArchivalSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_summary: Option<ArchivalSummary>,
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    #[serde(rename = "BillingModeSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "DeletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndexDescription>>,
    #[serde(rename = "GlobalTableSettingsReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_settings_replication_mode: Option<String>,
    #[serde(rename = "GlobalTableVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_version: Option<String>,
    #[serde(rename = "GlobalTableWitnesses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_witnesses: Option<Vec<GlobalTableWitnessDescription>>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "LatestStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<String>,
    #[serde(rename = "LatestStreamLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_label: Option<String>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndexDescription>>,
    #[serde(rename = "MultiRegionConsistency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_consistency: Option<String>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "Replicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<ReplicaDescription>>,
    #[serde(rename = "RestoreSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_summary: Option<RestoreSummary>,
    #[serde(rename = "SSEDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_description: Option<SSEDescription>,
    #[serde(rename = "StreamSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableClassSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class_summary: Option<TableClassSummary>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
    #[serde(rename = "TableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<TableWarmThroughputDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchivalSummary {
    #[serde(rename = "ArchivalBackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_backup_arn: Option<String>,
    #[serde(rename = "ArchivalDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_date_time: Option<f64>,
    #[serde(rename = "ArchivalReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BillingModeSummary {
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "LastUpdateToPayPerRequestDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndexDescription {
    #[serde(rename = "Backfilling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<bool>,
    #[serde(rename = "IndexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<GlobalSecondaryIndexWarmThroughputDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedThroughputDescription {
    #[serde(rename = "LastDecreaseDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<f64>,
    #[serde(rename = "LastIncreaseDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<f64>,
    #[serde(rename = "NumberOfDecreasesToday")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<i64>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTableWitnessDescription {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "WitnessStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalSecondaryIndexDescription {
    #[serde(rename = "IndexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreSummary {
    #[serde(rename = "RestoreDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<f64>,
    #[serde(rename = "RestoreInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_in_progress: Option<bool>,
    #[serde(rename = "SourceBackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_arn: Option<String>,
    #[serde(rename = "SourceTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSEDescription {
    #[serde(rename = "InaccessibleEncryptionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inaccessible_encryption_date_time: Option<f64>,
    #[serde(rename = "KMSMasterKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_arn: Option<String>,
    #[serde(rename = "SSEType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupInput {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    pub backup_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupOutput {
    #[serde(rename = "BackupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_description: Option<BackupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupDescription {
    #[serde(rename = "BackupDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_details: Option<BackupDetails>,
    #[serde(rename = "SourceTableDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_details: Option<SourceTableDetails>,
    #[serde(rename = "SourceTableFeatureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_feature_details: Option<SourceTableFeatureDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceTableDetails {
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableCreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_creation_date_time: Option<f64>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceTableFeatureDetails {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndexInfo>>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndexInfo>>,
    #[serde(rename = "SSEDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_description: Option<SSEDescription>,
    #[serde(rename = "StreamDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamSpecification>,
    #[serde(rename = "TimeToLiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeToLiveDescription {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "TimeToLiveStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteItemInput {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    #[serde(rename = "Expected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<std::collections::HashMap<String, ExpectedAttributeValue>>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    #[serde(rename = "ReturnValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpectedAttributeValue {
    #[serde(rename = "AttributeValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value_list: Option<Vec<AttributeValue>>,
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "Exists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyInput {
    #[serde(rename = "ExpectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyOutput {
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableOutput {
    #[serde(rename = "TableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupInput {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    pub backup_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupOutput {
    #[serde(rename = "BackupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_description: Option<BackupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContinuousBackupsInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContinuousBackupsOutput {
    #[serde(rename = "ContinuousBackupsDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousBackupsDescription {
    #[serde(rename = "ContinuousBackupsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_status: Option<String>,
    #[serde(rename = "PointInTimeRecoveryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_description: Option<PointInTimeRecoveryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PointInTimeRecoveryDescription {
    #[serde(rename = "EarliestRestorableDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_date_time: Option<f64>,
    #[serde(rename = "LatestRestorableDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_date_time: Option<f64>,
    #[serde(rename = "PointInTimeRecoveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_status: Option<String>,
    #[serde(rename = "RecoveryPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContributorInsightsInput {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContributorInsightsOutput {
    #[serde(rename = "ContributorInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_mode: Option<String>,
    #[serde(rename = "ContributorInsightsRuleList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_rule_list: Option<Vec<String>>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_status: Option<String>,
    #[serde(rename = "FailureException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_exception: Option<FailureException>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<f64>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureException {
    #[serde(rename = "ExceptionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_description: Option<String>,
    #[serde(rename = "ExceptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointsResponse {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "CachePeriodInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_period_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportInput {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportOutput {
    #[serde(rename = "ExportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_description: Option<ExportDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportDescription {
    #[serde(rename = "BilledSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_size_bytes: Option<i64>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "ExportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    #[serde(rename = "ExportManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_manifest: Option<String>,
    #[serde(rename = "ExportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "ExportTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_time: Option<f64>,
    #[serde(rename = "ExportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_type: Option<String>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "IncrementalExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_export_specification: Option<IncrementalExportSpecification>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "S3SseAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<String>,
    #[serde(rename = "S3SseKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncrementalExportSpecification {
    #[serde(rename = "ExportFromTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_from_time: Option<f64>,
    #[serde(rename = "ExportToTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_time: Option<f64>,
    #[serde(rename = "ExportViewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    pub global_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalTableSettingsInput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    pub global_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalTableSettingsOutput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    #[serde(rename = "ReplicaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_settings: Option<Vec<ReplicaSettingsDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaSettingsDescription {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "ReplicaBillingModeSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_settings:
        Option<Vec<ReplicaGlobalSecondaryIndexSettingsDescription>>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings:
        Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_units: Option<i64>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_auto_scaling_settings:
        Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_units: Option<i64>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<String>,
    #[serde(rename = "ReplicaTableClassSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_table_class_summary: Option<TableClassSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsDescription {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_units: Option<i64>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingSettingsDescription {
    #[serde(rename = "AutoScalingDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_disabled: Option<bool>,
    #[serde(rename = "AutoScalingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role_arn: Option<String>,
    #[serde(rename = "MaximumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_units: Option<i64>,
    #[serde(rename = "MinimumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_units: Option<i64>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<AutoScalingPolicyDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicyDescription {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<AutoScalingTargetTrackingScalingPolicyConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    #[serde(rename = "DisableScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "ScaleInCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i32>,
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i32>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportInput {
    #[serde(rename = "ImportArn")]
    #[serde(default)]
    pub import_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportOutput {
    #[serde(rename = "ImportTableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_table_description: Option<ImportTableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTableDescription {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ErrorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "ImportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_arn: Option<String>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "ImportedItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_item_count: Option<i64>,
    #[serde(rename = "InputCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_compression_type: Option<String>,
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "InputFormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_options: Option<InputFormatOptions>,
    #[serde(rename = "ProcessedItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_item_count: Option<i64>,
    #[serde(rename = "ProcessedSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_size_bytes: Option<i64>,
    #[serde(rename = "S3BucketSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_source: Option<S3BucketSource>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableCreationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_creation_parameters: Option<TableCreationParameters>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputFormatOptions {
    #[serde(rename = "Csv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CsvOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsvOptions {
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "HeaderList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketSource {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableCreationParameters {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default)]
    pub attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_specification: Option<SSESpecification>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKinesisStreamingDestinationInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKinesisStreamingDestinationOutput {
    #[serde(rename = "KinesisDataStreamDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream_destinations: Option<Vec<KinesisDataStreamDestination>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisDataStreamDestination {
    #[serde(rename = "ApproximateCreationDateTimePrecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_creation_date_time_precision: Option<String>,
    #[serde(rename = "DestinationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status: Option<String>,
    #[serde(rename = "DestinationStatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status_description: Option<String>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLimitsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLimitsOutput {
    #[serde(rename = "AccountMaxReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_read_capacity_units: Option<i64>,
    #[serde(rename = "AccountMaxWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_write_capacity_units: Option<i64>,
    #[serde(rename = "TableMaxReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_max_read_capacity_units: Option<i64>,
    #[serde(rename = "TableMaxWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_max_write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableOutput {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableReplicaAutoScalingInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableReplicaAutoScalingOutput {
    #[serde(rename = "TableAutoScalingDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_auto_scaling_description: Option<TableAutoScalingDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableAutoScalingDescription {
    #[serde(rename = "Replicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<ReplicaAutoScalingDescription>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaAutoScalingDescription {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndexAutoScalingDescription>>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings:
        Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_auto_scaling_settings:
        Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingDescription {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTimeToLiveInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTimeToLiveOutput {
    #[serde(rename = "TimeToLiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementInput {
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<AttributeValue>>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "Statement")]
    #[serde(default)]
    pub statement: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<std::collections::HashMap<String, AttributeValue>>>,
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteTransactionInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "TransactStatements")]
    #[serde(default)]
    pub transact_statements: Vec<ParameterizedStatement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterizedStatement {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<AttributeValue>>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "Statement")]
    #[serde(default)]
    pub statement: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteTransactionOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<ItemResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<std::collections::HashMap<String, AttributeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTableToPointInTimeInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ExportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    #[serde(rename = "ExportTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_time: Option<f64>,
    #[serde(rename = "ExportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_type: Option<String>,
    #[serde(rename = "IncrementalExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_export_specification: Option<IncrementalExportSpecification>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "S3SseAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<String>,
    #[serde(rename = "S3SseKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<String>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTableToPointInTimeOutput {
    #[serde(rename = "ExportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_description: Option<ExportDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetItemInput {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetItemOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<std::collections::HashMap<String, AttributeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTableInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InputCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_compression_type: Option<String>,
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    pub input_format: String,
    #[serde(rename = "InputFormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_options: Option<InputFormatOptions>,
    #[serde(rename = "S3BucketSource")]
    #[serde(default)]
    pub s3_bucket_source: S3BucketSource,
    #[serde(rename = "TableCreationParameters")]
    #[serde(default)]
    pub table_creation_parameters: TableCreationParameters,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTableOutput {
    #[serde(rename = "ImportTableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_table_description: Option<ImportTableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamingDestinationInput {
    #[serde(rename = "EnableKinesisStreamingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_kinesis_streaming_configuration: Option<EnableKinesisStreamingConfiguration>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    pub stream_arn: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableKinesisStreamingConfiguration {
    #[serde(rename = "ApproximateCreationDateTimePrecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_creation_date_time_precision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamingDestinationOutput {
    #[serde(rename = "DestinationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status: Option<String>,
    #[serde(rename = "EnableKinesisStreamingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_kinesis_streaming_configuration: Option<EnableKinesisStreamingConfiguration>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupsInput {
    #[serde(rename = "BackupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    #[serde(rename = "ExclusiveStartBackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_backup_arn: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TimeRangeLowerBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_lower_bound: Option<f64>,
    #[serde(rename = "TimeRangeUpperBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_upper_bound: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupsOutput {
    #[serde(rename = "BackupSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_summaries: Option<Vec<BackupSummary>>,
    #[serde(rename = "LastEvaluatedBackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_backup_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupSummary {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<String>,
    #[serde(rename = "BackupCreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_date_time: Option<f64>,
    #[serde(rename = "BackupExpiryDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expiry_date_time: Option<f64>,
    #[serde(rename = "BackupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    #[serde(rename = "BackupSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    #[serde(rename = "BackupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    #[serde(rename = "BackupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContributorInsightsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContributorInsightsOutput {
    #[serde(rename = "ContributorInsightsSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_summaries: Option<Vec<ContributorInsightsSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContributorInsightsSummary {
    #[serde(rename = "ContributorInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_mode: Option<String>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_status: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsOutput {
    #[serde(rename = "ExportSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_summaries: Option<Vec<ExportSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSummary {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "ExportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "ExportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGlobalTablesInput {
    #[serde(rename = "ExclusiveStartGlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_global_table_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGlobalTablesOutput {
    #[serde(rename = "GlobalTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_tables: Option<Vec<GlobalTable>>,
    #[serde(rename = "LastEvaluatedGlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_global_table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTable {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<Vec<Replica>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsOutput {
    #[serde(rename = "ImportSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_summary_list: Option<Vec<ImportSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSummary {
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ImportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_arn: Option<String>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "S3BucketSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_source: Option<S3BucketSource>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesInput {
    #[serde(rename = "ExclusiveStartTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_table_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesOutput {
    #[serde(rename = "LastEvaluatedTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_table_name: Option<String>,
    #[serde(rename = "TableNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsOfResourceInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsOfResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutItemInput {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    #[serde(rename = "Expected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<std::collections::HashMap<String, ExpectedAttributeValue>>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Item")]
    #[serde(default)]
    pub item: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    #[serde(rename = "ReturnValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyInput {
    #[serde(rename = "ConfirmRemoveSelfResourceAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_remove_self_resource_access: Option<bool>,
    #[serde(rename = "ExpectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyOutput {
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryInput {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "KeyConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_condition_expression: Option<String>,
    #[serde(rename = "KeyConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_conditions: Option<std::collections::HashMap<String, Condition>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "QueryFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter: Option<std::collections::HashMap<String, Condition>>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ScanIndexForward")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_index_forward: Option<bool>,
    #[serde(rename = "Select")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "AttributeValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value_list: Option<Vec<AttributeValue>>,
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<std::collections::HashMap<String, AttributeValue>>>,
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ScannedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableFromBackupInput {
    #[serde(rename = "BackupArn")]
    #[serde(default)]
    pub backup_arn: String,
    #[serde(rename = "BillingModeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_override: Option<String>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_override: Option<Vec<GlobalSecondaryIndex>>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_index_override: Option<Vec<LocalSecondaryIndex>>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecificationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_specification_override: Option<SSESpecification>,
    #[serde(rename = "TargetTableName")]
    #[serde(default)]
    pub target_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableFromBackupOutput {
    #[serde(rename = "TableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableToPointInTimeInput {
    #[serde(rename = "BillingModeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_override: Option<String>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_override: Option<Vec<GlobalSecondaryIndex>>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_index_override: Option<Vec<LocalSecondaryIndex>>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "RestoreDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<f64>,
    #[serde(rename = "SSESpecificationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_specification_override: Option<SSESpecification>,
    #[serde(rename = "SourceTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
    #[serde(rename = "SourceTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_name: Option<String>,
    #[serde(rename = "TargetTableName")]
    #[serde(default)]
    pub target_table_name: String,
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTableToPointInTimeOutput {
    #[serde(rename = "TableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanInput {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    #[serde(rename = "ConsistentRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ScanFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_filter: Option<std::collections::HashMap<String, Condition>>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<i32>,
    #[serde(rename = "Select")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TotalSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_segments: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<std::collections::HashMap<String, AttributeValue>>>,
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ScannedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactGetItemsInput {
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "TransactItems")]
    #[serde(default)]
    pub transact_items: Vec<TransactGetItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactGetItem {
    #[serde(rename = "Get")]
    #[serde(default)]
    pub get: Get,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Get {
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactGetItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<ItemResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactWriteItemsInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    #[serde(rename = "TransactItems")]
    #[serde(default)]
    pub transact_items: Vec<TransactWriteItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactWriteItem {
    #[serde(rename = "ConditionCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_check: Option<ConditionCheck>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Delete>,
    #[serde(rename = "Put")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Put>,
    #[serde(rename = "Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionCheck {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    pub condition_expression: String,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Delete {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Put {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Item")]
    #[serde(default)]
    pub item: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Update {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "UpdateExpression")]
    #[serde(default)]
    pub update_expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactWriteItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics:
        Option<std::collections::HashMap<String, Vec<ItemCollectionMetrics>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContinuousBackupsInput {
    #[serde(rename = "PointInTimeRecoverySpecification")]
    #[serde(default)]
    pub point_in_time_recovery_specification: PointInTimeRecoverySpecification,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PointInTimeRecoverySpecification {
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    #[serde(default)]
    pub point_in_time_recovery_enabled: bool,
    #[serde(rename = "RecoveryPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContinuousBackupsOutput {
    #[serde(rename = "ContinuousBackupsDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContributorInsightsInput {
    #[serde(rename = "ContributorInsightsAction")]
    #[serde(default)]
    pub contributor_insights_action: String,
    #[serde(rename = "ContributorInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_mode: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContributorInsightsOutput {
    #[serde(rename = "ContributorInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_mode: Option<String>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_status: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    pub global_table_name: String,
    #[serde(rename = "ReplicaUpdates")]
    #[serde(default)]
    pub replica_updates: Vec<ReplicaUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaUpdate {
    #[serde(rename = "Create")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateReplicaAction>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteReplicaAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicaAction {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicaAction {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalTableSettingsInput {
    #[serde(rename = "GlobalTableBillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_billing_mode: Option<String>,
    #[serde(rename = "GlobalTableGlobalSecondaryIndexSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_global_secondary_index_settings_update:
        Option<Vec<GlobalTableGlobalSecondaryIndexSettingsUpdate>>,
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    pub global_table_name: String,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_provisioned_write_capacity_auto_scaling_settings_update:
        Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_provisioned_write_capacity_units: Option<i64>,
    #[serde(rename = "ReplicaSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_settings_update: Option<Vec<ReplicaSettingsUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTableGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingSettingsUpdate {
    #[serde(rename = "AutoScalingDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_disabled: Option<bool>,
    #[serde(rename = "AutoScalingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role_arn: Option<String>,
    #[serde(rename = "MaximumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_units: Option<i64>,
    #[serde(rename = "MinimumUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_units: Option<i64>,
    #[serde(rename = "ScalingPolicyUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy_update: Option<AutoScalingPolicyUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingPolicyUpdate {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(default)]
    pub target_tracking_scaling_policy_configuration:
        AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationUpdate {
    #[serde(rename = "DisableScaleIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "ScaleInCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i32>,
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i32>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    pub target_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaSettingsUpdate {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_settings_update:
        Option<Vec<ReplicaGlobalSecondaryIndexSettingsUpdate>>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings_update:
        Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_units: Option<i64>,
    #[serde(rename = "ReplicaTableClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_table_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettingsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_units: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalTableSettingsOutput {
    #[serde(rename = "GlobalTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    #[serde(rename = "ReplicaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_settings: Option<Vec<ReplicaSettingsDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateItemInput {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_updates: Option<std::collections::HashMap<String, AttributeValueUpdate>>,
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    #[serde(rename = "Expected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<std::collections::HashMap<String, ExpectedAttributeValue>>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    #[serde(rename = "ReturnValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "UpdateExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValueUpdate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKinesisStreamingDestinationInput {
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    pub stream_arn: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "UpdateKinesisStreamingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_kinesis_streaming_configuration: Option<UpdateKinesisStreamingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKinesisStreamingConfiguration {
    #[serde(rename = "ApproximateCreationDateTimePrecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_creation_date_time_precision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKinesisStreamingDestinationOutput {
    #[serde(rename = "DestinationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status: Option<String>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "UpdateKinesisStreamingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_kinesis_streaming_configuration: Option<UpdateKinesisStreamingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableInput {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "DeletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_updates: Option<Vec<GlobalSecondaryIndexUpdate>>,
    #[serde(rename = "GlobalTableSettingsReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_settings_replication_mode: Option<String>,
    #[serde(rename = "GlobalTableWitnessUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_witness_updates: Option<Vec<GlobalTableWitnessGroupUpdate>>,
    #[serde(rename = "MultiRegionConsistency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_consistency: Option<String>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "ReplicaUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_updates: Option<Vec<ReplicationGroupUpdate>>,
    #[serde(rename = "SSESpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_specification: Option<SSESpecification>,
    #[serde(rename = "StreamSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "TableClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndexUpdate {
    #[serde(rename = "Create")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateGlobalSecondaryIndexAction>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteGlobalSecondaryIndexAction>,
    #[serde(rename = "Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateGlobalSecondaryIndexAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "OnDemandThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput: Option<OnDemandThroughput>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTableWitnessGroupUpdate {
    #[serde(rename = "Create")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateGlobalTableWitnessGroupMemberAction>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteGlobalTableWitnessGroupMemberAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalTableWitnessGroupMemberAction {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlobalTableWitnessGroupMemberAction {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationGroupUpdate {
    #[serde(rename = "Create")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateReplicationGroupMemberAction>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteReplicationGroupMemberAction>,
    #[serde(rename = "Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateReplicationGroupMemberAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationGroupMemberAction {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndex>>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_id: Option<String>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughputOverride>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
    #[serde(rename = "TableClassOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class_override: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughputOverride>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationGroupMemberAction {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReplicationGroupMemberAction {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndex>>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_id: Option<String>,
    #[serde(rename = "OnDemandThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_throughput_override: Option<OnDemandThroughputOverride>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
    #[serde(rename = "TableClassOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class_override: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableOutput {
    #[serde(rename = "TableDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableReplicaAutoScalingInput {
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_updates: Option<Vec<GlobalSecondaryIndexAutoScalingUpdate>>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_updates: Option<Vec<ReplicaAutoScalingUpdate>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaAutoScalingUpdate {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
    #[serde(rename = "ReplicaGlobalSecondaryIndexUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_updates:
        Option<Vec<ReplicaGlobalSecondaryIndexAutoScalingUpdate>>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableReplicaAutoScalingOutput {
    #[serde(rename = "TableAutoScalingDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_auto_scaling_description: Option<TableAutoScalingDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTimeToLiveInput {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TimeToLiveSpecification")]
    #[serde(default)]
    pub time_to_live_specification: TimeToLiveSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeToLiveSpecification {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTimeToLiveOutput {
    #[serde(rename = "TimeToLiveSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
}
