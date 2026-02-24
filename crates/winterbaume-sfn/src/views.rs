//! Serde-compatible view types for Step Functions state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SfnService;
use crate::state::StepFunctionsState;
use crate::types::{
    Activity, CloudWatchLogsLogGroup, EncryptionConfiguration, HistoryEvent, LogDestination,
    LoggingConfiguration, StateMachine, StateMachineAlias, StateMachineStatus, Tag, TaskToken,
    TracingConfiguration,
};

/// Serializable view of the entire Step Functions state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StepfunctionsStateView {
    /// State machines keyed by ARN.
    #[serde(default)]
    pub state_machines: HashMap<String, StateMachineView>,
    /// Activities keyed by ARN.
    #[serde(default)]
    pub activities: HashMap<String, ActivityView>,
    /// Task tokens keyed by token string.
    #[serde(default)]
    pub task_tokens: HashMap<String, TaskTokenView>,
    /// History events keyed by execution ARN.
    #[serde(default)]
    pub execution_history: HashMap<String, Vec<HistoryEventView>>,
    /// State machine aliases keyed by alias ARN.
    #[serde(default)]
    pub aliases: HashMap<String, StateMachineAliasView>,
}

/// Serializable view of a single state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineView {
    /// State machine name.
    pub name: String,
    /// State machine ARN.
    pub arn: String,
    /// Amazon States Language definition.
    pub definition: String,
    /// IAM role ARN.
    pub role_arn: String,
    /// Status string (ACTIVE or DELETING).
    pub status: String,
    /// Creation date in RFC 3339 format.
    pub creation_date: String,
    /// Type (STANDARD or EXPRESS).
    pub r#type: String,
    /// Tags.
    #[serde(default)]
    pub tags: Vec<TagView>,
    /// Logging configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfigurationView>,
    /// Tracing configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<TracingConfigurationView>,
    /// Encryption configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfigurationView>,
}

/// Serializable view of an activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityView {
    /// Activity name.
    pub name: String,
    /// Activity ARN.
    pub arn: String,
    /// Creation date in RFC 3339 format.
    pub creation_date: String,
    /// Tags.
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable key-value tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfigurationView {
    pub level: Option<String>,
    pub include_execution_data: Option<bool>,
    #[serde(default)]
    pub destinations: Vec<LogDestinationView>,
}

/// Serializable view of a log destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogDestinationView {
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroupView>,
}

/// Serializable view of a CloudWatch Logs log group reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudWatchLogsLogGroupView {
    pub log_group_arn: Option<String>,
}

/// Serializable view of a tracing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfigurationView {
    pub enabled: bool,
}

/// Serializable view of an encryption configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfigurationView {
    pub kms_key_id: Option<String>,
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    pub r#type: String,
}

/// Serializable view of a task token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTokenView {
    pub token: String,
    pub execution_arn: String,
}

/// Serializable view of an execution history event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEventView {
    pub id: i64,
    pub event_type: String,
    /// Event timestamp in RFC 3339 format.
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_event_id: Option<i64>,
}

/// Serializable view of a state machine alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineAliasView {
    pub alias_arn: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Routing configuration as `(version_arn, weight)` pairs.
    #[serde(default)]
    pub routing_configuration: Vec<RoutingConfigEntryView>,
    /// Creation date in RFC 3339 format.
    pub creation_date: String,
    /// Last update date in RFC 3339 format.
    pub update_date: String,
}

/// Serializable view of a single routing configuration entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfigEntryView {
    pub state_machine_version_arn: String,
    pub weight: i32,
}

// --- From internal types to view types ---

impl From<&StepFunctionsState> for StepfunctionsStateView {
    fn from(state: &StepFunctionsState) -> Self {
        StepfunctionsStateView {
            state_machines: state
                .state_machines
                .iter()
                .map(|(k, v)| (k.clone(), StateMachineView::from(v)))
                .collect(),
            activities: state
                .activities
                .iter()
                .map(|(k, v)| (k.clone(), ActivityView::from(v)))
                .collect(),
            task_tokens: state
                .task_tokens
                .iter()
                .map(|(k, v)| (k.clone(), TaskTokenView::from(v)))
                .collect(),
            execution_history: state
                .execution_history
                .iter()
                .map(|(k, events)| {
                    (
                        k.clone(),
                        events.iter().map(HistoryEventView::from).collect(),
                    )
                })
                .collect(),
            aliases: state
                .aliases
                .iter()
                .map(|(k, v)| (k.clone(), StateMachineAliasView::from(v)))
                .collect(),
        }
    }
}

