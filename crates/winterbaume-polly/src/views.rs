//! Serde-compatible view types for Polly state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PollyService;
use crate::state::PollyState;

/// Serializable view of a lexicon.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LexiconView {
    pub name: String,
    pub content: String,
    pub language_code: String,
    pub lexemes_count: i32,
    pub size: i32,
    pub last_modified: String,
}

/// Serializable view of a speech synthesis task.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpeechSynthesisTaskView {
    pub task_id: String,
    pub task_status: String,
    pub output_uri: String,
    pub voice_id: String,
    pub text: String,
    pub creation_time: String,
    pub output_format: String,
    pub engine: String,
}

/// Serializable view of the entire Polly state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PollyStateView {
    #[serde(default)]
    pub lexicons: HashMap<String, LexiconView>,
    #[serde(default)]
    pub synthesis_tasks: HashMap<String, SpeechSynthesisTaskView>,
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

impl From<&crate::types::Lexicon> for LexiconView {
    fn from(l: &crate::types::Lexicon) -> Self {
        LexiconView {
            name: l.name.clone(),
            content: l.content.clone(),
            language_code: l.language_code.clone(),
            lexemes_count: l.lexemes_count,
            size: l.size,
            last_modified: dt_to_string(&l.last_modified),
        }
    }
}

impl From<&crate::types::SpeechSynthesisTask> for SpeechSynthesisTaskView {
    fn from(t: &crate::types::SpeechSynthesisTask) -> Self {
        SpeechSynthesisTaskView {
            task_id: t.task_id.clone(),
            task_status: t.task_status.clone(),
            output_uri: t.output_uri.clone(),
            voice_id: t.voice_id.clone(),
            text: t.text.clone(),
            creation_time: dt_to_string(&t.creation_time),
            output_format: t.output_format.clone(),
            engine: t.engine.clone(),
        }
    }
}

impl From<&PollyState> for PollyStateView {
    fn from(state: &PollyState) -> Self {
        PollyStateView {
            lexicons: state
                .lexicons
                .iter()
                .map(|(k, v)| (k.clone(), LexiconView::from(v)))
                .collect(),
            synthesis_tasks: state
                .synthesis_tasks
                .iter()
                .map(|(k, v)| (k.clone(), SpeechSynthesisTaskView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<LexiconView> for crate::types::Lexicon {
    fn from(v: LexiconView) -> Self {
        crate::types::Lexicon {
            name: v.name,
            content: v.content,
            language_code: v.language_code,
            lexemes_count: v.lexemes_count,
            size: v.size,
            last_modified: parse_dt(&v.last_modified),
        }
    }
}

impl From<SpeechSynthesisTaskView> for crate::types::SpeechSynthesisTask {
    fn from(v: SpeechSynthesisTaskView) -> Self {
        crate::types::SpeechSynthesisTask {
            task_id: v.task_id,
            task_status: v.task_status,
            output_uri: v.output_uri,
            voice_id: v.voice_id,
            text: v.text,
            creation_time: parse_dt(&v.creation_time),
            output_format: v.output_format,
            engine: v.engine,
        }
    }
}

impl From<PollyStateView> for PollyState {
    fn from(view: PollyStateView) -> Self {
        PollyState {
            lexicons: view
                .lexicons
                .into_iter()
                .map(|(k, v)| (k, crate::types::Lexicon::from(v)))
                .collect(),
            synthesis_tasks: view
                .synthesis_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::SpeechSynthesisTask::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for PollyService {
    type StateView = PollyStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PollyStateView::from(&*guard)
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
            *guard = PollyState::from(view);
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
            for (name, lexicon_view) in view.lexicons {
                guard
                    .lexicons
                    .insert(name, crate::types::Lexicon::from(lexicon_view));
            }
            for (id, task_view) in view.synthesis_tasks {
                guard
                    .synthesis_tasks
                    .insert(id, crate::types::SpeechSynthesisTask::from(task_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
