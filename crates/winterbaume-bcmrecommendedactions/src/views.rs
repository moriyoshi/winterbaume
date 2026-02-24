use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BcmRecommendedActionsService;
use crate::state::{BcmRecommendedActionsState, RecommendedAction};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BcmRecommendedActionsStateView {
    #[serde(default)]
    pub recommended_actions: Vec<RecommendedActionView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedActionView {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub action_type: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub feature: Option<String>,
    #[serde(default)]
    pub context: HashMap<String, String>,
    #[serde(default)]
    pub next_steps: Vec<String>,
    #[serde(default)]
    pub last_updated_timestamp: Option<String>,
}

impl From<&RecommendedAction> for RecommendedActionView {
    fn from(r: &RecommendedAction) -> Self {
        Self {
            id: r.id.clone(),
            action_type: r.action_type.clone(),
            account_id: r.account_id.clone(),
            severity: r.severity.clone(),
            feature: r.feature.clone(),
            context: r.context.clone(),
            next_steps: r.next_steps.clone(),
            last_updated_timestamp: r.last_updated_timestamp.clone(),
        }
    }
}

impl From<RecommendedActionView> for RecommendedAction {
    fn from(v: RecommendedActionView) -> Self {
        Self {
            id: v.id,
            action_type: v.action_type,
            account_id: v.account_id,
            severity: v.severity,
            feature: v.feature,
            context: v.context,
            next_steps: v.next_steps,
            last_updated_timestamp: v.last_updated_timestamp,
        }
    }
}

impl From<&BcmRecommendedActionsState> for BcmRecommendedActionsStateView {
    fn from(state: &BcmRecommendedActionsState) -> Self {
        Self {
            recommended_actions: state.recommended_actions.iter().map(Into::into).collect(),
        }
    }
}

impl From<BcmRecommendedActionsStateView> for BcmRecommendedActionsState {
    fn from(view: BcmRecommendedActionsStateView) -> Self {
        Self {
            recommended_actions: view
                .recommended_actions
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl StatefulService for BcmRecommendedActionsService {
    type StateView = BcmRecommendedActionsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BcmRecommendedActionsStateView::from(&*guard)
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
            *guard = BcmRecommendedActionsState::from(view);
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
            for v in view.recommended_actions {
                guard.recommended_actions.push(v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
