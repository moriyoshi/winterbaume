use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{AutoScalingError, AutoScalingState};
use crate::types::Tag;
use crate::views::AutoScalingStateView;
use crate::wire;

/// Auto Scaling service handler.
pub struct AutoScalingService {
    pub(crate) state: Arc<BackendState<AutoScalingState>>,
    pub(crate) notifier: StateChangeNotifier<AutoScalingStateView>,
}

impl AutoScalingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AutoScalingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AutoScalingService {
    fn service_name(&self) -> &str {
        "autoscaling"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://autoscaling\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "CreateAutoScalingGroup",
    "UpdateAutoScalingGroup",
    "DeleteAutoScalingGroup",
    "CreateLaunchConfiguration",
    "DeleteLaunchConfiguration",
    "SetDesiredCapacity",
    "PutScalingPolicy",
    "DeletePolicy",
    "CreateOrUpdateTags",
    "DeleteTags",
    "PutLifecycleHook",
    "DeleteLifecycleHook",
    "PutScheduledUpdateGroupAction",
    "DeleteScheduledAction",
    "BatchDeleteScheduledAction",
    "BatchPutScheduledUpdateGroupAction",
    "AttachInstances",
    "DetachInstances",
    "AttachLoadBalancers",
    "DetachLoadBalancers",
    "AttachLoadBalancerTargetGroups",
    "DetachLoadBalancerTargetGroups",
    "PutWarmPool",
    "DeleteWarmPool",
    "SetInstanceHealth",
    "SetInstanceProtection",
    "SuspendProcesses",
    "ResumeProcesses",
    "PutNotificationConfiguration",
    "DeleteNotificationConfiguration",
    "EnableMetricsCollection",
    "DisableMetricsCollection",
    "TerminateInstanceInAutoScalingGroup",
    "ExecutePolicy",
];

impl AutoScalingService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.clone(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateAutoScalingGroup" => {
                self.handle_create_asg(&state, &params, account_id, &region)
                    .await
            }
            "UpdateAutoScalingGroup" => self.handle_update_asg(&state, &params).await,
            "DeleteAutoScalingGroup" => self.handle_delete_asg(&state, &params).await,
            "DescribeAutoScalingGroups" => self.handle_describe_asgs(&state, &params).await,
            "CreateLaunchConfiguration" => {
                self.handle_create_launch_config(&state, &params, account_id, &region)
                    .await
            }
            "DeleteLaunchConfiguration" => self.handle_delete_launch_config(&state, &params).await,
            "DescribeLaunchConfigurations" => {
                self.handle_describe_launch_configs(&state, &params).await
            }
            "SetDesiredCapacity" => self.handle_set_desired_capacity(&state, &params).await,
            "PutScalingPolicy" => {
                self.handle_put_scaling_policy(&state, &params, account_id, &region)
                    .await
            }
            "DeletePolicy" => self.handle_delete_policy(&state, &params).await,
            "DescribePolicies" => self.handle_describe_policies(&state, &params).await,
            "CreateOrUpdateTags" => self.handle_create_or_update_tags(&state, &params).await,
            "DeleteTags" => self.handle_delete_tags(&state, &params).await,
            "DescribeTags" => self.handle_describe_tags(&state, &params).await,
            "DescribeAutoScalingInstances" => {
                self.handle_describe_asg_instances(&state, &params).await
            }
            "PutLifecycleHook" => self.handle_put_lifecycle_hook(&state, &params).await,
            "DeleteLifecycleHook" => self.handle_delete_lifecycle_hook(&state, &params).await,
            "DescribeLifecycleHooks" => self.handle_describe_lifecycle_hooks(&state, &params).await,
            "PutScheduledUpdateGroupAction" => {
                self.handle_put_scheduled_action(&state, &params, account_id, &region)
                    .await
            }
            "DeleteScheduledAction" => self.handle_delete_scheduled_action(&state, &params).await,
            "DescribeScheduledActions" => {
                self.handle_describe_scheduled_actions(&state, &params)
                    .await
            }
            "AttachInstances" => self.handle_attach_instances(&state, &params).await,
            "DetachInstances" => self.handle_detach_instances(&state, &params).await,
            "SuspendProcesses" => self.handle_suspend_processes(&state, &params).await,
            "ResumeProcesses" => self.handle_resume_processes(&state, &params).await,
            "DescribeAccountLimits" => self.handle_describe_account_limits(&state).await,
            "DescribeAdjustmentTypes" => self.handle_describe_adjustment_types().await,
            "DescribeScalingActivities" => {
                self.handle_describe_scaling_activities(&state, &params)
                    .await
            }
            "TerminateInstanceInAutoScalingGroup" => {
                self.handle_terminate_instance(&state, &params).await
            }
            "DescribeNotificationConfigurations" => {
                self.handle_describe_notification_configurations(&state, &params)
                    .await
            }
            "PutNotificationConfiguration" => {
                self.handle_put_notification_configuration(&state, &params)
                    .await
            }
            "DeleteNotificationConfiguration" => {
                self.handle_delete_notification_configuration(&state, &params)
                    .await
            }
            "EnableMetricsCollection" => {
                self.handle_enable_metrics_collection(&state, &params).await
            }
            "DisableMetricsCollection" => {
                self.handle_disable_metrics_collection(&state, &params)
                    .await
            }
            "DescribeMetricCollectionTypes" => self.handle_describe_metric_collection_types().await,
            "DescribeScalingProcessTypes" => self.handle_describe_scaling_process_types().await,
            "DescribeTerminationPolicyTypes" => {
                self.handle_describe_termination_policy_types().await
            }
            "DescribeAutoScalingNotificationTypes" => {
                self.handle_describe_autoscaling_notification_types().await
            }
            "ExecutePolicy" => self.handle_execute_policy(&state, &params).await,
            "AttachLoadBalancers" => self.handle_attach_load_balancers(&state, &params).await,
            "DetachLoadBalancers" => self.handle_detach_load_balancers(&state, &params).await,
            "DescribeLoadBalancers" => self.handle_describe_load_balancers(&state, &params).await,
            "AttachLoadBalancerTargetGroups" => {
                self.handle_attach_load_balancer_target_groups(&state, &params)
                    .await
            }
            "DetachLoadBalancerTargetGroups" => {
                self.handle_detach_load_balancer_target_groups(&state, &params)
                    .await
            }
            "DescribeLoadBalancerTargetGroups" => {
                self.handle_describe_load_balancer_target_groups(&state, &params)
                    .await
            }
            "PutWarmPool" => self.handle_put_warm_pool(&state, &params).await,
            "DeleteWarmPool" => self.handle_delete_warm_pool(&state, &params).await,
            "DescribeWarmPool" => self.handle_describe_warm_pool(&state, &params).await,
            "SetInstanceHealth" => self.handle_set_instance_health(&state, &params).await,
            "SetInstanceProtection" => self.handle_set_instance_protection(&state, &params).await,
            "BatchDeleteScheduledAction" => {
                self.handle_batch_delete_scheduled_action(&state, &params)
                    .await
            }
            "BatchPutScheduledUpdateGroupAction" => {
                self.handle_batch_put_scheduled_update_group_action(
                    &state, &params, account_id, &region,
                )
                .await
            }
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for AutoScaling"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_asg(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_auto_scaling_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let availability_zones = input
            .availability_zones
            .map(|l| l.items)
            .unwrap_or_default();
        let tags = wire_tags_to_domain(input.tags.as_ref());
        let termination_policies = input
            .termination_policies
            .map(|l| l.items)
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.create_group(
            input.auto_scaling_group_name,
            input.min_size,
            input.max_size,
            input.desired_capacity,
            input.launch_configuration_name,
            input.v_p_c_zone_identifier,
            availability_zones,
            input.health_check_type,
            input.health_check_grace_period,
            input.default_cooldown,
            tags,
            termination_policies,
            account_id,
            region,
        ) {
            Ok(()) => wire::serialize_create_auto_scaling_group_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_update_asg(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_auto_scaling_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let availability_zones = input.availability_zones.map(|l| l.items);
        let mut state = state.write().await;
        match state.update_group(
            &input.auto_scaling_group_name,
            input.min_size,
            input.max_size,
            input.desired_capacity,
            input.launch_configuration_name,
            input.v_p_c_zone_identifier,
            availability_zones,
            input.health_check_type,
            input.health_check_grace_period,
            input.default_cooldown,
        ) {
            Ok(()) => wire::serialize_update_auto_scaling_group_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_asg(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_auto_scaling_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.delete_group(&input.auto_scaling_group_name) {
            Ok(()) => wire::serialize_delete_auto_scaling_group_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_asgs(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_auto_scaling_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let names = input
            .auto_scaling_group_names
            .map(|l| l.items)
            .unwrap_or_default();
        let names_opt = if names.is_empty() {
            None
        } else {
            Some(names.as_slice())
        };
        let state = state.read().await;
        let groups = state.describe_groups(names_opt);
        let wire_groups: Vec<wire::AutoScalingGroup> = groups
            .iter()
            .map(|g| wire::AutoScalingGroup {
                auto_scaling_group_name: Some(g.name.clone()),
                auto_scaling_group_a_r_n: Some(g.arn.clone()),
                min_size: Some(g.min_size),
                max_size: Some(g.max_size),
                desired_capacity: Some(g.desired_capacity),
                launch_configuration_name: g.launch_configuration_name.clone(),
                v_p_c_zone_identifier: g.vpc_zone_identifier.clone(),
                availability_zones: Some(wire::AvailabilityZones::from(
                    g.availability_zones.clone(),
                )),
                health_check_type: Some(g.health_check_type.clone()),
                health_check_grace_period: Some(g.health_check_grace_period),
                default_cooldown: Some(g.default_cooldown),
                created_time: Some(g.created_time.clone()),
                status: g.status.clone(),
                tags: Some(wire::TagDescriptionList::from(
                    g.tags
                        .iter()
                        .map(|t| wire::TagDescription {
                            key: Some(t.key.clone()),
                            value: Some(t.value.clone()),
                            resource_id: Some(t.resource_id.clone()),
                            resource_type: Some(t.resource_type.clone()),
                            propagate_at_launch: Some(t.propagate_at_launch),
                        })
                        .collect::<Vec<_>>(),
                )),
                suspended_processes: Some(wire::SuspendedProcesses::from(
                    g.suspended_processes
                        .iter()
                        .map(|p| wire::SuspendedProcess {
                            process_name: Some(p.clone()),
                            suspension_reason: None,
                        })
                        .collect::<Vec<_>>(),
                )),
                termination_policies: Some(wire::TerminationPolicies::from(
                    g.termination_policies.clone(),
                )),
                enabled_metrics: Some(wire::EnabledMetrics::from(
                    g.enabled_metrics
                        .iter()
                        .map(|m| wire::EnabledMetric {
                            granularity: Some("1Minute".to_string()),
                            metric: Some(m.clone()),
                        })
                        .collect::<Vec<_>>(),
                )),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_auto_scaling_groups_response(&wire::AutoScalingGroupsType {
            auto_scaling_groups: Some(wire::AutoScalingGroups::from(wire_groups)),
            next_token: None,
        })
    }

    async fn handle_create_launch_config(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_launch_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.launch_configuration_name.is_empty() {
            return MockResponse::error(
                400,
                "ValidationError",
                "Missing 'LaunchConfigurationName'",
            );
        }
        let security_groups = input.security_groups.map(|l| l.items).unwrap_or_default();
        let ebs_optimized = input.ebs_optimized.unwrap_or(false);
        let mut state = state.write().await;
        match state.create_launch_configuration(
            input.launch_configuration_name,
            input.image_id,
            input.instance_type,
            input.key_name,
            input.iam_instance_profile,
            input.user_data,
            security_groups,
            ebs_optimized,
            input.associate_public_ip_address,
            input.spot_price,
            account_id,
            region,
        ) {
            Ok(()) => wire::serialize_create_launch_configuration_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_launch_config(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_launch_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.launch_configuration_name.is_empty() {
            return MockResponse::error(
                400,
                "ValidationError",
                "Missing 'LaunchConfigurationName'",
            );
        }
        let mut state = state.write().await;
        match state.delete_launch_configuration(&input.launch_configuration_name) {
            Ok(()) => wire::serialize_delete_launch_configuration_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_launch_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_launch_configurations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let names = input
            .launch_configuration_names
            .map(|l| l.items)
            .unwrap_or_default();
        let names_opt = if names.is_empty() {
            None
        } else {
            Some(names.as_slice())
        };
        let state = state.read().await;
        let lcs = state.describe_launch_configurations(names_opt);
        let wire_lcs: Vec<wire::LaunchConfiguration> = lcs
            .iter()
            .map(|lc| wire::LaunchConfiguration {
                launch_configuration_name: Some(lc.name.clone()),
                launch_configuration_a_r_n: Some(lc.arn.clone()),
                image_id: lc.image_id.clone(),
                instance_type: lc.instance_type.clone(),
                key_name: lc.key_name.clone(),
                iam_instance_profile: lc.iam_instance_profile.clone(),
                user_data: lc.user_data.clone(),
                security_groups: if lc.security_groups.is_empty() {
                    None
                } else {
                    Some(wire::SecurityGroups::from(lc.security_groups.clone()))
                },
                ebs_optimized: Some(lc.ebs_optimized),
                associate_public_ip_address: lc.associate_public_ip_address,
                spot_price: lc.spot_price.clone(),
                created_time: Some(lc.created_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_launch_configurations_response(&wire::LaunchConfigurationsType {
            launch_configurations: Some(wire::LaunchConfigurations::from(wire_lcs)),
            next_token: None,
        })
    }

    async fn handle_set_desired_capacity(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_desired_capacity_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.set_desired_capacity(&input.auto_scaling_group_name, input.desired_capacity) {
            Ok(()) => wire::serialize_set_desired_capacity_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_put_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_scaling_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'PolicyName'");
        }
        let mut state = state.write().await;
        match state.put_scaling_policy(
            &input.auto_scaling_group_name,
            &input.policy_name,
            input.policy_type,
            input.adjustment_type,
            input.scaling_adjustment,
            input.cooldown,
            input.min_adjustment_magnitude,
            account_id,
            region,
        ) {
            Ok(arn) => wire::serialize_put_scaling_policy_response(&wire::PolicyARNType {
                policy_a_r_n: Some(arn),
                alarms: None,
            }),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'PolicyName'");
        }
        let mut state = state.write().await;
        match state.delete_policy(input.auto_scaling_group_name.as_deref(), &input.policy_name) {
            Ok(()) => wire::serialize_delete_policy_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let policy_names = input.policy_names.map(|l| l.items).unwrap_or_default();
        let policy_names_opt = if policy_names.is_empty() {
            None
        } else {
            Some(policy_names.as_slice())
        };
        let state = state.read().await;
        let policies =
            state.describe_policies(input.auto_scaling_group_name.as_deref(), policy_names_opt);
        let wire_policies: Vec<wire::ScalingPolicy> = policies
            .iter()
            .map(|p| wire::ScalingPolicy {
                policy_name: Some(p.name.clone()),
                policy_a_r_n: Some(p.arn.clone()),
                auto_scaling_group_name: Some(p.group_name.clone()),
                policy_type: p.policy_type.clone(),
                adjustment_type: p.adjustment_type.clone(),
                scaling_adjustment: p.scaling_adjustment,
                cooldown: p.cooldown,
                min_adjustment_magnitude: p.min_adjustment_magnitude,
                enabled: Some(true),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_policies_response(&wire::PoliciesType {
            scaling_policies: Some(wire::ScalingPolicies::from(wire_policies)),
            next_token: None,
        })
    }

    async fn handle_create_or_update_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_or_update_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let tags = wire_tags_to_domain(Some(&input.tags));
        let mut state = state.write().await;
        match state.create_or_update_tags(tags) {
            Ok(()) => wire::serialize_create_or_update_tags_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let tags = wire_tags_to_domain(Some(&input.tags));
        let mut state = state.write().await;
        state.delete_tags(tags);
        wire::serialize_delete_tags_response()
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let group_name = input
            .filters
            .as_ref()
            .and_then(|fs| fs.items.first())
            .filter(|f| f.name.as_deref() == Some("auto-scaling-group"))
            .and_then(|f| f.values.as_ref())
            .and_then(|v| v.items.first())
            .map(|s| s.as_str());
        let state = state.read().await;
        let tags = state.describe_tags(group_name);
        let wire_tags: Vec<wire::TagDescription> = tags
            .iter()
            .map(|(t, _)| wire::TagDescription {
                key: Some(t.key.clone()),
                value: Some(t.value.clone()),
                resource_id: Some(t.resource_id.clone()),
                resource_type: Some(t.resource_type.clone()),
                propagate_at_launch: Some(t.propagate_at_launch),
            })
            .collect();
        wire::serialize_describe_tags_response(&wire::TagsType {
            tags: Some(wire::TagDescriptionList::from(wire_tags)),
            next_token: None,
        })
    }

    async fn handle_describe_asg_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_auto_scaling_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let instance_ids = input.instance_ids.map(|l| l.items).unwrap_or_default();
        let ids_opt = if instance_ids.is_empty() {
            None
        } else {
            Some(instance_ids.as_slice())
        };
        let state = state.read().await;
        let instances = state.describe_auto_scaling_instances(ids_opt);
        let wire_instances: Vec<wire::AutoScalingInstanceDetails> = instances
            .iter()
            .map(|i| wire::AutoScalingInstanceDetails {
                instance_id: Some(i.instance_id.clone()),
                auto_scaling_group_name: Some(i.group_name.clone()),
                availability_zone: Some(i.availability_zone.clone()),
                lifecycle_state: Some(i.lifecycle_state.clone()),
                health_status: Some(i.health_status.clone()),
                launch_configuration_name: i.launch_configuration_name.clone(),
                protected_from_scale_in: Some(i.protected_from_scale_in),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_auto_scaling_instances_response(&wire::AutoScalingInstancesType {
            auto_scaling_instances: Some(wire::AutoScalingInstances::from(wire_instances)),
            next_token: None,
        })
    }

    async fn handle_put_lifecycle_hook(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_lifecycle_hook_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.lifecycle_hook_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'LifecycleHookName'");
        }
        let mut state = state.write().await;
        match state.put_lifecycle_hook(
            &input.auto_scaling_group_name,
            &input.lifecycle_hook_name,
            input.lifecycle_transition,
            input.notification_target_a_r_n,
            input.role_a_r_n,
            input.notification_metadata,
            input.heartbeat_timeout,
            input.default_result,
        ) {
            Ok(()) => wire::serialize_put_lifecycle_hook_response(&wire::PutLifecycleHookAnswer {}),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_lifecycle_hook(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_lifecycle_hook_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.lifecycle_hook_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'LifecycleHookName'");
        }
        let mut state = state.write().await;
        match state
            .delete_lifecycle_hook(&input.auto_scaling_group_name, &input.lifecycle_hook_name)
        {
            Ok(()) => {
                wire::serialize_delete_lifecycle_hook_response(&wire::DeleteLifecycleHookAnswer {})
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_lifecycle_hooks(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_lifecycle_hooks_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let hook_names = input
            .lifecycle_hook_names
            .map(|l| l.items)
            .unwrap_or_default();
        let hook_names_opt = if hook_names.is_empty() {
            None
        } else {
            Some(hook_names.as_slice())
        };
        let state = state.read().await;
        let hooks = state.describe_lifecycle_hooks(&input.auto_scaling_group_name, hook_names_opt);
        let wire_hooks: Vec<wire::LifecycleHook> = hooks
            .iter()
            .map(|h| wire::LifecycleHook {
                lifecycle_hook_name: Some(h.name.clone()),
                auto_scaling_group_name: Some(h.group_name.clone()),
                lifecycle_transition: h.lifecycle_transition.clone(),
                notification_target_a_r_n: h.notification_target_arn.clone(),
                role_a_r_n: h.role_arn.clone(),
                notification_metadata: h.notification_metadata.clone(),
                heartbeat_timeout: h.heartbeat_timeout,
                default_result: h.default_result.clone(),
                global_timeout: None,
            })
            .collect();
        wire::serialize_describe_lifecycle_hooks_response(&wire::DescribeLifecycleHooksAnswer {
            lifecycle_hooks: Some(wire::LifecycleHooks::from(wire_hooks)),
        })
    }

    async fn handle_put_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_scheduled_update_group_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.scheduled_action_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'ScheduledActionName'");
        }
        let mut state = state.write().await;
        match state.put_scheduled_action(
            &input.auto_scaling_group_name,
            &input.scheduled_action_name,
            input.desired_capacity,
            input.min_size,
            input.max_size,
            input.start_time,
            input.end_time,
            input.recurrence,
            input.time_zone,
            account_id,
            region,
        ) {
            Ok(()) => wire::serialize_put_scheduled_update_group_action_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scheduled_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.scheduled_action_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'ScheduledActionName'");
        }
        let mut state = state.write().await;
        match state
            .delete_scheduled_action(&input.auto_scaling_group_name, &input.scheduled_action_name)
        {
            Ok(()) => wire::serialize_delete_scheduled_action_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_scheduled_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scheduled_actions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        let actions = state.describe_scheduled_actions(input.auto_scaling_group_name.as_deref());
        let wire_actions: Vec<wire::ScheduledUpdateGroupAction> = actions
            .iter()
            .map(|a| wire::ScheduledUpdateGroupAction {
                scheduled_action_name: Some(a.name.clone()),
                scheduled_action_a_r_n: Some(a.arn.clone()),
                auto_scaling_group_name: Some(a.group_name.clone()),
                desired_capacity: a.desired_capacity,
                min_size: a.min_size,
                max_size: a.max_size,
                start_time: a.start_time.clone(),
                end_time: a.end_time.clone(),
                recurrence: a.recurrence.clone(),
                time_zone: a.time_zone.clone(),
                time: None,
            })
            .collect();
        wire::serialize_describe_scheduled_actions_response(&wire::ScheduledActionsType {
            scheduled_update_group_actions: Some(wire::ScheduledUpdateGroupActions::from(
                wire_actions,
            )),
            next_token: None,
        })
    }

    async fn handle_attach_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let state = state.read().await;
        if !state.groups.contains_key(&input.auto_scaling_group_name) {
            return asg_error_response(&AutoScalingError::NotFound(
                input.auto_scaling_group_name.clone(),
            ));
        }
        wire::serialize_attach_instances_response()
    }

    async fn handle_detach_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let state = state.read().await;
        if !state.groups.contains_key(&input.auto_scaling_group_name) {
            return asg_error_response(&AutoScalingError::NotFound(
                input.auto_scaling_group_name.clone(),
            ));
        }
        wire::serialize_detach_instances_response(&wire::DetachInstancesAnswer { activities: None })
    }

    async fn handle_suspend_processes(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_suspend_processes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let processes = input.scaling_processes.map(|l| l.items).unwrap_or_default();
        let mut state = state.write().await;
        match state.suspend_processes(&input.auto_scaling_group_name, processes) {
            Ok(()) => wire::serialize_suspend_processes_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_resume_processes(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_resume_processes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let processes = input.scaling_processes.map(|l| l.items).unwrap_or_default();
        let mut state = state.write().await;
        match state.resume_processes(&input.auto_scaling_group_name, processes) {
            Ok(()) => wire::serialize_resume_processes_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_account_limits(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let (max_groups, max_lcs, num_groups, num_lcs) = state.describe_account_limits();
        wire::serialize_describe_account_limits_response(&wire::DescribeAccountLimitsAnswer {
            max_number_of_auto_scaling_groups: Some(max_groups),
            max_number_of_launch_configurations: Some(max_lcs),
            number_of_auto_scaling_groups: Some(num_groups),
            number_of_launch_configurations: Some(num_lcs),
        })
    }

    async fn handle_describe_adjustment_types(&self) -> MockResponse {
        let adjustment_types = vec![
            wire::AdjustmentType {
                adjustment_type: Some("ChangeInCapacity".to_string()),
            },
            wire::AdjustmentType {
                adjustment_type: Some("ExactCapacity".to_string()),
            },
            wire::AdjustmentType {
                adjustment_type: Some("PercentChangeInCapacity".to_string()),
            },
        ];
        wire::serialize_describe_adjustment_types_response(&wire::DescribeAdjustmentTypesAnswer {
            adjustment_types: Some(wire::AdjustmentTypes::from(adjustment_types)),
        })
    }

    async fn handle_describe_scaling_activities(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scaling_activities_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_scaling_activities(input.auto_scaling_group_name.as_deref()) {
            Ok(activities) => {
                let wire_activities: Vec<wire::Activity> = activities
                    .iter()
                    .map(|a| wire::Activity {
                        activity_id: Some(a.activity_id.clone()),
                        auto_scaling_group_name: Some(a.group_name.clone()),
                        cause: Some(a.cause.clone()),
                        start_time: Some(a.start_time.clone()),
                        end_time: a.end_time.clone(),
                        status_code: Some(a.status_code.clone()),
                        description: Some(a.description.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_scaling_activities_response(&wire::ActivitiesType {
                    activities: Some(wire::Activities::from(wire_activities)),
                    next_token: None,
                })
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_terminate_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_instance_in_auto_scaling_group_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_id.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'InstanceId'");
        }
        let should_decrement = input.should_decrement_desired_capacity;
        let mut state = state.write().await;
        match state.terminate_instance(&input.instance_id, should_decrement) {
            Ok(activity) => {
                let wire_activity = wire::Activity {
                    activity_id: Some(activity.activity_id),
                    auto_scaling_group_name: Some(activity.group_name),
                    cause: Some(activity.cause),
                    start_time: Some(activity.start_time),
                    end_time: activity.end_time,
                    status_code: Some(activity.status_code),
                    description: Some(activity.description),
                    ..Default::default()
                };
                wire::serialize_terminate_instance_in_auto_scaling_group_response(
                    &wire::ActivityType {
                        activity: Some(wire_activity),
                    },
                )
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_notification_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_notification_configurations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let group_names = input
            .auto_scaling_group_names
            .map(|l| l.items)
            .unwrap_or_default();
        let names_opt = if group_names.is_empty() {
            None
        } else {
            Some(group_names.as_slice())
        };
        let state = state.read().await;
        let configs = state.describe_notification_configurations(names_opt);
        let wire_configs: Vec<wire::NotificationConfiguration> = configs
            .iter()
            .map(|n| wire::NotificationConfiguration {
                auto_scaling_group_name: Some(n.group_name.clone()),
                notification_type: Some(n.notification_type.clone()),
                topic_a_r_n: Some(n.topic_arn.clone()),
            })
            .collect();
        wire::serialize_describe_notification_configurations_response(
            &wire::DescribeNotificationConfigurationsAnswer {
                notification_configurations: Some(wire::NotificationConfigurations::from(
                    wire_configs,
                )),
                next_token: None,
            },
        )
    }

    async fn handle_put_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_notification_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.topic_a_r_n.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'TopicARN'");
        }
        let notification_types = input.notification_types.items;
        let mut state = state.write().await;
        match state.put_notification_configuration(
            &input.auto_scaling_group_name,
            &input.topic_a_r_n,
            notification_types,
        ) {
            Ok(()) => wire::serialize_put_notification_configuration_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_notification_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        if input.topic_a_r_n.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'TopicARN'");
        }
        let mut state = state.write().await;
        match state
            .delete_notification_configuration(&input.auto_scaling_group_name, &input.topic_a_r_n)
        {
            Ok(()) => wire::serialize_delete_notification_configuration_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_enable_metrics_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_metrics_collection_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let metrics = input.metrics.map(|l| l.items).unwrap_or_default();
        let mut state = state.write().await;
        match state.enable_metrics_collection(&input.auto_scaling_group_name, metrics) {
            Ok(()) => wire::serialize_enable_metrics_collection_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_disable_metrics_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_metrics_collection_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let metrics = input.metrics.map(|l| l.items).unwrap_or_default();
        let mut state = state.write().await;
        match state.disable_metrics_collection(&input.auto_scaling_group_name, metrics) {
            Ok(()) => wire::serialize_disable_metrics_collection_response(),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_metric_collection_types(&self) -> MockResponse {
        let metric_types = vec![
            wire::MetricCollectionType {
                metric: Some("GroupMinSize".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupMaxSize".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupDesiredCapacity".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupInServiceCapacity".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupPendingCapacity".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupStandbyCapacity".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupTerminatingCapacity".to_string()),
            },
            wire::MetricCollectionType {
                metric: Some("GroupTotalCapacity".to_string()),
            },
        ];
        let granularity_types = vec![wire::MetricGranularityType {
            granularity: Some("1Minute".to_string()),
        }];
        wire::serialize_describe_metric_collection_types_response(
            &wire::DescribeMetricCollectionTypesAnswer {
                metrics: Some(wire::MetricCollectionTypes::from(metric_types)),
                granularities: Some(wire::MetricGranularityTypes::from(granularity_types)),
            },
        )
    }

    async fn handle_describe_scaling_process_types(&self) -> MockResponse {
        let process_types = [
            "Launch",
            "Terminate",
            "HealthCheck",
            "AZRebalance",
            "AlarmNotification",
            "ScheduledActions",
            "AddToLoadBalancer",
            "InstanceRefresh",
        ];
        let wire_processes: Vec<wire::ProcessType> = process_types
            .iter()
            .map(|p| wire::ProcessType {
                process_name: Some(p.to_string()),
            })
            .collect();
        wire::serialize_describe_scaling_process_types_response(&wire::ProcessesType {
            processes: Some(wire::Processes::from(wire_processes)),
        })
    }

    async fn handle_describe_termination_policy_types(&self) -> MockResponse {
        let policies = [
            "OldestInstance",
            "NewestInstance",
            "OldestLaunchConfiguration",
            "ClosestToNextInstanceHour",
            "Default",
            "OldestLaunchTemplate",
            "AllocationStrategy",
        ];
        wire::serialize_describe_termination_policy_types_response(
            &wire::DescribeTerminationPolicyTypesAnswer {
                termination_policy_types: Some(wire::TerminationPolicies::from(
                    policies.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
                )),
            },
        )
    }

    async fn handle_describe_autoscaling_notification_types(&self) -> MockResponse {
        let notification_types = [
            "autoscaling:EC2_INSTANCE_LAUNCH",
            "autoscaling:EC2_INSTANCE_LAUNCH_ERROR",
            "autoscaling:EC2_INSTANCE_TERMINATE",
            "autoscaling:EC2_INSTANCE_TERMINATE_ERROR",
            "autoscaling:TEST_NOTIFICATION",
        ];
        wire::serialize_describe_auto_scaling_notification_types_response(
            &wire::DescribeAutoScalingNotificationTypesAnswer {
                auto_scaling_notification_types: Some(wire::AutoScalingNotificationTypes::from(
                    notification_types
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>(),
                )),
            },
        )
    }

    async fn handle_execute_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'PolicyName'");
        }
        let state = state.read().await;
        let policy_exists = state.policies.values().any(|p| {
            (p.name == input.policy_name || p.arn == input.policy_name)
                && input
                    .auto_scaling_group_name
                    .as_ref()
                    .is_none_or(|gn| p.group_name == *gn)
        });
        if !policy_exists {
            return MockResponse::error(
                400,
                "ValidationError",
                &format!("Policy '{}' not found", input.policy_name),
            );
        }
        wire::serialize_execute_policy_response()
    }

    async fn handle_attach_load_balancers(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_load_balancers_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.attach_load_balancers(
            &input.auto_scaling_group_name,
            input.load_balancer_names.items,
        ) {
            Ok(()) => wire::serialize_attach_load_balancers_response(
                &wire::AttachLoadBalancersResultType {},
            ),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_detach_load_balancers(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_load_balancers_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.detach_load_balancers(
            &input.auto_scaling_group_name,
            input.load_balancer_names.items,
        ) {
            Ok(()) => wire::serialize_detach_load_balancers_response(
                &wire::DetachLoadBalancersResultType {},
            ),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_load_balancers(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancers_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let state = state.read().await;
        match state.describe_load_balancers(&input.auto_scaling_group_name) {
            Ok(lbs) => {
                let wire_lbs: Vec<wire::LoadBalancerState> = lbs
                    .iter()
                    .map(|lb| wire::LoadBalancerState {
                        load_balancer_name: Some(lb.clone()),
                        state: Some("InService".to_string()),
                    })
                    .collect();
                wire::serialize_describe_load_balancers_response(
                    &wire::DescribeLoadBalancersResponse {
                        load_balancers: Some(wire::LoadBalancerStates::from(wire_lbs)),
                        next_token: None,
                    },
                )
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_attach_load_balancer_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_load_balancer_target_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.attach_target_groups(
            &input.auto_scaling_group_name,
            input.target_group_a_r_ns.items,
        ) {
            Ok(()) => wire::serialize_attach_load_balancer_target_groups_response(
                &wire::AttachLoadBalancerTargetGroupsResultType {},
            ),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_detach_load_balancer_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_load_balancer_target_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.detach_target_groups(
            &input.auto_scaling_group_name,
            input.target_group_a_r_ns.items,
        ) {
            Ok(()) => wire::serialize_detach_load_balancer_target_groups_response(
                &wire::DetachLoadBalancerTargetGroupsResultType {},
            ),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_load_balancer_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancer_target_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let state = state.read().await;
        match state.describe_target_groups(&input.auto_scaling_group_name) {
            Ok(tgs) => {
                let wire_tgs: Vec<wire::LoadBalancerTargetGroupState> = tgs
                    .iter()
                    .map(|tg| wire::LoadBalancerTargetGroupState {
                        load_balancer_target_group_a_r_n: Some(tg.clone()),
                        state: Some("InService".to_string()),
                    })
                    .collect();
                wire::serialize_describe_load_balancer_target_groups_response(
                    &wire::DescribeLoadBalancerTargetGroupsResponse {
                        load_balancer_target_groups: Some(
                            wire::LoadBalancerTargetGroupStates::from(wire_tgs),
                        ),
                        next_token: None,
                    },
                )
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_put_warm_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_warm_pool_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.put_warm_pool(
            &input.auto_scaling_group_name,
            input.min_size,
            input.max_group_prepared_capacity,
            input.pool_state,
        ) {
            Ok(()) => wire::serialize_put_warm_pool_response(&wire::PutWarmPoolAnswer {}),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_delete_warm_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_warm_pool_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.delete_warm_pool(&input.auto_scaling_group_name) {
            Ok(()) => wire::serialize_delete_warm_pool_response(&wire::DeleteWarmPoolAnswer {}),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_describe_warm_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_warm_pool_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let state = state.read().await;
        match state.describe_warm_pool(&input.auto_scaling_group_name) {
            Ok(wp) => {
                let warm_pool_config = wp.map(|w| wire::WarmPoolConfiguration {
                    min_size: w.min_size,
                    max_group_prepared_capacity: w.max_group_prepared_capacity,
                    pool_state: w.pool_state.clone(),
                    status: None,
                    instance_reuse_policy: None,
                });
                wire::serialize_describe_warm_pool_response(&wire::DescribeWarmPoolAnswer {
                    warm_pool_configuration: warm_pool_config,
                    instances: None,
                    next_token: None,
                })
            }
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_set_instance_health(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_instance_health_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_id.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'InstanceId'");
        }
        if input.health_status.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'HealthStatus'");
        }
        let mut state = state.write().await;
        state.set_instance_health(&input.instance_id, &input.health_status);
        wire::serialize_set_instance_health_response()
    }

    async fn handle_set_instance_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_instance_protection_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        match state.set_instance_protection(
            &input.auto_scaling_group_name,
            &input.instance_ids.items,
            input.protected_from_scale_in,
        ) {
            Ok(()) => wire::serialize_set_instance_protection_response(
                &wire::SetInstanceProtectionAnswer {},
            ),
            Err(e) => asg_error_response(&e),
        }
    }

    async fn handle_batch_delete_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_scheduled_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut state = state.write().await;
        for action_name in &input.scheduled_action_names.items {
            let _ = state.delete_scheduled_action(&input.auto_scaling_group_name, action_name);
        }
        wire::serialize_batch_delete_scheduled_action_response(
            &wire::BatchDeleteScheduledActionAnswer {
                failed_scheduled_actions: None,
            },
        )
    }

    async fn handle_batch_put_scheduled_update_group_action(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_put_scheduled_update_group_action_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.auto_scaling_group_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "Missing 'AutoScalingGroupName'");
        }
        let mut failed: Vec<wire::FailedScheduledUpdateGroupActionRequest> = Vec::new();
        let mut state = state.write().await;
        for action in &input.scheduled_update_group_actions.items {
            if let Err(e) = state.put_scheduled_action(
                &input.auto_scaling_group_name,
                &action.scheduled_action_name,
                action.desired_capacity,
                action.min_size,
                action.max_size,
                action.start_time.clone(),
                action.end_time.clone(),
                action.recurrence.clone(),
                action.time_zone.clone(),
                account_id,
                region,
            ) {
                let error_code = match &e {
                    AutoScalingError::AlreadyExists(_) => "AlreadyExists",
                    AutoScalingError::NotFound(_) | AutoScalingError::LaunchConfigNotFound(_) => {
                        "ValidationError"
                    }
                };
                failed.push(wire::FailedScheduledUpdateGroupActionRequest {
                    scheduled_action_name: Some(action.scheduled_action_name.clone()),
                    error_code: Some(error_code.to_string()),
                    error_message: Some(e.to_string()),
                });
            }
        }
        let failed_opt = if failed.is_empty() {
            None
        } else {
            Some(wire::FailedScheduledUpdateGroupActionRequests::from(failed))
        };
        wire::serialize_batch_put_scheduled_update_group_action_response(
            &wire::BatchPutScheduledUpdateGroupActionAnswer {
                failed_scheduled_update_group_actions: failed_opt,
            },
        )
    }
}

// --- Helpers ---

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            map.insert(urldecode(key), urldecode(value));
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Convert wire::Tag list to domain Tag list.
fn wire_tags_to_domain(tags: Option<&wire::Tags>) -> Vec<Tag> {
    let empty = Vec::new();
    tags.map(|t| &t.items)
        .unwrap_or(&empty)
        .iter()
        .map(|wt| Tag {
            key: wt.key.clone(),
            value: wt.value.clone().unwrap_or_default(),
            resource_id: wt.resource_id.clone().unwrap_or_default(),
            resource_type: wt
                .resource_type
                .clone()
                .unwrap_or_else(|| "auto-scaling-group".to_string()),
            propagate_at_launch: wt.propagate_at_launch.unwrap_or(false),
        })
        .collect()
}

fn asg_error_response(e: &AutoScalingError) -> MockResponse {
    let (status, error_type) = match e {
        AutoScalingError::AlreadyExists(_) => (400u16, "AlreadyExists"),
        AutoScalingError::NotFound(_) | AutoScalingError::LaunchConfigNotFound(_) => {
            (400u16, "ValidationError")
        }
    };
    MockResponse::error(status, error_type, &e.to_string())
}
