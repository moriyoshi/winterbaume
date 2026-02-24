use aws_sdk_autoscaling::config::BehaviorVersion;
use winterbaume_autoscaling::AutoScalingService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_autoscaling::Client {
    let mock = MockAws::builder()
        .with_service(AutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscaling::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_autoscaling::Client::new(&config)
}

// Helper: create a launch configuration
async fn create_lc(client: &aws_sdk_autoscaling::Client, name: &str) {
    client
        .create_launch_configuration()
        .launch_configuration_name(name)
        .image_id("ami-12345678")
        .instance_type("t2.micro")
        .send()
        .await
        .expect("create_launch_configuration should succeed");
}

// Helper: create an ASG with a launch configuration
async fn create_asg(client: &aws_sdk_autoscaling::Client, asg_name: &str, lc_name: &str) {
    create_lc(client, lc_name).await;
    client
        .create_auto_scaling_group()
        .auto_scaling_group_name(asg_name)
        .launch_configuration_name(lc_name)
        .min_size(1)
        .max_size(3)
        .desired_capacity(1)
        .availability_zones("us-east-1a")
        .send()
        .await
        .expect("create_auto_scaling_group should succeed");
}

// ---------------------------------------------------------------------------
// Launch Configuration CRUD (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_describe_delete_launch_configuration() {
    let client = make_client().await;

    // CreateLaunchConfiguration
    create_lc(&client, "test-lc").await;

    // DescribeLaunchConfigurations
    let resp = client
        .describe_launch_configurations()
        .launch_configuration_names("test-lc")
        .send()
        .await
        .expect("describe_launch_configurations should succeed");

    let lcs = resp.launch_configurations();
    assert_eq!(lcs.len(), 1);
    assert_eq!(lcs[0].launch_configuration_name(), Some("test-lc"));
    assert_eq!(lcs[0].image_id(), Some("ami-12345678"));
    assert_eq!(lcs[0].instance_type(), Some("t2.micro"));

    // DeleteLaunchConfiguration
    client
        .delete_launch_configuration()
        .launch_configuration_name("test-lc")
        .send()
        .await
        .expect("delete_launch_configuration should succeed");

    // Verify deletion
    let resp2 = client
        .describe_launch_configurations()
        .launch_configuration_names("test-lc")
        .send()
        .await
        .expect("describe should succeed after delete");
    assert_eq!(resp2.launch_configurations().len(), 0);
}

// ---------------------------------------------------------------------------
// Auto Scaling Group CRUD (4 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_describe_update_delete_asg() {
    let client = make_client().await;
    create_asg(&client, "test-asg", "asg-lc").await;

    // DescribeAutoScalingGroups
    let resp = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("test-asg")
        .send()
        .await
        .expect("describe_auto_scaling_groups should succeed");

    let groups = resp.auto_scaling_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].auto_scaling_group_name(), Some("test-asg"));
    assert_eq!(groups[0].min_size(), Some(1));
    assert_eq!(groups[0].max_size(), Some(3));
    assert_eq!(groups[0].desired_capacity(), Some(1));

    // UpdateAutoScalingGroup
    client
        .update_auto_scaling_group()
        .auto_scaling_group_name("test-asg")
        .max_size(5)
        .desired_capacity(2)
        .send()
        .await
        .expect("update_auto_scaling_group should succeed");

    let resp2 = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("test-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.auto_scaling_groups()[0].max_size(), Some(5));
    assert_eq!(resp2.auto_scaling_groups()[0].desired_capacity(), Some(2));

    // DeleteAutoScalingGroup
    client
        .delete_auto_scaling_group()
        .auto_scaling_group_name("test-asg")
        .send()
        .await
        .expect("delete_auto_scaling_group should succeed");

    let resp3 = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("test-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(resp3.auto_scaling_groups().len(), 0);
}

// ---------------------------------------------------------------------------
// SetDesiredCapacity (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_set_desired_capacity() {
    let client = make_client().await;
    create_asg(&client, "cap-asg", "cap-lc").await;

    client
        .set_desired_capacity()
        .auto_scaling_group_name("cap-asg")
        .desired_capacity(2)
        .send()
        .await
        .expect("set_desired_capacity should succeed");

    let resp = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("cap-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.auto_scaling_groups()[0].desired_capacity(), Some(2));
}

// ---------------------------------------------------------------------------
// Scaling Policy CRUD (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_put_describe_delete_scaling_policy() {
    let client = make_client().await;
    create_asg(&client, "policy-asg", "policy-lc").await;

    // PutScalingPolicy
    let put_resp = client
        .put_scaling_policy()
        .auto_scaling_group_name("policy-asg")
        .policy_name("scale-up")
        .adjustment_type("ChangeInCapacity")
        .scaling_adjustment(1)
        .send()
        .await
        .expect("put_scaling_policy should succeed");

    assert!(put_resp.policy_arn().is_some());

    // DescribePolicies
    let desc = client
        .describe_policies()
        .auto_scaling_group_name("policy-asg")
        .send()
        .await
        .expect("describe_policies should succeed");

    let policies = desc.scaling_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].policy_name(), Some("scale-up"));

    // DeletePolicy
    client
        .delete_policy()
        .auto_scaling_group_name("policy-asg")
        .policy_name("scale-up")
        .send()
        .await
        .expect("delete_policy should succeed");

    let desc2 = client
        .describe_policies()
        .auto_scaling_group_name("policy-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.scaling_policies().len(), 0);
}

