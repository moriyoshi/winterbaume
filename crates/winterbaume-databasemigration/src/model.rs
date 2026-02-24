//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-databasemigration

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToResourceMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplyPendingMaintenanceActionMessage {
    #[serde(rename = "ApplyAction")]
    #[serde(default)]
    pub apply_action: String,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    pub opt_in_type: String,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplyPendingMaintenanceActionResponse {
    #[serde(rename = "ResourcePendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePendingMaintenanceActions {
    #[serde(rename = "PendingMaintenanceActionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingMaintenanceAction {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AutoAppliedAfterDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_applied_after_date: Option<f64>,
    #[serde(rename = "CurrentApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForcedApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_apply_date: Option<f64>,
    #[serde(rename = "OptInStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartRecommendationsRequest {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<StartRecommendationsRequestEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRecommendationsRequestEntry {
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    pub database_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: RecommendationSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationSettings {
    #[serde(rename = "InstanceSizingType")]
    #[serde(default)]
    pub instance_sizing_type: String,
    #[serde(rename = "WorkloadType")]
    #[serde(default)]
    pub workload_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartRecommendationsResponse {
    #[serde(rename = "ErrorEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchStartRecommendationsErrorEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartRecommendationsErrorEntry {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMetadataModelConversionMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    pub request_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMetadataModelConversionResponse {
    #[serde(rename = "Request")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<SchemaConversionRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaConversionRequest {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "ExportSqlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_sql_details: Option<ExportSqlDetails>,
    #[serde(rename = "MigrationProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_arn: Option<String>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<Progress>,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    #[serde(rename = "defaultErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_error_details: Option<DefaultErrorDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultErrorDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSqlDetails {
    #[serde(rename = "ObjectURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_u_r_l: Option<String>,
    #[serde(rename = "S3ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Progress {
    #[serde(rename = "ProcessedObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_object: Option<ProcessedObject>,
    #[serde(rename = "ProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<f64>,
    #[serde(rename = "ProgressStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_step: Option<String>,
    #[serde(rename = "TotalObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessedObject {
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
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
pub struct CancelMetadataModelCreationMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    pub request_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMetadataModelCreationResponse {
    #[serde(rename = "Request")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<SchemaConversionRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelReplicationTaskAssessmentRunMessage {
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(default)]
    pub replication_task_assessment_run_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelReplicationTaskAssessmentRunResponse {
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskAssessmentRun {
    #[serde(rename = "AssessmentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_progress: Option<ReplicationTaskAssessmentRunProgress>,
    #[serde(rename = "AssessmentRunName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_name: Option<String>,
    #[serde(rename = "IsLatestTaskAssessmentRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_task_assessment_run: Option<bool>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_arn: Option<String>,
    #[serde(rename = "ReplicationTaskAssessmentRunCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_creation_date: Option<f64>,
    #[serde(rename = "ResultEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_encryption_mode: Option<String>,
    #[serde(rename = "ResultKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_kms_key_arn: Option<String>,
    #[serde(rename = "ResultLocationBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_bucket: Option<String>,
    #[serde(rename = "ResultLocationFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_folder: Option<String>,
    #[serde(rename = "ResultStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_statistic: Option<ReplicationTaskAssessmentRunResultStatistic>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskAssessmentRunProgress {
    #[serde(rename = "IndividualAssessmentCompletedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_completed_count: Option<i32>,
    #[serde(rename = "IndividualAssessmentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskAssessmentRunResultStatistic {
    #[serde(rename = "Cancelled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<i32>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<i32>,
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(rename = "Passed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<i32>,
    #[serde(rename = "Skipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i32>,
    #[serde(rename = "Warning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataMigrationMessage {
    #[serde(rename = "DataMigrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_name: Option<String>,
    #[serde(rename = "DataMigrationType")]
    #[serde(default)]
    pub data_migration_type: String,
    #[serde(rename = "EnableCloudwatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs: Option<bool>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "NumberOfJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_jobs: Option<i32>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_rules: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    pub service_access_role_arn: String,
    #[serde(rename = "SourceDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data_settings: Option<Vec<SourceDataSetting>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data_settings: Option<Vec<TargetDataSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceDataSetting {
    #[serde(rename = "CDCStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_d_c_start_position: Option<String>,
    #[serde(rename = "CDCStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_d_c_start_time: Option<String>,
    #[serde(rename = "CDCStopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_d_c_stop_time: Option<String>,
    #[serde(rename = "SlotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetDataSetting {
    #[serde(rename = "TablePreparationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_preparation_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataMigrationResponse {
    #[serde(rename = "DataMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration: Option<DataMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataMigration {
    #[serde(rename = "DataMigrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_arn: Option<String>,
    #[serde(rename = "DataMigrationCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "DataMigrationCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_create_time: Option<String>,
    #[serde(rename = "DataMigrationEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_end_time: Option<String>,
    #[serde(rename = "DataMigrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_name: Option<String>,
    #[serde(rename = "DataMigrationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_settings: Option<DataMigrationSettings>,
    #[serde(rename = "DataMigrationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_start_time: Option<String>,
    #[serde(rename = "DataMigrationStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_statistics: Option<DataMigrationStatistics>,
    #[serde(rename = "DataMigrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_status: Option<String>,
    #[serde(rename = "DataMigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_type: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "MigrationProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_arn: Option<String>,
    #[serde(rename = "PublicIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SourceDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data_settings: Option<Vec<SourceDataSetting>>,
    #[serde(rename = "StopReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(rename = "TargetDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data_settings: Option<Vec<TargetDataSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataMigrationSettings {
    #[serde(rename = "CloudwatchLogsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_enabled: Option<bool>,
    #[serde(rename = "NumberOfJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_jobs: Option<i32>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataMigrationStatistics {
    #[serde(rename = "CDCLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_d_c_latency: Option<i32>,
    #[serde(rename = "ElapsedTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis: Option<i64>,
    #[serde(rename = "FullLoadPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_percentage: Option<i32>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "StopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<String>,
    #[serde(rename = "TablesErrored")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_errored: Option<i32>,
    #[serde(rename = "TablesLoaded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loaded: Option<i32>,
    #[serde(rename = "TablesLoading")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loading: Option<i32>,
    #[serde(rename = "TablesQueued")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_queued: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataProviderMessage {
    #[serde(rename = "DataProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: DataProviderSettings,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Virtual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProviderSettings {
    #[serde(rename = "DocDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbDataProviderSettings>,
    #[serde(rename = "IbmDb2LuwSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db2_luw_settings: Option<IbmDb2LuwDataProviderSettings>,
    #[serde(rename = "IbmDb2zOsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db2z_os_settings: Option<IbmDb2zOsDataProviderSettings>,
    #[serde(rename = "MariaDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maria_db_settings: Option<MariaDbDataProviderSettings>,
    #[serde(rename = "MicrosoftSqlServerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_sql_server_settings: Option<MicrosoftSqlServerDataProviderSettings>,
    #[serde(rename = "MongoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbDataProviderSettings>,
    #[serde(rename = "MySqlSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_settings: Option<MySqlDataProviderSettings>,
    #[serde(rename = "OracleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleDataProviderSettings>,
    #[serde(rename = "PostgreSqlSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_settings: Option<PostgreSqlDataProviderSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftDataProviderSettings>,
    #[serde(rename = "SybaseAseSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_ase_settings: Option<SybaseAseDataProviderSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocDbDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IbmDb2LuwDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "SecurityMechanism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_mechanism: Option<i32>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IbmDb2zOsDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MariaDbDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MicrosoftSqlServerDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MongoDbDataProviderSettings {
    #[serde(rename = "AuthMechanism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<String>,
    #[serde(rename = "AuthSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source: Option<String>,
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MySqlDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OracleDataProviderSettings {
    #[serde(rename = "AsmServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_server: Option<String>,
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "SecretsManagerOracleAsmAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerOracleAsmSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_secret_id: Option<String>,
    #[serde(rename = "SecretsManagerSecurityDbEncryptionAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_security_db_encryption_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecurityDbEncryptionSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_security_db_encryption_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostgreSqlDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDataProviderSettings {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "S3AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SybaseAseDataProviderSettings {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EncryptPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_password: Option<bool>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataProviderResponse {
    #[serde(rename = "DataProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider: Option<DataProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProvider {
    #[serde(rename = "DataProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_arn: Option<String>,
    #[serde(rename = "DataProviderCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_creation_time: Option<String>,
    #[serde(rename = "DataProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DataProviderSettings>,
    #[serde(rename = "Virtual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointMessage {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DmsTransferSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    #[serde(rename = "DocDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    #[serde(rename = "DynamoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default)]
    pub endpoint_identifier: String,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    pub endpoint_type: String,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    pub engine_name: String,
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[serde(rename = "GcpMySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_my_s_q_l_settings: Option<GcpMySQLSettings>,
    #[serde(rename = "IBMDb2Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_b_m_db2_settings: Option<IBMDb2Settings>,
    #[serde(rename = "KafkaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    #[serde(rename = "KinesisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_s_q_l_server_settings: Option<MicrosoftSQLServerSettings>,
    #[serde(rename = "MongoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[serde(rename = "MySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_s_q_l_settings: Option<MySQLSettings>,
    #[serde(rename = "NeptuneSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    #[serde(rename = "OracleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_s_q_l_settings: Option<PostgreSQLSettings>,
    #[serde(rename = "RedisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_settings: Option<RedisSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "S3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    #[serde(rename = "SybaseSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimestreamSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream_settings: Option<TimestreamSettings>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DmsTransferSettings {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocDbSettings {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DocsToInvestigate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<i32>,
    #[serde(rename = "ExtractDocId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "NestingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ReplicateShardCollections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate_shard_collections: Option<bool>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "UseUpdateLookUp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_update_look_up: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDbSettings {
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    pub service_access_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchSettings {
    #[serde(rename = "EndpointUri")]
    #[serde(default)]
    pub endpoint_uri: String,
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i32>,
    #[serde(rename = "FullLoadErrorPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_percentage: Option<i32>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    pub service_access_role_arn: String,
    #[serde(rename = "UseNewMappingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_new_mapping_type: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GcpMySQLSettings {
    #[serde(rename = "AfterConnectScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_source_metadata_on_mismatch: Option<bool>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EventsPollInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_poll_interval: Option<i32>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "ParallelLoadThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_load_threads: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timezone: Option<String>,
    #[serde(rename = "TargetDbType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_db_type: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IBMDb2Settings {
    #[serde(rename = "CurrentLsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_lsn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "KeepCsvFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_csv_files: Option<bool>,
    #[serde(rename = "LoadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_timeout: Option<i32>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "MaxKBytesPerRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_k_bytes_per_read: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SetDataCaptureChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_data_capture_changes: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "WriteBufferSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaSettings {
    #[serde(rename = "Broker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker: Option<String>,
    #[serde(rename = "IncludeControlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,
    #[serde(rename = "IncludePartitionValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,
    #[serde(rename = "MessageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    #[serde(rename = "MessageMaxBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<i32>,
    #[serde(rename = "NoHexPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hex_prefix: Option<bool>,
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,
    #[serde(rename = "SaslMechanism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_mechanism: Option<String>,
    #[serde(rename = "SaslPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_password: Option<String>,
    #[serde(rename = "SaslUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_username: Option<String>,
    #[serde(rename = "SecurityProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_protocol: Option<String>,
    #[serde(rename = "SslCaCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_certificate_arn: Option<String>,
    #[serde(rename = "SslClientCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_certificate_arn: Option<String>,
    #[serde(rename = "SslClientKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_key_arn: Option<String>,
    #[serde(rename = "SslClientKeyPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_key_password: Option<String>,
    #[serde(rename = "SslEndpointIdentificationAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_endpoint_identification_algorithm: Option<String>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "UseLargeIntegerValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_large_integer_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisSettings {
    #[serde(rename = "IncludeControlDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,
    #[serde(rename = "IncludePartitionValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,
    #[serde(rename = "MessageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    #[serde(rename = "NoHexPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hex_prefix: Option<bool>,
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "UseLargeIntegerValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_large_integer_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MicrosoftSQLServerSettings {
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    #[serde(rename = "BcpPacketSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcp_packet_size: Option<i32>,
    #[serde(rename = "ControlTablesFileGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_tables_file_group: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "ForceLobLookup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_lob_lookup: Option<bool>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "QuerySingleAlwaysOnNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_single_always_on_node: Option<bool>,
    #[serde(rename = "ReadBackupOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_backup_only: Option<bool>,
    #[serde(rename = "SafeguardPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safeguard_policy: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "TlogAccessMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlog_access_mode: Option<String>,
    #[serde(rename = "TrimSpaceInChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_space_in_char: Option<bool>,
    #[serde(rename = "UseBcpFullLoad")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bcp_full_load: Option<bool>,
    #[serde(rename = "UseThirdPartyBackupDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_third_party_backup_device: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MongoDbSettings {
    #[serde(rename = "AuthMechanism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<String>,
    #[serde(rename = "AuthSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source: Option<String>,
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DocsToInvestigate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<String>,
    #[serde(rename = "ExtractDocId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "NestingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ReplicateShardCollections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate_shard_collections: Option<bool>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "UseUpdateLookUp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_update_look_up: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MySQLSettings {
    #[serde(rename = "AfterConnectScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_source_metadata_on_mismatch: Option<bool>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EventsPollInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_poll_interval: Option<i32>,
    #[serde(rename = "ExecuteTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_timeout: Option<i32>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "ParallelLoadThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_load_threads: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timezone: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "TargetDbType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_db_type: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NeptuneSettings {
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i32>,
    #[serde(rename = "IamAuthEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_auth_enabled: Option<bool>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "MaxRetryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_count: Option<i32>,
    #[serde(rename = "S3BucketFolder")]
    #[serde(default)]
    pub s3_bucket_folder: String,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OracleSettings {
    #[serde(rename = "AccessAlternateDirectly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_alternate_directly: Option<bool>,
    #[serde(rename = "AddSupplementalLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_supplemental_logging: Option<bool>,
    #[serde(rename = "AdditionalArchivedLogDestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_archived_log_dest_id: Option<i32>,
    #[serde(rename = "AllowSelectNestedTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_select_nested_tables: Option<bool>,
    #[serde(rename = "ArchivedLogDestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_log_dest_id: Option<i32>,
    #[serde(rename = "ArchivedLogsOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_logs_only: Option<bool>,
    #[serde(rename = "AsmPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_password: Option<String>,
    #[serde(rename = "AsmServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_server: Option<String>,
    #[serde(rename = "AsmUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_user: Option<String>,
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    #[serde(rename = "CharLengthSemantics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_length_semantics: Option<String>,
    #[serde(rename = "ConvertTimestampWithZoneToUTC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_timestamp_with_zone_to_u_t_c: Option<bool>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DirectPathNoLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_no_log: Option<bool>,
    #[serde(rename = "DirectPathParallelLoad")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_parallel_load: Option<bool>,
    #[serde(rename = "EnableHomogenousTablespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_homogenous_tablespace: Option<bool>,
    #[serde(rename = "ExtraArchivedLogDestIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_archived_log_dest_ids: Option<Vec<i32>>,
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    #[serde(rename = "NumberDatatypeScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_datatype_scale: Option<i32>,
    #[serde(rename = "OpenTransactionWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_transaction_window: Option<i32>,
    #[serde(rename = "OraclePathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_path_prefix: Option<String>,
    #[serde(rename = "ParallelAsmReadThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_asm_read_threads: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ReadAheadBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ahead_blocks: Option<i32>,
    #[serde(rename = "ReadTableSpaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_table_space_name: Option<bool>,
    #[serde(rename = "ReplacePathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_path_prefix: Option<bool>,
    #[serde(rename = "RetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerOracleAsmAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerOracleAsmSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_secret_id: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "SecurityDbEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption: Option<String>,
    #[serde(rename = "SecurityDbEncryptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption_name: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SpatialDataOptionToGeoJsonFunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_data_option_to_geo_json_function_name: Option<String>,
    #[serde(rename = "StandbyDelayTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_delay_time: Option<i32>,
    #[serde(rename = "TrimSpaceInChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_space_in_char: Option<bool>,
    #[serde(rename = "UseAlternateFolderForOnline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_alternate_folder_for_online: Option<bool>,
    #[serde(rename = "UseBFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_b_file: Option<bool>,
    #[serde(rename = "UseDirectPathFullLoad")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_direct_path_full_load: Option<bool>,
    #[serde(rename = "UseLogminerReader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_logminer_reader: Option<bool>,
    #[serde(rename = "UsePathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_path_prefix: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostgreSQLSettings {
    #[serde(rename = "AfterConnectScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    #[serde(rename = "BabelfishDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub babelfish_database_name: Option<String>,
    #[serde(rename = "CaptureDdls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_ddls: Option<bool>,
    #[serde(rename = "DatabaseMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_mode: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DdlArtifactsSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_artifacts_schema: Option<String>,
    #[serde(rename = "DisableUnicodeSourceFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_unicode_source_filter: Option<bool>,
    #[serde(rename = "ExecuteTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_timeout: Option<i32>,
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    #[serde(rename = "HeartbeatEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_enable: Option<bool>,
    #[serde(rename = "HeartbeatFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_frequency: Option<i32>,
    #[serde(rename = "HeartbeatSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_schema: Option<String>,
    #[serde(rename = "MapBooleanAsBoolean")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_boolean_as_boolean: Option<bool>,
    #[serde(rename = "MapJsonbAsClob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_jsonb_as_clob: Option<bool>,
    #[serde(rename = "MapLongVarcharAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_long_varchar_as: Option<String>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "PluginName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SlotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "TrimSpaceInChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_space_in_char: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedisSettings {
    #[serde(rename = "AuthPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_password: Option<String>,
    #[serde(rename = "AuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "AuthUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    pub server_name: String,
    #[serde(rename = "SslCaCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_certificate_arn: Option<String>,
    #[serde(rename = "SslSecurityProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_security_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftSettings {
    #[serde(rename = "AcceptAnyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_any_date: Option<bool>,
    #[serde(rename = "AfterConnectScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "BucketFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "CaseSensitiveNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive_names: Option<bool>,
    #[serde(rename = "CompUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp_update: Option<bool>,
    #[serde(rename = "ConnectionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i32>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "EmptyAsNull")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_as_null: Option<bool>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "ExplicitIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_ids: Option<bool>,
    #[serde(rename = "FileTransferUploadStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_transfer_upload_streams: Option<i32>,
    #[serde(rename = "LoadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_timeout: Option<i32>,
    #[serde(rename = "MapBooleanAsBoolean")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_boolean_as_boolean: Option<bool>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "RemoveQuotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_quotes: Option<bool>,
    #[serde(rename = "ReplaceChars")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_chars: Option<String>,
    #[serde(rename = "ReplaceInvalidChars")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_invalid_chars: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "TimeFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    #[serde(rename = "TrimBlanks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_blanks: Option<bool>,
    #[serde(rename = "TruncateColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate_columns: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "WriteBufferSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Settings {
    #[serde(rename = "AddColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_column_name: Option<bool>,
    #[serde(rename = "AddTrailingPaddingCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_trailing_padding_character: Option<bool>,
    #[serde(rename = "BucketFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "CannedAclForObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl_for_objects: Option<String>,
    #[serde(rename = "CdcInsertsAndUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_and_updates: Option<bool>,
    #[serde(rename = "CdcInsertsOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_only: Option<bool>,
    #[serde(rename = "CdcMaxBatchInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_max_batch_interval: Option<i32>,
    #[serde(rename = "CdcMinFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_min_file_size: Option<i32>,
    #[serde(rename = "CdcPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_path: Option<String>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "CsvDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    #[serde(rename = "CsvNoSupValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_no_sup_value: Option<String>,
    #[serde(rename = "CsvNullValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_null_value: Option<String>,
    #[serde(rename = "CsvRowDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_row_delimiter: Option<String>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "DataPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_page_size: Option<i32>,
    #[serde(rename = "DatePartitionDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_delimiter: Option<String>,
    #[serde(rename = "DatePartitionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_enabled: Option<bool>,
    #[serde(rename = "DatePartitionSequence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_sequence: Option<String>,
    #[serde(rename = "DatePartitionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_timezone: Option<String>,
    #[serde(rename = "DictPageSizeLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_page_size_limit: Option<i32>,
    #[serde(rename = "EnableStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_statistics: Option<bool>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "EncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    #[serde(rename = "GlueCatalogGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_catalog_generation: Option<bool>,
    #[serde(rename = "IgnoreHeaderRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_header_rows: Option<i32>,
    #[serde(rename = "IncludeOpForFullLoad")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_op_for_full_load: Option<bool>,
    #[serde(rename = "MaxFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    #[serde(rename = "ParquetTimestampInMillisecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_timestamp_in_millisecond: Option<bool>,
    #[serde(rename = "ParquetVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_version: Option<String>,
    #[serde(rename = "PreserveTransactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_transactions: Option<bool>,
    #[serde(rename = "Rfc4180")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc4180: Option<bool>,
    #[serde(rename = "RowGroupLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_length: Option<i32>,
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "TimestampColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_column_name: Option<String>,
    #[serde(rename = "UseCsvNoSupValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_csv_no_sup_value: Option<bool>,
    #[serde(rename = "UseTaskStartTimeForFullLoadTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_task_start_time_for_full_load_timestamp: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SybaseSettings {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamSettings {
    #[serde(rename = "CdcInsertsAndUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_and_updates: Option<bool>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "EnableMagneticStoreWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_magnetic_store_writes: Option<bool>,
    #[serde(rename = "MagneticDuration")]
    #[serde(default)]
    pub magnetic_duration: i32,
    #[serde(rename = "MemoryDuration")]
    #[serde(default)]
    pub memory_duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointResponse {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DmsTransferSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    #[serde(rename = "DocDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    #[serde(rename = "DynamoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "EngineDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[serde(rename = "GcpMySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_my_s_q_l_settings: Option<GcpMySQLSettings>,
    #[serde(rename = "IBMDb2Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_b_m_db2_settings: Option<IBMDb2Settings>,
    #[serde(rename = "IsReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "KafkaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    #[serde(rename = "KinesisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LakehouseSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_settings: Option<LakehouseSettings>,
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_s_q_l_server_settings: Option<MicrosoftSQLServerSettings>,
    #[serde(rename = "MongoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[serde(rename = "MySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_s_q_l_settings: Option<MySQLSettings>,
    #[serde(rename = "NeptuneSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    #[serde(rename = "OracleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_s_q_l_settings: Option<PostgreSQLSettings>,
    #[serde(rename = "RedisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_settings: Option<RedisSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    #[serde(rename = "S3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SybaseSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    #[serde(rename = "TimestreamSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream_settings: Option<TimestreamSettings>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LakehouseSettings {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    pub sns_topic_arn: String,
    #[serde(rename = "SourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventSubscriptionResponse {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSubscription {
    #[serde(rename = "CustSubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    #[serde(rename = "CustomerAwsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategoriesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<Vec<String>>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceIdsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<Vec<String>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetAdvisorCollectorRequest {
    #[serde(rename = "CollectorName")]
    #[serde(default)]
    pub collector_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    pub service_access_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetAdvisorCollectorResponse {
    #[serde(rename = "CollectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_name: Option<String>,
    #[serde(rename = "CollectorReferencedId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_referenced_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceProfileMessage {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "SubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceProfileResponse {
    #[serde(rename = "InstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceProfile {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
    #[serde(rename = "InstanceProfileCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_creation_time: Option<String>,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "SubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_identifier: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMigrationProjectMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileIdentifier")]
    #[serde(default)]
    pub instance_profile_identifier: String,
    #[serde(rename = "MigrationProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_name: Option<String>,
    #[serde(rename = "SchemaConversionApplicationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_conversion_application_attributes: Option<SCApplicationAttributes>,
    #[serde(rename = "SourceDataProviderDescriptors")]
    #[serde(default)]
    pub source_data_provider_descriptors: Vec<DataProviderDescriptorDefinition>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetDataProviderDescriptors")]
    #[serde(default)]
    pub target_data_provider_descriptors: Vec<DataProviderDescriptorDefinition>,
    #[serde(rename = "TransformationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SCApplicationAttributes {
    #[serde(rename = "S3BucketPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_path: Option<String>,
    #[serde(rename = "S3BucketRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProviderDescriptorDefinition {
    #[serde(rename = "DataProviderIdentifier")]
    #[serde(default)]
    pub data_provider_identifier: String,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMigrationProjectResponse {
    #[serde(rename = "MigrationProject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project: Option<MigrationProject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MigrationProject {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "MigrationProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_arn: Option<String>,
    #[serde(rename = "MigrationProjectCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_creation_time: Option<String>,
    #[serde(rename = "MigrationProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_name: Option<String>,
    #[serde(rename = "SchemaConversionApplicationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_conversion_application_attributes: Option<SCApplicationAttributes>,
    #[serde(rename = "SourceDataProviderDescriptors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data_provider_descriptors: Option<Vec<DataProviderDescriptor>>,
    #[serde(rename = "TargetDataProviderDescriptors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data_provider_descriptors: Option<Vec<DataProviderDescriptor>>,
    #[serde(rename = "TransformationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProviderDescriptor {
    #[serde(rename = "DataProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_arn: Option<String>,
    #[serde(rename = "DataProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_name: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationConfigMessage {
    #[serde(rename = "ComputeConfig")]
    #[serde(default)]
    pub compute_config: ComputeConfig,
    #[serde(rename = "ReplicationConfigIdentifier")]
    #[serde(default)]
    pub replication_config_identifier: String,
    #[serde(rename = "ReplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_settings: Option<String>,
    #[serde(rename = "ReplicationType")]
    #[serde(default)]
    pub replication_type: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    pub source_endpoint_arn: String,
    #[serde(rename = "SupplementalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_settings: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    pub table_mappings: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    pub target_endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeConfig {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "DnsNameServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaxCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity_units: Option<i32>,
    #[serde(rename = "MinCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity_units: Option<i32>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ReplicationSubnetGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_id: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationConfigResponse {
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationConfig {
    #[serde(rename = "ComputeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_config: Option<ComputeConfig>,
    #[serde(rename = "IsReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_arn: Option<String>,
    #[serde(rename = "ReplicationConfigCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_create_time: Option<f64>,
    #[serde(rename = "ReplicationConfigIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_identifier: Option<String>,
    #[serde(rename = "ReplicationConfigUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_update_time: Option<f64>,
    #[serde(rename = "ReplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_settings: Option<String>,
    #[serde(rename = "ReplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<String>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[serde(rename = "SupplementalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_settings: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationInstanceMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "DnsNameServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KerberosAuthenticationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_authentication_settings: Option<KerberosAuthenticationSettings>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    pub replication_instance_class: String,
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default)]
    pub replication_instance_identifier: String,
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KerberosAuthenticationSettings {
    #[serde(rename = "KeyCacheSecretIamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_cache_secret_iam_arn: Option<String>,
    #[serde(rename = "KeyCacheSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_cache_secret_id: Option<String>,
    #[serde(rename = "Krb5FileContents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub krb5_file_contents: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInstance {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "DnsNameServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "FreeUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_until: Option<f64>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<f64>,
    #[serde(rename = "KerberosAuthenticationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_authentication_settings: Option<KerberosAuthenticationSettings>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ReplicationPendingModifiedValues>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[serde(rename = "ReplicationInstanceIpv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_ipv6_addresses: Option<Vec<String>>,
    #[serde(rename = "ReplicationInstancePrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_private_ip_address: Option<String>,
    #[serde(rename = "ReplicationInstancePrivateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_private_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "ReplicationInstancePublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_public_ip_address: Option<String>,
    #[serde(rename = "ReplicationInstancePublicIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_public_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "ReplicationInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_status: Option<String>,
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationPendingModifiedValues {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationSubnetGroup {
    #[serde(rename = "IsReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
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
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    #[serde(rename = "SubnetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    #[serde(rename = "SubnetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcSecurityGroupMembership {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationSubnetGroupMessage {
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(default)]
    pub replication_subnet_group_description: String,
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    pub replication_subnet_group_identifier: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationSubnetGroupResponse {
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationTaskMessage {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "MigrationType")]
    #[serde(default)]
    pub migration_type: String,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default)]
    pub replication_task_identifier: String,
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    pub source_endpoint_arn: String,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    pub table_mappings: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    pub target_endpoint_arn: String,
    #[serde(rename = "TaskData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTask {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "MigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    #[serde(rename = "RecoveryCheckpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "ReplicationTaskCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_creation_date: Option<f64>,
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[serde(rename = "ReplicationTaskStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_start_date: Option<f64>,
    #[serde(rename = "ReplicationTaskStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_stats: Option<ReplicationTaskStats>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StopReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
    #[serde(rename = "TargetReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_replication_instance_arn: Option<String>,
    #[serde(rename = "TaskData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskStats {
    #[serde(rename = "ElapsedTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis: Option<i64>,
    #[serde(rename = "FreshStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh_start_date: Option<f64>,
    #[serde(rename = "FullLoadFinishDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_finish_date: Option<f64>,
    #[serde(rename = "FullLoadProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_progress_percent: Option<i32>,
    #[serde(rename = "FullLoadStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_start_date: Option<f64>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "StopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "TablesErrored")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_errored: Option<i32>,
    #[serde(rename = "TablesLoaded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loaded: Option<i32>,
    #[serde(rename = "TablesLoading")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loading: Option<i32>,
    #[serde(rename = "TablesQueued")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_queued: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateMessage {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_creation_date: Option<f64>,
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    #[serde(rename = "CertificateOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_owner: Option<String>,
    #[serde(rename = "CertificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "CertificateWallet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<String>,
    #[serde(rename = "KeyLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
    #[serde(rename = "ValidFromDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<f64>,
    #[serde(rename = "ValidToDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectorRequest {
    #[serde(rename = "CollectorReferencedId")]
    #[serde(default)]
    pub collector_referenced_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataMigrationMessage {
    #[serde(rename = "DataMigrationIdentifier")]
    #[serde(default)]
    pub data_migration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataMigrationResponse {
    #[serde(rename = "DataMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration: Option<DataMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataProviderMessage {
    #[serde(rename = "DataProviderIdentifier")]
    #[serde(default)]
    pub data_provider_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataProviderResponse {
    #[serde(rename = "DataProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider: Option<DataProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointResponse {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventSubscriptionMessage {
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventSubscriptionResponse {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFleetAdvisorDatabasesRequest {
    #[serde(rename = "DatabaseIds")]
    #[serde(default)]
    pub database_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFleetAdvisorDatabasesResponse {
    #[serde(rename = "DatabaseIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceProfileMessage {
    #[serde(rename = "InstanceProfileIdentifier")]
    #[serde(default)]
    pub instance_profile_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceProfileResponse {
    #[serde(rename = "InstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMigrationProjectMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMigrationProjectResponse {
    #[serde(rename = "MigrationProject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project: Option<MigrationProject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationConfigMessage {
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationConfigResponse {
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationInstanceMessage {
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationSubnetGroupMessage {
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    pub replication_subnet_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationSubnetGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationTaskAssessmentRunMessage {
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(default)]
    pub replication_task_assessment_run_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationTaskAssessmentRunResponse {
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationTaskMessage {
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAttributesMessage {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAttributesResponse {
    #[serde(rename = "AccountQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quotas: Option<Vec<AccountQuota>>,
    #[serde(rename = "UniqueAccountIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_account_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountQuota {
    #[serde(rename = "AccountQuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quota_name: Option<String>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Used")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicableIndividualAssessmentsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_arn: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "SourceEngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_engine_name: Option<String>,
    #[serde(rename = "TargetEngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_engine_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicableIndividualAssessmentsResponse {
    #[serde(rename = "IndividualAssessmentNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_names: Option<Vec<String>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificatesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificatesResponse {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConversionConfigurationMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConversionConfigurationResponse {
    #[serde(rename = "ConversionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_configuration: Option<String>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataMigrationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "WithoutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_settings: Option<bool>,
    #[serde(rename = "WithoutStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_statistics: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataMigrationsResponse {
    #[serde(rename = "DataMigrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migrations: Option<Vec<DataMigration>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataProvidersMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeDataProvidersResponse {
    #[serde(rename = "DataProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_providers: Option<Vec<DataProvider>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointSettingsMessage {
    #[serde(rename = "EngineName")]
    #[serde(default)]
    pub engine_name: String,
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
pub struct DescribeEndpointSettingsResponse {
    #[serde(rename = "EndpointSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_settings: Option<Vec<EndpointSetting>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointSetting {
    #[serde(rename = "Applicability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicability: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "EnumValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(rename = "IntValueMax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value_max: Option<i32>,
    #[serde(rename = "IntValueMin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value_min: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Sensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Units")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointTypesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeEndpointTypesResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SupportedEndpointTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_endpoint_types: Option<Vec<SupportedEndpointType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedEndpointType {
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "EngineDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "ReplicationInstanceEngineMinimumVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_engine_minimum_version: Option<String>,
    #[serde(rename = "SupportsCDC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_c_d_c: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeEndpointsResponse {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEngineVersionsMessage {
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
pub struct DescribeEngineVersionsResponse {
    #[serde(rename = "EngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<Vec<EngineVersion>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineVersion {
    #[serde(rename = "AutoUpgradeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_date: Option<f64>,
    #[serde(rename = "AvailableUpgrades")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_upgrades: Option<Vec<String>>,
    #[serde(rename = "DeprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    #[serde(rename = "ForceUpgradeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_upgrade_date: Option<f64>,
    #[serde(rename = "LaunchDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_date: Option<f64>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "ReleaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_status: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventCategoriesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventCategoriesResponse {
    #[serde(rename = "EventCategoryGroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category_group_list: Option<Vec<EventCategoryGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCategoryGroup {
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventSubscriptionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventSubscriptionsResponse {
    #[serde(rename = "EventSubscriptionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventsMessage {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
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
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExtensionPackAssociationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExtensionPackAssociationsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorCollectorsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorCollectorsResponse {
    #[serde(rename = "Collectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectors: Option<Vec<CollectorResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectorResponse {
    #[serde(rename = "CollectorHealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_health_check: Option<CollectorHealthCheck>,
    #[serde(rename = "CollectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_name: Option<String>,
    #[serde(rename = "CollectorReferencedId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_referenced_id: Option<String>,
    #[serde(rename = "CollectorVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_version: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InventoryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_data: Option<InventoryData>,
    #[serde(rename = "LastDataReceived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_data_received: Option<String>,
    #[serde(rename = "ModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
    #[serde(rename = "RegisteredDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_date: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "VersionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectorHealthCheck {
    #[serde(rename = "CollectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_status: Option<String>,
    #[serde(rename = "LocalCollectorS3Access")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_collector_s3_access: Option<bool>,
    #[serde(rename = "WebCollectorGrantedRoleBasedAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_collector_granted_role_based_access: Option<bool>,
    #[serde(rename = "WebCollectorS3Access")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_collector_s3_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryData {
    #[serde(rename = "NumberOfDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_databases: Option<i32>,
    #[serde(rename = "NumberOfSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_schemas: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorDatabasesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorDatabasesResponse {
    #[serde(rename = "Databases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseResponse {
    #[serde(rename = "Collectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectors: Option<Vec<CollectorShortInfoResponse>>,
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "NumberOfSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_schemas: Option<i64>,
    #[serde(rename = "Server")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerShortInfoResponse>,
    #[serde(rename = "SoftwareDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_details: Option<DatabaseInstanceSoftwareDetailsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectorShortInfoResponse {
    #[serde(rename = "CollectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_name: Option<String>,
    #[serde(rename = "CollectorReferencedId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_referenced_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerShortInfoResponse {
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseInstanceSoftwareDetailsResponse {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineEdition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_edition: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "OsArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<i32>,
    #[serde(rename = "ServicePack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
    #[serde(rename = "SupportLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_level: Option<String>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorLsaAnalysisRequest {
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorLsaAnalysisResponse {
    #[serde(rename = "Analysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Vec<FleetAdvisorLsaAnalysisResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FleetAdvisorLsaAnalysisResponse {
    #[serde(rename = "LsaAnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsa_analysis_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorSchemaObjectSummaryRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorSchemaObjectSummaryResponse {
    #[serde(rename = "FleetAdvisorSchemaObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_advisor_schema_objects: Option<Vec<FleetAdvisorSchemaObjectResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FleetAdvisorSchemaObjectResponse {
    #[serde(rename = "CodeLineCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_line_count: Option<i64>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "NumberOfObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_objects: Option<i64>,
    #[serde(rename = "ObjectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorSchemasRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetAdvisorSchemasResponse {
    #[serde(rename = "FleetAdvisorSchemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_advisor_schemas: Option<Vec<SchemaResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaResponse {
    #[serde(rename = "CodeLineCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_line_count: Option<i64>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "Complexity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<String>,
    #[serde(rename = "DatabaseInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_instance: Option<DatabaseShortInfoResponse>,
    #[serde(rename = "OriginalSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_schema: Option<SchemaShortInfoResponse>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "Server")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerShortInfoResponse>,
    #[serde(rename = "Similarity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseShortInfoResponse {
    #[serde(rename = "DatabaseEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_engine: Option<String>,
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "DatabaseIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_ip_address: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaShortInfoResponse {
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "DatabaseIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_ip_address: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "SchemaId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceProfilesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeInstanceProfilesResponse {
    #[serde(rename = "InstanceProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profiles: Option<Vec<InstanceProfile>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelAssessmentsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelAssessmentsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelChildrenMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelChildrenResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MetadataModelChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_model_children: Option<Vec<MetadataModelReference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataModelReference {
    #[serde(rename = "MetadataModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_model_name: Option<String>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelConversionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelConversionsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelCreationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelCreationsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelExportsAsScriptMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelExportsAsScriptResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelExportsToTargetMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelExportsToTargetResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelImportsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelImportsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Requests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<SchemaConversionRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetadataModelResponse {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "MetadataModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_model_name: Option<String>,
    #[serde(rename = "MetadataModelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_model_type: Option<String>,
    #[serde(rename = "TargetMetadataModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_metadata_models: Option<Vec<MetadataModelReference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMigrationProjectsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeMigrationProjectsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MigrationProjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_projects: Option<Vec<MigrationProject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrderableReplicationInstancesMessage {
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
pub struct DescribeOrderableReplicationInstancesResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OrderableReplicationInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_replication_instances: Option<Vec<OrderableReplicationInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderableReplicationInstance {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "DefaultAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allocated_storage: Option<i32>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IncludedAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_allocated_storage: Option<i32>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MinAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_allocated_storage: Option<i32>,
    #[serde(rename = "ReleaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_status: Option<String>,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePendingMaintenanceActionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePendingMaintenanceActionsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<Vec<ResourcePendingMaintenanceActions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommendationLimitationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommendationLimitationsResponse {
    #[serde(rename = "Limitations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitations: Option<Vec<Limitation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Limitation {
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "Impact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
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
pub struct DescribeRecommendationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommendationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Recommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Recommendation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RecommendationData>,
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "Preferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<RecommendationSettings>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationData {
    #[serde(rename = "RdsEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_engine: Option<RdsRecommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsRecommendation {
    #[serde(rename = "RequirementsToTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements_to_target: Option<RdsRequirements>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<RdsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsRequirements {
    #[serde(rename = "DeploymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    #[serde(rename = "EngineEdition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_edition: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "InstanceMemory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_memory: Option<f64>,
    #[serde(rename = "InstanceVcpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_vcpu: Option<f64>,
    #[serde(rename = "StorageIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_iops: Option<i32>,
    #[serde(rename = "StorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsConfiguration {
    #[serde(rename = "DeploymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    #[serde(rename = "EngineEdition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_edition: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "InstanceMemory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_memory: Option<f64>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "InstanceVcpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_vcpu: Option<f64>,
    #[serde(rename = "StorageIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_iops: Option<i32>,
    #[serde(rename = "StorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRefreshSchemasStatusMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRefreshSchemasStatusResponse {
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshSchemasStatus {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "LastRefreshDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_date: Option<f64>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationConfigsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationConfigsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configs: Option<Vec<ReplicationConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationInstanceTaskLogsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationInstanceTaskLogsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationInstanceTaskLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_logs: Option<Vec<ReplicationInstanceTaskLog>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationInstanceTaskLog {
    #[serde(rename = "ReplicationInstanceTaskLogSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_log_size: Option<i64>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "ReplicationTaskName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationInstancesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationInstancesResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instances: Option<Vec<ReplicationInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationSubnetGroupsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationSubnetGroupsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationSubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_groups: Option<Vec<ReplicationSubnetGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTableStatisticsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTableStatisticsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_arn: Option<String>,
    #[serde(rename = "ReplicationTableStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_table_statistics: Option<Vec<TableStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableStatistics {
    #[serde(rename = "AppliedDdls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_ddls: Option<i64>,
    #[serde(rename = "AppliedDeletes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_deletes: Option<i64>,
    #[serde(rename = "AppliedInserts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_inserts: Option<i64>,
    #[serde(rename = "AppliedUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_updates: Option<i64>,
    #[serde(rename = "Ddls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddls: Option<i64>,
    #[serde(rename = "Deletes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    #[serde(rename = "FullLoadCondtnlChkFailedRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_condtnl_chk_failed_rows: Option<i64>,
    #[serde(rename = "FullLoadEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_end_time: Option<f64>,
    #[serde(rename = "FullLoadErrorRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_rows: Option<i64>,
    #[serde(rename = "FullLoadReloaded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_reloaded: Option<bool>,
    #[serde(rename = "FullLoadRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_rows: Option<i64>,
    #[serde(rename = "FullLoadStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_start_time: Option<f64>,
    #[serde(rename = "Inserts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserts: Option<i64>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "ResyncProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resync_progress: Option<f64>,
    #[serde(rename = "ResyncRowsAttempted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resync_rows_attempted: Option<i64>,
    #[serde(rename = "ResyncRowsFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resync_rows_failed: Option<i64>,
    #[serde(rename = "ResyncRowsSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resync_rows_succeeded: Option<i64>,
    #[serde(rename = "ResyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resync_state: Option<String>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_state: Option<String>,
    #[serde(rename = "Updates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<i64>,
    #[serde(rename = "ValidationFailedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_failed_records: Option<i64>,
    #[serde(rename = "ValidationPendingRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_pending_records: Option<i64>,
    #[serde(rename = "ValidationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,
    #[serde(rename = "ValidationStateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state_details: Option<String>,
    #[serde(rename = "ValidationSuspendedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_suspended_records: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTaskAssessmentResultsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTaskAssessmentResultsResponse {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationTaskAssessmentResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_results: Option<Vec<ReplicationTaskAssessmentResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskAssessmentResult {
    #[serde(rename = "AssessmentResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results: Option<String>,
    #[serde(rename = "AssessmentResultsFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results_file: Option<String>,
    #[serde(rename = "AssessmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[serde(rename = "ReplicationTaskLastAssessmentDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_last_assessment_date: Option<f64>,
    #[serde(rename = "S3ObjectUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTaskAssessmentRunsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationTaskAssessmentRunsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationTaskAssessmentRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_runs: Option<Vec<ReplicationTaskAssessmentRun>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTaskIndividualAssessmentsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationTaskIndividualAssessmentsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationTaskIndividualAssessments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessments: Option<Vec<ReplicationTaskIndividualAssessment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationTaskIndividualAssessment {
    #[serde(rename = "IndividualAssessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_name: Option<String>,
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_arn: Option<String>,
    #[serde(rename = "ReplicationTaskIndividualAssessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessment_arn: Option<String>,
    #[serde(rename = "ReplicationTaskIndividualAssessmentStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessment_start_date: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTasksMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "WithoutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_settings: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationTasksResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_tasks: Option<Vec<ReplicationTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplicationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeReplicationsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Replications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replications: Option<Vec<Replication>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Replication {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "FailureMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_messages: Option<Vec<String>>,
    #[serde(rename = "IsReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "PremigrationAssessmentStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premigration_assessment_statuses: Option<Vec<PremigrationAssessmentStatus>>,
    #[serde(rename = "ProvisionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_data: Option<ProvisionData>,
    #[serde(rename = "RecoveryCheckpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint: Option<String>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_arn: Option<String>,
    #[serde(rename = "ReplicationConfigIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_identifier: Option<String>,
    #[serde(rename = "ReplicationCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_create_time: Option<f64>,
    #[serde(rename = "ReplicationDeprovisionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_deprovision_time: Option<f64>,
    #[serde(rename = "ReplicationLastStopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_last_stop_time: Option<f64>,
    #[serde(rename = "ReplicationStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_stats: Option<ReplicationStats>,
    #[serde(rename = "ReplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<String>,
    #[serde(rename = "ReplicationUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_update_time: Option<f64>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[serde(rename = "StartReplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_replication_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StopReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PremigrationAssessmentStatus {
    #[serde(rename = "AssessmentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_progress: Option<ReplicationTaskAssessmentRunProgress>,
    #[serde(rename = "FailOnAssessmentFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_assessment_failure: Option<bool>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "PremigrationAssessmentRunArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premigration_assessment_run_arn: Option<String>,
    #[serde(rename = "PremigrationAssessmentRunCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premigration_assessment_run_creation_date: Option<f64>,
    #[serde(rename = "ResultEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_encryption_mode: Option<String>,
    #[serde(rename = "ResultKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_kms_key_arn: Option<String>,
    #[serde(rename = "ResultLocationBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_bucket: Option<String>,
    #[serde(rename = "ResultLocationFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_folder: Option<String>,
    #[serde(rename = "ResultStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_statistic: Option<ReplicationTaskAssessmentRunResultStatistic>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionData {
    #[serde(rename = "DateNewProvisioningDataAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_new_provisioning_data_available: Option<f64>,
    #[serde(rename = "DateProvisioned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_provisioned: Option<f64>,
    #[serde(rename = "IsNewProvisioningAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_provisioning_available: Option<bool>,
    #[serde(rename = "ProvisionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_state: Option<String>,
    #[serde(rename = "ProvisionedCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity_units: Option<i32>,
    #[serde(rename = "ReasonForNewProvisioningData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_new_provisioning_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStats {
    #[serde(rename = "ElapsedTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis: Option<i64>,
    #[serde(rename = "FreshStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh_start_date: Option<f64>,
    #[serde(rename = "FullLoadFinishDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_finish_date: Option<f64>,
    #[serde(rename = "FullLoadProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_progress_percent: Option<i32>,
    #[serde(rename = "FullLoadStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_start_date: Option<f64>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "StopDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "TablesErrored")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_errored: Option<i32>,
    #[serde(rename = "TablesLoaded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loaded: Option<i32>,
    #[serde(rename = "TablesLoading")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loading: Option<i32>,
    #[serde(rename = "TablesQueued")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_queued: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSchemasMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
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
pub struct DescribeSchemasResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Schemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableStatisticsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableStatisticsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[serde(rename = "TableStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_statistics: Option<Vec<TableStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportMetadataModelAssessmentMessage {
    #[serde(rename = "AssessmentReportTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_report_types: Option<Vec<String>>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportMetadataModelAssessmentResponse {
    #[serde(rename = "CsvReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_report: Option<ExportMetadataModelAssessmentResultEntry>,
    #[serde(rename = "PdfReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_report: Option<ExportMetadataModelAssessmentResultEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportMetadataModelAssessmentResultEntry {
    #[serde(rename = "ObjectURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_u_r_l: Option<String>,
    #[serde(rename = "S3ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetSelectionRulesMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetSelectionRulesResponse {
    #[serde(rename = "TargetSelectionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateMessage {
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default)]
    pub certificate_identifier: String,
    #[serde(rename = "CertificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "CertificateWallet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<String>,
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
pub struct ImportCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyConversionConfigurationMessage {
    #[serde(rename = "ConversionConfiguration")]
    #[serde(default)]
    pub conversion_configuration: String,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyConversionConfigurationResponse {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDataMigrationMessage {
    #[serde(rename = "DataMigrationIdentifier")]
    #[serde(default)]
    pub data_migration_identifier: String,
    #[serde(rename = "DataMigrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_name: Option<String>,
    #[serde(rename = "DataMigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration_type: Option<String>,
    #[serde(rename = "EnableCloudwatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs: Option<bool>,
    #[serde(rename = "NumberOfJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_jobs: Option<i32>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_rules: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SourceDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data_settings: Option<Vec<SourceDataSetting>>,
    #[serde(rename = "TargetDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data_settings: Option<Vec<TargetDataSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDataMigrationResponse {
    #[serde(rename = "DataMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration: Option<DataMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDataProviderMessage {
    #[serde(rename = "DataProviderIdentifier")]
    #[serde(default)]
    pub data_provider_identifier: String,
    #[serde(rename = "DataProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "ExactSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_settings: Option<bool>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DataProviderSettings>,
    #[serde(rename = "Virtual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDataProviderResponse {
    #[serde(rename = "DataProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_provider: Option<DataProvider>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEndpointMessage {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DmsTransferSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    #[serde(rename = "DocDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    #[serde(rename = "DynamoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "ExactSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_settings: Option<bool>,
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[serde(rename = "GcpMySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_my_s_q_l_settings: Option<GcpMySQLSettings>,
    #[serde(rename = "IBMDb2Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_b_m_db2_settings: Option<IBMDb2Settings>,
    #[serde(rename = "KafkaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    #[serde(rename = "KinesisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_s_q_l_server_settings: Option<MicrosoftSQLServerSettings>,
    #[serde(rename = "MongoDbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[serde(rename = "MySQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_s_q_l_settings: Option<MySQLSettings>,
    #[serde(rename = "NeptuneSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    #[serde(rename = "OracleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_s_q_l_settings: Option<PostgreSQLSettings>,
    #[serde(rename = "RedisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_settings: Option<RedisSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    #[serde(rename = "S3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    #[serde(rename = "SybaseSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    #[serde(rename = "TimestreamSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream_settings: Option<TimestreamSettings>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEndpointResponse {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEventSubscriptionResponse {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyInstanceProfileMessage {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileIdentifier")]
    #[serde(default)]
    pub instance_profile_identifier: String,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "SubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_identifier: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyInstanceProfileResponse {
    #[serde(rename = "InstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyMigrationProjectMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceProfileIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_identifier: Option<String>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "MigrationProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project_name: Option<String>,
    #[serde(rename = "SchemaConversionApplicationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_conversion_application_attributes: Option<SCApplicationAttributes>,
    #[serde(rename = "SourceDataProviderDescriptors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data_provider_descriptors: Option<Vec<DataProviderDescriptorDefinition>>,
    #[serde(rename = "TargetDataProviderDescriptors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data_provider_descriptors: Option<Vec<DataProviderDescriptorDefinition>>,
    #[serde(rename = "TransformationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation_rules: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyMigrationProjectResponse {
    #[serde(rename = "MigrationProject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_project: Option<MigrationProject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationConfigMessage {
    #[serde(rename = "ComputeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_config: Option<ComputeConfig>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
    #[serde(rename = "ReplicationConfigIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_identifier: Option<String>,
    #[serde(rename = "ReplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_settings: Option<String>,
    #[serde(rename = "ReplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<String>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[serde(rename = "SupplementalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_settings: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationConfigResponse {
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationInstanceMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KerberosAuthenticationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_authentication_settings: Option<KerberosAuthenticationSettings>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationSubnetGroupMessage {
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    pub replication_subnet_group_identifier: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationSubnetGroupResponse {
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationTaskMessage {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "MigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    #[serde(rename = "TaskData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MoveReplicationTaskMessage {
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
    #[serde(rename = "TargetReplicationInstanceArn")]
    #[serde(default)]
    pub target_replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MoveReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootReplicationInstanceMessage {
    #[serde(rename = "ForceFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_failover: Option<bool>,
    #[serde(rename = "ForcePlannedFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_planned_failover: Option<bool>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshSchemasMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshSchemasResponse {
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReloadReplicationTablesMessage {
    #[serde(rename = "ReloadOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_option: Option<String>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
    #[serde(rename = "TablesToReload")]
    #[serde(default)]
    pub tables_to_reload: Vec<TableToReload>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableToReload {
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    pub schema_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReloadReplicationTablesResponse {
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReloadTablesMessage {
    #[serde(rename = "ReloadOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_option: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
    #[serde(rename = "TablesToReload")]
    #[serde(default)]
    pub tables_to_reload: Vec<TableToReload>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReloadTablesResponse {
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunFleetAdvisorLsaAnalysisResponse {
    #[serde(rename = "LsaAnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsa_analysis_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataMigrationMessage {
    #[serde(rename = "DataMigrationIdentifier")]
    #[serde(default)]
    pub data_migration_identifier: String,
    #[serde(rename = "StartType")]
    #[serde(default)]
    pub start_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataMigrationResponse {
    #[serde(rename = "DataMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration: Option<DataMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExtensionPackAssociationMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExtensionPackAssociationResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelAssessmentMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelAssessmentResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelConversionMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelConversionResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelCreationMessage {
    #[serde(rename = "MetadataModelName")]
    #[serde(default)]
    pub metadata_model_name: String,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: MetadataModelProperties,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataModelProperties {
    #[serde(rename = "StatementProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_properties: Option<StatementProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatementProperties {
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelCreationResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelExportAsScriptMessage {
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelExportAsScriptResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelExportToTargetMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "OverwriteExtensionPack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_extension_pack: Option<bool>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelExportToTargetResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelImportMessage {
    #[serde(rename = "MigrationProjectIdentifier")]
    #[serde(default)]
    pub migration_project_identifier: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "Refresh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    #[serde(rename = "SelectionRules")]
    #[serde(default)]
    pub selection_rules: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetadataModelImportResponse {
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRecommendationsRequest {
    #[serde(rename = "DatabaseId")]
    #[serde(default)]
    pub database_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: RecommendationSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationMessage {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "PremigrationAssessmentSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premigration_assessment_settings: Option<String>,
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
    #[serde(rename = "StartReplicationType")]
    #[serde(default)]
    pub start_replication_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationResponse {
    #[serde(rename = "Replication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<Replication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskAssessmentMessage {
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskAssessmentResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskAssessmentRunMessage {
    #[serde(rename = "AssessmentRunName")]
    #[serde(default)]
    pub assessment_run_name: String,
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "IncludeOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_only: Option<Vec<String>>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
    #[serde(rename = "ResultEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_encryption_mode: Option<String>,
    #[serde(rename = "ResultKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_kms_key_arn: Option<String>,
    #[serde(rename = "ResultLocationBucket")]
    #[serde(default)]
    pub result_location_bucket: String,
    #[serde(rename = "ResultLocationFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_folder: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(default)]
    pub service_access_role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskAssessmentRunResponse {
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskMessage {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
    #[serde(rename = "StartReplicationTaskType")]
    #[serde(default)]
    pub start_replication_task_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDataMigrationMessage {
    #[serde(rename = "DataMigrationIdentifier")]
    #[serde(default)]
    pub data_migration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDataMigrationResponse {
    #[serde(rename = "DataMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_migration: Option<DataMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationMessage {
    #[serde(rename = "ReplicationConfigArn")]
    #[serde(default)]
    pub replication_config_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationResponse {
    #[serde(rename = "Replication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<Replication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationTaskMessage {
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(default)]
    pub replication_task_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationTaskResponse {
    #[serde(rename = "ReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionMessage {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    pub replication_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionsToEventBridgeMessage {
    #[serde(rename = "ForceMove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_move: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionsToEventBridgeResponse {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
