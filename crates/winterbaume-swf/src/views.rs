//! Serde-compatible view types for SWF state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SwfService;
use crate::state::SwfState;
use crate::types::{
    ActivityTaskEntry, ActivityTypeDef, Domain, ExecutionStatus, HistoryEventDef,
    RegistrationStatus, WorkflowExecutionDef, WorkflowSignal, WorkflowTypeDef,
};

/// Serializable view of the entire SWF state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwfStateView {
    #[serde(default)]
    pub domains: HashMap<String, DomainView>,
    /// Activity types keyed by "domain\x1fdomain_name\x1fversion".
    #[serde(default)]
    pub activity_types: HashMap<String, ActivityTypeView>,
    /// Workflow types keyed by "domain\x1fname\x1fversion".
    #[serde(default)]
    pub workflow_types: HashMap<String, WorkflowTypeView>,
    /// Workflow executions keyed by "domain\x1fworkflow_id\x1frun_id".
    #[serde(default)]
    pub executions: HashMap<String, WorkflowExecutionView>,
    /// Activity tasks keyed by task_token.
    #[serde(default)]
    pub activity_tasks: HashMap<String, ActivityTaskView>,
    /// Signals keyed by "domain\x1fworkflow_id".
    #[serde(default)]
    pub signals: HashMap<String, Vec<WorkflowSignalView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainView {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub workflow_execution_retention_period_in_days: String,
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTypeView {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: f64,
    pub deprecation_date: Option<f64>,
    pub default_task_list: Option<String>,
    pub default_task_heartbeat_timeout: Option<String>,
    pub default_task_schedule_to_start_timeout: Option<String>,
    pub default_task_schedule_to_close_timeout: Option<String>,
    pub default_task_start_to_close_timeout: Option<String>,
    pub default_task_priority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTypeView {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub description: Option<String>,
    pub status: String,
    pub creation_date: f64,
    pub deprecation_date: Option<f64>,
    pub default_task_list: Option<String>,
    pub default_execution_start_to_close_timeout: Option<String>,
    pub default_task_start_to_close_timeout: Option<String>,
    pub default_child_policy: Option<String>,
    pub default_lambda_role: Option<String>,
    pub default_task_priority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionView {
    pub workflow_id: String,
    pub run_id: String,
    pub domain: String,
    pub workflow_type_name: String,
    pub workflow_type_version: String,
    pub status: String,
    pub close_status: Option<String>,
    pub start_timestamp: f64,
    pub close_timestamp: Option<f64>,
    #[serde(default)]
    pub tag_list: Option<Vec<String>>,
    pub cancel_requested: bool,
    pub task_list: String,
    pub child_policy: String,
    pub execution_start_to_close_timeout: String,
    pub task_start_to_close_timeout: String,
    pub task_priority: Option<String>,
    pub lambda_role: Option<String>,
    #[serde(default)]
    pub history_events: Vec<HistoryEventView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEventView {
    pub event_id: i64,
    pub event_timestamp: f64,
    pub event_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTaskView {
    pub task_token: String,
    pub activity_id: String,
    pub activity_type_name: String,
    pub activity_type_version: String,
    pub domain: String,
    pub workflow_id: String,
    pub run_id: String,
    pub input: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSignalView {
    pub signal_name: String,
    pub input: Option<String>,
}

// --- Tuple key helpers ---

fn tuple3_key(a: &str, b: &str, c: &str) -> String {
    format!("{}\x1f{}\x1f{}", a, b, c)
}

fn tuple2_key(a: &str, b: &str) -> String {
    format!("{}\x1f{}", a, b)
}

fn parse_tuple3(key: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = key.splitn(3, '\x1f').collect();
    if parts.len() == 3 {
        Some((
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ))
    } else {
        None
    }
}

fn parse_tuple2(key: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = key.splitn(2, '\x1f').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

// --- From internal types to view types ---

impl From<&SwfState> for SwfStateView {
    fn from(state: &SwfState) -> Self {
        let domains = state
            .domains
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    DomainView {
                        name: v.name.clone(),
                        description: v.description.clone(),
                        status: v.status.as_str().to_string(),
                        workflow_execution_retention_period_in_days: v
                            .workflow_execution_retention_period_in_days
                            .clone(),
                        arn: v.arn.clone(),
                    },
                )
            })
            .collect();

        let activity_types = state
            .activity_types
            .iter()
            .map(|((domain, name, version), v)| {
                (
                    tuple3_key(domain, name, version),
                    ActivityTypeView {
                        name: v.name.clone(),
                        version: v.version.clone(),
                        domain: v.domain.clone(),
                        description: v.description.clone(),
                        status: v.status.as_str().to_string(),
                        creation_date: v.creation_date,
                        deprecation_date: v.deprecation_date,
                        default_task_list: v.default_task_list.clone(),
                        default_task_heartbeat_timeout: v.default_task_heartbeat_timeout.clone(),
                        default_task_schedule_to_start_timeout: v
                            .default_task_schedule_to_start_timeout
                            .clone(),
                        default_task_schedule_to_close_timeout: v
                            .default_task_schedule_to_close_timeout
                            .clone(),
                        default_task_start_to_close_timeout: v
                            .default_task_start_to_close_timeout
                            .clone(),
                        default_task_priority: v.default_task_priority.clone(),
                    },
                )
            })
            .collect();

        let workflow_types = state
            .workflow_types
            .iter()
            .map(|((domain, name, version), v)| {
                (
                    tuple3_key(domain, name, version),
                    WorkflowTypeView {
                        name: v.name.clone(),
                        version: v.version.clone(),
                        domain: v.domain.clone(),
                        description: v.description.clone(),
                        status: v.status.as_str().to_string(),
                        creation_date: v.creation_date,
                        deprecation_date: v.deprecation_date,
                        default_task_list: v.default_task_list.clone(),
                        default_execution_start_to_close_timeout: v
                            .default_execution_start_to_close_timeout
                            .clone(),
                        default_task_start_to_close_timeout: v
                            .default_task_start_to_close_timeout
                            .clone(),
                        default_child_policy: v.default_child_policy.clone(),
                        default_lambda_role: v.default_lambda_role.clone(),
                        default_task_priority: v.default_task_priority.clone(),
                    },
                )
            })
            .collect();

        let executions = state
            .executions
            .iter()
            .map(|((domain, wf_id, run_id), v)| {
                (
                    tuple3_key(domain, wf_id, run_id),
                    WorkflowExecutionView {
                        workflow_id: v.workflow_id.clone(),
                        run_id: v.run_id.clone(),
                        domain: v.domain.clone(),
                        workflow_type_name: v.workflow_type_name.clone(),
                        workflow_type_version: v.workflow_type_version.clone(),
                        status: v.status.as_str().to_string(),
                        close_status: v.close_status.clone(),
                        start_timestamp: v.start_timestamp,
                        close_timestamp: v.close_timestamp,
                        tag_list: v.tag_list.clone(),
                        cancel_requested: v.cancel_requested,
                        task_list: v.task_list.clone(),
                        child_policy: v.child_policy.clone(),
                        execution_start_to_close_timeout: v
                            .execution_start_to_close_timeout
                            .clone(),
                        task_start_to_close_timeout: v.task_start_to_close_timeout.clone(),
                        task_priority: v.task_priority.clone(),
                        lambda_role: v.lambda_role.clone(),
                        history_events: v
                            .history_events
                            .iter()
                            .map(|e| HistoryEventView {
                                event_id: e.event_id,
                                event_timestamp: e.event_timestamp,
                                event_type: e.event_type.clone(),
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        let activity_tasks = state
            .activity_tasks
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ActivityTaskView {
                        task_token: v.task_token.clone(),
                        activity_id: v.activity_id.clone(),
                        activity_type_name: v.activity_type_name.clone(),
                        activity_type_version: v.activity_type_version.clone(),
                        domain: v.domain.clone(),
                        workflow_id: v.workflow_id.clone(),
                        run_id: v.run_id.clone(),
                        input: v.input.clone(),
                        completed: v.completed,
                    },
                )
            })
            .collect();

        let signals = state
            .signals
            .iter()
            .map(|((domain, wf_id), sigs)| {
                (
                    tuple2_key(domain, wf_id),
                    sigs.iter()
                        .map(|s| WorkflowSignalView {
                            signal_name: s.signal_name.clone(),
                            input: s.input.clone(),
                        })
                        .collect(),
                )
            })
            .collect();

        SwfStateView {
            domains,
            activity_types,
            workflow_types,
            executions,
            activity_tasks,
            signals,
        }
    }
}

// --- From view types to internal types ---

impl From<SwfStateView> for SwfState {
    fn from(view: SwfStateView) -> Self {
        let domains = view
            .domains
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Domain {
                        name: v.name,
                        description: v.description,
                        status: RegistrationStatus::from_str(&v.status)
                            .unwrap_or(RegistrationStatus::Registered),
                        workflow_execution_retention_period_in_days: v
                            .workflow_execution_retention_period_in_days,
                        arn: v.arn,
                    },
                )
            })
            .collect();

        let activity_types = view
            .activity_types
            .into_iter()
            .filter_map(|(k, v)| {
                parse_tuple3(&k).map(|(domain, name, version)| {
                    (
                        (domain, name, version),
                        ActivityTypeDef {
                            name: v.name,
                            version: v.version,
                            domain: v.domain,
                            description: v.description,
                            status: RegistrationStatus::from_str(&v.status)
                                .unwrap_or(RegistrationStatus::Registered),
                            creation_date: v.creation_date,
                            deprecation_date: v.deprecation_date,
                            default_task_list: v.default_task_list,
                            default_task_heartbeat_timeout: v.default_task_heartbeat_timeout,
                            default_task_schedule_to_start_timeout: v
                                .default_task_schedule_to_start_timeout,
                            default_task_schedule_to_close_timeout: v
                                .default_task_schedule_to_close_timeout,
                            default_task_start_to_close_timeout: v
                                .default_task_start_to_close_timeout,
                            default_task_priority: v.default_task_priority,
                        },
                    )
                })
            })
            .collect();

        let workflow_types = view
            .workflow_types
            .into_iter()
            .filter_map(|(k, v)| {
                parse_tuple3(&k).map(|(domain, name, version)| {
                    (
                        (domain, name, version),
                        WorkflowTypeDef {
                            name: v.name,
                            version: v.version,
                            domain: v.domain,
                            description: v.description,
                            status: RegistrationStatus::from_str(&v.status)
                                .unwrap_or(RegistrationStatus::Registered),
                            creation_date: v.creation_date,
                            deprecation_date: v.deprecation_date,
                            default_task_list: v.default_task_list,
                            default_execution_start_to_close_timeout: v
                                .default_execution_start_to_close_timeout,
                            default_task_start_to_close_timeout: v
                                .default_task_start_to_close_timeout,
                            default_child_policy: v.default_child_policy,
                            default_lambda_role: v.default_lambda_role,
                            default_task_priority: v.default_task_priority,
                        },
                    )
                })
            })
            .collect();

        let executions = view
            .executions
            .into_iter()
            .filter_map(|(k, v)| {
                parse_tuple3(&k).map(|(domain, wf_id, run_id)| {
                    (
                        (domain, wf_id, run_id),
                        WorkflowExecutionDef {
                            workflow_id: v.workflow_id,
                            run_id: v.run_id,
                            domain: v.domain,
                            workflow_type_name: v.workflow_type_name,
                            workflow_type_version: v.workflow_type_version,
                            status: if v.status == "OPEN" {
                                ExecutionStatus::Open
                            } else {
                                ExecutionStatus::Closed
                            },
                            close_status: v.close_status,
                            start_timestamp: v.start_timestamp,
                            close_timestamp: v.close_timestamp,
                            tag_list: v.tag_list,
                            cancel_requested: v.cancel_requested,
                            task_list: v.task_list,
                            child_policy: v.child_policy,
                            execution_start_to_close_timeout: v.execution_start_to_close_timeout,
                            task_start_to_close_timeout: v.task_start_to_close_timeout,
                            task_priority: v.task_priority,
                            lambda_role: v.lambda_role,
                            history_events: v
                                .history_events
                                .into_iter()
                                .map(|e| HistoryEventDef {
                                    event_id: e.event_id,
                                    event_timestamp: e.event_timestamp,
                                    event_type: e.event_type,
                                })
                                .collect(),
                        },
                    )
                })
            })
            .collect();

        let activity_tasks = view
            .activity_tasks
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ActivityTaskEntry {
                        task_token: v.task_token,
                        activity_id: v.activity_id,
                        activity_type_name: v.activity_type_name,
                        activity_type_version: v.activity_type_version,
                        domain: v.domain,
                        workflow_id: v.workflow_id,
                        run_id: v.run_id,
                        input: v.input,
                        completed: v.completed,
                    },
                )
            })
            .collect();

        let signals = view
            .signals
            .into_iter()
            .filter_map(|(k, sigs)| {
                parse_tuple2(&k).map(|(domain, wf_id)| {
                    (
                        (domain, wf_id),
                        sigs.into_iter()
                            .map(|s| WorkflowSignal {
                                signal_name: s.signal_name,
                                input: s.input,
                            })
                            .collect(),
                    )
                })
            })
            .collect();

        SwfState {
            domains,
            activity_types,
            workflow_types,
            executions,
            activity_tasks,
            signals,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SwfService {
    type StateView = SwfStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SwfStateView::from(&*guard)
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
            *guard = SwfState::from(view);
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
            let new_state = SwfState::from(view);
            guard.domains.extend(new_state.domains);
            guard.activity_types.extend(new_state.activity_types);
            guard.workflow_types.extend(new_state.workflow_types);
            guard.executions.extend(new_state.executions);
            guard.activity_tasks.extend(new_state.activity_tasks);
            guard.signals.extend(new_state.signals);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