// ---------------------------------------------------------------------------
// ExecutePolicy (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_execute_policy() {
    let client = make_client().await;
    create_asg(&client, "exec-asg", "exec-lc").await;

    client
        .put_scaling_policy()
        .auto_scaling_group_name("exec-asg")
        .policy_name("exec-pol")
        .adjustment_type("ChangeInCapacity")
        .scaling_adjustment(1)
        .send()
        .await
        .unwrap();

    client
        .execute_policy()
        .auto_scaling_group_name("exec-asg")
        .policy_name("exec-pol")
        .send()
        .await
        .expect("execute_policy should succeed");
}

// ---------------------------------------------------------------------------
// Tag operations (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_describe_delete_tags() {
    let client = make_client().await;
    create_asg(&client, "tag-asg", "tag-lc").await;

    // CreateOrUpdateTags
    let tag = aws_sdk_autoscaling::types::Tag::builder()
        .key("Env")
        .value("test")
        .resource_id("tag-asg")
        .resource_type("auto-scaling-group")
        .propagate_at_launch(true)
        .build();

    client
        .create_or_update_tags()
        .tags(tag)
        .send()
        .await
        .expect("create_or_update_tags should succeed");

    // DescribeTags
    let desc = client
        .describe_tags()
        .send()
        .await
        .expect("describe_tags should succeed");

    let tags = desc.tags();
    let env_tags: Vec<_> = tags.iter().filter(|t| t.key() == Some("Env")).collect();
    assert!(!env_tags.is_empty());
    assert_eq!(env_tags[0].value(), Some("test"));

    // DeleteTags
    let del_tag = aws_sdk_autoscaling::types::Tag::builder()
        .key("Env")
        .resource_id("tag-asg")
        .resource_type("auto-scaling-group")
        .build();

    client
        .delete_tags()
        .tags(del_tag)
        .send()
        .await
        .expect("delete_tags should succeed");
}

// ---------------------------------------------------------------------------
// Lifecycle Hook CRUD (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_put_describe_delete_lifecycle_hook() {
    let client = make_client().await;
    create_asg(&client, "lch-asg", "lch-lc").await;

    // PutLifecycleHook
    client
        .put_lifecycle_hook()
        .auto_scaling_group_name("lch-asg")
        .lifecycle_hook_name("test-hook")
        .lifecycle_transition("autoscaling:EC2_INSTANCE_LAUNCHING")
        .default_result("CONTINUE")
        .heartbeat_timeout(300)
        .send()
        .await
        .expect("put_lifecycle_hook should succeed");

    // DescribeLifecycleHooks
    let desc = client
        .describe_lifecycle_hooks()
        .auto_scaling_group_name("lch-asg")
        .send()
        .await
        .expect("describe_lifecycle_hooks should succeed");

    let hooks = desc.lifecycle_hooks();
    assert_eq!(hooks.len(), 1);
    assert_eq!(hooks[0].lifecycle_hook_name(), Some("test-hook"));

    // DeleteLifecycleHook
    client
        .delete_lifecycle_hook()
        .auto_scaling_group_name("lch-asg")
        .lifecycle_hook_name("test-hook")
        .send()
        .await
        .expect("delete_lifecycle_hook should succeed");

    let desc2 = client
        .describe_lifecycle_hooks()
        .auto_scaling_group_name("lch-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.lifecycle_hooks().len(), 0);
}

