use std::collections::HashMap;

use rand::Rng;

use crate::types::{
    AsgInstance, AutoScalingGroup, LaunchConfiguration, LifecycleHook, NotificationConfig,
    ScalingActivity, ScalingPolicy, ScheduledAction, Tag, WarmPool,
};

/// In-memory state for the Auto Scaling service.
#[derive(Debug, Default)]
pub struct AutoScalingState {
    pub groups: HashMap<String, AutoScalingGroup>,
    pub launch_configs: HashMap<String, LaunchConfiguration>,
    pub policies: HashMap<String, ScalingPolicy>,
    pub lifecycle_hooks: HashMap<String, LifecycleHook>,
    pub scheduled_actions: HashMap<String, ScheduledAction>,
    pub activities: Vec<ScalingActivity>,
}

/// Error type for Auto Scaling operations.
#[derive(Debug, thiserror::Error)]
pub enum AutoScalingError {
    #[error("{0} already exists.")]
    AlreadyExists(String),

    #[error("No group found with the name {0}.")]
    NotFound(String),

    #[error("No launch configuration found with the name {0}.")]
    LaunchConfigNotFound(String),
}

impl AutoScalingState {
    // --- Launch Configurations ---

    pub fn create_launch_configuration(
        &mut self,
        name: String,
        image_id: Option<String>,
        instance_type: Option<String>,
        key_name: Option<String>,
        iam_instance_profile: Option<String>,
        user_data: Option<String>,
        security_groups: Vec<String>,
        ebs_optimized: bool,
        associate_public_ip_address: Option<bool>,
        spot_price: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<(), AutoScalingError> {
        if self.launch_configs.contains_key(&name) {
            return Err(AutoScalingError::AlreadyExists(format!(
                "Launch configuration '{name}'"
            )));
        }
        let arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:launchConfiguration:{uuid}:launchConfigurationName/{name}",
            uuid = uuid::Uuid::new_v4()
        );
        let created_time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        self.launch_configs.insert(
            name.clone(),
            LaunchConfiguration {
                name,
                arn,
                image_id,
                instance_type,
                key_name,
                iam_instance_profile,
                user_data,
                security_groups,
                ebs_optimized,
                associate_public_ip_address,
                spot_price,
                created_time,
            },
        );
        Ok(())
    }

