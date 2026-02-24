//! Serde-compatible view types for Auto Scaling state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AutoScalingService;
use crate::state::AutoScalingState;
use crate::types::{
    AsgInstance, AutoScalingGroup, LaunchConfiguration, LifecycleHook, NotificationConfig,
    ScalingActivity, ScalingPolicy, ScheduledAction, Tag, WarmPool,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutoScalingStateView {
    #[serde(default)]
    pub groups: HashMap<String, AutoScalingGroupView>,
    #[serde(default)]
    pub launch_configs: HashMap<String, LaunchConfigurationView>,
    #[serde(default)]
    pub policies: HashMap<String, ScalingPolicyView>,
    #[serde(default)]
    pub lifecycle_hooks: HashMap<String, LifecycleHookView>,
    #[serde(default)]
    pub scheduled_actions: HashMap<String, ScheduledActionView>,
    #[serde(default)]
    pub activities: Vec<ScalingActivityView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingGroupView {
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
    pub tags: Vec<TagView>,
    pub suspended_processes: Vec<String>,
    pub termination_policies: Vec<String>,
    pub enabled_metrics: Vec<String>,
    pub created_time: String,
    pub status: Option<String>,
    pub notification_configurations: Vec<NotificationConfigView>,
    #[serde(default)]
    pub attached_load_balancers: Vec<String>,
    #[serde(default)]
    pub attached_target_groups: Vec<String>,
    #[serde(default)]
    pub warm_pool: Option<WarmPoolView>,
    #[serde(default)]
    pub instances: Vec<AsgInstanceView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmPoolView {
    pub min_size: Option<i32>,
    pub max_group_prepared_capacity: Option<i32>,
    pub pool_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsgInstanceView {
    pub instance_id: String,
    pub group_name: String,
    pub availability_zone: String,
    pub lifecycle_state: String,
    pub health_status: String,
    pub launch_configuration_name: Option<String>,
    #[serde(default)]
    pub protected_from_scale_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingActivityView {
    pub activity_id: String,
    pub group_name: String,
    pub cause: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status_code: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
    pub resource_id: String,
    pub resource_type: String,
    pub propagate_at_launch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfigView {
    pub group_name: String,
    pub notification_type: String,
    pub topic_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchConfigurationView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicyView {
    pub name: String,
    pub arn: String,
    pub group_name: String,
    pub policy_type: Option<String>,
    pub adjustment_type: Option<String>,
    pub scaling_adjustment: Option<i32>,
    pub cooldown: Option<i32>,
    pub min_adjustment_magnitude: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHookView {
    pub name: String,
    pub group_name: String,
    pub lifecycle_transition: Option<String>,
    pub notification_target_arn: Option<String>,
    pub role_arn: Option<String>,
    pub notification_metadata: Option<String>,
    pub heartbeat_timeout: Option<i32>,
    pub default_result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledActionView {
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

// --- From conversions ---

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        Self {
            key: t.key.clone(),
            value: t.value.clone(),
            resource_id: t.resource_id.clone(),
            resource_type: t.resource_type.clone(),
            propagate_at_launch: t.propagate_at_launch,
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Self {
            key: v.key,
            value: v.value,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            propagate_at_launch: v.propagate_at_launch,
        }
    }
}

impl From<&NotificationConfig> for NotificationConfigView {
    fn from(n: &NotificationConfig) -> Self {
        Self {
            group_name: n.group_name.clone(),
            notification_type: n.notification_type.clone(),
            topic_arn: n.topic_arn.clone(),
        }
    }
}

impl From<NotificationConfigView> for NotificationConfig {
    fn from(v: NotificationConfigView) -> Self {
        Self {
            group_name: v.group_name,
            notification_type: v.notification_type,
            topic_arn: v.topic_arn,
        }
    }
}

impl From<&WarmPool> for WarmPoolView {
    fn from(w: &WarmPool) -> Self {
        Self {
            min_size: w.min_size,
            max_group_prepared_capacity: w.max_group_prepared_capacity,
            pool_state: w.pool_state.clone(),
        }
    }
}

impl From<WarmPoolView> for WarmPool {
    fn from(v: WarmPoolView) -> Self {
        Self {
            min_size: v.min_size,
            max_group_prepared_capacity: v.max_group_prepared_capacity,
            pool_state: v.pool_state,
        }
    }
}

impl From<&AutoScalingGroup> for AutoScalingGroupView {
    fn from(g: &AutoScalingGroup) -> Self {
        Self {
            name: g.name.clone(),
            arn: g.arn.clone(),
            min_size: g.min_size,
            max_size: g.max_size,
            desired_capacity: g.desired_capacity,
            launch_configuration_name: g.launch_configuration_name.clone(),
            vpc_zone_identifier: g.vpc_zone_identifier.clone(),
            availability_zones: g.availability_zones.clone(),
            health_check_type: g.health_check_type.clone(),
            health_check_grace_period: g.health_check_grace_period,
            default_cooldown: g.default_cooldown,
            tags: g.tags.iter().map(TagView::from).collect(),
            suspended_processes: g.suspended_processes.clone(),
            termination_policies: g.termination_policies.clone(),
            enabled_metrics: g.enabled_metrics.clone(),
            created_time: g.created_time.clone(),
            status: g.status.clone(),
            notification_configurations: g
                .notification_configurations
                .iter()
                .map(NotificationConfigView::from)
                .collect(),
            attached_load_balancers: g.attached_load_balancers.clone(),
            attached_target_groups: g.attached_target_groups.clone(),
            warm_pool: g.warm_pool.as_ref().map(WarmPoolView::from),
            instances: g.instances.iter().map(AsgInstanceView::from).collect(),
        }
    }
}

impl From<AutoScalingGroupView> for AutoScalingGroup {
    fn from(v: AutoScalingGroupView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            min_size: v.min_size,
            max_size: v.max_size,
            desired_capacity: v.desired_capacity,
            launch_configuration_name: v.launch_configuration_name,
            vpc_zone_identifier: v.vpc_zone_identifier,
            availability_zones: v.availability_zones,
            health_check_type: v.health_check_type,
            health_check_grace_period: v.health_check_grace_period,
            default_cooldown: v.default_cooldown,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            suspended_processes: v.suspended_processes,
            termination_policies: v.termination_policies,
            enabled_metrics: v.enabled_metrics,
            created_time: v.created_time,
            status: v.status,
            notification_configurations: v
                .notification_configurations
                .into_iter()
                .map(NotificationConfig::from)
                .collect(),
            attached_load_balancers: v.attached_load_balancers,
            attached_target_groups: v.attached_target_groups,
            warm_pool: v.warm_pool.map(WarmPool::from),
            instances: v.instances.into_iter().map(AsgInstance::from).collect(),
        }
    }
}

impl From<&LaunchConfiguration> for LaunchConfigurationView {
    fn from(lc: &LaunchConfiguration) -> Self {
        Self {
            name: lc.name.clone(),
            arn: lc.arn.clone(),
            image_id: lc.image_id.clone(),
            instance_type: lc.instance_type.clone(),
            key_name: lc.key_name.clone(),
            iam_instance_profile: lc.iam_instance_profile.clone(),
            user_data: lc.user_data.clone(),
            security_groups: lc.security_groups.clone(),
            ebs_optimized: lc.ebs_optimized,
            associate_public_ip_address: lc.associate_public_ip_address,
            spot_price: lc.spot_price.clone(),
            created_time: lc.created_time.clone(),
        }
    }
}

impl From<LaunchConfigurationView> for LaunchConfiguration {
    fn from(v: LaunchConfigurationView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            image_id: v.image_id,
            instance_type: v.instance_type,
            key_name: v.key_name,
            iam_instance_profile: v.iam_instance_profile,
            user_data: v.user_data,
            security_groups: v.security_groups,
            ebs_optimized: v.ebs_optimized,
            associate_public_ip_address: v.associate_public_ip_address,
            spot_price: v.spot_price,
            created_time: v.created_time,
        }
    }
}

impl From<&ScalingPolicy> for ScalingPolicyView {
    fn from(p: &ScalingPolicy) -> Self {
        Self {
            name: p.name.clone(),
            arn: p.arn.clone(),
            group_name: p.group_name.clone(),
            policy_type: p.policy_type.clone(),
            adjustment_type: p.adjustment_type.clone(),
            scaling_adjustment: p.scaling_adjustment,
            cooldown: p.cooldown,
            min_adjustment_magnitude: p.min_adjustment_magnitude,
        }
    }
}

impl From<ScalingPolicyView> for ScalingPolicy {
    fn from(v: ScalingPolicyView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            group_name: v.group_name,
            policy_type: v.policy_type,
            adjustment_type: v.adjustment_type,
            scaling_adjustment: v.scaling_adjustment,
            cooldown: v.cooldown,
            min_adjustment_magnitude: v.min_adjustment_magnitude,
        }
    }
}

impl From<&LifecycleHook> for LifecycleHookView {
    fn from(h: &LifecycleHook) -> Self {
        Self {
            name: h.name.clone(),
            group_name: h.group_name.clone(),
            lifecycle_transition: h.lifecycle_transition.clone(),
            notification_target_arn: h.notification_target_arn.clone(),
            role_arn: h.role_arn.clone(),
            notification_metadata: h.notification_metadata.clone(),
            heartbeat_timeout: h.heartbeat_timeout,
            default_result: h.default_result.clone(),
        }
    }
}

impl From<LifecycleHookView> for LifecycleHook {
    fn from(v: LifecycleHookView) -> Self {
        Self {
            name: v.name,
            group_name: v.group_name,
            lifecycle_transition: v.lifecycle_transition,
            notification_target_arn: v.notification_target_arn,
            role_arn: v.role_arn,
            notification_metadata: v.notification_metadata,
            heartbeat_timeout: v.heartbeat_timeout,
            default_result: v.default_result,
        }
    }
}

impl From<&ScheduledAction> for ScheduledActionView {
    fn from(a: &ScheduledAction) -> Self {
        Self {
            name: a.name.clone(),
            arn: a.arn.clone(),
            group_name: a.group_name.clone(),
            desired_capacity: a.desired_capacity,
            min_size: a.min_size,
            max_size: a.max_size,
            start_time: a.start_time.clone(),
            end_time: a.end_time.clone(),
            recurrence: a.recurrence.clone(),
            time_zone: a.time_zone.clone(),
        }
    }
}

impl From<ScheduledActionView> for ScheduledAction {
    fn from(v: ScheduledActionView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            group_name: v.group_name,
            desired_capacity: v.desired_capacity,
            min_size: v.min_size,
            max_size: v.max_size,
            start_time: v.start_time,
            end_time: v.end_time,
            recurrence: v.recurrence,
            time_zone: v.time_zone,
        }
    }
}

impl From<&AsgInstance> for AsgInstanceView {
    fn from(i: &AsgInstance) -> Self {
        Self {
            instance_id: i.instance_id.clone(),
            group_name: i.group_name.clone(),
            availability_zone: i.availability_zone.clone(),
            lifecycle_state: i.lifecycle_state.clone(),
            health_status: i.health_status.clone(),
            launch_configuration_name: i.launch_configuration_name.clone(),
            protected_from_scale_in: i.protected_from_scale_in,
        }
    }
}

impl From<AsgInstanceView> for AsgInstance {
    fn from(v: AsgInstanceView) -> Self {
        Self {
            instance_id: v.instance_id,
            group_name: v.group_name,
            availability_zone: v.availability_zone,
            lifecycle_state: v.lifecycle_state,
            health_status: v.health_status,
            launch_configuration_name: v.launch_configuration_name,
            protected_from_scale_in: v.protected_from_scale_in,
        }
    }
}

impl From<&ScalingActivity> for ScalingActivityView {
    fn from(a: &ScalingActivity) -> Self {
        Self {
            activity_id: a.activity_id.clone(),
            group_name: a.group_name.clone(),
            cause: a.cause.clone(),
            start_time: a.start_time.clone(),
            end_time: a.end_time.clone(),
            status_code: a.status_code.clone(),
            description: a.description.clone(),
        }
    }
}

impl From<ScalingActivityView> for ScalingActivity {
    fn from(v: ScalingActivityView) -> Self {
        Self {
            activity_id: v.activity_id,
            group_name: v.group_name,
            cause: v.cause,
            start_time: v.start_time,
            end_time: v.end_time,
            status_code: v.status_code,
            description: v.description,
        }
    }
}

impl From<&AutoScalingState> for AutoScalingStateView {
    fn from(s: &AutoScalingState) -> Self {
        Self {
            groups: s
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), AutoScalingGroupView::from(v)))
                .collect(),
            launch_configs: s
                .launch_configs
                .iter()
                .map(|(k, v)| (k.clone(), LaunchConfigurationView::from(v)))
                .collect(),
            policies: s
                .policies
                .iter()
                .map(|(k, v)| (k.clone(), ScalingPolicyView::from(v)))
                .collect(),
            lifecycle_hooks: s
                .lifecycle_hooks
                .iter()
                .map(|(k, v)| (k.clone(), LifecycleHookView::from(v)))
                .collect(),
            scheduled_actions: s
                .scheduled_actions
                .iter()
                .map(|(k, v)| (k.clone(), ScheduledActionView::from(v)))
                .collect(),
            activities: s.activities.iter().map(ScalingActivityView::from).collect(),
        }
    }
}

impl From<AutoScalingStateView> for AutoScalingState {
    fn from(v: AutoScalingStateView) -> Self {
        Self {
            groups: v
                .groups
                .into_iter()
                .map(|(k, gv)| (k, AutoScalingGroup::from(gv)))
                .collect(),
            launch_configs: v
                .launch_configs
                .into_iter()
                .map(|(k, lv)| (k, LaunchConfiguration::from(lv)))
                .collect(),
            policies: v
                .policies
                .into_iter()
                .map(|(k, pv)| (k, ScalingPolicy::from(pv)))
                .collect(),
            lifecycle_hooks: v
                .lifecycle_hooks
                .into_iter()
                .map(|(k, hv)| (k, LifecycleHook::from(hv)))
                .collect(),
            scheduled_actions: v
                .scheduled_actions
                .into_iter()
                .map(|(k, av)| (k, ScheduledAction::from(av)))
                .collect(),
            activities: v
                .activities
                .into_iter()
                .map(ScalingActivity::from)
                .collect(),
        }
    }
}

impl StatefulService for AutoScalingService {
    type StateView = AutoScalingStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AutoScalingStateView::from(&*guard)
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
            *guard = AutoScalingState::from(view);
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
            for (k, v) in view.groups {
                guard.groups.insert(k, AutoScalingGroup::from(v));
            }
            for (k, v) in view.launch_configs {
                guard.launch_configs.insert(k, LaunchConfiguration::from(v));
            }
            for (k, v) in view.policies {
                guard.policies.insert(k, ScalingPolicy::from(v));
            }
            for (k, v) in view.lifecycle_hooks {
                guard.lifecycle_hooks.insert(k, LifecycleHook::from(v));
            }
            for (k, v) in view.scheduled_actions {
                guard.scheduled_actions.insert(k, ScheduledAction::from(v));
            }
            for a in view.activities {
                guard.activities.push(ScalingActivity::from(a));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
