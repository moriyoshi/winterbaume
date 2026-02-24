use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::FreeTierService;
use crate::state::{ActivityRecord, FreeTierState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeTierStateView {
    pub plan_type: String,
    pub plan_status: String,
    #[serde(default)]
    pub plan_expiration_date: Option<String>,
    pub remaining_credits_amount: f64,
    pub remaining_credits_unit: String,
    #[serde(default)]
    pub activities: HashMap<String, ActivityRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityRecordView {
    pub activity_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub estimated_time_to_complete_in_minutes: Option<i32>,
    #[serde(default)]
    pub instructions_url: Option<String>,
    pub reward_amount: f64,
    pub reward_unit: String,
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
    ActivityRecordView,
    ActivityRecord {
        activity_id,
        title,
        description,
        status,
        started_at,
        completed_at,
        expires_at,
        estimated_time_to_complete_in_minutes,
        instructions_url,
        reward_amount,
        reward_unit,
    }
);

impl From<&FreeTierState> for FreeTierStateView {
    fn from(state: &FreeTierState) -> Self {
        Self {
            plan_type: state.plan_type.clone(),
            plan_status: state.plan_status.clone(),
            plan_expiration_date: state.plan_expiration_date.clone(),
            remaining_credits_amount: state.remaining_credits_amount,
            remaining_credits_unit: state.remaining_credits_unit.clone(),
            activities: state
                .activities
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<FreeTierStateView> for FreeTierState {
    fn from(view: FreeTierStateView) -> Self {
        Self {
            plan_type: view.plan_type,
            plan_status: view.plan_status,
            plan_expiration_date: view.plan_expiration_date,
            remaining_credits_amount: view.remaining_credits_amount,
            remaining_credits_unit: view.remaining_credits_unit,
            activities: view
                .activities
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for FreeTierService {
    type StateView = FreeTierStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        FreeTierStateView::from(&*guard)
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
            *guard = FreeTierState::from(view);
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
        let merged = FreeTierState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.plan_type = merged.plan_type;
            guard.plan_status = merged.plan_status;
            if let Some(d) = merged.plan_expiration_date {
                guard.plan_expiration_date = Some(d);
            }
            guard.remaining_credits_amount = merged.remaining_credits_amount;
            guard.remaining_credits_unit = merged.remaining_credits_unit;
            for (k, v) in merged.activities {
                guard.activities.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
