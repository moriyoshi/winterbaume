use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TaxSettingsService;
use crate::state::TaxSettingsState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxSettingsStateView {
    #[serde(default)]
    pub registrations: HashMap<String, Value>,
    /// Supplemental keyed by "{account_id}/{authority_id}".
    #[serde(default)]
    pub supplemental: HashMap<String, Value>,
    #[serde(default)]
    pub exemptions: HashMap<String, Value>,
    #[serde(default)]
    pub inheritance_state: String,
}

impl From<&TaxSettingsState> for TaxSettingsStateView {
    fn from(state: &TaxSettingsState) -> Self {
        Self {
            registrations: state.registrations.clone(),
            supplemental: state
                .supplemental
                .iter()
                .map(|((a, h), v)| (format!("{a}/{h}"), v.clone()))
                .collect(),
            exemptions: state.exemptions.clone(),
            inheritance_state: state.inheritance_state.clone(),
        }
    }
}

impl From<TaxSettingsStateView> for TaxSettingsState {
    fn from(view: TaxSettingsStateView) -> Self {
        let supplemental = view
            .supplemental
            .into_iter()
            .filter_map(|(k, v)| {
                let mut parts = k.splitn(2, '/');
                let account_id = parts.next()?.to_string();
                let authority_id = parts.next()?.to_string();
                Some(((account_id, authority_id), v))
            })
            .collect();
        Self {
            registrations: view.registrations,
            supplemental,
            exemptions: view.exemptions,
            inheritance_state: view.inheritance_state,
        }
    }
}

impl StatefulService for TaxSettingsService {
    type StateView = TaxSettingsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TaxSettingsStateView::from(&*guard)
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
            *guard = TaxSettingsState::from(view);
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
        let merged = TaxSettingsState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.registrations {
                guard.registrations.insert(k, v);
            }
            for (k, v) in merged.supplemental {
                guard.supplemental.insert(k, v);
            }
            for (k, v) in merged.exemptions {
                guard.exemptions.insert(k, v);
            }
            if !merged.inheritance_state.is_empty() {
                guard.inheritance_state = merged.inheritance_state;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
