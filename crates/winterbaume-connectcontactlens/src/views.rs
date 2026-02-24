use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ConnectContactLensService;
use crate::state::ConnectContactLensState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectContactLensStateView {
    #[serde(default)]
    pub segments: HashMap<String, Vec<Value>>,
}

impl From<&ConnectContactLensState> for ConnectContactLensStateView {
    fn from(s: &ConnectContactLensState) -> Self {
        Self {
            segments: s
                .segments
                .iter()
                .map(|((instance, contact), v)| (format!("{instance}|{contact}"), v.clone()))
                .collect(),
        }
    }
}

impl From<ConnectContactLensStateView> for ConnectContactLensState {
    fn from(v: ConnectContactLensStateView) -> Self {
        let segments = v
            .segments
            .into_iter()
            .filter_map(|(k, val)| {
                let (instance, contact) = k.split_once('|')?;
                Some(((instance.to_string(), contact.to_string()), val))
            })
            .collect();
        Self { segments }
    }
}

impl StatefulService for ConnectContactLensService {
    type StateView = ConnectContactLensStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ConnectContactLensStateView::from(&*guard)
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
            *guard = ConnectContactLensState::from(view);
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
        let new_state = ConnectContactLensState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in new_state.segments {
                guard.segments.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
