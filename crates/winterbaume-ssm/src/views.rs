//! Serde-compatible view types for SSM state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SsmService;
use crate::state::SsmState;
use crate::types::{
    Activation, Association, AssociationOverview, Command, ComplianceRecord, Document,
    DocumentMetadataEntry, DocumentVersion, InventoryData, InventoryDeletion, MaintenanceWindow,
    MaintenanceWindowExecution, MaintenanceWindowExecutionTask,
    MaintenanceWindowExecutionTaskInvocation, MaintenanceWindowTarget, MaintenanceWindowTask,
    ManagedInstance, OpsItem, OpsItemRelatedItem, OpsMetadataEntry, Parameter, ParameterVersion,
    PatchBaseline, ResourceDataSyncEntry, ResourcePolicy, ServiceSetting, Session, Target,
};

/// Serializable view of the entire SSM state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SsmStateView {
    /// Parameters keyed by parameter name.
    #[serde(default)]
    pub parameters: HashMap<String, ParameterView>,
    /// Parameter version history keyed by parameter name.
    #[serde(default)]
    pub parameter_versions: HashMap<String, Vec<ParameterVersionView>>,
    /// Documents keyed by document name.
    #[serde(default)]
    pub documents: HashMap<String, DocumentView>,
    /// Maintenance windows keyed by window ID.
    #[serde(default)]
    pub maintenance_windows: HashMap<String, MaintenanceWindowView>,
    /// Patch baselines keyed by baseline ID.
    #[serde(default)]
    pub patch_baselines: HashMap<String, PatchBaselineView>,
    /// Default patch baseline IDs keyed by operating system name.
    #[serde(default)]
    pub default_patch_baselines: HashMap<String, String>,
    /// Commands keyed by command ID.
    #[serde(default)]
    pub commands: HashMap<String, CommandView>,
    /// Associations keyed by association ID.
    #[serde(default)]
    pub associations: HashMap<String, AssociationView>,
    /// Tags for arbitrary resources keyed by resource ARN or resource ID.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// OpsItems keyed by ops_item_id.
    #[serde(default)]
    pub ops_items: HashMap<String, OpsItemView>,
    /// Sessions keyed by session_id.
    #[serde(default)]
    pub sessions: HashMap<String, SessionView>,
    /// Activations keyed by activation_id.
    #[serde(default)]
    pub activations: HashMap<String, ActivationView>,
    /// Resource policies keyed by resource ARN.
    #[serde(default)]
    pub resource_policies: HashMap<String, Vec<ResourcePolicyView>>,
    /// Service settings keyed by setting_id.
    #[serde(default)]
    pub service_settings: HashMap<String, ServiceSettingView>,
    /// Inventory data keyed by (instance_id, type_name) encoded as "instance_id::type_name".
    #[serde(default)]
    pub inventory: HashMap<String, InventoryDataView>,
    /// Compliance records keyed by "resource_id::compliance_type::item_id".
    #[serde(default)]
    pub compliance_records: HashMap<String, ComplianceRecordView>,
    /// OpsMetadata entries keyed by ops_metadata_arn.
    #[serde(default)]
    pub ops_metadata: HashMap<String, OpsMetadataView>,
    /// Resource data sync configurations keyed by sync_name.
    #[serde(default)]
    pub resource_data_syncs: HashMap<String, ResourceDataSyncView>,
    /// OpsItem related-item associations keyed by association_id.
    #[serde(default)]
    pub ops_item_related_items: HashMap<String, OpsItemRelatedItemView>,
    /// Managed instances keyed by instance_id.
    #[serde(default)]
    pub managed_instances: HashMap<String, ManagedInstanceView>,
    /// Inventory deletion records keyed by deletion_id.
    #[serde(default)]
    pub inventory_deletions: HashMap<String, InventoryDeletionView>,
    /// Maintenance window execution records keyed by window_execution_id.
    #[serde(default)]
    pub maintenance_window_executions: HashMap<String, MaintenanceWindowExecutionView>,
    /// Document review metadata keyed by document name.
    #[serde(default)]
    pub document_metadata: HashMap<String, Vec<DocumentMetadataEntryView>>,
}

/// Serializable view of an SSM parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterView {
    pub name: String,
    pub r#type: String,
    pub value: String,
    pub version: i64,
    pub last_modified_date: Option<String>,
    pub arn: String,
    pub data_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a parameter version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterVersionView {
    pub name: String,
    pub r#type: String,
    pub value: String,
    pub version: i64,
    pub last_modified_date: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Serializable view of an SSM document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentView {
    pub name: String,
    pub content: String,
    pub document_type: String,
    pub document_format: String,
    pub status: String,
    pub owner: String,
    pub default_version: String,
    pub latest_version: String,
    #[serde(default)]
    pub versions: Vec<DocumentVersionView>,
    #[serde(default)]
    pub account_permissions: Vec<String>,
    pub created_date: Option<String>,
}

/// Serializable view of a document version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersionView {
    pub version_name: String,
    pub document_version: String,
    pub content: String,
    pub created_date: Option<String>,
    pub status: String,
}

/// Serializable view of an SSM maintenance window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowView {
    pub window_id: String,
    pub name: String,
    pub schedule: String,
    pub duration: i64,
    pub cutoff: i64,
    pub enabled: bool,
    #[serde(default)]
    pub targets: Vec<MaintenanceWindowTargetView>,
    #[serde(default)]
    pub tasks: Vec<MaintenanceWindowTaskView>,
}

/// Serializable view of a maintenance window target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowTargetView {
    pub window_target_id: String,
    pub window_id: String,
    pub resource_type: String,
    #[serde(default)]
    pub targets: Vec<TargetView>,
}

