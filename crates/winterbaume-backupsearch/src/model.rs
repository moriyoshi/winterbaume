//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-backupsearch

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSearchJobInput {
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    pub search_job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSearchJobOutput {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentSearchProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_search_progress: Option<CurrentSearchProgress>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "ItemFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_filters: Option<ItemFilters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SearchJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_arn: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_identifier: Option<String>,
    #[serde(rename = "SearchScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope: Option<SearchScope>,
    #[serde(rename = "SearchScopeSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope_summary: Option<SearchScopeSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentSearchProgress {
    #[serde(rename = "ItemsMatchedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_matched_count: Option<i64>,
    #[serde(rename = "ItemsScannedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_scanned_count: Option<i64>,
    #[serde(rename = "RecoveryPointsScannedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points_scanned_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemFilters {
    #[serde(rename = "EBSItemFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_item_filters: Option<Vec<EBSItemFilter>>,
    #[serde(rename = "S3ItemFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_item_filters: Option<Vec<S3ItemFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSItemFilter {
    #[serde(rename = "CreationTimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_times: Option<Vec<TimeCondition>>,
    #[serde(rename = "FilePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<StringCondition>>,
    #[serde(rename = "LastModificationTimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_times: Option<Vec<TimeCondition>>,
    #[serde(rename = "Sizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<LongCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LongCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ItemFilter {
    #[serde(rename = "CreationTimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_times: Option<Vec<TimeCondition>>,
    #[serde(rename = "ETags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tags: Option<Vec<StringCondition>>,
    #[serde(rename = "ObjectKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<Vec<StringCondition>>,
    #[serde(rename = "Sizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<LongCondition>>,
    #[serde(rename = "VersionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids: Option<Vec<StringCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchScope {
    #[serde(rename = "BackupResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_arns: Option<Vec<String>>,
    #[serde(rename = "BackupResourceCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_creation_time: Option<BackupCreationTimeFilter>,
    #[serde(rename = "BackupResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BackupResourceTypes")]
    #[serde(default)]
    pub backup_resource_types: Vec<String>,
    #[serde(rename = "SourceResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupCreationTimeFilter {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchScopeSummary {
    #[serde(rename = "TotalItemsToScanCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_to_scan_count: Option<i64>,
    #[serde(rename = "TotalRecoveryPointsToScanCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recovery_points_to_scan_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSearchResultExportJobInput {
    #[serde(rename = "ExportJobIdentifier")]
    #[serde(default)]
    pub export_job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSearchResultExportJobOutput {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_arn: Option<String>,
    #[serde(rename = "ExportJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_identifier: Option<String>,
    #[serde(rename = "ExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_specification: Option<ExportSpecification>,
    #[serde(rename = "SearchJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSpecification {
    #[serde(rename = "s3ExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_export_specification: Option<S3ExportSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ExportSpecification {
    #[serde(rename = "DestinationBucket")]
    #[serde(default)]
    pub destination_bucket: String,
    #[serde(rename = "DestinationPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchJobBackupsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    pub search_job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchJobBackupsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SearchJobBackupsResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchJobBackupsResult {
    #[serde(rename = "BackupCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_time: Option<f64>,
    #[serde(rename = "BackupResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_arn: Option<String>,
    #[serde(rename = "IndexCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_creation_time: Option<f64>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchJobResultsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    pub search_job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchJobResultsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultItem {
    #[serde(rename = "EBSResultItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_result_item: Option<EBSResultItem>,
    #[serde(rename = "S3ResultItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_result_item: Option<S3ResultItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSResultItem {
    #[serde(rename = "BackupResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "FileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(rename = "FileSystemIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_identifier: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ResultItem {
    #[serde(rename = "BackupResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_resource_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
    #[serde(rename = "ObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size: Option<i64>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchJobsInput {
    #[serde(rename = "ByStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_status: Option<String>,
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
pub struct ListSearchJobsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_jobs: Option<Vec<SearchJobSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchJobSummary {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SearchJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_arn: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_identifier: Option<String>,
    #[serde(rename = "SearchScopeSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope_summary: Option<SearchScopeSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchResultExportJobsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSearchResultExportJobsOutput {
    #[serde(rename = "ExportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_jobs: Option<Vec<ExportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobSummary {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_arn: Option<String>,
    #[serde(rename = "ExportJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_identifier: Option<String>,
    #[serde(rename = "SearchJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
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
pub struct StartSearchJobInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "ItemFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_filters: Option<ItemFilters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SearchScope")]
    #[serde(default)]
    pub search_scope: SearchScope,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSearchJobOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "SearchJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_arn: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_job_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSearchResultExportJobInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ExportSpecification")]
    #[serde(default)]
    pub export_specification: ExportSpecification,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    pub search_job_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSearchResultExportJobOutput {
    #[serde(rename = "ExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_arn: Option<String>,
    #[serde(rename = "ExportJobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSearchJobInput {
    #[serde(rename = "SearchJobIdentifier")]
    #[serde(default)]
    pub search_job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSearchJobOutput {}

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
