use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Overview of an association's last execution.
#[derive(Debug, Clone, Default)]
pub struct AssociationOverview {
    pub status: String,
    pub detailed_status: String,
}

/// An SSM parameter.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: String, // String, StringList, SecureString
    pub value: String,
    pub version: i64,
    pub last_modified_date: DateTime<Utc>,
    pub arn: String,
    pub data_type: String,
    pub tags: HashMap<String, String>,
}

/// A snapshot of a parameter at a specific version.
#[derive(Debug, Clone)]
pub struct ParameterVersion {
    pub name: String,
    pub r#type: String,
    pub value: String,
    pub version: i64,
    pub last_modified_date: DateTime<Utc>,
    pub labels: Vec<String>,
}

/// An SSM document.
#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub content: String,
    pub document_type: String,
    pub document_format: String,
    pub status: String,
    pub owner: String,
    pub default_version: String,
    pub latest_version: String,
    pub versions: Vec<DocumentVersion>,
    pub account_permissions: Vec<String>,
    pub created_date: DateTime<Utc>,
}

/// A version of an SSM document.
#[derive(Debug, Clone)]
pub struct DocumentVersion {
    pub version_name: String,
    pub document_version: String,
    pub content: String,
    pub created_date: DateTime<Utc>,
    pub status: String,
}

/// An SSM maintenance window.
#[derive(Debug, Clone)]
pub struct MaintenanceWindow {
    pub window_id: String,
    pub name: String,
    pub schedule: String,
    pub duration: i64,
    pub cutoff: i64,
    pub enabled: bool,
    pub targets: Vec<MaintenanceWindowTarget>,
    pub tasks: Vec<MaintenanceWindowTask>,
}

/// A target registered with a maintenance window.
#[derive(Debug, Clone)]
pub struct MaintenanceWindowTarget {
    pub window_target_id: String,
    pub window_id: String,
    pub resource_type: String,
    pub targets: Vec<Target>,
}

/// A task registered with a maintenance window.
#[derive(Debug, Clone)]
pub struct MaintenanceWindowTask {
    pub window_task_id: String,
    pub window_id: String,
    pub task_arn: String,
    pub task_type: String,
    pub targets: Vec<Target>,
}

/// A key/values target specification used for targeting instances.
#[derive(Debug, Clone)]
pub struct Target {
    pub key: String,
    pub values: Vec<String>,
}

/// An SSM patch baseline.
#[derive(Debug, Clone)]
pub struct PatchBaseline {
    pub baseline_id: String,
    pub name: String,
    pub operating_system: String,
    pub description: Option<String>,
    pub patch_groups: Vec<String>,
}

/// An SSM command (SendCommand result).
#[derive(Debug, Clone)]
pub struct Command {
    pub command_id: String,
    pub instance_ids: Vec<String>,
    pub document_name: String,
    pub status: String,
    pub requested_date_time: DateTime<Utc>,
    pub parameters: HashMap<String, Vec<String>>,
}

/// An SSM State Manager association.
#[derive(Debug, Clone)]
pub struct Association {
    pub association_id: String,
    pub association_name: Option<String>,
    pub name: String,
    pub document_version: Option<String>,
    pub targets: Vec<Target>,
    pub schedule_expression: Option<String>,
    pub parameters: HashMap<String, Vec<String>>,
    pub overview: AssociationOverview,
    pub last_execution_date: Option<DateTime<Utc>>,
    pub association_version: String,
    pub created_date: DateTime<Utc>,
}

/// An SSM OpsItem.
#[derive(Debug, Clone)]
pub struct OpsItem {
    pub ops_item_id: String,
    pub title: String,
    pub description: Option<String>,
    pub source: String,
    pub status: String,
    pub priority: Option<i32>,
    pub category: Option<String>,
    pub severity: Option<String>,
    pub ops_item_type: Option<String>,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
}

/// An SSM Session Manager session.
#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: String,
    pub target: String,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub document_name: Option<String>,
}

