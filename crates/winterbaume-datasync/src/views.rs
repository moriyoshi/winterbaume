//! Serde-compatible view types for DataSync state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DataSyncService;
use crate::state::DataSyncState;
use crate::types::{DataSyncLocation, DataSyncTask, TaskExecution};

/// Serializable view of the entire DataSync state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataSyncStateView {
    /// Tasks keyed by TaskArn.
    #[serde(default)]
    pub tasks: HashMap<String, DataSyncTaskView>,
    /// Locations keyed by LocationArn.
    #[serde(default)]
    pub locations: HashMap<String, DataSyncLocationView>,
    /// Task executions keyed by TaskExecutionArn.
    #[serde(default)]
    pub task_executions: HashMap<String, TaskExecutionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSyncTaskView {
    pub task_arn: String,
    pub name: Option<String>,
    pub status: String,
    pub source_location_arn: String,
    pub destination_location_arn: String,
    pub cloud_watch_log_group_arn: Option<String>,
    pub creation_time: DateTime<Utc>,
    #[serde(default)]
    pub excludes: Option<serde_json::Value>,
    #[serde(default)]
    pub includes: Option<serde_json::Value>,
    #[serde(default)]
    pub schedule: Option<serde_json::Value>,
    #[serde(default)]
    pub task_report_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSyncLocationView {
    pub location_arn: String,
    pub location_uri: String,
    pub creation_time: DateTime<Utc>,
    #[serde(default)]
    pub s3_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionView {
    pub task_execution_arn: String,
    pub task_arn: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
}

// --- From internal types to view types ---

impl From<&DataSyncState> for DataSyncStateView {
    fn from(state: &DataSyncState) -> Self {
        DataSyncStateView {
            tasks: state
                .tasks
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DataSyncTaskView {
                            task_arn: v.task_arn.clone(),
                            name: v.name.clone(),
                            status: v.status.clone(),
                            source_location_arn: v.source_location_arn.clone(),
                            destination_location_arn: v.destination_location_arn.clone(),
                            cloud_watch_log_group_arn: v.cloud_watch_log_group_arn.clone(),
                            creation_time: v.creation_time,
                            excludes: None,
                            includes: None,
                            schedule: None,
                            task_report_config: None,
                        },
                    )
                })
                .collect(),
            locations: state
                .locations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DataSyncLocationView {
                            location_arn: v.location_arn.clone(),
                            location_uri: v.location_uri.clone(),
                            creation_time: v.creation_time,
                            s3_config: None,
                        },
                    )
                })
                .collect(),
            task_executions: state
                .task_executions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TaskExecutionView {
                            task_execution_arn: v.task_execution_arn.clone(),
                            task_arn: v.task_arn.clone(),
                            status: v.status.clone(),
                            start_time: v.start_time,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<DataSyncStateView> for DataSyncState {
    fn from(view: DataSyncStateView) -> Self {
        DataSyncState {
            tasks: view
                .tasks
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DataSyncTask {
                            task_arn: v.task_arn,
                            name: v.name,
                            status: v.status,
                            source_location_arn: v.source_location_arn,
                            destination_location_arn: v.destination_location_arn,
                            cloud_watch_log_group_arn: v.cloud_watch_log_group_arn,
                            creation_time: v.creation_time,
                        },
                    )
                })
                .collect(),
            locations: view
                .locations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DataSyncLocation {
                            location_arn: v.location_arn,
                            location_uri: v.location_uri,
                            creation_time: v.creation_time,
                        },
                    )
                })
                .collect(),
            task_executions: view
                .task_executions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        TaskExecution {
                            task_execution_arn: v.task_execution_arn,
                            task_arn: v.task_arn,
                            status: v.status,
                            start_time: v.start_time,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DataSyncService {
    type StateView = DataSyncStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DataSyncStateView::from(&*guard)
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
            *guard = DataSyncState::from(view);
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
            let incoming = DataSyncState::from(view);
            for (k, v) in incoming.tasks {
                guard.tasks.insert(k, v);
            }
            for (k, v) in incoming.locations {
                guard.locations.insert(k, v);
            }
            for (k, v) in incoming.task_executions {
                guard.task_executions.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
