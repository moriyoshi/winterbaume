use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TrustedAdvisorService;
use crate::state::TrustedAdvisorState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrustedAdvisorStateView {
    #[serde(default)]
    pub recommendations: HashMap<String, Value>,
    #[serde(default)]
    pub organization_recommendations: HashMap<String, Value>,
    #[serde(default)]
    pub resource_exclusions: HashMap<String, bool>,
}

impl From<&TrustedAdvisorState> for TrustedAdvisorStateView {
    fn from(state: &TrustedAdvisorState) -> Self {
        Self {
            recommendations: state.recommendations.clone(),
            organization_recommendations: state.organization_recommendations.clone(),
            resource_exclusions: state.resource_exclusions.clone(),
        }
    }
}

impl From<TrustedAdvisorStateView> for TrustedAdvisorState {
    fn from(view: TrustedAdvisorStateView) -> Self {
        Self {
            recommendations: view.recommendations,
            organization_recommendations: view.organization_recommendations,
            resource_exclusions: view.resource_exclusions,
        }
    }
}

impl StatefulService for TrustedAdvisorService {
    type StateView = TrustedAdvisorStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TrustedAdvisorStateView::from(&*guard)
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
            *guard = TrustedAdvisorState::from(view);
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
            for (k, v) in view.recommendations {
                guard.recommendations.insert(k, v);
            }
            for (k, v) in view.organization_recommendations {
                guard.organization_recommendations.insert(k, v);
            }
            for (k, v) in view.resource_exclusions {
                guard.resource_exclusions.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
