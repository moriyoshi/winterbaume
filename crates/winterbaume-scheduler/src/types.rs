/// A schedule.
#[derive(Debug, Clone)]
pub struct Schedule {
    pub name: String,
    pub arn: String,
    pub group_name: String,
    pub schedule_expression: String,
    pub flexible_time_window: FlexibleTimeWindow,
    pub target: ScheduleTarget,
    pub state: ScheduleState,
    pub description: Option<String>,
    pub action_after_completion: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub creation_date: String,
    pub last_modification_date: String,
    pub tags: Vec<Tag>,
}

/// FlexibleTimeWindow
#[derive(Debug, Clone)]
pub struct FlexibleTimeWindow {
    pub mode: String,
    pub maximum_window_in_minutes: Option<i64>,
}

/// Schedule target
#[derive(Debug, Clone)]
pub struct ScheduleTarget {
    pub arn: String,
    pub role_arn: String,
    pub retry_policy: RetryPolicy,
}

/// Retry policy
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub maximum_event_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            maximum_event_age_in_seconds: 86400,
            maximum_retry_attempts: 185,
        }
    }
}

/// Schedule state
#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleState {
    Enabled,
    Disabled,
}

impl ScheduleState {
    pub fn as_str(&self) -> &str {
        match self {
            ScheduleState::Enabled => "ENABLED",
            ScheduleState::Disabled => "DISABLED",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "DISABLED" => ScheduleState::Disabled,
            _ => ScheduleState::Enabled,
        }
    }
}

/// A schedule group.
#[derive(Debug, Clone)]
pub struct ScheduleGroup {
    pub name: String,
    pub arn: String,
    pub state: String,
    pub creation_date: String,
    pub last_modification_date: String,
    pub tags: Vec<Tag>,
}

/// A tag.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}
