/// A tag on an Auto Scaling resource.
#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub key: String,
    pub value: String,
    pub resource_id: String,
    pub resource_type: String,
    pub propagate_at_launch: bool,
}

/// Represents an Auto Scaling Group.
#[derive(Debug, Clone)]
pub struct AutoScalingGroup {
    pub name: String,
    pub arn: String,
    pub min_size: i32,
    pub max_size: i32,
    pub desired_capacity: i32,
    pub launch_configuration_name: Option<String>,
    pub vpc_zone_identifier: Option<String>,
    pub availability_zones: Vec<String>,
    pub health_check_type: String,
    pub health_check_grace_period: i32,
    pub default_cooldown: i32,
    pub tags: Vec<Tag>,
    pub suspended_processes: Vec<String>,
    pub termination_policies: Vec<String>,
    pub enabled_metrics: Vec<String>,
    pub created_time: String,
    pub status: Option<String>,
    pub notification_configurations: Vec<NotificationConfig>,
    pub attached_load_balancers: Vec<String>,
    pub attached_target_groups: Vec<String>,
    pub warm_pool: Option<WarmPool>,
    pub instances: Vec<AsgInstance>,
}

/// Represents a warm pool configuration for an ASG.
#[derive(Debug, Clone)]
pub struct WarmPool {
    pub min_size: Option<i32>,
    pub max_group_prepared_capacity: Option<i32>,
    pub pool_state: Option<String>,
}

/// Represents a Launch Configuration.
#[derive(Debug, Clone)]
pub struct LaunchConfiguration {
    pub name: String,
    pub arn: String,
    pub image_id: Option<String>,
    pub instance_type: Option<String>,
    pub key_name: Option<String>,
    pub iam_instance_profile: Option<String>,
    pub user_data: Option<String>,
    pub security_groups: Vec<String>,
    pub ebs_optimized: bool,
    pub associate_public_ip_address: Option<bool>,
    pub spot_price: Option<String>,
    pub created_time: String,
}

/// Represents a Scaling Policy.
#[derive(Debug, Clone)]
pub struct ScalingPolicy {
    pub name: String,
    pub arn: String,
    pub group_name: String,
    pub policy_type: Option<String>,
    pub adjustment_type: Option<String>,
    pub scaling_adjustment: Option<i32>,
    pub cooldown: Option<i32>,
    pub min_adjustment_magnitude: Option<i32>,
}

/// Represents a Lifecycle Hook.
#[derive(Debug, Clone)]
pub struct LifecycleHook {
    pub name: String,
    pub group_name: String,
    pub lifecycle_transition: Option<String>,
    pub notification_target_arn: Option<String>,
    pub role_arn: Option<String>,
    pub notification_metadata: Option<String>,
    pub heartbeat_timeout: Option<i32>,
    pub default_result: Option<String>,
}

/// Represents a Scheduled Action.
#[derive(Debug, Clone)]
pub struct ScheduledAction {
    pub name: String,
    pub arn: String,
    pub group_name: String,
    pub desired_capacity: Option<i32>,
    pub min_size: Option<i32>,
    pub max_size: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub recurrence: Option<String>,
    pub time_zone: Option<String>,
}

/// Represents a notification configuration for an ASG.
#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub group_name: String,
    pub notification_type: String,
    pub topic_arn: String,
}

/// Represents a synthetic instance tracked by the mock ASG.
#[derive(Debug, Clone)]
pub struct AsgInstance {
    pub instance_id: String,
    pub group_name: String,
    pub availability_zone: String,
    pub lifecycle_state: String,
    pub health_status: String,
    pub launch_configuration_name: Option<String>,
    pub protected_from_scale_in: bool,
}

/// Represents a scaling activity recorded by the mock.
#[derive(Debug, Clone)]
pub struct ScalingActivity {
    pub activity_id: String,
    pub group_name: String,
    pub cause: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status_code: String,
    pub description: String,
}
