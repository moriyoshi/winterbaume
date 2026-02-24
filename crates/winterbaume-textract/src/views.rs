//! Serde-compatible view types for Textract state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TextractService;
use crate::state::{TextractJob, TextractState};
use crate::wire::Block;

/// Serialisable view of a single async Textract job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextractJobView {
    /// The action type (e.g. "DocumentAnalysis", "DocumentTextDetection").
    pub job_type: String,
    /// The blocks stored with this job.
    #[serde(default)]
    pub blocks: Vec<Block>,
}

impl From<&TextractJob> for TextractJobView {
    fn from(job: &TextractJob) -> Self {
        TextractJobView {
            job_type: job.job_type.clone(),
            blocks: job.blocks.clone(),
        }
    }
}

impl From<TextractJobView> for TextractJob {
    fn from(view: TextractJobView) -> Self {
        TextractJob {
            job_type: view.job_type,
            blocks: view.blocks,
        }
    }
}

/// Serializable view of the entire Textract state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextractStateView {
    /// Async jobs keyed by job ID.
    #[serde(default)]
    pub jobs: HashMap<String, TextractJobView>,
}

impl From<&TextractState> for TextractStateView {
    fn from(state: &TextractState) -> Self {
        TextractStateView {
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| (k.clone(), TextractJobView::from(v)))
                .collect(),
        }
    }
}

impl From<TextractStateView> for TextractState {
    fn from(view: TextractStateView) -> Self {
        TextractState {
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, v)| (k, TextractJob::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for TextractService {
    type StateView = TextractStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TextractStateView::from(&*guard)
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
            *guard = TextractState::from(view);
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
            guard.jobs.extend(
                view.jobs
                    .into_iter()
                    .map(|(k, v)| (k, TextractJob::from(v))),
            );
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
