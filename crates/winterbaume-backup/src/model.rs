//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-backup

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBackupVaultMpaApprovalTeamInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "MpaApprovalTeamArn")]
    #[serde(default)]
    pub mpa_approval_team_arn: String,
    #[serde(rename = "RequesterComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelLegalHoldInput {
    #[serde(rename = "CancelDescription")]
    #[serde(default)]
    pub cancel_description: String,
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    pub legal_hold_id: String,
    #[serde(rename = "RetainRecordInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_record_in_days: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelLegalHoldOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupPlanInput {
    #[serde(rename = "BackupPlan")]
    #[serde(default)]
    pub backup_plan: BackupPlanInput,
    #[serde(rename = "BackupPlanTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupPlanInput {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlanName")]
    #[serde(default)]
    pub backup_plan_name: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<BackupRuleInput>,
    #[serde(rename = "ScanSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_settings: Option<Vec<ScanSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedBackupSetting {
    #[serde(rename = "BackupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupRuleInput {
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<i64>,
    #[serde(rename = "CopyActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<CopyAction>>,
    #[serde(rename = "EnableContinuousBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_continuous_backup: Option<bool>,
    #[serde(rename = "IndexActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_actions: Option<Vec<IndexAction>>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "RecoveryPointTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "ScanActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_actions: Option<Vec<ScanAction>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
    #[serde(rename = "TargetBackupVaultName")]
    #[serde(default)]
    pub target_backup_vault_name: String,
    #[serde(rename = "TargetLogicallyAirGappedBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_logically_air_gapped_backup_vault_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyAction {
    #[serde(rename = "DestinationBackupVaultArn")]
    #[serde(default)]
    pub destination_backup_vault_arn: String,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Lifecycle {
    #[serde(rename = "DeleteAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_days: Option<i64>,
    #[serde(rename = "DeleteAfterEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_event: Option<String>,
    #[serde(rename = "MoveToColdStorageAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_after_days: Option<i64>,
    #[serde(rename = "OptInToArchiveForSupportedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_to_archive_for_supported_resources: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexAction {
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanAction {
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "ScanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanSetting {
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    #[serde(rename = "ScannerRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupPlanOutput {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupSelectionInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
    #[serde(rename = "BackupSelection")]
    #[serde(default)]
    pub backup_selection: BackupSelection,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupSelection {
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "ListOfTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_of_tags: Option<Vec<Condition>>,
    #[serde(rename = "NotResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_resources: Option<Vec<String>>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "SelectionName")]
    #[serde(default)]
    pub selection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Conditions {
    #[serde(rename = "StringEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_equals: Option<Vec<ConditionParameter>>,
    #[serde(rename = "StringLike")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_like: Option<Vec<ConditionParameter>>,
    #[serde(rename = "StringNotEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_not_equals: Option<Vec<ConditionParameter>>,
    #[serde(rename = "StringNotLike")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_not_like: Option<Vec<ConditionParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionParameter {
    #[serde(rename = "ConditionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_key: Option<String>,
    #[serde(rename = "ConditionValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "ConditionKey")]
    #[serde(default)]
    pub condition_key: String,
    #[serde(rename = "ConditionType")]
    #[serde(default)]
    pub condition_type: String,
    #[serde(rename = "ConditionValue")]
    #[serde(default)]
    pub condition_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupSelectionOutput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "SelectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupVaultInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "BackupVaultTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupVaultOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFrameworkInput {
    #[serde(rename = "FrameworkControls")]
    #[serde(default)]
    pub framework_controls: Vec<FrameworkControl>,
    #[serde(rename = "FrameworkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    pub framework_name: String,
    #[serde(rename = "FrameworkTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameworkControl {
    #[serde(rename = "ControlInputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_input_parameters: Option<Vec<ControlInputParameter>>,
    #[serde(rename = "ControlName")]
    #[serde(default)]
    pub control_name: String,
    #[serde(rename = "ControlScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_scope: Option<ControlScope>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlInputParameter {
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
pub struct ControlScope {
    #[serde(rename = "ComplianceResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_ids: Option<Vec<String>>,
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFrameworkOutput {
    #[serde(rename = "FrameworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arn: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLegalHoldInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_selection: Option<RecoveryPointSelection>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointSelection {
    #[serde(rename = "DateRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<String>>,
    #[serde(rename = "VaultNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateRange {
    #[serde(rename = "FromDate")]
    #[serde(default)]
    pub from_date: f64,
    #[serde(rename = "ToDate")]
    #[serde(default)]
    pub to_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLegalHoldOutput {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LegalHoldArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_arn: Option<String>,
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_id: Option<String>,
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_selection: Option<RecoveryPointSelection>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogicallyAirGappedBackupVaultInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "BackupVaultTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "MaxRetentionDays")]
    #[serde(default)]
    pub max_retention_days: i64,
    #[serde(rename = "MinRetentionDays")]
    #[serde(default)]
    pub min_retention_days: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogicallyAirGappedBackupVaultOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "VaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReportPlanInput {
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "ReportDeliveryChannel")]
    #[serde(default)]
    pub report_delivery_channel: ReportDeliveryChannel,
    #[serde(rename = "ReportPlanDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_description: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    pub report_plan_name: String,
    #[serde(rename = "ReportPlanTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ReportSetting")]
    #[serde(default)]
    pub report_setting: ReportSetting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDeliveryChannel {
    #[serde(rename = "Formats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportSetting {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "FrameworkArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arns: Option<Vec<String>>,
    #[serde(rename = "NumberOfFrameworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_frameworks: Option<i32>,
    #[serde(rename = "OrganizationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_units: Option<Vec<String>>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(rename = "ReportTemplate")]
    #[serde(default)]
    pub report_template: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReportPlanOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ReportPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_arn: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreAccessBackupVaultInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "BackupVaultTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "RequesterComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_comment: Option<String>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    pub source_backup_vault_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreAccessBackupVaultOutput {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "RestoreAccessBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_access_backup_vault_arn: Option<String>,
    #[serde(rename = "RestoreAccessBackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_access_backup_vault_name: Option<String>,
    #[serde(rename = "VaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreTestingPlanInput {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "RestoreTestingPlan")]
    #[serde(default)]
    pub restore_testing_plan: RestoreTestingPlanForCreate,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingPlanForCreate {
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    pub recovery_point_selection: RestoreTestingRecoveryPointSelection,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingRecoveryPointSelection {
    #[serde(rename = "Algorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "ExcludeVaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_vaults: Option<Vec<String>>,
    #[serde(rename = "IncludeVaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_vaults: Option<Vec<String>>,
    #[serde(rename = "RecoveryPointTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_types: Option<Vec<String>>,
    #[serde(rename = "SelectionWindowDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_window_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreTestingPlanOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreTestingSelectionInput {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
    #[serde(rename = "RestoreTestingSelection")]
    #[serde(default)]
    pub restore_testing_selection: RestoreTestingSelectionForCreate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingSelectionForCreate {
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "ProtectedResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_arns: Option<Vec<String>>,
    #[serde(rename = "ProtectedResourceConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_conditions: Option<ProtectedResourceConditions>,
    #[serde(rename = "ProtectedResourceType")]
    #[serde(default)]
    pub protected_resource_type: String,
    #[serde(rename = "RestoreMetadataOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    pub restore_testing_selection_name: String,
    #[serde(rename = "ValidationWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectedResourceConditions {
    #[serde(rename = "StringEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_equals: Option<Vec<KeyValue>>,
    #[serde(rename = "StringNotEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_not_equals: Option<Vec<KeyValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValue {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestoreTestingSelectionOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selection_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTieringConfigurationInput {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "TieringConfiguration")]
    #[serde(default)]
    pub tiering_configuration: TieringConfigurationInputForCreate,
    #[serde(rename = "TieringConfigurationTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TieringConfigurationInputForCreate {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "ResourceSelection")]
    #[serde(default)]
    pub resource_selection: Vec<ResourceSelection>,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    pub tiering_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSelection {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "Resources")]
    #[serde(default)]
    pub resources: Vec<String>,
    #[serde(rename = "TieringDownSettingsInDays")]
    #[serde(default)]
    pub tiering_down_settings_in_days: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTieringConfigurationOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "TieringConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_arn: Option<String>,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupPlanInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupPlanOutput {
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupSelectionInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
    #[serde(rename = "SelectionId")]
    #[serde(default)]
    pub selection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupVaultAccessPolicyInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupVaultInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupVaultLockConfigurationInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupVaultNotificationsInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFrameworkInput {
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    pub framework_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecoveryPointInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportPlanInput {
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    pub report_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRestoreTestingPlanInput {
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRestoreTestingSelectionInput {
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    pub restore_testing_selection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTieringConfigurationInput {
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    pub tiering_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTieringConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupJobInput {
    #[serde(rename = "BackupJobId")]
    #[serde(default)]
    pub backup_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupJobOutput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    #[serde(rename = "BackupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "BytesTransferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    #[serde(rename = "ChildJobsInState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_jobs_in_state: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "ExpectedCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<f64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "IsEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "NumberOfChildJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_child_jobs: Option<i64>,
    #[serde(rename = "ParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(rename = "PercentDone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_lifecycle: Option<Lifecycle>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StartBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_by: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VaultLockState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_lock_state: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointCreator {
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "BackupPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_name: Option<String>,
    #[serde(rename = "BackupPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_version: Option<String>,
    #[serde(rename = "BackupRuleCron")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_cron: Option<String>,
    #[serde(rename = "BackupRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_id: Option<String>,
    #[serde(rename = "BackupRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_name: Option<String>,
    #[serde(rename = "BackupRuleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupVaultInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupVaultOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
    #[serde(rename = "LatestMpaApprovalTeamUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_mpa_approval_team_update: Option<LatestMpaApprovalTeamUpdate>,
    #[serde(rename = "LockDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<f64>,
    #[serde(rename = "Locked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "MaxRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retention_days: Option<i64>,
    #[serde(rename = "MinRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_retention_days: Option<i64>,
    #[serde(rename = "MpaApprovalTeamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpa_approval_team_arn: Option<String>,
    #[serde(rename = "MpaSessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpa_session_arn: Option<String>,
    #[serde(rename = "NumberOfRecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recovery_points: Option<i64>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    #[serde(rename = "VaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_state: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatestMpaApprovalTeamUpdate {
    #[serde(rename = "ExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<f64>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "MpaSessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpa_session_arn: Option<String>,
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
pub struct DescribeCopyJobInput {
    #[serde(rename = "CopyJobId")]
    #[serde(default)]
    pub copy_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCopyJobOutput {
    #[serde(rename = "CopyJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job: Option<CopyJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyJob {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "ChildJobsInState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_jobs_in_state: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CompositeMemberIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_member_identifier: Option<String>,
    #[serde(rename = "CopyJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job_id: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    #[serde(rename = "CreatedByBackupJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_backup_job_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DestinationBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup_vault_arn: Option<String>,
    #[serde(rename = "DestinationEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_key_arn: Option<String>,
    #[serde(rename = "DestinationRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_recovery_point_arn: Option<String>,
    #[serde(rename = "DestinationRecoveryPointLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_recovery_point_lifecycle: Option<Lifecycle>,
    #[serde(rename = "DestinationVaultLockState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_vault_lock_state: Option<String>,
    #[serde(rename = "DestinationVaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_vault_type: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "NumberOfChildJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_child_jobs: Option<i64>,
    #[serde(rename = "ParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    #[serde(rename = "SourceRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_recovery_point_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFrameworkInput {
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    pub framework_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFrameworkOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "FrameworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arn: Option<String>,
    #[serde(rename = "FrameworkControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_controls: Option<Vec<FrameworkControl>>,
    #[serde(rename = "FrameworkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<String>,
    #[serde(rename = "FrameworkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_status: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalSettingsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalSettingsOutput {
    #[serde(rename = "GlobalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectedResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectedResourceOutput {
    #[serde(rename = "LastBackupTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_time: Option<f64>,
    #[serde(rename = "LastBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_vault_arn: Option<String>,
    #[serde(rename = "LastRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_arn: Option<String>,
    #[serde(rename = "LatestRestoreExecutionTimeMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restore_execution_time_minutes: Option<i64>,
    #[serde(rename = "LatestRestoreJobCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restore_job_creation_date: Option<f64>,
    #[serde(rename = "LatestRestoreRecoveryPointCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restore_recovery_point_creation_date: Option<f64>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecoveryPointInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecoveryPointOutput {
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CompositeMemberIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_member_identifier: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "IndexStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status_message: Option<String>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "IsEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "LastRestoreTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_time: Option<f64>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "ParentRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ScanResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_results: Option<Vec<ScanResult>>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculatedLifecycle {
    #[serde(rename = "DeleteAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<f64>,
    #[serde(rename = "MoveToColdStorageAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanResult {
    #[serde(rename = "Findings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<String>>,
    #[serde(rename = "LastScanTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan_timestamp: Option<f64>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "ScanJobState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_job_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionSettingsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegionSettingsOutput {
    #[serde(rename = "ResourceTypeManagementPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_management_preference: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "ResourceTypeOptInPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_opt_in_preference: Option<std::collections::HashMap<String, bool>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportJobInput {
    #[serde(rename = "ReportJobId")]
    #[serde(default)]
    pub report_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportJobOutput {
    #[serde(rename = "ReportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_job: Option<ReportJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportJob {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ReportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_destination: Option<ReportDestination>,
    #[serde(rename = "ReportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_job_id: Option<String>,
    #[serde(rename = "ReportPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_arn: Option<String>,
    #[serde(rename = "ReportTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_template: Option<String>,
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
pub struct ReportDestination {
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportPlanInput {
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    pub report_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportPlanOutput {
    #[serde(rename = "ReportPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan: Option<ReportPlan>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportPlan {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "LastAttemptedExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempted_execution_time: Option<f64>,
    #[serde(rename = "LastSuccessfulExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_execution_time: Option<f64>,
    #[serde(rename = "ReportDeliveryChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_delivery_channel: Option<ReportDeliveryChannel>,
    #[serde(rename = "ReportPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_arn: Option<String>,
    #[serde(rename = "ReportPlanDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_description: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_name: Option<String>,
    #[serde(rename = "ReportSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_setting: Option<ReportSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRestoreJobInput {
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    pub restore_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRestoreJobOutput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RestoreJobCreator>,
    #[serde(rename = "CreatedResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_resource_arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_status: Option<String>,
    #[serde(rename = "DeletionStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_status_message: Option<String>,
    #[serde(rename = "ExpectedCompletionTimeMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_time_minutes: Option<i64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "ParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(rename = "PercentDone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_creation_date: Option<f64>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
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
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(rename = "ValidationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreJobCreator {
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScanJobInput {
    #[serde(rename = "ScanJobId")]
    #[serde(default)]
    pub scan_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScanJobOutput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ScanJobCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ScanBaseRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_base_recovery_point_arn: Option<String>,
    #[serde(rename = "ScanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "ScanJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_job_id: Option<String>,
    #[serde(rename = "ScanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(rename = "ScanResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<ScanResultInfo>,
    #[serde(rename = "ScannerRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_role_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanJobCreator {
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "BackupPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_version: Option<String>,
    #[serde(rename = "BackupRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanResultInfo {
    #[serde(rename = "ScanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBackupVaultMpaApprovalTeamInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RequesterComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRecoveryPointFromParentInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRecoveryPointInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportBackupPlanTemplateInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportBackupPlanTemplateOutput {
    #[serde(rename = "BackupPlanTemplateJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_json: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanFromJSONInput {
    #[serde(rename = "BackupPlanTemplateJson")]
    #[serde(default)]
    pub backup_plan_template_json: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanFromJSONOutput {
    #[serde(rename = "BackupPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan: Option<BackupPlan>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupPlan {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<BackupRule>>,
    #[serde(rename = "ScanSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_settings: Option<Vec<ScanSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupRule {
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<i64>,
    #[serde(rename = "CopyActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<CopyAction>>,
    #[serde(rename = "EnableContinuousBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_continuous_backup: Option<bool>,
    #[serde(rename = "IndexActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_actions: Option<Vec<IndexAction>>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "RecoveryPointTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "ScanActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_actions: Option<Vec<ScanAction>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
    #[serde(rename = "TargetBackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_backup_vault_name: Option<String>,
    #[serde(rename = "TargetLogicallyAirGappedBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_logically_air_gapped_backup_vault_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanFromTemplateInput {
    #[serde(rename = "BackupPlanTemplateId")]
    #[serde(default)]
    pub backup_plan_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanFromTemplateOutput {
    #[serde(rename = "BackupPlanDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_document: Option<BackupPlan>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
    #[serde(rename = "MaxScheduledRunsPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_scheduled_runs_preview: Option<i32>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupPlanOutput {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan: Option<BackupPlan>,
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "ScheduledRunsPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_runs_preview: Option<Vec<ScheduledPlanExecutionMember>>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledPlanExecutionMember {
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<f64>,
    #[serde(rename = "RuleExecutionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_type: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupSelectionInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
    #[serde(rename = "SelectionId")]
    #[serde(default)]
    pub selection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupSelectionOutput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "BackupSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_selection: Option<BackupSelection>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "SelectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupVaultAccessPolicyInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupVaultAccessPolicyOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupVaultNotificationsInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackupVaultNotificationsOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_events: Option<Vec<String>>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "SNSTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLegalHoldInput {
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    pub legal_hold_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLegalHoldOutput {
    #[serde(rename = "CancelDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_description: Option<String>,
    #[serde(rename = "CancellationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LegalHoldArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_arn: Option<String>,
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_id: Option<String>,
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_selection: Option<RecoveryPointSelection>,
    #[serde(rename = "RetainRecordUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_record_until: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecoveryPointIndexDetailsInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecoveryPointIndexDetailsOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "IndexCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_completion_date: Option<f64>,
    #[serde(rename = "IndexCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_creation_date: Option<f64>,
    #[serde(rename = "IndexDeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_deletion_date: Option<f64>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "IndexStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status_message: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
    #[serde(rename = "TotalItemsIndexed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_indexed: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecoveryPointRestoreMetadataInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecoveryPointRestoreMetadataOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "RestoreMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreJobMetadataInput {
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    pub restore_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreJobMetadataOutput {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingInferredMetadataInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingInferredMetadataOutput {
    #[serde(rename = "InferredMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingPlanInput {
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingPlanOutput {
    #[serde(rename = "RestoreTestingPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan: Option<RestoreTestingPlanForGet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingPlanForGet {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "LastExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<f64>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_selection: Option<RestoreTestingRecoveryPointSelection>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingSelectionInput {
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    pub restore_testing_selection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestoreTestingSelectionOutput {
    #[serde(rename = "RestoreTestingSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selection: Option<RestoreTestingSelectionForGet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingSelectionForGet {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "ProtectedResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_arns: Option<Vec<String>>,
    #[serde(rename = "ProtectedResourceConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_conditions: Option<ProtectedResourceConditions>,
    #[serde(rename = "ProtectedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_type: Option<String>,
    #[serde(rename = "RestoreMetadataOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selection_name: Option<String>,
    #[serde(rename = "ValidationWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSupportedResourceTypesOutput {
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTieringConfigurationInput {
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    pub tiering_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTieringConfigurationOutput {
    #[serde(rename = "TieringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration: Option<TieringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TieringConfiguration {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ResourceSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_selection: Option<Vec<ResourceSelection>>,
    #[serde(rename = "TieringConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_arn: Option<String>,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupJobSummariesInput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupJobSummariesOutput {
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "BackupJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_summaries: Option<Vec<BackupJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupJobSummary {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
pub struct ListBackupJobsInput {
    #[serde(rename = "ByAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    #[serde(rename = "ByBackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_backup_vault_name: Option<String>,
    #[serde(rename = "ByCompleteAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_after: Option<f64>,
    #[serde(rename = "ByCompleteBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_before: Option<f64>,
    #[serde(rename = "ByCreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    #[serde(rename = "ByCreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    #[serde(rename = "ByMessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_message_category: Option<String>,
    #[serde(rename = "ByParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_parent_job_id: Option<String>,
    #[serde(rename = "ByResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    #[serde(rename = "ByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    #[serde(rename = "ByState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_state: Option<String>,
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
pub struct ListBackupJobsOutput {
    #[serde(rename = "BackupJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_jobs: Option<Vec<BackupJob>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupJob {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    #[serde(rename = "BackupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "BytesTransferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "ExpectedCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<f64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "IsEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "ParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(rename = "PercentDone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_lifecycle: Option<Lifecycle>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StartBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_by: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VaultLockState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_lock_state: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupPlanTemplatesInput {
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
pub struct ListBackupPlanTemplatesOutput {
    #[serde(rename = "BackupPlanTemplatesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_templates_list: Option<Vec<BackupPlanTemplatesListMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupPlanTemplatesListMember {
    #[serde(rename = "BackupPlanTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_id: Option<String>,
    #[serde(rename = "BackupPlanTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupPlanVersionsInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
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
pub struct ListBackupPlanVersionsOutput {
    #[serde(rename = "BackupPlanVersionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_versions_list: Option<Vec<BackupPlansListMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupPlansListMember {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "BackupPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupPlansInput {
    #[serde(rename = "IncludeDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
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
pub struct ListBackupPlansOutput {
    #[serde(rename = "BackupPlansList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plans_list: Option<Vec<BackupPlansListMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupSelectionsInput {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
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
pub struct ListBackupSelectionsOutput {
    #[serde(rename = "BackupSelectionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_selections_list: Option<Vec<BackupSelectionsListMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupSelectionsListMember {
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "SelectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
    #[serde(rename = "SelectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackupVaultsInput {
    #[serde(rename = "ByShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_shared: Option<bool>,
    #[serde(rename = "ByVaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_vault_type: Option<String>,
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
pub struct ListBackupVaultsOutput {
    #[serde(rename = "BackupVaultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_list: Option<Vec<BackupVaultListMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupVaultListMember {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
    #[serde(rename = "LockDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<f64>,
    #[serde(rename = "Locked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "MaxRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retention_days: Option<i64>,
    #[serde(rename = "MinRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_retention_days: Option<i64>,
    #[serde(rename = "NumberOfRecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recovery_points: Option<i64>,
    #[serde(rename = "VaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_state: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCopyJobSummariesInput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCopyJobSummariesOutput {
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "CopyJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job_summaries: Option<Vec<CopyJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyJobSummary {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_category: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
pub struct ListCopyJobsInput {
    #[serde(rename = "ByAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    #[serde(rename = "ByCompleteAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_after: Option<f64>,
    #[serde(rename = "ByCompleteBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_before: Option<f64>,
    #[serde(rename = "ByCreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    #[serde(rename = "ByCreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    #[serde(rename = "ByDestinationVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_destination_vault_arn: Option<String>,
    #[serde(rename = "ByMessageCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_message_category: Option<String>,
    #[serde(rename = "ByParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_parent_job_id: Option<String>,
    #[serde(rename = "ByResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    #[serde(rename = "ByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    #[serde(rename = "BySourceRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_source_recovery_point_arn: Option<String>,
    #[serde(rename = "ByState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_state: Option<String>,
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
pub struct ListCopyJobsOutput {
    #[serde(rename = "CopyJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_jobs: Option<Vec<CopyJob>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFrameworksInput {
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
pub struct ListFrameworksOutput {
    #[serde(rename = "Frameworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameworks: Option<Vec<Framework>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Framework {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "FrameworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arn: Option<String>,
    #[serde(rename = "FrameworkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<String>,
    #[serde(rename = "NumberOfControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_controls: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexedRecoveryPointsInput {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexedRecoveryPointsOutput {
    #[serde(rename = "IndexedRecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_recovery_points: Option<Vec<IndexedRecoveryPoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexedRecoveryPoint {
    #[serde(rename = "BackupCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_date: Option<f64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IndexCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_creation_date: Option<f64>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "IndexStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status_message: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLegalHoldsInput {
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
pub struct ListLegalHoldsOutput {
    #[serde(rename = "LegalHolds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_holds: Option<Vec<LegalHold>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LegalHold {
    #[serde(rename = "CancellationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LegalHoldArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_arn: Option<String>,
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectedResourcesByBackupVaultInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
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
pub struct ListProtectedResourcesByBackupVaultOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ProtectedResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectedResource {
    #[serde(rename = "LastBackupTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_time: Option<f64>,
    #[serde(rename = "LastBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_vault_arn: Option<String>,
    #[serde(rename = "LastRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectedResourcesInput {
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
pub struct ListProtectedResourcesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ProtectedResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecoveryPointsByBackupVaultInput {
    #[serde(rename = "BackupVaultAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_account_id: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "ByBackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_backup_plan_id: Option<String>,
    #[serde(rename = "ByCreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    #[serde(rename = "ByCreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    #[serde(rename = "ByParentRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_parent_recovery_point_arn: Option<String>,
    #[serde(rename = "ByResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    #[serde(rename = "ByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
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
pub struct ListRecoveryPointsByBackupVaultOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<Vec<RecoveryPointByBackupVault>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointByBackupVault {
    #[serde(rename = "AggregatedScanResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_scan_result: Option<AggregatedScanResult>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CompositeMemberIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_member_identifier: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "IndexStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status_message: Option<String>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "IsEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "LastRestoreTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_time: Option<f64>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "ParentRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatedScanResult {
    #[serde(rename = "FailedScan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_scan: Option<bool>,
    #[serde(rename = "Findings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<String>>,
    #[serde(rename = "LastComputed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_computed: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecoveryPointsByLegalHoldInput {
    #[serde(rename = "LegalHoldId")]
    #[serde(default)]
    pub legal_hold_id: String,
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
pub struct ListRecoveryPointsByLegalHoldOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<Vec<RecoveryPointMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointMember {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecoveryPointsByResourceInput {
    #[serde(rename = "ManagedByAWSBackupOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_a_w_s_backup_only: Option<bool>,
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
pub struct ListRecoveryPointsByResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecoveryPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<Vec<RecoveryPointByResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryPointByResource {
    #[serde(rename = "AggregatedScanResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_scan_result: Option<AggregatedScanResult>,
    #[serde(rename = "BackupSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "EncryptionKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_type: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "IndexStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status_message: Option<String>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "ParentRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "VaultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportJobsInput {
    #[serde(rename = "ByCreationAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_creation_after: Option<f64>,
    #[serde(rename = "ByCreationBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_creation_before: Option<f64>,
    #[serde(rename = "ByReportPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_report_plan_name: Option<String>,
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
pub struct ListReportJobsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_jobs: Option<Vec<ReportJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportPlansInput {
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
pub struct ListReportPlansOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plans: Option<Vec<ReportPlan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreAccessBackupVaultsInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
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
pub struct ListRestoreAccessBackupVaultsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreAccessBackupVaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_access_backup_vaults: Option<Vec<RestoreAccessBackupVaultListMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreAccessBackupVaultListMember {
    #[serde(rename = "ApprovalDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LatestRevokeRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revoke_request: Option<LatestRevokeRequest>,
    #[serde(rename = "RestoreAccessBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_access_backup_vault_arn: Option<String>,
    #[serde(rename = "VaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatestRevokeRequest {
    #[serde(rename = "ExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<f64>,
    #[serde(rename = "InitiationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_date: Option<f64>,
    #[serde(rename = "MpaSessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpa_session_arn: Option<String>,
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
pub struct ListRestoreJobSummariesInput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreJobSummariesOutput {
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_summaries: Option<Vec<RestoreJobSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreJobSummary {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
pub struct ListRestoreJobsByProtectedResourceInput {
    #[serde(rename = "ByRecoveryPointCreationDateAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_recovery_point_creation_date_after: Option<f64>,
    #[serde(rename = "ByRecoveryPointCreationDateBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_recovery_point_creation_date_before: Option<f64>,
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
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreJobsByProtectedResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_jobs: Option<Vec<RestoreJobsListMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreJobsListMember {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RestoreJobCreator>,
    #[serde(rename = "CreatedResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_resource_arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_status: Option<String>,
    #[serde(rename = "DeletionStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_status_message: Option<String>,
    #[serde(rename = "ExpectedCompletionTimeMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_time_minutes: Option<i64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "ParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(rename = "PercentDone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "RecoveryPointCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_creation_date: Option<f64>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
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
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(rename = "ValidationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreJobsInput {
    #[serde(rename = "ByAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    #[serde(rename = "ByCompleteAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_after: Option<f64>,
    #[serde(rename = "ByCompleteBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_before: Option<f64>,
    #[serde(rename = "ByCreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    #[serde(rename = "ByCreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    #[serde(rename = "ByParentJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_parent_job_id: Option<String>,
    #[serde(rename = "ByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    #[serde(rename = "ByRestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_restore_testing_plan_arn: Option<String>,
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
pub struct ListRestoreJobsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_jobs: Option<Vec<RestoreJobsListMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreTestingPlansInput {
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
pub struct ListRestoreTestingPlansOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreTestingPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plans: Option<Vec<RestoreTestingPlanForList>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingPlanForList {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<f64>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreTestingSelectionsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRestoreTestingSelectionsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RestoreTestingSelections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selections: Option<Vec<RestoreTestingSelectionForList>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingSelectionForList {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "ProtectedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_type: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selection_name: Option<String>,
    #[serde(rename = "ValidationWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScanJobSummariesInput {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ScanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_status: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScanJobSummariesOutput {
    #[serde(rename = "AggregationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_period: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_job_summaries: Option<Vec<ScanJobSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanJobSummary {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ScanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_status: Option<String>,
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
pub struct ListScanJobsInput {
    #[serde(rename = "ByAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    #[serde(rename = "ByBackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_backup_vault_name: Option<String>,
    #[serde(rename = "ByCompleteAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_after: Option<f64>,
    #[serde(rename = "ByCompleteBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_complete_before: Option<f64>,
    #[serde(rename = "ByMalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_malware_scanner: Option<String>,
    #[serde(rename = "ByRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_recovery_point_arn: Option<String>,
    #[serde(rename = "ByResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    #[serde(rename = "ByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    #[serde(rename = "ByScanResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_scan_result_status: Option<String>,
    #[serde(rename = "ByState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_state: Option<String>,
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
pub struct ListScanJobsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_jobs: Option<Vec<ScanJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanJob {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ScanJobCreator>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_scanner: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ScanBaseRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_base_recovery_point_arn: Option<String>,
    #[serde(rename = "ScanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "ScanJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_job_id: Option<String>,
    #[serde(rename = "ScanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(rename = "ScanResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<ScanResultInfo>,
    #[serde(rename = "ScannerRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_role_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsInput {
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
pub struct ListTagsOutput {
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
pub struct ListTieringConfigurationsInput {
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
pub struct ListTieringConfigurationsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TieringConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configurations: Option<Vec<TieringConfigurationsListMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TieringConfigurationsListMember {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "TieringConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_arn: Option<String>,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBackupVaultAccessPolicyInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBackupVaultLockConfigurationInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "ChangeableForDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeable_for_days: Option<i64>,
    #[serde(rename = "MaxRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retention_days: Option<i64>,
    #[serde(rename = "MinRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_retention_days: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBackupVaultNotificationsInput {
    #[serde(rename = "BackupVaultEvents")]
    #[serde(default)]
    pub backup_vault_events: Vec<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "SNSTopicArn")]
    #[serde(default)]
    pub s_n_s_topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRestoreValidationResultInput {
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    pub restore_job_id: String,
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    pub validation_status: String,
    #[serde(rename = "ValidationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeRestoreAccessBackupVaultInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "RequesterComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_comment: Option<String>,
    #[serde(rename = "RestoreAccessBackupVaultArn")]
    #[serde(default)]
    pub restore_access_backup_vault_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBackupJobInput {
    #[serde(rename = "BackupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "CompleteWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_window_minutes: Option<i64>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "LogicallyAirGappedBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logically_air_gapped_backup_vault_arn: Option<String>,
    #[serde(rename = "RecoveryPointTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "StartWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBackupJobOutput {
    #[serde(rename = "BackupJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCopyJobInput {
    #[serde(rename = "DestinationBackupVaultArn")]
    #[serde(default)]
    pub destination_backup_vault_arn: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
    #[serde(rename = "SourceBackupVaultName")]
    #[serde(default)]
    pub source_backup_vault_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCopyJobOutput {
    #[serde(rename = "CopyJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IsParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parent: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReportJobInput {
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    pub report_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReportJobOutput {
    #[serde(rename = "ReportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRestoreJobInput {
    #[serde(rename = "CopySourceTagsToRestoredResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_tags_to_restored_resource: Option<bool>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRestoreJobOutput {
    #[serde(rename = "RestoreJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartScanJobInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "MalwareScanner")]
    #[serde(default)]
    pub malware_scanner: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
    #[serde(rename = "ScanBaseRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_base_recovery_point_arn: Option<String>,
    #[serde(rename = "ScanMode")]
    #[serde(default)]
    pub scan_mode: String,
    #[serde(rename = "ScannerRoleArn")]
    #[serde(default)]
    pub scanner_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartScanJobOutput {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "ScanJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBackupJobInput {
    #[serde(rename = "BackupJobId")]
    #[serde(default)]
    pub backup_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeyList")]
    #[serde(default)]
    pub tag_key_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackupPlanInput {
    #[serde(rename = "BackupPlan")]
    #[serde(default)]
    pub backup_plan: BackupPlanInput,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    pub backup_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackupPlanOutput {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "ScanSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_settings: Option<Vec<ScanSetting>>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFrameworkInput {
    #[serde(rename = "FrameworkControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_controls: Option<Vec<FrameworkControl>>,
    #[serde(rename = "FrameworkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    pub framework_name: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFrameworkOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FrameworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arn: Option<String>,
    #[serde(rename = "FrameworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalSettingsInput {
    #[serde(rename = "GlobalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecoveryPointIndexSettingsInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "Index")]
    #[serde(default)]
    pub index: String,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecoveryPointIndexSettingsOutput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecoveryPointLifecycleInput {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    pub recovery_point_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecoveryPointLifecycleOutput {
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRegionSettingsInput {
    #[serde(rename = "ResourceTypeManagementPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_management_preference: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "ResourceTypeOptInPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_opt_in_preference: Option<std::collections::HashMap<String, bool>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportPlanInput {
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "ReportDeliveryChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_delivery_channel: Option<ReportDeliveryChannel>,
    #[serde(rename = "ReportPlanDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_description: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    pub report_plan_name: String,
    #[serde(rename = "ReportSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_setting: Option<ReportSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportPlanOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ReportPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_arn: Option<String>,
    #[serde(rename = "ReportPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRestoreTestingPlanInput {
    #[serde(rename = "RestoreTestingPlan")]
    #[serde(default)]
    pub restore_testing_plan: RestoreTestingPlanForUpdate,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingPlanForUpdate {
    #[serde(rename = "RecoveryPointSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_selection: Option<RestoreTestingRecoveryPointSelection>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<String>,
    #[serde(rename = "StartWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRestoreTestingPlanOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRestoreTestingSelectionInput {
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    pub restore_testing_plan_name: String,
    #[serde(rename = "RestoreTestingSelection")]
    #[serde(default)]
    pub restore_testing_selection: RestoreTestingSelectionForUpdate,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    pub restore_testing_selection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreTestingSelectionForUpdate {
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "ProtectedResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_arns: Option<Vec<String>>,
    #[serde(rename = "ProtectedResourceConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_conditions: Option<ProtectedResourceConditions>,
    #[serde(rename = "RestoreMetadataOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ValidationWindowHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_window_hours: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRestoreTestingSelectionOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "RestoreTestingPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_arn: Option<String>,
    #[serde(rename = "RestoreTestingPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_plan_name: Option<String>,
    #[serde(rename = "RestoreTestingSelectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_testing_selection_name: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTieringConfigurationInput {
    #[serde(rename = "TieringConfiguration")]
    #[serde(default)]
    pub tiering_configuration: TieringConfigurationInputForUpdate,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    pub tiering_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TieringConfigurationInputForUpdate {
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    pub backup_vault_name: String,
    #[serde(rename = "ResourceSelection")]
    #[serde(default)]
    pub resource_selection: Vec<ResourceSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTieringConfigurationOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "TieringConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_arn: Option<String>,
    #[serde(rename = "TieringConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_configuration_name: Option<String>,
}
