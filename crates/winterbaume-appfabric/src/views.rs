use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppFabricService;
use crate::state::AppFabricState;
use crate::types::AppBundle;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppFabricStateView {
    #[serde(default)]
    pub app_bundles: HashMap<String, AppBundleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBundleView {
    pub arn: String,
    #[serde(default)]
    pub customer_managed_key_arn: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&AppBundle> for AppBundleView {
    fn from(b: &AppBundle) -> Self {
        Self {
            arn: b.arn.clone(),
            customer_managed_key_arn: b.customer_managed_key_arn.clone(),
            tags: b.tags.clone(),
        }
    }
}

impl From<AppBundleView> for AppBundle {
    fn from(v: AppBundleView) -> Self {
        Self {
            arn: v.arn,
            customer_managed_key_arn: v.customer_managed_key_arn,
            tags: v.tags,
        }
    }
}

impl From<&AppFabricState> for AppFabricStateView {
    fn from(state: &AppFabricState) -> Self {
        Self {
            app_bundles: state
                .app_bundles
                .iter()
                .map(|(k, v)| (k.clone(), AppBundleView::from(v)))
                .collect(),
        }
    }
}

impl From<AppFabricStateView> for AppFabricState {
    fn from(view: AppFabricStateView) -> Self {
        Self {
            app_bundles: view
                .app_bundles
                .into_iter()
                .map(|(k, v)| (k, AppBundle::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for AppFabricService {
    type StateView = AppFabricStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppFabricStateView::from(&*guard)
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
            *guard = AppFabricState::from(view);
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
            for (k, v) in view.app_bundles {
                guard.app_bundles.insert(k, AppBundle::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
