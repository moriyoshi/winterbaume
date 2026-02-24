//! Serde-compatible view types for Timestream Query state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TimestreamQueryService;
use crate::state::{AccountSettings, TimestreamQueryState};
use crate::types::{ScheduledQuery, ScheduledQueryState as SQState};

/// Serializable view of the entire Timestream Query state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestreamQueryStateView {
    #[serde(default)]
    pub scheduled_queries: HashMap<String, ScheduledQueryView>,
    #[serde(default)]
    pub account_settings: AccountSettingsView,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

/// Serializable view of account-level settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingsView {
    pub max_query_tcu: i32,
    pub query_pricing_model: String,
}

impl Default for AccountSettingsView {
    fn default() -> Self {
        Self {
            max_query_tcu: 1000,
            query_pricing_model: "BYTES_SCANNED".to_string(),
        }
    }
}

impl From<&AccountSettings> for AccountSettingsView {
    fn from(s: &AccountSettings) -> Self {
        AccountSettingsView {
            max_query_tcu: s.max_query_tcu,
            query_pricing_model: s.query_pricing_model.clone(),
        }
    }
}

impl From<AccountSettingsView> for AccountSettings {
    fn from(v: AccountSettingsView) -> Self {
        AccountSettings {
            max_query_tcu: v.max_query_tcu,
            query_pricing_model: v.query_pricing_model,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledQueryView {
    pub arn: String,
    pub name: String,
    pub query_string: String,
    pub state: String,
    pub schedule_expression: String,
    pub target_database: String,
    pub target_table: String,
    pub s3_error_report_bucket: Option<String>,
    /// Creation time in RFC 3339 format.
    pub creation_time: String,
    pub last_run_status: Option<String>,
    pub notification_topic_arn: String,
    pub role_arn: String,
    /// `error_report_configuration` nested block.
    #[serde(default)]
    pub error_report_configuration: Option<JsonValue>,
    /// `target_configuration` nested block.
    #[serde(default)]
    pub target_configuration: Option<JsonValue>,
    /// `notification_configuration` nested block.
    #[serde(default)]
    pub notification_configuration: Option<JsonValue>,
    /// `schedule_configuration` nested block.
    #[serde(default)]
    pub schedule_configuration: Option<JsonValue>,
    /// `recently_failed_runs` nested blocks.
    #[serde(default)]
    pub recently_failed_runs: Vec<JsonValue>,
    /// `last_run_summary` nested block.
    #[serde(default)]
    pub last_run_summary: Option<JsonValue>,
}

// --- From internal types to view types ---

impl From<&TimestreamQueryState> for TimestreamQueryStateView {
    fn from(state: &TimestreamQueryState) -> Self {
        TimestreamQueryStateView {
            scheduled_queries: state
                .scheduled_queries
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ScheduledQueryView {
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            query_string: v.query_string.clone(),
                            state: v.state.as_str().to_string(),
                            schedule_expression: v.schedule_expression.clone(),
                            target_database: v.target_database.clone(),
                            target_table: v.target_table.clone(),
                            s3_error_report_bucket: v.s3_error_report_bucket.clone(),
                            creation_time: v.creation_time.to_rfc3339(),
                            last_run_status: v.last_run_status.clone(),
                            notification_topic_arn: v.notification_topic_arn.clone(),
                            role_arn: v.role_arn.clone(),
                            error_report_configuration: None,
                            target_configuration: None,
                            notification_configuration: None,
                            schedule_configuration: None,
                            recently_failed_runs: vec![],
                            last_run_summary: None,
                        },
                    )
                })
                .collect(),
            account_settings: AccountSettingsView::from(&state.account_settings),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<TimestreamQueryStateView> for TimestreamQueryState {
    fn from(view: TimestreamQueryStateView) -> Self {
        TimestreamQueryState {
            scheduled_queries: view
                .scheduled_queries
                .into_iter()
                .map(|(k, v)| {
                    let creation_time = chrono::DateTime::parse_from_rfc3339(&v.creation_time)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now());
                    (
                        k,
                        ScheduledQuery {
                            arn: v.arn,
                            name: v.name,
                            query_string: v.query_string,
                            state: SQState::from_str(&v.state).unwrap_or(SQState::Enabled),
                            schedule_expression: v.schedule_expression,
                            target_database: v.target_database,
                            target_table: v.target_table,
                            s3_error_report_bucket: v.s3_error_report_bucket,
                            creation_time,
                            last_run_status: v.last_run_status,
                            notification_topic_arn: v.notification_topic_arn,
                            role_arn: v.role_arn,
                        },
                    )
                })
                .collect(),
            account_settings: AccountSettings::from(view.account_settings),
            resource_tags: view.resource_tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for TimestreamQueryService {
    type StateView = TimestreamQueryStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TimestreamQueryStateView::from(&*guard)
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
            *guard = TimestreamQueryState::from(view);
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
            let new_state = TimestreamQueryState::from(view);
            guard.scheduled_queries.extend(new_state.scheduled_queries);
            for (arn, tags) in new_state.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }
            guard.account_settings = new_state.account_settings;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
