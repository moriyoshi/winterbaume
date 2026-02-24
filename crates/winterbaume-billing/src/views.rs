use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BillingService;
use crate::state::BillingState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingStateView {
    #[serde(default)]
    pub billing_views: HashMap<String, Value>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub policies: HashMap<String, String>,
}

impl From<&BillingState> for BillingStateView {
    fn from(s: &BillingState) -> Self {
        Self {
            billing_views: s.billing_views.clone(),
            tags: s.tags.clone(),
            policies: s.policies.clone(),
        }
    }
}

impl From<BillingStateView> for BillingState {
    fn from(v: BillingStateView) -> Self {
        Self {
            billing_views: v.billing_views,
            tags: v.tags,
            policies: v.policies,
        }
    }
}

impl StatefulService for BillingService {
    type StateView = BillingStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BillingStateView::from(&*guard)
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
            *guard = BillingState::from(view);
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
            for (k, v) in view.billing_views {
                guard.billing_views.insert(k, v);
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
            for (k, v) in view.policies {
                guard.policies.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
