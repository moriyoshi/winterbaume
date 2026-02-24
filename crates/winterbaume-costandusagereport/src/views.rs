use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CostAndUsageReportService;
use crate::state::{CostAndUsageReportState, ReportRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostAndUsageReportStateView {
    #[serde(default)]
    pub reports: HashMap<String, ReportRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportRecordView {
    pub report_name: String,
    pub time_unit: String,
    pub format: String,
    pub compression: String,
    #[serde(default)]
    pub additional_schema_elements: Vec<String>,
    pub s3_bucket: String,
    pub s3_prefix: String,
    pub s3_region: String,
    #[serde(default)]
    pub additional_artifacts: Vec<String>,
    #[serde(default)]
    pub refresh_closed_reports: Option<bool>,
    #[serde(default)]
    pub report_versioning: Option<String>,
    #[serde(default)]
    pub billing_view_arn: Option<String>,
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
        report_name,
        time_unit,
        format,
        compression,
        additional_schema_elements,
        s3_bucket,
        s3_prefix,
        s3_region,
        additional_artifacts,
        refresh_closed_reports,
        report_versioning,
        billing_view_arn,
    }
);

impl From<&CostAndUsageReportState> for CostAndUsageReportStateView {
    fn from(state: &CostAndUsageReportState) -> Self {
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

impl From<CostAndUsageReportStateView> for CostAndUsageReportState {
    fn from(view: CostAndUsageReportStateView) -> Self {
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

impl StatefulService for CostAndUsageReportService {
    type StateView = CostAndUsageReportStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CostAndUsageReportStateView::from(&*guard)
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
            *guard = CostAndUsageReportState::from(view);
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
