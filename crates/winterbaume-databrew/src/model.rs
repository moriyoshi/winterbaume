//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-databrew

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteRecipeVersionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeVersions")]
    #[serde(default)]
    pub recipe_versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteRecipeVersionResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RecipeVersionErrorDetail>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeVersionErrorDetail {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetRequest {
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<FormatOptions>,
    #[serde(rename = "Input")]
    #[serde(default)]
    pub input: Input,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PathOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_options: Option<PathOptions>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormatOptions {
    #[serde(rename = "Csv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CsvOptions>,
    #[serde(rename = "Excel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excel: Option<ExcelOptions>,
    #[serde(rename = "Json")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsvOptions {
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "HeaderRow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExcelOptions {
    #[serde(rename = "HeaderRow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,
    #[serde(rename = "SheetIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_indexes: Option<Vec<i32>>,
    #[serde(rename = "SheetNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonOptions {
    #[serde(rename = "MultiLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_line: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Input {
    #[serde(rename = "DataCatalogInputDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,
    #[serde(rename = "DatabaseInputDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_input_definition: Option<DatabaseInputDefinition>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "S3InputDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_definition: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCatalogInputDefinition {
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
    #[serde(rename = "TempDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseInputDefinition {
    #[serde(rename = "DatabaseTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_table_name: Option<String>,
    #[serde(rename = "GlueConnectionName")]
    #[serde(default)]
    pub glue_connection_name: String,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "TempDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathOptions {
    #[serde(rename = "FilesLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_limit: Option<FilesLimit>,
    #[serde(rename = "LastModifiedDateCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_condition: Option<FilterExpression>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, DatasetParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilesLimit {
    #[serde(rename = "MaxFiles")]
    #[serde(default)]
    pub max_files: i32,
    #[serde(rename = "Order")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "OrderedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterExpression {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "ValuesMap")]
    #[serde(default)]
    pub values_map: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetParameter {
    #[serde(rename = "CreateColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_column: Option<bool>,
    #[serde(rename = "DatetimeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_options: Option<DatetimeOptions>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterExpression>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatetimeOptions {
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "LocaleCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
    #[serde(rename = "TimezoneOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProfileJobRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ProfileConfiguration>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    pub dataset_name: String,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    pub output_location: S3Location,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProfileConfiguration {
    #[serde(rename = "ColumnStatisticsConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_statistics_configurations: Option<Vec<ColumnStatisticsConfiguration>>,
    #[serde(rename = "DatasetStatisticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_statistics_configuration: Option<StatisticsConfiguration>,
    #[serde(rename = "EntityDetectorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detector_configuration: Option<EntityDetectorConfiguration>,
    #[serde(rename = "ProfileColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_columns: Option<Vec<ColumnSelector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnStatisticsConfiguration {
    #[serde(rename = "Selectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<ColumnSelector>>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    pub statistics: StatisticsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnSelector {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Regex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticsConfiguration {
    #[serde(rename = "IncludedStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_statistics: Option<Vec<String>>,
    #[serde(rename = "Overrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<StatisticOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticOverride {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, String>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    pub statistic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityDetectorConfiguration {
    #[serde(rename = "AllowedStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_statistics: Option<Vec<AllowedStatistics>>,
    #[serde(rename = "EntityTypes")]
    #[serde(default)]
    pub entity_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedStatistics {
    #[serde(rename = "Statistics")]
    #[serde(default)]
    pub statistics: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSample {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationConfiguration {
    #[serde(rename = "RulesetArn")]
    #[serde(default)]
    pub ruleset_arn: String,
    #[serde(rename = "ValidationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProfileJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectRequest {
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    pub dataset_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeName")]
    #[serde(default)]
    pub recipe_name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Sample>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sample {
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecipeJobRequest {
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCatalogOutput {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "DatabaseOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_options: Option<DatabaseTableOutputOptions>,
    #[serde(rename = "Overwrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(rename = "S3Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_options: Option<S3TableOutputOptions>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseTableOutputOptions {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TempDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3TableOutputOptions {
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: S3Location,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseOutput {
    #[serde(rename = "DatabaseOptions")]
    #[serde(default)]
    pub database_options: DatabaseTableOutputOptions,
    #[serde(rename = "DatabaseOutputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_output_mode: Option<String>,
    #[serde(rename = "GlueConnectionName")]
    #[serde(default)]
    pub glue_connection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Output {
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<OutputFormatOptions>,
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: S3Location,
    #[serde(rename = "MaxOutputFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_files: Option<i32>,
    #[serde(rename = "Overwrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(rename = "PartitionColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_columns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputFormatOptions {
    #[serde(rename = "Csv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CsvOutputOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsvOutputOptions {
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeReference {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecipeJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecipeRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Steps")]
    #[serde(default)]
    pub steps: Vec<RecipeStep>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct CreateRecipeResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRulesetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "CheckExpression")]
    #[serde(default)]
    pub check_expression: String,
    #[serde(rename = "ColumnSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_selectors: Option<Vec<ColumnSelector>>,
    #[serde(rename = "Disabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SubstitutionMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Threshold>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Threshold {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRulesetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleRequest {
    #[serde(rename = "CronExpression")]
    #[serde(default)]
    pub cron_expression: String,
    #[serde(rename = "JobNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduleResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecipeVersionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    pub recipe_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecipeVersionResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRulesetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRulesetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<FormatOptions>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PathOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_options: Option<PathOptions>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "ProfileConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_configuration: Option<ProfileConfiguration>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRunResponse {
    #[serde(rename = "Attempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "ProfileConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_configuration: Option<ProfileConfiguration>,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OpenDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_date: Option<f64>,
    #[serde(rename = "OpenedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    #[serde(rename = "RecipeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Sample>,
    #[serde(rename = "SessionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecipeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecipeResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "PublishedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
    #[serde(rename = "PublishedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<f64>,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RecipeStep>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRulesetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRulesetResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduleRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduleResponse {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CronExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "JobNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsRequest {
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
pub struct ListDatasetsResponse {
    #[serde(rename = "Datasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<Dataset>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dataset {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<FormatOptions>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PathOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_options: Option<PathOptions>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunsRequest {
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
pub struct ListJobRunsResponse {
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
pub struct JobRun {
    #[serde(rename = "Attempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[serde(rename = "CompletedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i32>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "StartedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(rename = "StartedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsRequest {
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsResponse {
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
pub struct Job {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "RecipeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_reference: Option<RecipeReference>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProjectsRequest {
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
pub struct ListProjectsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Projects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Project {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OpenDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_date: Option<f64>,
    #[serde(rename = "OpenedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    #[serde(rename = "RecipeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Sample>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecipeVersionsRequest {
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
pub struct ListRecipeVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Recipes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<Recipe>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recipe {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "PublishedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
    #[serde(rename = "PublishedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<f64>,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RecipeStep>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecipesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecipeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecipesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Recipes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<Recipe>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesetsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Rulesets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<Vec<RulesetItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RulesetItem {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_count: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchedulesRequest {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
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
pub struct ListSchedulesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CronExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "JobNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishRecipeRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishRecipeResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendProjectSessionActionRequest {
    #[serde(rename = "ClientSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Preview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(rename = "RecipeStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_step: Option<RecipeStep>,
    #[serde(rename = "StepIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_index: Option<i32>,
    #[serde(rename = "ViewFrame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_frame: Option<ViewFrame>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewFrame {
    #[serde(rename = "Analytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<String>,
    #[serde(rename = "ColumnRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_range: Option<i32>,
    #[serde(rename = "HiddenColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_columns: Option<Vec<String>>,
    #[serde(rename = "RowRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_range: Option<i32>,
    #[serde(rename = "StartColumnIndex")]
    #[serde(default)]
    pub start_column_index: i32,
    #[serde(rename = "StartRowIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_row_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendProjectSessionActionResponse {
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartProjectSessionRequest {
    #[serde(rename = "AssumeControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_control: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartProjectSessionResponse {
    #[serde(rename = "ClientSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopJobRunRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RunId")]
    #[serde(default)]
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopJobRunResponse {
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

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
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetRequest {
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<FormatOptions>,
    #[serde(rename = "Input")]
    #[serde(default)]
    pub input: Input,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PathOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_options: Option<PathOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProfileJobRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ProfileConfiguration>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "JobSample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_sample: Option<JobSample>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    pub output_location: S3Location,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "ValidationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProfileJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Sample>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProjectResponse {
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecipeJobRequest {
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    #[serde(rename = "DatabaseOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "LogSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscription: Option<String>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MaxRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecipeJobResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecipeRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RecipeStep>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecipeResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRulesetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRulesetResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduleRequest {
    #[serde(rename = "CronExpression")]
    #[serde(default)]
    pub cron_expression: String,
    #[serde(rename = "JobNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduleResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
