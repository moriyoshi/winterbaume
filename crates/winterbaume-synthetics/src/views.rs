//! Serde-compatible view types for Synthetics state snapshots.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SyntheticsService;
use crate::state::SyntheticsState;
use crate::types::{Canary, CanaryStatus, Group};

/// Serializable view of the entire Synthetics state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyntheticsStateView {
    #[serde(default)]
    pub canaries: HashMap<String, CanaryView>,
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub name: String,
    pub id: String,
    pub arn: String,
    #[serde(default)]
    pub resource_arns: HashSet<String>,
    pub created_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryView {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub artifact_s3_location: String,
    pub runtime_version: String,
    pub handler: String,
    pub schedule_expression: String,
    pub schedule_duration_in_seconds: Option<i64>,
    pub success_retention_period_in_days: i32,
    pub failure_retention_period_in_days: i32,
    pub status_state: String,
    pub status_state_reason: Option<String>,
    pub status_state_reason_code: Option<String>,
    /// Created timestamp in RFC 3339 format.
    pub created_at: String,
    /// Last modified timestamp in RFC 3339 format.
    pub last_modified: String,
    pub execution_role_arn: String,
    pub s3_encryption_mode: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub artifact_config: Option<serde_json::Value>,
    #[serde(default)]
    pub run_config: Option<serde_json::Value>,
    #[serde(default)]
    pub vpc_config: Option<serde_json::Value>,
}

fn parse_datetime(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&SyntheticsState> for SyntheticsStateView {
    fn from(state: &SyntheticsState) -> Self {
        SyntheticsStateView {
            canaries: state
                .canaries
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CanaryView {
                            name: v.name.clone(),
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            artifact_s3_location: v.artifact_s3_location.clone(),
                            runtime_version: v.runtime_version.clone(),
                            handler: v.handler.clone(),
                            schedule_expression: v.schedule_expression.clone(),
                            schedule_duration_in_seconds: v.schedule_duration_in_seconds,
                            success_retention_period_in_days: v.success_retention_period_in_days,
                            failure_retention_period_in_days: v.failure_retention_period_in_days,
                            status_state: v.status.state.clone(),
                            status_state_reason: v.status.state_reason.clone(),
                            status_state_reason_code: v.status.state_reason_code.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            last_modified: v.last_modified.to_rfc3339(),
                            execution_role_arn: v.execution_role_arn.clone(),
                            s3_encryption_mode: v.s3_encryption_mode.clone(),
                            tags: v.tags.clone(),
                            artifact_config: None,
                            run_config: None,
                            vpc_config: None,
                        },
                    )
                })
                .collect(),
            groups: state
                .groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        GroupView {
                            name: v.name.clone(),
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            resource_arns: v.resource_arns.clone(),
                            created_time: v.created_time.to_rfc3339(),
                            last_modified_time: v.last_modified_time.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<SyntheticsStateView> for SyntheticsState {
    fn from(view: SyntheticsStateView) -> Self {
        SyntheticsState {
            canaries: view
                .canaries
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Canary {
                            name: v.name,
                            id: v.id,
                            arn: v.arn,
                            artifact_s3_location: v.artifact_s3_location,
                            runtime_version: v.runtime_version,
                            handler: v.handler,
                            schedule_expression: v.schedule_expression,
                            schedule_duration_in_seconds: v.schedule_duration_in_seconds,
                            success_retention_period_in_days: v.success_retention_period_in_days,
                            failure_retention_period_in_days: v.failure_retention_period_in_days,
                            status: CanaryStatus {
                                state: v.status_state,
                                state_reason: v.status_state_reason,
                                state_reason_code: v.status_state_reason_code,
                            },
                            created_at: parse_datetime(&v.created_at),
                            last_modified: parse_datetime(&v.last_modified),
                            execution_role_arn: v.execution_role_arn,
                            s3_encryption_mode: v.s3_encryption_mode,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            canary_runs: Default::default(),
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Group {
                            name: v.name,
                            id: v.id,
                            arn: v.arn,
                            resource_arns: v.resource_arns,
                            created_time: parse_datetime(&v.created_time),
                            last_modified_time: parse_datetime(&v.last_modified_time),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SyntheticsService {
    type StateView = SyntheticsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SyntheticsStateView::from(&*guard)
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
            *guard = SyntheticsState::from(view);
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
            let new_state = SyntheticsState::from(view);
            for (k, v) in new_state.canaries {
                guard.canaries.insert(k, v);
            }
            for (k, v) in new_state.groups {
                guard.groups.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