impl From<&TaskToken> for TaskTokenView {
    fn from(t: &TaskToken) -> Self {
        TaskTokenView {
            token: t.token.clone(),
            execution_arn: t.execution_arn.clone(),
        }
    }
}

impl From<&HistoryEvent> for HistoryEventView {
    fn from(e: &HistoryEvent) -> Self {
        HistoryEventView {
            id: e.id,
            event_type: e.event_type.clone(),
            timestamp: e.timestamp.to_rfc3339(),
            previous_event_id: e.previous_event_id,
        }
    }
}

impl From<&StateMachineAlias> for StateMachineAliasView {
    fn from(a: &StateMachineAlias) -> Self {
        StateMachineAliasView {
            alias_arn: a.alias_arn.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            routing_configuration: a
                .routing_configuration
                .iter()
                .map(|(arn, weight)| RoutingConfigEntryView {
                    state_machine_version_arn: arn.clone(),
                    weight: *weight,
                })
                .collect(),
            creation_date: a.creation_date.to_rfc3339(),
            update_date: a.update_date.to_rfc3339(),
        }
    }
}

impl From<&StateMachine> for StateMachineView {
    fn from(sm: &StateMachine) -> Self {
        StateMachineView {
            name: sm.name.clone(),
            arn: sm.arn.clone(),
            definition: sm.definition.clone(),
            role_arn: sm.role_arn.clone(),
            status: sm.status.as_str().to_string(),
            creation_date: sm.creation_date.to_rfc3339(),
            r#type: sm.r#type.clone(),
            tags: sm.tags.iter().map(TagView::from).collect(),
            logging_configuration: sm.logging_configuration.as_ref().map(|lc| {
                LoggingConfigurationView {
                    level: lc.level.clone(),
                    include_execution_data: lc.include_execution_data,
                    destinations: lc
                        .destinations
                        .iter()
                        .map(|d| LogDestinationView {
                            cloud_watch_logs_log_group: d.cloud_watch_logs_log_group.as_ref().map(
                                |cw| CloudWatchLogsLogGroupView {
                                    log_group_arn: cw.log_group_arn.clone(),
                                },
                            ),
                        })
                        .collect(),
                }
            }),
            tracing_configuration: sm.tracing_configuration.as_ref().map(|tc| {
                TracingConfigurationView {
                    enabled: tc.enabled,
                }
            }),
            encryption_configuration: sm.encryption_configuration.as_ref().map(|ec| {
                EncryptionConfigurationView {
                    kms_key_id: ec.kms_key_id.clone(),
                    kms_data_key_reuse_period_seconds: ec.kms_data_key_reuse_period_seconds,
                    r#type: ec.r#type.clone(),
                }
            }),
        }
    }
}