    pub fn delete_launch_configuration(&mut self, name: &str) -> Result<(), AutoScalingError> {
        if self.launch_configs.remove(name).is_none() {
            return Err(AutoScalingError::LaunchConfigNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn describe_launch_configurations(
        &self,
        names: Option<&[String]>,
    ) -> Vec<&LaunchConfiguration> {
        match names {
            Some(ns) if !ns.is_empty() => ns
                .iter()
                .filter_map(|n| self.launch_configs.get(n))
                .collect(),
            _ => {
                let mut lcs: Vec<&LaunchConfiguration> = self.launch_configs.values().collect();
                lcs.sort_by(|a, b| a.name.cmp(&b.name));
                lcs
            }
        }
    }

    // --- Auto Scaling Groups ---

    pub fn create_group(
        &mut self,
        name: String,
        min_size: i32,
        max_size: i32,
        desired_capacity: Option<i32>,
        launch_configuration_name: Option<String>,
        vpc_zone_identifier: Option<String>,
        availability_zones: Vec<String>,
        health_check_type: Option<String>,
        health_check_grace_period: Option<i32>,
        default_cooldown: Option<i32>,
        tags: Vec<Tag>,
        termination_policies: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> Result<(), AutoScalingError> {
        if self.groups.contains_key(&name) {
            return Err(AutoScalingError::AlreadyExists(format!(
                "AutoScalingGroup '{name}'"
            )));
        }
        let desired = desired_capacity.unwrap_or(min_size);
        let arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:autoScalingGroup:{uuid}:autoScalingGroupName/{name}",
            uuid = uuid::Uuid::new_v4()
        );
        let created_time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let azs = if availability_zones.is_empty() {
            vec![format!("{region}a")]
        } else {
            availability_zones.clone()
        };
        let instances =
            self.spawn_instances(&name, desired, &azs, launch_configuration_name.as_deref());
        let activity = ScalingActivity {
            activity_id: uuid::Uuid::new_v4().to_string(),
            group_name: name.clone(),
            cause: format!(
                "At {} a user request created an AutoScalingGroup",
                &created_time
            ),
            start_time: created_time.clone(),
            end_time: Some(created_time.clone()),
            status_code: "Successful".to_string(),
            description: format!(
                "Launching a new EC2 instance. Launching {} instance(s).",
                desired
            ),
        };
        self.activities.push(activity);
        self.groups.insert(
            name.clone(),
            AutoScalingGroup {
                name,
                arn,
                min_size,
                max_size,
                desired_capacity: desired,
                launch_configuration_name,
                vpc_zone_identifier,
                availability_zones,
                health_check_type: health_check_type.unwrap_or_else(|| "EC2".to_string()),
                health_check_grace_period: health_check_grace_period.unwrap_or(0),
                default_cooldown: default_cooldown.unwrap_or(300),
                tags,
                suspended_processes: Vec::new(),
                termination_policies,
                enabled_metrics: Vec::new(),
                created_time,
                status: None,
                notification_configurations: Vec::new(),
                attached_load_balancers: Vec::new(),
                attached_target_groups: Vec::new(),
                warm_pool: None,
                instances,
            },
        );
        Ok(())
    }

    pub fn update_group(
        &mut self,
        name: &str,
        min_size: Option<i32>,
        max_size: Option<i32>,
        desired_capacity: Option<i32>,
        launch_configuration_name: Option<String>,
        vpc_zone_identifier: Option<String>,
        availability_zones: Option<Vec<String>>,
        health_check_type: Option<String>,
        health_check_grace_period: Option<i32>,
        default_cooldown: Option<i32>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(name)
            .ok_or_else(|| AutoScalingError::NotFound(name.to_string()))?;
        if let Some(v) = min_size {
            group.min_size = v;
        }
        if let Some(v) = max_size {
            group.max_size = v;
        }
        if let Some(v) = desired_capacity {
            group.desired_capacity = v;
        }
        if let Some(v) = launch_configuration_name {
            group.launch_configuration_name = Some(v);
        }
        if let Some(v) = vpc_zone_identifier {
            group.vpc_zone_identifier = Some(v);
        }
        if let Some(v) = availability_zones {
            group.availability_zones = v;
        }
        if let Some(v) = health_check_type {
            group.health_check_type = v;
        }
        if let Some(v) = health_check_grace_period {
            group.health_check_grace_period = v;
        }
        if let Some(v) = default_cooldown {
            group.default_cooldown = v;
        }
        Ok(())
    }

    pub fn delete_group(&mut self, name: &str) -> Result<(), AutoScalingError> {
        if self.groups.remove(name).is_none() {
            return Err(AutoScalingError::NotFound(name.to_string()));
        }
        // Remove associated resources
        self.policies.retain(|_, p| p.group_name != name);
        self.lifecycle_hooks.retain(|_, h| h.group_name != name);
        self.scheduled_actions.retain(|_, a| a.group_name != name);
        Ok(())
    }

    pub fn describe_groups(&self, names: Option<&[String]>) -> Vec<&AutoScalingGroup> {
        match names {
            Some(ns) if !ns.is_empty() => ns.iter().filter_map(|n| self.groups.get(n)).collect(),
            _ => {
                let mut groups: Vec<&AutoScalingGroup> = self.groups.values().collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                groups
            }
        }
    }

    pub fn set_desired_capacity(
        &mut self,
        name: &str,
        desired: i32,
    ) -> Result<(), AutoScalingError> {
        {
            let group = self
                .groups
                .get_mut(name)
                .ok_or_else(|| AutoScalingError::NotFound(name.to_string()))?;
            group.desired_capacity = desired;
        }
        self.reconcile_instances(name);
        Ok(())
    }

    // --- Scaling Policies ---

    pub fn put_scaling_policy(
        &mut self,
        group_name: &str,
        policy_name: &str,
        policy_type: Option<String>,
        adjustment_type: Option<String>,
        scaling_adjustment: Option<i32>,
        cooldown: Option<i32>,
        min_adjustment_magnitude: Option<i32>,
        account_id: &str,
        region: &str,
    ) -> Result<String, AutoScalingError> {
        if !self.groups.contains_key(group_name) {
            return Err(AutoScalingError::NotFound(group_name.to_string()));
        }
        let key = format!("{group_name}/{policy_name}");
        let arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:scalingPolicy:{uuid}:autoScalingGroupName/{group_name}:policyName/{policy_name}",
            uuid = uuid::Uuid::new_v4()
        );
        let arn_clone = arn.clone();
        self.policies.insert(
            key,
            ScalingPolicy {
                name: policy_name.to_string(),
                arn,
                group_name: group_name.to_string(),
                policy_type,
                adjustment_type,
                scaling_adjustment,
                cooldown,
                min_adjustment_magnitude,
            },
        );
        Ok(arn_clone)
    }

    pub fn delete_policy(
        &mut self,
        group_name: Option<&str>,
        policy_name: &str,
    ) -> Result<(), AutoScalingError> {
        let key = if let Some(gn) = group_name {
            format!("{gn}/{policy_name}")
        } else {
            // Try to find by policy name alone
            let found = self
                .policies
                .iter()
                .find(|(_, p)| p.name == policy_name || p.arn == policy_name)
                .map(|(k, _)| k.clone());
            match found {
                Some(k) => k,
                None => return Ok(()), // Idempotent
            }
        };
        self.policies.remove(&key);
        Ok(())
    }

    pub fn describe_policies(
        &self,
        group_name: Option<&str>,
        policy_names: Option<&[String]>,
    ) -> Vec<&ScalingPolicy> {
        let mut result: Vec<&ScalingPolicy> = self
            .policies
            .values()
            .filter(|p| {
                let matches_group = group_name.is_none_or(|gn| p.group_name == gn);
                let matches_policy = policy_names.is_none_or(|pns| {
                    pns.is_empty() || pns.iter().any(|pn| p.name == *pn || p.arn == *pn)
                });
                matches_group && matches_policy
            })
            .collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));
        result
    }

