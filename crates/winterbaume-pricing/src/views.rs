use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PricingService;
use crate::state::PricingState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PricingStateView {
    #[serde(default)]
    pub call_counts: HashMap<String, u64>,
}

impl From<&PricingState> for PricingStateView {
    fn from(state: &PricingState) -> Self {
        Self {
            call_counts: state.call_counts.clone(),
        }
    }
}

impl From<PricingStateView> for PricingState {
    fn from(view: PricingStateView) -> Self {
        Self {
            call_counts: view.call_counts,
        }
    }
}

impl StatefulService for PricingService {
    type StateView = PricingStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PricingStateView::from(&*guard)
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
            *guard = PricingState::from(view);
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
            for (k, v) in view.call_counts {
                *guard.call_counts.entry(k).or_default() += v;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
