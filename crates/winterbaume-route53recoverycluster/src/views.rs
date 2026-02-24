use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RecoveryClusterService;
use crate::state::{RecoveryClusterState, RoutingControlRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoveryClusterStateView {
    #[serde(default)]
    pub routing_controls: HashMap<String, RoutingControlRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingControlRecordView {
    pub arn: String,
    pub name: String,
    pub control_panel_arn: String,
    pub control_panel_name: String,
    pub owner: String,
    pub state: String,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    RoutingControlRecordView,
    RoutingControlRecord {
        arn,
        name,
        control_panel_arn,
        control_panel_name,
        owner,
        state,
    }
);

impl From<&RecoveryClusterState> for RecoveryClusterStateView {
    fn from(state: &RecoveryClusterState) -> Self {
        Self {
            routing_controls: state
                .routing_controls
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<RecoveryClusterStateView> for RecoveryClusterState {
    fn from(view: RecoveryClusterStateView) -> Self {
        Self {
            routing_controls: view
                .routing_controls
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for RecoveryClusterService {
    type StateView = RecoveryClusterStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RecoveryClusterStateView::from(&*guard)
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
            *guard = RecoveryClusterState::from(view);
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
        let merged = RecoveryClusterState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.routing_controls {
                guard.routing_controls.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
