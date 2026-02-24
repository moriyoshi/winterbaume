use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApplicationDiscoveryService;
use crate::state::ApplicationDiscoveryState;
use crate::types::{
    Application, BatchDeleteTask, ConfigurationTag, ContinuousExport, ExportTask, ImportTask,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationDiscoveryStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    #[serde(default)]
    pub tags: Vec<ConfigurationTagView>,
    #[serde(default)]
    pub import_tasks: HashMap<String, ImportTaskView>,
    #[serde(default)]
    pub export_tasks: HashMap<String, ExportTaskView>,
    #[serde(default)]
    pub continuous_exports: HashMap<String, ContinuousExportView>,
    #[serde(default)]
    pub batch_delete_tasks: HashMap<String, BatchDeleteTaskView>,
    #[serde(default)]
    pub associations: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationView {
    pub configuration_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub wave: Option<String>,
    pub time_of_creation: i64,
    pub last_modified_time_stamp: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigurationTagView {
    pub configuration_id: String,
    pub configuration_type: String,
    pub key: String,
    pub value: String,
    pub time_of_creation: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportTaskView {
    pub id: String,
    pub name: String,
    pub status: String,
    pub import_url: String,
    pub import_request_time: i64,
    #[serde(default)]
    pub import_completion_time: Option<i64>,
    #[serde(default)]
    pub server_import_success: i32,
    #[serde(default)]
    pub server_import_failure: i32,
    #[serde(default)]
    pub application_import_success: i32,
    #[serde(default)]
    pub application_import_failure: i32,
    #[serde(default)]
    pub error_url: Option<String>,
    #[serde(default)]
    pub import_deleted_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportTaskView {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub configurations_download_url: Option<String>,
    pub export_request_time: i64,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default)]
    pub requested_start_time: Option<i64>,
    #[serde(default)]
    pub requested_end_time: Option<i64>,
    pub data_source: String,
    #[serde(default)]
    pub filters: Vec<Value>,
    #[serde(default)]
    pub preferences: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContinuousExportView {
    pub export_id: String,
    pub status: String,
    #[serde(default)]
    pub status_detail: Option<String>,
    #[serde(default)]
    pub s3_bucket: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub stop_time: Option<i64>,
    pub data_source: String,
    #[serde(default)]
    pub schema_storage_config: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteTaskView {
    pub id: String,
    pub configuration_type: String,
    pub status: String,
    pub start_time: i64,
    #[serde(default)]
    pub end_time: Option<i64>,
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
    ApplicationView,
    Application {
        configuration_id,
        name,
        description,
        wave,
        time_of_creation,
        last_modified_time_stamp
    }
);

basic_from!(
    ConfigurationTagView,
    ConfigurationTag {
        configuration_id,
        configuration_type,
        key,
        value,
        time_of_creation
    }
);

basic_from!(
    ImportTaskView,
    ImportTask {
        id,
        name,
        status,
        import_url,
        import_request_time,
        import_completion_time,
        server_import_success,
        server_import_failure,
        application_import_success,
        application_import_failure,
        error_url,
        import_deleted_time
    }
);

basic_from!(
    ExportTaskView,
    ExportTask {
        id,
        status,
        status_message,
        configurations_download_url,
        export_request_time,
        is_truncated,
        requested_start_time,
        requested_end_time,
        data_source,
        filters,
        preferences
    }
);

basic_from!(
    ContinuousExportView,
    ContinuousExport {
        export_id,
        status,
        status_detail,
        s3_bucket,
        start_time,
        stop_time,
        data_source,
        schema_storage_config
    }
);

basic_from!(
    BatchDeleteTaskView,
    BatchDeleteTask {
        id,
        configuration_type,
        status,
        start_time,
        end_time
    }
);

impl From<&ApplicationDiscoveryState> for ApplicationDiscoveryStateView {
    fn from(state: &ApplicationDiscoveryState) -> Self {
        Self {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.iter().map(Into::into).collect(),
            import_tasks: state
                .import_tasks
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            export_tasks: state
                .export_tasks
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            continuous_exports: state
                .continuous_exports
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            batch_delete_tasks: state
                .batch_delete_tasks
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            associations: state.associations.clone(),
        }
    }
}

impl From<ApplicationDiscoveryStateView> for ApplicationDiscoveryState {
    fn from(view: ApplicationDiscoveryStateView) -> Self {
        Self {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            tags: view.tags.into_iter().map(Into::into).collect(),
            import_tasks: view
                .import_tasks
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            export_tasks: view
                .export_tasks
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            continuous_exports: view
                .continuous_exports
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            batch_delete_tasks: view
                .batch_delete_tasks
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            associations: view.associations,
        }
    }
}

impl StatefulService for ApplicationDiscoveryService {
    type StateView = ApplicationDiscoveryStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApplicationDiscoveryStateView::from(&*guard)
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
            *guard = ApplicationDiscoveryState::from(view);
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
            for (k, v) in view.applications {
                guard.applications.insert(k, v.into());
            }
            for t in view.tags {
                guard.tags.push(t.into());
            }
            for (k, v) in view.import_tasks {
                guard.import_tasks.insert(k, v.into());
            }
            for (k, v) in view.export_tasks {
                guard.export_tasks.insert(k, v.into());
            }
            for (k, v) in view.continuous_exports {
                guard.continuous_exports.insert(k, v.into());
            }
            for (k, v) in view.batch_delete_tasks {
                guard.batch_delete_tasks.insert(k, v.into());
            }
            for (k, v) in view.associations {
                guard.associations.entry(k).or_default().extend(v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
