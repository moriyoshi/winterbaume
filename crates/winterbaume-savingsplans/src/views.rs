use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SavingsPlansService;
use crate::state::{PlanRecord, SavingsPlansState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavingsPlansStateView {
    #[serde(default)]
    pub plans: HashMap<String, PlanRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanRecordView {
    pub savings_plan_id: String,
    pub savings_plan_arn: String,
    pub offering_id: String,
    pub commitment: String,
    #[serde(default)]
    pub upfront_payment_amount: Option<String>,
    #[serde(default)]
    pub recurring_payment_amount: Option<String>,
    pub start: String,
    pub end: String,
    pub state: String,
    pub region: String,
    pub currency: String,
    pub savings_plan_type: String,
    pub payment_option: String,
    #[serde(default)]
    pub product_types: Vec<String>,
    pub term_duration_in_seconds: i64,
    #[serde(default)]
    pub returnable_until: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
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
    PlanRecordView,
    PlanRecord {
        savings_plan_id,
        savings_plan_arn,
        offering_id,
        commitment,
        upfront_payment_amount,
        recurring_payment_amount,
        start,
        end,
        state,
        region,
        currency,
        savings_plan_type,
        payment_option,
        product_types,
        term_duration_in_seconds,
        returnable_until,
        tags,
    }
);

impl From<&SavingsPlansState> for SavingsPlansStateView {
    fn from(state: &SavingsPlansState) -> Self {
        Self {
            plans: state
                .plans
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<SavingsPlansStateView> for SavingsPlansState {
    fn from(view: SavingsPlansStateView) -> Self {
        Self {
            plans: view.plans.into_iter().map(|(k, v)| (k, v.into())).collect(),
            tags: view.tags,
        }
    }
}

impl StatefulService for SavingsPlansService {
    type StateView = SavingsPlansStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SavingsPlansStateView::from(&*guard)
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
            *guard = SavingsPlansState::from(view);
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
            for (k, v) in view.plans {
                guard.plans.insert(k, v.into());
            }
            for (k, v) in view.tags {
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