    // --- Tags ---

    pub fn create_or_update_tags(&mut self, tags: Vec<Tag>) -> Result<(), AutoScalingError> {
        for tag in tags {
            let group = self
                .groups
                .get_mut(&tag.resource_id)
                .ok_or_else(|| AutoScalingError::NotFound(tag.resource_id.clone()))?;
            if let Some(existing) = group.tags.iter_mut().find(|t| t.key == tag.key) {
                existing.value = tag.value;
                existing.propagate_at_launch = tag.propagate_at_launch;
            } else {
                group.tags.push(tag);
            }
        }
        Ok(())
    }

    pub fn delete_tags(&mut self, tags: Vec<Tag>) {
        for tag in &tags {
            if let Some(group) = self.groups.get_mut(&tag.resource_id) {
                group.tags.retain(|t| t.key != tag.key);
            }
        }
    }

    pub fn describe_tags(&self, group_name: Option<&str>) -> Vec<(&Tag, &str)> {
        let mut result = Vec::new();
        for group in self.groups.values() {
            if group_name.is_none_or(|gn| group.name == gn) {
                for tag in &group.tags {
                    result.push((tag, group.name.as_str()));
                }
            }
        }
        result
    }

    // --- Lifecycle Hooks ---

    pub fn put_lifecycle_hook(
        &mut self,
        group_name: &str,
        hook_name: &str,
        lifecycle_transition: Option<String>,
        notification_target_arn: Option<String>,
        role_arn: Option<String>,
        notification_metadata: Option<String>,
        heartbeat_timeout: Option<i32>,
        default_result: Option<String>,
    ) -> Result<(), AutoScalingError> {
        if !self.groups.contains_key(group_name) {
            return Err(AutoScalingError::NotFound(group_name.to_string()));
        }
        let key = format!("{group_name}/{hook_name}");
        self.lifecycle_hooks.insert(
            key,
            LifecycleHook {
                name: hook_name.to_string(),
                group_name: group_name.to_string(),
                lifecycle_transition,
                notification_target_arn,
                role_arn,
                notification_metadata,
                heartbeat_timeout,
                default_result,
            },
        );
        Ok(())
    }

    pub fn delete_lifecycle_hook(
        &mut self,
        group_name: &str,
        hook_name: &str,
    ) -> Result<(), AutoScalingError> {
        let key = format!("{group_name}/{hook_name}");
        self.lifecycle_hooks.remove(&key);
        Ok(())
    }

