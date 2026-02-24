//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestreamwrite

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchLoadTaskRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DataModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model_configuration: Option<DataModelConfiguration>,
    #[serde(rename = "DataSourceConfiguration")]
    #[serde(default)]
    pub data_source_configuration: DataSourceConfiguration,
    #[serde(rename = "RecordVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_version: Option<i64>,
    #[serde(rename = "ReportConfiguration")]
    #[serde(default)]
    pub report_configuration: ReportConfiguration,
    #[serde(rename = "TargetDatabaseName")]
    #[serde(default)]
    pub target_database_name: String,
    #[serde(rename = "TargetTableName")]
    #[serde(default)]
    pub target_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataModelConfiguration {
    #[serde(rename = "DataModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model: Option<DataModel>,
    #[serde(rename = "DataModelS3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model_s3_configuration: Option<DataModelS3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataModel {
    #[serde(rename = "DimensionMappings")]
    #[serde(default)]
    pub dimension_mappings: Vec<DimensionMapping>,
    #[serde(rename = "MeasureNameColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name_column: Option<String>,
    #[serde(rename = "MixedMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_measure_mappings: Option<Vec<MixedMeasureMapping>>,
    #[serde(rename = "MultiMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_mappings: Option<MultiMeasureMappings>,
    #[serde(rename = "TimeColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_column: Option<String>,
    #[serde(rename = "TimeUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionMapping {
    #[serde(rename = "DestinationColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_column: Option<String>,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MixedMeasureMapping {
    #[serde(rename = "MeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name: Option<String>,
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    pub measure_value_type: String,
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_attribute_mappings: Option<Vec<MultiMeasureAttributeMapping>>,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<String>,
    #[serde(rename = "TargetMeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_measure_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureAttributeMapping {
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_value_type: Option<String>,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    pub source_column: String,
    #[serde(rename = "TargetMultiMeasureAttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_attribute_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureMappings {
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(default)]
    pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping>,
    #[serde(rename = "TargetMultiMeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataModelS3Configuration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceConfiguration {
    #[serde(rename = "CsvConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_configuration: Option<CsvConfiguration>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    pub data_format: String,
    #[serde(rename = "DataSourceS3Configuration")]
    #[serde(default)]
    pub data_source_s3_configuration: DataSourceS3Configuration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsvConfiguration {
    #[serde(rename = "ColumnSeparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_separator: Option<String>,
    #[serde(rename = "EscapeChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escape_char: Option<String>,
    #[serde(rename = "NullValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    #[serde(rename = "QuoteChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_char: Option<String>,
    #[serde(rename = "TrimWhiteSpace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_white_space: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceS3Configuration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportConfiguration {
    #[serde(rename = "ReportS3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_s3_configuration: Option<ReportS3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportS3Configuration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchLoadTaskResponse {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
pub struct CreateDatabaseResponse {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Database {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "TableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "MagneticStoreWriteProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,
    #[serde(rename = "RetentionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MagneticStoreWriteProperties {
    #[serde(rename = "EnableMagneticStoreWrites")]
    #[serde(default)]
    pub enable_magnetic_store_writes: bool,
    #[serde(rename = "MagneticStoreRejectedDataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_rejected_data_location: Option<MagneticStoreRejectedDataLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MagneticStoreRejectedDataLocation {
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Configuration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionProperties {
    #[serde(rename = "MagneticStoreRetentionPeriodInDays")]
    #[serde(default)]
    pub magnetic_store_retention_period_in_days: i64,
    #[serde(rename = "MemoryStoreRetentionPeriodInHours")]
    #[serde(default)]
    pub memory_store_retention_period_in_hours: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "CompositePartitionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_partition_key: Option<Vec<PartitionKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionKey {
    #[serde(rename = "EnforcementInRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_in_record: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTableResponse {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "MagneticStoreWriteProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,
    #[serde(rename = "RetentionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
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
pub struct DeleteDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTableRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchLoadTaskRequest {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchLoadTaskResponse {
    #[serde(rename = "BatchLoadTaskDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_load_task_description: Option<BatchLoadTaskDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchLoadTaskDescription {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model_configuration: Option<DataModelConfiguration>,
    #[serde(rename = "DataSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_configuration: Option<DataSourceConfiguration>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ProgressReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_report: Option<BatchLoadProgressReport>,
    #[serde(rename = "RecordVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_version: Option<i64>,
    #[serde(rename = "ReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_configuration: Option<ReportConfiguration>,
    #[serde(rename = "ResumableUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable_until: Option<f64>,
    #[serde(rename = "TargetDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[serde(rename = "TargetTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table_name: Option<String>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "TaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchLoadProgressReport {
    #[serde(rename = "BytesMetered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_metered: Option<i64>,
    #[serde(rename = "FileFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_failures: Option<i64>,
    #[serde(rename = "ParseFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_failures: Option<i64>,
    #[serde(rename = "RecordIngestionFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ingestion_failures: Option<i64>,
    #[serde(rename = "RecordsIngested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_ingested: Option<i64>,
    #[serde(rename = "RecordsProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_processed: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatabaseResponse {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
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
pub struct DescribeTableRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableResponse {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchLoadTasksRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchLoadTasksResponse {
    #[serde(rename = "BatchLoadTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_load_tasks: Option<Vec<BatchLoadTask>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchLoadTask {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ResumableUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable_until: Option<f64>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "TaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatabasesRequest {
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
pub struct ListDatabasesResponse {
    #[serde(rename = "Databases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<Database>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesRequest {
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<Table>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeBatchLoadTaskRequest {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeBatchLoadTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatabaseResponse {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "MagneticStoreWriteProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,
    #[serde(rename = "RetentionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableResponse {
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteRecordsRequest {
    #[serde(rename = "CommonAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attributes: Option<Record>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<Record>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name: Option<String>,
    #[serde(rename = "MeasureValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_value: Option<String>,
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_value_type: Option<String>,
    #[serde(rename = "MeasureValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_values: Option<Vec<MeasureValue>>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "TimeUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(rename = "DimensionValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeasureValue {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteRecordsResponse {
    #[serde(rename = "RecordsIngested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_ingested: Option<RecordsIngested>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordsIngested {
    #[serde(rename = "MagneticStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store: Option<i32>,
    #[serde(rename = "MemoryStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_store: Option<i32>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
