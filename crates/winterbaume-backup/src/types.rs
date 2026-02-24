use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A backup selection associated with a plan.
#[derive(Debug, Clone)]
pub struct BackupSelectionData {
    pub selection_id: String,
    pub backup_plan_id: String,
    pub selection_name: String,
    pub iam_role_arn: String,
    pub resources: Vec<String>,
    pub creation_date: DateTime<Utc>,
    /// Full selection JSON from the request (for round-tripping).
    pub selection_json: serde_json::Value,
}

/// A recovery point stored in a vault.
#[derive(Debug, Clone)]
pub struct RecoveryPointData {
    pub recovery_point_arn: String,
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub resource_arn: String,
    pub resource_type: String,
    pub iam_role_arn: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub backup_size_bytes: i64,
    pub account_id: String,
}

/// A backup job.
#[derive(Debug, Clone)]
pub struct BackupJobData {
    pub backup_job_id: String,
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub recovery_point_arn: String,
    pub resource_arn: String,
    pub resource_type: String,
    pub iam_role_arn: String,
    pub state: String,
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub account_id: String,
}

/// A backup vault.
#[derive(Debug, Clone)]
pub struct BackupVault {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub creation_date: DateTime<Utc>,
    pub number_of_recovery_points: i64,
    pub locked: bool,
    pub min_retention_days: Option<i64>,
    pub max_retention_days: Option<i64>,
    pub lock_date: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
}

/// A backup plan.
#[derive(Debug, Clone)]
pub struct BackupPlanData {
    pub backup_plan_id: String,
    pub backup_plan_arn: String,
    pub backup_plan_name: String,
    pub version_id: String,
    pub creation_date: DateTime<Utc>,
    pub backup_plan_json: serde_json::Value,
    pub tags: HashMap<String, String>,
}

/// A report plan.
#[derive(Debug, Clone)]
pub struct ReportPlanData {
    pub report_plan_name: String,
    pub report_plan_arn: String,
    pub report_plan_description: String,
    pub report_delivery_channel: serde_json::Value,
    pub report_setting: serde_json::Value,
    pub creation_time: DateTime<Utc>,
    pub deployment_status: String,
    pub tags: HashMap<String, String>,
}

/// Vault access policy.
#[derive(Debug, Clone)]
pub struct VaultAccessPolicy {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub policy: String,
}

/// Vault notification configuration.
#[derive(Debug, Clone)]
pub struct VaultNotificationConfig {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub sns_topic_arn: String,
    pub backup_vault_events: Vec<String>,
}

/// An audit framework.
#[derive(Debug, Clone)]
pub struct FrameworkData {
    pub framework_name: String,
    pub framework_arn: String,
    pub framework_description: String,
    pub framework_controls: serde_json::Value,
    pub creation_time: DateTime<Utc>,
    pub deployment_status: String,
    pub number_of_controls: i32,
}

/// Global settings (account-level).
#[derive(Debug, Clone, Default)]
pub struct GlobalSettings {
    pub global_settings: HashMap<String, String>,
}

/// Region settings.
#[derive(Debug, Clone, Default)]
pub struct RegionSettings {
    pub resource_type_opt_in_preference: HashMap<String, bool>,
    pub resource_type_management_preference: HashMap<String, bool>,
}

/// A report job.
#[derive(Debug, Clone)]
pub struct ReportJobData {
    pub report_job_id: String,
    pub report_plan_arn: String,
    pub report_template: String,
    pub creation_time: DateTime<Utc>,
    pub completion_time: Option<DateTime<Utc>>,
    pub status: String,
}

/// A scan job.
#[derive(Debug, Clone)]
pub struct ScanJobData {
    pub scan_job_id: String,
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub recovery_point_arn: String,
    pub iam_role_arn: String,
    pub malware_scanner: String,
    pub scan_mode: String,
    pub scanner_role_arn: String,
    pub scan_base_recovery_point_arn: Option<String>,
    pub state: String,
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub account_id: String,
}

/// A tiering configuration.
#[derive(Debug, Clone)]
pub struct TieringConfigData {
    pub tiering_configuration_name: String,
    pub tiering_configuration_arn: String,
    pub backup_vault_name: String,
    pub resource_selection: serde_json::Value,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub creator_request_id: Option<String>,
    pub tags: HashMap<String, String>,
}

/// A legal hold.
#[derive(Debug, Clone)]
pub struct LegalHoldData {
    pub legal_hold_id: String,
    pub legal_hold_arn: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub cancellation_date: Option<DateTime<Utc>>,
    pub recovery_point_selection: serde_json::Value,
    pub tags: HashMap<String, String>,
}

/// A copy job.
#[derive(Debug, Clone)]
pub struct CopyJobData {
    pub copy_job_id: String,
    pub source_backup_vault_name: String,
    pub source_backup_vault_arn: String,
    pub source_recovery_point_arn: String,
    pub destination_backup_vault_arn: String,
    pub destination_recovery_point_arn: String,
    pub resource_arn: String,
    pub resource_type: String,
    pub iam_role_arn: String,
    pub state: String,
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub account_id: String,
}

/// A restore job.
#[derive(Debug, Clone)]
pub struct RestoreJobData {
    pub restore_job_id: String,
    pub recovery_point_arn: String,
    pub resource_type: String,
    pub iam_role_arn: String,
    pub status: String,
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub backup_size_in_bytes: i64,
    pub account_id: String,
    pub metadata: HashMap<String, String>,
    pub validation_status: Option<String>,
    pub validation_status_message: Option<String>,
}

/// A restore testing plan.
#[derive(Debug, Clone)]
pub struct RestoreTestingPlanData {
    pub restore_testing_plan_name: String,
    pub restore_testing_plan_arn: String,
    pub schedule_expression: String,
    pub schedule_expression_timezone: Option<String>,
    pub start_window_hours: Option<i32>,
    pub recovery_point_selection: serde_json::Value,
    pub creator_request_id: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A restore testing selection.
#[derive(Debug, Clone)]
pub struct RestoreTestingSelectionData {
    pub restore_testing_selection_name: String,
    pub restore_testing_plan_name: String,
    pub restore_testing_plan_arn: String,
    pub iam_role_arn: String,
    pub protected_resource_type: String,
    pub protected_resource_arns: Vec<String>,
    pub protected_resource_conditions: serde_json::Value,
    pub restore_metadata_overrides: HashMap<String, String>,
    pub validation_window_hours: Option<i32>,
    pub creator_request_id: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
}
