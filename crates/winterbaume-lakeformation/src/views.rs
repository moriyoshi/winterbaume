//! Serde-compatible view types for LakeFormation state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::LakeFormationService;
use crate::state::LakeFormationState;
use crate::types::{
    DataLakePrincipal, DataLakeSettings, LFTag, LFTagPair, PermissionGrant, PrincipalPermissions,
    RegisteredResource,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LakeFormationStateView {
    #[serde(default)]
    pub resources: HashMap<String, RegisteredResourceView>,
    #[serde(default)]
    pub data_lake_settings: DataLakeSettingsView,
    #[serde(default)]
    pub lf_tags: HashMap<String, LFTagView>,
    /// Tag assignments: resource key string -> list of LFTagPairView.
    #[serde(default)]
    pub tag_assignments: HashMap<String, Vec<LFTagPairView>>,
    #[serde(default)]
    pub permissions: Vec<PermissionGrantView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredResourceView {
    pub resource_arn: String,
    pub role_arn: Option<String>,
    pub use_service_linked_role: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataLakeSettingsView {
    #[serde(default)]
    pub data_lake_admins: Vec<String>,
    pub allow_external_data_filtering: bool,
    #[serde(default)]
    pub authorized_session_tag_value_list: Vec<String>,
    #[serde(default)]
    pub create_database_default_permissions: Vec<PrincipalPermissionsView>,
    #[serde(default)]
    pub create_table_default_permissions: Vec<PrincipalPermissionsView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalPermissionsView {
    pub principal: String,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct LFTagView {
    pub tag_key: String,
    #[serde(default)]
    pub tag_values: Vec<String>,
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct LFTagPairView {
    pub catalog_id: Option<String>,
    pub tag_key: String,
    #[serde(default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGrantView {
    pub principal: String,
    pub resource: Value,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub permissions_with_grant_option: Vec<String>,
}

// --- From internal types to view types ---

impl From<&LakeFormationState> for LakeFormationStateView {
    fn from(state: &LakeFormationState) -> Self {
        LakeFormationStateView {
            resources: state
                .resources
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        RegisteredResourceView {
                            resource_arn: v.resource_arn.clone(),
                            role_arn: v.role_arn.clone(),
                            use_service_linked_role: v.use_service_linked_role,
                        },
                    )
                })
                .collect(),
            data_lake_settings: DataLakeSettingsView {
                data_lake_admins: state
                    .data_lake_settings
                    .data_lake_admins
                    .iter()
                    .map(|p| p.data_lake_principal_identifier.clone())
                    .collect(),
                allow_external_data_filtering: state
                    .data_lake_settings
                    .allow_external_data_filtering,
                authorized_session_tag_value_list: state
                    .data_lake_settings
                    .authorized_session_tag_value_list
                    .clone(),
                create_database_default_permissions: state
                    .data_lake_settings
                    .create_database_default_permissions
                    .iter()
                    .map(|pp| PrincipalPermissionsView {
                        principal: pp.principal.data_lake_principal_identifier.clone(),
                        permissions: pp.permissions.clone(),
                    })
                    .collect(),
                create_table_default_permissions: state
                    .data_lake_settings
                    .create_table_default_permissions
                    .iter()
                    .map(|pp| PrincipalPermissionsView {
                        principal: pp.principal.data_lake_principal_identifier.clone(),
                        permissions: pp.permissions.clone(),
                    })
                    .collect(),
            },
            lf_tags: state
                .lf_tags
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        LFTagView {
                            tag_key: v.tag_key.clone(),
                            tag_values: v.tag_values.clone(),
                            catalog_id: v.catalog_id.clone(),
                        },
                    )
                })
                .collect(),
            tag_assignments: state
                .tag_assignments
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|pair| LFTagPairView {
                                catalog_id: pair.catalog_id.clone(),
                                tag_key: pair.tag_key.clone(),
                                tag_values: pair.tag_values.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            permissions: state
                .permissions
                .iter()
                .map(|pg| PermissionGrantView {
                    principal: pg.principal.clone(),
                    resource: pg.resource.clone(),
                    permissions: pg.permissions.clone(),
                    permissions_with_grant_option: pg.permissions_with_grant_option.clone(),
                })
                .collect(),
        }
    }
}

fn settings_from_view(view: DataLakeSettingsView) -> DataLakeSettings {
    DataLakeSettings {
        data_lake_admins: view
            .data_lake_admins
            .into_iter()
            .map(|s| DataLakePrincipal {
                data_lake_principal_identifier: s,
            })
            .collect(),
        allow_external_data_filtering: view.allow_external_data_filtering,
        authorized_session_tag_value_list: view.authorized_session_tag_value_list,
        create_database_default_permissions: view
            .create_database_default_permissions
            .into_iter()
            .map(|pp| PrincipalPermissions {
                principal: DataLakePrincipal {
                    data_lake_principal_identifier: pp.principal,
                },
                permissions: pp.permissions,
            })
            .collect(),
        create_table_default_permissions: view
            .create_table_default_permissions
            .into_iter()
            .map(|pp| PrincipalPermissions {
                principal: DataLakePrincipal {
                    data_lake_principal_identifier: pp.principal,
                },
                permissions: pp.permissions,
            })
            .collect(),
    }
}

// --- From view types to internal types ---

impl From<LakeFormationStateView> for LakeFormationState {
    fn from(view: LakeFormationStateView) -> Self {
        LakeFormationState {
            resources: view
                .resources
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        RegisteredResource {
                            resource_arn: v.resource_arn,
                            role_arn: v.role_arn,
                            use_service_linked_role: v.use_service_linked_role,
                        },
                    )
                })
                .collect(),
            data_lake_settings: settings_from_view(view.data_lake_settings),
            lf_tags: view
                .lf_tags
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        LFTag {
                            tag_key: v.tag_key,
                            tag_values: v.tag_values,
                            catalog_id: v.catalog_id,
                        },
                    )
                })
                .collect(),
            tag_assignments: view
                .tag_assignments
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|pair| LFTagPair {
                                catalog_id: pair.catalog_id,
                                tag_key: pair.tag_key,
                                tag_values: pair.tag_values,
                            })
                            .collect(),
                    )
                })
                .collect(),
            permissions: view
                .permissions
                .into_iter()
                .map(|pg| PermissionGrant {
                    principal: pg.principal,
                    resource: pg.resource,
                    permissions: pg.permissions,
                    permissions_with_grant_option: pg.permissions_with_grant_option,
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for LakeFormationService {
    type StateView = LakeFormationStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        LakeFormationStateView::from(&*guard)
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
            *guard = LakeFormationState::from(view);
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
            let incoming = LakeFormationState::from(view);
            for (k, v) in incoming.resources {
                guard.resources.insert(k, v);
            }
            for (k, v) in incoming.lf_tags {
                guard.lf_tags.insert(k, v);
            }
            for (k, v) in incoming.tag_assignments {
                guard.tag_assignments.insert(k, v);
            }
            for pg in incoming.permissions {
                guard.permissions.push(pg);
            }
            // Merge data lake settings (non-default values override)
            if !incoming.data_lake_settings.data_lake_admins.is_empty() {
                guard.data_lake_settings.data_lake_admins =
                    incoming.data_lake_settings.data_lake_admins;
            }
            if incoming.data_lake_settings.allow_external_data_filtering {
                guard.data_lake_settings.allow_external_data_filtering = true;
            }
            if !incoming
                .data_lake_settings
                .authorized_session_tag_value_list
                .is_empty()
            {
                guard.data_lake_settings.authorized_session_tag_value_list = incoming
                    .data_lake_settings
                    .authorized_session_tag_value_list;
            }
            if !incoming
                .data_lake_settings
                .create_database_default_permissions
                .is_empty()
            {
                guard.data_lake_settings.create_database_default_permissions = incoming
                    .data_lake_settings
                    .create_database_default_permissions;
            }
            if !incoming
                .data_lake_settings
                .create_table_default_permissions
                .is_empty()
            {
                guard.data_lake_settings.create_table_default_permissions =
                    incoming.data_lake_settings.create_table_default_permissions;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
