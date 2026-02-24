use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AmplifyBackendService;
use crate::state::AmplifyBackendState;
use crate::types::Backend;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmplifyBackendStateView {
    /// Backends keyed by "{app_id}\x00{environment}" — composite-key map kept
    /// serde-friendly via a delimiter.
    #[serde(default)]
    pub backends: HashMap<String, BackendView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendView {
    pub app_id: String,
    pub app_name: String,
    pub backend_environment_name: String,
    #[serde(default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub amplify_meta_config: Option<String>,
    #[serde(default)]
    pub amplify_feature_flags: Option<String>,
}

impl From<&Backend> for BackendView {
    fn from(b: &Backend) -> Self {
        Self {
            app_id: b.app_id.clone(),
            app_name: b.app_name.clone(),
            backend_environment_name: b.backend_environment_name.clone(),
            resource_name: b.resource_name.clone(),
            amplify_meta_config: b.amplify_meta_config.clone(),
            amplify_feature_flags: b.amplify_feature_flags.clone(),
        }
    }
}

impl From<BackendView> for Backend {
    fn from(v: BackendView) -> Self {
        Self {
            app_id: v.app_id,
            app_name: v.app_name,
            backend_environment_name: v.backend_environment_name,
            resource_name: v.resource_name,
            amplify_meta_config: v.amplify_meta_config,
            amplify_feature_flags: v.amplify_feature_flags,
        }
    }
}

fn key_to_string(app_id: &str, env: &str) -> String {
    format!("{}\x00{}", app_id, env)
}

fn string_to_key(s: &str) -> Option<(String, String)> {
    s.split_once('\x00')
        .map(|(a, e)| (a.to_string(), e.to_string()))
}

impl From<&AmplifyBackendState> for AmplifyBackendStateView {
    fn from(state: &AmplifyBackendState) -> Self {
        Self {
            backends: state
                .backends
                .iter()
                .map(|((a, e), b)| (key_to_string(a, e), BackendView::from(b)))
                .collect(),
        }
    }
}

impl From<AmplifyBackendStateView> for AmplifyBackendState {
    fn from(view: AmplifyBackendStateView) -> Self {
        let mut backends = HashMap::new();
        for (k, v) in view.backends {
            if let Some(key) = string_to_key(&k) {
                backends.insert(key, Backend::from(v));
            }
        }
        Self { backends }
    }
}

impl StatefulService for AmplifyBackendService {
    type StateView = AmplifyBackendStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AmplifyBackendStateView::from(&*guard)
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
            *guard = AmplifyBackendState::from(view);
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
            for (k, v) in view.backends {
                if let Some(key) = string_to_key(&k) {
                    guard.backends.insert(key, Backend::from(v));
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
