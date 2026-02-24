use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CostOptimizationHubService;
use crate::state::CostOptimizationHubState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostOptimizationHubStateView {
    #[serde(default)]
    pub enrollment_statuses: HashMap<String, Value>,
    #[serde(default)]
    pub preferences: Value,
    #[serde(default)]
    pub recommendations: HashMap<String, Value>,
}

impl From<&CostOptimizationHubState> for CostOptimizationHubStateView {
    fn from(s: &CostOptimizationHubState) -> Self {
        Self {
            enrollment_statuses: s.enrollment_statuses.clone(),
            preferences: s.preferences.clone(),
            recommendations: s.recommendations.clone(),
        }
    }
}

impl From<CostOptimizationHubStateView> for CostOptimizationHubState {
    fn from(v: CostOptimizationHubStateView) -> Self {
        Self {
            enrollment_statuses: v.enrollment_statuses,
            preferences: v.preferences,
            recommendations: v.recommendations,
        }
    }
}

impl StatefulService for CostOptimizationHubService {
    type StateView = CostOptimizationHubStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CostOptimizationHubStateView::from(&*guard)
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
            *guard = CostOptimizationHubState::from(view);
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
            for (k, v) in view.enrollment_statuses {
                guard.enrollment_statuses.insert(k, v);
            }
            for (k, v) in view.recommendations {
                guard.recommendations.insert(k, v);
            }
            if !view.preferences.is_null() {
                guard.preferences = view.preferences;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
