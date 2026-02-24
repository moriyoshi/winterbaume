//! Serde-compatible view types for ResourceGroups state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ResourceGroupsService;
use crate::state::ResourceGroupsState;
use crate::types::{
    AccountSettings, GroupConfigParam, GroupConfigurationEntry, ResourceGroup, TagSyncTask,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceGroupsStateView {
    #[serde(default)]
    pub groups: HashMap<String, ResourceGroupView>,
    #[serde(default)]
    pub tag_sync_tasks: HashMap<String, TagSyncTaskView>,
    #[serde(default)]
    pub next_task_id: u64,
    #[serde(default)]
    pub account_settings: AccountSettingsView,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountSettingsView {
    #[serde(default)]
    pub group_lifecycle_events_desired_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroupView {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub resource_query_type: String,
    pub resource_query_query: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub configuration: Option<Vec<GroupConfigEntryView>>,
    #[serde(default)]
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfigEntryView {
    pub config_type: String,
    #[serde(default)]
    pub parameters: Vec<GroupConfigParamView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfigParamView {
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSyncTaskView {
    pub task_arn: String,
    pub group_name: String,
    pub group_arn: String,
    pub tag_key: String,
    pub tag_value: String,
    pub role_arn: String,
    pub status: String,
    pub created_at: f64,
}

impl From<&ResourceGroupsState> for ResourceGroupsStateView {
    fn from(state: &ResourceGroupsState) -> Self {
        ResourceGroupsStateView {
            groups: state
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), ResourceGroupView::from(v)))
                .collect(),
            tag_sync_tasks: state
                .tag_sync_tasks
                .iter()
                .map(|(k, v)| (k.clone(), TagSyncTaskView::from(v)))
                .collect(),
            next_task_id: state.next_task_id,
            account_settings: AccountSettingsView {
                group_lifecycle_events_desired_status: state
                    .account_settings
                    .group_lifecycle_events_desired_status
                    .clone(),
            },
        }
    }
}

impl From<&ResourceGroup> for ResourceGroupView {
    fn from(g: &ResourceGroup) -> Self {
        ResourceGroupView {
            name: g.name.clone(),
            arn: g.arn.clone(),
            description: g.description.clone(),
            resource_query_type: g.resource_query_type.clone(),
            resource_query_query: g.resource_query_query.clone(),
            tags: g.tags.clone(),
            configuration: g.configuration.as_ref().map(|configs| {
                configs
                    .iter()
                    .map(|c| GroupConfigEntryView {
                        config_type: c.config_type.clone(),
                        parameters: c
                            .parameters
                            .iter()
                            .map(|p| GroupConfigParamView {
                                name: p.name.clone(),
                                values: p.values.clone(),
                            })
                            .collect(),
                    })
                    .collect()
            }),
            resource_arns: g.resource_arns.clone(),
        }
    }
}

impl From<&TagSyncTask> for TagSyncTaskView {
    fn from(t: &TagSyncTask) -> Self {
        TagSyncTaskView {
            task_arn: t.task_arn.clone(),
            group_name: t.group_name.clone(),
            group_arn: t.group_arn.clone(),
            tag_key: t.tag_key.clone(),
            tag_value: t.tag_value.clone(),
            role_arn: t.role_arn.clone(),
            status: t.status.clone(),
            created_at: t.created_at,
        }
    }
}

impl From<ResourceGroupsStateView> for ResourceGroupsState {
    fn from(view: ResourceGroupsStateView) -> Self {
        ResourceGroupsState {
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| (k, ResourceGroup::from(v)))
                .collect(),
            tag_sync_tasks: view
                .tag_sync_tasks
                .into_iter()
                .map(|(k, v)| (k, TagSyncTask::from(v)))
                .collect(),
            next_task_id: view.next_task_id,
            account_settings: AccountSettings {
                group_lifecycle_events_desired_status: view
                    .account_settings
                    .group_lifecycle_events_desired_status,
            },
        }
    }
}

impl From<ResourceGroupView> for ResourceGroup {
    fn from(v: ResourceGroupView) -> Self {
        ResourceGroup {
            name: v.name,
            arn: v.arn,
            description: v.description,
            resource_query_type: v.resource_query_type,
            resource_query_query: v.resource_query_query,
            tags: v.tags,
            configuration: v.configuration.map(|configs| {
                configs
                    .into_iter()
                    .map(|c| GroupConfigurationEntry {
                        config_type: c.config_type,
                        parameters: c
                            .parameters
                            .into_iter()
                            .map(|p| GroupConfigParam {
                                name: p.name,
                                values: p.values,
                            })
                            .collect(),
                    })
                    .collect()
            }),
            resource_arns: v.resource_arns,
        }
    }
}

impl From<TagSyncTaskView> for TagSyncTask {
    fn from(v: TagSyncTaskView) -> Self {
        TagSyncTask {
            task_arn: v.task_arn,
            group_name: v.group_name,
            group_arn: v.group_arn,
            tag_key: v.tag_key,
            tag_value: v.tag_value,
            role_arn: v.role_arn,
            status: v.status,
            created_at: v.created_at,
        }
    }
}

impl StatefulService for ResourceGroupsService {
    type StateView = ResourceGroupsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ResourceGroupsStateView::from(&*guard)
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
            *guard = ResourceGroupsState::from(view);
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
            for (k, v) in view.groups {
                guard.groups.insert(k, ResourceGroup::from(v));
            }
            for (k, v) in view.tag_sync_tasks {
                guard.tag_sync_tasks.insert(k, TagSyncTask::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
