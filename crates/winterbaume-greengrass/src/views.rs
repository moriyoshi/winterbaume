//! Serde-compatible view types for Greengrass state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::GreengrassService;
use crate::state::GreengrassState;
use crate::types::{
    Definition, DefinitionType, DefinitionVersion, DeploymentInfo, Group, GroupVersionInfo,
    RoleAssociation,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreengrassStateView {
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
    /// Definitions keyed by "{def_type_str}:{def_id}".
    #[serde(default)]
    pub definitions: Vec<DefinitionView>,
    /// Definition versions keyed by "{def_type_str}:{def_id}:{version_id}".
    #[serde(default)]
    pub definition_versions: Vec<DefinitionVersionView>,
    /// Group versions keyed by "{group_id}:{version_id}".
    #[serde(default)]
    pub group_versions: Vec<GroupVersionView>,
    /// Deployments keyed by "{group_id}:{deployment_id}".
    #[serde(default)]
    pub deployments: Vec<DeploymentView>,
    #[serde(default)]
    pub role_associations: HashMap<String, RoleAssociationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub creation_timestamp: String,
    pub last_updated_timestamp: String,
    pub latest_version: String,
    pub latest_version_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionView {
    pub def_type: String,
    pub id: String,
    pub name: String,
    pub arn: String,
    pub creation_timestamp: String,
    pub last_updated_timestamp: String,
    pub latest_version: String,
    pub latest_version_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionVersionView {
    pub def_type: String,
    pub arn: String,
    pub id: String,
    pub version: String,
    pub creation_timestamp: String,
    pub definition_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupVersionView {
    pub group_id: String,
    pub arn: String,
    pub id: String,
    pub version: String,
    pub creation_timestamp: String,
    pub core_definition_version_arn: Option<String>,
    pub device_definition_version_arn: Option<String>,
    pub function_definition_version_arn: Option<String>,
    pub resource_definition_version_arn: Option<String>,
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentView {
    pub group_id: String,
    pub deployment_id: String,
    pub deployment_arn: String,
    pub deployment_type: String,
    pub group_arn: String,
    pub created_at: String,
    pub deployment_status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssociationView {
    pub role_arn: String,
    pub associated_at: String,
}

fn def_type_to_str(dt: DefinitionType) -> &'static str {
    match dt {
        DefinitionType::Connector => "Connector",
        DefinitionType::Core => "Core",
        DefinitionType::Device => "Device",
        DefinitionType::Function => "Function",
        DefinitionType::Logger => "Logger",
        DefinitionType::Resource => "Resource",
        DefinitionType::Subscription => "Subscription",
    }
}

fn str_to_def_type(s: &str) -> DefinitionType {
    match s {
        "Connector" => DefinitionType::Connector,
        "Core" => DefinitionType::Core,
        "Device" => DefinitionType::Device,
        "Function" => DefinitionType::Function,
        "Logger" => DefinitionType::Logger,
        "Resource" => DefinitionType::Resource,
        _ => DefinitionType::Subscription,
    }
}

// --- From internal types to view types ---

impl From<&GreengrassState> for GreengrassStateView {
    fn from(state: &GreengrassState) -> Self {
        GreengrassStateView {
            groups: state
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), GroupView::from(v)))
                .collect(),
            definitions: state
                .definitions
                .iter()
                .map(|((dt, _id), def)| DefinitionView {
                    def_type: def_type_to_str(*dt).to_string(),
                    id: def.id.clone(),
                    name: def.name.clone(),
                    arn: def.arn.clone(),
                    creation_timestamp: def.creation_timestamp.clone(),
                    last_updated_timestamp: def.last_updated_timestamp.clone(),
                    latest_version: def.latest_version.clone(),
                    latest_version_arn: def.latest_version_arn.clone(),
                })
                .collect(),
            definition_versions: state
                .definition_versions
                .iter()
                .map(|((dt, _def_id, _ver_id), dv)| DefinitionVersionView {
                    def_type: def_type_to_str(*dt).to_string(),
                    arn: dv.arn.clone(),
                    id: dv.id.clone(),
                    version: dv.version.clone(),
                    creation_timestamp: dv.creation_timestamp.clone(),
                    definition_id: dv.definition_id.clone(),
                })
                .collect(),
            group_versions: state
                .group_versions
                .iter()
                .map(|((_gid, _vid), gv)| GroupVersionView {
                    group_id: gv.group_id.clone(),
                    arn: gv.arn.clone(),
                    id: gv.id.clone(),
                    version: gv.version.clone(),
                    creation_timestamp: gv.creation_timestamp.clone(),
                    core_definition_version_arn: gv.core_definition_version_arn.clone(),
                    device_definition_version_arn: gv.device_definition_version_arn.clone(),
                    function_definition_version_arn: gv.function_definition_version_arn.clone(),
                    resource_definition_version_arn: gv.resource_definition_version_arn.clone(),
                    subscription_definition_version_arn: gv
                        .subscription_definition_version_arn
                        .clone(),
                })
                .collect(),
            deployments: state
                .deployments
                .iter()
                .map(|((gid, did), dep)| DeploymentView {
                    group_id: gid.clone(),
                    deployment_id: dep.deployment_id.clone(),
                    deployment_arn: dep.deployment_arn.clone(),
                    deployment_type: dep.deployment_type.clone(),
                    group_arn: dep.group_arn.clone(),
                    created_at: dep.created_at.clone(),
                    deployment_status: dep.deployment_status.clone(),
                    updated_at: dep.updated_at.clone(),
                })
                .collect(),
            role_associations: state
                .role_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        RoleAssociationView {
                            role_arn: v.role_arn.clone(),
                            associated_at: v.associated_at.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<&Group> for GroupView {
    fn from(g: &Group) -> Self {
        GroupView {
            id: g.id.clone(),
            name: g.name.clone(),
            arn: g.arn.clone(),
            creation_timestamp: g.creation_timestamp.clone(),
            last_updated_timestamp: g.last_updated_timestamp.clone(),
            latest_version: g.latest_version.clone(),
            latest_version_arn: g.latest_version_arn.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<GreengrassStateView> for GreengrassState {
    fn from(view: GreengrassStateView) -> Self {
        let groups = view
            .groups
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Group {
                        id: v.id,
                        name: v.name,
                        arn: v.arn,
                        creation_timestamp: v.creation_timestamp,
                        last_updated_timestamp: v.last_updated_timestamp,
                        latest_version: v.latest_version,
                        latest_version_arn: v.latest_version_arn,
                    },
                )
            })
            .collect();

        let definitions = view
            .definitions
            .into_iter()
            .map(|dv| {
                let dt = str_to_def_type(&dv.def_type);
                let id = dv.id.clone();
                (
                    (dt, id),
                    Definition {
                        id: dv.id,
                        name: dv.name,
                        arn: dv.arn,
                        creation_timestamp: dv.creation_timestamp,
                        last_updated_timestamp: dv.last_updated_timestamp,
                        latest_version: dv.latest_version,
                        latest_version_arn: dv.latest_version_arn,
                        def_type: dt,
                    },
                )
            })
            .collect();

        let definition_versions = view
            .definition_versions
            .into_iter()
            .map(|dvv| {
                let dt = str_to_def_type(&dvv.def_type);
                let def_id = dvv.definition_id.clone();
                let ver_id = dvv.version.clone();
                (
                    (dt, def_id, ver_id),
                    DefinitionVersion {
                        arn: dvv.arn,
                        id: dvv.id,
                        version: dvv.version,
                        creation_timestamp: dvv.creation_timestamp,
                        definition_id: dvv.definition_id,
                    },
                )
            })
            .collect();

        let group_versions = view
            .group_versions
            .into_iter()
            .map(|gv| {
                let gid = gv.group_id.clone();
                let vid = gv.version.clone();
                (
                    (gid, vid),
                    GroupVersionInfo {
                        arn: gv.arn,
                        id: gv.id,
                        version: gv.version,
                        creation_timestamp: gv.creation_timestamp,
                        group_id: gv.group_id,
                        core_definition_version_arn: gv.core_definition_version_arn,
                        device_definition_version_arn: gv.device_definition_version_arn,
                        function_definition_version_arn: gv.function_definition_version_arn,
                        resource_definition_version_arn: gv.resource_definition_version_arn,
                        subscription_definition_version_arn: gv.subscription_definition_version_arn,
                    },
                )
            })
            .collect();

        let deployments = view
            .deployments
            .into_iter()
            .map(|dep| {
                let gid = dep.group_id.clone();
                let did = dep.deployment_id.clone();
                (
                    (gid, did),
                    DeploymentInfo {
                        deployment_id: dep.deployment_id,
                        deployment_arn: dep.deployment_arn,
                        deployment_type: dep.deployment_type,
                        group_arn: dep.group_arn,
                        created_at: dep.created_at,
                        deployment_status: dep.deployment_status,
                        updated_at: dep.updated_at,
                    },
                )
            })
            .collect();

        let role_associations = view
            .role_associations
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    RoleAssociation {
                        role_arn: v.role_arn,
                        associated_at: v.associated_at,
                    },
                )
            })
            .collect();

        GreengrassState {
            groups,
            definitions,
            definition_versions,
            group_versions,
            deployments,
            role_associations,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for GreengrassService {
    type StateView = GreengrassStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        GreengrassStateView::from(&*guard)
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
            *guard = GreengrassState::from(view);
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
            let incoming = GreengrassState::from(view);
            for (k, v) in incoming.groups {
                guard.groups.insert(k, v);
            }
            for (k, v) in incoming.definitions {
                guard.definitions.insert(k, v);
            }
            for (k, v) in incoming.definition_versions {
                guard.definition_versions.insert(k, v);
            }
            for (k, v) in incoming.group_versions {
                guard.group_versions.insert(k, v);
            }
            for (k, v) in incoming.deployments {
                guard.deployments.insert(k, v);
            }
            for (k, v) in incoming.role_associations {
                guard.role_associations.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
