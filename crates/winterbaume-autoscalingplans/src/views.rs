use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AutoScalingPlansService;
use crate::state::AutoScalingPlansState;
use crate::types::ScalingPlan;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoScalingPlansStateView {
    /// Plans keyed by `"<name>\x00<version>"` to keep the composite key
    /// serde-friendly.
    #[serde(default)]
    pub plans: HashMap<String, ScalingPlanView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPlanView {
    pub scaling_plan_name: String,
    pub scaling_plan_version: i64,
    #[serde(default)]
    pub application_source: Value,
    #[serde(default)]
    pub scaling_instructions: Vec<Value>,
    pub status_code: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub status_start_time: i64,
    pub creation_time: i64,
}

fn key_to_string(name: &str, version: i64) -> String {
    format!("{}\x00{}", name, version)
}

fn string_to_key(s: &str) -> Option<(String, i64)> {
    let (name, ver) = s.split_once('\x00')?;
    Some((name.to_string(), ver.parse().ok()?))
}

impl From<&ScalingPlan> for ScalingPlanView {
    fn from(p: &ScalingPlan) -> Self {
        Self {
            scaling_plan_name: p.scaling_plan_name.clone(),
            scaling_plan_version: p.scaling_plan_version,
            application_source: p.application_source.clone(),
            scaling_instructions: p.scaling_instructions.clone(),
            status_code: p.status_code.clone(),
            status_message: p.status_message.clone(),
            status_start_time: p.status_start_time,
            creation_time: p.creation_time,
        }
    }
}

impl From<ScalingPlanView> for ScalingPlan {
    fn from(v: ScalingPlanView) -> Self {
        Self {
            scaling_plan_name: v.scaling_plan_name,
            scaling_plan_version: v.scaling_plan_version,
            application_source: v.application_source,
            scaling_instructions: v.scaling_instructions,
            status_code: v.status_code,
            status_message: v.status_message,
            status_start_time: v.status_start_time,
            creation_time: v.creation_time,
        }
    }
}

impl From<&AutoScalingPlansState> for AutoScalingPlansStateView {
    fn from(state: &AutoScalingPlansState) -> Self {
        Self {
            plans: state
                .plans
                .iter()
                .map(|((n, v), p)| (key_to_string(n, *v), ScalingPlanView::from(p)))
                .collect(),
        }
    }
}

impl From<AutoScalingPlansStateView> for AutoScalingPlansState {
    fn from(view: AutoScalingPlansStateView) -> Self {
        let mut plans = HashMap::new();
        for (k, v) in view.plans {
            if let Some(key) = string_to_key(&k) {
                plans.insert(key, ScalingPlan::from(v));
            }
        }
        Self { plans }
    }
}

impl StatefulService for AutoScalingPlansService {
    type StateView = AutoScalingPlansStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AutoScalingPlansStateView::from(&*guard)
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
            *guard = AutoScalingPlansState::from(view);
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
                if let Some(key) = string_to_key(&k) {
                    guard.plans.insert(key, ScalingPlan::from(v));
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
