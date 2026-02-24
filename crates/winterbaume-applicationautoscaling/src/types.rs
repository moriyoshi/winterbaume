use chrono::{DateTime, Utc};

/// A scalable target registered with Application Auto Scaling.
#[derive(Debug, Clone)]
pub struct ScalableTarget {
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub min_capacity: i64,
    pub max_capacity: i64,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub suspended_state: Option<SuspendedState>,
    pub scalable_target_arn: String,
}

/// Suspended state configuration for a scalable target.
#[derive(Debug, Clone)]
pub struct SuspendedState {
    pub dynamic_scaling_in_suspended: bool,
    pub dynamic_scaling_out_suspended: bool,
    pub scheduled_scaling_suspended: bool,
}

/// A scaling policy.
#[derive(Debug, Clone)]
pub struct ScalingPolicy {
    pub policy_arn: String,
    pub policy_name: String,
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub policy_type: String,
    pub creation_time: DateTime<Utc>,
    pub target_tracking_scaling_policy_configuration: Option<serde_json::Value>,
    pub step_scaling_policy_configuration: Option<serde_json::Value>,
    pub alarms: Vec<Alarm>,
}

/// A CloudWatch alarm associated with a scaling policy.
#[derive(Debug, Clone)]
pub struct Alarm {
    pub alarm_name: String,
    pub alarm_arn: String,
}

/// A scheduled action for a scalable target.
#[derive(Debug, Clone)]
pub struct ScheduledAction {
    pub scheduled_action_name: String,
    pub scheduled_action_arn: String,
    pub service_namespace: String,
    pub resource_id: String,
    pub scalable_dimension: String,
    pub schedule: Option<String>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub timezone: Option<String>,
    pub scalable_target_action: Option<ScalableTargetAction>,
    pub creation_time: chrono::DateTime<chrono::Utc>,
}

/// The minimum and maximum capacity for a scheduled action.
#[derive(Debug, Clone)]
pub struct ScalableTargetAction {
    pub min_capacity: Option<i32>,
    pub max_capacity: Option<i32>,
}
