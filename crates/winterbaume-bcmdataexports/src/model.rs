//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bcmdataexports

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportRequest {
    #[serde(rename = "Export")]
    #[serde(default)]
    pub export: Export,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Export {
    #[serde(rename = "DataQuery")]
    #[serde(default)]
    pub data_query: DataQuery,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DestinationConfigurations")]
    #[serde(default)]
    pub destination_configurations: DestinationConfigurations,
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RefreshCadence")]
    #[serde(default)]
    pub refresh_cadence: RefreshCadence,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQuery {
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    pub query_statement: String,
    #[serde(rename = "TableConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_configurations:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfigurations {
    #[serde(rename = "S3Destination")]
    #[serde(default)]
    pub s3_destination: S3Destination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "S3OutputConfigurations")]
    #[serde(default)]
    pub s3_output_configurations: S3OutputConfigurations,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    pub s3_prefix: String,
    #[serde(rename = "S3Region")]
    #[serde(default)]
    pub s3_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3OutputConfigurations {
    #[serde(rename = "Compression")]
    #[serde(default)]
    pub compression: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "OutputType")]
    #[serde(default)]
    pub output_type: String,
    #[serde(rename = "Overwrite")]
    #[serde(default)]
    pub overwrite: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshCadence {
    #[serde(rename = "Frequency")]
    #[serde(default)]
    pub frequency: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportResponse {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExportRequest {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExportResponse {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionRequest {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionResponse {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<ExecutionStatus>,
    #[serde(rename = "Export")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStatus {
    #[serde(rename = "CompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportRequest {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportResponse {
    #[serde(rename = "Export")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    #[serde(rename = "ExportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<ExportStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportStatus {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "LastRefreshedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableRequest {
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TableProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<Column>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Column {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
pub struct ListExecutionsRequest {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
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
pub struct ListExecutionsResponse {
    #[serde(rename = "Executions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<Vec<ExecutionReference>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionReference {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<ExecutionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsRequest {
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
pub struct ListExportsResponse {
    #[serde(rename = "Exports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<Vec<ExportReference>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportReference {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "ExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_name: Option<String>,
    #[serde(rename = "ExportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<ExportStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesRequest {
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
pub struct Table {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_properties: Option<Vec<TablePropertyDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TablePropertyDescription {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ValidValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    pub resource_tags: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourceTagKeys")]
    #[serde(default)]
    pub resource_tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExportRequest {
    #[serde(rename = "Export")]
    #[serde(default)]
    pub export: Export,
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExportResponse {
    #[serde(rename = "ExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
}