/// Serializable view of a maintenance window task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskView {
    pub window_task_id: String,
    pub window_id: String,
    pub task_arn: String,
    pub task_type: String,
    #[serde(default)]
    pub targets: Vec<TargetView>,
}

/// Serializable view of a target specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetView {
    pub key: String,
    #[serde(default)]
    pub values: Vec<String>,
}

/// Serializable view of an SSM patch baseline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchBaselineView {
    pub baseline_id: String,
    pub name: String,
    pub operating_system: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub patch_groups: Vec<String>,
}

/// Serializable view of an SSM command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandView {
    pub command_id: String,
    #[serde(default)]
    pub instance_ids: Vec<String>,
    pub document_name: String,
    pub status: String,
    pub requested_date_time: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, Vec<String>>,
}

/// Serializable view of an SSM State Manager association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociationView {
    pub association_id: String,
    pub association_name: Option<String>,
    pub name: String,
    pub document_version: Option<String>,
    #[serde(default)]
    pub targets: Vec<TargetView>,
    pub schedule_expression: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, Vec<String>>,
    pub status: String,
    pub detailed_status: String,
    pub last_execution_date: Option<String>,
    pub association_version: String,
    pub created_date: Option<String>,
}

/// Serializable view of an SSM OpsItem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsItemView {
    pub ops_item_id: String,
    pub title: String,
    pub description: Option<String>,
    pub source: String,
    pub status: String,
    pub priority: Option<i32>,
    pub category: Option<String>,
    pub severity: Option<String>,
    pub ops_item_type: Option<String>,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
}

/// Serializable view of an SSM Session Manager session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionView {
    pub session_id: String,
    pub target: String,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub document_name: Option<String>,
}

/// Serializable view of an SSM managed instance activation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationView {
    pub activation_id: String,
    pub activation_code: String,
    pub iam_role: String,
    pub description: Option<String>,
    pub default_instance_name: Option<String>,
    pub registration_limit: Option<i32>,
    pub registrations_count: i32,
    pub expiration_date: Option<String>,
    pub expired: bool,
    pub created_date: Option<String>,
}

/// Serializable view of a resource policy entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy_id: String,
    pub policy_hash: String,
    pub policy: String,
}

/// Serializable view of a service setting entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSettingView {
    pub setting_id: String,
    pub setting_value: String,
    pub last_modified_time: Option<String>,
    pub arn: String,
}

/// Serializable view of an SSM inventory data entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryDataView {
    pub instance_id: String,
    pub type_name: String,
    pub capture_time: String,
    pub schema_version: String,
    #[serde(default)]
    pub content: Vec<HashMap<String, String>>,
}

/// Serializable view of a compliance record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRecordView {
    pub compliance_type: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub severity: String,
    pub execution_type: String,
    pub execution_id: String,
    pub execution_time: Option<String>,
    pub item_id: String,
    pub title: String,
    #[serde(default)]
    pub details: HashMap<String, String>,
}

/// Serializable view of an OpsMetadata entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsMetadataView {
    pub ops_metadata_arn: String,
    pub resource_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    pub created_date: Option<String>,
    pub last_modified_date: Option<String>,
}

/// Serializable view of a Resource Data Sync configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDataSyncView {
    pub sync_name: String,
    pub sync_type: String,
    pub s3_destination_bucket: String,
    pub s3_destination_region: String,
    pub s3_destination_prefix: Option<String>,
    pub created_time: Option<String>,
    pub last_status: String,
}

/// Serializable view of an OpsItem related-item association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsItemRelatedItemView {
    pub association_id: String,
    pub ops_item_id: String,
    pub association_type: String,
    pub resource_type: String,
    pub resource_uri: String,
    pub created_time: Option<String>,
}

/// Serializable view of a managed instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedInstanceView {
    pub instance_id: String,
    pub ping_status: String,
    pub platform_type: String,
    pub platform_name: String,
    pub platform_version: String,
    pub activation_id: Option<String>,
    pub iam_role: Option<String>,
    pub registration_date: Option<String>,
    pub resource_type: String,
    pub ip_address: String,
    pub computer_name: String,
    pub is_latest_version: bool,
    pub last_ping_date_time: Option<String>,
}

/// Serializable view of an inventory deletion record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryDeletionView {
    pub deletion_id: String,
    pub type_name: String,
    pub deletion_start_time: Option<String>,
    pub last_status: String,
    pub last_status_message: String,
}

/// Serializable view of a maintenance window execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowExecutionView {
    pub window_execution_id: String,
    pub window_id: String,
    pub status: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    #[serde(default)]
    pub tasks: Vec<MaintenanceWindowExecutionTaskViewEntry>,
}

/// Serializable view of a maintenance window execution task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowExecutionTaskViewEntry {
    pub window_execution_id: String,
    pub task_execution_id: String,
    pub window_task_id: String,
    pub task_arn: String,
    pub task_type: String,
    pub status: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    #[serde(default)]
    pub invocations: Vec<MaintenanceWindowExecutionTaskInvocationView>,
}

/// Serializable view of a maintenance window execution task invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindowExecutionTaskInvocationView {
    pub invocation_id: String,
    pub window_execution_id: String,
    pub task_execution_id: String,
    pub execution_id: String,
    pub status: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Serializable view of document review metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadataEntryView {
    pub reviewer: String,
    pub status: String,
    pub comment: String,
    pub updated_date: Option<String>,
}

// --- From internal types to view types ---

