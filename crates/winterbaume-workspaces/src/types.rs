use serde::{Deserialize, Serialize};

/// Represents a WorkSpace directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDirectory {
    pub directory_id: String,
    pub directory_name: String,
    pub directory_type: String,
    pub alias: String,
    pub state: String,
    pub registration_code: String,
    pub workspace_security_group_id: String,
    pub iam_role_id: String,
}

/// Represents a WorkSpace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub workspace_id: String,
    pub directory_id: String,
    pub user_name: String,
    pub bundle_id: String,
    pub state: String,
    pub ip_address: String,
    pub computer_name: String,
    pub subnet_id: String,
    pub root_volume_size_gib: i32,
    pub user_volume_size_gib: i32,
    pub volume_encryption_key: Option<String>,
    pub user_volume_encryption_enabled: bool,
    pub root_volume_encryption_enabled: bool,
    pub running_mode: String,
    pub running_mode_auto_stop_timeout_in_minutes: i32,
}

/// Request to create a single workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub directory_id: String,
    pub user_name: String,
    pub bundle_id: String,
    pub volume_encryption_key: Option<String>,
    pub user_volume_encryption_enabled: Option<bool>,
    pub root_volume_encryption_enabled: Option<bool>,
    pub workspace_properties: Option<WorkspaceProperties>,
    pub tags: Option<Vec<Tag>>,
}

/// WorkSpace properties (running mode etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    pub running_mode: Option<String>,
    pub running_mode_auto_stop_timeout_in_minutes: Option<i32>,
    pub root_volume_size_gib: Option<i32>,
    pub user_volume_size_gib: Option<i32>,
    pub compute_type_name: Option<String>,
}

/// A tag key-value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// A failed workspace creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedCreateWorkspaceRequest {
    pub workspace_request: Option<WorkspaceRequest>,
    pub error_code: String,
    pub error_message: String,
}

/// A workspace ID for terminate request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateRequest {
    pub workspace_id: String,
}

/// A failed workspace termination request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTerminateWorkspaceRequest {
    pub workspace_id: String,
    pub error_code: String,
    pub error_message: String,
}

/// Represents a WorkSpace image in internal state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceImage {
    pub image_id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub operating_system_type: Option<String>,
    pub owner_account_id: String,
    pub required_tenancy: String,
    pub created: String,
}

/// Client properties for a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientProperties {
    pub reconnect_enabled: Option<String>,
    pub log_upload_enabled: Option<String>,
}

/// Self-service permissions for a directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfservicePermissions {
    pub restart_workspace: Option<String>,
    pub increase_volume_size: Option<String>,
    pub change_compute_type: Option<String>,
    pub switch_running_mode: Option<String>,
    pub rebuild_workspace: Option<String>,
}

/// Workspace creation properties for a directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCreationProperties {
    pub custom_security_group_id: Option<String>,
    pub default_ou: Option<String>,
    pub enable_internet_access: Option<bool>,
    pub enable_maintenance_mode: Option<bool>,
    pub user_enabled_as_local_administrator: Option<bool>,
    pub instance_iam_role_arn: Option<String>,
}

/// Image permission entry (shared account ID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePermission {
    pub shared_account_id: String,
}

/// An IP access control group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpGroup {
    pub group_id: String,
    pub group_name: String,
    pub group_desc: Option<String>,
    pub user_rules: Vec<IpRule>,
}

/// A rule in an IP access control group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRule {
    pub ip_rule: String,
    pub rule_desc: Option<String>,
}

/// A connection alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAlias {
    pub alias_id: String,
    pub connection_string: String,
    pub owner_account_id: String,
    pub state: String,
    /// directory_id -> connection_identifier
    pub associations: Vec<ConnectionAliasAssociation>,
}

/// A connection alias association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAliasAssociation {
    pub connection_identifier: String,
    pub resource_id: Option<String>,
    pub associated_account_id: Option<String>,
}

/// A connection alias permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAliasPermission {
    pub shared_account_id: String,
    pub allow_association: bool,
}

/// A workspace bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceBundle {
    pub bundle_id: String,
    pub name: String,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub bundle_type: Option<String>,
    pub compute_type_name: Option<String>,
    pub root_storage_capacity: Option<i32>,
    pub user_storage_capacity: Option<i32>,
    pub creation_time: String,
}

/// A WorkSpaces pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacesPool {
    pub pool_id: String,
    pub pool_arn: String,
    pub pool_name: String,
    pub description: Option<String>,
    pub state: String,
    pub bundle_id: String,
    pub directory_id: String,
    pub created_at: String,
}

/// Request to start a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRequest {
    pub workspace_id: String,
}

/// Request to stop a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRequest {
    pub workspace_id: String,
}

/// Request to reboot a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebootRequest {
    pub workspace_id: String,
}

/// Request to rebuild a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildRequest {
    pub workspace_id: String,
}
