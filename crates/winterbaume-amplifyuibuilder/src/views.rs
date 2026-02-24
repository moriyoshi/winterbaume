use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AmplifyUiBuilderService;
use crate::model;
use crate::state::AmplifyUiBuilderState;

/// Composite key serialized as `"{app_id}::{environment_name}::{id}"`.
fn join3(a: &str, e: &str, i: &str) -> String {
    format!("{a}::{e}::{i}")
}

fn split3(s: &str) -> (String, String, String) {
    let mut parts = s.splitn(3, "::");
    let a = parts.next().unwrap_or("").to_string();
    let b = parts.next().unwrap_or("").to_string();
    let c = parts.next().unwrap_or("").to_string();
    (a, b, c)
}

fn join2(a: &str, b: &str) -> String {
    format!("{a}::{b}")
}

fn split2(s: &str) -> (String, String) {
    let mut parts = s.splitn(2, "::");
    let a = parts.next().unwrap_or("").to_string();
    let b = parts.next().unwrap_or("").to_string();
    (a, b)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmplifyUiBuilderStateView {
    #[serde(default)]
    pub components: HashMap<String, model::Component>,
    #[serde(default)]
    pub forms: HashMap<String, model::Form>,
    #[serde(default)]
    pub themes: HashMap<String, model::Theme>,
    #[serde(default)]
    pub codegen_jobs: HashMap<String, model::CodegenJob>,
    #[serde(default)]
    pub metadata: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

impl From<&AmplifyUiBuilderState> for AmplifyUiBuilderStateView {
    fn from(state: &AmplifyUiBuilderState) -> Self {
        Self {
            components: state
                .components
                .iter()
                .map(|((a, e, i), v)| (join3(a, e, i), v.clone()))
                .collect(),
            forms: state
                .forms
                .iter()
                .map(|((a, e, i), v)| (join3(a, e, i), v.clone()))
                .collect(),
            themes: state
                .themes
                .iter()
                .map(|((a, e, i), v)| (join3(a, e, i), v.clone()))
                .collect(),
            codegen_jobs: state
                .codegen_jobs
                .iter()
                .map(|((a, e, i), v)| (join3(a, e, i), v.clone()))
                .collect(),
            metadata: state
                .metadata
                .iter()
                .map(|((a, e), v)| (join2(a, e), v.clone()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<AmplifyUiBuilderStateView> for AmplifyUiBuilderState {
    fn from(view: AmplifyUiBuilderStateView) -> Self {
        Self {
            components: view
                .components
                .into_iter()
                .map(|(k, v)| (split3(&k), v))
                .collect(),
            forms: view
                .forms
                .into_iter()
                .map(|(k, v)| (split3(&k), v))
                .collect(),
            themes: view
                .themes
                .into_iter()
                .map(|(k, v)| (split3(&k), v))
                .collect(),
            codegen_jobs: view
                .codegen_jobs
                .into_iter()
                .map(|(k, v)| (split3(&k), v))
                .collect(),
            metadata: view
                .metadata
                .into_iter()
                .map(|(k, v)| (split2(&k), v))
                .collect(),
            tags: view.tags,
        }
    }
}

impl StatefulService for AmplifyUiBuilderService {
    type StateView = AmplifyUiBuilderStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AmplifyUiBuilderStateView::from(&*guard)
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
            *guard = AmplifyUiBuilderState::from(view);
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
            for (k, v) in view.components {
                guard.components.insert(split3(&k), v);
            }
            for (k, v) in view.forms {
                guard.forms.insert(split3(&k), v);
            }
            for (k, v) in view.themes {
                guard.themes.insert(split3(&k), v);
            }
            for (k, v) in view.codegen_jobs {
                guard.codegen_jobs.insert(split3(&k), v);
            }
            for (k, v) in view.metadata {
                guard.metadata.insert(split2(&k), v);
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