impl From<&SsmState> for SsmStateView {
    fn from(state: &SsmState) -> Self {
        SsmStateView {
            parameters: state
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), ParameterView::from(v)))
                .collect(),
            parameter_versions: state
                .parameter_versions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(ParameterVersionView::from).collect(),
                    )
                })
                .collect(),
            documents: state
                .documents
                .iter()
                .map(|(k, v)| (k.clone(), DocumentView::from(v)))
                .collect(),
            maintenance_windows: state
                .maintenance_windows
                .iter()
                .map(|(k, v)| (k.clone(), MaintenanceWindowView::from(v)))
                .collect(),
            patch_baselines: state
                .patch_baselines
                .iter()
                .map(|(k, v)| (k.clone(), PatchBaselineView::from(v)))
                .collect(),
            default_patch_baselines: state.default_patch_baselines.clone(),
            commands: state
                .commands
                .iter()
                .map(|(k, v)| (k.clone(), CommandView::from(v)))
                .collect(),
            associations: state
                .associations
                .iter()
                .map(|(k, v)| (k.clone(), AssociationView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            ops_items: state
                .ops_items
                .iter()
                .map(|(k, v)| (k.clone(), OpsItemView::from(v)))
                .collect(),
            sessions: state
                .sessions
                .iter()
                .map(|(k, v)| (k.clone(), SessionView::from(v)))
                .collect(),
            activations: state
                .activations
                .iter()
                .map(|(k, v)| (k.clone(), ActivationView::from(v)))
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(ResourcePolicyView::from).collect()))
                .collect(),
            service_settings: state
                .service_settings
                .iter()
                .map(|(k, v)| (k.clone(), ServiceSettingView::from(v)))
                .collect(),
            inventory: state
                .inventory
                .iter()
                .map(|((instance_id, type_name), v)| {
                    (
                        format!("{}::{}", instance_id, type_name),
                        InventoryDataView::from(v),
                    )
                })
                .collect(),
            compliance_records: state
                .compliance_records
                .iter()
                .map(|((resource_id, compliance_type, item_id), v)| {
                    (
                        format!("{}::{}::{}", resource_id, compliance_type, item_id),
                        ComplianceRecordView::from(v),
                    )
                })
                .collect(),
            ops_metadata: state
                .ops_metadata
                .iter()
                .map(|(k, v)| (k.clone(), OpsMetadataView::from(v)))
                .collect(),
            resource_data_syncs: state
                .resource_data_syncs
                .iter()
                .map(|(k, v)| (k.clone(), ResourceDataSyncView::from(v)))
                .collect(),
            ops_item_related_items: state
                .ops_item_related_items
                .iter()
                .map(|(k, v)| (k.clone(), OpsItemRelatedItemView::from(v)))
                .collect(),
            managed_instances: state
                .managed_instances
                .iter()
                .map(|(k, v)| (k.clone(), ManagedInstanceView::from(v)))
                .collect(),
            inventory_deletions: state
                .inventory_deletions
                .iter()
                .map(|(k, v)| (k.clone(), InventoryDeletionView::from(v)))
                .collect(),
            maintenance_window_executions: state
                .maintenance_window_executions
                .iter()
                .map(|(k, v)| (k.clone(), MaintenanceWindowExecutionView::from(v)))
                .collect(),
            document_metadata: state
                .document_metadata
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(DocumentMetadataEntryView::from).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<&Association> for AssociationView {
    fn from(a: &Association) -> Self {
        AssociationView {
            association_id: a.association_id.clone(),
            association_name: a.association_name.clone(),
            name: a.name.clone(),
            document_version: a.document_version.clone(),
            targets: a.targets.iter().map(TargetView::from).collect(),
            schedule_expression: a.schedule_expression.clone(),
            parameters: a.parameters.clone(),
            status: a.overview.status.clone(),
            detailed_status: a.overview.detailed_status.clone(),
            last_execution_date: a.last_execution_date.map(|dt| dt.to_rfc3339()),
            association_version: a.association_version.clone(),
            created_date: Some(a.created_date.to_rfc3339()),
        }
    }
}