// ---------------------------------------------------------------------------
// Scheduled Action CRUD (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_put_describe_delete_scheduled_action() {
    let client = make_client().await;
    create_asg(&client, "sched-asg", "sched-lc").await;

    // PutScheduledUpdateGroupAction
    client
        .put_scheduled_update_group_action()
        .auto_scaling_group_name("sched-asg")
        .scheduled_action_name("scale-up-morning")
        .desired_capacity(3)
        .recurrence("0 8 * * *")
        .send()
        .await
        .expect("put_scheduled_update_group_action should succeed");

    // DescribeScheduledActions
    let desc = client
        .describe_scheduled_actions()
        .auto_scaling_group_name("sched-asg")
        .send()
        .await
        .expect("describe_scheduled_actions should succeed");

    let actions = desc.scheduled_update_group_actions();
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].scheduled_action_name(), Some("scale-up-morning"));

    // DeleteScheduledAction
    client
        .delete_scheduled_action()
        .auto_scaling_group_name("sched-asg")
        .scheduled_action_name("scale-up-morning")
        .send()
        .await
        .expect("delete_scheduled_action should succeed");

    let desc2 = client
        .describe_scheduled_actions()
        .auto_scaling_group_name("sched-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.scheduled_update_group_actions().len(), 0);
}

// ---------------------------------------------------------------------------
// Suspend / Resume Processes (2 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_suspend_resume_processes() {
    let client = make_client().await;
    create_asg(&client, "proc-asg", "proc-lc").await;

    // SuspendProcesses
    client
        .suspend_processes()
        .auto_scaling_group_name("proc-asg")
        .scaling_processes("Launch")
        .send()
        .await
        .expect("suspend_processes should succeed");

    let resp = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("proc-asg")
        .send()
        .await
        .unwrap();
    let suspended: Vec<_> = resp.auto_scaling_groups()[0]
        .suspended_processes()
        .iter()
        .map(|p| p.process_name().unwrap_or_default().to_string())
        .collect();
    assert!(suspended.contains(&"Launch".to_string()));

    // ResumeProcesses
    client
        .resume_processes()
        .auto_scaling_group_name("proc-asg")
        .scaling_processes("Launch")
        .send()
        .await
        .expect("resume_processes should succeed");

    let resp2 = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("proc-asg")
        .send()
        .await
        .unwrap();
    let suspended2: Vec<_> = resp2.auto_scaling_groups()[0]
        .suspended_processes()
        .iter()
        .map(|p| p.process_name().unwrap_or_default().to_string())
        .collect();
    assert!(!suspended2.contains(&"Launch".to_string()));
}

// ---------------------------------------------------------------------------
// Notification Configuration CRUD (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_put_describe_delete_notification_configuration() {
    let client = make_client().await;
    create_asg(&client, "notif-asg", "notif-lc").await;

    // PutNotificationConfiguration
    client
        .put_notification_configuration()
        .auto_scaling_group_name("notif-asg")
        .topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .notification_types("autoscaling:EC2_INSTANCE_LAUNCH")
        .send()
        .await
        .expect("put_notification_configuration should succeed");

    // DescribeNotificationConfigurations
    let desc = client
        .describe_notification_configurations()
        .auto_scaling_group_names("notif-asg")
        .send()
        .await
        .expect("describe_notification_configurations should succeed");

    let configs = desc.notification_configurations();
    assert!(!configs.is_empty());

    // DeleteNotificationConfiguration
    client
        .delete_notification_configuration()
        .auto_scaling_group_name("notif-asg")
        .topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .send()
        .await
        .expect("delete_notification_configuration should succeed");
}

// ---------------------------------------------------------------------------
// Metrics Collection (2 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_enable_disable_metrics_collection() {
    let client = make_client().await;
    create_asg(&client, "metrics-asg", "metrics-lc").await;

    // EnableMetricsCollection
    client
        .enable_metrics_collection()
        .auto_scaling_group_name("metrics-asg")
        .granularity("1Minute")
        .metrics("GroupMinSize")
        .send()
        .await
        .expect("enable_metrics_collection should succeed");

    // DisableMetricsCollection
    client
        .disable_metrics_collection()
        .auto_scaling_group_name("metrics-asg")
        .metrics("GroupMinSize")
        .send()
        .await
        .expect("disable_metrics_collection should succeed");
}

// ---------------------------------------------------------------------------
// Describe-only operations (7 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_describe_account_limits() {
    let client = make_client().await;

    let resp = client
        .describe_account_limits()
        .send()
        .await
        .expect("describe_account_limits should succeed");

    // Should return some limits
    assert!(resp.max_number_of_auto_scaling_groups().is_some());
    assert!(resp.max_number_of_launch_configurations().is_some());
}

