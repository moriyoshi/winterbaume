//! Serde-compatible view types for Budgets state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BudgetsService;
use crate::state::BudgetsState;

/// Serializable view of the entire Budgets state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BudgetsStateView {
    /// Budgets keyed by budget name.
    #[serde(default)]
    pub budgets: HashMap<String, BudgetView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetView {
    pub budget_name: String,
    pub budget_type: String,
    pub budget_limit_amount: String,
    pub budget_limit_unit: String,
    pub time_unit: String,
    #[serde(default)]
    pub notifications: Vec<NotificationView>,
    #[serde(default)]
    pub auto_adjust_data: Option<serde_json::Value>,
    #[serde(default)]
    pub cost_types: Option<serde_json::Value>,
    #[serde(default)]
    pub planned_limit: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationView {
    pub notification_type: String,
    pub comparison_operator: String,
    pub threshold: f64,
    pub threshold_type: String,
}

// --- From internal types to view types ---

impl From<&BudgetsState> for BudgetsStateView {
    fn from(state: &BudgetsState) -> Self {
        BudgetsStateView {
            budgets: state
                .budgets
                .iter()
                .map(|(k, b)| {
                    (
                        k.clone(),
                        BudgetView {
                            budget_name: b.budget_name.clone(),
                            budget_type: b.budget_type.clone(),
                            budget_limit_amount: b.budget_limit_amount.clone(),
                            budget_limit_unit: b.budget_limit_unit.clone(),
                            time_unit: b.time_unit.clone(),
                            notifications: b
                                .notifications
                                .iter()
                                .map(|n| NotificationView {
                                    notification_type: n.notification_type.clone(),
                                    comparison_operator: n.comparison_operator.clone(),
                                    threshold: n.threshold,
                                    threshold_type: n.threshold_type.clone(),
                                })
                                .collect(),
                            auto_adjust_data: None,
                            cost_types: None,
                            planned_limit: None,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<BudgetsStateView> for BudgetsState {
    fn from(view: BudgetsStateView) -> Self {
        BudgetsState {
            budgets: view
                .budgets
                .into_iter()
                .map(|(k, b)| {
                    (
                        k,
                        crate::types::Budget {
                            budget_name: b.budget_name,
                            budget_type: b.budget_type,
                            budget_limit_amount: b.budget_limit_amount,
                            budget_limit_unit: b.budget_limit_unit,
                            time_unit: b.time_unit,
                            notifications: b
                                .notifications
                                .into_iter()
                                .map(|n| crate::types::Notification {
                                    notification_type: n.notification_type,
                                    comparison_operator: n.comparison_operator,
                                    threshold: n.threshold,
                                    threshold_type: n.threshold_type,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for BudgetsService {
    type StateView = BudgetsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BudgetsStateView::from(&*guard)
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
            *guard = BudgetsState::from(view);
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
            let merged = BudgetsState::from(view);
            for (k, v) in merged.budgets {
                guard.budgets.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