impl From<&Parameter> for ParameterView {
    fn from(p: &Parameter) -> Self {
        ParameterView {
            name: p.name.clone(),
            r#type: p.r#type.clone(),
            value: p.value.clone(),
            version: p.version,
            last_modified_date: Some(p.last_modified_date.to_rfc3339()),
            arn: p.arn.clone(),
            data_type: p.data_type.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&ParameterVersion> for ParameterVersionView {
    fn from(pv: &ParameterVersion) -> Self {
        ParameterVersionView {
            name: pv.name.clone(),
            r#type: pv.r#type.clone(),
            value: pv.value.clone(),
            version: pv.version,
            last_modified_date: Some(pv.last_modified_date.to_rfc3339()),
            labels: pv.labels.clone(),
        }
    }
}

impl From<&Document> for DocumentView {
    fn from(d: &Document) -> Self {
        DocumentView {
            name: d.name.clone(),
            content: d.content.clone(),
            document_type: d.document_type.clone(),
            document_format: d.document_format.clone(),
            status: d.status.clone(),
            owner: d.owner.clone(),
            default_version: d.default_version.clone(),
            latest_version: d.latest_version.clone(),
            versions: d.versions.iter().map(DocumentVersionView::from).collect(),
            account_permissions: d.account_permissions.clone(),
            created_date: Some(d.created_date.to_rfc3339()),
        }
    }
}

impl From<&DocumentVersion> for DocumentVersionView {
    fn from(dv: &DocumentVersion) -> Self {
        DocumentVersionView {
            version_name: dv.version_name.clone(),
            document_version: dv.document_version.clone(),
            content: dv.content.clone(),
            created_date: Some(dv.created_date.to_rfc3339()),
            status: dv.status.clone(),
        }
    }
}

impl From<&MaintenanceWindow> for MaintenanceWindowView {
    fn from(w: &MaintenanceWindow) -> Self {
        MaintenanceWindowView {
            window_id: w.window_id.clone(),
            name: w.name.clone(),
            schedule: w.schedule.clone(),
            duration: w.duration,
            cutoff: w.cutoff,
            enabled: w.enabled,
            targets: w
                .targets
                .iter()
                .map(MaintenanceWindowTargetView::from)
                .collect(),
            tasks: w
                .tasks
                .iter()
                .map(MaintenanceWindowTaskView::from)
                .collect(),
        }
    }
}

impl From<&MaintenanceWindowTarget> for MaintenanceWindowTargetView {
    fn from(t: &MaintenanceWindowTarget) -> Self {
        MaintenanceWindowTargetView {
            window_target_id: t.window_target_id.clone(),
            window_id: t.window_id.clone(),
            resource_type: t.resource_type.clone(),
            targets: t.targets.iter().map(TargetView::from).collect(),
        }
    }
}

impl From<&MaintenanceWindowTask> for MaintenanceWindowTaskView {
    fn from(t: &MaintenanceWindowTask) -> Self {
        MaintenanceWindowTaskView {
            window_task_id: t.window_task_id.clone(),
            window_id: t.window_id.clone(),
            task_arn: t.task_arn.clone(),
            task_type: t.task_type.clone(),
            targets: t.targets.iter().map(TargetView::from).collect(),
        }
    }
}

impl From<&Target> for TargetView {
    fn from(t: &Target) -> Self {
        TargetView {
            key: t.key.clone(),
            values: t.values.clone(),
        }
    }
}

impl From<&PatchBaseline> for PatchBaselineView {
    fn from(b: &PatchBaseline) -> Self {
        PatchBaselineView {
            baseline_id: b.baseline_id.clone(),
            name: b.name.clone(),
            operating_system: b.operating_system.clone(),
            description: b.description.clone(),
            patch_groups: b.patch_groups.clone(),
        }
    }
}

impl From<&Command> for CommandView {
    fn from(c: &Command) -> Self {
        CommandView {
            command_id: c.command_id.clone(),
            instance_ids: c.instance_ids.clone(),
            document_name: c.document_name.clone(),
            status: c.status.clone(),
            requested_date_time: Some(c.requested_date_time.to_rfc3339()),
            parameters: c.parameters.clone(),
        }
    }
}

impl From<&OpsItem> for OpsItemView {
    fn from(o: &OpsItem) -> Self {
        OpsItemView {
            ops_item_id: o.ops_item_id.clone(),
            title: o.title.clone(),
            description: o.description.clone(),
            source: o.source.clone(),
            status: o.status.clone(),
            priority: o.priority,
            category: o.category.clone(),
            severity: o.severity.clone(),
            ops_item_type: o.ops_item_type.clone(),
            created_time: Some(o.created_time.to_rfc3339()),
            last_modified_time: Some(o.last_modified_time.to_rfc3339()),
        }
    }
}

impl From<&Session> for SessionView {
    fn from(s: &Session) -> Self {
        SessionView {
            session_id: s.session_id.clone(),
            target: s.target.clone(),
            status: s.status.clone(),
            start_date: Some(s.start_date.to_rfc3339()),
            end_date: s.end_date.map(|dt| dt.to_rfc3339()),
            document_name: s.document_name.clone(),
        }
    }
}

impl From<&Activation> for ActivationView {
    fn from(a: &Activation) -> Self {
        ActivationView {
            activation_id: a.activation_id.clone(),
            activation_code: a.activation_code.clone(),
            iam_role: a.iam_role.clone(),
            description: a.description.clone(),
            default_instance_name: a.default_instance_name.clone(),
            registration_limit: a.registration_limit,
            registrations_count: a.registrations_count,
            expiration_date: a.expiration_date.map(|dt| dt.to_rfc3339()),
            expired: a.expired,
            created_date: Some(a.created_date.to_rfc3339()),
        }
    }
}

impl From<&ResourcePolicy> for ResourcePolicyView {
    fn from(p: &ResourcePolicy) -> Self {
        ResourcePolicyView {
            policy_id: p.policy_id.clone(),
            policy_hash: p.policy_hash.clone(),
            policy: p.policy.clone(),
        }
    }
}

impl From<&ServiceSetting> for ServiceSettingView {
    fn from(s: &ServiceSetting) -> Self {
        ServiceSettingView {
            setting_id: s.setting_id.clone(),
            setting_value: s.setting_value.clone(),
            last_modified_time: Some(s.last_modified_time.to_rfc3339()),
            arn: s.arn.clone(),
        }
    }
}

impl From<&InventoryData> for InventoryDataView {
    fn from(d: &InventoryData) -> Self {
        InventoryDataView {
            instance_id: d.instance_id.clone(),
            type_name: d.type_name.clone(),
            capture_time: d.capture_time.clone(),
            schema_version: d.schema_version.clone(),
            content: d.content.clone(),
        }
    }
}

impl From<&ComplianceRecord> for ComplianceRecordView {
    fn from(r: &ComplianceRecord) -> Self {
        ComplianceRecordView {
            compliance_type: r.compliance_type.clone(),
            resource_id: r.resource_id.clone(),
            resource_type: r.resource_type.clone(),
            status: r.status.clone(),
            severity: r.severity.clone(),
            execution_type: r.execution_type.clone(),
            execution_id: r.execution_id.clone(),
            execution_time: Some(r.execution_time.to_rfc3339()),
            item_id: r.item_id.clone(),
            title: r.title.clone(),
            details: r.details.clone(),
        }
    }
}

impl From<&OpsMetadataEntry> for OpsMetadataView {
    fn from(e: &OpsMetadataEntry) -> Self {
        OpsMetadataView {
            ops_metadata_arn: e.ops_metadata_arn.clone(),
            resource_id: e.resource_id.clone(),
            metadata: e.metadata.clone(),
            created_date: Some(e.created_date.to_rfc3339()),
            last_modified_date: Some(e.last_modified_date.to_rfc3339()),
        }
    }
}

impl From<&ResourceDataSyncEntry> for ResourceDataSyncView {
    fn from(e: &ResourceDataSyncEntry) -> Self {
        ResourceDataSyncView {
            sync_name: e.sync_name.clone(),
            sync_type: e.sync_type.clone(),
            s3_destination_bucket: e.s3_destination_bucket.clone(),
            s3_destination_region: e.s3_destination_region.clone(),
            s3_destination_prefix: e.s3_destination_prefix.clone(),
            created_time: Some(e.created_time.to_rfc3339()),
            last_status: e.last_status.clone(),
        }
    }
}

impl From<&OpsItemRelatedItem> for OpsItemRelatedItemView {
    fn from(i: &OpsItemRelatedItem) -> Self {
        OpsItemRelatedItemView {
            association_id: i.association_id.clone(),
            ops_item_id: i.ops_item_id.clone(),
            association_type: i.association_type.clone(),
            resource_type: i.resource_type.clone(),
            resource_uri: i.resource_uri.clone(),
            created_time: Some(i.created_time.to_rfc3339()),
        }
    }
}

impl From<&ManagedInstance> for ManagedInstanceView {
    fn from(mi: &ManagedInstance) -> Self {
        ManagedInstanceView {
            instance_id: mi.instance_id.clone(),
            ping_status: mi.ping_status.clone(),
            platform_type: mi.platform_type.clone(),
            platform_name: mi.platform_name.clone(),
            platform_version: mi.platform_version.clone(),
            activation_id: mi.activation_id.clone(),
            iam_role: mi.iam_role.clone(),
            registration_date: Some(mi.registration_date.to_rfc3339()),
            resource_type: mi.resource_type.clone(),
            ip_address: mi.ip_address.clone(),
            computer_name: mi.computer_name.clone(),
            is_latest_version: mi.is_latest_version,
            last_ping_date_time: Some(mi.last_ping_date_time.to_rfc3339()),
        }
    }
}

impl From<&InventoryDeletion> for InventoryDeletionView {
    fn from(d: &InventoryDeletion) -> Self {
        InventoryDeletionView {
            deletion_id: d.deletion_id.clone(),
            type_name: d.type_name.clone(),
            deletion_start_time: Some(d.deletion_start_time.to_rfc3339()),
            last_status: d.last_status.clone(),
            last_status_message: d.last_status_message.clone(),
        }
    }
}

impl From<&MaintenanceWindowExecution> for MaintenanceWindowExecutionView {
    fn from(e: &MaintenanceWindowExecution) -> Self {
        MaintenanceWindowExecutionView {
            window_execution_id: e.window_execution_id.clone(),
            window_id: e.window_id.clone(),
            status: e.status.clone(),
            start_time: Some(e.start_time.to_rfc3339()),
            end_time: e.end_time.map(|dt| dt.to_rfc3339()),
            tasks: e
                .tasks
                .iter()
                .map(MaintenanceWindowExecutionTaskViewEntry::from)
                .collect(),
        }
    }
}

impl From<&MaintenanceWindowExecutionTask> for MaintenanceWindowExecutionTaskViewEntry {
    fn from(t: &MaintenanceWindowExecutionTask) -> Self {
        MaintenanceWindowExecutionTaskViewEntry {
            window_execution_id: t.window_execution_id.clone(),
            task_execution_id: t.task_execution_id.clone(),
            window_task_id: t.window_task_id.clone(),
            task_arn: t.task_arn.clone(),
            task_type: t.task_type.clone(),
            status: t.status.clone(),
            start_time: Some(t.start_time.to_rfc3339()),
            end_time: t.end_time.map(|dt| dt.to_rfc3339()),
            invocations: t
                .invocations
                .iter()
                .map(MaintenanceWindowExecutionTaskInvocationView::from)
                .collect(),
        }
    }
}

impl From<&MaintenanceWindowExecutionTaskInvocation>
    for MaintenanceWindowExecutionTaskInvocationView
{
    fn from(i: &MaintenanceWindowExecutionTaskInvocation) -> Self {
        MaintenanceWindowExecutionTaskInvocationView {
            invocation_id: i.invocation_id.clone(),
            window_execution_id: i.window_execution_id.clone(),
            task_execution_id: i.task_execution_id.clone(),
            execution_id: i.execution_id.clone(),
            status: i.status.clone(),
            start_time: Some(i.start_time.to_rfc3339()),
            end_time: i.end_time.map(|dt| dt.to_rfc3339()),
        }
    }
}

impl From<&DocumentMetadataEntry> for DocumentMetadataEntryView {
    fn from(e: &DocumentMetadataEntry) -> Self {
        DocumentMetadataEntryView {
            reviewer: e.reviewer.clone(),
            status: e.status.clone(),
            comment: e.comment.clone(),
            updated_date: Some(e.updated_date.to_rfc3339()),
        }
    }
}

// --- From view types to internal types ---

impl From<SsmStateView> for SsmState {
    fn from(view: SsmStateView) -> Self {
        let mut state = SsmState::default();
        state.parameters = view
            .parameters
            .into_iter()
            .map(|(k, v)| (k, Parameter::from(v)))
            .collect();
        state.parameter_versions = view
            .parameter_versions
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(ParameterVersion::from).collect()))
            .collect();
        state.documents = view
            .documents
            .into_iter()
            .map(|(k, v)| (k, Document::from(v)))
            .collect();
        state.maintenance_windows = view
            .maintenance_windows
            .into_iter()
            .map(|(k, v)| (k, MaintenanceWindow::from(v)))
            .collect();
        state.patch_baselines = view
            .patch_baselines
            .into_iter()
            .map(|(k, v)| (k, PatchBaseline::from(v)))
            .collect();
        state.default_patch_baselines = view.default_patch_baselines;
        state.commands = view
            .commands
            .into_iter()
            .map(|(k, v)| (k, Command::from(v)))
            .collect();
        state.associations = view
            .associations
            .into_iter()
            .map(|(k, v)| (k, Association::from(v)))
            .collect();
        state.resource_tags = view.resource_tags;
        state.ops_items = view
            .ops_items
            .into_iter()
            .map(|(k, v)| (k, OpsItem::from(v)))
            .collect();
        state.sessions = view
            .sessions
            .into_iter()
            .map(|(k, v)| (k, Session::from(v)))
            .collect();
        state.activations = view
            .activations
            .into_iter()
            .map(|(k, v)| (k, Activation::from(v)))
            .collect();
        state.resource_policies = view
            .resource_policies
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(ResourcePolicy::from).collect()))
            .collect();
        state.service_settings = view
            .service_settings
            .into_iter()
            .map(|(k, v)| (k, ServiceSetting::from(v)))
            .collect();
        state.inventory = view
            .inventory
            .into_values()
            .map(|v| {
                (
                    (v.instance_id.clone(), v.type_name.clone()),
                    InventoryData::from(v),
                )
            })
            .collect();
        state.compliance_records = view
            .compliance_records
            .into_values()
            .map(|v| {
                (
                    (
                        v.resource_id.clone(),
                        v.compliance_type.clone(),
                        v.item_id.clone(),
                    ),
                    ComplianceRecord::from(v),
                )
            })
            .collect();
        state.ops_metadata = view
            .ops_metadata
            .into_iter()
            .map(|(k, v)| (k, OpsMetadataEntry::from(v)))
            .collect();
        state.resource_data_syncs = view
            .resource_data_syncs
            .into_iter()
            .map(|(k, v)| (k, ResourceDataSyncEntry::from(v)))
            .collect();
        state.ops_item_related_items = view
            .ops_item_related_items
            .into_iter()
            .map(|(k, v)| (k, OpsItemRelatedItem::from(v)))
            .collect();
        state.managed_instances = view
            .managed_instances
            .into_iter()
            .map(|(k, v)| (k, ManagedInstance::from(v)))
            .collect();
        state.inventory_deletions = view
            .inventory_deletions
            .into_iter()
            .map(|(k, v)| (k, InventoryDeletion::from(v)))
            .collect();
        state.maintenance_window_executions = view
            .maintenance_window_executions
            .into_iter()
            .map(|(k, v)| (k, MaintenanceWindowExecution::from(v)))
            .collect();
        state.document_metadata = view
            .document_metadata
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(DocumentMetadataEntry::from).collect()))
            .collect();
        state
    }
}

