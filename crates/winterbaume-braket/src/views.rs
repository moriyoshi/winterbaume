use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BraketService;
use crate::state::BraketState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BraketStateView {
    #[serde(default)]
    pub jobs: HashMap<String, Value>,
    #[serde(default)]
    pub tasks: HashMap<String, Value>,
    #[serde(default)]
    pub spending_limits: HashMap<String, Value>,
    #[serde(default)]
    pub devices: HashMap<String, Value>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

impl From<&BraketState> for BraketStateView {
    fn from(s: &BraketState) -> Self {
        Self {
            jobs: s.jobs.clone(),
            tasks: s.tasks.clone(),
            spending_limits: s.spending_limits.clone(),
            devices: s.devices.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<BraketStateView> for BraketState {
    fn from(v: BraketStateView) -> Self {
        let mut state = BraketState::default();
        state.jobs = v.jobs;
        state.tasks = v.tasks;
        state.spending_limits = v.spending_limits;
        state.devices = v.devices;
        state.tags = v.tags;
        state.ensure_seeded();
        state
    }
}

impl StatefulService for BraketService {
    type StateView = BraketStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BraketStateView::from(&*guard)
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
            *guard = BraketState::from(view);
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
            for (k, v) in view.jobs {
                guard.jobs.insert(k, v);
            }
            for (k, v) in view.tasks {
                guard.tasks.insert(k, v);
            }
            for (k, v) in view.spending_limits {
                guard.spending_limits.insert(k, v);
            }
            for (k, v) in view.devices {
                guard.devices.insert(k, v);
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
