//! Domain types for EMR service.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Cluster status states.
pub const CLUSTER_STATUS_STARTING: &str = "STARTING";
pub const CLUSTER_STATUS_BOOTSTRAPPING: &str = "BOOTSTRAPPING";
pub const CLUSTER_STATUS_RUNNING: &str = "RUNNING";
pub const CLUSTER_STATUS_WAITING: &str = "WAITING";
pub const CLUSTER_STATUS_TERMINATING: &str = "TERMINATING";
pub const CLUSTER_STATUS_TERMINATED: &str = "TERMINATED";
pub const CLUSTER_STATUS_TERMINATED_WITH_ERRORS: &str = "TERMINATED_WITH_ERRORS";

/// Step status states.
pub const STEP_STATUS_PENDING: &str = "PENDING";
pub const STEP_STATUS_RUNNING: &str = "RUNNING";
pub const STEP_STATUS_COMPLETED: &str = "COMPLETED";
pub const STEP_STATUS_CANCELLED: &str = "CANCELLED";
pub const STEP_STATUS_FAILED: &str = "FAILED";
pub const STEP_STATUS_CANCEL_PENDING: &str = "CANCEL_PENDING";

#[derive(Debug, Clone)]
pub struct ClusterData {
    pub id: String,
    pub name: String,
    pub status: String,
    pub creation_date_time: DateTime<Utc>,
    pub ready_date_time: Option<DateTime<Utc>>,
    pub end_date_time: Option<DateTime<Utc>>,
    pub termination_protected: bool,
    pub visible_to_all_users: bool,
    pub log_uri: Option<String>,
    pub release_label: Option<String>,
    pub applications: Vec<ApplicationData>,
    pub tags: HashMap<String, String>,
    pub service_role: Option<String>,
    pub job_flow_role: Option<String>,
    pub auto_scaling_role: Option<String>,
    pub scale_down_behavior: Option<String>,
    pub security_configuration: Option<String>,
    pub step_concurrency_level: Option<i32>,
    pub auto_termination_policy: Option<AutoTerminationPolicyData>,
    pub managed_scaling_policy: Option<ManagedScalingPolicyData>,
    pub cluster_arn: String,
    pub normalized_instance_hours: Option<i32>,
    pub master_public_dns_name: Option<String>,
    pub instance_groups: Vec<InstanceGroupData>,
    pub instance_fleets: Vec<InstanceFleetData>,
    pub bootstrap_actions: Vec<BootstrapActionData>,
}

#[derive(Debug, Clone)]
pub struct ApplicationData {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AutoTerminationPolicyData {
    pub idle_timeout: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ManagedScalingPolicyData {
    pub compute_limits: Option<ComputeLimitsData>,
    pub scaling_strategy: Option<String>,
    pub utilization_performance_index: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ComputeLimitsData {
    pub minimum_capacity_units: i32,
    pub maximum_capacity_units: i32,
    pub maximum_on_demand_capacity_units: Option<i32>,
    pub maximum_core_capacity_units: Option<i32>,
    pub unit_type: String,
}

#[derive(Debug, Clone)]
pub struct InstanceGroupData {
    pub id: String,
    pub name: Option<String>,
    pub instance_type: String,
    pub instance_role: String,
    pub instance_count: i32,
    pub market: Option<String>,
    pub bid_price: Option<String>,
    pub status: String,
    pub running_instance_count: Option<i32>,
    pub auto_scaling_policy: Option<AutoScalingPolicyData>,
}

#[derive(Debug, Clone)]
pub struct AutoScalingPolicyData {
    pub status: String,
    pub constraints: Option<ScalingConstraintsData>,
    pub rules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ScalingConstraintsData {
    pub min_capacity: i32,
    pub max_capacity: i32,
}

#[derive(Debug, Clone)]
pub struct InstanceFleetData {
    pub id: String,
    pub name: Option<String>,
    pub instance_fleet_type: String,
    pub target_on_demand_capacity: Option<i32>,
    pub target_spot_capacity: Option<i32>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct BootstrapActionData {
    pub name: String,
    pub script_path: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StepData {
    pub id: String,
    pub name: String,
    pub cluster_id: String,
    pub status: String,
    pub creation_date_time: DateTime<Utc>,
    pub start_date_time: Option<DateTime<Utc>>,
    pub end_date_time: Option<DateTime<Utc>>,
    pub jar: String,
    pub args: Option<Vec<String>>,
    pub main_class: Option<String>,
    pub action_on_failure: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityConfigurationData {
    pub name: String,
    pub security_configuration: String,
    pub creation_date_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BlockPublicAccessConfigurationData {
    pub block_public_security_group_rules: bool,
    pub permitted_ranges: Vec<PortRangeData>,
    pub creation_date_time: DateTime<Utc>,
    pub created_by_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PortRangeData {
    pub min_range: i32,
    pub max_range: Option<i32>,
}

// ── EMR Studio types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StudioData {
    pub studio_id: String,
    pub name: String,
    pub description: Option<String>,
    pub auth_mode: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub service_role: Option<String>,
    pub user_role: Option<String>,
    pub workspace_security_group_id: Option<String>,
    pub engine_security_group_id: Option<String>,
    pub studio_arn: String,
    pub url: String,
    pub creation_time: DateTime<Utc>,
    pub default_s3_location: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SessionMappingData {
    pub studio_id: String,
    pub identity_id: String,
    pub identity_name: String,
    pub identity_type: String,
    pub session_policy_arn: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NotebookExecutionData {
    pub notebook_execution_id: String,
    pub editor_id: String,
    pub execution_engine: serde_json::Value,
    pub notebook_execution_name: Option<String>,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub notebook_s3_location: Option<serde_json::Value>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PersistentAppUiData {
    pub persistent_app_ui_id: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
}