impl From<AssociationView> for Association {
    fn from(v: AssociationView) -> Self {
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_execution_date = v
            .last_execution_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        Association {
            association_id: v.association_id,
            association_name: v.association_name,
            name: v.name,
            document_version: v.document_version,
            targets: v.targets.into_iter().map(Target::from).collect(),
            schedule_expression: v.schedule_expression,
            parameters: v.parameters,
            overview: AssociationOverview {
                status: v.status,
                detailed_status: v.detailed_status,
            },
            last_execution_date,
            association_version: v.association_version,
            created_date,
        }
    }
}

impl From<ParameterView> for Parameter {
    fn from(v: ParameterView) -> Self {
        let last_modified_date = v
            .last_modified_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Parameter {
            name: v.name,
            r#type: v.r#type,
            value: v.value,
            version: v.version,
            last_modified_date,
            arn: v.arn,
            data_type: v.data_type,
            tags: v.tags,
        }
    }
}

impl From<ParameterVersionView> for ParameterVersion {
    fn from(v: ParameterVersionView) -> Self {
        let last_modified_date = v
            .last_modified_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ParameterVersion {
            name: v.name,
            r#type: v.r#type,
            value: v.value,
            version: v.version,
            last_modified_date,
            labels: v.labels,
        }
    }
}

