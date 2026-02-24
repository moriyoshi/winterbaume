use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApplicationCostProfilerService;
use crate::state::ApplicationCostProfilerState;
use crate::types::{ImportJob, ReportDefinition, S3Location};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationCostProfilerStateView {
    #[serde(default)]
    pub reports: HashMap<String, ReportDefinitionView>,
    #[serde(default)]
    pub import_jobs: HashMap<String, ImportJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinitionView {
    pub report_id: String,
    pub report_description: String,
    pub report_frequency: String,
    pub format: String,
    pub destination_s3_location: S3LocationView,
    pub created_at: i64,
    pub last_updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3LocationView {
    pub bucket: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportJobView {
    pub import_id: String,
    pub source_bucket: String,
    pub source_key: String,
    #[serde(default)]
    pub source_region: Option<String>,
    pub created_at: i64,
}

impl From<&S3Location> for S3LocationView {
    fn from(s: &S3Location) -> Self {
        Self {
            bucket: s.bucket.clone(),
            prefix: s.prefix.clone(),
        }
    }
}

impl From<S3LocationView> for S3Location {
    fn from(v: S3LocationView) -> Self {
        Self {
            bucket: v.bucket,
            prefix: v.prefix,
        }
    }
}

impl From<&ReportDefinition> for ReportDefinitionView {
    fn from(r: &ReportDefinition) -> Self {
        Self {
            report_id: r.report_id.clone(),
            report_description: r.report_description.clone(),
            report_frequency: r.report_frequency.clone(),
            format: r.format.clone(),
            destination_s3_location: S3LocationView::from(&r.destination_s3_location),
            created_at: r.created_at,
            last_updated_at: r.last_updated_at,
        }
    }
}

impl From<ReportDefinitionView> for ReportDefinition {
    fn from(v: ReportDefinitionView) -> Self {
        Self {
            report_id: v.report_id,
            report_description: v.report_description,
            report_frequency: v.report_frequency,
            format: v.format,
            destination_s3_location: S3Location::from(v.destination_s3_location),
            created_at: v.created_at,
            last_updated_at: v.last_updated_at,
        }
    }
}

impl From<&ImportJob> for ImportJobView {
    fn from(j: &ImportJob) -> Self {
        Self {
            import_id: j.import_id.clone(),
            source_bucket: j.source_bucket.clone(),
            source_key: j.source_key.clone(),
            source_region: j.source_region.clone(),
            created_at: j.created_at,
        }
    }
}

impl From<ImportJobView> for ImportJob {
    fn from(v: ImportJobView) -> Self {
        Self {
            import_id: v.import_id,
            source_bucket: v.source_bucket,
            source_key: v.source_key,
            source_region: v.source_region,
            created_at: v.created_at,
        }
    }
}

impl From<&ApplicationCostProfilerState> for ApplicationCostProfilerStateView {
    fn from(state: &ApplicationCostProfilerState) -> Self {
        Self {
            reports: state
                .reports
                .iter()
                .map(|(k, v)| (k.clone(), ReportDefinitionView::from(v)))
                .collect(),
            import_jobs: state
                .import_jobs
                .iter()
                .map(|(k, v)| (k.clone(), ImportJobView::from(v)))
                .collect(),
        }
    }
}

impl From<ApplicationCostProfilerStateView> for ApplicationCostProfilerState {
    fn from(view: ApplicationCostProfilerStateView) -> Self {
        Self {
            reports: view
                .reports
                .into_iter()
                .map(|(k, v)| (k, ReportDefinition::from(v)))
                .collect(),
            import_jobs: view
                .import_jobs
                .into_iter()
                .map(|(k, v)| (k, ImportJob::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for ApplicationCostProfilerService {
    type StateView = ApplicationCostProfilerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApplicationCostProfilerStateView::from(&*guard)
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
            *guard = ApplicationCostProfilerState::from(view);
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
                guard.reports.insert(k, ReportDefinition::from(v));
            }
            for (k, v) in view.import_jobs {
                guard.import_jobs.insert(k, ImportJob::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
