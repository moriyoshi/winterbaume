use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PersonalizeEventsService;
use crate::state::PersonalizeEventsState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalizeEventsStateView {
    #[serde(default)]
    pub events: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub action_interactions: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub actions: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub items: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub users: HashMap<String, Vec<Value>>,
}

impl From<&PersonalizeEventsState> for PersonalizeEventsStateView {
    fn from(state: &PersonalizeEventsState) -> Self {
        Self {
            events: state.events.clone(),
            action_interactions: state.action_interactions.clone(),
            actions: state.actions.clone(),
            items: state.items.clone(),
            users: state.users.clone(),
        }
    }
}

impl From<PersonalizeEventsStateView> for PersonalizeEventsState {
    fn from(view: PersonalizeEventsStateView) -> Self {
        Self {
            events: view.events,
            action_interactions: view.action_interactions,
            actions: view.actions,
            items: view.items,
            users: view.users,
        }
    }
}

impl StatefulService for PersonalizeEventsService {
    type StateView = PersonalizeEventsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PersonalizeEventsStateView::from(&*guard)
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
            *guard = PersonalizeEventsState::from(view);
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
            for (k, v) in view.events {
                guard.events.entry(k).or_default().extend(v);
            }
            for (k, v) in view.action_interactions {
                guard.action_interactions.entry(k).or_default().extend(v);
            }
            for (k, v) in view.actions {
                guard.actions.entry(k).or_default().extend(v);
            }
            for (k, v) in view.items {
                guard.items.entry(k).or_default().extend(v);
            }
            for (k, v) in view.users {
                guard.users.entry(k).or_default().extend(v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
