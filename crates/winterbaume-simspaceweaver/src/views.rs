use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SimSpaceWeaverService;
use crate::state::{AppRecord, SimSpaceWeaverState, SimulationRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimSpaceWeaverStateView {
    #[serde(default)]
    pub simulations: HashMap<String, SimulationRecordView>,
    /// Apps keyed by "{simulation}/{domain}/{name}".
    #[serde(default)]
    pub apps: HashMap<String, AppRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimulationRecordView {
    pub name: String,
    pub arn: String,
    pub execution_id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub role_arn: String,
    pub status: String,
    pub target_status: String,
    pub creation_time: f64,
    #[serde(default)]
    pub maximum_duration: Option<String>,
    pub clock_status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRecordView {
    pub simulation: String,
    pub domain: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
    pub target_status: String,
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
    SimulationRecordView,
    SimulationRecord {
        name,
        arn,
        execution_id,
        description,
        role_arn,
        status,
        target_status,
        creation_time,
        maximum_duration,
        clock_status,
        tags,
    }
);

basic_from!(
    AppRecordView,
    AppRecord {
        simulation,
        domain,
        name,
        description,
        status,
        target_status,
    }
);

impl From<&SimSpaceWeaverState> for SimSpaceWeaverStateView {
    fn from(state: &SimSpaceWeaverState) -> Self {
        Self {
            simulations: state
                .simulations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            apps: state
                .apps
                .iter()
                .map(|((s, d, n), v)| (format!("{s}/{d}/{n}"), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<SimSpaceWeaverStateView> for SimSpaceWeaverState {
    fn from(view: SimSpaceWeaverStateView) -> Self {
        let apps = view
            .apps
            .into_values()
            .map(|v| {
                let a: AppRecord = v.into();
                ((a.simulation.clone(), a.domain.clone(), a.name.clone()), a)
            })
            .collect();
        Self {
            simulations: view
                .simulations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            apps,
            tags: view.tags,
        }
    }
}

impl StatefulService for SimSpaceWeaverService {
    type StateView = SimSpaceWeaverStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SimSpaceWeaverStateView::from(&*guard)
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
            *guard = SimSpaceWeaverState::from(view);
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
        let merged = SimSpaceWeaverState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.simulations {
                guard.simulations.insert(k, v);
            }
            for (k, v) in merged.apps {
                guard.apps.insert(k, v);
            }
            for (k, v) in merged.tags {
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