impl From<&Activity> for ActivityView {
    fn from(a: &Activity) -> Self {
        ActivityView {
            name: a.name.clone(),
            arn: a.arn.clone(),
            creation_date: a.creation_date.to_rfc3339(),
            tags: a.tags.iter().map(TagView::from).collect(),
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

impl From<StepfunctionsStateView> for StepFunctionsState {
    fn from(view: StepfunctionsStateView) -> Self {
        StepFunctionsState {
            state_machines: view
                .state_machines
                .into_iter()
                .map(|(k, v)| (k, StateMachine::from(v)))
                .collect(),
            activities: view
                .activities
                .into_iter()
                .map(|(k, v)| (k, Activity::from(v)))
                .collect(),
            map_runs: HashMap::new(),
            task_tokens: view
                .task_tokens
                .into_iter()
                .map(|(k, v)| (k, TaskToken::from(v)))
                .collect(),
            execution_history: view
                .execution_history
                .into_iter()
                .map(|(k, events)| {
                    (
                        k,
                        events
                            .into_iter()
                            .map(HistoryEvent::from)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect(),
            aliases: view
                .aliases
                .into_iter()
                .map(|(k, v)| (k, StateMachineAlias::from(v)))
                .collect(),
            versions: HashMap::new(),
            next_version: HashMap::new(),
        }
    }
}

impl From<TaskTokenView> for TaskToken {
    fn from(view: TaskTokenView) -> Self {
        TaskToken {
            token: view.token,
            execution_arn: view.execution_arn,
        }
    }
}

impl From<HistoryEventView> for HistoryEvent {
    fn from(view: HistoryEventView) -> Self {
        let timestamp = DateTime::parse_from_rfc3339(&view.timestamp)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        HistoryEvent {
            id: view.id,
            event_type: view.event_type,
            timestamp,
            previous_event_id: view.previous_event_id,
        }
    }
}

impl From<StateMachineAliasView> for StateMachineAlias {
    fn from(view: StateMachineAliasView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let update_date = DateTime::parse_from_rfc3339(&view.update_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        StateMachineAlias {
            alias_arn: view.alias_arn,
            name: view.name,
            description: view.description,
            routing_configuration: view
                .routing_configuration
                .into_iter()
                .map(|entry| (entry.state_machine_version_arn, entry.weight))
                .collect(),
            creation_date,
            update_date,
        }
    }
}

impl From<StateMachineView> for StateMachine {
    fn from(view: StateMachineView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let status = match view.status.as_str() {
            "DELETING" => StateMachineStatus::Deleting,
            _ => StateMachineStatus::Active,
        };
        StateMachine {
            name: view.name,
            arn: view.arn,
            definition: view.definition,
            role_arn: view.role_arn,
            status,
            creation_date,
            r#type: view.r#type,
            executions: Vec::new(),
            tags: view.tags.into_iter().map(Tag::from).collect(),
            logging_configuration: view.logging_configuration.map(|lc| LoggingConfiguration {
                level: lc.level,
                include_execution_data: lc.include_execution_data,
                destinations: lc
                    .destinations
                    .into_iter()
                    .map(|d| LogDestination {
                        cloud_watch_logs_log_group: d.cloud_watch_logs_log_group.map(|cw| {
                            CloudWatchLogsLogGroup {
                                log_group_arn: cw.log_group_arn,
                            }
                        }),
                    })
                    .collect(),
            }),
            tracing_configuration: view.tracing_configuration.map(|tc| TracingConfiguration {
                enabled: tc.enabled,
            }),
            encryption_configuration: view.encryption_configuration.map(|ec| {
                EncryptionConfiguration {
                    kms_key_id: ec.kms_key_id,
                    kms_data_key_reuse_period_seconds: ec.kms_data_key_reuse_period_seconds,
                    r#type: ec.r#type,
                }
            }),
        }
    }
}

impl From<ActivityView> for Activity {
    fn from(view: ActivityView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        Activity {
            name: view.name,
            arn: view.arn,
            creation_date,
            tags: view.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<TagView> for Tag {
    fn from(view: TagView) -> Self {
        Tag {
            key: view.key,
            value: view.value,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SfnService {
    type StateView = StepfunctionsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        StepfunctionsStateView::from(&*guard)
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
            *guard = StepFunctionsState::from(view);
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
            for (arn, sm_view) in view.state_machines {
                guard
                    .state_machines
                    .insert(arn, StateMachine::from(sm_view));
            }
            for (arn, activity_view) in view.activities {
                guard.activities.insert(arn, Activity::from(activity_view));
            }
            for (token, tt_view) in view.task_tokens {
                guard.task_tokens.insert(token, TaskToken::from(tt_view));
            }
            for (exec_arn, events_view) in view.execution_history {
                let events: Vec<HistoryEvent> =
                    events_view.into_iter().map(HistoryEvent::from).collect();
                guard
                    .execution_history
                    .entry(exec_arn)
                    .or_default()
                    .extend(events);
            }
            for (arn, alias_view) in view.aliases {
                guard
                    .aliases
                    .insert(arn, StateMachineAlias::from(alias_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