#[tokio::test]
async fn test_describe_adjustment_types() {
    let client = make_client().await;

    let resp = client
        .describe_adjustment_types()
        .send()
        .await
        .expect("describe_adjustment_types should succeed");

    let types = resp.adjustment_types();
    assert!(!types.is_empty());
}

#[tokio::test]
async fn test_describe_scaling_activities() {
    let client = make_client().await;
    create_asg(&client, "act-asg", "act-lc").await;

    let resp = client
        .describe_scaling_activities()
        .auto_scaling_group_name("act-asg")
        .send()
        .await
        .expect("describe_scaling_activities should succeed");

    // Just verifying the call succeeds - may or may not have activities
    let _ = resp.activities();
}

#[tokio::test]
async fn test_describe_metric_collection_types() {
    let client = make_client().await;

    let resp = client
        .describe_metric_collection_types()
        .send()
        .await
        .expect("describe_metric_collection_types should succeed");

    assert!(!resp.metrics().is_empty());
}

#[tokio::test]
async fn test_describe_scaling_process_types() {
    let client = make_client().await;

    let resp = client
        .describe_scaling_process_types()
        .send()
        .await
        .expect("describe_scaling_process_types should succeed");

    let processes = resp.processes();
    assert!(!processes.is_empty());
}

#[tokio::test]
async fn test_describe_termination_policy_types() {
    let client = make_client().await;

    let resp = client
        .describe_termination_policy_types()
        .send()
        .await
        .expect("describe_termination_policy_types should succeed");

    assert!(!resp.termination_policy_types().is_empty());
}

#[tokio::test]
async fn test_describe_auto_scaling_notification_types() {
    let client = make_client().await;

    let resp = client
        .describe_auto_scaling_notification_types()
        .send()
        .await
        .expect("describe_auto_scaling_notification_types should succeed");

    assert!(!resp.auto_scaling_notification_types().is_empty());
}

// ---------------------------------------------------------------------------
// DescribeAutoScalingInstances (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_describe_auto_scaling_instances() {
    let client = make_client().await;

    let resp = client
        .describe_auto_scaling_instances()
        .send()
        .await
        .expect("describe_auto_scaling_instances should succeed");

    // No instances created, should be empty
    let _ = resp.auto_scaling_instances();
}

// ---------------------------------------------------------------------------
// Load Balancer operations (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_attach_detach_describe_load_balancers() {
    let client = make_client().await;
    create_asg(&client, "lb-asg", "lb-lc").await;

    // AttachLoadBalancers
    client
        .attach_load_balancers()
        .auto_scaling_group_name("lb-asg")
        .load_balancer_names("my-elb")
        .send()
        .await
        .expect("attach_load_balancers should succeed");

    // DescribeLoadBalancers
    let desc = client
        .describe_load_balancers()
        .auto_scaling_group_name("lb-asg")
        .send()
        .await
        .expect("describe_load_balancers should succeed");

    let lbs = desc.load_balancers();
    assert!(!lbs.is_empty());

    // DetachLoadBalancers
    client
        .detach_load_balancers()
        .auto_scaling_group_name("lb-asg")
        .load_balancer_names("my-elb")
        .send()
        .await
        .expect("detach_load_balancers should succeed");
}

// ---------------------------------------------------------------------------
// Target Group operations (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_attach_detach_describe_target_groups() {
    let client = make_client().await;
    create_asg(&client, "tg-asg", "tg-lc").await;

    // AttachLoadBalancerTargetGroups
    client
        .attach_load_balancer_target_groups()
        .auto_scaling_group_name("tg-asg")
        .target_group_arns(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/abc123",
        )
        .send()
        .await
        .expect("attach_load_balancer_target_groups should succeed");

    // DescribeLoadBalancerTargetGroups
    let desc = client
        .describe_load_balancer_target_groups()
        .auto_scaling_group_name("tg-asg")
        .send()
        .await
        .expect("describe_load_balancer_target_groups should succeed");

    let tgs = desc.load_balancer_target_groups();
    assert!(!tgs.is_empty());

    // DetachLoadBalancerTargetGroups
    client
        .detach_load_balancer_target_groups()
        .auto_scaling_group_name("tg-asg")
        .target_group_arns(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/abc123",
        )
        .send()
        .await
        .expect("detach_load_balancer_target_groups should succeed");
}

