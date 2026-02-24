//! Serde-compatible view types for OSIS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::OsisService;
use crate::state::OsisState;

/// Serializable view of a single OSIS pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineView {
    pub pipeline_name: String,
    pub pipeline_arn: String,
    pub min_units: i32,
    pub max_units: i32,
    pub status: String,
    pub pipeline_configuration_body: String,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub ingest_endpoint_urls: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub buffer_options: Option<serde_json::Value>,
    #[serde(default)]
    pub encryption_at_rest_options: Option<serde_json::Value>,
    #[serde(default)]
    pub log_publishing_options: Option<serde_json::Value>,
    #[serde(default)]
    pub vpc_options: Option<serde_json::Value>,
}

/// Serializable view of the entire OSIS state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OsisStateView {
    #[serde(default)]
    pub pipelines: HashMap<String, PipelineView>,
}

// --- From internal types to view types ---

impl From<&crate::types::Pipeline> for PipelineView {
    fn from(p: &crate::types::Pipeline) -> Self {
        PipelineView {
            pipeline_name: p.pipeline_name.clone(),
            pipeline_arn: p.pipeline_arn.clone(),
            min_units: p.min_units,
            max_units: p.max_units,
            status: p.status.clone(),
            pipeline_configuration_body: p.pipeline_configuration_body.clone(),
            created_at: p.created_at.to_rfc3339(),
            last_updated_at: p.last_updated_at.to_rfc3339(),
            ingest_endpoint_urls: p.ingest_endpoint_urls.clone(),
            tags: p.tags.clone(),
            buffer_options: None,
            encryption_at_rest_options: None,
            log_publishing_options: None,
            vpc_options: None,
        }
    }
}

impl From<&OsisState> for OsisStateView {
    fn from(state: &OsisState) -> Self {
        OsisStateView {
            pipelines: state
                .pipelines
                .iter()
                .map(|(k, v)| (k.clone(), PipelineView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

fn parse_dt(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<PipelineView> for crate::types::Pipeline {
    fn from(v: PipelineView) -> Self {
        crate::types::Pipeline {
            pipeline_name: v.pipeline_name,
            pipeline_arn: v.pipeline_arn,
            min_units: v.min_units,
            max_units: v.max_units,
            status: v.status,
            pipeline_configuration_body: v.pipeline_configuration_body,
            created_at: parse_dt(&v.created_at),
            last_updated_at: parse_dt(&v.last_updated_at),
            ingest_endpoint_urls: v.ingest_endpoint_urls,
            tags: v.tags,
        }
    }
}

impl From<OsisStateView> for OsisState {
    fn from(view: OsisStateView) -> Self {
        OsisState {
            pipelines: view
                .pipelines
                .into_iter()
                .map(|(k, v)| (k, crate::types::Pipeline::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for OsisService {
    type StateView = OsisStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        OsisStateView::from(&*guard)
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
            *guard = OsisState::from(view);
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
            for (name, pipeline_view) in view.pipelines {
                guard
                    .pipelines
                    .insert(name, crate::types::Pipeline::from(pipeline_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
