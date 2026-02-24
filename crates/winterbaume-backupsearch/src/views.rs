//! Serde-compatible view types for BackupSearch state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BackupSearchService;
use crate::state::BackupSearchState;
use crate::types::{SearchJob, SearchResultExportJob};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupSearchStateView {
    #[serde(default)]
    pub search_jobs: HashMap<String, SearchJobView>,
    #[serde(default)]
    pub export_jobs: HashMap<String, SearchResultExportJobView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchJobView {
    pub identifier: String,
    pub arn: String,
    #[serde(default)]
    pub name: Option<String>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub encryption_key_arn: Option<String>,
    #[serde(default)]
    pub search_scope: Option<Value>,
    #[serde(default)]
    pub item_filters: Option<Value>,
    pub creation_time: i64,
    #[serde(default)]
    pub completion_time: Option<i64>,
    #[serde(default)]
    pub items_matched: i64,
    #[serde(default)]
    pub items_scanned: i64,
    #[serde(default)]
    pub recovery_points_scanned: i32,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchResultExportJobView {
    pub identifier: String,
    pub arn: String,
    pub search_job_identifier: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub export_specification: Option<Value>,
    #[serde(default)]
    pub role_arn: Option<String>,
    pub creation_time: i64,
    #[serde(default)]
    pub completion_time: Option<i64>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
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
    SearchJobView,
    SearchJob {
        identifier,
        arn,
        name,
        status,
        status_message,
        encryption_key_arn,
        search_scope,
        item_filters,
        creation_time,
        completion_time,
        items_matched,
        items_scanned,
        recovery_points_scanned,
        tags
    }
);

basic_from!(
    SearchResultExportJobView,
    SearchResultExportJob {
        identifier,
        arn,
        search_job_identifier,
        status,
        status_message,
        export_specification,
        role_arn,
        creation_time,
        completion_time,
        tags
    }
);

impl From<&BackupSearchState> for BackupSearchStateView {
    fn from(state: &BackupSearchState) -> Self {
        Self {
            search_jobs: state
                .search_jobs
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            export_jobs: state
                .export_jobs
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<BackupSearchStateView> for BackupSearchState {
    fn from(view: BackupSearchStateView) -> Self {
        Self {
            search_jobs: view
                .search_jobs
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            export_jobs: view
                .export_jobs
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for BackupSearchService {
    type StateView = BackupSearchStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BackupSearchStateView::from(&*guard)
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
            *guard = BackupSearchState::from(view);
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
            for (k, v) in view.search_jobs {
                guard.search_jobs.insert(k, v.into());
            }
            for (k, v) in view.export_jobs {
                guard.export_jobs.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
