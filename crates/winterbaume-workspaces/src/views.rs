//! Serde-compatible view types for WorkSpaces state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::WorkSpacesService;
use crate::state::WorkSpacesState;
use crate::types::{
    ClientProperties, ConnectionAlias, ConnectionAliasAssociation, ConnectionAliasPermission,
    IpGroup, IpRule, SelfservicePermissions, Tag, Workspace, WorkspaceBundle,
    WorkspaceCreationProperties, WorkspaceDirectory, WorkspaceImage, WorkspacesPool,
};

/// Serializable view of the entire WorkSpaces state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkSpacesStateView {
    #[serde(default)]
    pub workspaces: HashMap<String, WorkspaceView>,
    #[serde(default)]
    pub directories: HashMap<String, WorkspaceDirectoryView>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
    #[serde(default)]
    pub images: HashMap<String, WorkspaceImageView>,
    #[serde(default)]
    pub client_properties: HashMap<String, ClientPropertiesView>,
    #[serde(default)]
    pub selfservice_permissions: HashMap<String, SelfservicePermissionsView>,
    #[serde(default)]
    pub workspace_creation_properties: HashMap<String, WorkspaceCreationPropertiesView>,
    #[serde(default)]
    pub image_permissions: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub ip_groups: HashMap<String, IpGroupView>,
    #[serde(default)]
    pub connection_aliases: HashMap<String, ConnectionAliasView>,
    #[serde(default)]
    pub connection_alias_permissions: HashMap<String, Vec<ConnectionAliasPermissionView>>,
    #[serde(default)]
    pub bundles: HashMap<String, WorkspaceBundleView>,
    #[serde(default)]
    pub pools: HashMap<String, WorkspacesPoolView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceView {
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
    /// `workspace_properties` nested block stored as raw JSON.
    #[serde(default)]
    pub workspace_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDirectoryView {
    pub directory_id: String,
    pub directory_name: String,
    pub directory_type: String,
    pub alias: String,
    pub state: String,
    pub registration_code: String,
    pub workspace_security_group_id: String,
    pub iam_role_id: String,
    /// `self_service_permissions` nested block.
    #[serde(default)]
    pub self_service_permissions: Option<serde_json::Value>,
    /// `workspace_access_properties` nested block.
    #[serde(default)]
    pub workspace_access_properties: Option<serde_json::Value>,
    /// `workspace_creation_properties` nested block.
    #[serde(default)]
    pub workspace_creation_properties: Option<serde_json::Value>,
    /// `streaming_properties` nested block.
    #[serde(default)]
    pub streaming_properties: Option<serde_json::Value>,
    /// `active_directory_config` nested block.
    #[serde(default)]
    pub active_directory_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceImageView {
    pub image_id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub operating_system_type: Option<String>,
    pub owner_account_id: String,
    pub required_tenancy: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPropertiesView {
    pub reconnect_enabled: Option<String>,
    pub log_upload_enabled: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfservicePermissionsView {
    pub restart_workspace: Option<String>,
    pub increase_volume_size: Option<String>,
    pub change_compute_type: Option<String>,
    pub switch_running_mode: Option<String>,
    pub rebuild_workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCreationPropertiesView {
    pub custom_security_group_id: Option<String>,
    pub default_ou: Option<String>,
    pub enable_internet_access: Option<bool>,
    pub enable_maintenance_mode: Option<bool>,
    pub user_enabled_as_local_administrator: Option<bool>,
    pub instance_iam_role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpGroupView {
    pub group_id: String,
    pub group_name: String,
    pub group_desc: Option<String>,
    pub user_rules: Vec<IpRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRuleView {
    pub ip_rule: String,
    pub rule_desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAliasView {
    pub alias_id: String,
    pub connection_string: String,
    pub owner_account_id: String,
    pub state: String,
    pub associations: Vec<ConnectionAliasAssociationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAliasAssociationView {
    pub connection_identifier: String,
    pub resource_id: Option<String>,
    pub associated_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAliasPermissionView {
    pub shared_account_id: String,
    pub allow_association: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceBundleView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacesPoolView {
    pub pool_id: String,
    pub pool_arn: String,
    pub pool_name: String,
    pub description: Option<String>,
    pub state: String,
    pub bundle_id: String,
    pub directory_id: String,
    pub created_at: String,
}

// --- From internal types to view types ---

impl From<&WorkSpacesState> for WorkSpacesStateView {
    fn from(state: &WorkSpacesState) -> Self {
        WorkSpacesStateView {
            workspaces: state
                .workspaces
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspaceView {
                            workspace_id: v.workspace_id.clone(),
                            directory_id: v.directory_id.clone(),
                            user_name: v.user_name.clone(),
                            bundle_id: v.bundle_id.clone(),
                            state: v.state.clone(),
                            ip_address: v.ip_address.clone(),
                            computer_name: v.computer_name.clone(),
                            subnet_id: v.subnet_id.clone(),
                            root_volume_size_gib: v.root_volume_size_gib,
                            user_volume_size_gib: v.user_volume_size_gib,
                            volume_encryption_key: v.volume_encryption_key.clone(),
                            user_volume_encryption_enabled: v.user_volume_encryption_enabled,
                            root_volume_encryption_enabled: v.root_volume_encryption_enabled,
                            running_mode: v.running_mode.clone(),
                            running_mode_auto_stop_timeout_in_minutes: v
                                .running_mode_auto_stop_timeout_in_minutes,
                            workspace_properties: None,
                        },
                    )
                })
                .collect(),
            directories: state
                .directories
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspaceDirectoryView {
                            directory_id: v.directory_id.clone(),
                            directory_name: v.directory_name.clone(),
                            directory_type: v.directory_type.clone(),
                            alias: v.alias.clone(),
                            state: v.state.clone(),
                            registration_code: v.registration_code.clone(),
                            workspace_security_group_id: v.workspace_security_group_id.clone(),
                            iam_role_id: v.iam_role_id.clone(),
                            self_service_permissions: None,
                            workspace_access_properties: None,
                            workspace_creation_properties: None,
                            streaming_properties: None,
                            active_directory_config: None,
                        },
                    )
                })
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|t| TagView {
                                key: t.key.clone(),
                                value: t.value.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            images: state
                .images
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspaceImageView {
                            image_id: v.image_id.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            state: v.state.clone(),
                            operating_system_type: v.operating_system_type.clone(),
                            owner_account_id: v.owner_account_id.clone(),
                            required_tenancy: v.required_tenancy.clone(),
                            created: v.created.clone(),
                        },
                    )
                })
                .collect(),
            client_properties: state
                .client_properties
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ClientPropertiesView {
                            reconnect_enabled: v.reconnect_enabled.clone(),
                            log_upload_enabled: v.log_upload_enabled.clone(),
                        },
                    )
                })
                .collect(),
            selfservice_permissions: state
                .selfservice_permissions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SelfservicePermissionsView {
                            restart_workspace: v.restart_workspace.clone(),
                            increase_volume_size: v.increase_volume_size.clone(),
                            change_compute_type: v.change_compute_type.clone(),
                            switch_running_mode: v.switch_running_mode.clone(),
                            rebuild_workspace: v.rebuild_workspace.clone(),
                        },
                    )
                })
                .collect(),
            workspace_creation_properties: state
                .workspace_creation_properties
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspaceCreationPropertiesView {
                            custom_security_group_id: v.custom_security_group_id.clone(),
                            default_ou: v.default_ou.clone(),
                            enable_internet_access: v.enable_internet_access,
                            enable_maintenance_mode: v.enable_maintenance_mode,
                            user_enabled_as_local_administrator: v
                                .user_enabled_as_local_administrator,
                            instance_iam_role_arn: v.instance_iam_role_arn.clone(),
                        },
                    )
                })
                .collect(),
            image_permissions: state.image_permissions.clone(),
            ip_groups: state
                .ip_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IpGroupView {
                            group_id: v.group_id.clone(),
                            group_name: v.group_name.clone(),
                            group_desc: v.group_desc.clone(),
                            user_rules: v
                                .user_rules
                                .iter()
                                .map(|r| IpRuleView {
                                    ip_rule: r.ip_rule.clone(),
                                    rule_desc: r.rule_desc.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            connection_aliases: state
                .connection_aliases
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ConnectionAliasView {
                            alias_id: v.alias_id.clone(),
                            connection_string: v.connection_string.clone(),
                            owner_account_id: v.owner_account_id.clone(),
                            state: v.state.clone(),
                            associations: v
                                .associations
                                .iter()
                                .map(|a| ConnectionAliasAssociationView {
                                    connection_identifier: a.connection_identifier.clone(),
                                    resource_id: a.resource_id.clone(),
                                    associated_account_id: a.associated_account_id.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            connection_alias_permissions: state
                .connection_alias_permissions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|p| ConnectionAliasPermissionView {
                                shared_account_id: p.shared_account_id.clone(),
                                allow_association: p.allow_association,
                            })
                            .collect(),
                    )
                })
                .collect(),
            bundles: state
                .bundles
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspaceBundleView {
                            bundle_id: v.bundle_id.clone(),
                            name: v.name.clone(),
                            owner: v.owner.clone(),
                            description: v.description.clone(),
                            bundle_type: v.bundle_type.clone(),
                            compute_type_name: v.compute_type_name.clone(),
                            root_storage_capacity: v.root_storage_capacity,
                            user_storage_capacity: v.user_storage_capacity,
                            creation_time: v.creation_time.clone(),
                        },
                    )
                })
                .collect(),
            pools: state
                .pools
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        WorkspacesPoolView {
                            pool_id: v.pool_id.clone(),
                            pool_arn: v.pool_arn.clone(),
                            pool_name: v.pool_name.clone(),
                            description: v.description.clone(),
                            state: v.state.clone(),
                            bundle_id: v.bundle_id.clone(),
                            directory_id: v.directory_id.clone(),
                            created_at: v.created_at.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<WorkSpacesStateView> for WorkSpacesState {
    fn from(view: WorkSpacesStateView) -> Self {
        WorkSpacesState {
            workspaces: view
                .workspaces
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Workspace {
                            workspace_id: v.workspace_id,
                            directory_id: v.directory_id,
                            user_name: v.user_name,
                            bundle_id: v.bundle_id,
                            state: v.state,
                            ip_address: v.ip_address,
                            computer_name: v.computer_name,
                            subnet_id: v.subnet_id,
                            root_volume_size_gib: v.root_volume_size_gib,
                            user_volume_size_gib: v.user_volume_size_gib,
                            volume_encryption_key: v.volume_encryption_key,
                            user_volume_encryption_enabled: v.user_volume_encryption_enabled,
                            root_volume_encryption_enabled: v.root_volume_encryption_enabled,
                            running_mode: v.running_mode,
                            running_mode_auto_stop_timeout_in_minutes: v
                                .running_mode_auto_stop_timeout_in_minutes,
                        },
                    )
                })
                .collect(),
            directories: view
                .directories
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        WorkspaceDirectory {
                            directory_id: v.directory_id,
                            directory_name: v.directory_name,
                            directory_type: v.directory_type,
                            alias: v.alias,
                            state: v.state,
                            registration_code: v.registration_code,
                            workspace_security_group_id: v.workspace_security_group_id,
                            iam_role_id: v.iam_role_id,
                        },
                    )
                })
                .collect(),
            tags: view
                .tags
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|t| Tag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
            images: view
                .images
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        WorkspaceImage {
                            image_id: v.image_id,
                            name: v.name,
                            description: v.description,
                            state: v.state,
                            operating_system_type: v.operating_system_type,
                            owner_account_id: v.owner_account_id,
                            required_tenancy: v.required_tenancy,
                            created: v.created,
                        },
                    )
                })
                .collect(),
            client_properties: view
                .client_properties
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ClientProperties {
                            reconnect_enabled: v.reconnect_enabled,
                            log_upload_enabled: v.log_upload_enabled,
                        },
                    )
                })
                .collect(),
            selfservice_permissions: view
                .selfservice_permissions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SelfservicePermissions {
                            restart_workspace: v.restart_workspace,
                            increase_volume_size: v.increase_volume_size,
                            change_compute_type: v.change_compute_type,
                            switch_running_mode: v.switch_running_mode,
                            rebuild_workspace: v.rebuild_workspace,
                        },
                    )
                })
                .collect(),
            workspace_creation_properties: view
                .workspace_creation_properties
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        WorkspaceCreationProperties {
                            custom_security_group_id: v.custom_security_group_id,
                            default_ou: v.default_ou,
                            enable_internet_access: v.enable_internet_access,
                            enable_maintenance_mode: v.enable_maintenance_mode,
                            user_enabled_as_local_administrator: v
                                .user_enabled_as_local_administrator,
                            instance_iam_role_arn: v.instance_iam_role_arn,
                        },
                    )
                })
                .collect(),
            image_permissions: view.image_permissions,
            ip_groups: view
                .ip_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        IpGroup {
                            group_id: v.group_id,
                            group_name: v.group_name,
                            group_desc: v.group_desc,
                            user_rules: v
                                .user_rules
                                .into_iter()
                                .map(|r| IpRule {
                                    ip_rule: r.ip_rule,
                                    rule_desc: r.rule_desc,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            connection_aliases: view
                .connection_aliases
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ConnectionAlias {
                            alias_id: v.alias_id,
                            connection_string: v.connection_string,
                            owner_account_id: v.owner_account_id,
                            state: v.state,
                            associations: v
                                .associations
                                .into_iter()
                                .map(|a| ConnectionAliasAssociation {
                                    connection_identifier: a.connection_identifier,
                                    resource_id: a.resource_id,
                                    associated_account_id: a.associated_account_id,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            connection_alias_permissions: view
                .connection_alias_permissions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|p| ConnectionAliasPermission {
                                shared_account_id: p.shared_account_id,
                                allow_association: p.allow_association,
                            })
                            .collect(),
                    )
                })
                .collect(),
            bundles: view
                .bundles
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        WorkspaceBundle {
                            bundle_id: v.bundle_id,
                            name: v.name,
                            owner: v.owner,
                            description: v.description,
                            bundle_type: v.bundle_type,
                            compute_type_name: v.compute_type_name,
                            root_storage_capacity: v.root_storage_capacity,
                            user_storage_capacity: v.user_storage_capacity,
                            creation_time: v.creation_time.clone(),
                        },
                    )
                })
                .collect(),
            pools: view
                .pools
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        WorkspacesPool {
                            pool_id: v.pool_id,
                            pool_arn: v.pool_arn,
                            pool_name: v.pool_name,
                            description: v.description,
                            state: v.state,
                            bundle_id: v.bundle_id,
                            directory_id: v.directory_id,
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for WorkSpacesService {
    type StateView = WorkSpacesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        WorkSpacesStateView::from(&*guard)
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
            *guard = WorkSpacesState::from(view);
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
            let new_state = WorkSpacesState::from(view);
            guard.workspaces.extend(new_state.workspaces);
            guard.directories.extend(new_state.directories);
            guard.tags.extend(new_state.tags);
            guard.images.extend(new_state.images);
            guard.client_properties.extend(new_state.client_properties);
            guard
                .selfservice_permissions
                .extend(new_state.selfservice_permissions);
            guard
                .workspace_creation_properties
                .extend(new_state.workspace_creation_properties);
            guard.image_permissions.extend(new_state.image_permissions);
            guard.ip_groups.extend(new_state.ip_groups);
            guard
                .connection_aliases
                .extend(new_state.connection_aliases);
            guard
                .connection_alias_permissions
                .extend(new_state.connection_alias_permissions);
            guard.bundles.extend(new_state.bundles);
            guard.pools.extend(new_state.pools);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