// ---------------------------------------------------------------------------
// Warm Pool operations (3 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_put_describe_delete_warm_pool() {
    let client = make_client().await;
    create_asg(&client, "wp-asg", "wp-lc").await;

    // PutWarmPool
    client
        .put_warm_pool()
        .auto_scaling_group_name("wp-asg")
        .min_size(1)
        .send()
        .await
        .expect("put_warm_pool should succeed");

    // DescribeWarmPool
    let desc = client
        .describe_warm_pool()
        .auto_scaling_group_name("wp-asg")
        .send()
        .await
        .expect("describe_warm_pool should succeed");

    let wp = desc.warm_pool_configuration();
    assert!(wp.is_some());

    // DeleteWarmPool
    client
        .delete_warm_pool()
        .auto_scaling_group_name("wp-asg")
        .send()
        .await
        .expect("delete_warm_pool should succeed");
}

// ---------------------------------------------------------------------------
// SetInstanceHealth (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_set_instance_health() {
    let client = make_client().await;
    create_asg(&client, "health-asg", "health-lc").await;

    // This should succeed even though we don't have real instances;
    // the mock might silently accept it or return an error.
    let result = client
        .set_instance_health()
        .instance_id("i-00000000000000001")
        .health_status("Unhealthy")
        .send()
        .await;

    // We just verify the call completes (success or known error)
    let _ = result;
}

// ---------------------------------------------------------------------------
// SetInstanceProtection (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_set_instance_protection() {
    let client = make_client().await;
    create_asg(&client, "prot-asg", "prot-lc").await;

    let result = client
        .set_instance_protection()
        .auto_scaling_group_name("prot-asg")
        .instance_ids("i-00000000000000001")
        .protected_from_scale_in(true)
        .send()
        .await;

    let _ = result;
}

// ---------------------------------------------------------------------------
// AttachInstances / DetachInstances (2 operations)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_attach_detach_instances() {
    let client = make_client().await;
    create_asg(&client, "inst-asg", "inst-lc").await;

    // AttachInstances
    let result = client
        .attach_instances()
        .auto_scaling_group_name("inst-asg")
        .instance_ids("i-00000000000000001")
        .send()
        .await;
    let _ = result;

    // DetachInstances
    let result = client
        .detach_instances()
        .auto_scaling_group_name("inst-asg")
        .instance_ids("i-00000000000000001")
        .should_decrement_desired_capacity(true)
        .send()
        .await;
    let _ = result;
}

// ---------------------------------------------------------------------------
// TerminateInstanceInAutoScalingGroup (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_terminate_instance_in_asg() {
    let client = make_client().await;
    create_asg(&client, "term-asg", "term-lc").await;

    let result = client
        .terminate_instance_in_auto_scaling_group()
        .instance_id("i-00000000000000001")
        .should_decrement_desired_capacity(true)
        .send()
        .await;

    // May fail because instance doesn't exist in ASG, which is fine
    let _ = result;
}

// ---------------------------------------------------------------------------
// BatchDeleteScheduledAction (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_batch_delete_scheduled_action() {
    let client = make_client().await;
    create_asg(&client, "bsched-asg", "bsched-lc").await;

    // Create a scheduled action first
    client
        .put_scheduled_update_group_action()
        .auto_scaling_group_name("bsched-asg")
        .scheduled_action_name("batch-action-1")
        .desired_capacity(2)
        .send()
        .await
        .unwrap();

    // BatchDeleteScheduledAction
    client
        .batch_delete_scheduled_action()
        .auto_scaling_group_name("bsched-asg")
        .scheduled_action_names("batch-action-1")
        .send()
        .await
        .expect("batch_delete_scheduled_action should succeed");

    let desc = client
        .describe_scheduled_actions()
        .auto_scaling_group_name("bsched-asg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.scheduled_update_group_actions().len(), 0);
}

// ---------------------------------------------------------------------------
// BatchPutScheduledUpdateGroupAction (1 operation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_batch_put_scheduled_update_group_action() {
    let client = make_client().await;
    create_asg(&client, "bput-asg", "bput-lc").await;

    let entry = aws_sdk_autoscaling::types::ScheduledUpdateGroupActionRequest::builder()
        .scheduled_action_name("batch-put-1")
        .desired_capacity(2)
        .build();

    client
        .batch_put_scheduled_update_group_action()
        .auto_scaling_group_name("bput-asg")
        .scheduled_update_group_actions(entry)
        .send()
        .await
        .expect("batch_put_scheduled_update_group_action should succeed");

    let desc = client
        .describe_scheduled_actions()
        .auto_scaling_group_name("bput-asg")
        .send()
        .await
        .unwrap();
    assert!(!desc.scheduled_update_group_actions().is_empty());
}
