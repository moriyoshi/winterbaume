use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PinpointSmsVoiceService;
use crate::state::{ConfigurationSet, PinpointSmsVoiceState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PinpointSmsVoiceStateView {
    #[serde(default)]
    pub configuration_sets: HashMap<String, ConfigurationSetView>,
    #[serde(default)]
    pub messages: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigurationSetView {
    pub name: String,
    #[serde(default)]
    pub event_destinations: HashMap<String, Value>,
}

impl From<&ConfigurationSet> for ConfigurationSetView {
    fn from(cs: &ConfigurationSet) -> Self {
        Self {
            name: cs.name.clone(),
            event_destinations: cs.event_destinations.clone(),
        }
    }
}

impl From<ConfigurationSetView> for ConfigurationSet {
    fn from(v: ConfigurationSetView) -> Self {
        Self {
            name: v.name,
            event_destinations: v.event_destinations,
        }
    }
}

impl From<&PinpointSmsVoiceState> for PinpointSmsVoiceStateView {
    fn from(state: &PinpointSmsVoiceState) -> Self {
        Self {
            configuration_sets: state
                .configuration_sets
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            messages: state.messages.clone(),
        }
    }
}

impl From<PinpointSmsVoiceStateView> for PinpointSmsVoiceState {
    fn from(view: PinpointSmsVoiceStateView) -> Self {
        Self {
            configuration_sets: view
                .configuration_sets
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            messages: view.messages,
        }
    }
}

impl StatefulService for PinpointSmsVoiceService {
    type StateView = PinpointSmsVoiceStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PinpointSmsVoiceStateView::from(&*guard)
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
            *guard = PinpointSmsVoiceState::from(view);
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
            for (k, v) in view.configuration_sets {
                guard.configuration_sets.insert(k, v.into());
            }
            for (k, v) in view.messages {
                guard.messages.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
