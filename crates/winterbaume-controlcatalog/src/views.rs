use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ControlCatalogService;
use crate::state::ControlCatalogState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlCatalogStateView {
    #[serde(default)]
    pub controls: HashMap<String, Value>,
    #[serde(default)]
    pub domains: HashMap<String, Value>,
    #[serde(default)]
    pub objectives: HashMap<String, Value>,
    #[serde(default)]
    pub common_controls: HashMap<String, Value>,
}

impl From<&ControlCatalogState> for ControlCatalogStateView {
    fn from(state: &ControlCatalogState) -> Self {
        Self {
            controls: state.controls.clone(),
            domains: state.domains.clone(),
            objectives: state.objectives.clone(),
            common_controls: state.common_controls.clone(),
        }
    }
}

impl From<ControlCatalogStateView> for ControlCatalogState {
    fn from(view: ControlCatalogStateView) -> Self {
        Self {
            controls: view.controls,
            domains: view.domains,
            objectives: view.objectives,
            common_controls: view.common_controls,
        }
    }
}

impl StatefulService for ControlCatalogService {
    type StateView = ControlCatalogStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ControlCatalogStateView::from(&*guard)
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
            *guard = ControlCatalogState::from(view);
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
        let merged = ControlCatalogState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.controls {
                guard.controls.insert(k, v);
            }
            for (k, v) in merged.domains {
                guard.domains.insert(k, v);
            }
            for (k, v) in merged.objectives {
                guard.objectives.insert(k, v);
            }
            for (k, v) in merged.common_controls {
                guard.common_controls.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
