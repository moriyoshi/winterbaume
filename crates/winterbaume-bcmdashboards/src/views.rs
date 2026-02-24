use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BcmDashboardsService;
use crate::state::BcmDashboardsState;
use crate::types::Dashboard;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BcmDashboardsStateView {
    #[serde(default)]
    pub dashboards: HashMap<String, DashboardView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardView {
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub r#type: String,
    #[serde(default)]
    pub widgets: Vec<Value>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&Dashboard> for DashboardView {
    fn from(d: &Dashboard) -> Self {
        Self {
            arn: d.arn.clone(),
            name: d.name.clone(),
            description: d.description.clone(),
            r#type: d.r#type.clone(),
            widgets: d.widgets.clone(),
            created_at: d.created_at,
            updated_at: d.updated_at,
            tags: d.tags.clone(),
        }
    }
}

impl From<DashboardView> for Dashboard {
    fn from(v: DashboardView) -> Self {
        Self {
            arn: v.arn,
            name: v.name,
            description: v.description,
            r#type: v.r#type,
            widgets: v.widgets,
            created_at: v.created_at,
            updated_at: v.updated_at,
            tags: v.tags,
        }
    }
}

impl From<&BcmDashboardsState> for BcmDashboardsStateView {
    fn from(state: &BcmDashboardsState) -> Self {
        Self {
            dashboards: state
                .dashboards
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<BcmDashboardsStateView> for BcmDashboardsState {
    fn from(view: BcmDashboardsStateView) -> Self {
        Self {
            dashboards: view
                .dashboards
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for BcmDashboardsService {
    type StateView = BcmDashboardsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BcmDashboardsStateView::from(&*guard)
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
            *guard = BcmDashboardsState::from(view);
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
            for (k, v) in view.dashboards {
                guard.dashboards.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