/// An SSM managed instance activation.
#[derive(Debug, Clone)]
pub struct Activation {
    pub activation_id: String,
    pub activation_code: String,
    pub iam_role: String,
    pub description: Option<String>,
    pub default_instance_name: Option<String>,
    pub registration_limit: Option<i32>,
    pub registrations_count: i32,
    pub expiration_date: Option<DateTime<Utc>>,
    pub expired: bool,
    pub created_date: DateTime<Utc>,
}

/// A resource policy entry.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy_id: String,
    pub policy_hash: String,
    pub policy: String,
}

/// A service setting entry.
#[derive(Debug, Clone)]
pub struct ServiceSetting {
    pub setting_id: String,
    pub setting_value: String,
    pub last_modified_time: DateTime<Utc>,
    pub arn: String,
}

/// A single inventory item stored for an instance.
#[derive(Debug, Clone)]
pub struct InventoryData {
    pub instance_id: String,
    pub type_name: String,
    pub capture_time: String,
    pub schema_version: String,
    pub content: Vec<HashMap<String, String>>,
}

/// A compliance item stored for a resource.
#[derive(Debug, Clone)]
pub struct ComplianceRecord {
    pub compliance_type: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub severity: String,
    pub execution_type: String,
    pub execution_id: String,
    pub execution_time: DateTime<Utc>,
    pub item_id: String,
    pub title: String,
    pub details: HashMap<String, String>,
}

/// An OpsMetadata entry.
#[derive(Debug, Clone)]
pub struct OpsMetadataEntry {
    pub ops_metadata_arn: String,
    pub resource_id: String,
    pub metadata: HashMap<String, String>,
    pub created_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
}

/// A Resource Data Sync configuration.
#[derive(Debug, Clone)]
pub struct ResourceDataSyncEntry {
    pub sync_name: String,
    pub sync_type: String,
    pub s3_destination_bucket: String,
    pub s3_destination_region: String,
    pub s3_destination_prefix: Option<String>,
    pub created_time: DateTime<Utc>,
    pub last_status: String,
}

/// An OpsItem related-item association.
#[derive(Debug, Clone)]
pub struct OpsItemRelatedItem {
    pub association_id: String,
    pub ops_item_id: String,
    pub association_type: String,
    pub resource_type: String,
    pub resource_uri: String,
    pub created_time: DateTime<Utc>,
}

/// A managed instance registered through hybrid activation.
#[derive(Debug, Clone)]
pub struct ManagedInstance {
    pub instance_id: String,
    pub ping_status: String,
    pub platform_type: String,
    pub platform_name: String,
    pub platform_version: String,
    pub activation_id: Option<String>,
    pub iam_role: Option<String>,
    pub registration_date: DateTime<Utc>,
    pub resource_type: String,
    pub ip_address: String,
    pub computer_name: String,
    pub is_latest_version: bool,
    pub last_ping_date_time: DateTime<Utc>,
}

/// An inventory deletion status record.
#[derive(Debug, Clone)]
pub struct InventoryDeletion {
    pub deletion_id: String,
    pub type_name: String,
    pub deletion_start_time: DateTime<Utc>,
    pub last_status: String,
    pub last_status_message: String,
}

/// A maintenance window execution record.
#[derive(Debug, Clone)]
pub struct MaintenanceWindowExecution {
    pub window_execution_id: String,
    pub window_id: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub tasks: Vec<MaintenanceWindowExecutionTask>,
}

/// A task execution within a maintenance window execution.
#[derive(Debug, Clone)]
pub struct MaintenanceWindowExecutionTask {
    pub window_execution_id: String,
    pub task_execution_id: String,
    pub window_task_id: String,
    pub task_arn: String,
    pub task_type: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub invocations: Vec<MaintenanceWindowExecutionTaskInvocation>,
}

/// An invocation within a maintenance window task execution.
#[derive(Debug, Clone)]
pub struct MaintenanceWindowExecutionTaskInvocation {
    pub invocation_id: String,
    pub window_execution_id: String,
    pub task_execution_id: String,
    pub execution_id: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Document review metadata.
#[derive(Debug, Clone)]
pub struct DocumentMetadataEntry {
    pub reviewer: String,
    pub status: String,
    pub comment: String,
    pub updated_date: DateTime<Utc>,
}
