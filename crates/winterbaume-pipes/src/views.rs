//! Serde-compatible view types for Pipes state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PipesService;
use crate::state::PipesState;

/// Serializable view of a pipe.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipeView {
    pub name: String,
    pub arn: String,
    pub source: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enrichment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    pub desired_state: String,
    pub current_state: String,
    pub creation_time: String,
    #[serde(default)]
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub enrichment_parameters: Option<serde_json::Value>,
    #[serde(default)]
    pub log_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub source_parameters: Option<serde_json::Value>,
    #[serde(default)]
    pub target_parameters: Option<serde_json::Value>,
}

/// Serializable view of the entire Pipes state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipesStateView {
    #[serde(default)]
    pub pipes: HashMap<String, PipeView>,
}

// --- From internal types to view types ---

fn dt_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

impl From<&crate::types::Pipe> for PipeView {
    fn from(p: &crate::types::Pipe) -> Self {
        PipeView {
            name: p.name.clone(),
            arn: p.arn.clone(),
            source: p.source.clone(),
            target: p.target.clone(),
            description: p.description.clone(),
            enrichment: p.enrichment.clone(),
            role_arn: p.role_arn.clone(),
            desired_state: p.desired_state.clone(),
            current_state: p.current_state.clone(),
            creation_time: dt_to_string(&p.creation_time),
            last_modified_time: dt_to_string(&p.last_modified_time),
            tags: p.tags.clone(),
            enrichment_parameters: None,
            log_configuration: None,
            source_parameters: None,
            target_parameters: None,
        }
    }
}

impl From<&PipesState> for PipesStateView {
    fn from(state: &PipesState) -> Self {
        PipesStateView {
            pipes: state
                .pipes
                .iter()
                .map(|(k, v)| (k.clone(), PipeView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<PipeView> for crate::types::Pipe {
    fn from(v: PipeView) -> Self {
        crate::types::Pipe {
            name: v.name,
            arn: v.arn,
            source: v.source,
            target: v.target,
            description: v.description,
            enrichment: v.enrichment,
            role_arn: v.role_arn,
            desired_state: v.desired_state,
            current_state: v.current_state,
            creation_time: parse_dt(&v.creation_time),
            last_modified_time: if v.last_modified_time.is_empty() {
                parse_dt(&v.creation_time)
            } else {
                parse_dt(&v.last_modified_time)
            },
            tags: v.tags,
        }
    }
}

impl From<PipesStateView> for PipesState {
    fn from(view: PipesStateView) -> Self {
        PipesState {
            pipes: view
                .pipes
                .into_iter()
                .map(|(k, v)| (k, crate::types::Pipe::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for PipesService {
    type StateView = PipesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PipesStateView::from(&*guard)
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
            *guard = PipesState::from(view);
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
            for (name, pipe_view) in view.pipes {
                guard
                    .pipes
                    .insert(name, crate::types::Pipe::from(pipe_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
