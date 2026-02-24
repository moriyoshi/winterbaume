use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SnowDeviceManagementService;
use crate::state::{DeviceRecord, ExecutionRecord, SnowDeviceManagementState, TaskRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnowDeviceManagementStateView {
    #[serde(default)]
    pub tasks: HashMap<String, TaskRecordView>,
    #[serde(default)]
    pub devices: HashMap<String, DeviceRecordView>,
    /// Executions keyed by "{task_id}/{device_id}".
    #[serde(default)]
    pub executions: HashMap<String, ExecutionRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskRecordView {
    pub task_id: String,
    pub task_arn: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    pub created_at: f64,
    pub last_updated_at: f64,
    #[serde(default)]
    pub completed_at: Option<f64>,
    pub command: Value,
    #[serde(default)]
    pub targets: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordView {
    pub managed_device_id: String,
    pub managed_device_arn: String,
    #[serde(default)]
    pub job_id: Option<String>,
    pub state: String,
    pub device_type: String,
    #[serde(default)]
    pub last_reached_out_at: Option<f64>,
    #[serde(default)]
    pub last_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionRecordView {
    pub task_id: String,
    pub managed_device_id: String,
    pub execution_id: String,
    pub state: String,
    pub last_updated_at: f64,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    TaskRecordView,
    TaskRecord {
        task_id,
        task_arn,
        description,
        state,
        created_at,
        last_updated_at,
        completed_at,
        command,
        targets,
        tags,
    }
);

basic_from!(
    DeviceRecordView,
    DeviceRecord {
        managed_device_id,
        managed_device_arn,
        job_id,
        state,
        device_type,
        last_reached_out_at,
        last_updated_at,
    }
);

basic_from!(
    ExecutionRecordView,
    ExecutionRecord {
        task_id,
        managed_device_id,
        execution_id,
        state,
        last_updated_at,
    }
);

impl From<&SnowDeviceManagementState> for SnowDeviceManagementStateView {
    fn from(state: &SnowDeviceManagementState) -> Self {
        Self {
            tasks: state
                .tasks
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            devices: state
                .devices
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            executions: state
                .executions
                .iter()
                .map(|((t, d), v)| (format!("{t}/{d}"), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<SnowDeviceManagementStateView> for SnowDeviceManagementState {
    fn from(view: SnowDeviceManagementStateView) -> Self {
        let executions = view
            .executions
            .into_values()
            .map(|v| {
                let e: ExecutionRecord = v.into();
                ((e.task_id.clone(), e.managed_device_id.clone()), e)
            })
            .collect();
        Self {
            tasks: view.tasks.into_iter().map(|(k, v)| (k, v.into())).collect(),
            devices: view
                .devices
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            executions,
            tags: view.tags,
        }
    }
}

impl StatefulService for SnowDeviceManagementService {
    type StateView = SnowDeviceManagementStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SnowDeviceManagementStateView::from(&*guard)
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
            *guard = SnowDeviceManagementState::from(view);
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
        let merged = SnowDeviceManagementState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.tasks {
                guard.tasks.insert(k, v);
            }
            for (k, v) in merged.devices {
                guard.devices.insert(k, v);
            }
            for (k, v) in merged.executions {
                guard.executions.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