    pub fn describe_lifecycle_hooks(
        &self,
        group_name: &str,
        hook_names: Option<&[String]>,
    ) -> Vec<&LifecycleHook> {
        self.lifecycle_hooks
            .values()
            .filter(|h| {
                h.group_name == group_name
                    && hook_names.is_none_or(|hns| hns.is_empty() || hns.contains(&h.name))
            })
            .collect()
    }

    // --- Scheduled Actions ---

    pub fn put_scheduled_action(
        &mut self,
        group_name: &str,
        action_name: &str,
        desired_capacity: Option<i32>,
        min_size: Option<i32>,
        max_size: Option<i32>,
        start_time: Option<String>,
        end_time: Option<String>,
        recurrence: Option<String>,
        time_zone: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<(), AutoScalingError> {
        if !self.groups.contains_key(group_name) {
            return Err(AutoScalingError::NotFound(group_name.to_string()));
        }
        let key = format!("{group_name}/{action_name}");
        let arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:scheduledUpdateGroupAction:{uuid}:autoScalingGroupName/{group_name}:scheduledActionName/{action_name}",
            uuid = uuid::Uuid::new_v4()
        );
        self.scheduled_actions.insert(
            key,
            ScheduledAction {
                name: action_name.to_string(),
                arn,
                group_name: group_name.to_string(),
                desired_capacity,
                min_size,
                max_size,
                start_time,
                end_time,
                recurrence,
                time_zone,
            },
        );
        Ok(())
    }

    pub fn delete_scheduled_action(
        &mut self,
        group_name: &str,
        action_name: &str,
    ) -> Result<(), AutoScalingError> {
        let key = format!("{group_name}/{action_name}");
        self.scheduled_actions.remove(&key);
        Ok(())
    }

