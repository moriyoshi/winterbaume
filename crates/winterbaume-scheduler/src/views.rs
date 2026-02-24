//! Serde-compatible view types for Scheduler state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SchedulerService;
use crate::state::SchedulerState;
use crate::types::{
    FlexibleTimeWindow, RetryPolicy, Schedule, ScheduleGroup, ScheduleState, ScheduleTarget, Tag,
};

/// Serializable view of the entire Scheduler state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStateView {
    /// Schedule groups keyed by group name.
    #[serde(default)]
    pub groups: HashMap<String, ScheduleGroupView>,
    /// Schedules keyed by "(group_name, schedule_name)".
    #[serde(default)]
    pub schedules: HashMap<String, ScheduleView>,
}

/// Serializable view of a schedule group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleGroupView {
    pub name: String,
    pub arn: String,
    pub state: String,
    pub creation_date: String,
    pub last_modification_date: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a single schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleView {
    pub name: String,
    pub arn: String,
    pub group_name: String,
    pub schedule_expression: String,
    pub flexible_time_window: FlexibleTimeWindowView,
    pub target: ScheduleTargetView,
    /// "ENABLED" or "DISABLED"
    pub state: String,
    pub description: Option<String>,
    pub action_after_completion: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub creation_date: String,
    pub last_modification_date: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a flexible time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlexibleTimeWindowView {
    pub mode: String,
    pub maximum_window_in_minutes: Option<i64>,
}

/// Serializable view of a schedule target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleTargetView {
    pub arn: String,
    pub role_arn: String,
    pub retry_policy: RetryPolicyView,
}

/// Serializable view of a retry policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicyView {
    pub maximum_event_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
}

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// --- composite key helpers ---

fn schedule_key(group_name: &str, name: &str) -> String {
    format!("{}\x00{}", group_name, name)
}

// --- From internal types to view types ---

impl From<&SchedulerState> for SchedulerStateView {
    fn from(state: &SchedulerState) -> Self {
        SchedulerStateView {
            groups: state
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), ScheduleGroupView::from(v)))
                .collect(),
            schedules: state
                .schedules
                .iter()
                .map(|((g, n), v)| (schedule_key(g, n), ScheduleView::from(v)))
                .collect(),
        }
    }
}

impl From<&ScheduleGroup> for ScheduleGroupView {
    fn from(g: &ScheduleGroup) -> Self {
        ScheduleGroupView {
            name: g.name.clone(),
            arn: g.arn.clone(),
            state: g.state.clone(),
            creation_date: g.creation_date.clone(),
            last_modification_date: g.last_modification_date.clone(),
            tags: g.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Schedule> for ScheduleView {
    fn from(s: &Schedule) -> Self {
        ScheduleView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            group_name: s.group_name.clone(),
            schedule_expression: s.schedule_expression.clone(),
            flexible_time_window: FlexibleTimeWindowView::from(&s.flexible_time_window),
            target: ScheduleTargetView::from(&s.target),
            state: s.state.as_str().to_string(),
            description: s.description.clone(),
            action_after_completion: s.action_after_completion.clone(),
            start_date: s.start_date.clone(),
            end_date: s.end_date.clone(),
            creation_date: s.creation_date.clone(),
            last_modification_date: s.last_modification_date.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&FlexibleTimeWindow> for FlexibleTimeWindowView {
    fn from(f: &FlexibleTimeWindow) -> Self {
        FlexibleTimeWindowView {
            mode: f.mode.clone(),
            maximum_window_in_minutes: f.maximum_window_in_minutes,
        }
    }
}

impl From<&ScheduleTarget> for ScheduleTargetView {
    fn from(t: &ScheduleTarget) -> Self {
        ScheduleTargetView {
            arn: t.arn.clone(),
            role_arn: t.role_arn.clone(),
            retry_policy: RetryPolicyView::from(&t.retry_policy),
        }
    }
}

impl From<&RetryPolicy> for RetryPolicyView {
    fn from(r: &RetryPolicy) -> Self {
        RetryPolicyView {
            maximum_event_age_in_seconds: r.maximum_event_age_in_seconds,
            maximum_retry_attempts: r.maximum_retry_attempts,
        }
    }
}

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<SchedulerStateView> for SchedulerState {
    fn from(view: SchedulerStateView) -> Self {
        SchedulerState {
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| (k, ScheduleGroup::from(v)))
                .collect(),
            schedules: view
                .schedules
                .into_values()
                .map(|v| {
                    let key = (v.group_name.clone(), v.name.clone());
                    (key, Schedule::from(v))
                })
                .collect(),
        }
    }
}

impl From<ScheduleGroupView> for ScheduleGroup {
    fn from(v: ScheduleGroupView) -> Self {
        ScheduleGroup {
            name: v.name,
            arn: v.arn,
            state: v.state,
            creation_date: v.creation_date,
            last_modification_date: v.last_modification_date,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<ScheduleView> for Schedule {
    fn from(v: ScheduleView) -> Self {
        Schedule {
            name: v.name,
            arn: v.arn,
            group_name: v.group_name,
            schedule_expression: v.schedule_expression,
            flexible_time_window: FlexibleTimeWindow::from(v.flexible_time_window),
            target: ScheduleTarget::from(v.target),
            state: ScheduleState::parse(&v.state),
            description: v.description,
            action_after_completion: v.action_after_completion,
            start_date: v.start_date,
            end_date: v.end_date,
            creation_date: v.creation_date,
            last_modification_date: v.last_modification_date,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<FlexibleTimeWindowView> for FlexibleTimeWindow {
    fn from(v: FlexibleTimeWindowView) -> Self {
        FlexibleTimeWindow {
            mode: v.mode,
            maximum_window_in_minutes: v.maximum_window_in_minutes,
        }
    }
}

impl From<ScheduleTargetView> for ScheduleTarget {
    fn from(v: ScheduleTargetView) -> Self {
        ScheduleTarget {
            arn: v.arn,
            role_arn: v.role_arn,
            retry_policy: RetryPolicy::from(v.retry_policy),
        }
    }
}

impl From<RetryPolicyView> for RetryPolicy {
    fn from(v: RetryPolicyView) -> Self {
        RetryPolicy {
            maximum_event_age_in_seconds: v.maximum_event_age_in_seconds,
            maximum_retry_attempts: v.maximum_retry_attempts,
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Tag {
            key: v.key,
            value: v.value,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SchedulerService {
    type StateView = SchedulerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SchedulerStateView::from(&*guard)
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
            *guard = SchedulerState::from(view);
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
            for (k, gv) in view.groups {
                guard.groups.insert(k, ScheduleGroup::from(gv));
            }
            for sv in view.schedules.into_values() {
                let key = (sv.group_name.clone(), sv.name.clone());
                guard.schedules.insert(key, Schedule::from(sv));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