impl From<DocumentView> for Document {
    fn from(v: DocumentView) -> Self {
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Document {
            name: v.name,
            content: v.content,
            document_type: v.document_type,
            document_format: v.document_format,
            status: v.status,
            owner: v.owner,
            default_version: v.default_version,
            latest_version: v.latest_version,
            versions: v.versions.into_iter().map(DocumentVersion::from).collect(),
            account_permissions: v.account_permissions,
            created_date,
        }
    }
}

impl From<DocumentVersionView> for DocumentVersion {
    fn from(v: DocumentVersionView) -> Self {
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        DocumentVersion {
            version_name: v.version_name,
            document_version: v.document_version,
            content: v.content,
            created_date,
            status: v.status,
        }
    }
}

impl From<MaintenanceWindowView> for MaintenanceWindow {
    fn from(v: MaintenanceWindowView) -> Self {
        MaintenanceWindow {
            window_id: v.window_id,
            name: v.name,
            schedule: v.schedule,
            duration: v.duration,
            cutoff: v.cutoff,
            enabled: v.enabled,
            targets: v
                .targets
                .into_iter()
                .map(MaintenanceWindowTarget::from)
                .collect(),
            tasks: v
                .tasks
                .into_iter()
                .map(MaintenanceWindowTask::from)
                .collect(),
        }
    }
}

impl From<MaintenanceWindowTargetView> for MaintenanceWindowTarget {
    fn from(v: MaintenanceWindowTargetView) -> Self {
        MaintenanceWindowTarget {
            window_target_id: v.window_target_id,
            window_id: v.window_id,
            resource_type: v.resource_type,
            targets: v.targets.into_iter().map(Target::from).collect(),
        }
    }
}

impl From<MaintenanceWindowTaskView> for MaintenanceWindowTask {
    fn from(v: MaintenanceWindowTaskView) -> Self {
        MaintenanceWindowTask {
            window_task_id: v.window_task_id,
            window_id: v.window_id,
            task_arn: v.task_arn,
            task_type: v.task_type,
            targets: v.targets.into_iter().map(Target::from).collect(),
        }
    }
}