    pub fn describe_scheduled_actions(&self, group_name: Option<&str>) -> Vec<&ScheduledAction> {
        let mut result: Vec<&ScheduledAction> = self
            .scheduled_actions
            .values()
            .filter(|a| group_name.is_none_or(|gn| a.group_name == gn))
            .collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));
        result
    }

    // --- Process suspension ---

    pub fn suspend_processes(
        &mut self,
        group_name: &str,
        processes: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        for p in processes {
            if !group.suspended_processes.contains(&p) {
                group.suspended_processes.push(p);
            }
        }
        Ok(())
    }

    pub fn resume_processes(
        &mut self,
        group_name: &str,
        processes: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        if processes.is_empty() {
            group.suspended_processes.clear();
        } else {
            group.suspended_processes.retain(|p| !processes.contains(p));
        }
        Ok(())
    }

    // --- Notifications ---

    pub fn put_notification_configuration(
        &mut self,
        group_name: &str,
        topic_arn: &str,
        notification_types: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        // Remove old notifications for this topic
        group
            .notification_configurations
            .retain(|n| n.topic_arn != topic_arn);
        for nt in notification_types {
            group.notification_configurations.push(NotificationConfig {
                group_name: group_name.to_string(),
                notification_type: nt,
                topic_arn: topic_arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn delete_notification_configuration(
        &mut self,
        group_name: &str,
        topic_arn: &str,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        group
            .notification_configurations
            .retain(|n| n.topic_arn != topic_arn);
        Ok(())
    }

    pub fn describe_notification_configurations(
        &self,
        group_names: Option<&[String]>,
    ) -> Vec<&NotificationConfig> {
        let mut result: Vec<&NotificationConfig> = self
            .groups
            .values()
            .filter(|g| group_names.is_none_or(|gns| gns.is_empty() || gns.contains(&g.name)))
            .flat_map(|g| g.notification_configurations.iter())
            .collect();
        result.sort_by(|a, b| a.group_name.cmp(&b.group_name));
        result
    }

    // --- Metrics collection ---

    pub fn enable_metrics_collection(
        &mut self,
        group_name: &str,
        metrics: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        if metrics.is_empty() {
            // Enable all standard metrics
            let all_metrics = vec![
                "GroupMinSize",
                "GroupMaxSize",
                "GroupDesiredCapacity",
                "GroupInServiceCapacity",
                "GroupTotalCapacity",
            ];
            for m in all_metrics {
                let ms = m.to_string();
                if !group.enabled_metrics.contains(&ms) {
                    group.enabled_metrics.push(ms);
                }
            }
        } else {
            for m in metrics {
                if !group.enabled_metrics.contains(&m) {
                    group.enabled_metrics.push(m);
                }
            }
        }
        Ok(())
    }

    pub fn disable_metrics_collection(
        &mut self,
        group_name: &str,
        metrics: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        if metrics.is_empty() {
            group.enabled_metrics.clear();
        } else {
            group.enabled_metrics.retain(|m| !metrics.contains(m));
        }
        Ok(())
    }

    // --- Account limits ---

    pub fn describe_account_limits(&self) -> (i32, i32, i32, i32) {
        let num_groups = self.groups.len() as i32;
        let num_lcs = self.launch_configs.len() as i32;
        (200, 200, num_groups, num_lcs)
    }

    // --- Load balancer attachments ---

    pub fn attach_load_balancers(
        &mut self,
        group_name: &str,
        lb_names: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        for lb in lb_names {
            if !group.attached_load_balancers.contains(&lb) {
                group.attached_load_balancers.push(lb);
            }
        }
        Ok(())
    }

    pub fn detach_load_balancers(
        &mut self,
        group_name: &str,
        lb_names: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        group
            .attached_load_balancers
            .retain(|lb| !lb_names.contains(lb));
        Ok(())
    }

    pub fn describe_load_balancers(
        &self,
        group_name: &str,
    ) -> Result<Vec<String>, AutoScalingError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        Ok(group.attached_load_balancers.clone())
    }

    // --- Target group attachments ---

    pub fn attach_target_groups(
        &mut self,
        group_name: &str,
        tg_arns: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        for tg in tg_arns {
            if !group.attached_target_groups.contains(&tg) {
                group.attached_target_groups.push(tg);
            }
        }
        Ok(())
    }

    pub fn detach_target_groups(
        &mut self,
        group_name: &str,
        tg_arns: Vec<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        group
            .attached_target_groups
            .retain(|tg| !tg_arns.contains(tg));
        Ok(())
    }

    pub fn describe_target_groups(
        &self,
        group_name: &str,
    ) -> Result<Vec<String>, AutoScalingError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        Ok(group.attached_target_groups.clone())
    }

    // --- Warm pool ---

    pub fn put_warm_pool(
        &mut self,
        group_name: &str,
        min_size: Option<i32>,
        max_group_prepared_capacity: Option<i32>,
        pool_state: Option<String>,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        group.warm_pool = Some(WarmPool {
            min_size,
            max_group_prepared_capacity,
            pool_state,
        });
        Ok(())
    }

    pub fn delete_warm_pool(&mut self, group_name: &str) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        group.warm_pool = None;
        Ok(())
    }

    pub fn describe_warm_pool(
        &self,
        group_name: &str,
    ) -> Result<Option<&WarmPool>, AutoScalingError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        Ok(group.warm_pool.as_ref())
    }

    // --- Instance health / protection (mock: no-op) ---

    pub fn set_instance_health(&mut self, instance_id: &str, health_status: &str) {
        for group in self.groups.values_mut() {
            for inst in &mut group.instances {
                if inst.instance_id == instance_id {
                    inst.health_status = health_status.to_string();
                    return;
                }
            }
        }
    }

    pub fn set_instance_protection(
        &mut self,
        group_name: &str,
        instance_ids: &[String],
        protected: bool,
    ) -> Result<(), AutoScalingError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| AutoScalingError::NotFound(group_name.to_string()))?;
        for inst in &mut group.instances {
            if instance_ids.contains(&inst.instance_id) {
                inst.protected_from_scale_in = protected;
            }
        }
        Ok(())
    }

    // --- Instance tracking ---

    /// Spawn synthetic instances for an ASG (round-robin across AZs).
    fn spawn_instances(
        &mut self,
        group_name: &str,
        count: i32,
        availability_zones: &[String],
        launch_configuration_name: Option<&str>,
    ) -> Vec<AsgInstance> {
        let mut instances = Vec::new();
        for i in 0..count {
            let az_idx = (i as usize) % availability_zones.len().max(1);
            let az = availability_zones
                .get(az_idx)
                .cloned()
                .unwrap_or_else(|| "us-east-1a".to_string());
            let instance_id = format!("i-{:017x}", rand::rng().random::<u64>());
            instances.push(AsgInstance {
                instance_id,
                group_name: group_name.to_string(),
                availability_zone: az,
                lifecycle_state: "InService".to_string(),
                health_status: "Healthy".to_string(),
                launch_configuration_name: launch_configuration_name.map(|s| s.to_string()),
                protected_from_scale_in: false,
            });
        }
        instances
    }

    /// Describe ASG instances, optionally filtering by instance IDs.
    pub fn describe_auto_scaling_instances(
        &self,
        instance_ids: Option<&[String]>,
    ) -> Vec<&AsgInstance> {
        let mut result: Vec<&AsgInstance> = self
            .groups
            .values()
            .flat_map(|g| g.instances.iter())
            .filter(|inst| {
                instance_ids.is_none_or(|ids| ids.is_empty() || ids.contains(&inst.instance_id))
            })
            .collect();
        result.sort_by(|a, b| a.instance_id.cmp(&b.instance_id));
        result
    }

    /// Describe scaling activities, optionally filtered by group name.
    pub fn describe_scaling_activities(
        &self,
        group_name: Option<&str>,
    ) -> Result<Vec<&ScalingActivity>, AutoScalingError> {
        if let Some(name) = group_name {
            if !self.groups.contains_key(name) {
                return Err(AutoScalingError::NotFound(name.to_string()));
            }
        }
        let result: Vec<&ScalingActivity> = self
            .activities
            .iter()
            .filter(|a| group_name.is_none_or(|gn| a.group_name == gn))
            .collect();
        Ok(result)
    }

    /// Terminate an instance in an ASG. Returns the activity.
    pub fn terminate_instance(
        &mut self,
        instance_id: &str,
        should_decrement: bool,
    ) -> Result<ScalingActivity, AutoScalingError> {
        // Find which group has this instance
        let group_name = self
            .groups
            .values()
            .find(|g| g.instances.iter().any(|i| i.instance_id == instance_id))
            .map(|g| g.name.clone());
        let group_name = match group_name {
            Some(n) => n,
            None => {
                return Err(AutoScalingError::NotFound(format!(
                    "Instance '{instance_id}'"
                )));
            }
        };
        let group = self.groups.get_mut(&group_name).unwrap();
        group.instances.retain(|i| i.instance_id != instance_id);
        if should_decrement && group.desired_capacity > group.min_size {
            group.desired_capacity -= 1;
        }
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let activity = ScalingActivity {
            activity_id: uuid::Uuid::new_v4().to_string(),
            group_name: group_name.clone(),
            cause: format!(
                "At {} instance {instance_id} was taken out of service in response to a user request.",
                &now
            ),
            start_time: now.clone(),
            end_time: Some(now),
            status_code: "Successful".to_string(),
            description: format!("Terminating EC2 instance: {instance_id}"),
        };
        self.activities.push(activity.clone());
        Ok(activity)
    }

    /// Reconcile instances when desired capacity changes (e.g. from set_desired_capacity or update).
    fn reconcile_instances(&mut self, group_name: &str) {
        let group = match self.groups.get_mut(group_name) {
            Some(g) => g,
            None => return,
        };
        let current = group.instances.len() as i32;
        let desired = group.desired_capacity;
        if current < desired {
            let azs = if group.availability_zones.is_empty() {
                vec!["us-east-1a".to_string()]
            } else {
                group.availability_zones.clone()
            };
            let lcn = group.launch_configuration_name.clone();
            let gn = group.name.clone();
            let diff = desired - current;
            for i in 0..diff {
                let az_idx = ((current + i) as usize) % azs.len().max(1);
                let az = azs
                    .get(az_idx)
                    .cloned()
                    .unwrap_or_else(|| "us-east-1a".to_string());
                let instance_id = format!("i-{:017x}", rand::rng().random::<u64>());
                group.instances.push(AsgInstance {
                    instance_id,
                    group_name: gn.clone(),
                    availability_zone: az,
                    lifecycle_state: "InService".to_string(),
                    health_status: "Healthy".to_string(),
                    launch_configuration_name: lcn.clone(),
                    protected_from_scale_in: false,
                });
            }
        } else if current > desired {
            let excess = (current - desired) as usize;
            for _ in 0..excess {
                group.instances.pop();
            }
        }
    }
}
