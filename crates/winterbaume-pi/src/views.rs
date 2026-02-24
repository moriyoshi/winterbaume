use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PiService;
use crate::state::{PiState, ReportRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PiStateView {
    #[serde(default)]
    pub reports: HashMap<String, ReportRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportRecordView {
    pub analysis_report_id: String,
    pub identifier: String,
    pub service_type: String,
    pub start_time: f64,
    pub end_time: f64,
    pub status: String,
    pub create_time: f64,
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
    ReportRecordView,
    ReportRecord {
        analysis_report_id,
        identifier,
        service_type,
        start_time,
        end_time,
        status,
        create_time,
        tags,
    }
);

impl From<&PiState> for PiStateView {
    fn from(state: &PiState) -> Self {
        Self {
            reports: state
                .reports
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<PiStateView> for PiState {
    fn from(view: PiStateView) -> Self {
        Self {
            reports: view
                .reports
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            tags: view.tags,
        }
    }
}

impl StatefulService for PiService {
    type StateView = PiStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PiStateView::from(&*guard)
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
            *guard = PiState::from(view);
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
            for (k, v) in view.reports {
                guard.reports.insert(k, v.into());
            }
            for (k, v) in view.tags {
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
