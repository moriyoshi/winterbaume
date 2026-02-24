//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-glue

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreatePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionInputList")]
    #[serde(default)]
    pub partition_input_list: Vec<PartitionInput>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionInput {
    #[serde(rename = "LastAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StorageDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageDescriptor {
    #[serde(rename = "AdditionalLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_locations: Option<Vec<String>>,
    #[serde(rename = "BucketColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_columns: Option<Vec<String>>,
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "Compressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "NumberOfBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_buckets: Option<i32>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SchemaReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_reference: Option<SchemaReference>,
    #[serde(rename = "SerdeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde_info: Option<SerDeInfo>,
    #[serde(rename = "SkewedInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_info: Option<SkewedInfo>,
    #[serde(rename = "SortColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_columns: Option<Vec<Order>>,
    #[serde(rename = "StoredAsSubDirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_as_sub_directories: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Column {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaReference {
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaId {
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SerDeInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SerializationLibrary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_library: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SkewedInfo {
    #[serde(rename = "SkewedColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_names: Option<Vec<String>>,
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_value_location_maps: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SkewedColumnValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Order {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: String,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    pub sort_order: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreatePartitionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionError {
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteConnectionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionNameList")]
    #[serde(default)]
    pub connection_name_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteConnectionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, ErrorDetail>>,
    #[serde(rename = "Succeeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeletePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionsToDelete")]
    #[serde(default)]
    pub partitions_to_delete: Vec<PartitionValueList>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionValueList {
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeletePartitionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TablesToDelete")]
    #[serde(default)]
    pub tables_to_delete: Vec<String>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTableResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableError {
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTableVersionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "VersionIds")]
    #[serde(default)]
    pub version_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTableVersionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableVersionError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableVersionError {
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBlueprintsRequest {
    #[serde(rename = "IncludeBlueprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blueprint: Option<bool>,
    #[serde(rename = "IncludeParameterSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parameter_spec: Option<bool>,
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetBlueprintsResponse {
    #[serde(rename = "Blueprints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<Blueprint>>,
    #[serde(rename = "MissingBlueprints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_blueprints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Blueprint {
    #[serde(rename = "BlueprintLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_location: Option<String>,
    #[serde(rename = "BlueprintServiceLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_service_location: Option<String>,
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastActiveDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_definition: Option<LastActiveDefinition>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParameterSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_spec: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastActiveDefinition {
    #[serde(rename = "BlueprintLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_location: Option<String>,
    #[serde(rename = "BlueprintServiceLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_service_location: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "ParameterSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_spec: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCrawlersRequest {
    #[serde(rename = "CrawlerNames")]
    #[serde(default)]
    pub crawler_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCrawlersResponse {
    #[serde(rename = "Crawlers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    #[serde(rename = "CrawlersNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Crawler {
    #[serde(rename = "Classifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "CrawlElapsedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_elapsed_time: Option<i64>,
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LakeFormationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_configuration: Option<LakeFormationConfiguration>,
    #[serde(rename = "LastCrawl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_crawl: Option<LastCrawlInfo>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "LineageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_configuration: Option<LineageConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecrawlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_policy: Option<RecrawlPolicy>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TablePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LakeFormationConfiguration {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "UseLakeFormationCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_lake_formation_credentials: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastCrawlInfo {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "LogStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    #[serde(rename = "MessagePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_prefix: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineageConfiguration {
    #[serde(rename = "CrawlerLineageSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_lineage_settings: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecrawlPolicy {
    #[serde(rename = "RecrawlBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaChangePolicy {
    #[serde(rename = "DeleteBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_behavior: Option<String>,
    #[serde(rename = "UpdateBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrawlerTargets {
    #[serde(rename = "CatalogTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_targets: Option<Vec<CatalogTarget>>,
    #[serde(rename = "DeltaTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_targets: Option<Vec<DeltaTarget>>,
    #[serde(rename = "DynamoDBTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_targets: Option<Vec<DynamoDBTarget>>,
    #[serde(rename = "HudiTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hudi_targets: Option<Vec<HudiTarget>>,
    #[serde(rename = "IcebergTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_targets: Option<Vec<IcebergTarget>>,
    #[serde(rename = "JdbcTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,
    #[serde(rename = "MongoDBTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_d_b_targets: Option<Vec<MongoDBTarget>>,
    #[serde(rename = "S3Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_targets: Option<Vec<S3Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "DlqEventQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_event_queue_arn: Option<String>,
    #[serde(rename = "EventQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_queue_arn: Option<String>,
    #[serde(rename = "Tables")]
    #[serde(default)]
    pub tables: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeltaTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "CreateNativeDeltaTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_native_delta_table: Option<bool>,
    #[serde(rename = "DeltaTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_tables: Option<Vec<String>>,
    #[serde(rename = "WriteManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_manifest: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBTarget {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "scanAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_all: Option<bool>,
    #[serde(rename = "scanRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rate: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HudiTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "MaximumTraversalDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_traversal_depth: Option<i32>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "MaximumTraversalDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_traversal_depth: Option<i32>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JdbcTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "EnableAdditionalMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_additional_metadata: Option<Vec<String>>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MongoDBTarget {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ScanAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_all: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Target {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "DlqEventQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_event_queue_arn: Option<String>,
    #[serde(rename = "EventQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_queue_arn: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCustomEntityTypesRequest {
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCustomEntityTypesResponse {
    #[serde(rename = "CustomEntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_entity_types: Option<Vec<CustomEntityType>>,
    #[serde(rename = "CustomEntityTypesNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_entity_types_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomEntityType {
    #[serde(rename = "ContextWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_words: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RegexString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDataQualityResultRequest {
    #[serde(rename = "ResultIds")]
    #[serde(default)]
    pub result_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDataQualityResultResponse {
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DataQualityResult>>,
    #[serde(rename = "ResultsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityResult {
    #[serde(rename = "AggregatedMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_metrics: Option<DataQualityAggregatedMetrics>,
    #[serde(rename = "AnalyzerResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer_results: Option<Vec<DataQualityAnalyzerResult>>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "Observations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<DataQualityObservation>>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "ResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(rename = "RuleResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_results: Option<Vec<DataQualityRuleResult>>,
    #[serde(rename = "RulesetEvaluationRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_evaluation_run_id: Option<String>,
    #[serde(rename = "RulesetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_name: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityAggregatedMetrics {
    #[serde(rename = "TotalRowsFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows_failed: Option<f64>,
    #[serde(rename = "TotalRowsPassed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows_passed: Option<f64>,
    #[serde(rename = "TotalRowsProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows_processed: Option<f64>,
    #[serde(rename = "TotalRulesFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rules_failed: Option<f64>,
    #[serde(rename = "TotalRulesPassed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rules_passed: Option<f64>,
    #[serde(rename = "TotalRulesProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rules_processed: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityAnalyzerResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluatedMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "EvaluationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_message: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "DataQualityGlueTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_glue_table: Option<DataQualityGlueTable>,
    #[serde(rename = "GlueTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_table: Option<GlueTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityGlueTable {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PreProcessingQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_processing_query: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueTable {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityObservation {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MetricBasedObservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_based_observation: Option<MetricBasedObservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricBasedObservation {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_values: Option<DataQualityMetricValues>,
    #[serde(rename = "NewRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_rules: Option<Vec<String>>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityMetricValues {
    #[serde(rename = "ActualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[serde(rename = "ExpectedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<f64>,
    #[serde(rename = "LowerLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<f64>,
    #[serde(rename = "UpperLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityRuleResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluatedMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "EvaluatedRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_rule: Option<String>,
    #[serde(rename = "EvaluationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_message: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "RuleMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_metrics: Option<std::collections::HashMap<String, f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDevEndpointsRequest {
    #[serde(rename = "DevEndpointNames")]
    #[serde(default)]
    pub dev_endpoint_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDevEndpointsResponse {
    #[serde(rename = "DevEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    #[serde(rename = "DevEndpointsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DevEndpoint {
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[serde(rename = "LastUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "PrivateAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_address: Option<String>,
    #[serde(rename = "PublicAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "PublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetJobsRequest {
    #[serde(rename = "JobNames")]
    #[serde(default)]
    pub job_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetJobsResponse {
    #[serde(rename = "Jobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    #[serde(rename = "JobsNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<i32>,
    #[serde(rename = "CodeGenConfigurationNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_gen_configuration_nodes:
        Option<std::collections::HashMap<String, CodeGenConfigurationNode>>,
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "DefaultArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<String>,
    #[serde(rename = "ExecutionProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "JobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
    #[serde(rename = "JobRunQueuingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_queuing_enabled: Option<bool>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NonOverridableArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_overridable_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "ProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SourceControlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_control_details: Option<SourceControlDetails>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeGenConfigurationNode {
    #[serde(rename = "Aggregate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<Aggregate>,
    #[serde(rename = "AmazonRedshiftSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_redshift_source: Option<AmazonRedshiftSource>,
    #[serde(rename = "AmazonRedshiftTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_redshift_target: Option<AmazonRedshiftTarget>,
    #[serde(rename = "ApplyMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_mapping: Option<ApplyMapping>,
    #[serde(rename = "AthenaConnectorSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_connector_source: Option<AthenaConnectorSource>,
    #[serde(rename = "CatalogDeltaSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_delta_source: Option<CatalogDeltaSource>,
    #[serde(rename = "CatalogHudiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_hudi_source: Option<CatalogHudiSource>,
    #[serde(rename = "CatalogIcebergSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_iceberg_source: Option<CatalogIcebergSource>,
    #[serde(rename = "CatalogKafkaSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_kafka_source: Option<CatalogKafkaSource>,
    #[serde(rename = "CatalogKinesisSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_kinesis_source: Option<CatalogKinesisSource>,
    #[serde(rename = "CatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_source: Option<CatalogSource>,
    #[serde(rename = "CatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_target: Option<BasicCatalogTarget>,
    #[serde(rename = "ConnectorDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_data_source: Option<ConnectorDataSource>,
    #[serde(rename = "ConnectorDataTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_data_target: Option<ConnectorDataTarget>,
    #[serde(rename = "CustomCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_code: Option<CustomCode>,
    #[serde(rename = "DirectJDBCSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_j_d_b_c_source: Option<DirectJDBCSource>,
    #[serde(rename = "DirectKafkaSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_kafka_source: Option<DirectKafkaSource>,
    #[serde(rename = "DirectKinesisSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_kinesis_source: Option<DirectKinesisSource>,
    #[serde(rename = "DropDuplicates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_duplicates: Option<DropDuplicates>,
    #[serde(rename = "DropFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_fields: Option<DropFields>,
    #[serde(rename = "DropNullFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_null_fields: Option<DropNullFields>,
    #[serde(rename = "DynamicTransform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_transform: Option<DynamicTransform>,
    #[serde(rename = "DynamoDBCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_catalog_source: Option<DynamoDBCatalogSource>,
    #[serde(rename = "DynamoDBELTConnectorSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_e_l_t_connector_source: Option<DynamoDBELTConnectorSource>,
    #[serde(rename = "EvaluateDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_data_quality: Option<EvaluateDataQuality>,
    #[serde(rename = "EvaluateDataQualityMultiFrame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_data_quality_multi_frame: Option<EvaluateDataQualityMultiFrame>,
    #[serde(rename = "FillMissingValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_missing_values: Option<FillMissingValues>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(rename = "GovernedCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governed_catalog_source: Option<GovernedCatalogSource>,
    #[serde(rename = "GovernedCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governed_catalog_target: Option<GovernedCatalogTarget>,
    #[serde(rename = "JDBCConnectorSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_d_b_c_connector_source: Option<JDBCConnectorSource>,
    #[serde(rename = "JDBCConnectorTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_d_b_c_connector_target: Option<JDBCConnectorTarget>,
    #[serde(rename = "Join")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<Join>,
    #[serde(rename = "Merge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<Merge>,
    #[serde(rename = "MicrosoftSQLServerCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_s_q_l_server_catalog_source: Option<MicrosoftSQLServerCatalogSource>,
    #[serde(rename = "MicrosoftSQLServerCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_s_q_l_server_catalog_target: Option<MicrosoftSQLServerCatalogTarget>,
    #[serde(rename = "MySQLCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_s_q_l_catalog_source: Option<MySQLCatalogSource>,
    #[serde(rename = "MySQLCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_s_q_l_catalog_target: Option<MySQLCatalogTarget>,
    #[serde(rename = "OracleSQLCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_s_q_l_catalog_source: Option<OracleSQLCatalogSource>,
    #[serde(rename = "OracleSQLCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_s_q_l_catalog_target: Option<OracleSQLCatalogTarget>,
    #[serde(rename = "PIIDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_i_i_detection: Option<PIIDetection>,
    #[serde(rename = "PostgreSQLCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_s_q_l_catalog_source: Option<PostgreSQLCatalogSource>,
    #[serde(rename = "PostgreSQLCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_s_q_l_catalog_target: Option<PostgreSQLCatalogTarget>,
    #[serde(rename = "Recipe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<Recipe>,
    #[serde(rename = "RedshiftSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_source: Option<RedshiftSource>,
    #[serde(rename = "RedshiftTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_target: Option<RedshiftTarget>,
    #[serde(rename = "RelationalCatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_catalog_source: Option<RelationalCatalogSource>,
    #[serde(rename = "RenameField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_field: Option<RenameField>,
    #[serde(rename = "Route")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
    #[serde(rename = "S3CatalogDeltaSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_catalog_delta_source: Option<S3CatalogDeltaSource>,
    #[serde(rename = "S3CatalogHudiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_catalog_hudi_source: Option<S3CatalogHudiSource>,
    #[serde(rename = "S3CatalogIcebergSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_catalog_iceberg_source: Option<S3CatalogIcebergSource>,
    #[serde(rename = "S3CatalogSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_catalog_source: Option<S3CatalogSource>,
    #[serde(rename = "S3CatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_catalog_target: Option<S3CatalogTarget>,
    #[serde(rename = "S3CsvSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_csv_source: Option<S3CsvSource>,
    #[serde(rename = "S3DeltaCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delta_catalog_target: Option<S3DeltaCatalogTarget>,
    #[serde(rename = "S3DeltaDirectTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delta_direct_target: Option<S3DeltaDirectTarget>,
    #[serde(rename = "S3DeltaSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delta_source: Option<S3DeltaSource>,
    #[serde(rename = "S3DirectTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_direct_target: Option<S3DirectTarget>,
    #[serde(rename = "S3ExcelSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_excel_source: Option<S3ExcelSource>,
    #[serde(rename = "S3GlueParquetTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_glue_parquet_target: Option<S3GlueParquetTarget>,
    #[serde(rename = "S3HudiCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_hudi_catalog_target: Option<S3HudiCatalogTarget>,
    #[serde(rename = "S3HudiDirectTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_hudi_direct_target: Option<S3HudiDirectTarget>,
    #[serde(rename = "S3HudiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_hudi_source: Option<S3HudiSource>,
    #[serde(rename = "S3HyperDirectTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_hyper_direct_target: Option<S3HyperDirectTarget>,
    #[serde(rename = "S3IcebergCatalogTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_iceberg_catalog_target: Option<S3IcebergCatalogTarget>,
    #[serde(rename = "S3IcebergDirectTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_iceberg_direct_target: Option<S3IcebergDirectTarget>,
    #[serde(rename = "S3JsonSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_json_source: Option<S3JsonSource>,
    #[serde(rename = "S3ParquetSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_parquet_source: Option<S3ParquetSource>,
    #[serde(rename = "SelectFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<SelectFields>,
    #[serde(rename = "SelectFromCollection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_from_collection: Option<SelectFromCollection>,
    #[serde(rename = "SnowflakeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_source: Option<SnowflakeSource>,
    #[serde(rename = "SnowflakeTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_target: Option<SnowflakeTarget>,
    #[serde(rename = "SparkConnectorSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_connector_source: Option<SparkConnectorSource>,
    #[serde(rename = "SparkConnectorTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_connector_target: Option<SparkConnectorTarget>,
    #[serde(rename = "SparkSQL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_s_q_l: Option<SparkSQL>,
    #[serde(rename = "Spigot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spigot: Option<Spigot>,
    #[serde(rename = "SplitFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_fields: Option<SplitFields>,
    #[serde(rename = "Union")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union: Option<Union>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Aggregate {
    #[serde(rename = "Aggs")]
    #[serde(default)]
    pub aggs: Vec<AggregateOperation>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    pub groups: Vec<Vec<String>>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateOperation {
    #[serde(rename = "AggFunc")]
    #[serde(default)]
    pub agg_func: String,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonRedshiftSource {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AmazonRedshiftNodeData>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonRedshiftNodeData {
    #[serde(rename = "AccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<Vec<AmazonRedshiftAdvancedOption>>,
    #[serde(rename = "CatalogDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_database: Option<Option_>,
    #[serde(rename = "CatalogRedshiftSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_redshift_schema: Option<String>,
    #[serde(rename = "CatalogRedshiftTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_redshift_table: Option<String>,
    #[serde(rename = "CatalogTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_table: Option<Option_>,
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Option_>,
    #[serde(rename = "CrawlerConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_connection: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<Option_>,
    #[serde(rename = "MergeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_action: Option<String>,
    #[serde(rename = "MergeClause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_clause: Option<String>,
    #[serde(rename = "MergeWhenMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_when_matched: Option<String>,
    #[serde(rename = "MergeWhenNotMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_when_not_matched: Option<String>,
    #[serde(rename = "PostAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_action: Option<String>,
    #[serde(rename = "PreAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_action: Option<String>,
    #[serde(rename = "SampleQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_query: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Option_>,
    #[serde(rename = "SelectedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_columns: Option<Vec<Option_>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StagingTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging_table: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Option_>,
    #[serde(rename = "TablePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    #[serde(rename = "TableSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<Vec<Option_>>,
    #[serde(rename = "TempDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_dir: Option<String>,
    #[serde(rename = "Upsert")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsert: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonRedshiftAdvancedOption {
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
pub struct Option_ {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonRedshiftTarget {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AmazonRedshiftNodeData>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplyMapping {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Mapping")]
    #[serde(default)]
    pub mapping: Vec<Mapping>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(rename = "Children")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Mapping>>,
    #[serde(rename = "Dropped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<bool>,
    #[serde(rename = "FromPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_path: Option<Vec<String>>,
    #[serde(rename = "FromType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_type: Option<String>,
    #[serde(rename = "ToKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_key: Option<String>,
    #[serde(rename = "ToType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AthenaConnectorSource {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_table: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    pub schema_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueSchema {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<GlueStudioSchemaColumn>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueStudioSchemaColumn {
    #[serde(rename = "GlueStudioType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_studio_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogDeltaSource {
    #[serde(rename = "AdditionalDeltaOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_delta_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogHudiSource {
    #[serde(rename = "AdditionalHudiOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_hudi_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogIcebergSource {
    #[serde(rename = "AdditionalIcebergOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_iceberg_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogKafkaSource {
    #[serde(rename = "DataPreviewOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_preview_options: Option<StreamingDataPreviewOptions>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DetectSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_schema: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StreamingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_options: Option<KafkaStreamingSourceOptions>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
    #[serde(rename = "WindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamingDataPreviewOptions {
    #[serde(rename = "PollingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_time: Option<i64>,
    #[serde(rename = "RecordPollingLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_polling_limit: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaStreamingSourceOptions {
    #[serde(rename = "AddRecordTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_record_timestamp: Option<String>,
    #[serde(rename = "Assign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign: Option<String>,
    #[serde(rename = "BootstrapServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_servers: Option<String>,
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EmitConsumerLagMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_consumer_lag_metrics: Option<String>,
    #[serde(rename = "EndingOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_offsets: Option<String>,
    #[serde(rename = "IncludeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_headers: Option<bool>,
    #[serde(rename = "MaxOffsetsPerTrigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offsets_per_trigger: Option<i64>,
    #[serde(rename = "MinPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_partitions: Option<i32>,
    #[serde(rename = "NumRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "PollTimeoutMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_timeout_ms: Option<i64>,
    #[serde(rename = "RetryIntervalMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval_ms: Option<i64>,
    #[serde(rename = "SecurityProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_protocol: Option<String>,
    #[serde(rename = "StartingOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_offsets: Option<String>,
    #[serde(rename = "StartingTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_timestamp: Option<String>,
    #[serde(rename = "SubscribePattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_pattern: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogKinesisSource {
    #[serde(rename = "DataPreviewOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_preview_options: Option<StreamingDataPreviewOptions>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DetectSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_schema: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StreamingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_options: Option<KinesisStreamingSourceOptions>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
    #[serde(rename = "WindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamingSourceOptions {
    #[serde(rename = "AddIdleTimeBetweenReads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_idle_time_between_reads: Option<bool>,
    #[serde(rename = "AddRecordTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_record_timestamp: Option<String>,
    #[serde(rename = "AvoidEmptyBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_empty_batches: Option<bool>,
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DescribeShardInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub describe_shard_interval: Option<i64>,
    #[serde(rename = "EmitConsumerLagMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_consumer_lag_metrics: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "FanoutConsumerARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fanout_consumer_a_r_n: Option<String>,
    #[serde(rename = "IdleTimeBetweenReadsInMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time_between_reads_in_ms: Option<i64>,
    #[serde(rename = "MaxFetchRecordsPerShard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fetch_records_per_shard: Option<i64>,
    #[serde(rename = "MaxFetchTimeInMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fetch_time_in_ms: Option<i64>,
    #[serde(rename = "MaxRecordPerRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_per_read: Option<i64>,
    #[serde(rename = "MaxRetryIntervalMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_interval_ms: Option<i64>,
    #[serde(rename = "NumRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "RetryIntervalMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval_ms: Option<i64>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoleSessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_session_name: Option<String>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    #[serde(rename = "StartingTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_timestamp: Option<String>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_predicate: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasicCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorDataSource {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: std::collections::HashMap<String, String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorDataTarget {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: std::collections::HashMap<String, String>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomCode {
    #[serde(rename = "ClassName")]
    #[serde(default)]
    pub class_name: String,
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectJDBCSource {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "RedshiftTmpDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_tmp_dir: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectKafkaSource {
    #[serde(rename = "DataPreviewOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_preview_options: Option<StreamingDataPreviewOptions>,
    #[serde(rename = "DetectSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_schema: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StreamingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_options: Option<KafkaStreamingSourceOptions>,
    #[serde(rename = "WindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectKinesisSource {
    #[serde(rename = "DataPreviewOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_preview_options: Option<StreamingDataPreviewOptions>,
    #[serde(rename = "DetectSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_schema: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StreamingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_options: Option<KinesisStreamingSourceOptions>,
    #[serde(rename = "WindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DropDuplicates {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Vec<String>>>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DropFields {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DropNullFields {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NullCheckBoxList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_check_box_list: Option<NullCheckBoxList>,
    #[serde(rename = "NullTextList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_text_list: Option<Vec<NullValueField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NullCheckBoxList {
    #[serde(rename = "IsEmpty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(rename = "IsNegOne")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_neg_one: Option<bool>,
    #[serde(rename = "IsNullString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null_string: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NullValueField {
    #[serde(rename = "Datatype")]
    #[serde(default)]
    pub datatype: Datatype,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Datatype {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamicTransform {
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<TransformConfigParameter>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "TransformName")]
    #[serde(default)]
    pub transform_name: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformConfigParameter {
    #[serde(rename = "IsOptional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[serde(rename = "ListType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "ValidationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<String>,
    #[serde(rename = "ValidationRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rule: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBCatalogSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<DDBELTCatalogAdditionalOptions>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PitrEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitr_enabled: Option<bool>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DDBELTCatalogAdditionalOptions {
    #[serde(rename = "DynamodbExport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_export: Option<String>,
    #[serde(rename = "DynamodbUnnestDDBJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_unnest_d_d_b_json: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBELTConnectorSource {
    #[serde(rename = "ConnectionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_options: Option<DDBELTConnectionOptions>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DDBELTConnectionOptions {
    #[serde(rename = "DynamodbExport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_export: Option<String>,
    #[serde(rename = "DynamodbS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_s3_bucket: Option<String>,
    #[serde(rename = "DynamodbS3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_s3_bucket_owner: Option<String>,
    #[serde(rename = "DynamodbS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_s3_prefix: Option<String>,
    #[serde(rename = "DynamodbStsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_sts_role_arn: Option<String>,
    #[serde(rename = "DynamodbTableArn")]
    #[serde(default)]
    pub dynamodb_table_arn: String,
    #[serde(rename = "DynamodbUnnestDDBJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_unnest_d_d_b_json: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateDataQuality {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "PublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_options: Option<DQResultsPublishingOptions>,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    pub ruleset: String,
    #[serde(rename = "StopJobOnFailureOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_job_on_failure_options: Option<DQStopJobOnFailureOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DQResultsPublishingOptions {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<String>,
    #[serde(rename = "ResultsPublishingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_publishing_enabled: Option<bool>,
    #[serde(rename = "ResultsS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_s3_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DQStopJobOnFailureOptions {
    #[serde(rename = "StopJobOnFailureTiming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_job_on_failure_timing: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateDataQualityMultiFrame {
    #[serde(rename = "AdditionalDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data_sources: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_options: Option<DQResultsPublishingOptions>,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    pub ruleset: String,
    #[serde(rename = "StopJobOnFailureOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_job_on_failure_options: Option<DQStopJobOnFailureOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FillMissingValues {
    #[serde(rename = "FilledPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_path: Option<String>,
    #[serde(rename = "ImputedPath")]
    #[serde(default)]
    pub imputed_path: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<FilterExpression>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "LogicalOperator")]
    #[serde(default)]
    pub logical_operator: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterExpression {
    #[serde(rename = "Negated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<FilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterValue {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GovernedCatalogSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3SourceAdditionalOptions>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_predicate: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3SourceAdditionalOptions {
    #[serde(rename = "BoundedFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_files: Option<i64>,
    #[serde(rename = "BoundedSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GovernedCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<CatalogSchemaChangePolicy>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogSchemaChangePolicy {
    #[serde(rename = "EnableUpdateCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_update_catalog: Option<bool>,
    #[serde(rename = "UpdateBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JDBCConnectorSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<JDBCConnectorOptions>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_table: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Query")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JDBCConnectorOptions {
    #[serde(rename = "DataTypeMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type_mapping: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "FilterPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_predicate: Option<String>,
    #[serde(rename = "JobBookmarkKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_keys: Option<Vec<String>>,
    #[serde(rename = "JobBookmarkKeysSortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_keys_sort_order: Option<String>,
    #[serde(rename = "LowerBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<i64>,
    #[serde(rename = "NumPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_partitions: Option<i64>,
    #[serde(rename = "PartitionColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_column: Option<String>,
    #[serde(rename = "UpperBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JDBCConnectorTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionTable")]
    #[serde(default)]
    pub connection_table: String,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Join {
    #[serde(rename = "Columns")]
    #[serde(default)]
    pub columns: Vec<JoinColumn>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "JoinType")]
    #[serde(default)]
    pub join_type: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JoinColumn {
    #[serde(rename = "From")]
    #[serde(default)]
    pub from: String,
    #[serde(rename = "Keys")]
    #[serde(default)]
    pub keys: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Merge {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PrimaryKeys")]
    #[serde(default)]
    pub primary_keys: Vec<Vec<String>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MicrosoftSQLServerCatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MicrosoftSQLServerCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MySQLCatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MySQLCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OracleSQLCatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OracleSQLCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PIIDetection {
    #[serde(rename = "DetectionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_parameters: Option<String>,
    #[serde(rename = "DetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_sensitivity: Option<String>,
    #[serde(rename = "EntityTypesToDetect")]
    #[serde(default)]
    pub entity_types_to_detect: Vec<String>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "MaskValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_value: Option<String>,
    #[serde(rename = "MatchPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumLeftCharsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_left_chars_to_exclude: Option<i32>,
    #[serde(rename = "NumRightCharsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_right_chars_to_exclude: Option<i32>,
    #[serde(rename = "OutputColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_column_name: Option<String>,
    #[serde(rename = "PiiType")]
    #[serde(default)]
    pub pii_type: String,
    #[serde(rename = "RedactChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_char: Option<String>,
    #[serde(rename = "RedactText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_text: Option<String>,
    #[serde(rename = "SampleFraction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_fraction: Option<f64>,
    #[serde(rename = "ThresholdFraction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_fraction: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostgreSQLCatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostgreSQLCatalogTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recipe {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "RecipeSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_steps: Option<Vec<RecipeStep>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeReference {
    #[serde(rename = "RecipeArn")]
    #[serde(default)]
    pub recipe_arn: String,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    pub recipe_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeStep {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: RecipeAction,
    #[serde(rename = "ConditionExpressions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expressions: Option<Vec<ConditionExpression>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeAction {
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionExpression {
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: String,
    #[serde(rename = "TargetColumn")]
    #[serde(default)]
    pub target_column: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RedshiftTmpDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_tmp_dir: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
    #[serde(rename = "TmpDirIAMRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmp_dir_i_a_m_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftTarget {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RedshiftTmpDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_tmp_dir: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
    #[serde(rename = "TmpDirIAMRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmp_dir_i_a_m_role: Option<String>,
    #[serde(rename = "UpsertRedshiftOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsert_redshift_options: Option<UpsertRedshiftTargetOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpsertRedshiftTargetOptions {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "TableLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_location: Option<String>,
    #[serde(rename = "UpsertKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsert_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelationalCatalogSource {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameField {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    pub source_path: Vec<String>,
    #[serde(rename = "TargetPath")]
    #[serde(default)]
    pub target_path: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Route {
    #[serde(rename = "GroupFiltersList")]
    #[serde(default)]
    pub group_filters_list: Vec<GroupFilters>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupFilters {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<FilterExpression>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "LogicalOperator")]
    #[serde(default)]
    pub logical_operator: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CatalogDeltaSource {
    #[serde(rename = "AdditionalDeltaOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_delta_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CatalogHudiSource {
    #[serde(rename = "AdditionalHudiOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_hudi_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CatalogIcebergSource {
    #[serde(rename = "AdditionalIcebergOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_iceberg_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CatalogSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3SourceAdditionalOptions>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_predicate: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CatalogTarget {
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<CatalogSchemaChangePolicy>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoDataQuality {
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<String>,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3CsvSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "Escaper")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escaper: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "GroupFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_files: Option<String>,
    #[serde(rename = "GroupSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_size: Option<String>,
    #[serde(rename = "MaxBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_band: Option<i32>,
    #[serde(rename = "MaxFilesInBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_in_band: Option<i32>,
    #[serde(rename = "Multiline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OptimizePerformance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_performance: Option<bool>,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(rename = "QuoteChar")]
    #[serde(default)]
    pub quote_char: String,
    #[serde(rename = "Recurse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,
    #[serde(rename = "Separator")]
    #[serde(default)]
    pub separator: String,
    #[serde(rename = "SkipFirst")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_first: Option<bool>,
    #[serde(rename = "WithHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_header: Option<bool>,
    #[serde(rename = "WriteHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_header: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DirectSourceAdditionalOptions {
    #[serde(rename = "BoundedFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_files: Option<i64>,
    #[serde(rename = "BoundedSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_size: Option<i64>,
    #[serde(rename = "EnableSamplePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sample_path: Option<bool>,
    #[serde(rename = "SamplePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DeltaCatalogTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<CatalogSchemaChangePolicy>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DeltaDirectTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    pub compression: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberTargetPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_target_partitions: Option<String>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectSchemaChangePolicy {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "EnableUpdateCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_update_catalog: Option<bool>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "UpdateBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DeltaSource {
    #[serde(rename = "AdditionalDeltaOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_delta_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DirectTarget {
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberTargetPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_target_partitions: Option<String>,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ExcelSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "GroupFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_files: Option<String>,
    #[serde(rename = "GroupSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_size: Option<String>,
    #[serde(rename = "MaxBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_band: Option<i32>,
    #[serde(rename = "MaxFilesInBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_in_band: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_rows: Option<i64>,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(rename = "Recurse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,
    #[serde(rename = "SkipFooter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_footer: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3GlueParquetTarget {
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberTargetPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_target_partitions: Option<String>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3HudiCatalogTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    pub additional_options: std::collections::HashMap<String, String>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<CatalogSchemaChangePolicy>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3HudiDirectTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    pub additional_options: std::collections::HashMap<String, String>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    pub compression: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberTargetPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_target_partitions: Option<String>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3HudiSource {
    #[serde(rename = "AdditionalHudiOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_hudi_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3HyperDirectTarget {
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3IcebergCatalogTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<CatalogSchemaChangePolicy>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3IcebergDirectTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoDataQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_data_quality: Option<AutoDataQuality>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    pub compression: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberTargetPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_target_partitions: Option<String>,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Vec<String>>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<DirectSchemaChangePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3JsonSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "GroupFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_files: Option<String>,
    #[serde(rename = "GroupSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_size: Option<String>,
    #[serde(rename = "JsonPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(rename = "MaxBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_band: Option<i32>,
    #[serde(rename = "MaxFilesInBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_in_band: Option<i32>,
    #[serde(rename = "Multiline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(rename = "Recurse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ParquetSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<S3DirectSourceAdditionalOptions>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "GroupFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_files: Option<String>,
    #[serde(rename = "GroupSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_size: Option<String>,
    #[serde(rename = "MaxBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_band: Option<i32>,
    #[serde(rename = "MaxFilesInBand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_in_band: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(rename = "Recurse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectFields {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectFromCollection {
    #[serde(rename = "Index")]
    #[serde(default)]
    pub index: i32,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeSource {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: SnowflakeNodeData,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeNodeData {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoPushdown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pushdown: Option<bool>,
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Option_>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<Option_>,
    #[serde(rename = "MergeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_action: Option<String>,
    #[serde(rename = "MergeClause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_clause: Option<String>,
    #[serde(rename = "MergeWhenMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_when_matched: Option<String>,
    #[serde(rename = "MergeWhenNotMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_when_not_matched: Option<String>,
    #[serde(rename = "PostAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_action: Option<String>,
    #[serde(rename = "PreAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_action: Option<String>,
    #[serde(rename = "SampleQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_query: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "SelectedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_columns: Option<Vec<Option_>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StagingTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging_table: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "TableSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<Vec<Option_>>,
    #[serde(rename = "TempDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_dir: Option<String>,
    #[serde(rename = "Upsert")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsert: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeTarget {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: SnowflakeNodeData,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkConnectorSource {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkConnectorTarget {
    #[serde(rename = "AdditionalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkSQL {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schemas: Option<Vec<GlueSchema>>,
    #[serde(rename = "SqlAliases")]
    #[serde(default)]
    pub sql_aliases: Vec<SqlAlias>,
    #[serde(rename = "SqlQuery")]
    #[serde(default)]
    pub sql_query: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlAlias {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "From")]
    #[serde(default)]
    pub from: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Spigot {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "Prob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prob: Option<f64>,
    #[serde(rename = "Topk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topk: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplitFields {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Union {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "UnionType")]
    #[serde(default)]
    pub union_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobCommand {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PythonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "ScriptLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionsList {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionProperty {
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationProperty {
    #[serde(rename = "NotifyDelayAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_delay_after: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceControlDetails {
    #[serde(rename = "AuthStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_strategy: Option<String>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "Branch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "Folder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "LastCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "Repository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetPartitionRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionsToGet")]
    #[serde(default)]
    pub partitions_to_get: Vec<PartitionValueList>,
    #[serde(rename = "QuerySessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_session_context: Option<QuerySessionContext>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditContext {
    #[serde(rename = "AdditionalAuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_audit_context: Option<String>,
    #[serde(rename = "AllColumnsRequested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_columns_requested: Option<bool>,
    #[serde(rename = "RequestedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_columns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySessionContext {
    #[serde(rename = "AdditionalContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "QueryAuthorizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_authorization_id: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetPartitionResponse {
    #[serde(rename = "Partitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
    #[serde(rename = "UnprocessedKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<Vec<PartitionValueList>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Partition {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "LastAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StorageDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTableOptimizerRequest {
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<BatchGetTableOptimizerEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTableOptimizerEntry {
    #[serde(rename = "catalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTableOptimizerResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<BatchGetTableOptimizerError>>,
    #[serde(rename = "TableOptimizers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_optimizers: Option<Vec<BatchTableOptimizer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTableOptimizerError {
    #[serde(rename = "catalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchTableOptimizer {
    #[serde(rename = "catalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "tableOptimizer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_optimizer: Option<TableOptimizer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableOptimizer {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TableOptimizerConfiguration>,
    #[serde(rename = "configurationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_source: Option<String>,
    #[serde(rename = "lastRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<TableOptimizerRun>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableOptimizerConfiguration {
    #[serde(rename = "compactionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_configuration: Option<CompactionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "orphanFileDeletionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_file_deletion_configuration: Option<OrphanFileDeletionConfiguration>,
    #[serde(rename = "retentionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration: Option<RetentionConfiguration>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "vpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<TableOptimizerVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompactionConfiguration {
    #[serde(rename = "icebergConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_configuration: Option<IcebergCompactionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergCompactionConfiguration {
    #[serde(rename = "deleteFileThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_file_threshold: Option<i32>,
    #[serde(rename = "minInputFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_input_files: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrphanFileDeletionConfiguration {
    #[serde(rename = "icebergConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_configuration: Option<IcebergOrphanFileDeletionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergOrphanFileDeletionConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "orphanFileRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_file_retention_period_in_days: Option<i32>,
    #[serde(rename = "runRateInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_rate_in_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionConfiguration {
    #[serde(rename = "icebergConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_configuration: Option<IcebergRetentionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergRetentionConfiguration {
    #[serde(rename = "cleanExpiredFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_expired_files: Option<bool>,
    #[serde(rename = "numberOfSnapshotsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_snapshots_to_retain: Option<i32>,
    #[serde(rename = "runRateInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_rate_in_hours: Option<i32>,
    #[serde(rename = "snapshotRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableOptimizerVpcConfiguration {
    #[serde(rename = "glueConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_connection_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableOptimizerRun {
    #[serde(rename = "compactionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_metrics: Option<CompactionMetrics>,
    #[serde(rename = "compactionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_strategy: Option<String>,
    #[serde(rename = "endTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<RunMetrics>,
    #[serde(rename = "orphanFileDeletionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_file_deletion_metrics: Option<OrphanFileDeletionMetrics>,
    #[serde(rename = "retentionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_metrics: Option<RetentionMetrics>,
    #[serde(rename = "startTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompactionMetrics {
    #[serde(rename = "IcebergMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_metrics: Option<IcebergCompactionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergCompactionMetrics {
    #[serde(rename = "DpuHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_hours: Option<f64>,
    #[serde(rename = "JobDurationInHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_duration_in_hour: Option<f64>,
    #[serde(rename = "NumberOfBytesCompacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes_compacted: Option<i64>,
    #[serde(rename = "NumberOfDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_dpus: Option<i32>,
    #[serde(rename = "NumberOfFilesCompacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_files_compacted: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunMetrics {
    #[serde(rename = "JobDurationInHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_duration_in_hour: Option<String>,
    #[serde(rename = "NumberOfBytesCompacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes_compacted: Option<String>,
    #[serde(rename = "NumberOfDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_dpus: Option<String>,
    #[serde(rename = "NumberOfFilesCompacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_files_compacted: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrphanFileDeletionMetrics {
    #[serde(rename = "IcebergMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_metrics: Option<IcebergOrphanFileDeletionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergOrphanFileDeletionMetrics {
    #[serde(rename = "DpuHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_hours: Option<f64>,
    #[serde(rename = "JobDurationInHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_duration_in_hour: Option<f64>,
    #[serde(rename = "NumberOfDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_dpus: Option<i32>,
    #[serde(rename = "NumberOfOrphanFilesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_orphan_files_deleted: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionMetrics {
    #[serde(rename = "IcebergMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_metrics: Option<IcebergRetentionMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergRetentionMetrics {
    #[serde(rename = "DpuHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_hours: Option<f64>,
    #[serde(rename = "JobDurationInHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_duration_in_hour: Option<f64>,
    #[serde(rename = "NumberOfDataFilesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_data_files_deleted: Option<i64>,
    #[serde(rename = "NumberOfDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_dpus: Option<i32>,
    #[serde(rename = "NumberOfManifestFilesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_manifest_files_deleted: Option<i64>,
    #[serde(rename = "NumberOfManifestListsDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_manifest_lists_deleted: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTriggersRequest {
    #[serde(rename = "TriggerNames")]
    #[serde(default)]
    pub trigger_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTriggersResponse {
    #[serde(rename = "Triggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
    #[serde(rename = "TriggersNotFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers_not_found: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Trigger {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBatchingCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_batching_condition: Option<EventBatchingCondition>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Predicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WorkflowName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBatchingCondition {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    pub batch_size: i32,
    #[serde(rename = "BatchWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_window: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Predicate {
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "Logical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "CrawlState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_state: Option<String>,
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LogicalOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetWorkflowsRequest {
    #[serde(rename = "IncludeGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetWorkflowsResponse {
    #[serde(rename = "MissingWorkflows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_workflows: Option<Vec<String>>,
    #[serde(rename = "Workflows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<Workflow>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workflow {
    #[serde(rename = "BlueprintDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_details: Option<BlueprintDetails>,
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "DefaultRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Graph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<WorkflowGraph>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "LastRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<WorkflowRun>,
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueprintDetails {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowGraph {
    #[serde(rename = "Edges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Edge {
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "CrawlerDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_details: Option<CrawlerNodeDetails>,
    #[serde(rename = "JobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobNodeDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TriggerDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_details: Option<TriggerNodeDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UniqueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrawlerNodeDetails {
    #[serde(rename = "Crawls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawls: Option<Vec<Crawl>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Crawl {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "LogStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobNodeDetails {
    #[serde(rename = "JobRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRun {
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<i32>,
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Attempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DPUSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_p_u_seconds: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExecutionClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<String>,
    #[serde(rename = "ExecutionRoleSessionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_session_policy: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunQueuingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_queuing_enabled: Option<bool>,
    #[serde(rename = "JobRunState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_state: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "PredecessorRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessor_runs: Option<Vec<Predecessor>>,
    #[serde(rename = "PreviousRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    #[serde(rename = "ProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "StateDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_detail: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TriggerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Predecessor {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerNodeDetails {
    #[serde(rename = "Trigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowRun {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Graph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<WorkflowGraph>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PreviousRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "StartingEventBatchCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_event_batch_condition: Option<StartingEventBatchCondition>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<WorkflowRunStatistics>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WorkflowRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<String>,
    #[serde(rename = "WorkflowRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartingEventBatchCondition {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "BatchWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_window: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowRunStatistics {
    #[serde(rename = "ErroredActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored_actions: Option<i32>,
    #[serde(rename = "FailedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_actions: Option<i32>,
    #[serde(rename = "RunningActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_actions: Option<i32>,
    #[serde(rename = "StoppedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_actions: Option<i32>,
    #[serde(rename = "SucceededActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_actions: Option<i32>,
    #[serde(rename = "TimeoutActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_actions: Option<i32>,
    #[serde(rename = "TotalActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actions: Option<i32>,
    #[serde(rename = "WaitingActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_actions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutDataQualityStatisticAnnotationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InclusionAnnotations")]
    #[serde(default)]
    pub inclusion_annotations: Vec<DatapointInclusionAnnotation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatapointInclusionAnnotation {
    #[serde(rename = "InclusionAnnotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_annotation: Option<String>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutDataQualityStatisticAnnotationResponse {
    #[serde(rename = "FailedInclusionAnnotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_inclusion_annotations: Option<Vec<AnnotationError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnnotationError {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopJobRunRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "JobRunIds")]
    #[serde(default)]
    pub job_run_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopJobRunResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchStopJobRunError>>,
    #[serde(rename = "SuccessfulSubmissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_submissions: Option<Vec<BatchStopJobRunSuccessfulSubmission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopJobRunError {
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopJobRunSuccessfulSubmission {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdatePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<BatchUpdatePartitionRequestEntry>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdatePartitionRequestEntry {
    #[serde(rename = "PartitionInput")]
    #[serde(default)]
    pub partition_input: PartitionInput,
    #[serde(rename = "PartitionValueList")]
    #[serde(default)]
    pub partition_value_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdatePartitionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchUpdatePartitionFailureEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdatePartitionFailureEntry {
    #[serde(rename = "ErrorDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "PartitionValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_value_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataQualityRuleRecommendationRunRequest {
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataQualityRuleRecommendationRunResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataQualityRulesetEvaluationRunRequest {
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataQualityRulesetEvaluationRunResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMLTaskRunRequest {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    pub task_run_id: String,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMLTaskRunResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStatementRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i32,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStatementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckSchemaVersionValidityInput {
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    pub data_format: String,
    #[serde(rename = "SchemaDefinition")]
    #[serde(default)]
    pub schema_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckSchemaVersionValidityResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Valid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBlueprintRequest {
    #[serde(rename = "BlueprintLocation")]
    #[serde(default)]
    pub blueprint_location: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBlueprintResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCatalogRequest {
    #[serde(rename = "CatalogInput")]
    #[serde(default)]
    pub catalog_input: CatalogInput,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogInput {
    #[serde(rename = "AllowFullTableExternalDataAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_full_table_external_data_access: Option<String>,
    #[serde(rename = "CatalogProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_properties: Option<CatalogProperties>,
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_database_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FederatedCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_catalog: Option<FederatedCatalog>,
    #[serde(rename = "OverwriteChildResourcePermissionsWithDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_child_resource_permissions_with_default: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetRedshiftCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_redshift_catalog: Option<TargetRedshiftCatalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogProperties {
    #[serde(rename = "CustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataLakeAccessProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_access_properties: Option<DataLakeAccessProperties>,
    #[serde(rename = "IcebergOptimizationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_optimization_properties: Option<IcebergOptimizationProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLakeAccessProperties {
    #[serde(rename = "CatalogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_type: Option<String>,
    #[serde(rename = "DataLakeAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_access: Option<bool>,
    #[serde(rename = "DataTransferRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_role: Option<String>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergOptimizationProperties {
    #[serde(rename = "Compaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "OrphanFileDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_file_deletion: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrincipalPermissions {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_principal_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FederatedCatalog {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetRedshiftCatalog {
    #[serde(rename = "CatalogArn")]
    #[serde(default)]
    pub catalog_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCatalogResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClassifierRequest {
    #[serde(rename = "CsvClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<CreateCsvClassifierRequest>,
    #[serde(rename = "GrokClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<CreateGrokClassifierRequest>,
    #[serde(rename = "JsonClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<CreateJsonClassifierRequest>,
    #[serde(rename = "XMLClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_m_l_classifier: Option<CreateXMLClassifierRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCsvClassifierRequest {
    #[serde(rename = "AllowSingleColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    #[serde(rename = "ContainsHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    #[serde(rename = "CustomDatatypeConfigured")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatype_configured: Option<bool>,
    #[serde(rename = "CustomDatatypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatypes: Option<Vec<String>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisableValueTrimming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QuoteSymbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
    #[serde(rename = "Serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGrokClassifierRequest {
    #[serde(rename = "Classification")]
    #[serde(default)]
    pub classification: String,
    #[serde(rename = "CustomPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    #[serde(rename = "GrokPattern")]
    #[serde(default)]
    pub grok_pattern: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJsonClassifierRequest {
    #[serde(rename = "JsonPath")]
    #[serde(default)]
    pub json_path: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateXMLClassifierRequest {
    #[serde(rename = "Classification")]
    #[serde(default)]
    pub classification: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RowTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClassifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateColumnStatisticsTaskSettingsRequest {
    #[serde(rename = "CatalogID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_i_d: Option<String>,
    #[serde(rename = "ColumnNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name_list: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<f64>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateColumnStatisticsTaskSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionInput")]
    #[serde(default)]
    pub connection_input: ConnectionInput,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionInput {
    #[serde(rename = "AthenaProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfigurationInput>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    pub connection_properties: std::collections::HashMap<String, String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MatchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
    #[serde(rename = "PythonProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SparkProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ValidateCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_credentials: Option<bool>,
    #[serde(rename = "ValidateForComputeEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_for_compute_environments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationConfigurationInput {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "BasicAuthenticationCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_authentication_credentials: Option<BasicAuthenticationCredentials>,
    #[serde(rename = "CustomAuthenticationCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_authentication_credentials: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "OAuth2Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_properties: Option<OAuth2PropertiesInput>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasicAuthenticationCredentials {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2PropertiesInput {
    #[serde(rename = "AuthorizationCodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_properties: Option<AuthorizationCodeProperties>,
    #[serde(rename = "OAuth2ClientApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_client_application: Option<OAuth2ClientApplication>,
    #[serde(rename = "OAuth2Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_credentials: Option<OAuth2Credentials>,
    #[serde(rename = "OAuth2GrantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_grant_type: Option<String>,
    #[serde(rename = "TokenUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    #[serde(rename = "TokenUrlParametersMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_parameters_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationCodeProperties {
    #[serde(rename = "AuthorizationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[serde(rename = "RedirectUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2ClientApplication {
    #[serde(rename = "AWSManagedClientApplicationReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_managed_client_application_reference: Option<String>,
    #[serde(rename = "UserManagedClientApplicationClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_managed_client_application_client_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2Credentials {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "JwtToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "UserManagedClientApplicationClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_managed_client_application_client_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalConnectionRequirements {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "SecurityGroupIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_list: Option<Vec<String>>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionResponse {
    #[serde(rename = "CreateConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_connection_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCrawlerRequest {
    #[serde(rename = "Classifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LakeFormationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_configuration: Option<LakeFormationConfiguration>,
    #[serde(rename = "LineageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_configuration: Option<LineageConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecrawlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_policy: Option<RecrawlPolicy>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[serde(rename = "TablePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: CrawlerTargets,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCrawlerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomEntityTypeRequest {
    #[serde(rename = "ContextWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_words: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RegexString")]
    #[serde(default)]
    pub regex_string: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomEntityTypeResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataQualityRulesetRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DataQualitySecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_security_configuration: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    pub ruleset: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<DataQualityTargetTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityTargetTable {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataQualityRulesetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatabaseRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseInput")]
    #[serde(default)]
    pub database_input: DatabaseInput,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseInput {
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FederatedDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_database: Option<FederatedDatabase>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database: Option<DatabaseIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FederatedDatabase {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseIdentifier {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatabaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDevEndpointRequest {
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "PublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDevEndpointResponse {
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlueIdentityCenterConfigurationRequest {
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "UserBackgroundSessionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_sessions_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlueIdentityCenterConfigurationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationRequest {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    pub integration_name: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationConfig {
    #[serde(rename = "ContinuousSync")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_sync: Option<bool>,
    #[serde(rename = "RefreshInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<String>,
    #[serde(rename = "SourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResourcePropertyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceProcessingProperties {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetProcessingProperties {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "EventBusArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_arn: Option<String>,
    #[serde(rename = "KmsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResourcePropertyResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePropertyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_property_arn: Option<String>,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResponse {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IntegrationError>>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationTablePropertiesRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "SourceTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_config: Option<SourceTableConfig>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TargetTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table_config: Option<TargetTableConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceTableConfig {
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(rename = "FilterPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_predicate: Option<String>,
    #[serde(rename = "PrimaryKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<Vec<String>>,
    #[serde(rename = "RecordUpdateField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_update_field: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetTableConfig {
    #[serde(rename = "PartitionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_spec: Option<Vec<IntegrationPartition>>,
    #[serde(rename = "TargetTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table_name: Option<String>,
    #[serde(rename = "UnnestSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnest_spec: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationPartition {
    #[serde(rename = "ConversionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_spec: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "FunctionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_spec: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationTablePropertiesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobRequest {
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<i32>,
    #[serde(rename = "CodeGenConfigurationNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_gen_configuration_nodes:
        Option<std::collections::HashMap<String, CodeGenConfigurationNode>>,
    #[serde(rename = "Command")]
    #[serde(default)]
    pub command: JobCommand,
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[serde(rename = "DefaultArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<String>,
    #[serde(rename = "ExecutionProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "JobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
    #[serde(rename = "JobRunQueuingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_queuing_enabled: Option<bool>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NonOverridableArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_overridable_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SourceControlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_control_details: Option<SourceControlDetails>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMLTransformRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "InputRecordTables")]
    #[serde(default)]
    pub input_record_tables: Vec<GlueTable>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: TransformParameters,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TransformEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_encryption: Option<TransformEncryption>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformParameters {
    #[serde(rename = "FindMatchesParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_parameters: Option<FindMatchesParameters>,
    #[serde(rename = "TransformType")]
    #[serde(default)]
    pub transform_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindMatchesParameters {
    #[serde(rename = "AccuracyCostTradeoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_cost_tradeoff: Option<f64>,
    #[serde(rename = "EnforceProvidedLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_provided_labels: Option<bool>,
    #[serde(rename = "PrecisionRecallTradeoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision_recall_tradeoff: Option<f64>,
    #[serde(rename = "PrimaryKeyColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_column_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformEncryption {
    #[serde(rename = "MlUserDataEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_user_data_encryption: Option<MLUserDataEncryption>,
    #[serde(rename = "TaskRunSecurityConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_security_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MLUserDataEncryption {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MlUserDataEncryptionMode")]
    #[serde(default)]
    pub ml_user_data_encryption_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMLTransformResponse {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartitionIndexRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionIndex")]
    #[serde(default)]
    pub partition_index: PartitionIndex,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionIndex {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "Keys")]
    #[serde(default)]
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartitionIndexResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionInput")]
    #[serde(default)]
    pub partition_input: PartitionInput,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRegistryInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    pub registry_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRegistryResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaInput {
    #[serde(rename = "Compatibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    pub data_format: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<RegistryId>,
    #[serde(rename = "SchemaDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_definition: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    pub schema_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistryId {
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaResponse {
    #[serde(rename = "Compatibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LatestSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_schema_version: Option<i64>,
    #[serde(rename = "NextSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_schema_version: Option<i64>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaCheckpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_checkpoint: Option<i64>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "SchemaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_status: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScriptRequest {
    #[serde(rename = "DagEdges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    #[serde(rename = "DagNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
    #[serde(rename = "Language")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeGenEdge {
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
    #[serde(rename = "TargetParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeGenNode {
    #[serde(rename = "Args")]
    #[serde(default)]
    pub args: Vec<CodeGenNodeArg>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LineNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    pub node_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeGenNodeArg {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Param")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<bool>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScriptResponse {
    #[serde(rename = "PythonScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    #[serde(rename = "ScalaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationRequest {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    pub encryption_configuration: EncryptionConfiguration,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "CloudWatchEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,
    #[serde(rename = "DataQualityEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_encryption: Option<DataQualityEncryption>,
    #[serde(rename = "JobBookmarksEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,
    #[serde(rename = "S3Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption: Option<Vec<S3Encryption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchEncryption {
    #[serde(rename = "CloudWatchEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityEncryption {
    #[serde(rename = "DataQualityEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobBookmarksEncryption {
    #[serde(rename = "JobBookmarksEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Encryption {
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "S3EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSessionRequest {
    #[serde(rename = "Command")]
    #[serde(default)]
    pub command: SessionCommand,
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[serde(rename = "DefaultArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IdleTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionCommand {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PythonVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSessionResponse {
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<SessionCommand>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "DPUSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_p_u_seconds: Option<f64>,
    #[serde(rename = "DefaultArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<f64>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdleTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "ProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableOptimizerRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TableOptimizerConfiguration")]
    #[serde(default)]
    pub table_optimizer_configuration: TableOptimizerConfiguration,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableOptimizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OpenTableFormatInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_table_format_input: Option<OpenTableFormatInput>,
    #[serde(rename = "PartitionIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_indexes: Option<Vec<PartitionIndex>>,
    #[serde(rename = "TableInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_input: Option<TableInput>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenTableFormatInput {
    #[serde(rename = "IcebergInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_input: Option<IcebergInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergInput {
    #[serde(rename = "CreateIcebergTableInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_iceberg_table_input: Option<CreateIcebergTableInput>,
    #[serde(rename = "MetadataOperation")]
    #[serde(default)]
    pub metadata_operation: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIcebergTableInput {
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: String,
    #[serde(rename = "PartitionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_spec: Option<IcebergPartitionSpec>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: IcebergSchema,
    #[serde(rename = "WriteOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_order: Option<IcebergSortOrder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergPartitionSpec {
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<IcebergPartitionField>,
    #[serde(rename = "SpecId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergPartitionField {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    pub source_id: i32,
    #[serde(rename = "Transform")]
    #[serde(default)]
    pub transform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSchema {
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<IcebergStructField>,
    #[serde(rename = "IdentifierFieldIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_field_ids: Option<Vec<i32>>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergStructField {
    #[serde(rename = "Doc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i32,
    #[serde(rename = "InitialDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_default: Option<serde_json::Value>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Required")]
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: serde_json::Value,
    #[serde(rename = "WriteDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSortOrder {
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<IcebergSortField>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergSortField {
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "NullOrder")]
    #[serde(default)]
    pub null_order: String,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    pub source_id: i32,
    #[serde(rename = "Transform")]
    #[serde(default)]
    pub transform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i32>,
    #[serde(rename = "StorageDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "TableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<TableIdentifier>,
    #[serde(rename = "ViewDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_definition: Option<ViewDefinitionInput>,
    #[serde(rename = "ViewExpandedText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[serde(rename = "ViewOriginalText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableIdentifier {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewDefinitionInput {
    #[serde(rename = "Definer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definer: Option<String>,
    #[serde(rename = "IsProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(rename = "LastRefreshType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_type: Option<String>,
    #[serde(rename = "RefreshSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_seconds: Option<i64>,
    #[serde(rename = "Representations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representations: Option<Vec<ViewRepresentationInput>>,
    #[serde(rename = "SubObjectVersionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_object_version_ids: Option<Vec<i64>>,
    #[serde(rename = "SubObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_objects: Option<Vec<String>>,
    #[serde(rename = "ViewVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_version_id: Option<i64>,
    #[serde(rename = "ViewVersionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewRepresentationInput {
    #[serde(rename = "Dialect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    #[serde(rename = "DialectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect_version: Option<String>,
    #[serde(rename = "ValidationConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_connection: Option<String>,
    #[serde(rename = "ViewExpandedText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[serde(rename = "ViewOriginalText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTriggerRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBatchingCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_batching_condition: Option<EventBatchingCondition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Predicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "StartOnCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on_creation: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "WorkflowName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTriggerResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUsageProfileRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: ProfileConfiguration,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProfileConfiguration {
    #[serde(rename = "JobConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_configuration: Option<std::collections::HashMap<String, ConfigurationObject>>,
    #[serde(rename = "SessionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_configuration: Option<std::collections::HashMap<String, ConfigurationObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationObject {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUsageProfileResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserDefinedFunctionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "FunctionInput")]
    #[serde(default)]
    pub function_input: UserDefinedFunctionInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserDefinedFunctionInput {
    #[serde(rename = "ClassName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "FunctionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_type: Option<String>,
    #[serde(rename = "OwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "OwnerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    #[serde(rename = "ResourceUris")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceUri {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserDefinedFunctionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkflowRequest {
    #[serde(rename = "DefaultRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkflowResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBlueprintRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBlueprintResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCatalogRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCatalogResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClassifierRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClassifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsForPartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsForPartitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsForTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsForTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsTaskSettingsRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteColumnStatisticsTaskSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionTypeRequest {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionTypeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCrawlerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCrawlerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomEntityTypeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomEntityTypeResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataQualityRulesetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataQualityRulesetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatabaseRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatabaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDevEndpointRequest {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDevEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlueIdentityCenterConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlueIdentityCenterConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationRequest {
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    pub integration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResourcePropertyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResourcePropertyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResponse {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IntegrationError>>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationTablePropertiesRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationTablePropertiesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobResponse {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMLTransformRequest {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMLTransformResponse {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePartitionIndexRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePartitionIndexResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePartitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistryInput {
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    pub registry_id: RegistryId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistryResponse {
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "PolicyHashCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaInput {
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaResponse {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaVersionsInput {
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
    #[serde(rename = "Versions")]
    #[serde(default)]
    pub versions: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaVersionsResponse {
    #[serde(rename = "SchemaVersionErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_errors: Option<Vec<SchemaVersionErrorItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaVersionErrorItem {
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigurationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSessionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSessionResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableOptimizerRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableOptimizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableVersionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    pub version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTriggerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTriggerResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUsageProfileRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUsageProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserDefinedFunctionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserDefinedFunctionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkflowRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkflowResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionTypeRequest {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionTypeResponse {
    #[serde(rename = "AthenaConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_connection_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthConfiguration>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ComputeEnvironmentConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_configurations:
        Option<std::collections::HashMap<String, ComputeEnvironmentConfiguration>>,
    #[serde(rename = "ConnectionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_options: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "PythonConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_connection_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "RestConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_configuration: Option<RestConfiguration>,
    #[serde(rename = "SparkConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_connection_properties: Option<std::collections::HashMap<String, Property>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Property {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<AllowedValue>>,
    #[serde(rename = "DataOperationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_operation_scopes: Option<Vec<String>>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_override: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PropertyLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_location: Option<String>,
    #[serde(rename = "PropertyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_types: Option<Vec<String>>,
    #[serde(rename = "Required")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedValue {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthConfiguration {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<Property>,
    #[serde(rename = "BasicAuthenticationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_authentication_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "CustomAuthenticationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_authentication_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "OAuth2Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<Property>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "SupportedAuthenticationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_authentication_types: Option<Vec<String>>,
    #[serde(rename = "SupportedComputeEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_compute_environments: Option<Vec<String>>,
    #[serde(rename = "SupportedDataOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_data_operations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeEnvironmentConfiguration {
    #[serde(rename = "ComputeEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment: Option<String>,
    #[serde(rename = "ConnectionOptionNameOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_option_name_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConnectionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_options: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "ConnectionPropertiesRequiredOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties_required_overrides: Option<Vec<String>>,
    #[serde(rename = "ConnectionPropertyNameOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_property_name_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PhysicalConnectionPropertiesRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_properties_required: Option<bool>,
    #[serde(rename = "SupportedAuthenticationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_authentication_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestConfiguration {
    #[serde(rename = "EntityConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_configurations: Option<std::collections::HashMap<String, EntityConfiguration>>,
    #[serde(rename = "GlobalSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_source_configuration: Option<SourceConfiguration>,
    #[serde(rename = "ValidationEndpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_endpoint_configuration: Option<SourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityConfiguration {
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, FieldDefinition>>,
    #[serde(rename = "SourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<SourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldDefinition {
    #[serde(rename = "FieldDataType")]
    #[serde(default)]
    pub field_data_type: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConfiguration {
    #[serde(rename = "PaginationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_configuration: Option<PaginationConfiguration>,
    #[serde(rename = "RequestMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(rename = "RequestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<Vec<ConnectorProperty>>,
    #[serde(rename = "RequestPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,
    #[serde(rename = "ResponseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_configuration: Option<ResponseConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PaginationConfiguration {
    #[serde(rename = "CursorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_configuration: Option<CursorConfiguration>,
    #[serde(rename = "OffsetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_configuration: Option<OffsetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CursorConfiguration {
    #[serde(rename = "LimitParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_parameter: Option<ExtractedParameter>,
    #[serde(rename = "NextPage")]
    #[serde(default)]
    pub next_page: ExtractedParameter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtractedParameter {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "PropertyLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_location: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ResponseExtractionMapping>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseExtractionMapping {
    #[serde(rename = "ContentPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_path: Option<String>,
    #[serde(rename = "HeaderKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OffsetConfiguration {
    #[serde(rename = "LimitParameter")]
    #[serde(default)]
    pub limit_parameter: ExtractedParameter,
    #[serde(rename = "OffsetParameter")]
    #[serde(default)]
    pub offset_parameter: ExtractedParameter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProperty {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "KeyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_override: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PropertyLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_location: Option<String>,
    #[serde(rename = "PropertyType")]
    #[serde(default)]
    pub property_type: String,
    #[serde(rename = "Required")]
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseConfiguration {
    #[serde(rename = "ErrorPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_path: Option<String>,
    #[serde(rename = "ResultPath")]
    #[serde(default)]
    pub result_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntityRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "DataStoreApiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_store_api_version: Option<String>,
    #[serde(rename = "EntityName")]
    #[serde(default)]
    pub entity_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntityResponse {
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(rename = "CustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "FieldType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(rename = "IsCreateable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_createable: Option<bool>,
    #[serde(rename = "IsDefaultOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_on_create: Option<bool>,
    #[serde(rename = "IsFilterable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_filterable: Option<bool>,
    #[serde(rename = "IsNullable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_nullable: Option<bool>,
    #[serde(rename = "IsPartitionable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partitionable: Option<bool>,
    #[serde(rename = "IsPrimaryKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_key: Option<bool>,
    #[serde(rename = "IsRetrievable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_retrievable: Option<bool>,
    #[serde(rename = "IsUpdateable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_updateable: Option<bool>,
    #[serde(rename = "IsUpsertable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upsertable: Option<bool>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "NativeDataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_data_type: Option<String>,
    #[serde(rename = "ParentField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_field: Option<String>,
    #[serde(rename = "SupportedFilterOperators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_filter_operators: Option<Vec<String>>,
    #[serde(rename = "SupportedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInboundIntegrationsRequest {
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInboundIntegrationsResponse {
    #[serde(rename = "InboundIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_integrations: Option<Vec<InboundIntegration>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundIntegration {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IntegrationError>>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIntegrationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<IntegrationFilter>>,
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIntegrationsResponse {
    #[serde(rename = "Integrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<Integration>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Integration {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IntegrationError>>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlueprintRequest {
    #[serde(rename = "IncludeBlueprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blueprint: Option<bool>,
    #[serde(rename = "IncludeParameterSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parameter_spec: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlueprintResponse {
    #[serde(rename = "Blueprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<Blueprint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlueprintRunRequest {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    pub blueprint_name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlueprintRunResponse {
    #[serde(rename = "BlueprintRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_run: Option<BlueprintRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueprintRun {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RollbackErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_error_message: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "WorkflowName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlueprintRunsRequest {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    pub blueprint_name: String,
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
pub struct GetBlueprintRunsResponse {
    #[serde(rename = "BlueprintRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_runs: Option<Vec<BlueprintRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogImportStatusRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogImportStatusResponse {
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<CatalogImportStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogImportStatus {
    #[serde(rename = "ImportCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_completed: Option<bool>,
    #[serde(rename = "ImportTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_time: Option<f64>,
    #[serde(rename = "ImportedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogResponse {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<Catalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Catalog {
    #[serde(rename = "AllowFullTableExternalDataAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_full_table_external_data_access: Option<String>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "CatalogProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_properties: Option<CatalogPropertiesOutput>,
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_database_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FederatedCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_catalog: Option<FederatedCatalog>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "TargetRedshiftCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_redshift_catalog: Option<TargetRedshiftCatalog>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogPropertiesOutput {
    #[serde(rename = "CustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataLakeAccessProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_access_properties: Option<DataLakeAccessPropertiesOutput>,
    #[serde(rename = "IcebergOptimizationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_optimization_properties: Option<IcebergOptimizationPropertiesOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLakeAccessPropertiesOutput {
    #[serde(rename = "CatalogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_type: Option<String>,
    #[serde(rename = "DataLakeAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_access: Option<bool>,
    #[serde(rename = "DataTransferRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_role: Option<String>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "ManagedWorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_workgroup_name: Option<String>,
    #[serde(rename = "ManagedWorkgroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_workgroup_status: Option<String>,
    #[serde(rename = "RedshiftDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_database_name: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergOptimizationPropertiesOutput {
    #[serde(rename = "Compaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "OrphanFileDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_file_deletion: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogsRequest {
    #[serde(rename = "IncludeRoot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_root: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentCatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_catalog_id: Option<String>,
    #[serde(rename = "Recursive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogsResponse {
    #[serde(rename = "CatalogList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_list: Option<Vec<Catalog>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassifierRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassifierResponse {
    #[serde(rename = "Classifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<Classifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Classifier {
    #[serde(rename = "CsvClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<CsvClassifier>,
    #[serde(rename = "GrokClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<GrokClassifier>,
    #[serde(rename = "JsonClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<JsonClassifier>,
    #[serde(rename = "XMLClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_m_l_classifier: Option<XMLClassifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsvClassifier {
    #[serde(rename = "AllowSingleColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    #[serde(rename = "ContainsHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomDatatypeConfigured")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatype_configured: Option<bool>,
    #[serde(rename = "CustomDatatypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatypes: Option<Vec<String>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisableValueTrimming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QuoteSymbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
    #[serde(rename = "Serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrokClassifier {
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    #[serde(rename = "GrokPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_pattern: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonClassifier {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "JsonPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XMLClassifier {
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RowTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassifiersRequest {
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
pub struct GetClassifiersResponse {
    #[serde(rename = "Classifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<Classifier>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsForPartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    pub column_names: Vec<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsForPartitionResponse {
    #[serde(rename = "ColumnStatisticsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_list: Option<Vec<ColumnStatistics>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ColumnError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatistics {
    #[serde(rename = "AnalyzedTime")]
    #[serde(default)]
    pub analyzed_time: f64,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "ColumnType")]
    #[serde(default)]
    pub column_type: String,
    #[serde(rename = "StatisticsData")]
    #[serde(default)]
    pub statistics_data: ColumnStatisticsData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatisticsData {
    #[serde(rename = "BinaryColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_column_statistics_data: Option<BinaryColumnStatisticsData>,
    #[serde(rename = "BooleanColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_column_statistics_data: Option<BooleanColumnStatisticsData>,
    #[serde(rename = "DateColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_column_statistics_data: Option<DateColumnStatisticsData>,
    #[serde(rename = "DecimalColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_column_statistics_data: Option<DecimalColumnStatisticsData>,
    #[serde(rename = "DoubleColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_column_statistics_data: Option<DoubleColumnStatisticsData>,
    #[serde(rename = "LongColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_column_statistics_data: Option<LongColumnStatisticsData>,
    #[serde(rename = "StringColumnStatisticsData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_column_statistics_data: Option<StringColumnStatisticsData>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BinaryColumnStatisticsData {
    #[serde(rename = "AverageLength")]
    #[serde(default)]
    pub average_length: f64,
    #[serde(rename = "MaximumLength")]
    #[serde(default)]
    pub maximum_length: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BooleanColumnStatisticsData {
    #[serde(rename = "NumberOfFalses")]
    #[serde(default)]
    pub number_of_falses: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
    #[serde(rename = "NumberOfTrues")]
    #[serde(default)]
    pub number_of_trues: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateColumnStatisticsData {
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<f64>,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<f64>,
    #[serde(rename = "NumberOfDistinctValues")]
    #[serde(default)]
    pub number_of_distinct_values: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalColumnStatisticsData {
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<DecimalNumber>,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<DecimalNumber>,
    #[serde(rename = "NumberOfDistinctValues")]
    #[serde(default)]
    pub number_of_distinct_values: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalNumber {
    #[serde(rename = "Scale")]
    #[serde(default)]
    pub scale: i32,
    #[serde(rename = "UnscaledValue")]
    #[serde(default)]
    pub unscaled_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DoubleColumnStatisticsData {
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<f64>,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<f64>,
    #[serde(rename = "NumberOfDistinctValues")]
    #[serde(default)]
    pub number_of_distinct_values: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LongColumnStatisticsData {
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<i64>,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<i64>,
    #[serde(rename = "NumberOfDistinctValues")]
    #[serde(default)]
    pub number_of_distinct_values: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringColumnStatisticsData {
    #[serde(rename = "AverageLength")]
    #[serde(default)]
    pub average_length: f64,
    #[serde(rename = "MaximumLength")]
    #[serde(default)]
    pub maximum_length: i64,
    #[serde(rename = "NumberOfDistinctValues")]
    #[serde(default)]
    pub number_of_distinct_values: i64,
    #[serde(rename = "NumberOfNulls")]
    #[serde(default)]
    pub number_of_nulls: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnError {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsForTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    pub column_names: Vec<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsForTableResponse {
    #[serde(rename = "ColumnStatisticsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_list: Option<Vec<ColumnStatistics>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ColumnError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskRunRequest {
    #[serde(rename = "ColumnStatisticsTaskRunId")]
    #[serde(default)]
    pub column_statistics_task_run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskRunResponse {
    #[serde(rename = "ColumnStatisticsTaskRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_run: Option<ColumnStatisticsTaskRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatisticsTaskRun {
    #[serde(rename = "CatalogID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_i_d: Option<String>,
    #[serde(rename = "ColumnNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name_list: Option<Vec<String>>,
    #[serde(rename = "ColumnStatisticsTaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_run_id: Option<String>,
    #[serde(rename = "ComputationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computation_type: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "DPUSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_p_u_seconds: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<f64>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskRunsRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
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
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskRunsResponse {
    #[serde(rename = "ColumnStatisticsTaskRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_runs: Option<Vec<ColumnStatisticsTaskRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskSettingsRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetColumnStatisticsTaskSettingsResponse {
    #[serde(rename = "ColumnStatisticsTaskSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_settings: Option<ColumnStatisticsTaskSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatisticsTaskSettings {
    #[serde(rename = "CatalogID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_i_d: Option<String>,
    #[serde(rename = "ColumnNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name_list: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "LastExecutionAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_attempt: Option<ExecutionAttempt>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<f64>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "ScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SettingSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_source: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionAttempt {
    #[serde(rename = "ColumnStatisticsTaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_run_id: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExecutionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionRequest {
    #[serde(rename = "ApplyOverrideForComputeEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_override_for_compute_environment: Option<String>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "HidePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "AthenaProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfiguration>,
    #[serde(rename = "CompatibleComputeEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_compute_environments: Option<Vec<String>>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConnectionSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_schema_version: Option<i32>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastConnectionValidationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_validation_time: Option<f64>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "MatchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
    #[serde(rename = "PythonProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SparkProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationConfiguration {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "OAuth2Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_properties: Option<OAuth2Properties>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2Properties {
    #[serde(rename = "OAuth2ClientApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_client_application: Option<OAuth2ClientApplication>,
    #[serde(rename = "OAuth2GrantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_grant_type: Option<String>,
    #[serde(rename = "TokenUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    #[serde(rename = "TokenUrlParametersMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_parameters_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetConnectionsFilter>,
    #[serde(rename = "HidePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
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
pub struct GetConnectionsFilter {
    #[serde(rename = "ConnectionSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_schema_version: Option<i32>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "MatchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionsResponse {
    #[serde(rename = "ConnectionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_list: Option<Vec<Connection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCrawlerMetricsRequest {
    #[serde(rename = "CrawlerNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name_list: Option<Vec<String>>,
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
pub struct GetCrawlerMetricsResponse {
    #[serde(rename = "CrawlerMetricsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_metrics_list: Option<Vec<CrawlerMetrics>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrawlerMetrics {
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    #[serde(rename = "LastRuntimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_runtime_seconds: Option<f64>,
    #[serde(rename = "MedianRuntimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_runtime_seconds: Option<f64>,
    #[serde(rename = "StillEstimating")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_estimating: Option<bool>,
    #[serde(rename = "TablesCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_created: Option<i32>,
    #[serde(rename = "TablesDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_deleted: Option<i32>,
    #[serde(rename = "TablesUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_updated: Option<i32>,
    #[serde(rename = "TimeLeftSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_left_seconds: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCrawlerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCrawlerResponse {
    #[serde(rename = "Crawler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler: Option<Crawler>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCrawlersRequest {
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
pub struct GetCrawlersResponse {
    #[serde(rename = "Crawlers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomEntityTypeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomEntityTypeResponse {
    #[serde(rename = "ContextWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_words: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RegexString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCatalogEncryptionSettingsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCatalogEncryptionSettingsResponse {
    #[serde(rename = "DataCatalogEncryptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_encryption_settings: Option<DataCatalogEncryptionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCatalogEncryptionSettings {
    #[serde(rename = "ConnectionPasswordEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_password_encryption: Option<ConnectionPasswordEncryption>,
    #[serde(rename = "EncryptionAtRest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionPasswordEncryption {
    #[serde(rename = "AwsKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key_id: Option<String>,
    #[serde(rename = "ReturnConnectionPasswordEncrypted")]
    #[serde(default)]
    pub return_connection_password_encrypted: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAtRest {
    #[serde(rename = "CatalogEncryptionMode")]
    #[serde(default)]
    pub catalog_encryption_mode: String,
    #[serde(rename = "CatalogEncryptionServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_encryption_service_role: Option<String>,
    #[serde(rename = "SseAwsKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_aws_kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityModelRequest {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityModelResponse {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityModelResultRequest {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    pub statistic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityModelResultResponse {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<Vec<StatisticModelResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticModelResult {
    #[serde(rename = "ActualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "InclusionAnnotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_annotation: Option<String>,
    #[serde(rename = "LowerBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<f64>,
    #[serde(rename = "PredictedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_value: Option<f64>,
    #[serde(rename = "UpperBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityResultRequest {
    #[serde(rename = "ResultId")]
    #[serde(default)]
    pub result_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityResultResponse {
    #[serde(rename = "AggregatedMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_metrics: Option<DataQualityAggregatedMetrics>,
    #[serde(rename = "AnalyzerResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer_results: Option<Vec<DataQualityAnalyzerResult>>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "Observations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<DataQualityObservation>>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "ResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(rename = "RuleResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_results: Option<Vec<DataQualityRuleResult>>,
    #[serde(rename = "RulesetEvaluationRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_evaluation_run_id: Option<String>,
    #[serde(rename = "RulesetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_name: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRuleRecommendationRunRequest {
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRuleRecommendationRunResponse {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "CreatedRulesetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_ruleset_name: Option<String>,
    #[serde(rename = "DataQualitySecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_security_configuration: Option<String>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "ErrorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "RecommendedRuleset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_ruleset: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRulesetEvaluationRunRequest {
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRulesetEvaluationRunResponse {
    #[serde(rename = "AdditionalDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data_sources: Option<std::collections::HashMap<String, DataSource>>,
    #[serde(rename = "AdditionalRunOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_run_options: Option<DataQualityEvaluationRunAdditionalRunOptions>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "ErrorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "ResultIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_ids: Option<Vec<String>>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "RulesetNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset_names: Option<Vec<String>>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityEvaluationRunAdditionalRunOptions {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "CompositeRuleEvaluationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_rule_evaluation_method: Option<String>,
    #[serde(rename = "ResultsS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_s3_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRulesetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataQualityRulesetResponse {
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "DataQualitySecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_security_configuration: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecommendationRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_run_id: Option<String>,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<String>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<DataQualityTargetTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabaseRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabaseResponse {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Database {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FederatedDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_database: Option<FederatedDatabase>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database: Option<DatabaseIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabasesRequest {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabasesResponse {
    #[serde(rename = "DatabaseList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_list: Option<Vec<Database>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataflowGraphRequest {
    #[serde(rename = "PythonScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataflowGraphResponse {
    #[serde(rename = "DagEdges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    #[serde(rename = "DagNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDevEndpointRequest {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDevEndpointResponse {
    #[serde(rename = "DevEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoint: Option<DevEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDevEndpointsRequest {
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
pub struct GetDevEndpointsResponse {
    #[serde(rename = "DevEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEntityRecordsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataStoreApiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_store_api_version: Option<String>,
    #[serde(rename = "EntityName")]
    #[serde(default)]
    pub entity_name: String,
    #[serde(rename = "FilterPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_predicate: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "SelectedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEntityRecordsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGlueIdentityCenterConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGlueIdentityCenterConfigurationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "UserBackgroundSessionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_sessions_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResourcePropertyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResourcePropertyResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePropertyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_property_arn: Option<String>,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationTablePropertiesRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationTablePropertiesResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "SourceTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_config: Option<SourceTableConfig>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TargetTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table_config: Option<TargetTableConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobBookmarkRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobBookmarkResponse {
    #[serde(rename = "JobBookmarkEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobBookmarkEntry {
    #[serde(rename = "Attempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "JobBookmark")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "PreviousRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    #[serde(rename = "Run")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<i32>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobResponse {
    #[serde(rename = "Job")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRunRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "PredecessorsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessors_included: Option<bool>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRunResponse {
    #[serde(rename = "JobRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRunsRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
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
pub struct GetJobRunsResponse {
    #[serde(rename = "JobRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobsRequest {
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
pub struct GetJobsResponse {
    #[serde(rename = "Jobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTaskRunRequest {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    pub task_run_id: String,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTaskRunResponse {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "ErrorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunProperties>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskRunProperties {
    #[serde(rename = "ExportLabelsTaskRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_labels_task_run_properties: Option<ExportLabelsTaskRunProperties>,
    #[serde(rename = "FindMatchesTaskRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_task_run_properties: Option<FindMatchesTaskRunProperties>,
    #[serde(rename = "ImportLabelsTaskRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_labels_task_run_properties: Option<ImportLabelsTaskRunProperties>,
    #[serde(rename = "LabelingSetGenerationTaskRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_set_generation_task_run_properties: Option<LabelingSetGenerationTaskRunProperties>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportLabelsTaskRunProperties {
    #[serde(rename = "OutputS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindMatchesTaskRunProperties {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportLabelsTaskRunProperties {
    #[serde(rename = "InputS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_s3_path: Option<String>,
    #[serde(rename = "Replace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelingSetGenerationTaskRunProperties {
    #[serde(rename = "OutputS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTaskRunsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TaskRunFilterCriteria>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TaskRunSortCriteria>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskRunFilterCriteria {
    #[serde(rename = "StartedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    #[serde(rename = "StartedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskRunType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskRunSortCriteria {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: String,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    pub sort_direction: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTaskRunsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_runs: Option<Vec<TaskRun>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskRun {
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "ErrorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunProperties>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTransformRequest {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTransformResponse {
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EvaluationMetrics>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "InputRecordTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_record_tables: Option<Vec<GlueTable>>,
    #[serde(rename = "LabelCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_count: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TransformEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_encryption: Option<TransformEncryption>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationMetrics {
    #[serde(rename = "FindMatchesMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_metrics: Option<FindMatchesMetrics>,
    #[serde(rename = "TransformType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindMatchesMetrics {
    #[serde(rename = "AreaUnderPRCurve")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_under_p_r_curve: Option<f64>,
    #[serde(rename = "ColumnImportances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_importances: Option<Vec<ColumnImportance>>,
    #[serde(rename = "ConfusionMatrix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confusion_matrix: Option<ConfusionMatrix>,
    #[serde(rename = "F1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1: Option<f64>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    #[serde(rename = "Recall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnImportance {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "Importance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfusionMatrix {
    #[serde(rename = "NumFalseNegatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_false_negatives: Option<i64>,
    #[serde(rename = "NumFalsePositives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_false_positives: Option<i64>,
    #[serde(rename = "NumTrueNegatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_true_negatives: Option<i64>,
    #[serde(rename = "NumTruePositives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_true_positives: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaColumn {
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTransformsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TransformFilterCriteria>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TransformSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformFilterCriteria {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "LastModifiedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_after: Option<f64>,
    #[serde(rename = "LastModifiedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_before: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TransformType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformSortCriteria {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: String,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    pub sort_direction: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMLTransformsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Transforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<MLTransform>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MLTransform {
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EvaluationMetrics>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "InputRecordTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_record_tables: Option<Vec<GlueTable>>,
    #[serde(rename = "LabelCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_count: Option<i32>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TransformEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_encryption: Option<TransformEncryption>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMappingRequest {
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Sinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: CatalogEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "DynamoDB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b: Option<Vec<CodeGenNodeArg>>,
    #[serde(rename = "Jdbc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc: Option<Vec<CodeGenNodeArg>>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<Vec<CodeGenNodeArg>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogEntry {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMappingResponse {
    #[serde(rename = "Mapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<Vec<MappingEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappingEntry {
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(rename = "SourceTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "TargetPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaterializedViewRefreshTaskRunRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "MaterializedViewRefreshTaskRunId")]
    #[serde(default)]
    pub materialized_view_refresh_task_run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaterializedViewRefreshTaskRunResponse {
    #[serde(rename = "MaterializedViewRefreshTaskRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_view_refresh_task_run: Option<MaterializedViewRefreshTaskRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaterializedViewRefreshTaskRun {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "DPUSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_p_u_seconds: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "MaterializedViewRefreshTaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_view_refresh_task_run_id: Option<String>,
    #[serde(rename = "ProcessedBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_bytes: Option<i64>,
    #[serde(rename = "RefreshType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_type: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionIndexesRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionIndexesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartitionIndexDescriptorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_index_descriptor_list: Option<Vec<PartitionIndexDescriptor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionIndexDescriptor {
    #[serde(rename = "BackfillErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_errors: Option<Vec<BackfillError>>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeySchemaElement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackfillError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Partitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<PartitionValueList>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeySchemaElement {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionResponse {
    #[serde(rename = "Partition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionsRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "ExcludeColumnSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_column_schema: Option<bool>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryAsOfTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_as_of_time: Option<f64>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Segment {
    #[serde(rename = "SegmentNumber")]
    #[serde(default)]
    pub segment_number: i32,
    #[serde(rename = "TotalSegments")]
    #[serde(default)]
    pub total_segments: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPartitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Partitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlanRequest {
    #[serde(rename = "AdditionalPlanOptionsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_plan_options_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Language")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Mapping")]
    #[serde(default)]
    pub mapping: Vec<MappingEntry>,
    #[serde(rename = "Sinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: CatalogEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlanResponse {
    #[serde(rename = "PythonScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    #[serde(rename = "ScalaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryInput {
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    pub registry_id: RegistryId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryResponse {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesRequest {
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
pub struct GetResourcePoliciesResponse {
    #[serde(rename = "GetResourcePoliciesResponseList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_resource_policies_response_list: Option<Vec<GluePolicy>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GluePolicy {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[serde(rename = "PolicyInJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in_json: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[serde(rename = "PolicyInJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in_json: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaByDefinitionInput {
    #[serde(rename = "SchemaDefinition")]
    #[serde(default)]
    pub schema_definition: String,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaByDefinitionResponse {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaInput {
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaResponse {
    #[serde(rename = "Compatibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LatestSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_schema_version: Option<i64>,
    #[serde(rename = "NextSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_schema_version: Option<i64>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaCheckpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_checkpoint: Option<i64>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "SchemaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_status: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaVersionInput {
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<SchemaVersionNumber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaVersionNumber {
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<bool>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaVersionResponse {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_definition: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaVersionsDiffInput {
    #[serde(rename = "FirstSchemaVersionNumber")]
    #[serde(default)]
    pub first_schema_version_number: SchemaVersionNumber,
    #[serde(rename = "SchemaDiffType")]
    #[serde(default)]
    pub schema_diff_type: String,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
    #[serde(rename = "SecondSchemaVersionNumber")]
    #[serde(default)]
    pub second_schema_version_number: SchemaVersionNumber,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaVersionsDiffResponse {
    #[serde(rename = "Diff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityConfigurationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityConfigurationResponse {
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<SecurityConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfiguration {
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityConfigurationsRequest {
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
pub struct GetSecurityConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionResponse {
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i32,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementResponse {
    #[serde(rename = "Statement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<Statement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Statement {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<i64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<StatementOutput>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<i64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatementOutput {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StatementOutputData>,
    #[serde(rename = "ErrorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(rename = "ErrorValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_value: Option<String>,
    #[serde(rename = "ExecutionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Traceback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traceback: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatementOutputData {
    #[serde(rename = "TextPlain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_plain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableOptimizerRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableOptimizerResponse {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableOptimizer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_optimizer: Option<TableOptimizer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "IncludeStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status_details: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QueryAsOfTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_as_of_time: Option<f64>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableResponse {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FederatedTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_table: Option<FederatedTable>,
    #[serde(rename = "IsMaterializedView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_materialized_view: Option<bool>,
    #[serde(rename = "IsMultiDialectView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_dialect_view: Option<bool>,
    #[serde(rename = "IsRegisteredWithLakeFormation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registered_with_lake_formation: Option<bool>,
    #[serde(rename = "LastAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<TableStatus>>,
    #[serde(rename = "StorageDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "TableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<TableIdentifier>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "ViewDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_definition: Option<ViewDefinition>,
    #[serde(rename = "ViewExpandedText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[serde(rename = "ViewOriginalText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FederatedTable {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "DatabaseIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_identifier: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableStatus {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<StatusDetails>>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "RequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<f64>,
    #[serde(rename = "RequestedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusDetails {
    #[serde(rename = "RequestedChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_change: Option<Box<Table>>,
    #[serde(rename = "ViewValidations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_validations: Option<Vec<ViewValidation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewValidation {
    #[serde(rename = "Dialect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    #[serde(rename = "DialectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect_version: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "ViewValidationText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_validation_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewDefinition {
    #[serde(rename = "Definer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definer: Option<String>,
    #[serde(rename = "IsProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(rename = "LastRefreshType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_type: Option<String>,
    #[serde(rename = "RefreshSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_seconds: Option<i64>,
    #[serde(rename = "Representations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representations: Option<Vec<ViewRepresentation>>,
    #[serde(rename = "SubObjectVersionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_object_version_ids: Option<Vec<i64>>,
    #[serde(rename = "SubObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_objects: Option<Vec<String>>,
    #[serde(rename = "ViewVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_version_id: Option<i64>,
    #[serde(rename = "ViewVersionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_version_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewRepresentation {
    #[serde(rename = "Dialect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    #[serde(rename = "DialectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect_version: Option<String>,
    #[serde(rename = "IsStale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_stale: Option<bool>,
    #[serde(rename = "ValidationConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_connection: Option<String>,
    #[serde(rename = "ViewExpandedText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[serde(rename = "ViewOriginalText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableVersionRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableVersionResponse {
    #[serde(rename = "TableVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_version: Option<TableVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableVersion {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableVersionsRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
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
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_versions: Option<Vec<TableVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTablesRequest {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "IncludeStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status_details: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryAsOfTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_as_of_time: Option<f64>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTablesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTriggerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTriggerResponse {
    #[serde(rename = "Trigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTriggersRequest {
    #[serde(rename = "DependentJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_job_name: Option<String>,
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
pub struct GetTriggersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Triggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredPartitionMetadataRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "QuerySessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_session_context: Option<QuerySessionContext>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SupportedPermissionTypes")]
    #[serde(default)]
    pub supported_permission_types: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredPartitionMetadataResponse {
    #[serde(rename = "AuthorizedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_columns: Option<Vec<String>>,
    #[serde(rename = "IsRegisteredWithLakeFormation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registered_with_lake_formation: Option<bool>,
    #[serde(rename = "Partition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredPartitionsMetadataRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuerySessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_session_context: Option<QuerySessionContext>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "SupportedPermissionTypes")]
    #[serde(default)]
    pub supported_permission_types: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredPartitionsMetadataResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UnfilteredPartitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfiltered_partitions: Option<Vec<UnfilteredPartition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnfilteredPartition {
    #[serde(rename = "AuthorizedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_columns: Option<Vec<String>>,
    #[serde(rename = "IsRegisteredWithLakeFormation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registered_with_lake_formation: Option<bool>,
    #[serde(rename = "Partition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredTableMetadataRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParentResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_resource_arn: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "QuerySessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_session_context: Option<QuerySessionContext>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RootResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_resource_arn: Option<String>,
    #[serde(rename = "SupportedDialect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_dialect: Option<SupportedDialect>,
    #[serde(rename = "SupportedPermissionTypes")]
    #[serde(default)]
    pub supported_permission_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedDialect {
    #[serde(rename = "Dialect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    #[serde(rename = "DialectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUnfilteredTableMetadataResponse {
    #[serde(rename = "AuthorizedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_columns: Option<Vec<String>>,
    #[serde(rename = "CellFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_filters: Option<Vec<ColumnRowFilter>>,
    #[serde(rename = "IsMaterializedView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_materialized_view: Option<bool>,
    #[serde(rename = "IsMultiDialectView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_dialect_view: Option<bool>,
    #[serde(rename = "IsProtected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(rename = "IsRegisteredWithLakeFormation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registered_with_lake_formation: Option<bool>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "QueryAuthorizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_authorization_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RowFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_filter: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnRowFilter {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "RowFilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_filter_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageProfileRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageProfileResponse {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ProfileConfiguration>,
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserDefinedFunctionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserDefinedFunctionResponse {
    #[serde(rename = "UserDefinedFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function: Option<UserDefinedFunction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserDefinedFunction {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ClassName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "FunctionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_type: Option<String>,
    #[serde(rename = "OwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "OwnerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    #[serde(rename = "ResourceUris")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserDefinedFunctionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "FunctionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    pub pattern: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserDefinedFunctionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserDefinedFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_functions: Option<Vec<UserDefinedFunction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRequest {
    #[serde(rename = "IncludeGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowResponse {
    #[serde(rename = "Workflow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunPropertiesRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunPropertiesResponse {
    #[serde(rename = "RunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunRequest {
    #[serde(rename = "IncludeGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunResponse {
    #[serde(rename = "Run")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<WorkflowRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunsRequest {
    #[serde(rename = "IncludeGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowRunsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Runs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<WorkflowRun>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCatalogToGlueRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCatalogToGlueResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBlueprintsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBlueprintsResponse {
    #[serde(rename = "Blueprints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListColumnStatisticsTaskRunsRequest {
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
pub struct ListColumnStatisticsTaskRunsResponse {
    #[serde(rename = "ColumnStatisticsTaskRunIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_run_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectionTypesRequest {
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
pub struct ListConnectionTypesResponse {
    #[serde(rename = "ConnectionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_types: Option<Vec<ConnectionTypeBrief>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionTypeBrief {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "ConnectionTypeVariants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type_variants: Option<Vec<ConnectionTypeVariant>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "LogoUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionTypeVariant {
    #[serde(rename = "ConnectionTypeVariantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type_variant_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "LogoUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCrawlersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCrawlersResponse {
    #[serde(rename = "CrawlerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCrawlsRequest {
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    pub crawler_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CrawlsFilter>>,
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
pub struct CrawlsFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "FilterOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCrawlsResponse {
    #[serde(rename = "Crawls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawls: Option<Vec<CrawlerHistory>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrawlerHistory {
    #[serde(rename = "CrawlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_id: Option<String>,
    #[serde(rename = "DPUHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_p_u_hour: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "LogStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    #[serde(rename = "MessagePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_prefix: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomEntityTypesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomEntityTypesResponse {
    #[serde(rename = "CustomEntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_entity_types: Option<Vec<CustomEntityType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityResultsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DataQualityResultFilterCriteria>,
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
pub struct DataQualityResultFilterCriteria {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "StartedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    #[serde(rename = "StartedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityResultsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DataQualityResultDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityResultDescription {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "ResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRuleRecommendationRunsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DataQualityRuleRecommendationRunFilter>,
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
pub struct DataQualityRuleRecommendationRunFilter {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "StartedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    #[serde(rename = "StartedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRuleRecommendationRunsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Runs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<DataQualityRuleRecommendationRunDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityRuleRecommendationRunDescription {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRulesetEvaluationRunsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DataQualityRulesetEvaluationRunFilter>,
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
pub struct DataQualityRulesetEvaluationRunFilter {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "StartedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    #[serde(rename = "StartedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRulesetEvaluationRunsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Runs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<DataQualityRulesetEvaluationRunDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityRulesetEvaluationRunDescription {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRulesetsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DataQualityRulesetFilterCriteria>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityRulesetFilterCriteria {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_after: Option<f64>,
    #[serde(rename = "LastModifiedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_before: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<DataQualityTargetTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityRulesetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Rulesets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<Vec<DataQualityRulesetListDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQualityRulesetListDetails {
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecommendationRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_run_id: Option<String>,
    #[serde(rename = "RuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_count: Option<i32>,
    #[serde(rename = "TargetTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<DataQualityTargetTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityStatisticAnnotationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
    #[serde(rename = "TimestampFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_filter: Option<TimestampFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestampFilter {
    #[serde(rename = "RecordedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_after: Option<f64>,
    #[serde(rename = "RecordedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityStatisticAnnotationsResponse {
    #[serde(rename = "Annotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<StatisticAnnotation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticAnnotation {
    #[serde(rename = "InclusionAnnotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_annotation: Option<TimestampedInclusionAnnotation>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
    #[serde(rename = "StatisticRecordedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_recorded_on: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestampedInclusionAnnotation {
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityStatisticsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
    #[serde(rename = "TimestampFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_filter: Option<TimestampFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataQualityStatisticsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<StatisticSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticSummary {
    #[serde(rename = "ColumnsReferenced")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_referenced: Option<Vec<String>>,
    #[serde(rename = "DoubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "EvaluationLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_level: Option<String>,
    #[serde(rename = "InclusionAnnotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_annotation: Option<TimestampedInclusionAnnotation>,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "RecordedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<f64>,
    #[serde(rename = "ReferencedDatasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_datasets: Option<Vec<String>>,
    #[serde(rename = "RunIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_identifier: Option<RunIdentifier>,
    #[serde(rename = "StatisticId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_id: Option<String>,
    #[serde(rename = "StatisticName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_name: Option<String>,
    #[serde(rename = "StatisticProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunIdentifier {
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevEndpointsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevEndpointsResponse {
    #[serde(rename = "DevEndpointNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoint_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitiesRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "DataStoreApiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_store_api_version: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentEntityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_entity_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitiesResponse {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EntityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "IsParentEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent_entity: Option<bool>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntegrationResourcePropertiesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<IntegrationResourcePropertyFilter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationResourcePropertyFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntegrationResourcePropertiesResponse {
    #[serde(rename = "IntegrationResourcePropertyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_resource_property_list: Option<Vec<IntegrationResourceProperty>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationResourceProperty {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePropertyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_property_arn: Option<String>,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsResponse {
    #[serde(rename = "JobNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMLTransformsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TransformFilterCriteria>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TransformSortCriteria>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMLTransformsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransformIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMaterializedViewRefreshTaskRunsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
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
pub struct ListMaterializedViewRefreshTaskRunsResponse {
    #[serde(rename = "MaterializedViewRefreshTaskRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_view_refresh_task_runs: Option<Vec<MaterializedViewRefreshTaskRun>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegistriesInput {
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
pub struct ListRegistriesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Registries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<Vec<RegistryListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistryListItem {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemaVersionsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemaVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<SchemaVersionListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaVersionListItem {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemasInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<RegistryId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemasResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<SchemaListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaListItem {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "SchemaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_status: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsResponse {
    #[serde(rename = "Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Session>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStatementsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStatementsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Statements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableOptimizerRunsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
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
    pub table_name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableOptimizerRunsResponse {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableOptimizerRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_optimizer_runs: Option<Vec<TableOptimizerRun>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTriggersRequest {
    #[serde(rename = "DependentJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_job_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTriggersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TriggerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsageProfilesRequest {
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
pub struct ListUsageProfilesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Profiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<UsageProfileDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageProfileDefinition {
    #[serde(rename = "CreatedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkflowsRequest {
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
pub struct ListWorkflowsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Workflows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyIntegrationRequest {
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    pub integration_identifier: String,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyIntegrationResponse {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IntegrationError>>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<IntegrationConfig>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataCatalogEncryptionSettingsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DataCatalogEncryptionSettings")]
    #[serde(default)]
    pub data_catalog_encryption_settings: DataCatalogEncryptionSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataCatalogEncryptionSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataQualityProfileAnnotationRequest {
    #[serde(rename = "InclusionAnnotation")]
    #[serde(default)]
    pub inclusion_annotation: String,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataQualityProfileAnnotationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "EnableHybrid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_hybrid: Option<String>,
    #[serde(rename = "PolicyExistsCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_exists_condition: Option<String>,
    #[serde(rename = "PolicyHashCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
    #[serde(rename = "PolicyInJson")]
    #[serde(default)]
    pub policy_in_json: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSchemaVersionMetadataInput {
    #[serde(rename = "MetadataKeyValue")]
    #[serde(default)]
    pub metadata_key_value: MetadataKeyValuePair,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<SchemaVersionNumber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataKeyValuePair {
    #[serde(rename = "MetadataKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,
    #[serde(rename = "MetadataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSchemaVersionMetadataResponse {
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<bool>,
    #[serde(rename = "MetadataKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,
    #[serde(rename = "MetadataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_value: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutWorkflowRunPropertiesRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "RunProperties")]
    #[serde(default)]
    pub run_properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutWorkflowRunPropertiesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySchemaVersionMetadataInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_list: Option<Vec<MetadataKeyValuePair>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<SchemaVersionNumber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySchemaVersionMetadataResponse {
    #[serde(rename = "MetadataInfoMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_info_map: Option<std::collections::HashMap<String, MetadataInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataInfo {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "MetadataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_value: Option<String>,
    #[serde(rename = "OtherMetadataValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_metadata_value_list: Option<Vec<OtherMetadataValueListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OtherMetadataValueListItem {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "MetadataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectionTypeRequest {
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    pub connection_properties: ConnectionPropertiesConfiguration,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "ConnectorAuthenticationConfiguration")]
    #[serde(default)]
    pub connector_authentication_configuration: ConnectorAuthenticationConfiguration,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationType")]
    #[serde(default)]
    pub integration_type: String,
    #[serde(rename = "RestConfiguration")]
    #[serde(default)]
    pub rest_configuration: RestConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionPropertiesConfiguration {
    #[serde(rename = "AdditionalRequestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_request_parameters: Option<Vec<ConnectorProperty>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<ConnectorProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorAuthenticationConfiguration {
    #[serde(rename = "AuthenticationTypes")]
    #[serde(default)]
    pub authentication_types: Vec<String>,
    #[serde(rename = "BasicAuthenticationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_authentication_properties: Option<BasicAuthenticationProperties>,
    #[serde(rename = "CustomAuthenticationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_authentication_properties: Option<CustomAuthenticationProperties>,
    #[serde(rename = "OAuth2Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_properties: Option<ConnectorOAuth2Properties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasicAuthenticationProperties {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<ConnectorProperty>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<ConnectorProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAuthenticationProperties {
    #[serde(rename = "AuthenticationParameters")]
    #[serde(default)]
    pub authentication_parameters: Vec<ConnectorProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorOAuth2Properties {
    #[serde(rename = "AuthorizationCodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_properties: Option<ConnectorAuthorizationCodeProperties>,
    #[serde(rename = "ClientCredentialsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_properties: Option<ClientCredentialsProperties>,
    #[serde(rename = "JWTBearerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_w_t_bearer_properties: Option<JWTBearerProperties>,
    #[serde(rename = "OAuth2GrantType")]
    #[serde(default)]
    pub o_auth2_grant_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorAuthorizationCodeProperties {
    #[serde(rename = "AuthorizationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<ConnectorProperty>,
    #[serde(rename = "AuthorizationCodeUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_url: Option<ConnectorProperty>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ConnectorProperty>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ConnectorProperty>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Prompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ConnectorProperty>,
    #[serde(rename = "RedirectUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<ConnectorProperty>,
    #[serde(rename = "RequestMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ConnectorProperty>,
    #[serde(rename = "TokenUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<ConnectorProperty>,
    #[serde(rename = "TokenUrlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_parameters: Option<Vec<ConnectorProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCredentialsProperties {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ConnectorProperty>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ConnectorProperty>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "RequestMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ConnectorProperty>,
    #[serde(rename = "TokenUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<ConnectorProperty>,
    #[serde(rename = "TokenUrlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_parameters: Option<Vec<ConnectorProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JWTBearerProperties {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "JwtToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<ConnectorProperty>,
    #[serde(rename = "RequestMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(rename = "TokenUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<ConnectorProperty>,
    #[serde(rename = "TokenUrlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_parameters: Option<Vec<ConnectorProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectionTypeResponse {
    #[serde(rename = "ConnectionTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterSchemaVersionInput {
    #[serde(rename = "SchemaDefinition")]
    #[serde(default)]
    pub schema_definition: String,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterSchemaVersionResponse {
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveSchemaVersionMetadataInput {
    #[serde(rename = "MetadataKeyValue")]
    #[serde(default)]
    pub metadata_key_value: MetadataKeyValuePair,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<SchemaVersionNumber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveSchemaVersionMetadataResponse {
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<bool>,
    #[serde(rename = "MetadataKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,
    #[serde(rename = "MetadataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_value: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetJobBookmarkRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetJobBookmarkResponse {
    #[serde(rename = "JobBookmarkEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeWorkflowRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NodeIds")]
    #[serde(default)]
    pub node_ids: Vec<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeWorkflowRunResponse {
    #[serde(rename = "NodeIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<String>>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunStatementRequest {
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: String,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunStatementResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTablesRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PropertyPredicate>>,
    #[serde(rename = "IncludeStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status_details: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_type: Option<String>,
    #[serde(rename = "SearchText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[serde(rename = "SortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PropertyPredicate {
    #[serde(rename = "Comparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
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
pub struct SortCriterion {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTablesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBlueprintRunRequest {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    pub blueprint_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBlueprintRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartColumnStatisticsTaskRunRequest {
    #[serde(rename = "CatalogID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_i_d: Option<String>,
    #[serde(rename = "ColumnNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name_list: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<f64>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartColumnStatisticsTaskRunResponse {
    #[serde(rename = "ColumnStatisticsTaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartColumnStatisticsTaskRunScheduleRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartColumnStatisticsTaskRunScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCrawlerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCrawlerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCrawlerScheduleRequest {
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    pub crawler_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCrawlerScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataQualityRuleRecommendationRunRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CreatedRulesetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_ruleset_name: Option<String>,
    #[serde(rename = "DataQualitySecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_security_configuration: Option<String>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataQualityRuleRecommendationRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataQualityRulesetEvaluationRunRequest {
    #[serde(rename = "AdditionalDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data_sources: Option<std::collections::HashMap<String, DataSource>>,
    #[serde(rename = "AdditionalRunOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_run_options: Option<DataQualityEvaluationRunAdditionalRunOptions>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "RulesetNames")]
    #[serde(default)]
    pub ruleset_names: Vec<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataQualityRulesetEvaluationRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExportLabelsTaskRunRequest {
    #[serde(rename = "OutputS3Path")]
    #[serde(default)]
    pub output_s3_path: String,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExportLabelsTaskRunResponse {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportLabelsTaskRunRequest {
    #[serde(rename = "InputS3Path")]
    #[serde(default)]
    pub input_s3_path: String,
    #[serde(rename = "ReplaceAllLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_all_labels: Option<bool>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportLabelsTaskRunResponse {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunRequest {
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<i32>,
    #[serde(rename = "Arguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExecutionClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<String>,
    #[serde(rename = "ExecutionRoleSessionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_session_policy: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    #[serde(rename = "JobRunQueuingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_queuing_enabled: Option<bool>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunResponse {
    #[serde(rename = "JobRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMLEvaluationTaskRunRequest {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMLEvaluationTaskRunResponse {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMLLabelingSetGenerationTaskRunRequest {
    #[serde(rename = "OutputS3Path")]
    #[serde(default)]
    pub output_s3_path: String,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMLLabelingSetGenerationTaskRunResponse {
    #[serde(rename = "TaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMaterializedViewRefreshTaskRunRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "FullRefresh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refresh: Option<bool>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMaterializedViewRefreshTaskRunResponse {
    #[serde(rename = "MaterializedViewRefreshTaskRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_view_refresh_task_run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTriggerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTriggerResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkflowRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkflowRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopColumnStatisticsTaskRunRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopColumnStatisticsTaskRunResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopColumnStatisticsTaskRunScheduleRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopColumnStatisticsTaskRunScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCrawlerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCrawlerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCrawlerScheduleRequest {
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    pub crawler_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCrawlerScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMaterializedViewRefreshTaskRunRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMaterializedViewRefreshTaskRunResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSessionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "RequestOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSessionResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTriggerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTriggerResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkflowRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkflowRunResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagsToAdd")]
    #[serde(default)]
    pub tags_to_add: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "TestConnectionInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_connection_input: Option<TestConnectionInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionInput {
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfigurationInput>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    pub connection_properties: std::collections::HashMap<String, String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagsToRemove")]
    #[serde(default)]
    pub tags_to_remove: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBlueprintRequest {
    #[serde(rename = "BlueprintLocation")]
    #[serde(default)]
    pub blueprint_location: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBlueprintResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCatalogRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "CatalogInput")]
    #[serde(default)]
    pub catalog_input: CatalogInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCatalogResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassifierRequest {
    #[serde(rename = "CsvClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<UpdateCsvClassifierRequest>,
    #[serde(rename = "GrokClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<UpdateGrokClassifierRequest>,
    #[serde(rename = "JsonClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<UpdateJsonClassifierRequest>,
    #[serde(rename = "XMLClassifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_m_l_classifier: Option<UpdateXMLClassifierRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCsvClassifierRequest {
    #[serde(rename = "AllowSingleColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    #[serde(rename = "ContainsHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    #[serde(rename = "CustomDatatypeConfigured")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatype_configured: Option<bool>,
    #[serde(rename = "CustomDatatypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_datatypes: Option<Vec<String>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisableValueTrimming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QuoteSymbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
    #[serde(rename = "Serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGrokClassifierRequest {
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "CustomPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    #[serde(rename = "GrokPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_pattern: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJsonClassifierRequest {
    #[serde(rename = "JsonPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateXMLClassifierRequest {
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RowTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsForPartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnStatisticsList")]
    #[serde(default)]
    pub column_statistics_list: Vec<ColumnStatistics>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    pub partition_values: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsForPartitionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ColumnStatisticsError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatisticsError {
    #[serde(rename = "ColumnStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics: Option<ColumnStatistics>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsForTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnStatisticsList")]
    #[serde(default)]
    pub column_statistics_list: Vec<ColumnStatistics>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsForTableResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ColumnStatisticsError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsTaskSettingsRequest {
    #[serde(rename = "CatalogID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_i_d: Option<String>,
    #[serde(rename = "ColumnNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name_list: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<f64>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateColumnStatisticsTaskSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ConnectionInput")]
    #[serde(default)]
    pub connection_input: ConnectionInput,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCrawlerRequest {
    #[serde(rename = "Classifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LakeFormationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_configuration: Option<LakeFormationConfiguration>,
    #[serde(rename = "LineageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_configuration: Option<LineageConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecrawlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_policy: Option<RecrawlPolicy>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[serde(rename = "TablePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCrawlerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCrawlerScheduleRequest {
    #[serde(rename = "CrawlerName")]
    #[serde(default)]
    pub crawler_name: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCrawlerScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataQualityRulesetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataQualityRulesetResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Ruleset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatabaseRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseInput")]
    #[serde(default)]
    pub database_input: DatabaseInput,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatabaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDevEndpointRequest {
    #[serde(rename = "AddArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AddPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_public_keys: Option<Vec<String>>,
    #[serde(rename = "CustomLibraries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_libraries: Option<DevEndpointCustomLibraries>,
    #[serde(rename = "DeleteArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_arguments: Option<Vec<String>>,
    #[serde(rename = "DeletePublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_public_keys: Option<Vec<String>>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "UpdateEtlLibraries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_etl_libraries: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DevEndpointCustomLibraries {
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDevEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlueIdentityCenterConfigurationRequest {
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "UserBackgroundSessionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_background_sessions_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlueIdentityCenterConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResourcePropertyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResourcePropertyResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePropertyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_property_arn: Option<String>,
    #[serde(rename = "SourceProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing_properties: Option<SourceProcessingProperties>,
    #[serde(rename = "TargetProcessingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_processing_properties: Option<TargetProcessingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationTablePropertiesRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "SourceTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_config: Option<SourceTableConfig>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TargetTableConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table_config: Option<TargetTableConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationTablePropertiesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobFromSourceControlRequest {
    #[serde(rename = "AuthStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_strategy: Option<String>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "BranchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "CommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "Folder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "RepositoryOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobFromSourceControlResponse {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "JobUpdate")]
    #[serde(default)]
    pub job_update: JobUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobUpdate {
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<i32>,
    #[serde(rename = "CodeGenConfigurationNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_gen_configuration_nodes:
        Option<std::collections::HashMap<String, CodeGenConfigurationNode>>,
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[serde(rename = "DefaultArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<String>,
    #[serde(rename = "ExecutionProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "JobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
    #[serde(rename = "JobRunQueuingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_queuing_enabled: Option<bool>,
    #[serde(rename = "LogUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[serde(rename = "MaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "NonOverridableArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_overridable_arguments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NotificationProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    #[serde(rename = "SourceControlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_control_details: Option<SourceControlDetails>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobResponse {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMLTransformRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlueVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfWorkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TransformId")]
    #[serde(default)]
    pub transform_id: String,
    #[serde(rename = "WorkerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMLTransformResponse {
    #[serde(rename = "TransformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePartitionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartitionInput")]
    #[serde(default)]
    pub partition_input: PartitionInput,
    #[serde(rename = "PartitionValueList")]
    #[serde(default)]
    pub partition_value_list: Vec<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePartitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRegistryInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    pub registry_id: RegistryId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRegistryResponse {
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSchemaInput {
    #[serde(rename = "Compatibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    pub schema_id: SchemaId,
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<SchemaVersionNumber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSchemaResponse {
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSourceControlFromJobRequest {
    #[serde(rename = "AuthStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_strategy: Option<String>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "BranchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "CommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "Folder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "RepositoryOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSourceControlFromJobResponse {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableOptimizerRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    pub catalog_id: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TableOptimizerConfiguration")]
    #[serde(default)]
    pub table_optimizer_configuration: TableOptimizerConfiguration,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableOptimizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SkipArchive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_archive: Option<bool>,
    #[serde(rename = "TableInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_input: Option<TableInput>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "UpdateOpenTableFormatInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_open_table_format_input: Option<UpdateOpenTableFormatInput>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "ViewUpdateAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_update_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpenTableFormatInput {
    #[serde(rename = "UpdateIcebergInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_iceberg_input: Option<UpdateIcebergInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIcebergInput {
    #[serde(rename = "UpdateIcebergTableInput")]
    #[serde(default)]
    pub update_iceberg_table_input: UpdateIcebergTableInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIcebergTableInput {
    #[serde(rename = "Updates")]
    #[serde(default)]
    pub updates: Vec<IcebergTableUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergTableUpdate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "EncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<IcebergEncryptedKey>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: String,
    #[serde(rename = "PartitionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_spec: Option<IcebergPartitionSpec>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: IcebergSchema,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<IcebergSortOrder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergEncryptedKey {
    #[serde(rename = "EncryptedById")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_by_id: Option<String>,
    #[serde(rename = "EncryptedKeyMetadata")]
    #[serde(default)]
    pub encrypted_key_metadata: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTriggerRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TriggerUpdate")]
    #[serde(default)]
    pub trigger_update: TriggerUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerUpdate {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBatchingCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_batching_condition: Option<EventBatchingCondition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Predicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTriggerResponse {
    #[serde(rename = "Trigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUsageProfileRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: ProfileConfiguration,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUsageProfileResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserDefinedFunctionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "FunctionInput")]
    #[serde(default)]
    pub function_input: UserDefinedFunctionInput,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    pub function_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserDefinedFunctionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkflowRequest {
    #[serde(rename = "DefaultRunProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkflowResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
