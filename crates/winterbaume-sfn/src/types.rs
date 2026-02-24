use chrono::{DateTime, Utc};

/// Logging configuration for a state machine.
#[derive(Debug, Clone)]
pub struct LoggingConfiguration {
    pub level: Option<String>,
    pub include_execution_data: Option<bool>,
    pub destinations: Vec<LogDestination>,
}

/// A single log destination entry.
#[derive(Debug, Clone)]
pub struct LogDestination {
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,
}

/// CloudWatch Logs log group reference.
#[derive(Debug, Clone)]
pub struct CloudWatchLogsLogGroup {
    pub log_group_arn: Option<String>,
}

/// Tracing configuration for a state machine.
#[derive(Debug, Clone)]
pub struct TracingConfiguration {
    pub enabled: bool,
}

/// Encryption configuration for a state machine.
#[derive(Debug, Clone)]
pub struct EncryptionConfiguration {
    pub kms_key_id: Option<String>,
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    pub r#type: String,
}

/// A Step Functions state machine.
#[derive(Debug, Clone)]
pub struct StateMachine {
    pub name: String,
    pub arn: String,
    pub definition: String,
    pub role_arn: String,
    pub status: StateMachineStatus,
    pub creation_date: DateTime<Utc>,
    pub r#type: String,
    pub executions: Vec<Execution>,
    pub tags: Vec<Tag>,
    pub logging_configuration: Option<LoggingConfiguration>,
    pub tracing_configuration: Option<TracingConfiguration>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
}

/// State machine status.
#[derive(Debug, Clone, PartialEq)]
pub enum StateMachineStatus {
    Active,
    Deleting,
}

impl StateMachineStatus {
    pub fn as_str(&self) -> &str {
        match self {
            StateMachineStatus::Active => "ACTIVE",
            StateMachineStatus::Deleting => "DELETING",
        }
    }
}

/// A state machine execution.
#[derive(Debug, Clone)]
pub struct Execution {
    pub execution_arn: String,
    pub name: String,
    pub status: ExecutionStatus,
    pub start_date: DateTime<Utc>,
    pub stop_date: Option<DateTime<Utc>>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub state_machine_arn: String,
}

/// Execution status.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Running,
    Succeeded,
    Failed,
    TimedOut,
    Aborted,
}

impl ExecutionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionStatus::Running => "RUNNING",
            ExecutionStatus::Succeeded => "SUCCEEDED",
            ExecutionStatus::Failed => "FAILED",
            ExecutionStatus::TimedOut => "TIMED_OUT",
            ExecutionStatus::Aborted => "ABORTED",
        }
    }
}

/// A resource tag.
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// An activity resource.
#[derive(Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub arn: String,
    pub creation_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
}

/// A map run associated with an execution.
#[derive(Debug, Clone)]
pub struct MapRun {
    pub map_run_arn: String,
    pub execution_arn: String,
    pub state_machine_arn: String,
    pub start_date: DateTime<Utc>,
    pub stop_date: Option<DateTime<Utc>>,
    pub status: MapRunStatus,
    pub max_concurrency: i32,
    pub tolerated_failure_count: i64,
    pub tolerated_failure_percentage: f32,
}

/// Map run status.
#[derive(Debug, Clone, PartialEq)]
pub enum MapRunStatus {
    Running,
    Succeeded,
    Failed,
    Aborted,
}

impl MapRunStatus {
    pub fn as_str(&self) -> &str {
        match self {
            MapRunStatus::Running => "RUNNING",
            MapRunStatus::Succeeded => "SUCCEEDED",
            MapRunStatus::Failed => "FAILED",
            MapRunStatus::Aborted => "ABORTED",
        }
    }
}

/// A history event for an execution.
#[derive(Debug, Clone)]
pub struct HistoryEvent {
    pub id: i64,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub previous_event_id: Option<i64>,
}

/// A task token entry for callback-based tasks.
#[derive(Debug, Clone)]
pub struct TaskToken {
    pub token: String,
    pub execution_arn: String,
}

/// A state machine alias that maps a name to one or two version ARNs with weights.
#[derive(Debug, Clone)]
pub struct StateMachineAlias {
    pub alias_arn: String,
    pub name: String,
    pub description: Option<String>,
    /// Routing configuration: list of (version_arn, weight) pairs.
    pub routing_configuration: Vec<(String, i32)>,
    pub creation_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
}

/// A published version of a state machine.
#[derive(Debug, Clone)]
pub struct StateMachineVersion {
    pub version_arn: String,
    /// Base state machine ARN (without :N suffix).
    pub state_machine_arn: String,
    pub description: Option<String>,
    pub creation_date: DateTime<Utc>,
    /// Version number (1-based).
    pub version_number: u64,
}
