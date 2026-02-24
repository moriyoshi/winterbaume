//! Serde-compatible view types for Cloud Directory state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudDirectoryService;
use crate::state::CloudDirectoryState;
use crate::types::{Directory, Schema, SchemaType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDirectoryStateView {
    #[serde(default)]
    pub directories: HashMap<String, DirectoryView>,
    #[serde(default)]
    pub schemas: HashMap<String, SchemaView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryView {
    pub directory_arn: String,
    pub name: String,
    pub schema_arn: String,
    pub state: String,
    pub created_date_time: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaView {
    pub schema_arn: String,
    pub name: String,
    pub type_: String,
    #[serde(default)]
    pub published_arns: Vec<String>,
}

impl From<&CloudDirectoryState> for CloudDirectoryStateView {
    fn from(state: &CloudDirectoryState) -> Self {
        CloudDirectoryStateView {
            directories: state
                .directories
                .iter()
                .map(|(k, d)| {
                    (
                        k.clone(),
                        DirectoryView {
                            directory_arn: d.directory_arn.clone(),
                            name: d.name.clone(),
                            schema_arn: d.schema_arn.clone(),
                            state: d.state.clone(),
                            created_date_time: d.created_date_time,
                            tags: d.tags.clone(),
                        },
                    )
                })
                .collect(),
            schemas: state
                .schemas
                .iter()
                .map(|(k, s)| {
                    (
                        k.clone(),
                        SchemaView {
                            schema_arn: s.schema_arn.clone(),
                            name: s.name.clone(),
                            type_: match s.type_ {
                                SchemaType::Development => "Development".to_string(),
                                SchemaType::Published => "Published".to_string(),
                            },
                            published_arns: s.published_arns.clone(),
                        },
                    )
                })
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

impl From<CloudDirectoryStateView> for CloudDirectoryState {
    fn from(view: CloudDirectoryStateView) -> Self {
        CloudDirectoryState {
            directories: view
                .directories
                .into_iter()
                .map(|(k, d)| {
                    (
                        k,
                        Directory {
                            directory_arn: d.directory_arn,
                            name: d.name,
                            schema_arn: d.schema_arn,
                            state: d.state,
                            created_date_time: d.created_date_time,
                            tags: d.tags,
                        },
                    )
                })
                .collect(),
            schemas: view
                .schemas
                .into_iter()
                .map(|(k, s)| {
                    (
                        k,
                        Schema {
                            schema_arn: s.schema_arn,
                            name: s.name,
                            type_: if s.type_ == "Development" {
                                SchemaType::Development
                            } else {
                                SchemaType::Published
                            },
                            published_arns: s.published_arns,
                        },
                    )
                })
                .collect(),
            resource_tags: view.resource_tags,
        }
    }
}

impl StatefulService for CloudDirectoryService {
    type StateView = CloudDirectoryStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudDirectoryStateView::from(&*guard)
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
            *guard = CloudDirectoryState::from(view);
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
            let merged = CloudDirectoryState::from(view);
            for (k, v) in merged.directories {
                guard.directories.insert(k, v);
            }
            for (k, v) in merged.schemas {
                guard.schemas.insert(k, v);
            }
            for (k, v) in merged.resource_tags {
                guard.resource_tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
