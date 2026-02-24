use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppConfigDataService;
use crate::state::AppConfigDataState;
use crate::types::ConfigurationSession;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigDataStateView {
    #[serde(default)]
    pub sessions: HashMap<String, ConfigurationSessionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSessionView {
    pub token: String,
    pub application_id: String,
    pub environment_id: String,
    pub configuration_profile_id: String,
    #[serde(default)]
    pub required_minimum_poll_interval_in_seconds: Option<i32>,
}

impl From<&ConfigurationSession> for ConfigurationSessionView {
    fn from(s: &ConfigurationSession) -> Self {
        Self {
            token: s.token.clone(),
            application_id: s.application_id.clone(),
            environment_id: s.environment_id.clone(),
            configuration_profile_id: s.configuration_profile_id.clone(),
            required_minimum_poll_interval_in_seconds: s.required_minimum_poll_interval_in_seconds,
        }
    }
}

impl From<ConfigurationSessionView> for ConfigurationSession {
    fn from(v: ConfigurationSessionView) -> Self {
        Self {
            token: v.token,
            application_id: v.application_id,
            environment_id: v.environment_id,
            configuration_profile_id: v.configuration_profile_id,
            required_minimum_poll_interval_in_seconds: v.required_minimum_poll_interval_in_seconds,
        }
    }
}

impl From<&AppConfigDataState> for AppConfigDataStateView {
    fn from(state: &AppConfigDataState) -> Self {
        Self {
            sessions: state
                .sessions
                .iter()
                .map(|(k, v)| (k.clone(), ConfigurationSessionView::from(v)))
                .collect(),
        }
    }
}

impl From<AppConfigDataStateView> for AppConfigDataState {
    fn from(view: AppConfigDataStateView) -> Self {
        Self {
            sessions: view
                .sessions
                .into_iter()
                .map(|(k, v)| (k, ConfigurationSession::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for AppConfigDataService {
    type StateView = AppConfigDataStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppConfigDataStateView::from(&*guard)
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
            *guard = AppConfigDataState::from(view);
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
            for (k, v) in view.sessions {
                guard.sessions.insert(k, ConfigurationSession::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
