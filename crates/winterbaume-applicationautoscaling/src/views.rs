//! Serde-compatible view types for Application Auto Scaling state snapshots.

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApplicationAutoScalingService;
use crate::state::ApplicationAutoScalingState;

/// Serializable view of the entire Application Auto Scaling state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationAutoScalingStateView {
    /// Scalable targets as a list (tuple keys are flattened).
    #[serde(default)]
    pub scalable_targets: Vec<ScalableTargetView>,
    /// Scaling policies as a list (tuple keys are flattened).
    #[serde(default)]
    pub scaling_policies: Vec<ScalingPolicyView>,
    /// Scheduled actions as a list.
    #[serde(default)]
    pub scheduled_actions: Vec<ScheduledActionView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalableTargetView {
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub min_capacity: i64,
    pub max_capacity: i64,
    pub role_arn: String,
    pub creation_time: String,
    pub suspended_state: Option<SuspendedStateView>,
    pub scalable_target_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendedStateView {
    pub dynamic_scaling_in_suspended: bool,
    pub dynamic_scaling_out_suspended: bool,
    pub scheduled_scaling_suspended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicyView {
    pub policy_arn: String,
    pub policy_name: String,
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub policy_type: String,
    pub creation_time: String,
    pub target_tracking_scaling_policy_configuration: Option<serde_json::Value>,
    pub step_scaling_policy_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledActionView {
    pub scheduled_action_name: String,
    pub scheduled_action_arn: String,
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub schedule: Option<String>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub timezone: Option<String>,
    pub min_capacity: Option<i32>,
    pub max_capacity: Option<i32>,
    pub creation_time: String,
}

// --- From internal types to view types ---

impl From<&ApplicationAutoScalingState> for ApplicationAutoScalingStateView {
    fn from(state: &ApplicationAutoScalingState) -> Self {
        ApplicationAutoScalingStateView {
            scalable_targets: state
                .scalable_targets
                .values()
                .map(|t| ScalableTargetView {
                    service_namespace: t.service_namespace.clone(),
                    resource_id: t.resource_id.clone(),
                    scalable_dimension: t.scalable_dimension.clone(),
                    min_capacity: t.min_capacity,
                    max_capacity: t.max_capacity,
                    role_arn: t.role_arn.clone(),
                    creation_time: t.creation_time.to_rfc3339(),
                    suspended_state: t.suspended_state.as_ref().map(|ss| SuspendedStateView {
                        dynamic_scaling_in_suspended: ss.dynamic_scaling_in_suspended,
                        dynamic_scaling_out_suspended: ss.dynamic_scaling_out_suspended,
                        scheduled_scaling_suspended: ss.scheduled_scaling_suspended,
                    }),
                    scalable_target_arn: t.scalable_target_arn.clone(),
                })
                .collect(),
            scaling_policies: state
                .scaling_policies
                .values()
                .map(|p| ScalingPolicyView {
                    policy_arn: p.policy_arn.clone(),
                    policy_name: p.policy_name.clone(),
                    service_namespace: p.service_namespace.clone(),
                    resource_id: p.resource_id.clone(),
                    scalable_dimension: p.scalable_dimension.clone(),
                    policy_type: p.policy_type.clone(),
                    creation_time: p.creation_time.to_rfc3339(),
                    target_tracking_scaling_policy_configuration: p
                        .target_tracking_scaling_policy_configuration
                        .clone(),
                    step_scaling_policy_configuration: p.step_scaling_policy_configuration.clone(),
                })
                .collect(),
            scheduled_actions: state
                .scheduled_actions
                .values()
                .map(|a| ScheduledActionView {
                    scheduled_action_name: a.scheduled_action_name.clone(),
                    scheduled_action_arn: a.scheduled_action_arn.clone(),
                    service_namespace: a.service_namespace.clone(),
                    resource_id: a.resource_id.clone(),
                    scalable_dimension: a.scalable_dimension.clone(),
                    schedule: a.schedule.clone(),
                    start_time: a.start_time,
                    end_time: a.end_time,
                    timezone: a.timezone.clone(),
                    min_capacity: a
                        .scalable_target_action
                        .as_ref()
                        .and_then(|s| s.min_capacity),
                    max_capacity: a
                        .scalable_target_action
                        .as_ref()
                        .and_then(|s| s.max_capacity),
                    creation_time: a.creation_time.to_rfc3339(),
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

// --- From view types to internal types ---

impl From<ApplicationAutoScalingStateView> for ApplicationAutoScalingState {
    fn from(view: ApplicationAutoScalingStateView) -> Self {
        let mut state = ApplicationAutoScalingState::default();
        for t in view.scalable_targets {
            let key = (
                t.service_namespace.clone(),
                t.resource_id.clone(),
                t.scalable_dimension.clone(),
            );
            state.scalable_targets.insert(
                key,
                crate::types::ScalableTarget {
                    service_namespace: t.service_namespace,
                    resource_id: t.resource_id,
                    scalable_dimension: t.scalable_dimension,
                    min_capacity: t.min_capacity,
                    max_capacity: t.max_capacity,
                    role_arn: t.role_arn,
                    creation_time: parse_datetime(&t.creation_time),
                    suspended_state: t.suspended_state.map(|ss| crate::types::SuspendedState {
                        dynamic_scaling_in_suspended: ss.dynamic_scaling_in_suspended,
                        dynamic_scaling_out_suspended: ss.dynamic_scaling_out_suspended,
                        scheduled_scaling_suspended: ss.scheduled_scaling_suspended,
                    }),
                    scalable_target_arn: t.scalable_target_arn,
                },
            );
        }
        for p in view.scaling_policies {
            let key = (
                p.service_namespace.clone(),
                p.resource_id.clone(),
                p.scalable_dimension.clone(),
                p.policy_name.clone(),
            );
            state.scaling_policies.insert(
                key,
                crate::types::ScalingPolicy {
                    policy_arn: p.policy_arn,
                    policy_name: p.policy_name,
                    service_namespace: p.service_namespace,
                    resource_id: p.resource_id,
                    scalable_dimension: p.scalable_dimension,
                    policy_type: p.policy_type,
                    creation_time: parse_datetime(&p.creation_time),
                    target_tracking_scaling_policy_configuration: p
                        .target_tracking_scaling_policy_configuration,
                    step_scaling_policy_configuration: p.step_scaling_policy_configuration,
                    alarms: Vec::new(),
                },
            );
        }
        for a in view.scheduled_actions {
            let key = (
                a.service_namespace.clone(),
                a.resource_id.clone(),
                a.scalable_dimension.clone(),
                a.scheduled_action_name.clone(),
            );
            state.scheduled_actions.insert(
                key,
                crate::types::ScheduledAction {
                    scheduled_action_name: a.scheduled_action_name,
                    scheduled_action_arn: a.scheduled_action_arn,
                    service_namespace: a.service_namespace,
                    resource_id: a.resource_id,
                    scalable_dimension: a.scalable_dimension,
                    schedule: a.schedule,
                    start_time: a.start_time,
                    end_time: a.end_time,
                    timezone: a.timezone,
                    scalable_target_action: if a.min_capacity.is_some() || a.max_capacity.is_some()
                    {
                        Some(crate::types::ScalableTargetAction {
                            min_capacity: a.min_capacity,
                            max_capacity: a.max_capacity,
                        })
                    } else {
                        None
                    },
                    creation_time: parse_datetime(&a.creation_time),
                },
            );
        }
        state.tags = view.tags;
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for ApplicationAutoScalingService {
    type StateView = ApplicationAutoScalingStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApplicationAutoScalingStateView::from(&*guard)
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
            *guard = ApplicationAutoScalingState::from(view);
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
            let merged = ApplicationAutoScalingState::from(view);
            for (k, v) in merged.scalable_targets {
                guard.scalable_targets.insert(k, v);
            }
            for (k, v) in merged.scaling_policies {
                guard.scaling_policies.insert(k, v);
            }
            for (k, v) in merged.scheduled_actions {
                guard.scheduled_actions.insert(k, v);
            }
            for (arn, resource_tags) in merged.tags {
                guard.tags.entry(arn).or_default().extend(resource_tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
