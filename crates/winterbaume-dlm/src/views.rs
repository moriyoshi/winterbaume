use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DlmService;
use crate::model;
use crate::state::DlmState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DlmStateView {
    #[serde(default)]
    pub policies: HashMap<String, model::LifecyclePolicy>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

impl From<&DlmState> for DlmStateView {
    fn from(state: &DlmState) -> Self {
        Self {
            policies: state.policies.clone(),
            tags: state.tags.clone(),
        }
    }
}

impl From<DlmStateView> for DlmState {
    fn from(view: DlmStateView) -> Self {
        Self {
            policies: view.policies,
            tags: view.tags,
        }
    }
}

impl StatefulService for DlmService {
    type StateView = DlmStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DlmStateView::from(&*guard)
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
            *guard = DlmState::from(view);
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
            for (k, v) in view.policies {
                guard.policies.insert(k, v);
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