impl From<TargetView> for Target {
    fn from(v: TargetView) -> Self {
        Target {
            key: v.key,
            values: v.values,
        }
    }
}

impl From<PatchBaselineView> for PatchBaseline {
    fn from(v: PatchBaselineView) -> Self {
        PatchBaseline {
            baseline_id: v.baseline_id,
            name: v.name,
            operating_system: v.operating_system,
            description: v.description,
            patch_groups: v.patch_groups,
        }
    }
}

impl From<CommandView> for Command {
    fn from(v: CommandView) -> Self {
        let requested_date_time = v
            .requested_date_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Command {
            command_id: v.command_id,
            instance_ids: v.instance_ids,
            document_name: v.document_name,
            status: v.status,
            requested_date_time,
            parameters: v.parameters,
        }
    }
}

impl From<OpsItemView> for OpsItem {
    fn from(v: OpsItemView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        OpsItem {
            ops_item_id: v.ops_item_id,
            title: v.title,
            description: v.description,
            source: v.source,
            status: v.status,
            priority: v.priority,
            category: v.category,
            severity: v.severity,
            ops_item_type: v.ops_item_type,
            created_time,
            last_modified_time,
        }
    }
}

impl From<SessionView> for Session {
    fn from(v: SessionView) -> Self {
        let start_date = v
            .start_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let end_date = v
            .end_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        Session {
            session_id: v.session_id,
            target: v.target,
            status: v.status,
            start_date,
            end_date,
            document_name: v.document_name,
        }
    }
}

impl From<ActivationView> for Activation {
    fn from(v: ActivationView) -> Self {
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let expiration_date = v
            .expiration_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        Activation {
            activation_id: v.activation_id,
            activation_code: v.activation_code,
            iam_role: v.iam_role,
            description: v.description,
            default_instance_name: v.default_instance_name,
            registration_limit: v.registration_limit,
            registrations_count: v.registrations_count,
            expiration_date,
            expired: v.expired,
            created_date,
        }
    }
}

impl From<ResourcePolicyView> for ResourcePolicy {
    fn from(v: ResourcePolicyView) -> Self {
        ResourcePolicy {
            policy_id: v.policy_id,
            policy_hash: v.policy_hash,
            policy: v.policy,
        }
    }
}

impl From<ServiceSettingView> for ServiceSetting {
    fn from(v: ServiceSettingView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ServiceSetting {
            setting_id: v.setting_id,
            setting_value: v.setting_value,
            last_modified_time,
            arn: v.arn,
        }
    }
}

impl From<InventoryDataView> for InventoryData {
    fn from(v: InventoryDataView) -> Self {
        InventoryData {
            instance_id: v.instance_id,
            type_name: v.type_name,
            capture_time: v.capture_time,
            schema_version: v.schema_version,
            content: v.content,
        }
    }
}

impl From<ComplianceRecordView> for ComplianceRecord {
    fn from(v: ComplianceRecordView) -> Self {
        let execution_time = v
            .execution_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ComplianceRecord {
            compliance_type: v.compliance_type,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            status: v.status,
            severity: v.severity,
            execution_type: v.execution_type,
            execution_id: v.execution_id,
            execution_time,
            item_id: v.item_id,
            title: v.title,
            details: v.details,
        }
    }
}

impl From<OpsMetadataView> for OpsMetadataEntry {
    fn from(v: OpsMetadataView) -> Self {
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_date = v
            .last_modified_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        OpsMetadataEntry {
            ops_metadata_arn: v.ops_metadata_arn,
            resource_id: v.resource_id,
            metadata: v.metadata,
            created_date,
            last_modified_date,
        }
    }
}

impl From<ResourceDataSyncView> for ResourceDataSyncEntry {
    fn from(v: ResourceDataSyncView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ResourceDataSyncEntry {
            sync_name: v.sync_name,
            sync_type: v.sync_type,
            s3_destination_bucket: v.s3_destination_bucket,
            s3_destination_region: v.s3_destination_region,
            s3_destination_prefix: v.s3_destination_prefix,
            created_time,
            last_status: v.last_status,
        }
    }
}

impl From<OpsItemRelatedItemView> for OpsItemRelatedItem {
    fn from(v: OpsItemRelatedItemView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        OpsItemRelatedItem {
            association_id: v.association_id,
            ops_item_id: v.ops_item_id,
            association_type: v.association_type,
            resource_type: v.resource_type,
            resource_uri: v.resource_uri,
            created_time,
        }
    }
}

impl From<ManagedInstanceView> for ManagedInstance {
    fn from(v: ManagedInstanceView) -> Self {
        let registration_date = v
            .registration_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_ping_date_time = v
            .last_ping_date_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ManagedInstance {
            instance_id: v.instance_id,
            ping_status: v.ping_status,
            platform_type: v.platform_type,
            platform_name: v.platform_name,
            platform_version: v.platform_version,
            activation_id: v.activation_id,
            iam_role: v.iam_role,
            registration_date,
            resource_type: v.resource_type,
            ip_address: v.ip_address,
            computer_name: v.computer_name,
            is_latest_version: v.is_latest_version,
            last_ping_date_time,
        }
    }
}

impl From<InventoryDeletionView> for InventoryDeletion {
    fn from(v: InventoryDeletionView) -> Self {
        let deletion_start_time = v
            .deletion_start_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        InventoryDeletion {
            deletion_id: v.deletion_id,
            type_name: v.type_name,
            deletion_start_time,
            last_status: v.last_status,
            last_status_message: v.last_status_message,
        }
    }
}

