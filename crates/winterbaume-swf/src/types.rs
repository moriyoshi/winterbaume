/// An in-flight activity task assigned to a worker.
#[derive(Debug, Clone)]
pub struct ActivityTaskEntry {
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

/// A workflow signal.
#[derive(Debug, Clone)]
pub struct WorkflowSignal {
    pub signal_name: String,
    pub input: Option<String>,
}

/// A registered SWF domain.
#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub description: Option<String>,
    pub status: RegistrationStatus,
    pub workflow_execution_retention_period_in_days: String,
    pub arn: String,
}

/// A registered activity type.
#[derive(Debug, Clone)]
pub struct ActivityTypeDef {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub description: Option<String>,
    pub status: RegistrationStatus,
    pub creation_date: f64,
    pub deprecation_date: Option<f64>,
    pub default_task_list: Option<String>,
    pub default_task_heartbeat_timeout: Option<String>,
    pub default_task_schedule_to_start_timeout: Option<String>,
    pub default_task_schedule_to_close_timeout: Option<String>,
    pub default_task_start_to_close_timeout: Option<String>,
    pub default_task_priority: Option<String>,
}

/// A registered workflow type.
#[derive(Debug, Clone)]
pub struct WorkflowTypeDef {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub description: Option<String>,
    pub status: RegistrationStatus,
    pub creation_date: f64,
    pub deprecation_date: Option<f64>,
    pub default_task_list: Option<String>,
    pub default_execution_start_to_close_timeout: Option<String>,
    pub default_task_start_to_close_timeout: Option<String>,
    pub default_child_policy: Option<String>,
    pub default_lambda_role: Option<String>,
    pub default_task_priority: Option<String>,
}

/// A workflow execution.
#[derive(Debug, Clone)]
pub struct WorkflowExecutionDef {
    pub workflow_id: String,
    pub run_id: String,
    pub domain: String,
    pub workflow_type_name: String,
    pub workflow_type_version: String,
    pub status: ExecutionStatus,
    pub close_status: Option<String>,
    pub start_timestamp: f64,
    pub close_timestamp: Option<f64>,
    pub tag_list: Option<Vec<String>>,
    pub cancel_requested: bool,
    pub task_list: String,
    pub child_policy: String,
    pub execution_start_to_close_timeout: String,
    pub task_start_to_close_timeout: String,
    pub task_priority: Option<String>,
    pub lambda_role: Option<String>,
    /// History events for this execution.
    pub history_events: Vec<HistoryEventDef>,
}

/// A history event in a workflow execution.
#[derive(Debug, Clone)]
pub struct HistoryEventDef {
    pub event_id: i64,
    pub event_timestamp: f64,
    pub event_type: String,
}

/// Registration status of a domain, activity type, or workflow type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationStatus {
    Registered,
    Deprecated,
}

impl RegistrationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            RegistrationStatus::Registered => "REGISTERED",
            RegistrationStatus::Deprecated => "DEPRECATED",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "REGISTERED" => Some(RegistrationStatus::Registered),
            "DEPRECATED" => Some(RegistrationStatus::Deprecated),
            _ => None,
        }
    }
}

/// Execution status of a workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    Open,
    Closed,
}

impl ExecutionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionStatus::Open => "OPEN",
            ExecutionStatus::Closed => "CLOSED",
        }
    }
}