impl From<MaintenanceWindowExecutionView> for MaintenanceWindowExecution {
    fn from(v: MaintenanceWindowExecutionView) -> Self {
        let start_time = v
            .start_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let end_time = v
            .end_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        MaintenanceWindowExecution {
            window_execution_id: v.window_execution_id,
            window_id: v.window_id,
            status: v.status,
            start_time,
            end_time,
            tasks: v
                .tasks
                .into_iter()
                .map(MaintenanceWindowExecutionTask::from)
                .collect(),
        }
    }
}

impl From<MaintenanceWindowExecutionTaskViewEntry> for MaintenanceWindowExecutionTask {
    fn from(v: MaintenanceWindowExecutionTaskViewEntry) -> Self {
        let start_time = v
            .start_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let end_time = v
            .end_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        MaintenanceWindowExecutionTask {
            window_execution_id: v.window_execution_id,
            task_execution_id: v.task_execution_id,
            window_task_id: v.window_task_id,
            task_arn: v.task_arn,
            task_type: v.task_type,
            status: v.status,
            start_time,
            end_time,
            invocations: v
                .invocations
                .into_iter()
                .map(MaintenanceWindowExecutionTaskInvocation::from)
                .collect(),
        }
    }
}

impl From<MaintenanceWindowExecutionTaskInvocationView>
    for MaintenanceWindowExecutionTaskInvocation
{
    fn from(v: MaintenanceWindowExecutionTaskInvocationView) -> Self {
        let start_time = v
            .start_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let end_time = v
            .end_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        MaintenanceWindowExecutionTaskInvocation {
            invocation_id: v.invocation_id,
            window_execution_id: v.window_execution_id,
            task_execution_id: v.task_execution_id,
            execution_id: v.execution_id,
            status: v.status,
            start_time,
            end_time,
        }
    }
}

impl From<DocumentMetadataEntryView> for DocumentMetadataEntry {
    fn from(v: DocumentMetadataEntryView) -> Self {
        let updated_date = v
            .updated_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        DocumentMetadataEntry {
            reviewer: v.reviewer,
            status: v.status,
            comment: v.comment,
            updated_date,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SsmService {
    type StateView = SsmStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SsmStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = SsmState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, param_view) in view.parameters {
                guard.parameters.insert(name, Parameter::from(param_view));
            }
            for (name, versions) in view.parameter_versions {
                guard.parameter_versions.insert(
                    name,
                    versions.into_iter().map(ParameterVersion::from).collect(),
                );
            }
            for (name, doc_view) in view.documents {
                guard.documents.insert(name, Document::from(doc_view));
            }
            for (id, window_view) in view.maintenance_windows {
                guard
                    .maintenance_windows
                    .insert(id, MaintenanceWindow::from(window_view));
            }
            for (id, baseline_view) in view.patch_baselines {
                guard
                    .patch_baselines
                    .insert(id, PatchBaseline::from(baseline_view));
            }
            for (os, baseline_id) in view.default_patch_baselines {
                guard.default_patch_baselines.insert(os, baseline_id);
            }
            for (id, command_view) in view.commands {
                guard.commands.insert(id, Command::from(command_view));
            }
            for (id, assoc_view) in view.associations {
                guard.associations.insert(id, Association::from(assoc_view));
            }
            for (resource_id, tags) in view.resource_tags {
                guard
                    .resource_tags
                    .entry(resource_id)
                    .or_default()
                    .extend(tags);
            }
            for (id, ops_item_view) in view.ops_items {
                guard.ops_items.insert(id, OpsItem::from(ops_item_view));
            }
            for (id, session_view) in view.sessions {
                guard.sessions.insert(id, Session::from(session_view));
            }
            for (id, activation_view) in view.activations {
                guard
                    .activations
                    .insert(id, Activation::from(activation_view));
            }
            for (arn, policies) in view.resource_policies {
                guard.resource_policies.insert(
                    arn,
                    policies.into_iter().map(ResourcePolicy::from).collect(),
                );
            }
            for (id, setting_view) in view.service_settings {
                guard
                    .service_settings
                    .insert(id, ServiceSetting::from(setting_view));
            }
            for inv_view in view.inventory.into_values() {
                let key = (inv_view.instance_id.clone(), inv_view.type_name.clone());
                guard.inventory.insert(key, InventoryData::from(inv_view));
            }
            for rec_view in view.compliance_records.into_values() {
                let key = (
                    rec_view.resource_id.clone(),
                    rec_view.compliance_type.clone(),
                    rec_view.item_id.clone(),
                );
                guard
                    .compliance_records
                    .insert(key, ComplianceRecord::from(rec_view));
            }
            for (arn, meta_view) in view.ops_metadata {
                guard
                    .ops_metadata
                    .insert(arn, OpsMetadataEntry::from(meta_view));
            }
            for (name, sync_view) in view.resource_data_syncs {
                guard
                    .resource_data_syncs
                    .insert(name, ResourceDataSyncEntry::from(sync_view));
            }
            for (id, item_view) in view.ops_item_related_items {
                guard
                    .ops_item_related_items
                    .insert(id, OpsItemRelatedItem::from(item_view));
            }
            for (id, mi_view) in view.managed_instances {
                guard
                    .managed_instances
                    .insert(id, ManagedInstance::from(mi_view));
            }
            for (id, del_view) in view.inventory_deletions {
                guard
                    .inventory_deletions
                    .insert(id, InventoryDeletion::from(del_view));
            }
            for (id, exec_view) in view.maintenance_window_executions {
                guard
                    .maintenance_window_executions
                    .insert(id, MaintenanceWindowExecution::from(exec_view));
            }
            for (name, entries_view) in view.document_metadata {
                guard
                    .document_metadata
                    .entry(name)
                    .or_default()
                    .extend(entries_view.into_iter().map(DocumentMetadataEntry::from));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
